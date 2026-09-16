#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0

import { randomUUID } from "node:crypto";
import { execFileSync } from "node:child_process";
import {
  chmodSync,
  existsSync,
  linkSync,
  lstatSync,
  mkdirSync,
  readFileSync,
  realpathSync,
  renameSync,
  unlinkSync,
  writeFileSync,
} from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { parseJsonWithoutDuplicates } from "./strict-json.mjs";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");
const policy = parseJsonWithoutDuplicates(readFileSync(path.join(root, ".factory-policy.json"), "utf8"));
const v1Keys = new Set([
  "schemaVersion", "repository", "issue", "pullRequest", "headSha", "profile",
  "startedAt", "completedAt", "durationSeconds", "retryCount", "toolCalls",
  "inputTokens", "outputTokens", "coveragePercent", "diskBytes", "outcome",
]);
const v1NullableCounters = ["durationSeconds", "retryCount", "toolCalls", "inputTokens", "outputTokens", "coveragePercent", "diskBytes"];
const v2Keys = [
  "schemaVersion", "repository", "issue", "pullRequest", "headSha", "profile",
  "startedAt", "completedAt", "durationSeconds", "phaseDurationsSeconds", "ci",
  "runtime", "coveragePercent", "resources", "outcome",
];
const phaseKeys = ["planning", "implementation", "review", "validation", "hostedCi", "blocked"];
const ciKeys = [
  "queueSeconds", "executionSeconds", "failedAttempts", "canceledAttempts",
  "retryCount", "postCiPushCount", "checks",
];
const runtimeKeys = [
  "sessions", "turns", "toolCalls", "inputTokens", "outputTokens", "cacheReadTokens",
  "cacheWriteTokens",
];
const resourceKeys = ["worktreePeakBytes", "targetPeakBytes", "cachePeakBytes", "processPeakRssBytes"];
const unavailableReasons = new Set([
  "not-measured", "source-missing", "source-malformed", "source-unsafe",
  "unsupported-version", "incomplete-usage", "incomplete-events", "not-applicable",
]);
const forbiddenPattern = /(?:prompt|message|transcript|sessionId|responseId|provider|model|cost|rawOutput|tokenValue|credential|secret|privateKey|mnemonic|seed|authorization)/iu;

function fail(message) {
  throw new Error(message);
}

function git(args) {
  return execFileSync("git", args, { cwd: root, encoding: "utf8", stdio: ["ignore", "pipe", "pipe"] }).trim();
}

function exactSha(value) {
  return /^[0-9a-f]{40}$/u.test(value ?? "");
}

function positiveInteger(value) {
  return Number.isSafeInteger(value) && value > 0;
}

function nonnegativeInteger(value) {
  return Number.isSafeInteger(value) && value >= 0;
}

function nullableNonnegative(value) {
  return value === null || (Number.isFinite(value) && value >= 0);
}

function isoDate(value, exact = false) {
  if (typeof value !== "string") return false;
  const parsed = Date.parse(value);
  return Number.isFinite(parsed) && (!exact || new Date(parsed).toISOString() === value);
}

function exactKeys(record, expected) {
  if (!record || typeof record !== "object" || Array.isArray(record)) return false;
  const actual = Object.keys(record).sort();
  const sortedExpected = [...expected].sort();
  return actual.length === sortedExpected.length && actual.every((key, index) => key === sortedExpected[index]);
}

function validateStableIdentity(record, errors, { exactTimestamps = false } = {}) {
  if (record.repository !== policy.repository) errors.push("repository identity is invalid");
  if (!positiveInteger(record.issue)) errors.push("issue must be a positive integer");
  if (record.pullRequest !== null && !positiveInteger(record.pullRequest)) errors.push("pullRequest must be null or a positive integer");
  if (!exactSha(record.headSha)) errors.push("headSha must be an exact lowercase SHA");
  if (!["prototype", "production-ready", "integration"].includes(record.profile)) errors.push("profile is invalid");
  if (!isoDate(record.startedAt, exactTimestamps) || !isoDate(record.completedAt, exactTimestamps)) errors.push("timestamps must be ISO date-times");
  if (isoDate(record.startedAt) && isoDate(record.completedAt) && Date.parse(record.completedAt) < Date.parse(record.startedAt)) errors.push("completedAt precedes startedAt");
  if (!["merged", "closed", "blocked", "in-progress"].includes(record.outcome)) errors.push("outcome is invalid");
}

function validateV1(record) {
  const errors = [];
  for (const key of Object.keys(record)) {
    if (!v1Keys.has(key)) errors.push(`unknown field: ${key}`);
    if (forbiddenPattern.test(key)) errors.push(`forbidden field: ${key}`);
  }
  for (const key of v1Keys) if (!Object.hasOwn(record, key)) errors.push(`missing field: ${key}`);
  if (record.schemaVersion !== 1) errors.push("schema identity is invalid");
  validateStableIdentity(record, errors);
  for (const key of v1NullableCounters) if (!nullableNonnegative(record[key])) errors.push(`${key} must be null or non-negative`);
  if (record.coveragePercent !== null && record.coveragePercent > 100) errors.push("coveragePercent exceeds 100");
  return errors;
}

function validateMeasurement(candidate, label, { percentage = false } = {}) {
  if (!exactKeys(candidate, ["value", "unavailableReason"])) return [`${label} measurement is malformed`];
  if (candidate.value === null) {
    if (!unavailableReasons.has(candidate.unavailableReason)) return [`${label} measurement lacks a closed unavailable reason`];
    return [];
  }
  const valueIsValid = percentage
    ? Number.isFinite(candidate.value) && candidate.value >= 0 && candidate.value <= 100
    : nonnegativeInteger(candidate.value);
  if (!valueIsValid || candidate.unavailableReason !== null) return [`${label} measurement is not exact`];
  return [];
}

function retryCount(checks) {
  const maxima = new Map();
  for (const check of checks) maxima.set(check.name, Math.max(maxima.get(check.name) ?? 0, check.attempt));
  return [...maxima.values()].reduce((total, attempts) => total + attempts - 1, 0);
}

function validateV2(record) {
  const errors = [];
  if (!exactKeys(record, v2Keys)) return ["version 2 record has an unknown or missing field"];
  if (record.schemaVersion !== 2) errors.push("schema identity is invalid");
  validateStableIdentity(record, errors, { exactTimestamps: true });
  errors.push(...validateMeasurement(record.durationSeconds, "durationSeconds"));

  if (!exactKeys(record.phaseDurationsSeconds, phaseKeys)) errors.push("phase durations have an unknown or missing field");
  else for (const key of phaseKeys) errors.push(...validateMeasurement(record.phaseDurationsSeconds[key], `phaseDurationsSeconds.${key}`));

  if (!exactKeys(record.ci, ciKeys)) errors.push("CI metrics have an unknown or missing field");
  else {
    for (const key of ciKeys.filter((key) => key !== "checks")) errors.push(...validateMeasurement(record.ci[key], `ci.${key}`));
    if (!Array.isArray(record.ci.checks) || record.ci.checks.length > 128) errors.push("CI check history is invalid");
    else {
      const identities = new Set();
      for (const [index, check] of record.ci.checks.entries()) {
        if (!exactKeys(check, ["name", "attempt", "outcome", "queueSeconds", "executionSeconds"])) {
          errors.push("CI check has an unknown or missing field");
          continue;
        }
        if (typeof check.name !== "string" || !/^[a-z0-9][a-z0-9._-]{0,63}$/u.test(check.name)) errors.push("CI check name is invalid");
        else if (forbiddenPattern.test(check.name)) errors.push("CI check name contains a forbidden privacy term");
        if (!positiveInteger(check.attempt) || check.attempt > 64) errors.push("CI check attempt is invalid");
        const identity = `${check.name}:${check.attempt}`;
        if (identities.has(identity)) errors.push("CI check identity is duplicated");
        identities.add(identity);
        if (!["passed", "failed", "canceled"].includes(check.outcome)) errors.push("CI check outcome is invalid");
        errors.push(...validateMeasurement(check.queueSeconds, `ci.checks.${index + 1}.queueSeconds`));
        errors.push(...validateMeasurement(check.executionSeconds, `ci.checks.${index + 1}.executionSeconds`));
      }
      const attemptsByCheck = new Map();
      for (const check of record.ci.checks) {
        if (typeof check?.name !== "string" || !positiveInteger(check?.attempt)) continue;
        const attempts = attemptsByCheck.get(check.name) ?? [];
        attempts.push(check.attempt);
        attemptsByCheck.set(check.name, attempts);
      }
      for (const attempts of attemptsByCheck.values()) {
        const ordered = [...new Set(attempts)].sort((left, right) => left - right);
        if (ordered.some((value, index) => value !== index + 1)) errors.push("CI check attempts are not contiguous");
      }
      const failed = record.ci.checks.filter((check) => check?.outcome === "failed").length;
      const canceled = record.ci.checks.filter((check) => check?.outcome === "canceled").length;
      const retries = retryCount(record.ci.checks.filter((check) => typeof check?.name === "string" && positiveInteger(check?.attempt)));
      if (record.ci.failedAttempts.value !== null && record.ci.failedAttempts.value !== failed) errors.push("CI failed-attempt counter conflicts with attempt history");
      if (record.ci.canceledAttempts.value !== null && record.ci.canceledAttempts.value !== canceled) errors.push("CI canceled-attempt counter conflicts with attempt history");
      if (record.ci.retryCount.value !== null && record.ci.retryCount.value !== retries) errors.push("CI retry counter conflicts with attempt history");
    }
  }

  if (!exactKeys(record.runtime, runtimeKeys)) errors.push("runtime metrics have an unknown or missing field");
  else for (const key of runtimeKeys) errors.push(...validateMeasurement(record.runtime[key], `runtime.${key}`));
  errors.push(...validateMeasurement(record.coveragePercent, "coveragePercent", { percentage: true }));
  if (!exactKeys(record.resources, resourceKeys)) errors.push("resource metrics have an unknown or missing field");
  else for (const key of resourceKeys) errors.push(...validateMeasurement(record.resources[key], `resources.${key}`));
  return errors;
}

export function validateMetric(record, { requireCurrentHead = false } = {}) {
  if (!record || typeof record !== "object" || Array.isArray(record)) return { ok: false, errors: ["record must be an object"] };
  let errors;
  if (record.schemaVersion === 1) errors = validateV1(record);
  else if (record.schemaVersion === 2) errors = validateV2(record);
  else errors = ["unsupported metric schema version"];
  if (requireCurrentHead && exactSha(record.headSha) && record.headSha !== git(["rev-parse", "HEAD"])) errors.push("headSha is not the current exact HEAD");
  return { ok: errors.length === 0, errors };
}

function metricVersionPolicy(schemaVersion) {
  const selected = policy.metrics.versions[String(schemaVersion)];
  if (!selected) fail("unsupported metric schema version");
  return selected;
}

function renderedValue(measurement, suffix = "") {
  return measurement.value === null
    ? `unknown (${measurement.unavailableReason})`
    : `${measurement.value}${suffix}`;
}

function renderV1(record) {
  const value = (candidate, suffix = "") => candidate === null ? "unknown" : `${candidate}${suffix}`;
  return [
    "### Factory metrics",
    "",
    `- Exact head: \`${record.headSha}\``,
    `- Profile/outcome: \`${record.profile}\` / \`${record.outcome}\``,
    `- Duration/retries/tool calls: ${value(record.durationSeconds, "s")} / ${value(record.retryCount)} / ${value(record.toolCalls)}`,
    `- Tokens (input/output): ${value(record.inputTokens)} / ${value(record.outputTokens)}`,
    `- Coverage/disk: ${value(record.coveragePercent, "%")} / ${value(record.diskBytes, " bytes")}`,
  ].join("\n");
}

function renderV2(record) {
  const phase = record.phaseDurationsSeconds;
  const runtime = record.runtime;
  const ci = record.ci;
  const resources = record.resources;
  return [
    "### Factory metrics",
    "",
    `- Exact head: \`${record.headSha}\``,
    `- Profile/outcome: \`${record.profile}\` / \`${record.outcome}\``,
    `- Duration (total/planning/implementation/review/validation/hosted CI/blocked): ${renderedValue(record.durationSeconds, "s")} / ${renderedValue(phase.planning, "s")} / ${renderedValue(phase.implementation, "s")} / ${renderedValue(phase.review, "s")} / ${renderedValue(phase.validation, "s")} / ${renderedValue(phase.hostedCi, "s")} / ${renderedValue(phase.blocked, "s")}`,
    `- CI (queue/execution/failed/canceled/retries/post-CI pushes): ${renderedValue(ci.queueSeconds, "s")} / ${renderedValue(ci.executionSeconds, "s")} / ${renderedValue(ci.failedAttempts)} / ${renderedValue(ci.canceledAttempts)} / ${renderedValue(ci.retryCount)} / ${renderedValue(ci.postCiPushCount)}`,
    `- Runtime (sessions/turns/tools): ${renderedValue(runtime.sessions)} / ${renderedValue(runtime.turns)} / ${renderedValue(runtime.toolCalls)}`,
    `- Tokens (input/output/cache read/cache write): ${renderedValue(runtime.inputTokens)} / ${renderedValue(runtime.outputTokens)} / ${renderedValue(runtime.cacheReadTokens)} / ${renderedValue(runtime.cacheWriteTokens)}`,
    `- Coverage: ${renderedValue(record.coveragePercent, "%")}`,
    `- Peaks (worktree/target/cache/process RSS): ${renderedValue(resources.worktreePeakBytes, " bytes")} / ${renderedValue(resources.targetPeakBytes, " bytes")} / ${renderedValue(resources.cachePeakBytes, " bytes")} / ${renderedValue(resources.processPeakRssBytes, " bytes")}`,
  ].join("\n");
}

export function renderMetric(record) {
  const result = validateMetric(record);
  if (!result.ok) fail(result.errors.join("; "));
  const versionPolicy = metricVersionPolicy(record.schemaVersion);
  const summary = record.schemaVersion === 1 ? renderV1(record) : renderV2(record);
  const payload = Buffer.from(JSON.stringify(record), "utf8").toString("base64url");
  const rendered = `${summary}\n\n<!-- ${versionPolicy.publicMarker}:${payload} -->\n`;
  if (Buffer.byteLength(rendered) > policy.metrics.maximumPublicBytes) fail("public metrics payload exceeds configured byte bound");
  return rendered;
}

function ensurePrivateDirectory(directory) {
  if (!existsSync(directory)) mkdirSync(directory, { mode: 0o700 });
  const info = lstatSync(directory);
  if (!info.isDirectory() || info.isSymbolicLink()) fail("metric private store must contain regular directories only");
  chmodSync(directory, 0o700);
}

function metricStoreLayout(schemaVersion, { create = false } = {}) {
  const common = realpathSync(path.resolve(root, git(["rev-parse", "--git-common-dir"])));
  const versionPolicy = metricVersionPolicy(schemaVersion);
  const segments = versionPolicy.privateStore.split("/");
  if (segments.some((segment) => segment === "" || segment === "." || segment === ".." || segment.includes("\\"))) fail("metric private store path is invalid");
  let store = common;
  for (const segment of segments) {
    store = path.join(store, segment);
    if (create) ensurePrivateDirectory(store);
    else if (existsSync(store)) {
      const info = lstatSync(store);
      if (!info.isDirectory() || info.isSymbolicLink()) fail("metric private store must contain regular directories only");
    }
  }
  const relative = path.relative(common, store);
  if (relative.startsWith("..") || path.isAbsolute(relative)) fail("metric private store is outside the Git common directory");
  return store;
}

export function privateStoreRoot(schemaVersion = policy.metrics.defaultSchemaVersion) {
  return metricStoreLayout(schemaVersion);
}

export function readMetricFile(file) {
  const resolved = path.resolve(file);
  const info = lstatSync(resolved);
  if (!info.isFile() || info.isSymbolicLink() || info.size > 32768) fail("metric record must be a regular non-symlink file no larger than 32 KiB");
  const content = readFileSync(resolved, "utf8");
  if (forbiddenPattern.test(content)) fail("metric record contains a forbidden privacy term");
  return parseJsonWithoutDuplicates(content);
}

function canonicalJson(value) {
  if (Array.isArray(value)) return `[${value.map(canonicalJson).join(",")}]`;
  if (value !== null && typeof value === "object") {
    return `{${Object.keys(value).sort().map((key) => `${JSON.stringify(key)}:${canonicalJson(value[key])}`).join(",")}}`;
  }
  return JSON.stringify(value);
}

function isLegacyDraftTransition(existing, record) {
  return existing.outcome === "in-progress"
    && record.outcome !== "in-progress"
    && existing.schemaVersion === record.schemaVersion
    && existing.repository === record.repository
    && existing.issue === record.issue
    && existing.headSha === record.headSha
    && existing.profile === record.profile
    && existing.startedAt === record.startedAt
    && (existing.pullRequest === record.pullRequest || existing.pullRequest === null);
}

export function retainMetricRecord(target, record) {
  const result = validateMetric(record);
  if (!result.ok) fail(result.errors.join("; "));
  if (record.outcome === "in-progress") fail("in-progress metric record cannot be retained");
  const temporary = `${target}.${process.pid}.${randomUUID()}.tmp`;
  try {
    writeFileSync(temporary, `${JSON.stringify(record, null, 2)}\n`, { flag: "wx", mode: 0o600 });
    try {
      linkSync(temporary, target);
    } catch (error) {
      if (error?.code !== "EEXIST") throw error;
      const existing = readMetricFile(target);
      const existingValidation = validateMetric(existing);
      if (!existingValidation.ok) fail("existing private metric record conflicts with exact record");
      if (canonicalJson(existing) === canonicalJson(record)) {
        // Equal terminal evidence is idempotent.
      } else if (isLegacyDraftTransition(existing, record)) {
        renameSync(temporary, target);
      } else {
        fail("existing private metric record conflicts with exact record");
      }
    }
    chmodSync(target, 0o600);
  } finally {
    if (existsSync(temporary)) unlinkSync(temporary);
  }
  return target;
}

function persistPrivate(record, { requireCurrentHead = false } = {}) {
  const result = validateMetric(record, { requireCurrentHead });
  if (!result.ok) fail(result.errors.join("; "));
  const store = metricStoreLayout(record.schemaVersion, { create: true });
  const directory = path.join(store, `issue-${record.issue}`);
  ensurePrivateDirectory(directory);
  return retainMetricRecord(path.join(directory, `${record.headSha}.json`), record);
}

function writePrivate(record) {
  process.stdout.write(`${persistPrivate(record, { requireCurrentHead: true })}\n`);
}

export function selectOwnedMetricComment(comments, login, publicMarker) {
  if (!Array.isArray(comments) || typeof login !== "string" || typeof publicMarker !== "string") fail("metric publication inputs are invalid");
  const marker = `<!-- ${publicMarker}:`;
  const owned = comments.filter((comment) => comment?.user?.login === login && String(comment.body).includes(marker));
  if (owned.length > 1) fail("multiple owned metrics comments exist; refusing ambiguous update");
  return owned[0] ?? null;
}

export function resolveMetricPublicationTarget(record, requestedTarget = "auto") {
  if (!["auto", "pull-request", "issue"].includes(requestedTarget)) {
    fail("metric publication target must be auto, pull-request or issue");
  }
  if (requestedTarget === "pull-request" && record.pullRequest === null) {
    fail("pull-request publication requires a recorded pull request");
  }
  if (requestedTarget === "issue" || record.pullRequest === null) {
    return { kind: "issue", number: record.issue };
  }
  return { kind: "pull-request", number: record.pullRequest };
}

export function validateHostedMetricIdentity(record, { issue, pullRequest = null, currentHead = null }) {
  if (issue?.number !== record.issue || Object.hasOwn(issue ?? {}, "pull_request")) {
    fail("metric issue does not match an authoritative repository issue");
  }
  if (record.pullRequest !== null) {
    if (pullRequest?.number !== record.pullRequest || pullRequest?.headRefOid !== record.headSha) {
      fail("metric pull request does not match its exact hosted head");
    }
    const closesRecordedIssue = Array.isArray(pullRequest.closingIssuesReferences)
      && pullRequest.closingIssuesReferences.some((reference) => (
        reference?.number === record.issue && reference?.repository === record.repository
      ));
    if (!closesRecordedIssue) fail("metric pull request is not linked to its recorded issue");
  } else if (currentHead !== record.headSha) {
    fail("metric without a pull request must match the current exact HEAD");
  }
}

export function runMetricPublicationMutation(mutate) {
  const maximumAttempts = policy.metrics.publication.maximumMutationAttempts;
  if (maximumAttempts !== 2) fail("metric publication policy must allow exactly one retry");
  for (let attempt = 1; attempt <= maximumAttempts; attempt += 1) {
    try {
      return mutate();
    } catch {
      if (attempt === 2) {
        fail("metric publication remains telemetry debt after bounded retry");
      }
    }
  }
  fail("metric publication remains telemetry debt after bounded retry");
}

function createMetricComment(github, target, login, marker, body) {
  try {
    return github.createComment(target, body);
  } catch {
    let recovered;
    try {
      recovered = selectOwnedMetricComment(github.readComments(target), login, marker);
    } catch {
      fail("metric publication remains telemetry debt after bounded retry");
    }
    if (recovered !== null) {
      if (!positiveInteger(recovered.id)) fail("owned metrics comment has an invalid identity");
      if (recovered.body === body) return recovered;
      try {
        return github.updateComment(recovered.id, body);
      } catch {
        fail("metric publication remains telemetry debt after bounded retry");
      }
    }
    try {
      return github.createComment(target, body);
    } catch {
      fail("metric publication remains telemetry debt after bounded retry");
    }
  }
}

export function publishMetric(record, {
  issue,
  requestedTarget = "auto",
  execute = false,
  currentHead,
  github,
  persist = persistPrivate,
}) {
  if (!execute) fail("publication requires --execute");
  const validation = validateMetric(record);
  if (!validation.ok) fail(validation.errors.join("; "));
  if (record.issue !== issue) fail("metric issue does not match publication target");
  const target = resolveMetricPublicationTarget(record, requestedTarget);
  const hostedIssue = github.readIssue(record.issue);
  const hostedPullRequest = record.pullRequest === null ? null : github.readPullRequest(record.pullRequest);
  validateHostedMetricIdentity(record, { issue: hostedIssue, pullRequest: hostedPullRequest, currentHead });
  const body = renderMetric(record);
  persist(record);
  const versionPolicy = metricVersionPolicy(record.schemaVersion);
  const login = github.readLogin();
  const comments = github.readComments(target.number);
  const owned = selectOwnedMetricComment(comments, login, versionPolicy.publicMarker);
  if (owned !== null) {
    if (!positiveInteger(owned.id)) fail("owned metrics comment has an invalid identity");
    runMetricPublicationMutation(() => github.updateComment(owned.id, body));
  } else {
    createMetricComment(github, target.number, login, versionPolicy.publicMarker, body);
  }
  return target;
}

function githubJson(args, failureMessage) {
  try {
    return JSON.parse(execFileSync("gh", args, { encoding: "utf8", maxBuffer: 16 * 1024 * 1024 }));
  } catch {
    fail(failureMessage);
  }
}

function githubText(args, failureMessage) {
  try {
    return execFileSync("gh", args, { encoding: "utf8" }).trim();
  } catch {
    fail(failureMessage);
  }
}

function githubPagedArray(args, failureMessage) {
  const pages = githubJson([...args, "--paginate", "--slurp"], failureMessage);
  if (!Array.isArray(pages) || pages.some((page) => !Array.isArray(page))) fail(failureMessage);
  return pages.flat();
}

function publish(record, issue, requestedTarget, execute) {
  const [owner, name] = policy.repository.split("/");
  const github = {
    readIssue: (number) => githubJson(
      ["api", `repos/${policy.repository}/issues/${number}`],
      "unable to verify authoritative metric issue",
    ),
    readPullRequest: (number) => {
      const response = githubJson([
        "api", "graphql",
        "-f", `owner=${owner}`,
        "-f", `name=${name}`,
        "-F", `number=${number}`,
        "-f", "query=query($owner:String!,$name:String!,$number:Int!){repository(owner:$owner,name:$name){pullRequest(number:$number){number headRefOid closingIssuesReferences(first:100){nodes{number repository{nameWithOwner}}}}}}",
      ], "unable to verify authoritative metric pull request");
      const pullRequest = response?.data?.repository?.pullRequest;
      return {
        number: pullRequest?.number,
        headRefOid: pullRequest?.headRefOid,
        closingIssuesReferences: pullRequest?.closingIssuesReferences?.nodes?.map((reference) => ({
          number: reference?.number,
          repository: reference?.repository?.nameWithOwner,
        })),
      };
    },
    readLogin: () => githubText(["api", "user", "--jq", ".login"], "unable to resolve authenticated metric publisher"),
    readComments: (number) => githubPagedArray(
      ["api", `repos/${policy.repository}/issues/${number}/comments`],
      "unable to inspect existing metric comments",
    ),
    updateComment: (comment, body) => execFileSync(
      "gh",
      ["api", "--method", "PATCH", `repos/${policy.repository}/issues/comments/${comment}`, "-f", `body=${body}`],
      { stdio: ["ignore", "ignore", "ignore"] },
    ),
    createComment: (number, body) => execFileSync(
      "gh",
      ["api", "--method", "POST", `repos/${policy.repository}/issues/${number}/comments`, "-f", `body=${body}`],
      { stdio: ["ignore", "ignore", "ignore"] },
    ),
  };
  return publishMetric(record, {
    issue,
    requestedTarget,
    execute,
    currentHead: git(["rev-parse", "HEAD"]),
    github,
  });
}

function argument(argv, name, required = true) {
  const index = argv.indexOf(name);
  const value = index >= 0 ? argv[index + 1] : null;
  if (required && !value) fail(`${name} is required`);
  return value;
}

function unknownMeasurement() {
  return { value: null, unavailableReason: "not-measured" };
}

export function metricTemplate(issue, schemaVersion = policy.metrics.defaultSchemaVersion, { headSha = null } = {}) {
  const now = new Date().toISOString();
  const exactHead = headSha ?? git(["rev-parse", "HEAD"]);
  if (!exactSha(exactHead)) fail("metric template head must be an exact lowercase SHA");
  const stable = {
    repository: policy.repository,
    issue,
    pullRequest: null,
    headSha: exactHead,
    profile: policy.delivery.defaultProfile,
    startedAt: now,
    completedAt: now,
  };
  if (schemaVersion === 1) {
    return {
      schemaVersion: 1,
      ...stable,
      durationSeconds: null,
      retryCount: null,
      toolCalls: null,
      inputTokens: null,
      outputTokens: null,
      coveragePercent: null,
      diskBytes: null,
      outcome: "in-progress",
    };
  }
  if (schemaVersion !== 2) fail("unsupported metric schema version");
  const phaseDurationsSeconds = Object.fromEntries(phaseKeys.map((key) => [key, unknownMeasurement()]));
  const ci = Object.fromEntries(ciKeys.filter((key) => key !== "checks").map((key) => [key, unknownMeasurement()]));
  ci.checks = [];
  return {
    schemaVersion: 2,
    ...stable,
    durationSeconds: unknownMeasurement(),
    phaseDurationsSeconds,
    ci,
    runtime: Object.fromEntries(runtimeKeys.map((key) => [key, unknownMeasurement()])),
    coveragePercent: unknownMeasurement(),
    resources: Object.fromEntries(resourceKeys.map((key) => [key, unknownMeasurement()])),
    outcome: "in-progress",
  };
}

function main() {
  const [command, ...argv] = process.argv.slice(2);
  if (command === "template") {
    const issue = Number(argument(argv, "--issue"));
    const schemaVersion = Number(argument(argv, "--schema-version", false) ?? policy.metrics.defaultSchemaVersion);
    if (!positiveInteger(issue)) fail("--issue must be a positive integer");
    process.stdout.write(`${JSON.stringify(metricTemplate(issue, schemaVersion), null, 2)}\n`);
  } else if (["validate", "write", "render", "publish"].includes(command)) {
    const record = readMetricFile(argument(argv, "--file"));
    if (command === "validate") {
      const result = validateMetric(record, { requireCurrentHead: argv.includes("--current-head") });
      if (!result.ok) fail(result.errors.join("; "));
      process.stdout.write("factory: metric record is valid\n");
    } else if (command === "write") writePrivate(record);
    else if (command === "render") process.stdout.write(renderMetric(record));
    else {
      const target = publish(
        record,
        Number(argument(argv, "--issue")),
        argument(argv, "--target", false) ?? "auto",
        argv.includes("--execute"),
      );
      process.stdout.write(`factory: metric receipt published to ${target.kind} #${target.number}\n`);
    }
  } else {
    fail("Usage: metrics.mjs <template|validate|write|render|publish> [options]; publish accepts --target auto|pull-request|issue");
  }
}

if (path.resolve(process.argv[1] ?? "") === fileURLToPath(import.meta.url)) {
  try {
    main();
  } catch (error) {
    process.stderr.write(`[factory-metrics] ${error.message}\n`);
    process.exitCode = 1;
  }
}
