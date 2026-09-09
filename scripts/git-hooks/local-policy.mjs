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

function git(args, options = {}) {
  return execFileSync("git", args, { cwd: root, encoding: "utf8", maxBuffer: 16 * 1024 * 1024, ...options });
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
    let base = remoteSha;
    if (!base || zeroSha.test(base)) base = git(["merge-base", "HEAD", "origin/develop"]).trim();
    const outcome = validateCommitRange({ repository: root, base, head: localSha, verifyOpenPgp: true });
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
