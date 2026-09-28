#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0

import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { parseConventionalSubject, validateBranchName, validateCommitRange } from "../ci/contribution-policy.mjs";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");
const zeroSha = /^0{40}$/u;
const forbiddenPath = /(^|\/)(?:\.env(?:\..*)?|auth\.json|id_(?:rsa|ed25519)|.*\.(?:pem|key|p12))$/iu;
const forbiddenAddedContent = /(?:-----BEGIN (?:RSA |EC |OPENSSH |PGP )?PRIVATE KEY-----|github_pat_[A-Za-z0-9_]{20,}|ghp_[A-Za-z0-9]{20,}|sk-[A-Za-z0-9_-]{20,})/u;

function gitAt(repository, args, options = {}) {
  return execFileSync("git", args, {
    cwd: repository,
    encoding: "utf8",
    maxBuffer: 16 * 1024 * 1024,
    stdio: ["ignore", "pipe", "pipe"],
    ...options,
  });
}

function git(args, options = {}) {
  return gitAt(root, args, options);
}

function exactCommit(repository, revision) {
  try {
    const commit = gitAt(repository, ["rev-parse", "--verify", `${revision}^{commit}`]).trim();
    return /^[0-9a-f]{40}$/u.test(commit) ? commit : null;
  } catch {
    return null;
  }
}

function isAncestor(repository, ancestor, descendant) {
  try {
    gitAt(repository, ["merge-base", "--is-ancestor", ancestor, descendant], { stdio: "ignore" });
    return true;
  } catch {
    return false;
  }
}

function hasCommonAncestor(repository, left, right) {
  try {
    return /^[0-9a-f]{40}$/u.test(gitAt(repository, ["merge-base", left, right]).trim());
  } catch {
    return false;
  }
}

export function resolvePrePushBase({ repository = root, localSha, remoteSha }) {
  const errors = [];
  if (!/^[0-9a-f]{40}$/u.test(localSha ?? "")) errors.push("local push head must be an exact lowercase SHA");
  const localHead = errors.length === 0 ? exactCommit(repository, localSha) : null;
  if (errors.length === 0 && localHead !== localSha) errors.push("local push head is missing or is not the exact commit object");

  const protectedBase = exactCommit(repository, "refs/remotes/origin/develop");
  if (protectedBase === null) errors.push("current origin/develop is missing or invalid");
  if (errors.length > 0) return { ok: false, errors, base: null, mode: null };
  if (!isAncestor(repository, protectedBase, localHead)) {
    return {
      ok: false,
      errors: ["outgoing head does not descend from current origin/develop"],
      base: null,
      mode: null,
    };
  }

  if (zeroSha.test(remoteSha ?? "")) return { ok: true, errors: [], base: protectedBase, mode: "first-push" };
  if (!/^[0-9a-f]{40}$/u.test(remoteSha ?? "")) {
    return { ok: false, errors: ["remote branch head must be an exact lowercase SHA"], base: null, mode: null };
  }
  const remoteHead = exactCommit(repository, remoteSha);
  if (remoteHead !== remoteSha) {
    return { ok: false, errors: ["remote branch head is missing or is not the exact commit object"], base: null, mode: null };
  }

  if (isAncestor(repository, remoteHead, localHead) && isAncestor(repository, protectedBase, remoteHead)) {
    return { ok: true, errors: [], base: remoteHead, mode: "fast-forward" };
  }
  if (!hasCommonAncestor(repository, remoteHead, protectedBase)) {
    return {
      ok: false,
      errors: ["remote branch history is unrelated to current origin/develop"],
      base: null,
      mode: null,
    };
  }
  return { ok: true, errors: [], base: protectedBase, mode: "protected-base" };
}

export function inspectStagedSecrets() {
  const errors = [];
  const paths = git(["diff", "--cached", "--name-only", "--diff-filter=ACMR"]).trim().split("\n").filter(Boolean);
  for (const candidate of paths) if (forbiddenPath.test(candidate)) errors.push(`forbidden staged secret path: ${candidate}`);
  const patch = git(["diff", "--cached", "--no-ext-diff", "--unified=0", "--", ...paths]);
  for (const line of patch.split("\n")) {
    if (line.startsWith("+") && !line.startsWith("+++") && forbiddenAddedContent.test(line)) {
      errors.push("staged additions contain a private-key or credential-shaped value");
      break;
    }
  }
  return { ok: errors.length === 0, errors };
}

function fail(errors) {
  for (const error of errors) process.stderr.write(`[local-policy] ${error}\n`);
  process.exitCode = 1;
}

function validateMessage(file) {
  const message = readFileSync(file, "utf8").replace(/\n+$/u, "");
  const [subject = "", ...body] = message.split("\n");
  const outcome = parseConventionalSubject(subject, body.join("\n"));
  if (!outcome.ok) fail(outcome.errors);
}

function prePush() {
  const branch = git(["branch", "--show-current"]).trim();
  const branchResult = validateBranchName(branch);
  if (!branchResult.ok) return fail(branchResult.errors);
  const lines = readFileSync(0, "utf8").trim().split("\n").filter(Boolean);
  const errors = [];
  for (const line of lines) {
    const [localRef, localSha, , remoteSha] = line.trim().split(/\s+/u);
    if (!localRef?.startsWith("refs/heads/") || zeroSha.test(localSha ?? "")) continue;
    const selection = resolvePrePushBase({ repository: root, localSha, remoteSha });
    if (!selection.ok) {
      errors.push(...selection.errors);
      continue;
    }
    const outcome = validateCommitRange({ repository: root, base: selection.base, head: localSha, verifySignature: true });
    errors.push(...outcome.errors);
  }
  if (errors.length) return fail(errors);
  execFileSync(path.join(root, "scripts/factory"), ["check"], { cwd: root, stdio: "inherit" });
}

function main() {
  const command = process.argv[2];
  if (command === "commit-msg") validateMessage(process.argv[3]);
  else if (command === "pre-commit") {
    const outcome = inspectStagedSecrets();
    if (!outcome.ok) fail(outcome.errors);
  } else if (command === "pre-push") prePush();
  else throw new Error("Usage: local-policy.mjs <commit-msg FILE|pre-commit|pre-push>");
}

if (path.resolve(process.argv[1] ?? "") === fileURLToPath(import.meta.url)) {
  try { main(); } catch (error) {
    process.stderr.write(`[local-policy] ${error.message}\n`);
    process.exitCode = 2;
  }
}
