#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0

import assert from "node:assert/strict";
import { mkdtempSync, mkdirSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { test } from "node:test";
import { parseConventionalSubject, validateBranchName, validateHostedCommits, validatePullRequest } from "../ci/contribution-policy.mjs";
import { buildPlan, classifyPaths } from "../ci/target-plan.mjs";
import { checkUserPolicy, mergePolicy, policyMismatches } from "../factory-tools/pi-policy.mjs";
import { auditPi } from "../factory-tools/audit-pi.mjs";
import { renderMetric, validateMetric } from "../factory-tools/metrics.mjs";
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

test("metrics schema is closed, nullable and privacy bounded", () => {
  assert.equal(validateMetric(metric).ok, true);
  assert.match(renderMetric(metric), /sdk-rust-factory-metrics:v1/u);
  assert.equal(renderMetric(metric).match(/sdk-rust-factory-metrics:v1/gu)?.length, 1);
  assert.equal(validateMetric({ ...metric, prompt: "do not publish" }).ok, false);
  assert.equal(validateMetric({ ...metric, coveragePercent: 101 }).ok, false);
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
