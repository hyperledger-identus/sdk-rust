#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0

import { execFileSync } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");
const factoryPolicy = Object.freeze(JSON.parse(readFileSync(path.join(root, ".factory-policy.json"), "utf8")));
const validatedPolicy = validateLanePolicy(factoryPolicy);
const ciPolicy = Object.freeze(validatedPolicy.ci);
const deliveryPolicy = Object.freeze(validatedPolicy.delivery);
const shaPattern = /^[0-9a-f]{40}$/u;
const allSlow = Object.freeze(["full-nix-linux", "full-nix-macos", "portable-targets", "security", "fuzz-conformance"]);

const routes = Object.freeze([
  ["factory", /^(?:\.factory-policy\.json|\.devloops|\.pi\/|\.githooks\/|bootstrap\.sh|docs\/factory\/|scripts\/(?:factory$|factory-tools\/|git-hooks\/|worktree-lifecycle\.mjs|loop\/)|\.github\/(?:contribution-policy\.json|ISSUE_TEMPLATE\/factory-work-item\.yml))/u],
  ["ci", /^(?:\.github\/workflows\/|scripts\/ci\/|nix\/checks\/)/u],
  ["build", /^(?:Cargo\.(?:toml|lock)|flake\.(?:nix|lock)|nix\/|deny\.toml|rust-toolchain(?:\.toml)?)/u],
  ["bindings", /^(?:tools\/uniffi|crates\/(?:uniffi|wasm)|scripts\/check-(?:uniffi|wasm))/u],
  ["security", /^(?:SECURITY\.md|deny\.toml|fuzz\/|scripts\/.*(?:secret|audit)|\.github\/workflows\/.*(?:fuzz|scan))/u],
  ["rust", /^(?:crates\/|tests\/|Cargo\.(?:toml|lock))/u],
  ["spec", /^(?:openspec\/|docs\/(?:adr|governance|architecture|roadmap)\/)/u],
  ["docs", /^(?:.*\.md|docs\/)/u],
]);

export function validateLanePolicy(policy) {
  const ci = policy?.ci;
  const delivery = policy?.delivery;
  if (!ci || !delivery) throw new Error("factory policy must define ci and delivery contracts");
  if (ci.requiredPullRequestGate !== "fast") throw new Error("the only required pull-request gate must be fast");
  if (ci.slowSchedule !== "weekly-or-manual" || ci.slow?.schedule !== ci.slowSchedule) {
    throw new Error("slow schedule must remain weekly-or-manual and internally consistent");
  }
  if (JSON.stringify(ci.fast?.requiredStatuses) !== JSON.stringify(["fast"])) {
    throw new Error("fast line must expose exactly one required status");
  }
  if (ci.fast.purpose !== "active-development-integration" || ci.fast.platform !== "x86_64-linux") {
    throw new Error("fast line identity must remain active-development integration on Linux");
  }
  const requiredFastEvidence = [
    "factory-policy",
    "openspec",
    "formatting",
    "workspace-build",
    "strict-clippy",
    "normal-tests",
    "bounded-first-party-analysis",
  ];
  const excludedFastEvidence = [
    "cross-platform-matrix",
    "mobile-runtime",
    "browser-matrix",
    "coverage",
    "performance",
    "fuzz",
    "sanitizers",
    "release-artifacts",
  ];
  if (JSON.stringify(ci.fast.includes) !== JSON.stringify(requiredFastEvidence)
      || JSON.stringify(ci.fast.excludes) !== JSON.stringify(excludedFastEvidence)) {
    throw new Error("fast evidence placement is incomplete");
  }
  const slo = ci.fast?.executionSloSeconds;
  if (![slo?.p50, slo?.p95, slo?.optimizationTrigger].every((value) => Number.isSafeInteger(value) && value > 0)
      || slo.p50 > slo.p95 || slo.p95 >= slo.optimizationTrigger) {
    throw new Error("fast execution SLO must be positive and ordered p50 <= p95 < optimization trigger");
  }
  if (delivery.maximumAutomaticReviewRounds !== 1 || delivery.maximumRemediationRounds !== 1) {
    throw new Error("delivery must allow one automatic review and one remediation round");
  }
  if (delivery.pushStrategy !== "local-first-batched"
      || delivery.reviewCutoffDisposition !== "linked-follow-up-for-independent-non-blocking-findings") {
    throw new Error("delivery iteration policy is incomplete");
  }
  const slice = delivery.sliceGuidance;
  if (slice?.changedFiles !== 12
      || slice.changedTextLines !== 1000
      || slice.thresholdAction !== "decomposition-note") {
    throw new Error("slice guidance must remain exactly 12 files, 1,000 text lines, and a decomposition note");
  }
  const requiredSlowBlockers = ["production-promotion", "publication", "release-preparation"];
  if (ci.slow?.purpose !== "production-promotion" || ci.slow.exactShaRequired !== true
      || ci.slow.unchangedCandidateRequired !== true
      || JSON.stringify(ci.slow.blocks) !== JSON.stringify(requiredSlowBlockers)
      || JSON.stringify(ci.slow.doesNotBlock) !== JSON.stringify(["ordinary-pull-request-integration"])) {
    throw new Error("slow production-promotion invariants are incomplete");
  }
  return { ci, delivery };
}

function git(args) {
  return execFileSync("git", args, { cwd: root, encoding: "utf8", maxBuffer: 16 * 1024 * 1024, stdio: ["ignore", "pipe", "pipe"] });
}

function resolveCommit(revision, label) {
  let sha;
  try { sha = git(["rev-parse", "--verify", `${revision}^{commit}`]).trim(); } catch { throw new Error(`${label} is not an available commit: ${revision}`); }
  if (!shaPattern.test(sha)) throw new Error(`${label} did not resolve to an exact SHA`);
  return sha;
}

export function classifyPaths(paths) {
  const areas = new Set();
  let unknown = false;
  for (const candidate of paths) {
    if (typeof candidate !== "string" || !candidate || candidate.length > 4096 || candidate.startsWith("/") || candidate.split("/").includes("..")) {
      unknown = true;
      continue;
    }
    const matched = routes.filter(([, pattern]) => pattern.test(candidate)).map(([area]) => area);
    if (matched.length === 0) unknown = true;
    for (const area of matched) areas.add(area);
  }
  if (paths.length === 0) unknown = true;
  if (unknown) areas.add("unknown");
  return [...areas].sort();
}

export function parseNumstat(raw) {
  if (typeof raw !== "string") throw new Error("numstat input must be a string");
  let changedTextLines = 0;
  let binaryFileCount = 0;
  for (const record of raw.split("\0").filter(Boolean)) {
    const firstTab = record.indexOf("\t");
    const secondTab = firstTab < 0 ? -1 : record.indexOf("\t", firstTab + 1);
    if (firstTab < 1 || secondTab < firstTab + 2) throw new Error("numstat record is malformed");
    const additions = record.slice(0, firstTab);
    const deletions = record.slice(firstTab + 1, secondTab);
    if (additions === "-" && deletions === "-") {
      binaryFileCount += 1;
      continue;
    }
    if (!/^\d+$/u.test(additions) || !/^\d+$/u.test(deletions)) throw new Error("numstat count is malformed");
    changedTextLines += Number(additions) + Number(deletions);
    if (!Number.isSafeInteger(changedTextLines)) throw new Error("numstat count exceeds the safe integer range");
  }
  return { changedTextLines, binaryFileCount };
}

export function buildPlan({
  baseSha,
  headSha,
  paths,
  profile = "production-ready",
  changedTextLines = 0,
  binaryFileCount = 0,
}) {
  if (!shaPattern.test(baseSha) || !shaPattern.test(headSha)) throw new Error("plan requires exact base and head SHAs");
  if (!Array.isArray(paths) || paths.length > 2000) throw new Error("changed path set is invalid or exceeds 2,000 entries");
  if (!["prototype", "production-ready", "integration"].includes(profile)) throw new Error(`unsupported delivery profile: ${profile}`);
  if (!Number.isSafeInteger(changedTextLines) || changedTextLines < 0) throw new Error("changed text lines must be a non-negative safe integer");
  if (!Number.isSafeInteger(binaryFileCount) || binaryFileCount < 0) throw new Error("binary file count must be a non-negative safe integer");
  const areas = classifyPaths(paths);
  const slow = new Set();
  if (areas.includes("unknown") || areas.some((area) => ["factory", "ci", "build"].includes(area))) {
    for (const target of allSlow) slow.add(target);
  }
  if (areas.some((area) => ["rust", "bindings", "build"].includes(area))) slow.add("portable-targets");
  if (areas.some((area) => ["security", "rust", "bindings"].includes(area))) slow.add("security");
  if (areas.some((area) => ["security", "rust"].includes(area))) slow.add("fuzz-conformance");
  const sliceGuidance = deliveryPolicy.sliceGuidance;
  const decompositionNoteRequired = paths.length > sliceGuidance.changedFiles
    || changedTextLines > sliceGuidance.changedTextLines;
  return {
    schemaVersion: 2,
    repository: "hyperledger-identus/sdk-rust",
    baseSha,
    headSha,
    profile,
    changedPathCount: paths.length,
    changedTextLines,
    binaryFileCount,
    paths,
    areas,
    requiredPullRequestChecks: profile === "prototype" ? ["factory-basic"] : [ciPolicy.requiredPullRequestGate],
    integration: profile === "prototype" ? {
      line: null,
      purpose: "provisional-local-validation",
      platform: "local",
      requiredStatuses: [],
      executionSloSeconds: null,
    } : {
      line: ciPolicy.requiredPullRequestGate,
      purpose: ciPolicy.fast.purpose,
      platform: ciPolicy.fast.platform,
      requiredStatuses: ciPolicy.fast.requiredStatuses,
      executionSloSeconds: ciPolicy.fast.executionSloSeconds,
    },
    slowRecommended: [...slow].sort(),
    slowPolicy: "native-weekly-or-manual",
    promotion: {
      line: "slow",
      purpose: ciPolicy.slow.purpose,
      exactShaRequired: ciPolicy.slow.exactShaRequired,
      unchangedCandidateRequired: ciPolicy.slow.unchangedCandidateRequired,
      blocks: ciPolicy.slow.blocks,
      readiness: "not-evaluated-by-diff-plan",
    },
    iteration: {
      pushStrategy: deliveryPolicy.pushStrategy,
      maximumAutomaticReviewRounds: deliveryPolicy.maximumAutomaticReviewRounds,
      maximumRemediationRounds: deliveryPolicy.maximumRemediationRounds,
      reviewCutoffDisposition: deliveryPolicy.reviewCutoffDisposition,
      sliceGuidance,
      decompositionNoteRequired,
    },
    unknownDiffFailsClosed: areas.includes("unknown"),
  };
}

function parseArgs(argv) {
  const values = {};
  for (let index = 0; index < argv.length; index += 1) {
    const entry = argv[index];
    if (!["--base", "--head", "--profile", "--output"].includes(entry)) throw new Error(`unknown argument: ${entry}`);
    values[entry.slice(2)] = argv[index + 1];
    index += 1;
  }
  if (!values.base || !values.head) throw new Error("Usage: target-plan.mjs --base REF --head REF [--profile NAME] [--output FILE]");
  return values;
}

function main() {
  const options = parseArgs(process.argv.slice(2));
  const baseSha = resolveCommit(options.base, "base");
  const headSha = resolveCommit(options.head, "head");
  const raw = git(["diff", "--name-only", "-z", `${baseSha}...${headSha}`]);
  const paths = raw.split("\0").filter(Boolean);
  const numstat = parseNumstat(git(["diff", "--numstat", "--no-renames", "-z", `${baseSha}...${headSha}`]));
  const plan = buildPlan({ baseSha, headSha, paths, profile: options.profile ?? "production-ready", ...numstat });
  const rendered = `${JSON.stringify(plan, null, 2)}\n`;
  if (options.output) writeFileSync(path.resolve(root, options.output), rendered, { flag: "w", mode: 0o600 });
  process.stdout.write(rendered);
}

if (path.resolve(process.argv[1] ?? "") === fileURLToPath(import.meta.url)) {
  try { main(); } catch (error) {
    process.stderr.write(`[target-plan] ${error.message}\n`);
    process.exitCode = 1;
  }
}
