#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0

import { execFileSync } from "node:child_process";
import { existsSync, lstatSync, mkdirSync, readFileSync, realpathSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { validateBranchName } from "./ci/contribution-policy.mjs";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const policy = JSON.parse(readFileSync(path.join(root, ".factory-policy.json"), "utf8"));

function git(args, { cwd = root } = {}) {
  return execFileSync("git", args, { cwd, encoding: "utf8", maxBuffer: 16 * 1024 * 1024, stdio: ["ignore", "pipe", "pipe"] }).trim();
}

function fail(message) {
  throw new Error(message);
}

export function parseWorktrees(text) {
  const records = [];
  let current = null;
  for (const line of String(text).split("\n")) {
    if (line.startsWith("worktree ")) {
      if (current) records.push(current);
      current = { path: line.slice(9), head: null, branch: null, bare: false, detached: false, locked: false, prunable: false };
    } else if (current && line.startsWith("HEAD ")) current.head = line.slice(5);
    else if (current && line.startsWith("branch ")) current.branch = line.slice(7).replace(/^refs\/heads\//u, "");
    else if (current && line === "bare") current.bare = true;
    else if (current && line === "detached") current.detached = true;
    else if (current && line.startsWith("locked")) current.locked = true;
    else if (current && line.startsWith("prunable")) current.prunable = true;
  }
  if (current) records.push(current);
  return records;
}

function commonCheckoutRoot() {
  const common = path.resolve(root, git(["rev-parse", "--git-common-dir"]));
  if (path.basename(common) !== ".git") fail("managed worktrees require a normal Git common directory");
  return path.dirname(common);
}

export function managedRoot() {
  return path.resolve(commonCheckoutRoot(), policy.worktrees.rootRelativeToCommonCheckout);
}

export function canonicalWorktreePath(issue) {
  if (!Number.isSafeInteger(issue) || issue < 1) fail("issue must be a positive integer");
  return path.join(managedRoot(), `issue-${issue}`);
}

export function isWithinManagedRoot(candidate, base) {
  if (!path.isAbsolute(candidate) || !path.isAbsolute(base)) return false;
  const resolved = path.resolve(candidate);
  const normalizedBase = path.resolve(base);
  return resolved !== normalizedBase && resolved.startsWith(`${normalizedBase}${path.sep}`);
}

function inventory() {
  const records = parseWorktrees(git(["worktree", "list", "--porcelain"]));
  const managedBase = `${managedRoot()}${path.sep}`;
  return records.map((record) => ({
    ...record,
    current: path.resolve(record.path) === realpathSync(root),
    primary: path.resolve(record.path) === commonCheckoutRoot(),
    managed: path.resolve(record.path).startsWith(managedBase),
    dirty: existsSync(record.path) ? Boolean(git(["status", "--porcelain"], { cwd: record.path })) : null,
  }));
}

export function validateManagedTarget(candidate, expectedHead, records = inventory()) {
  if (!/^[0-9a-f]{40}$/u.test(expectedHead ?? "")) fail("--expect-head must be an exact lowercase SHA");
  const resolved = path.resolve(candidate);
  if (!isWithinManagedRoot(candidate, managedRoot())) fail("target is outside the canonical managed worktree root");
  const record = records.find((entry) => path.resolve(entry.path) === resolved);
  if (!record) fail("target is not a registered Git worktree");
  if (record.current || record.primary) fail("refusing current or primary checkout");
  if (record.locked) fail("refusing a locked worktree");
  if (record.dirty !== false) fail("refusing a dirty or unreadable worktree");
  if (record.head !== expectedHead) fail(`worktree head ${record.head} does not match ${expectedHead}`);
  const info = lstatSync(record.path);
  if (!info.isDirectory() || info.isSymbolicLink()) fail("target must be a regular directory, not a symlink");
  return record;
}

function requireExecute(argv) {
  if (!argv.includes("--execute")) fail("mutation requires --execute");
}

function option(argv, name, { required = true } = {}) {
  const index = argv.indexOf(name);
  const value = index >= 0 ? argv[index + 1] : null;
  if (required && !value) fail(`${name} is required`);
  return value;
}

function audit(argv) {
  const records = inventory();
  const managed = records.filter((entry) => entry.managed);
  const report = {
    schemaVersion: 1,
    root: managedRoot(),
    limit: policy.delivery.managedWorktrees,
    active: managed.length,
    available: Math.max(0, policy.delivery.managedWorktrees - managed.length),
    worktrees: managed,
  };
  if (argv.includes("--json")) process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
  else {
    process.stdout.write(`Managed worktrees: ${report.active}/${report.limit}; available: ${report.available}\n`);
    for (const record of managed) process.stdout.write(`- ${record.path} ${record.branch ?? "detached"} ${record.head ?? "unknown"}${record.dirty ? " dirty" : ""}${record.locked ? " locked" : ""}\n`);
  }
  return report;
}

function ensure(argv) {
  const issue = Number(option(argv, "--issue"));
  const branch = option(argv, "--branch");
  const base = option(argv, "--base");
  if (base !== "origin/develop") fail("managed SDK worktrees must use --base origin/develop");
  const branchResult = validateBranchName(branch);
  if (!branchResult.ok || branchResult.issue !== issue) fail("branch grammar or issue identity is invalid");
  const target = canonicalWorktreePath(issue);
  const records = inventory();
  const existing = records.find((entry) => path.resolve(entry.path) === target);
  if (existing) {
    if (existing.branch !== branch) fail(`canonical worktree is owned by ${existing.branch ?? "detached"}`);
    process.stdout.write(`${target}\n`);
    return;
  }
  if (records.filter((entry) => entry.managed).length >= policy.delivery.managedWorktrees) fail("managed worktree capacity is exhausted");
  requireExecute(argv);
  const baseSha = git(["rev-parse", "--verify", `${base}^{commit}`]);
  const parent = path.dirname(target);
  mkdirSync(parent, { recursive: true, mode: 0o700 });
  try {
    git(["show-ref", "--verify", `refs/heads/${branch}`]);
    execFileSync("git", ["worktree", "add", target, branch], { cwd: root, stdio: "inherit" });
  } catch {
    execFileSync("git", ["worktree", "add", "-b", branch, target, baseSha], { cwd: root, stdio: "inherit" });
  }
  process.stdout.write(`${target}\n`);
}

function closeout(argv) {
  requireExecute(argv);
  const pr = Number(option(argv, "--pr"));
  const candidate = option(argv, "--path");
  const expectedHead = option(argv, "--expect-head");
  if (!Number.isSafeInteger(pr) || pr < 1) fail("--pr must be a positive integer");
  const record = validateManagedTarget(candidate, expectedHead);
  const repository = git(["remote", "get-url", "origin"]).match(/github\.com[/:]([^/]+\/[^/.]+)(?:\.git)?$/u)?.[1];
  if (repository !== policy.repository) fail("origin does not identify the authoritative repository");
  const prState = JSON.parse(execFileSync("gh", ["pr", "view", String(pr), "--repo", repository, "--json", "state,mergedAt,headRefOid,baseRefName"], { encoding: "utf8", stdio: ["ignore", "pipe", "pipe"] }));
  if (prState.state !== "MERGED" || !prState.mergedAt) fail("pull request is not merged");
  if (prState.baseRefName !== policy.integrationBranch) fail("pull request did not target develop");
  if (prState.headRefOid !== expectedHead) fail("pull request head does not match expected worktree head");
  execFileSync("git", ["worktree", "remove", record.path], { cwd: root, stdio: "inherit" });
  process.stdout.write(`Closed managed worktree ${record.path} for PR #${pr}.\n`);
}

function main() {
  const [command, ...argv] = process.argv.slice(2);
  if (command === "audit") audit(argv);
  else if (command === "ensure") ensure(argv);
  else if (command === "closeout-pr") closeout(argv);
  else fail("Usage: worktree-lifecycle.mjs <audit|ensure|closeout-pr> [options]");
}

if (path.resolve(process.argv[1] ?? "") === fileURLToPath(import.meta.url)) {
  try { main(); } catch (error) {
    process.stderr.write(`[worktree-lifecycle] ${error.message}\n`);
    process.exitCode = 1;
  }
}
