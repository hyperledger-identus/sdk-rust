#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0

import { createHash, randomUUID } from "node:crypto";
import { execFileSync, spawn } from "node:child_process";
import {
  chmodSync,
  closeSync,
  existsSync,
  lstatSync,
  mkdirSync,
  openSync,
  readFileSync,
  readdirSync,
  realpathSync,
  renameSync,
  writeFileSync,
} from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { validateReceipt } from "./preflight.mjs";
import { parseJsonWithoutDuplicates } from "./strict-json.mjs";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");
const policy = parseJsonWithoutDuplicates(readFileSync(path.join(root, ".factory-policy.json"), "utf8"));
const shaPattern = /^[0-9a-f]{40}$/u;
const digestPattern = /^[0-9a-f]{64}$/u;
const changePattern = /^[a-z0-9]+(?:-[a-z0-9]+)*$/u;
const taskPattern = /^[0-9]+\.[0-9]+(?:\.[0-9]+)?$/u;
const evidencePattern = /^[a-z0-9][a-z0-9.-]{0,63}$/u;
const forbiddenValuePattern = /(?:prompt|transcript|session-id|response-id|provider|model|cost|raw-output|credential|secret|private-key|mnemonic|authorization)/iu;
const invocationKeys = [
  "schemaVersion", "repository", "issue", "profile", "baseRef", "baseSha",
  "branch", "headSha", "change", "contractHeadSha", "receiptSha256", "role",
  "task", "allowedPaths", "tools", "deadlineSeconds", "postArtifactGraceSeconds",
  "preparedAt",
];
const heartbeatKeys = [
  "schemaVersion", "state", "observedAt", "elapsedSeconds", "processActive",
  "workerExitCode", "terminationReason",
];
const handoffKeys = [
  "schemaVersion", "repository", "issue", "change", "baseSha", "branch", "headSha",
  "contractHeadSha", "receiptSha256", "role", "task", "status", "changedPaths",
  "acceptance", "checks", "findings", "processOwnership", "nextAction", "completedAt",
];
const identityKeys = [
  "repository", "issue", "baseRef", "baseSha", "branch", "headSha", "change",
  "contractHeadSha", "receiptSha256",
];

function fail(message) {
  throw new Error(message);
}

function git(repositoryRoot, args, options = {}) {
  return execFileSync("git", args, {
    cwd: repositoryRoot,
    encoding: Object.hasOwn(options, "encoding") ? options.encoding : "utf8",
    maxBuffer: 8 * 1024 * 1024,
    stdio: ["ignore", "pipe", "pipe"],
  });
}

function gitText(repositoryRoot, args) {
  return git(repositoryRoot, args).trim();
}

function positiveInteger(value) {
  return Number.isSafeInteger(value) && value > 0;
}

function nonnegativeInteger(value) {
  return Number.isSafeInteger(value) && value >= 0;
}

function isoDate(value) {
  if (typeof value !== "string") return false;
  const parsed = Date.parse(value);
  return Number.isFinite(parsed) && new Date(parsed).toISOString() === value;
}

function hasExactKeys(record, expected) {
  if (!record || typeof record !== "object" || Array.isArray(record)) return false;
  const actual = Object.keys(record).sort();
  return actual.length === expected.length && actual.every((key, index) => key === [...expected].sort()[index]);
}

function uniqueStrings(values, maximum) {
  return Array.isArray(values)
    && values.length <= maximum
    && values.every((value) => typeof value === "string")
    && new Set(values).size === values.length;
}

export function normalizeRelativePath(candidate) {
  if (typeof candidate !== "string" || candidate.length < 1 || candidate.length > 256) fail("path must contain 1 to 256 characters");
  if (candidate.includes("\\") || candidate.includes("\0") || path.posix.isAbsolute(candidate)) fail("path must be repository-relative");
  if (!/^[A-Za-z0-9._/-]+$/u.test(candidate)) fail("path contains a disallowed character");
  const segments = candidate.split("/");
  if (segments.some((segment) => segment === "" || segment === "." || segment === "..")) fail("path is not normalized");
  const normalized = path.posix.normalize(candidate);
  if (normalized !== candidate || normalized === "." || normalized.startsWith("../")) fail("path is not normalized");
  return normalized;
}

export function pathIsAllowed(candidate, allowedPaths) {
  return allowedPaths.some((prefix) => candidate === prefix || candidate.startsWith(`${prefix}/`));
}

function assertWithin(candidate, boundary, label) {
  const relative = path.relative(boundary, candidate);
  if (relative === "" || (!relative.startsWith(`..${path.sep}`) && relative !== ".." && !path.isAbsolute(relative))) return;
  fail(`${label} is outside its private boundary`);
}

function assertNoSymlinkComponents(candidate, boundary, { allowMissing = false } = {}) {
  const resolvedBoundary = realpathSync(boundary);
  assertWithin(path.resolve(candidate), resolvedBoundary, "path");
  const relative = path.relative(resolvedBoundary, path.resolve(candidate));
  let current = resolvedBoundary;
  for (const component of relative.split(path.sep).filter(Boolean)) {
    current = path.join(current, component);
    if (!existsSync(current)) {
      if (allowMissing) continue;
      fail("private path does not exist");
    }
    if (lstatSync(current).isSymbolicLink()) fail("private path contains a symbolic link");
  }
}

function ensureAllowedPathLocation(repositoryRoot, candidate) {
  const normalized = normalizeRelativePath(candidate);
  const resolvedRoot = realpathSync(repositoryRoot);
  const absolute = path.resolve(resolvedRoot, normalized);
  assertWithin(absolute, resolvedRoot, "allowed path");
  assertNoSymlinkComponents(absolute, resolvedRoot, { allowMissing: true });
  return normalized;
}

export function assertDeclaredTask(repositoryRoot, change, task) {
  const taskFile = path.join(repositoryRoot, "openspec", "changes", change, "tasks.md");
  const bytes = readBoundedRegular(taskFile, { boundary: repositoryRoot, maximumBytes: 65536 });
  const escaped = task.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&");
  if (!new RegExp(`^- \\[(?: |[xX])\\] ${escaped}(?:\\s|$)`, "mu").test(bytes.toString("utf8"))) {
    fail("invocation task is not declared by the active OpenSpec change");
  }
}

function readBoundedRegular(file, { boundary, maximumBytes, ownerOnly = false }) {
  if (!path.isAbsolute(file) || path.resolve(file) !== file) fail("private artifact path must be canonical and absolute");
  const resolvedBoundary = realpathSync(boundary);
  assertWithin(file, resolvedBoundary, "private artifact");
  assertNoSymlinkComponents(file, resolvedBoundary);
  const info = lstatSync(file);
  if (!info.isFile() || info.isSymbolicLink()) fail("private artifact must be a regular non-symlink file");
  if (info.size > maximumBytes) fail("private artifact exceeds its byte bound");
  if (ownerOnly && (info.mode & 0o077) !== 0) fail("private artifact permissions are not owner-only");
  return readFileSync(file);
}

export function readBoundedPrivateJsonFile(file, options) {
  const bytes = readBoundedRegular(file, options);
  return { bytes, value: parseJsonWithoutDuplicates(bytes.toString("utf8")) };
}

function ensurePrivateRunDirectory(directory, boundary) {
  const resolvedBoundary = realpathSync(boundary);
  assertWithin(directory, resolvedBoundary, "private directory");
  assertNoSymlinkComponents(path.dirname(directory), resolvedBoundary, { allowMissing: true });
  if (!existsSync(directory)) mkdirSync(directory, { recursive: false, mode: 0o700 });
  const info = lstatSync(directory);
  if (!info.isDirectory() || info.isSymbolicLink()) fail("private run path must contain regular directories only");
  chmodSync(directory, 0o700);
  assertNoSymlinkComponents(directory, resolvedBoundary);
}

export function hardenPrivateSessionTree(sessionRoot, runDirectory) {
  assertNoSymlinkComponents(sessionRoot, runDirectory);
  let entries = 0;
  function visit(directory, depth) {
    if (depth > 4) fail("session directory exceeds its depth bound");
    chmodSync(directory, 0o700);
    for (const name of readdirSync(directory)) {
      entries += 1;
      if (entries > 256) fail("session directory exceeds its entry bound");
      const candidate = path.join(directory, name);
      const info = lstatSync(candidate);
      if (info.isSymbolicLink()) fail("session directory contains a symbolic link");
      if (info.isDirectory()) visit(candidate, depth + 1);
      else if (info.isFile()) chmodSync(candidate, 0o600);
      else fail("session directory contains a non-regular entry");
    }
  }
  visit(sessionRoot, 0);
}

function atomicWriteJson(file, record, { replace = false } = {}) {
  const temporary = `${file}.${process.pid}.${randomUUID()}.tmp`;
  writeFileSync(temporary, `${JSON.stringify(record, null, 2)}\n`, { flag: "wx", mode: 0o600 });
  if (!replace && existsSync(file)) fail("private artifact already exists");
  renameSync(temporary, file);
  chmodSync(file, 0o600);
}

function privateCommonDirectory(repositoryRoot) {
  const common = path.resolve(repositoryRoot, gitText(repositoryRoot, ["rev-parse", "--git-common-dir"]));
  const resolved = realpathSync(common);
  if (!lstatSync(resolved).isDirectory()) fail("Git common directory is invalid");
  return resolved;
}

export function supervisorStoreRoot(repositoryRoot = root) {
  return path.join(privateCommonDirectory(repositoryRoot), policy.supervisor.privateStore);
}

function ensureSupervisorStore(repositoryRoot) {
  const common = privateCommonDirectory(repositoryRoot);
  const segments = policy.supervisor.privateStore.split("/");
  if (segments.some((segment) => segment === "" || segment === "." || segment === ".." || segment.includes("\\"))) fail("supervisor private store path is invalid");
  let current = common;
  for (const segment of segments) {
    current = path.join(current, segment);
    if (!existsSync(current)) mkdirSync(current, { mode: 0o700 });
    const info = lstatSync(current);
    if (!info.isDirectory() || info.isSymbolicLink()) fail("supervisor private store must contain regular directories only");
    chmodSync(current, 0o700);
  }
  return current;
}

export function validateInvocationEnvelope(record, { identity = null } = {}) {
  const errors = [];
  if (!hasExactKeys(record, invocationKeys)) return { ok: false, errors: ["invocation has an unknown or missing field"] };
  if (record.schemaVersion !== 1 || record.repository !== policy.repository) errors.push("invocation repository identity is invalid");
  if (!positiveInteger(record.issue)) errors.push("invocation issue is invalid");
  if (!["prototype", "production-ready", "integration"].includes(record.profile)) errors.push("invocation profile is invalid");
  if (record.baseRef !== "origin/develop" || !shaPattern.test(record.baseSha) || !shaPattern.test(record.headSha) || !shaPattern.test(record.contractHeadSha)) errors.push("invocation Git identity is invalid");
  if (typeof record.branch !== "string" || !/^[A-Za-z0-9][A-Za-z0-9._/-]{0,127}$/u.test(record.branch)) errors.push("invocation branch is invalid");
  if (typeof record.change !== "string" || record.change.length > 128 || !changePattern.test(record.change)) errors.push("invocation change is invalid");
  if (!digestPattern.test(record.receiptSha256)) errors.push("invocation receipt digest is invalid");
  if (!Object.hasOwn(policy.supervisor.roleTools, record.role)) errors.push("invocation role is invalid");
  if (typeof record.task !== "string" || record.task.length > 32 || !taskPattern.test(record.task)) errors.push("invocation task is invalid");
  if (!uniqueStrings(record.allowedPaths, 64) || record.allowedPaths.length < 1) errors.push("invocation allowed paths are invalid");
  else {
    for (const candidate of record.allowedPaths) {
      try { normalizeRelativePath(candidate); } catch { errors.push("invocation contains an unsafe allowed path"); }
    }
  }
  if (!uniqueStrings(record.tools, 7) || record.tools.length < 1) errors.push("invocation tools are invalid");
  else if (Object.hasOwn(policy.supervisor.roleTools, record.role)) {
    const roleTools = new Set(policy.supervisor.roleTools[record.role]);
    if (record.tools.some((tool) => !roleTools.has(tool))) errors.push("invocation tool is not allowed for its role");
  }
  if (!positiveInteger(record.deadlineSeconds) || record.deadlineSeconds > 7200) errors.push("invocation deadline is invalid");
  if (!positiveInteger(record.postArtifactGraceSeconds) || record.postArtifactGraceSeconds > 60) errors.push("invocation post-artifact grace is invalid");
  if (!isoDate(record.preparedAt)) errors.push("invocation timestamp is invalid");
  if (identity) {
    for (const key of identityKeys) if (record[key] !== identity[key]) errors.push(`invocation ${key} is stale`);
  }
  return { ok: errors.length === 0, errors };
}

export function validateHeartbeat(record) {
  const errors = [];
  if (!hasExactKeys(record, heartbeatKeys)) return { ok: false, errors: ["heartbeat has an unknown or missing field"] };
  if (record.schemaVersion !== 1) errors.push("heartbeat schema is invalid");
  if (!["prepared", "running", "terminating", "exited", "timed-out"].includes(record.state)) errors.push("heartbeat state is invalid");
  if (!isoDate(record.observedAt) || !nonnegativeInteger(record.elapsedSeconds)) errors.push("heartbeat observation is invalid");
  if (typeof record.processActive !== "boolean") errors.push("heartbeat process state is invalid");
  if (record.workerExitCode !== null && (!nonnegativeInteger(record.workerExitCode) || record.workerExitCode > 255)) errors.push("heartbeat worker exit is invalid");
  if (![null, "deadline", "post-artifact-grace"].includes(record.terminationReason)) errors.push("heartbeat termination reason is invalid");
  if (["running", "terminating"].includes(record.state) !== record.processActive) errors.push("heartbeat process activity conflicts with state");
  if (["prepared", "running"].includes(record.state) && (record.workerExitCode !== null || record.terminationReason !== null)) errors.push("heartbeat running state contains terminal data");
  if (record.state === "exited" && record.terminationReason !== null) errors.push("heartbeat exited state conflicts with termination data");
  if (record.state === "timed-out" && record.terminationReason === null) errors.push("heartbeat timeout reason is missing");
  return { ok: errors.length === 0, errors };
}

function validateEvidenceList(values, { checks = false } = {}) {
  const errors = [];
  if (!Array.isArray(values) || values.length > 128 || (!checks && values.length < 1)) return ["handoff evidence list is invalid"];
  const identities = new Set();
  for (const value of values) {
    const expected = checks ? ["id", "outcome", "durationSeconds"] : ["id", "outcome"];
    if (!hasExactKeys(value, expected)) {
      errors.push("handoff evidence has an unknown or missing field");
      continue;
    }
    if (typeof value.id !== "string" || !evidencePattern.test(value.id) || forbiddenValuePattern.test(value.id)) errors.push("handoff evidence identity is invalid");
    if (identities.has(value.id)) errors.push("handoff evidence identity is duplicated");
    identities.add(value.id);
    if (!["passed", "failed", "not-run"].includes(value.outcome)) errors.push("handoff evidence outcome is invalid");
    if (checks) {
      if (value.outcome === "not-run" && value.durationSeconds !== null) errors.push("unrun handoff check has a duration");
      if (value.outcome !== "not-run" && !nonnegativeInteger(value.durationSeconds)) errors.push("run handoff check lacks an exact duration");
    }
  }
  return errors;
}

export function validateHandoffRecord(record, { envelope, currentIdentity, actualChangedPaths, processGroupActive = false }) {
  const errors = [];
  if (!hasExactKeys(record, handoffKeys)) return { ok: false, accepted: false, errors: ["handoff has an unknown or missing field"] };
  if (record.schemaVersion !== 1 || record.repository !== policy.repository || !positiveInteger(record.issue)) errors.push("handoff repository identity is invalid");
  for (const key of ["repository", "issue", "change", "baseSha", "branch", "headSha", "contractHeadSha", "receiptSha256", "role", "task"]) {
    if (record[key] !== envelope[key]) errors.push(`handoff ${key} does not match invocation`);
  }
  const invocationResult = validateInvocationEnvelope(envelope, { identity: currentIdentity });
  if (!invocationResult.ok) errors.push(...invocationResult.errors);
  if (!["completed", "checkpoint", "blocked", "failed"].includes(record.status)) errors.push("handoff status is invalid");
  if (!uniqueStrings(record.changedPaths, 256)) errors.push("handoff changed paths are invalid");
  else {
    for (const candidate of record.changedPaths) {
      try {
        const normalized = normalizeRelativePath(candidate);
        if (!pathIsAllowed(normalized, envelope.allowedPaths)) errors.push("handoff contains an out-of-scope changed path");
      } catch {
        errors.push("handoff contains an unsafe changed path");
      }
    }
    const declared = [...record.changedPaths].sort();
    const actual = [...actualChangedPaths].sort();
    if (JSON.stringify(declared) !== JSON.stringify(actual)) errors.push("handoff changed paths do not match current Git effects");
  }
  errors.push(...validateEvidenceList(record.acceptance));
  errors.push(...validateEvidenceList(record.checks, { checks: true }));
  if (!Array.isArray(record.findings) || record.findings.length > 128) errors.push("handoff findings are invalid");
  else {
    for (const finding of record.findings) {
      if (!hasExactKeys(finding, ["class", "disposition"])) errors.push("handoff finding has an unknown or missing field");
      else if (!["must-fix", "worth-fixing-now", "defer"].includes(finding.class) || !["resolved", "unresolved", "deferred"].includes(finding.disposition)) errors.push("handoff finding is invalid");
    }
  }
  if (!hasExactKeys(record.processOwnership, ["workerOwnedRemaining"]) || !nonnegativeInteger(record.processOwnership.workerOwnedRemaining) || record.processOwnership.workerOwnedRemaining > 1024) errors.push("handoff process ownership is invalid");
  if (!["supervisor-verify", "supervisor-review", "supervisor-resume", "stop-blocked", "none"].includes(record.nextAction)) errors.push("handoff next action is unsafe");
  if (!isoDate(record.completedAt)) errors.push("handoff completion timestamp is invalid");
  else if (isoDate(envelope.preparedAt) && Date.parse(record.completedAt) < Date.parse(envelope.preparedAt)) errors.push("handoff completion precedes invocation");
  const acceptancePassed = Array.isArray(record.acceptance) && record.acceptance.every((item) => item?.outcome === "passed");
  const checksAcceptable = Array.isArray(record.checks) && record.checks.every((item) => item?.outcome !== "failed");
  const findingsClosed = Array.isArray(record.findings) && record.findings.every((item) => {
    if (item?.class === "must-fix") return item.disposition === "resolved";
    return item?.disposition !== "unresolved";
  });
  const processClosed = record.processOwnership?.workerOwnedRemaining === 0 && !processGroupActive;
  const checkpoint = ["completed", "checkpoint"].includes(record.status);
  const safeCloseout = ["supervisor-verify", "supervisor-review", "none"].includes(record.nextAction);
  return {
    ok: errors.length === 0,
    accepted: errors.length === 0 && acceptancePassed && checksAcceptable && findingsClosed && processClosed && checkpoint && safeCloseout,
    errors,
  };
}

export async function collectCurrentIdentity(change, issue, { repositoryRoot = root } = {}) {
  if (realpathSync(repositoryRoot) !== realpathSync(root)) fail("current identity validation is restricted to this repository");
  const receipt = await validateReceipt(change, issue);
  const receiptFile = path.join(repositoryRoot, "openspec", "changes", change, "preimplementation.json");
  const { bytes, value: strictReceipt } = readBoundedPrivateJsonFile(receiptFile, {
    boundary: repositoryRoot,
    maximumBytes: 8192,
  });
  if (JSON.stringify(receipt) !== JSON.stringify(strictReceipt)) fail("pre-implementation receipt changed during validation");
  const branch = gitText(repositoryRoot, ["branch", "--show-current"]);
  const headSha = gitText(repositoryRoot, ["rev-parse", "HEAD"]);
  const baseSha = gitText(repositoryRoot, ["rev-parse", receipt.baseRef]);
  const mergeBase = gitText(repositoryRoot, ["merge-base", receipt.baseRef, "HEAD"]);
  if (receipt.baseSha !== baseSha || mergeBase !== baseSha) fail("pre-implementation receipt no longer matches exact origin/develop base");
  return {
    repository: policy.repository,
    issue,
    baseRef: receipt.baseRef,
    baseSha,
    branch,
    headSha,
    change,
    contractHeadSha: receipt.contractHeadSha,
    receiptSha256: createHash("sha256").update(bytes).digest("hex"),
  };
}

export function currentChangedPaths(repositoryRoot = root) {
  const tracked = git(repositoryRoot, ["diff", "--name-only", "--no-renames", "-z", "HEAD"], { encoding: null });
  const untracked = git(repositoryRoot, ["ls-files", "--others", "--exclude-standard", "-z"], { encoding: null });
  return [...new Set(Buffer.concat([tracked, untracked]).toString("utf8").split("\0").filter(Boolean))].sort();
}

export async function prepareInvocation(options, { repositoryRoot = root } = {}) {
  const identity = await collectCurrentIdentity(options.change, options.issue, { repositoryRoot });
  assertDeclaredTask(repositoryRoot, options.change, options.task);
  const allowedPaths = options.allowedPaths.map((candidate) => ensureAllowedPathLocation(repositoryRoot, candidate));
  const outOfScopeExisting = currentChangedPaths(repositoryRoot).filter((candidate) => !pathIsAllowed(candidate, allowedPaths));
  if (outOfScopeExisting.length > 0) fail("current Git effects are outside the invocation path allowlist");
  const envelope = {
    schemaVersion: 1,
    ...identity,
    profile: options.profile,
    role: options.role,
    task: options.task,
    allowedPaths,
    tools: options.tools,
    deadlineSeconds: options.deadlineSeconds,
    postArtifactGraceSeconds: options.postArtifactGraceSeconds,
    preparedAt: new Date().toISOString(),
  };
  const validation = validateInvocationEnvelope(envelope, { identity });
  if (!validation.ok) fail(validation.errors.join("; "));
  const store = ensureSupervisorStore(repositoryRoot);
  const issueDirectory = path.join(store, `issue-${envelope.issue}`);
  ensurePrivateRunDirectory(issueDirectory, store);
  const headDirectory = path.join(issueDirectory, envelope.headSha);
  ensurePrivateRunDirectory(headDirectory, issueDirectory);
  const runName = `${envelope.preparedAt.replace(/[:.]/gu, "-")}-${randomUUID()}`;
  const runDirectory = path.join(headDirectory, runName);
  ensurePrivateRunDirectory(runDirectory, headDirectory);
  for (const child of ["session", "gh-config"]) ensurePrivateRunDirectory(path.join(runDirectory, child), runDirectory);
  const envelopeFile = path.join(runDirectory, "invocation.json");
  atomicWriteJson(envelopeFile, envelope);
  const heartbeat = {
    schemaVersion: 1,
    state: "prepared",
    observedAt: envelope.preparedAt,
    elapsedSeconds: 0,
    processActive: false,
    workerExitCode: null,
    terminationReason: null,
  };
  const heartbeatValidation = validateHeartbeat(heartbeat);
  if (!heartbeatValidation.ok) fail(heartbeatValidation.errors.join("; "));
  atomicWriteJson(path.join(runDirectory, "heartbeat.json"), heartbeat);
  return { envelope, envelopeFile, runDirectory };
}

export async function validateInvocationFile(envelopeFile, { repositoryRoot = root } = {}) {
  const store = supervisorStoreRoot(repositoryRoot);
  const { value: envelope } = readBoundedPrivateJsonFile(envelopeFile, {
    boundary: store,
    maximumBytes: policy.supervisor.maximumEnvelopeBytes,
    ownerOnly: true,
  });
  if (path.basename(envelopeFile) !== "invocation.json") fail("invocation artifact name is invalid");
  const identity = await collectCurrentIdentity(envelope.change, envelope.issue, { repositoryRoot });
  const result = validateInvocationEnvelope(envelope, { identity });
  if (!result.ok) fail(result.errors.join("; "));
  for (const candidate of envelope.allowedPaths) ensureAllowedPathLocation(repositoryRoot, candidate);
  return envelope;
}

export async function validateHandoffFile(envelopeFile, { repositoryRoot = root, processGroupActive = false } = {}) {
  const envelope = await validateInvocationFile(envelopeFile, { repositoryRoot });
  const runDirectory = path.dirname(envelopeFile);
  const handoffFile = path.join(runDirectory, "handoff.json");
  const { value: handoff } = readBoundedPrivateJsonFile(handoffFile, {
    boundary: runDirectory,
    maximumBytes: policy.supervisor.maximumHandoffBytes,
    ownerOnly: true,
  });
  const currentIdentity = await collectCurrentIdentity(envelope.change, envelope.issue, { repositoryRoot });
  const result = validateHandoffRecord(handoff, {
    envelope,
    currentIdentity,
    actualChangedPaths: currentChangedPaths(repositoryRoot),
    processGroupActive,
  });
  return { handoff, ...result };
}

export function buildPiLaunch(envelope, envelopeFile, { repositoryRoot = root } = {}) {
  const runDirectory = path.dirname(envelopeFile);
  return {
    command: path.join(repositoryRoot, "bootstrap.sh"),
    args: [
      "--pi",
      "--mode", "json",
      "--print",
      "--session-dir", path.join(runDirectory, "session"),
      "--tools", envelope.tools.join(","),
      "--no-extensions",
      "--no-skills",
      "--no-prompt-templates",
      "--approve",
    ],
    cwd: repositoryRoot,
  };
}

function workerContract(envelope) {
  return Buffer.from([
    "Factory supervisor contract:",
    `- Deliver only OpenSpec task ${envelope.task} as role ${envelope.role}.`,
    "- Read SDK_FACTORY_INVOCATION for exact identity and scope.",
    "- Do not commit, push, mutate GitHub, merge, release, publish, clean a worktree, or mutate a downstream repository.",
    "- Before exit, atomically write an owner-only worker-handoff-v1 JSON record to SDK_FACTORY_HANDOFF.",
    "- The supervisor independently validates Git effects, evidence, processes, and the handoff; worker prose is not acceptance.",
    "",
  ].join("\n"), "utf8");
}

async function readBoundedStdin(input, maximumBytes) {
  const chunks = [];
  let total = 0;
  for await (const chunk of input) {
    const bytes = Buffer.isBuffer(chunk) ? chunk : Buffer.from(chunk);
    total += bytes.length;
    if (total > maximumBytes) fail("worker input exceeds its byte bound");
    chunks.push(bytes);
  }
  if (total === 0) fail("worker input is required on standard input");
  return Buffer.concat(chunks);
}

function writeHeartbeat(runDirectory, record) {
  const validation = validateHeartbeat(record);
  if (!validation.ok) fail(validation.errors.join("; "));
  atomicWriteJson(path.join(runDirectory, "heartbeat.json"), record, { replace: true });
}

function elapsedSeconds(startedAt, now = Date.now()) {
  return Math.max(0, Math.floor((now - startedAt) / 1000));
}

function processGroupIsActive(pid) {
  if (!positiveInteger(pid)) return false;
  try {
    process.kill(-pid, 0);
    return true;
  } catch (error) {
    return error?.code === "EPERM";
  }
}

function signalProcessGroup(child, signal) {
  try {
    process.kill(-child.pid, signal);
  } catch (error) {
    if (error?.code !== "ESRCH") {
      try { child.kill(signal); } catch {}
    }
  }
}

function wait(milliseconds) {
  return new Promise((resolve) => setTimeout(resolve, milliseconds));
}

export async function runValidatedSupervisor(envelope, envelopeFile, input, {
  repositoryRoot = root,
  spawnWorker = spawn,
  heartbeatIntervalMilliseconds = policy.supervisor.heartbeatIntervalSeconds * 1000,
  terminationGraceMilliseconds = policy.supervisor.terminationGraceSeconds * 1000,
} = {}) {
  const runDirectory = path.dirname(envelopeFile);
  const handoffFile = path.join(runDirectory, "handoff.json");
  const eventFile = path.join(runDirectory, "events.jsonl");
  const errorFile = path.join(runDirectory, "worker.stderr");
  if (existsSync(eventFile) || existsSync(errorFile)) fail("supervisor run artifacts already exist");
  const suppliedInput = await readBoundedStdin(input, policy.supervisor.maximumPromptBytes);
  const contract = workerContract(envelope);
  if (contract.length + suppliedInput.length > policy.supervisor.maximumPromptBytes) {
    suppliedInput.fill(0);
    fail("worker input plus supervisor contract exceeds its byte bound");
  }
  const workerInput = Buffer.concat([contract, suppliedInput]);
  contract.fill(0);
  suppliedInput.fill(0);
  const eventDescriptor = openSync(eventFile, "wx", 0o600);
  const errorDescriptor = openSync(errorFile, "wx", 0o600);
  const launch = buildPiLaunch(envelope, envelopeFile, { repositoryRoot });
  const workerEnvironment = { ...process.env };
  for (const name of [
    "PI_SESSION_ID", "PI_SESSION_FILE", "PI_PROVIDER", "PI_MODEL", "PI_REASONING_LEVEL",
    "GH_TOKEN", "GITHUB_TOKEN", "GH_ENTERPRISE_TOKEN", "GITHUB_ENTERPRISE_TOKEN",
  ]) delete workerEnvironment[name];
  workerEnvironment.SDK_FACTORY_INVOCATION = envelopeFile;
  workerEnvironment.SDK_FACTORY_HANDOFF = handoffFile;
  workerEnvironment.SDK_FACTORY_EVENT_STREAM = "1";
  workerEnvironment.GH_CONFIG_DIR = path.join(runDirectory, "gh-config");
  workerEnvironment.GIT_CONFIG_COUNT = "2";
  workerEnvironment.GIT_CONFIG_KEY_0 = "credential.helper";
  workerEnvironment.GIT_CONFIG_VALUE_0 = "";
  workerEnvironment.GIT_CONFIG_KEY_1 = "remote.origin.pushurl";
  workerEnvironment.GIT_CONFIG_VALUE_1 = "https://127.0.0.1/sdk-rust-worker-push-disabled";
  const previousUmask = process.umask(0o077);
  let child;
  try {
    child = spawnWorker(launch.command, launch.args, {
      cwd: launch.cwd,
      detached: true,
      env: workerEnvironment,
      stdio: ["pipe", eventDescriptor, errorDescriptor],
    });
  } catch (error) {
    closeSync(eventDescriptor);
    closeSync(errorDescriptor);
    workerInput.fill(0);
    throw error;
  } finally {
    process.umask(previousUmask);
  }
  const startedAt = Date.now();
  let terminationReason = null;
  let terminating = false;
  let postArtifactTimer = null;
  let terminationTimer = null;
  const heartbeat = (state, processActive, workerExitCode = null) => writeHeartbeat(runDirectory, {
    schemaVersion: 1,
    state,
    observedAt: new Date().toISOString(),
    elapsedSeconds: elapsedSeconds(startedAt),
    processActive,
    workerExitCode,
    terminationReason,
  });
  heartbeat("running", true);

  const beginTermination = (reason) => {
    if (terminating) return;
    terminating = true;
    terminationReason = reason;
    heartbeat("terminating", true);
    signalProcessGroup(child, "SIGTERM");
    terminationTimer = setTimeout(() => {
      if (processGroupIsActive(child.pid)) signalProcessGroup(child, "SIGKILL");
    }, terminationGraceMilliseconds);
    terminationTimer.unref();
  };

  const interval = setInterval(() => {
    if (!terminating) heartbeat("running", true);
  }, Math.max(50, heartbeatIntervalMilliseconds));
  const handoffPoll = setInterval(() => {
    if (existsSync(handoffFile) && postArtifactTimer === null && !terminating) {
      postArtifactTimer = setTimeout(
        () => beginTermination("post-artifact-grace"),
        envelope.postArtifactGraceSeconds * 1000,
      );
      postArtifactTimer.unref();
    }
  }, 50);
  const deadline = setTimeout(() => beginTermination("deadline"), envelope.deadlineSeconds * 1000);
  deadline.unref();

  const exit = await new Promise((resolve) => {
    let settled = false;
    const settle = (result) => {
      if (!settled) {
        settled = true;
        resolve(result);
      }
    };
    child.once("error", () => settle({ code: null, signal: null }));
    child.once("exit", (code, signal) => settle({ code, signal }));
    child.stdin.once("error", () => {});
    child.stdin.end(workerInput, () => workerInput.fill(0));
  });
  workerInput.fill(0);
  clearInterval(interval);
  clearInterval(handoffPoll);
  clearTimeout(deadline);
  if (postArtifactTimer !== null) clearTimeout(postArtifactTimer);
  if (terminationTimer !== null) clearTimeout(terminationTimer);

  if (processGroupIsActive(child.pid)) {
    signalProcessGroup(child, "SIGTERM");
    await wait(terminationGraceMilliseconds);
    if (processGroupIsActive(child.pid)) signalProcessGroup(child, "SIGKILL");
  }
  closeSync(eventDescriptor);
  closeSync(errorDescriptor);
  let sessionArtifacts = "ready";
  try {
    hardenPrivateSessionTree(path.join(runDirectory, "session"), runDirectory);
  } catch {
    sessionArtifacts = "unsafe";
  }
  const workerExitCode = nonnegativeInteger(exit.code) && exit.code <= 255 ? exit.code : null;
  heartbeat(terminationReason === null ? "exited" : "timed-out", false, workerExitCode);

  let handoffStatus = "missing";
  try {
    const handoff = await validateHandoffFile(envelopeFile, {
      repositoryRoot,
      processGroupActive: processGroupIsActive(child.pid),
    });
    handoffStatus = handoff.ok && handoff.accepted ? "accepted" : "rejected";
  } catch {
    handoffStatus = existsSync(handoffFile) ? "rejected" : "missing";
  }
  return {
    worker: terminationReason === null
      ? (workerExitCode === 0 ? "exited-successfully" : "exited-unsuccessfully")
      : "timed-out",
    handoff: handoffStatus,
    sessionArtifacts,
    accepted: terminationReason === null && workerExitCode === 0 && handoffStatus === "accepted" && sessionArtifacts === "ready",
  };
}

export async function runSupervisor(envelopeFile, input, options = {}) {
  const repositoryRoot = options.repositoryRoot ?? root;
  const envelope = await validateInvocationFile(envelopeFile, { repositoryRoot });
  return runValidatedSupervisor(envelope, envelopeFile, input, options);
}

function argument(argv, name, { required = true, multiple = false } = {}) {
  const values = [];
  for (let index = 0; index < argv.length; index += 1) {
    if (argv[index] === name && argv[index + 1]) values.push(argv[index + 1]);
  }
  if (required && values.length === 0) fail(`${name} is required`);
  return multiple ? values : (values.at(-1) ?? null);
}

function assertKnownArguments(argv, valueArguments) {
  const known = new Set(valueArguments);
  for (let index = 0; index < argv.length; index += 2) {
    if (!known.has(argv[index]) || !argv[index + 1]) fail(`unknown or incomplete supervisor argument at position ${index + 1}`);
  }
}

async function main() {
  const [command, ...argv] = process.argv.slice(2);
  if (command === "prepare") {
    const valueArguments = [
      "--change", "--issue", "--profile", "--role", "--task", "--allow-path",
      "--tool", "--deadline-seconds", "--post-artifact-grace-seconds",
    ];
    assertKnownArguments(argv, valueArguments);
    const prepared = await prepareInvocation({
      change: argument(argv, "--change"),
      issue: Number(argument(argv, "--issue")),
      profile: argument(argv, "--profile"),
      role: argument(argv, "--role"),
      task: argument(argv, "--task"),
      allowedPaths: argument(argv, "--allow-path", { multiple: true }),
      tools: argument(argv, "--tool", { multiple: true }),
      deadlineSeconds: Number(argument(argv, "--deadline-seconds")),
      postArtifactGraceSeconds: Number(argument(argv, "--post-artifact-grace-seconds")),
    });
    process.stdout.write(`${prepared.envelopeFile}\n`);
  } else if (command === "validate") {
    assertKnownArguments(argv, ["--envelope"]);
    const envelopeFile = argument(argv, "--envelope");
    await validateInvocationFile(envelopeFile);
    process.stdout.write("factory: supervisor invocation is valid\n");
  } else if (command === "run") {
    assertKnownArguments(argv, ["--envelope"]);
    const result = await runSupervisor(argument(argv, "--envelope"), process.stdin);
    process.stdout.write(`${JSON.stringify({ worker: result.worker, handoff: result.handoff, sessionArtifacts: result.sessionArtifacts })}\n`);
    if (!result.accepted) process.exitCode = 2;
  } else if (command === "handoff-validate") {
    assertKnownArguments(argv, ["--envelope"]);
    const result = await validateHandoffFile(argument(argv, "--envelope"));
    if (!result.ok) fail(result.errors.join("; "));
    process.stdout.write(`factory: worker handoff is ${result.accepted ? "accepted" : "valid but not accepted"}\n`);
    if (!result.accepted) process.exitCode = 2;
  } else if (command === "harvest") {
    assertKnownArguments(argv, ["--envelope"]);
    const { harvestRun } = await import("./pi-session-harvest.mjs");
    const aggregate = await harvestRun(argument(argv, "--envelope"));
    process.stdout.write(`${JSON.stringify(aggregate)}\n`);
  } else {
    fail("Usage: supervisor.mjs <prepare|validate|run|handoff-validate|harvest> [options]");
  }
}

if (path.resolve(process.argv[1] ?? "") === fileURLToPath(import.meta.url)) {
  main().catch((error) => {
    process.stderr.write(`[factory-supervisor] ${error.message}\n`);
    process.exitCode = 1;
  });
}
