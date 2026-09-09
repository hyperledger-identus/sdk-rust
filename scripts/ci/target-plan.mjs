#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0

import { execFileSync } from "node:child_process";
import { writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");
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

export function buildPlan({ baseSha, headSha, paths, profile = "production-ready" }) {
  if (!shaPattern.test(baseSha) || !shaPattern.test(headSha)) throw new Error("plan requires exact base and head SHAs");
  if (!Array.isArray(paths) || paths.length > 2000) throw new Error("changed path set is invalid or exceeds 2,000 entries");
  if (!["prototype", "production-ready", "integration"].includes(profile)) throw new Error(`unsupported delivery profile: ${profile}`);
  const areas = classifyPaths(paths);
  const slow = new Set();
  if (areas.includes("unknown") || areas.some((area) => ["factory", "ci", "build"].includes(area))) {
    for (const target of allSlow) slow.add(target);
  }
  if (areas.some((area) => ["rust", "bindings", "build"].includes(area))) slow.add("portable-targets");
  if (areas.some((area) => ["security", "rust", "bindings"].includes(area))) slow.add("security");
  if (areas.some((area) => ["security", "rust"].includes(area))) slow.add("fuzz-conformance");
  return {
    schemaVersion: 1,
    repository: "hyperledger-identus/sdk-rust",
    baseSha,
    headSha,
    profile,
    changedPathCount: paths.length,
    paths,
    areas,
    requiredPullRequestChecks: profile === "prototype" ? ["factory-basic"] : ["fast"],
    slowRecommended: [...slow].sort(),
    slowPolicy: "weekly-or-manual",
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
  const plan = buildPlan({ baseSha, headSha, paths, profile: options.profile ?? "production-ready" });
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
