#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0

import { execFileSync } from "node:child_process";
import { lstatSync } from "node:fs";
import { readFile, rename, writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { randomUUID } from "node:crypto";
import { validateBranchName } from "../ci/contribution-policy.mjs";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");
const shaPattern = /^[0-9a-f]{40}$/u;

function git(args) {
  return execFileSync("git", args, { cwd: root, encoding: "utf8", maxBuffer: 8 * 1024 * 1024, stdio: ["ignore", "pipe", "pipe"] }).trim();
}

function fail(message) {
  throw new Error(message);
}

function parseArguments(argv) {
  const [change, ...rest] = argv;
  if (!change || !/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(change)) fail("preflight requires a kebab-case change name");
  const issueIndex = rest.indexOf("--issue");
  let issue = issueIndex >= 0 ? Number(rest[issueIndex + 1]) : null;
  const validateOnly = rest.includes("--validate-receipt");
  const write = rest.includes("--write");
  if (write === validateOnly) fail("preflight requires exactly one of --write or --validate-receipt");
  if (issue !== null && (!Number.isSafeInteger(issue) || issue < 1)) fail("--issue must be a positive integer");
  if (!validateOnly && issue === null) fail("preflight receipt creation requires --issue <positive-integer>");
  const unknown = rest.filter((entry, index) => index !== issueIndex && index !== issueIndex + 1 && entry !== "--write" && entry !== "--validate-receipt");
  if (unknown.length) fail(`unknown preflight argument: ${unknown[0]}`);
  return { change, issue, write, validateOnly };
}

function receiptPath(change) {
  return path.join(root, "openspec", "changes", change, "preimplementation.json");
}

function branchAndIssue(expectedIssue) {
  const branch = git(["branch", "--show-current"]);
  const outcome = validateBranchName(branch);
  if (!outcome.ok) fail(outcome.errors.join("; "));
  if (outcome.issue !== expectedIssue) fail(`branch issue ${outcome.issue} does not match requested issue ${expectedIssue}`);
  return branch;
}

export function validatePlanningPaths(change, changed) {
  if (changed.length === 0) fail("planning contract diff is empty");
  const changePrefix = `openspec/changes/${change}/`;
  const invalid = changed.filter((candidate) => !candidate.startsWith(changePrefix) && !candidate.startsWith("docs/adr/"));
  if (invalid.length) fail(`implementation changed before preflight: ${invalid.join(", ")}`);
  if (!changed.some((candidate) => candidate.startsWith(changePrefix))) fail("planning diff does not contain the named OpenSpec change");
  return changed;
}

function validatePlanningDiff(change, baseSha, contractHeadSha) {
  const changed = git(["diff", "--name-only", `${baseSha}...${contractHeadSha}`]).split("\n").filter(Boolean);
  return validatePlanningPaths(change, changed);
}

export async function validateReceipt(change, issue) {
  const file = receiptPath(change);
  const info = lstatSync(file);
  if (!info.isFile() || info.isSymbolicLink() || info.size > 8192) fail("pre-implementation receipt must be a regular file no larger than 8 KiB");
  const receipt = JSON.parse(await readFile(file, "utf8"));
  const expectedKeys = [
    "schemaVersion", "repository", "issue", "change", "branch", "baseRef", "baseSha",
    "contractHeadSha", "createdAt", "researchReady", "constraintsReady", "strictValidation",
  ];
  if (JSON.stringify(Object.keys(receipt).sort()) !== JSON.stringify(expectedKeys.sort())) fail("pre-implementation receipt has an unknown or missing field");
  if (receipt.schemaVersion !== 1 || receipt.repository !== "hyperledger-identus/sdk-rust") fail("pre-implementation receipt identity is invalid");
  if (issue === null) issue = receipt.issue;
  if (receipt.issue !== issue || receipt.change !== change) fail("pre-implementation receipt does not match the requested issue/change");
  if (receipt.baseRef !== "origin/develop" || !shaPattern.test(receipt.baseSha) || !shaPattern.test(receipt.contractHeadSha)) fail("pre-implementation receipt Git identity is invalid");
  if (![receipt.researchReady, receipt.constraintsReady, receipt.strictValidation].every((value) => value === true)) fail("pre-implementation receipt contains an incomplete gate");
  if (!Number.isFinite(Date.parse(receipt.createdAt))) fail("pre-implementation receipt timestamp is invalid");
  const branch = branchAndIssue(issue);
  if (receipt.branch !== branch) fail(`receipt branch ${receipt.branch} does not match ${branch}`);
  try { git(["merge-base", "--is-ancestor", receipt.baseSha, receipt.contractHeadSha]); } catch { fail("receipt contract head does not descend from its base"); }
  try { git(["merge-base", "--is-ancestor", receipt.contractHeadSha, "HEAD"]); } catch { fail("current head does not descend from the preflighted contract head"); }
  validatePlanningDiff(change, receipt.baseSha, receipt.contractHeadSha);
  return receipt;
}

async function createReceipt(change, issue) {
  if (git(["status", "--porcelain"])) fail("preflight receipt creation requires a clean worktree");
  const branch = branchAndIssue(issue);
  const baseRef = "origin/develop";
  const baseSha = git(["rev-parse", baseRef]);
  const contractHeadSha = git(["rev-parse", "HEAD"]);
  try { git(["merge-base", "--is-ancestor", baseSha, contractHeadSha]); } catch { fail("contract head must descend from current origin/develop; refresh the issue branch"); }
  validatePlanningDiff(change, baseSha, contractHeadSha);
  for (const command of ["research-ready", "constraints-ready", "validate"]) {
    execFileSync(path.join(root, "scripts/factory"), [command, change], { cwd: root, stdio: "inherit" });
  }
  const receipt = {
    schemaVersion: 1,
    repository: "hyperledger-identus/sdk-rust",
    issue,
    change,
    branch,
    baseRef,
    baseSha,
    contractHeadSha,
    createdAt: new Date().toISOString(),
    researchReady: true,
    constraintsReady: true,
    strictValidation: true,
  };
  const target = receiptPath(change);
  const temporary = `${target}.${process.pid}.${randomUUID()}.tmp`;
  await writeFile(temporary, `${JSON.stringify(receipt, null, 2)}\n`, { flag: "wx", mode: 0o600 });
  await rename(temporary, target);
  return receipt;
}

async function main() {
  const options = parseArguments(process.argv.slice(2));
  let receipt;
  if (options.validateOnly) receipt = await validateReceipt(options.change, options.issue);
  else if (options.write) receipt = await createReceipt(options.change, options.issue);
  else fail("preflight requires --write or --validate-receipt");
  process.stdout.write(`factory: OpenSpec pre-implementation receipt valid for issue #${receipt.issue} at ${receipt.contractHeadSha}\n`);
}

if (path.resolve(process.argv[1] ?? "") === fileURLToPath(import.meta.url)) {
  main().catch((error) => {
    process.stderr.write(`[factory-preflight] ${error.message}\n`);
    process.exitCode = 1;
  });
}
