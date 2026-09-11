#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0

import assert from "node:assert/strict";
import { execFileSync, spawn } from "node:child_process";
import {
  chmodSync,
  lstatSync,
  mkdtempSync,
  mkdirSync,
  readFileSync,
  realpathSync,
  rmSync,
  symlinkSync,
  writeFileSync,
} from "node:fs";
import os from "node:os";
import path from "node:path";
import { Readable } from "node:stream";
import { test } from "node:test";
import { parseConventionalSubject, validateBranchName, validateHostedCommits, validatePullRequest } from "../ci/contribution-policy.mjs";
import { buildPlan, classifyPaths } from "../ci/target-plan.mjs";
import { checkUserPolicy, mergePolicy, policyMismatches } from "../factory-tools/pi-policy.mjs";
import { auditPi } from "../factory-tools/audit-pi.mjs";
import {
  metricTemplate,
  readMetricFile,
  renderMetric,
  selectOwnedMetricComment,
  validateMetric,
} from "../factory-tools/metrics.mjs";
import { harvestPiUsage, validateUsageAggregate } from "../factory-tools/pi-session-harvest.mjs";
import { parseJsonWithoutDuplicates } from "../factory-tools/strict-json.mjs";
import {
  assertDeclaredTask,
  buildPiLaunch,
  hardenPrivateSessionTree,
  normalizeRelativePath,
  readBoundedPrivateJsonFile,
  runValidatedSupervisor,
  validateHandoffRecord,
  validateHeartbeat,
  validateInvocationEnvelope,
} from "../factory-tools/supervisor.mjs";
import {
  packageCacheIdentity,
  parseExactNpmSource,
  preparePiPackageCache,
} from "../factory-tools/pi-package-cache.mjs";
import { validatePlanningPaths } from "../factory-tools/preflight.mjs";
import { isWithinManagedRoot, parseWorktrees } from "../worktree-lifecycle.mjs";

const sha = "a".repeat(40);
const metric = {
  schemaVersion: 1,
  repository: "hyperledger-identus/sdk-rust",
  issue: 243,
  pullRequest: null,
  headSha: sha,
  profile: "production-ready",
  startedAt: "2026-09-09T00:00:00.000Z",
  completedAt: "2026-09-09T00:01:00.000Z",
  durationSeconds: 60,
  retryCount: 0,
  toolCalls: 7,
  inputTokens: null,
  outputTokens: null,
  coveragePercent: null,
  diskBytes: 1024,
  outcome: "merged",
};

const measured = (value) => ({ value, unavailableReason: null });
const unavailable = (unavailableReason = "not-measured") => ({ value: null, unavailableReason });
const invocation = {
  schemaVersion: 1,
  repository: "hyperledger-identus/sdk-rust",
  issue: 243,
  profile: "production-ready",
  baseRef: "origin/develop",
  baseSha: sha,
  branch: "codex/feat/issue-243",
  headSha: "b".repeat(40),
  change: "bounded-change",
  contractHeadSha: "c".repeat(40),
  receiptSha256: "d".repeat(64),
  role: "developer",
  task: "2.1",
  allowedPaths: ["docs/factory", "scripts/factory-tools"],
  tools: ["read", "grep", "find", "ls", "bash", "edit", "write"],
  deadlineSeconds: 60,
  postArtifactGraceSeconds: 5,
  preparedAt: "2026-09-09T00:00:00.000Z",
};
const invocationIdentity = Object.fromEntries([
  "repository", "issue", "baseRef", "baseSha", "branch", "headSha", "change",
  "contractHeadSha", "receiptSha256",
].map((key) => [key, invocation[key]]));
const metricV2 = {
  schemaVersion: 2,
  repository: "hyperledger-identus/sdk-rust",
  issue: 243,
  pullRequest: null,
  headSha: sha,
  profile: "production-ready",
  startedAt: "2026-09-09T00:00:00.000Z",
  completedAt: "2026-09-09T00:10:00.000Z",
  durationSeconds: measured(600),
  phaseDurationsSeconds: {
    planning: measured(100),
    implementation: measured(300),
    review: measured(50),
    validation: measured(50),
    hostedCi: measured(100),
    blocked: measured(0),
  },
  ci: {
    queueSeconds: measured(5),
    executionSeconds: measured(30),
    failedAttempts: measured(1),
    canceledAttempts: measured(0),
    retryCount: measured(1),
    postCiPushCount: measured(0),
    checks: [
      { name: "fast", attempt: 1, outcome: "failed", queueSeconds: measured(2), executionSeconds: measured(10) },
      { name: "fast", attempt: 2, outcome: "passed", queueSeconds: measured(3), executionSeconds: measured(20) },
    ],
  },
  runtime: {
    sessions: measured(1),
    turns: measured(3),
    toolCalls: measured(7),
    inputTokens: measured(100),
    outputTokens: measured(20),
    cacheReadTokens: measured(80),
    cacheWriteTokens: measured(5),
  },
  coveragePercent: unavailable("not-applicable"),
  resources: {
    worktreePeakBytes: measured(1000),
    targetPeakBytes: measured(2000),
    cachePeakBytes: measured(3000),
    processPeakRssBytes: measured(4000),
  },
  outcome: "merged",
};

test("contribution metadata binds conventional type, scope and issue branch", () => {
  assert.equal(parseConventionalSubject("feat(factory): add bounded runtime").ok, true);
  assert.equal(parseConventionalSubject("feat(unknown): add runtime").ok, false);
  assert.deepEqual(validateBranchName("codex/feat/issue-243").issue, 243);
  assert.equal(validatePullRequest({
    title: "feat(factory): add bounded runtime",
    branch: "codex/feat/issue-243",
    body: "Closes #243",
  }).ok, true);
  assert.equal(validatePullRequest({
    title: "fix(factory): wrong type",
    branch: "codex/feat/issue-243",
    body: "Closes #243",
  }).ok, false);
});

test("hosted commit evidence fails closed on invalid verification and head", () => {
  const record = {
    sha,
    message: "feat(factory): add bounded runtime\n\nSigned-off-by: Agent <agent@example.com>",
    authorName: "Agent",
    authorEmail: "agent@example.com",
    verification: { verified: true, reason: "valid", signature: "-----BEGIN PGP SIGNATURE-----\nfixture" },
  };
  assert.equal(validateHostedCommits([record], sha).ok, true);
  assert.equal(validateHostedCommits([{ ...record, verification: { verified: false, reason: "unsigned" } }], sha).ok, false);
  assert.equal(validateHostedCommits([record], "b".repeat(40)).ok, false);
});

test("target plan keeps one fast PR gate and routes risk to slow evidence", () => {
  assert.deepEqual(classifyPaths(["crates/identus-crypto/src/lib.rs"]), ["rust"]);
  const plan = buildPlan({ baseSha: sha, headSha: "b".repeat(40), paths: ["nix/devshells/default.nix"], profile: "production-ready" });
  assert.deepEqual(plan.requiredPullRequestChecks, ["fast"]);
  assert.deepEqual(plan.slowRecommended, ["full-nix-linux", "full-nix-macos", "fuzz-conformance", "portable-targets", "security"]);
  assert.equal(buildPlan({ baseSha: sha, headSha: "b".repeat(40), paths: ["unclassified.bin"] }).unknownDiffFailsClosed, true);
});

test("Pi policy merge preserves unrelated user choices", () => {
  const current = { provider: "personal", nested: { keep: true, bounded: false } };
  const required = { nested: { bounded: true }, limit: 2 };
  assert.deepEqual(mergePolicy(current, required), { provider: "personal", nested: { keep: true, bounded: true }, limit: 2 });
  assert.deepEqual(policyMismatches(current, required).map((item) => item.field), ["nested.bounded", "limit"]);
});

test("Pi policy refuses a symlinked user configuration", async () => {
  const directory = mkdtempSync(path.join(os.tmpdir(), "sdk-rust-pi-policy-"));
  try {
    const extension = path.join(directory, "extensions", "subagent");
    mkdirSync(extension, { recursive: true });
    const target = path.join(directory, "elsewhere.json");
    writeFileSync(target, "{}\n");
    symlinkSync(target, path.join(extension, "config.json"));
    await assert.rejects(checkUserPolicy({ env: { PI_CODING_AGENT_DIR: directory } }), /regular JSON file/u);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});

test("tracked Pi configuration is internally bounded without inspecting credentials", async () => {
  assert.equal((await auditPi({ configOnly: true, enforceConfig: false })).ok, true);
});

const piCacheInputs = {
  piVersion: "0.84.2",
  nodeVersion: "v24.19.0",
  npmVersion: "11.17.0",
  packageLockSha256: "c".repeat(64),
  packages: ["npm:dev-loops@0.9.0", "npm:typebox@1.3.9"],
};

function installPiCacheFixture(stagingPath, identity) {
  writeFileSync(path.join(stagingPath, "package.json"), "{\"private\":true}\n");
  for (const source of identity.inputs.packages) {
    const parsed = parseExactNpmSource(source);
    const packagePath = path.join(stagingPath, "node_modules", parsed.name);
    mkdirSync(packagePath, { recursive: true });
    writeFileSync(
      path.join(packagePath, "package.json"),
      `${JSON.stringify({ name: parsed.name, version: parsed.version })}\n`,
    );
  }
}

function makePiCacheFixture() {
  const directory = mkdtempSync(path.join(os.tmpdir(), "sdk-rust-pi-cache-"));
  const primary = path.join(directory, "sdk-rust");
  const second = path.join(directory, "issue-2");
  for (const worktree of [primary, second]) mkdirSync(path.join(worktree, ".pi"), { recursive: true });
  return {
    directory,
    primary,
    second,
    context(repositoryRoot) {
      return { repositoryRoot, primaryWorktree: primary, worktrees: [primary, second] };
    },
  };
}

test("Pi cache identity is exact, stable and separates changed inputs", () => {
  const first = packageCacheIdentity(piCacheInputs);
  const second = packageCacheIdentity({ ...piCacheInputs, npmVersion: "11.17.1" });
  const changedLock = packageCacheIdentity({ ...piCacheInputs, packageLockSha256: "d".repeat(64) });
  assert.equal(first.digest, packageCacheIdentity({ ...piCacheInputs }).digest);
  assert.notEqual(first.digest, second.digest);
  assert.notEqual(first.digest, changedLock.digest);
  assert.throws(() => packageCacheIdentity({ ...piCacheInputs, packages: ["npm:typebox@^1.3.9"] }), /exact npm version/u);
  assert.throws(
    () => packageCacheIdentity({ ...piCacheInputs, packages: ["npm:typebox@1.3.9", "npm:typebox@1.3.9"] }),
    /duplicate package/u,
  );
  assert.throws(() => packageCacheIdentity({ ...piCacheInputs, nodeVersion: "v24" }), /exact Node version/u);
});

test("two worktrees converge on one complete external Pi package cache", () => {
  const fixture = makePiCacheFixture();
  try {
    const first = preparePiPackageCache({
      repositoryRoot: fixture.primary,
      inputs: piCacheInputs,
      gitContext: fixture.context(fixture.primary),
      install: installPiCacheFixture,
    });
    const second = preparePiPackageCache({
      repositoryRoot: fixture.second,
      inputs: piCacheInputs,
      gitContext: fixture.context(fixture.second),
      install: installPiCacheFixture,
    });
    assert.equal(first.layout.cachePath, second.layout.cachePath);
    assert.equal(first.publication.populated, true);
    assert.equal(second.publication.populated, false);
    assert.equal(first.link.linkTarget, second.link.linkTarget);
    assert.equal(first.layout.cachePath.startsWith(`${fixture.primary}${path.sep}`), false);
    assert.equal(first.layout.cachePath.startsWith(`${fixture.second}${path.sep}`), false);
  } finally {
    rmSync(fixture.directory, { recursive: true, force: true });
  }
});

test("concurrent Pi cache initializers expose only one complete identity", () => {
  const fixture = makePiCacheFixture();
  try {
    let competitor;
    const winnerInstall = (stagingPath, identity) => {
      installPiCacheFixture(stagingPath, identity);
      competitor = preparePiPackageCache({
        repositoryRoot: fixture.second,
        inputs: piCacheInputs,
        gitContext: fixture.context(fixture.second),
        install: installPiCacheFixture,
      });
    };
    const converged = preparePiPackageCache({
      repositoryRoot: fixture.primary,
      inputs: piCacheInputs,
      gitContext: fixture.context(fixture.primary),
      install: winnerInstall,
    });
    assert.equal(competitor.publication.populated, true);
    assert.equal(converged.publication.converged, true);
    assert.equal(converged.layout.cachePath, competitor.layout.cachePath);
  } finally {
    rmSync(fixture.directory, { recursive: true, force: true });
  }
});

test("Pi cache refuses operator data, unexpected links and symlinked storage", () => {
  for (const unsafe of ["directory", "wrong-link", "storage-link"]) {
    const fixture = makePiCacheFixture();
    try {
      if (unsafe === "directory") mkdirSync(path.join(fixture.primary, ".pi", "npm"));
      if (unsafe === "wrong-link") symlinkSync(fixture.second, path.join(fixture.primary, ".pi", "npm"));
      if (unsafe === "storage-link") {
        const outside = path.join(fixture.directory, "outside");
        mkdirSync(outside);
        symlinkSync(outside, path.join(fixture.directory, ".sdk-rust-factory"));
      }
      assert.throws(
        () => preparePiPackageCache({
          repositoryRoot: fixture.primary,
          inputs: piCacheInputs,
          gitContext: fixture.context(fixture.primary),
          install: installPiCacheFixture,
        }),
        unsafe === "storage-link" ? /regular directories only/u : /refusing to replace|unexpected cache/u,
      );
    } finally {
      rmSync(fixture.directory, { recursive: true, force: true });
    }
  }
});

test("ignored Pi project link leaves Git status clean", () => {
  const fixture = makePiCacheFixture();
  try {
    writeFileSync(path.join(fixture.primary, ".gitignore"), "/.pi/npm\n");
    writeFileSync(path.join(fixture.primary, ".pi", "settings.json"), "{}\n");
    execFileSync("git", ["init", "-b", "main"], { cwd: fixture.primary });
    execFileSync("git", ["config", "user.name", "Factory Test"], { cwd: fixture.primary });
    execFileSync("git", ["config", "user.email", "factory@example.invalid"], { cwd: fixture.primary });
    execFileSync("git", ["add", ".gitignore", ".pi/settings.json"], { cwd: fixture.primary });
    execFileSync("git", ["commit", "-m", "fixture"], { cwd: fixture.primary });
    preparePiPackageCache({
      repositoryRoot: fixture.primary,
      inputs: piCacheInputs,
      gitContext: fixture.context(fixture.primary),
      install: installPiCacheFixture,
    });
    assert.equal(execFileSync("git", ["status", "--porcelain"], { cwd: fixture.primary, encoding: "utf8" }), "");
  } finally {
    rmSync(fixture.directory, { recursive: true, force: true });
  }
});

test("strict private JSON rejects duplicates, oversize and symlinks without echoing content", () => {
  assert.equal(parseJsonWithoutDuplicates('{"safe":1}').safe, 1);
  assert.throws(() => parseJsonWithoutDuplicates('{"safe":1,"safe":2}'), /duplicate field/u);
  const created = mkdtempSync(path.join(os.tmpdir(), "sdk-rust-private-json-"));
  const directory = realpathSync(created);
  try {
    chmodSync(directory, 0o700);
    const record = path.join(directory, "record.json");
    writeFileSync(record, '{"safe":1}\n', { mode: 0o600 });
    assert.equal(readBoundedPrivateJsonFile(record, { boundary: directory, maximumBytes: 64, ownerOnly: true }).value.safe, 1);
    assert.throws(
      () => readBoundedPrivateJsonFile(record, { boundary: directory, maximumBytes: 4, ownerOnly: true }),
      /byte bound/u,
    );
    const linked = path.join(directory, "linked.json");
    symlinkSync(record, linked);
    assert.throws(
      () => readBoundedPrivateJsonFile(linked, { boundary: directory, maximumBytes: 64, ownerOnly: true }),
      /symbolic link/u,
    );
    writeFileSync(record, '{"private-fixture":1,"private-fixture":2}\n');
    assert.throws(
      () => readBoundedPrivateJsonFile(record, { boundary: directory, maximumBytes: 128, ownerOnly: true }),
      (error) => /duplicate field/u.test(error.message) && !error.message.includes("private-fixture"),
    );
  } finally {
    rmSync(created, { recursive: true, force: true });
  }
});

test("supervisor invocation, heartbeat and handoff are closed and exact-identity bound", () => {
  assert.deepEqual(validateInvocationEnvelope(invocation, { identity: invocationIdentity }), { ok: true, errors: [] });
  assert.equal(validateInvocationEnvelope({ ...invocation, unexpected: true }, { identity: invocationIdentity }).ok, false);
  assert.equal(validateInvocationEnvelope(invocation, { identity: { ...invocationIdentity, headSha: "e".repeat(40) } }).ok, false);
  assert.equal(validateInvocationEnvelope({ ...invocation, role: "reviewer", tools: ["read", "bash"] }).ok, false);
  assert.throws(() => normalizeRelativePath("docs/../outside"), /normalized/u);
  assert.throws(() => normalizeRelativePath("/outside"), /repository-relative/u);

  const preparedHeartbeat = {
    schemaVersion: 1,
    state: "prepared",
    observedAt: "2026-09-09T00:00:00.000Z",
    elapsedSeconds: 0,
    processActive: false,
    workerExitCode: null,
    terminationReason: null,
  };
  assert.equal(validateHeartbeat(preparedHeartbeat).ok, true);
  assert.equal(validateHeartbeat({ ...preparedHeartbeat, processActive: true }).ok, false);
  assert.equal(validateHeartbeat({ ...preparedHeartbeat, transcript: true }).ok, false);

  const handoff = {
    schemaVersion: 1,
    repository: invocation.repository,
    issue: invocation.issue,
    change: invocation.change,
    baseSha: invocation.baseSha,
    branch: invocation.branch,
    headSha: invocation.headSha,
    contractHeadSha: invocation.contractHeadSha,
    receiptSha256: invocation.receiptSha256,
    role: invocation.role,
    task: invocation.task,
    status: "checkpoint",
    changedPaths: ["docs/factory/metrics.md"],
    acceptance: [{ id: "bounded-run", outcome: "passed" }],
    checks: [{ id: "focused-node", outcome: "passed", durationSeconds: 3 }],
    findings: [{ class: "must-fix", disposition: "resolved" }],
    processOwnership: { workerOwnedRemaining: 0 },
    nextAction: "supervisor-review",
    completedAt: "2026-09-09T00:01:00.000Z",
  };
  const accepted = validateHandoffRecord(handoff, {
    envelope: invocation,
    currentIdentity: invocationIdentity,
    actualChangedPaths: ["docs/factory/metrics.md"],
  });
  assert.equal(accepted.ok, true);
  assert.equal(accepted.accepted, true);
  assert.equal(validateHandoffRecord({ ...handoff, nextAction: "merge" }, {
    envelope: invocation,
    currentIdentity: invocationIdentity,
    actualChangedPaths: handoff.changedPaths,
  }).ok, false);
  assert.equal(validateHandoffRecord({ ...handoff, changedPaths: ["Cargo.toml"] }, {
    envelope: invocation,
    currentIdentity: invocationIdentity,
    actualChangedPaths: ["Cargo.toml"],
  }).ok, false);
  assert.equal(validateHandoffRecord(handoff, {
    envelope: invocation,
    currentIdentity: { ...invocationIdentity, headSha: "e".repeat(40) },
    actualChangedPaths: handoff.changedPaths,
  }).ok, false);
  assert.equal(validateHandoffRecord({ ...handoff, processOwnership: { workerOwnedRemaining: 1 } }, {
    envelope: invocation,
    currentIdentity: invocationIdentity,
    actualChangedPaths: handoff.changedPaths,
  }).accepted, false);
});

test("supervisor task identity must exist in the active OpenSpec checklist", () => {
  const temporary = mkdtempSync(path.join(os.tmpdir(), "sdk-rust-supervisor-task-"));
  const created = realpathSync(temporary);
  try {
    const taskRoot = path.join(created, "openspec", "changes", "bounded-change");
    mkdirSync(taskRoot, { recursive: true });
    writeFileSync(path.join(taskRoot, "tasks.md"), "- [ ] 2.1 First task\n- [x] 2.2 Completed task\n");
    assert.doesNotThrow(() => assertDeclaredTask(created, "bounded-change", "2.1"));
    assert.doesNotThrow(() => assertDeclaredTask(created, "bounded-change", "2.2"));
    assert.throws(() => assertDeclaredTask(created, "bounded-change", "9.9"), /not declared/u);
  } finally {
    rmSync(created, { recursive: true, force: true });
  }
});

test("supervisor launch is bootstrap-only and times out independently of handoff", async () => {
  const created = mkdtempSync(path.join(os.tmpdir(), "sdk-rust-supervisor-"));
  const directory = realpathSync(created);
  try {
    chmodSync(directory, 0o700);
    mkdirSync(path.join(directory, "session"), { mode: 0o700 });
    const envelopeFile = path.join(directory, "invocation.json");
    const timedInvocation = { ...invocation, deadlineSeconds: 1, postArtifactGraceSeconds: 1 };
    const launch = buildPiLaunch(timedInvocation, envelopeFile, { repositoryRoot: path.resolve(".") });
    assert.equal(launch.command, path.join(path.resolve("."), "bootstrap.sh"));
    assert.equal(launch.args[0], "--pi");
    assert.equal(launch.args.includes("--no-session"), false);
    assert.equal(launch.args[launch.args.indexOf("--tools") + 1], timedInvocation.tools.join(","));
    let observedCommand;
    let observedEnvironment;
    const spawnWorker = (command, _args, options) => {
      observedCommand = command;
      observedEnvironment = options.env;
      return spawn(process.execPath, ["-e", "setInterval(() => {}, 1000)"], options);
    };
    const result = await runValidatedSupervisor(
      timedInvocation,
      envelopeFile,
      Readable.from([Buffer.from("private worker input")]),
      {
        repositoryRoot: path.resolve("."),
        spawnWorker,
        heartbeatIntervalMilliseconds: 20,
        terminationGraceMilliseconds: 25,
      },
    );
    assert.equal(observedCommand, path.join(path.resolve("."), "bootstrap.sh"));
    assert.equal(observedEnvironment.SDK_FACTORY_EVENT_STREAM, "1");
    assert.equal(observedEnvironment.GH_CONFIG_DIR, path.join(directory, "gh-config"));
    assert.equal(observedEnvironment.GITHUB_TOKEN, undefined);
    assert.match(observedEnvironment.GIT_CONFIG_VALUE_1, /push-disabled/u);
    assert.deepEqual(result, { worker: "timed-out", handoff: "missing", sessionArtifacts: "ready", accepted: false });
    const heartbeat = parseJsonWithoutDuplicates(readFileSync(path.join(directory, "heartbeat.json"), "utf8"));
    assert.equal(heartbeat.state, "timed-out");
    assert.equal(heartbeat.processActive, false);
    assert.equal(heartbeat.terminationReason, "deadline");
    const retained = [
      readFileSync(path.join(directory, "heartbeat.json"), "utf8"),
      readFileSync(path.join(directory, "events.jsonl"), "utf8"),
      readFileSync(path.join(directory, "worker.stderr"), "utf8"),
    ].join("\n");
    assert.equal(retained.includes("private worker input"), false);
  } finally {
    rmSync(created, { recursive: true, force: true });
  }
});

test("supervisor hardens Pi-created session files and rejects links", () => {
  const temporary = mkdtempSync(path.join(os.tmpdir(), "sdk-rust-session-tree-"));
  const directory = realpathSync(temporary);
  try {
    chmodSync(directory, 0o700);
    const session = path.join(directory, "session");
    mkdirSync(session, { mode: 0o700 });
    const sessionFile = path.join(session, "session.jsonl");
    writeFileSync(sessionFile, "{}\n", { mode: 0o644 });
    hardenPrivateSessionTree(session, directory);
    assert.equal((lstatSync(sessionFile).mode & 0o077), 0);
    const linked = path.join(session, "linked.jsonl");
    symlinkSync(sessionFile, linked);
    assert.throws(() => hardenPrivateSessionTree(session, directory), /symbolic link/u);
  } finally {
    rmSync(temporary, { recursive: true, force: true });
  }
});

test("Pi v3 harvest deduplicates terminal usage and emits aggregate counters only", () => {
  const created = mkdtempSync(path.join(os.tmpdir(), "sdk-rust-harvest-"));
  const directory = realpathSync(created);
  try {
    chmodSync(directory, 0o700);
    const sessionFile = path.join(directory, "session.jsonl");
    const eventFile = path.join(directory, "events.jsonl");
    writeFileSync(sessionFile, [
      JSON.stringify({ type: "session", version: 3, id: "private-session", cwd: "/private" }),
      JSON.stringify({ type: "message", id: "private-response", message: { role: "assistant", provider: "private-provider", model: "private-model", content: [{ type: "text", text: "private-output" }], usage: { input: 10, output: 2, cacheRead: 3, cacheWrite: 4, totalTokens: 19, cost: { total: 9 } } } }),
      JSON.stringify({ type: "message", id: "private-response-2", message: { role: "assistant", content: [], usage: { input: 5, output: 1, cacheRead: 2, cacheWrite: 0 } } }),
    ].join("\n") + "\n", { mode: 0o600 });
    writeFileSync(eventFile, [
      JSON.stringify({ type: "session", version: 3, id: "private-session" }),
      JSON.stringify({ type: "turn_start" }),
      JSON.stringify({ type: "tool_execution_start", toolCallId: "private-tool", toolName: "read", args: { path: "private-command" } }),
      JSON.stringify({ type: "message_update", usage: { input: 999, output: 999 } }),
      JSON.stringify({ type: "message_end", message: { role: "assistant", usage: { input: 999, output: 999 } } }),
      JSON.stringify({ type: "tool_execution_end", toolCallId: "private-tool", result: "private-raw-output" }),
      JSON.stringify({ type: "turn_end", message: { role: "assistant", usage: { input: 999, output: 999 } } }),
    ].join("\n") + "\n", { mode: 0o600 });
    const aggregate = harvestPiUsage({ sessionFile, eventFile, runDirectory: directory });
    assert.equal(validateUsageAggregate(aggregate).ok, true);
    assert.equal(aggregate.sessions.value, 1);
    assert.equal(aggregate.turns.value, 1);
    assert.equal(aggregate.toolCalls.value, 1);
    assert.equal(aggregate.inputTokens.value, 15);
    assert.equal(aggregate.outputTokens.value, 3);
    assert.equal(aggregate.cacheReadTokens.value, 5);
    assert.equal(aggregate.cacheWriteTokens.value, 4);
    assert.equal(/private|provider|model|cost|content|command|raw-output|id/u.test(JSON.stringify(aggregate)), false);
    assert.equal(validateUsageAggregate({ ...aggregate, transcript: "private" }).ok, false);

    writeFileSync(eventFile, [
      JSON.stringify({ type: "session", version: 3, id: "private" }),
      JSON.stringify({ type: "turn_end" }),
      JSON.stringify({ type: "turn_start" }),
    ].join("\n") + "\n");
    assert.equal(harvestPiUsage({ sessionFile, eventFile, runDirectory: directory }).turns.unavailableReason, "incomplete-events");
    writeFileSync(sessionFile, `${JSON.stringify({ type: "session", version: 4, id: "private" })}\n`);
    assert.equal(harvestPiUsage({ sessionFile, eventFile, runDirectory: directory }).sessions.unavailableReason, "unsupported-version");
    writeFileSync(sessionFile, '{"type":"session","version":3,"version":3}\n');
    assert.equal(harvestPiUsage({ sessionFile, eventFile, runDirectory: directory }).sessions.unavailableReason, "source-malformed");
    assert.throws(
      () => harvestPiUsage({ sessionFile, eventFile, runDirectory: directory, maximumSessionBytes: 8 }),
      /byte bound/u,
    );
    const linked = path.join(directory, "linked.jsonl");
    symlinkSync(sessionFile, linked);
    assert.throws(
      () => harvestPiUsage({ sessionFile: linked, eventFile, runDirectory: directory }),
      /unsafe/u,
    );
  } finally {
    rmSync(created, { recursive: true, force: true });
  }
});

test("metrics v1 and v2 are closed, versioned and publication-safe", () => {
  assert.equal(validateMetric(metric).ok, true);
  assert.match(renderMetric(metric), /sdk-rust-factory-metrics:v1/u);
  assert.equal(renderMetric(metric).match(/sdk-rust-factory-metrics:v1/gu)?.length, 1);
  assert.equal(validateMetric({ ...metric, prompt: "do not publish" }).ok, false);
  assert.equal(validateMetric({ ...metric, coveragePercent: 101 }).ok, false);

  assert.equal(validateMetric(metricV2).ok, true);
  const renderedV2 = renderMetric(metricV2);
  assert.match(renderedV2, /sdk-rust-factory-metrics:v2/u);
  assert.equal(renderedV2.match(/sdk-rust-factory-metrics:v2/gu)?.length, 1);
  assert.doesNotMatch(renderedV2, /sdk-rust-factory-metrics:v1/u);
  assert.equal(validateMetric({ ...metricV2, runtime: { ...metricV2.runtime, provider: measured(1) } }).ok, false);
  assert.equal(validateMetric({ ...metricV2, durationSeconds: { value: null, unavailableReason: null } }).ok, false);
  assert.equal(validateMetric({ ...metricV2, durationSeconds: { value: 0, unavailableReason: "not-measured" } }).ok, false);
  assert.equal(validateMetric({ ...metricV2, ci: { ...metricV2.ci, retryCount: measured(0) } }).ok, false);
  assert.equal(validateMetric({ ...metricV2, ci: { ...metricV2.ci, checks: [{ name: "fast" }] } }).ok, false);
  assert.equal(validateMetric({ ...metricV2, ci: { ...metricV2.ci, checks: [...metricV2.ci.checks, metricV2.ci.checks[1]] } }).ok, false);
  assert.equal(validateMetric({ ...metricV2, ci: { ...metricV2.ci, checks: [metricV2.ci.checks[1]] } }).ok, false);
  assert.equal(validateMetric({ ...metricV2, ci: { ...metricV2.ci, checks: [{ ...metricV2.ci.checks[0], name: "secret-data" }, metricV2.ci.checks[1]] } }).ok, false);
  assert.equal(metricTemplate(243, 2, { headSha: sha }).schemaVersion, 2);
  assert.equal(metricTemplate(243, 1, { headSha: sha }).schemaVersion, 1);

  const comments = [
    { id: 1, user: { login: "factory" }, body: "<!-- sdk-rust-factory-metrics:v1:old -->" },
    { id: 2, user: { login: "factory" }, body: "<!-- sdk-rust-factory-metrics:v2:new -->" },
    { id: 3, user: { login: "other" }, body: "<!-- sdk-rust-factory-metrics:v2:other -->" },
  ];
  assert.equal(selectOwnedMetricComment(comments, "factory", "sdk-rust-factory-metrics:v1").id, 1);
  assert.equal(selectOwnedMetricComment(comments, "factory", "sdk-rust-factory-metrics:v2").id, 2);
  assert.throws(
    () => selectOwnedMetricComment([...comments, comments[1]], "factory", "sdk-rust-factory-metrics:v2"),
    /multiple owned/u,
  );
});

test("metric file ingestion rejects duplicate, oversized and symlinked payloads", () => {
  const created = mkdtempSync(path.join(os.tmpdir(), "sdk-rust-metric-input-"));
  const directory = realpathSync(created);
  try {
    const metricFile = path.join(directory, "metric.json");
    writeFileSync(metricFile, '{"schemaVersion":1,"schemaVersion":1}\n');
    assert.throws(() => readMetricFile(metricFile), /duplicate field/u);
    writeFileSync(metricFile, "x".repeat(32769));
    assert.throws(() => readMetricFile(metricFile), /32 KiB/u);
    writeFileSync(metricFile, `${JSON.stringify(metric)}\n`);
    const linked = path.join(directory, "linked.json");
    symlinkSync(metricFile, linked);
    assert.throws(() => readMetricFile(linked), /non-symlink/u);
  } finally {
    rmSync(created, { recursive: true, force: true });
  }
});

test("preflight accepts only the named planning path before implementation", () => {
  assert.doesNotThrow(() => validatePlanningPaths("bounded-change", ["openspec/changes/bounded-change/proposal.md"]));
  assert.throws(() => validatePlanningPaths("bounded-change", ["openspec/changes/bounded-change/proposal.md", "crates/core/src/lib.rs"]), /implementation changed/u);
});

test("worktree porcelain parser retains safety-relevant state", () => {
  const records = parseWorktrees(`worktree /tmp/primary\nHEAD ${sha}\nbranch refs/heads/develop\n\nworktree /tmp/issue\nHEAD ${"b".repeat(40)}\nbranch refs/heads/codex/feat/issue-243\nlocked reason\n`);
  assert.equal(records.length, 2);
  assert.equal(records[1].locked, true);
  assert.equal(records[1].branch, "codex/feat/issue-243");
  assert.equal(isWithinManagedRoot("/tmp/unmanaged", "/tmp/managed"), false);
  assert.equal(isWithinManagedRoot("/tmp/managed/issue-243", "/tmp/managed"), true);
});
