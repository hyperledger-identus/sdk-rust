#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0

import { execFileSync, spawnSync } from "node:child_process";
import { createHash, randomUUID } from "node:crypto";
import {
  chmodSync,
  closeSync,
  constants,
  existsSync,
  fstatSync,
  linkSync,
  lstatSync,
  mkdirSync,
  openSync,
  readFileSync,
  readSync,
  realpathSync,
  unlinkSync,
  writeFileSync,
} from "node:fs";
import path from "node:path";
import { TextDecoder } from "node:util";
import { fileURLToPath } from "node:url";
import { validatePullRequest } from "../ci/contribution-policy.mjs";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");
const pullRequestPolicy = path.join(root, "scripts", "check-pr-policy.sh");
export const maximumPullRequestBodyBytes = 64 * 1024;
const exactShaPattern = /^[0-9a-f]{40}$/u;
const deliveryStore = "sdk-rust-factory/delivery-v1";

function fail(message) {
  throw new Error(message);
}

function requireText(value, label) {
  if (typeof value !== "string" || value.length === 0 || value.includes("\0")) {
    fail(`${label} must be an explicit non-empty string`);
  }
  if (value.length > 4096) fail(`${label} exceeds its input bound`);
  return value;
}

export function readPullRequestBody(file) {
  requireText(file, "--body-file");
  const resolved = path.resolve(file);
  let before;
  try {
    before = lstatSync(resolved);
  } catch {
    fail("pull request body file is unavailable");
  }
  if (!before.isFile() || before.isSymbolicLink()) {
    fail("pull request body must be a regular non-symlink file");
  }
  if (before.size > maximumPullRequestBodyBytes) {
    fail("pull request body file exceeds its 64 KiB byte bound");
  }

  let descriptor;
  try {
    const flags = constants.O_RDONLY | (constants.O_NOFOLLOW ?? 0) | (constants.O_NONBLOCK ?? 0);
    descriptor = openSync(resolved, flags);
  } catch {
    fail("pull request body file could not be opened safely");
  }

  let bytes;
  try {
    const opened = fstatSync(descriptor);
    if (!opened.isFile() || opened.dev !== before.dev || opened.ino !== before.ino) {
      fail("pull request body file changed during validation");
    }
    if (opened.size > maximumPullRequestBodyBytes) {
      fail("pull request body file exceeds its 64 KiB byte bound");
    }
    const bounded = Buffer.allocUnsafe(maximumPullRequestBodyBytes + 1);
    let length = 0;
    while (length < bounded.length) {
      const count = readSync(descriptor, bounded, length, bounded.length - length, null);
      if (count === 0) break;
      length += count;
    }
    if (length > maximumPullRequestBodyBytes) {
      fail("pull request body file exceeds its 64 KiB byte bound");
    }
    bytes = bounded.subarray(0, length);
  } finally {
    closeSync(descriptor);
  }

  let body;
  try {
    body = new TextDecoder("utf-8", { fatal: true }).decode(bytes);
  } catch {
    fail("pull request body file must contain valid UTF-8");
  }
  if (body.includes("\0")) fail("pull request body file contains an invalid null byte");
  return body;
}

export function runPullRequestPolicy(environment) {
  const childEnvironment = { ...process.env, ...environment };
  delete childEnvironment.GITHUB_OUTPUT;
  const outcome = spawnSync(pullRequestPolicy, [], {
    cwd: root,
    encoding: "utf8",
    env: childEnvironment,
    maxBuffer: 1024 * 1024,
    stdio: ["ignore", "pipe", "pipe"],
  });
  if (outcome.error) fail("pull-request policy checker could not be executed");
  return outcome;
}

class PullRequestPolicyError extends Error {
  constructor(diagnostics) {
    super("pull request metadata policy failed");
    this.name = "PullRequestPolicyError";
    this.diagnostics = diagnostics;
  }
}

function policyDiagnostics(stderr) {
  const diagnostics = String(stderr ?? "").split(/\r?\n/u).filter(Boolean);
  return diagnostics.length > 0
    ? diagnostics
    : ["pull-request-policy: checker failed without diagnostics"];
}

export function preflightPullRequest({ title, bodyFile, headRef, baseRef, draft }, {
  runPolicy = runPullRequestPolicy,
} = {}) {
  requireText(title, "--title");
  requireText(headRef, "--head-ref");
  requireText(baseRef, "--base-ref");
  if (typeof draft !== "boolean") fail("--draft must be explicitly true or false");
  const body = readPullRequestBody(bodyFile);
  const environment = Object.freeze({
    PR_BASE_REF: baseRef,
    PR_DRAFT: String(draft),
    PR_BODY: body,
  });
  const shellOutcome = runPolicy(environment);
  if (!shellOutcome || shellOutcome.status !== 0) {
    throw new PullRequestPolicyError(policyDiagnostics(shellOutcome?.stderr));
  }

  const contribution = validatePullRequest({ title, body, branch: headRef });
  if (!contribution.ok) {
    throw new PullRequestPolicyError(
      contribution.errors.map((entry) => `[contribution-policy] ${entry}`),
    );
  }
  return {
    ok: true,
    issue: contribution.branch.issue,
    type: contribution.subject.type,
    scope: contribution.subject.scope,
  };
}

export function parseDeliveryArguments(argv) {
  const [command, ...rest] = argv;
  if (command !== "pr-preflight") fail("delivery requires the pr-preflight command");
  const names = new Set(["--title", "--body-file", "--head-ref", "--base-ref", "--draft"]);
  const values = new Map();
  for (let index = 0; index < rest.length; index += 2) {
    const name = rest[index];
    const value = rest[index + 1];
    if (!names.has(name)) fail(`unknown delivery argument: ${name ?? "<missing>"}`);
    if (values.has(name)) fail(`duplicate delivery argument: ${name}`);
    if (value === undefined || value.startsWith("--")) fail(`${name} requires an explicit value`);
    values.set(name, value);
  }
  for (const name of names) if (!values.has(name)) fail(`pr-preflight requires ${name}`);
  const draftValue = values.get("--draft");
  if (draftValue !== "true" && draftValue !== "false") {
    fail("--draft must be explicitly true or false");
  }
  return {
    title: values.get("--title"),
    bodyFile: values.get("--body-file"),
    headRef: values.get("--head-ref"),
    baseRef: values.get("--base-ref"),
    draft: draftValue === "true",
  };
}

function positiveInteger(value, label) {
  const parsed = Number(value);
  if (!Number.isSafeInteger(parsed) || parsed < 1) fail(`${label} must be a positive integer`);
  return parsed;
}

function exactSha(value, label) {
  if (!exactShaPattern.test(value ?? "")) fail(`${label} must be an exact lowercase SHA`);
  return value;
}

export function parseMergeArguments(argv) {
  const [command, ...rest] = argv;
  if (command !== "merge-pr") fail("delivery requires the merge-pr command");
  const names = new Set(["--pr", "--expect-head", "--body-file"]);
  const values = new Map();
  let execute = false;
  for (let index = 0; index < rest.length;) {
    const name = rest[index];
    if (name === "--execute") {
      if (execute) fail("duplicate delivery argument: --execute");
      execute = true;
      index += 1;
      continue;
    }
    if (!names.has(name)) fail(`unknown delivery argument: ${name ?? "<missing>"}`);
    if (values.has(name)) fail(`duplicate delivery argument: ${name}`);
    const value = rest[index + 1];
    if (value === undefined || value.startsWith("--")) fail(`${name} requires an explicit value`);
    values.set(name, value);
    index += 2;
  }
  for (const name of names) if (!values.has(name)) fail(`merge-pr requires ${name}`);
  return {
    pullRequest: positiveInteger(values.get("--pr"), "--pr"),
    expectedHead: exactSha(values.get("--expect-head"), "--expect-head"),
    bodyFile: values.get("--body-file"),
    execute,
  };
}

export function validateMergeBody(body, { name, email }) {
  requireText(name, "Git user name");
  requireText(email, "Git user email");
  if (typeof body !== "string" || body.length === 0) fail("merge body must not be empty");
  if (body.includes("\\n")) fail("merge body contains a literal escaped newline");
  const normalized = body.replace(/\r\n/gu, "\n").replace(/\n+$/u, "");
  if (!normalized.includes("\n")) fail("merge body must contain real multiline text");
  const trailer = `Signed-off-by: ${name} <${email}>`;
  if (normalized.split("\n").at(-1) !== trailer) fail("merge body must end with the exact Git identity DCO trailer");
  return normalized;
}

function gitText(repositoryRoot, args) {
  return execFileSync("git", args, {
    cwd: repositoryRoot,
    encoding: "utf8",
    maxBuffer: 1024 * 1024,
    stdio: ["ignore", "pipe", "pipe"],
  }).trim();
}

function repositoryIdentity(repositoryRoot = root) {
  const remote = gitText(repositoryRoot, ["remote", "get-url", "origin"]);
  const repository = remote.match(/github\.com[/:]([^/]+\/[^/.]+)(?:\.git)?$/u)?.[1];
  if (!repository) fail("origin does not identify a GitHub repository");
  return repository;
}

function gitIdentity(repositoryRoot = root) {
  return {
    name: gitText(repositoryRoot, ["config", "user.name"]),
    email: gitText(repositoryRoot, ["config", "user.email"]),
  };
}

function hostedGithub(repository) {
  const run = (args, options = {}) => {
    const outcome = spawnSync("gh", args, {
      encoding: "utf8",
      maxBuffer: 4 * 1024 * 1024,
      stdio: [options.input === undefined ? "ignore" : "pipe", "pipe", "pipe"],
      input: options.input,
    });
    if (outcome.error || outcome.status !== 0) fail(options.failure ?? "GitHub delivery operation failed");
    return outcome.stdout;
  };
  return {
    readPullRequest(number) {
      return JSON.parse(run([
        "pr", "view", String(number), "--repo", repository, "--json",
        "state,isDraft,baseRefName,headRefOid,mergeable,mergeStateStatus,reviewDecision,mergedAt,mergeCommit",
      ], { failure: "pull request state is unavailable" }));
    },
    requiredChecksPass(number) {
      run(["pr", "checks", String(number), "--repo", repository, "--required"], {
        failure: "required pull request checks are not successful",
      });
      return true;
    },
    merge(number, expectedHead, body) {
      run([
        "pr", "merge", String(number), "--repo", repository, "--squash",
        "--match-head-commit", expectedHead, "--body-file", "-",
      ], { input: `${body}\n`, failure: "protected pull request merge failed" });
    },
    readCommit(sha) {
      return JSON.parse(run(["api", `repos/${repository}/commits/${sha}`], {
        failure: "merged commit verification is unavailable",
      }));
    },
  };
}

function validatePreMergeState(state, expectedHead) {
  if (!state || typeof state !== "object") fail("pull request state is malformed");
  if (state.state !== "OPEN") fail("pull request is not open");
  if (state.isDraft !== false) fail("pull request is not ready");
  if (state.baseRefName !== "develop") fail("pull request does not target develop");
  if (state.headRefOid !== expectedHead) fail("pull request head does not match the expected head");
  if (state.mergeable !== "MERGEABLE" || state.mergeStateStatus !== "CLEAN") {
    fail("pull request mergeability is not clean");
  }
  if (state.reviewDecision === "CHANGES_REQUESTED") fail("pull request has blocking review changes");
}

export function exactIsoDate(value) {
  if (typeof value !== "string") return false;
  const match = /^(\d{4})-(\d{2})-(\d{2})T(\d{2}):(\d{2}):(\d{2})(?:\.(\d{1,9}))?Z$/u.exec(value);
  if (!match) return false;
  const parsed = Date.parse(value);
  if (!Number.isFinite(parsed)) return false;
  const instant = new Date(parsed);
  const expected = match.slice(1, 7).map(Number);
  const actual = [
    instant.getUTCFullYear(),
    instant.getUTCMonth() + 1,
    instant.getUTCDate(),
    instant.getUTCHours(),
    instant.getUTCMinutes(),
    instant.getUTCSeconds(),
  ];
  return actual.every((part, index) => part === expected[index]);
}

export function validateMergeReceipt(receipt) {
  const expected = [
    "schemaVersion", "repository", "pullRequest", "headSha", "mergeCommitSha",
    "baseRef", "mergedAt", "mergeBodySha256", "requiredChecksPassed",
    "verifiedSignature", "verificationReason", "protectedMergePath",
  ].sort();
  const keys = receipt && typeof receipt === "object" && !Array.isArray(receipt)
    ? Object.keys(receipt).sort()
    : [];
  const errors = [];
  if (keys.length !== expected.length || keys.some((key, index) => key !== expected[index])) {
    errors.push("merge receipt has an unknown or missing field");
    return { ok: false, errors };
  }
  if (receipt.schemaVersion !== 1) errors.push("merge receipt schema is invalid");
  if (!/^[A-Za-z0-9_.-]+\/[A-Za-z0-9_.-]+$/u.test(receipt.repository)) errors.push("merge receipt repository is invalid");
  if (!Number.isSafeInteger(receipt.pullRequest) || receipt.pullRequest < 1) errors.push("merge receipt pull request is invalid");
  if (!exactShaPattern.test(receipt.headSha) || !exactShaPattern.test(receipt.mergeCommitSha)) errors.push("merge receipt SHA is invalid");
  if (receipt.baseRef !== "develop") errors.push("merge receipt base is invalid");
  if (!exactIsoDate(receipt.mergedAt)) errors.push("merge receipt timestamp is invalid");
  if (!/^[0-9a-f]{64}$/u.test(receipt.mergeBodySha256)) errors.push("merge receipt body digest is invalid");
  if (receipt.requiredChecksPassed !== true || receipt.verifiedSignature !== true
      || receipt.verificationReason !== "valid" || receipt.protectedMergePath !== true) {
    errors.push("merge receipt verification is incomplete");
  }
  return { ok: errors.length === 0, errors };
}

function ensurePrivateDirectory(directory) {
  if (!existsSync(directory)) mkdirSync(directory, { mode: 0o700 });
  const info = lstatSync(directory);
  if (!info.isDirectory() || info.isSymbolicLink()) fail("delivery private store must contain regular directories only");
  chmodSync(directory, 0o700);
}

function deliveryStoreRoot(repositoryRoot = root) {
  const common = realpathSync(path.resolve(repositoryRoot, gitText(repositoryRoot, ["rev-parse", "--git-common-dir"])));
  if (!lstatSync(common).isDirectory()) fail("Git common directory is invalid");
  let current = common;
  for (const segment of deliveryStore.split("/")) {
    current = path.join(current, segment);
    ensurePrivateDirectory(current);
  }
  return current;
}

export function retainMergeReceipt(receipt, { repositoryRoot = root } = {}) {
  const validation = validateMergeReceipt(receipt);
  if (!validation.ok) fail(validation.errors.join("; "));
  const store = deliveryStoreRoot(repositoryRoot);
  const directory = path.join(store, `pr-${receipt.pullRequest}`);
  ensurePrivateDirectory(directory);
  const file = path.join(directory, `${receipt.headSha}.json`);
  const payload = `${JSON.stringify(receipt, null, 2)}\n`;
  if (existsSync(file)) {
    const info = lstatSync(file);
    if (!info.isFile() || info.isSymbolicLink() || (info.mode & 0o077) !== 0) fail("existing merge receipt is unsafe");
    if (readFileSync(file, "utf8") !== payload) fail("existing merge receipt conflicts with exact receipt");
    return file;
  }
  const temporary = path.join(directory, `.${receipt.headSha}.${randomUUID()}.tmp`);
  writeFileSync(temporary, payload, { encoding: "utf8", mode: 0o600, flag: "wx" });
  try {
    linkSync(temporary, file);
  } catch (error) {
    if (!existsSync(file)) throw error;
    const existing = lstatSync(file);
    if (!existing.isFile() || existing.isSymbolicLink() || (existing.mode & 0o077) !== 0
        || readFileSync(file, "utf8") !== payload) {
      throw error;
    }
  } finally {
    unlinkSync(temporary);
  }
  chmodSync(file, 0o600);
  return file;
}

export function mergePullRequest(options, dependencies = {}) {
  const repository = dependencies.repository ?? repositoryIdentity(dependencies.repositoryRoot ?? root);
  const identity = dependencies.identity ?? gitIdentity(dependencies.repositoryRoot ?? root);
  const github = dependencies.github ?? hostedGithub(repository);
  const persist = dependencies.persist ?? ((receipt) => retainMergeReceipt(receipt, {
    repositoryRoot: dependencies.repositoryRoot ?? root,
  }));
  const body = validateMergeBody(readPullRequestBody(options.bodyFile), identity);
  const before = github.readPullRequest(options.pullRequest);
  const alreadyMerged = before?.state === "MERGED";
  if (!alreadyMerged) validatePreMergeState(before, options.expectedHead);
  if (github.requiredChecksPass(options.pullRequest) !== true) fail("required pull request checks are not successful");
  if (!options.execute && !alreadyMerged) {
    return { eligible: true, executed: false, mergePerformed: false, receipt: null };
  }

  if (!alreadyMerged) github.merge(options.pullRequest, options.expectedHead, body);
  const after = alreadyMerged ? before : github.readPullRequest(options.pullRequest);
  if (after.state !== "MERGED" || after.baseRefName !== "develop" || after.headRefOid !== options.expectedHead
      || !exactIsoDate(after.mergedAt) || !exactShaPattern.test(after.mergeCommit?.oid ?? "")) {
    fail("hosted post-merge state does not match the validated pull request");
  }
  const commit = github.readCommit(after.mergeCommit.oid);
  if (commit?.sha !== after.mergeCommit.oid || commit?.commit?.verification?.verified !== true
      || commit.commit.verification.reason !== "valid") {
    fail("merged commit signature verification is invalid");
  }
  const commitMessage = String(commit?.commit?.message ?? "").replace(/\r\n/gu, "\n").replace(/\n+$/u, "");
  if (!commitMessage.endsWith(body)) fail("merged commit message does not contain the exact validated body");
  const receipt = {
    schemaVersion: 1,
    repository,
    pullRequest: options.pullRequest,
    headSha: options.expectedHead,
    mergeCommitSha: after.mergeCommit.oid,
    baseRef: "develop",
    mergedAt: after.mergedAt,
    mergeBodySha256: createHash("sha256").update(body).digest("hex"),
    requiredChecksPassed: true,
    verifiedSignature: true,
    verificationReason: "valid",
    protectedMergePath: true,
  };
  if (options.execute) persist(receipt);
  return {
    eligible: true,
    executed: options.execute,
    mergePerformed: !alreadyMerged,
    receipt,
  };
}

function usage() {
  return [
    "Usage:",
    "  scripts/factory delivery pr-preflight [options]",
    "  scripts/factory delivery merge-pr [options]",
    "",
    "Options:",
    "  --title <title>       Explicit pull-request title",
    "  --body-file <path>    Bounded regular non-symlink pull-request body file",
    "  --head-ref <ref>      Explicit pull-request head branch",
    "  --base-ref <ref>      Explicit pull-request base branch",
    "  --draft <true|false>  Explicit pull-request draft state",
    "",
    "Merge options:",
    "  --pr <number>         Pull request number",
    "  --expect-head <sha>   Exact hosted pull-request head",
    "  --body-file <path>    Real multiline DCO-bearing squash body",
    "  --execute             Perform the protected merge after validation",
  ].join("\n");
}

function main() {
  if (["help", "--help", "-h"].includes(process.argv[2])) {
    process.stdout.write(`${usage()}\n`);
    return;
  }
  if (process.argv[2] === "pr-preflight") {
    const options = parseDeliveryArguments(process.argv.slice(2));
    const outcome = preflightPullRequest(options);
    process.stdout.write(`factory: pull request metadata preflight passed for issue #${outcome.issue}\n`);
    return;
  }
  if (process.argv[2] === "merge-pr") {
    const options = parseMergeArguments(process.argv.slice(2));
    const outcome = mergePullRequest(options);
    process.stdout.write(outcome.executed
      ? `factory: protected merge receipt retained for PR #${options.pullRequest}\n`
      : `factory: PR #${options.pullRequest} is eligible; pass --execute to merge\n`);
    return;
  }
  fail("delivery requires pr-preflight or merge-pr");
}

if (path.resolve(process.argv[1] ?? "") === fileURLToPath(import.meta.url)) {
  try {
    main();
  } catch (error) {
    if (Array.isArray(error?.diagnostics)) {
      for (const diagnostic of error.diagnostics) process.stderr.write(`${diagnostic}\n`);
    } else {
      process.stderr.write(`[factory-delivery] ${error.message}\n`);
    }
    process.exitCode = 1;
  }
}
