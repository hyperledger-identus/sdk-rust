#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0

import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");
export const policy = Object.freeze(JSON.parse(readFileSync(path.join(root, ".github/contribution-policy.json"), "utf8")));
const types = new Set(policy.types);
const scopes = new Set(policy.scopes);
const pgpHeader = "gpgsig -----BEGIN PGP SIGNATURE-----";

function result(errors = [], values = {}) {
  return { ok: errors.length === 0, errors, ...values };
}

export function parseConventionalSubject(subject, body = "") {
  const errors = [];
  if (typeof subject !== "string" || subject.length === 0) return result(["subject is empty"]);
  if (subject.includes("\n") || subject.includes("\r")) errors.push("subject must be one line");
  if (subject.length > policy.commit.maxSubjectLength) errors.push(`subject exceeds ${policy.commit.maxSubjectLength} characters`);
  const match = subject.match(/^([a-z]+)\(([a-z][a-z0-9-]*)\)(!)?: (\S(?:.*\S)?)$/u);
  if (!match) return result([...errors, "subject must match <type>(<scope>)[!]: <description>"]);
  const [, type, scope, breakingMarker, description] = match;
  if (!types.has(type)) errors.push(`type '${type}' is not allowed`);
  if (!scopes.has(scope)) errors.push(`scope '${scope}' is not allowed`);
  if (description.endsWith(".")) errors.push("description must not end with a period");
  if (/^(?:fixup!|squash!|wip\b)/iu.test(description)) errors.push("temporary commit description is forbidden");
  const breaking = breakingMarker === "!";
  if (breaking && policy.commit.requireBreakingChangeFooter && !/(?:^|\n)BREAKING(?: |-)CHANGE: \S/u.test(body)) {
    errors.push("a ! marker requires a non-empty BREAKING CHANGE footer");
  }
  return result(errors, { type, scope, breaking, description });
}

export function validateBranchName(branch, { expectedType = null, actor = "" } = {}) {
  if (policy.bots[actor]?.branchExempt) return result([], { exempt: true, type: null, issue: null });
  const match = typeof branch === "string"
    ? branch.match(/^(?:codex\/)?([a-z]+)\/issue-([1-9][0-9]*)$/u)
    : null;
  if (!match) return result([`branch must match ${policy.branch.formats.join(" or ")}`]);
  const type = match[1];
  const issue = Number(match[2]);
  const errors = [];
  if (!types.has(type)) errors.push(`branch type '${type}' is not allowed`);
  if (expectedType && type !== expectedType) errors.push(`branch type '${type}' does not match subject type '${expectedType}'`);
  return result(errors, { exempt: false, type, issue });
}

export function validatePullRequest({ title, body = "", branch, actor = "" }) {
  const subject = parseConventionalSubject(title, body);
  const branchResult = validateBranchName(branch, { expectedType: subject.type, actor });
  const errors = [
    ...subject.errors.map((entry) => `title: ${entry}`),
    ...branchResult.errors.map((entry) => `branch: ${entry}`),
  ];
  if (branchResult.issue && !new RegExp(`(?:close[sd]?|fix(?:e[sd])?|resolve[sd]?)\\s+#${branchResult.issue}(?![0-9])`, "iu").test(body)) {
    errors.push(`body must close issue #${branchResult.issue}`);
  }
  return result(errors, { subject, branch: branchResult });
}

export function validateCommitEvidence({ message, authorName, authorEmail, rawCommit, verification = null, actor = "" }) {
  const normalized = String(message ?? "").replace(/\n+$/u, "");
  const [subject = "", ...bodyLines] = normalized.split("\n");
  const subjectResult = parseConventionalSubject(subject, bodyLines.join("\n"));
  const errors = [...subjectResult.errors.map((entry) => `subject: ${entry}`)];
  if (policy.commit.requireDco && !policy.bots[actor]?.dcoAuthorNames?.includes(authorName)) {
    const expected = `Signed-off-by: ${authorName} <${authorEmail}>`;
    if (!bodyLines.includes(expected)) errors.push(`missing exact DCO trailer '${expected}'`);
  }
  if (policy.commit.requireOpenPgp) {
    if (verification) {
      if (!verification.verified || verification.reason !== "valid" || !verification.signature?.startsWith("-----BEGIN PGP SIGNATURE-----")) {
        errors.push(`GitHub OpenPGP verification failed (${verification.reason ?? "missing"})`);
      }
    } else if (!String(rawCommit ?? "").includes(pgpHeader)) {
      errors.push("commit does not contain an OpenPGP signature envelope");
    }
  }
  return result(errors, { subject: subjectResult });
}

function git(repository, args, options = {}) {
  return execFileSync("git", args, {
    cwd: repository,
    encoding: "utf8",
    maxBuffer: 16 * 1024 * 1024,
    stdio: ["ignore", "pipe", "pipe"],
    ...options,
  });
}

export function validateCommitRange({ repository, base, head, verifyOpenPgp = false }) {
  const commits = git(repository, ["rev-list", "--reverse", `${base}..${head}`]).trim().split("\n").filter(Boolean);
  if (commits.length === 0) return result(["commit range is empty"], { commits: [] });
  if (commits.length > policy.commit.maximumRange) return result([`commit range exceeds ${policy.commit.maximumRange}`], { commits: [] });
  const reports = commits.map((commit) => {
    const message = git(repository, ["show", "-s", "--format=%B", commit]);
    const evidence = validateCommitEvidence({
      message,
      authorName: git(repository, ["show", "-s", "--format=%an", commit]).trim(),
      authorEmail: git(repository, ["show", "-s", "--format=%ae", commit]).trim(),
      rawCommit: git(repository, ["cat-file", "commit", commit]),
    });
    if (verifyOpenPgp) {
      try {
        git(repository, ["verify-commit", "--raw", commit], { stdio: ["ignore", "ignore", "ignore"] });
      } catch {
        evidence.errors.push("local OpenPGP cryptographic verification failed");
        evidence.ok = false;
      }
    }
    return { commit, ...evidence };
  });
  return result(reports.flatMap((entry) => entry.errors.map((error) => `${entry.commit}: ${error}`)), { commits: reports });
}

export function validateHostedCommits(records, expectedHead) {
  if (!Array.isArray(records) || records.length === 0) return result(["pull request has no commits"]);
  if (records.length > policy.commit.maximumRange) return result([`pull request exceeds ${policy.commit.maximumRange} commits`]);
  const errors = [];
  const seen = new Set();
  for (const record of records) {
    if (!/^[0-9a-f]{40}$/u.test(record?.sha ?? "") || seen.has(record.sha)) {
      errors.push("hosted commit records contain an invalid or repeated SHA");
      continue;
    }
    seen.add(record.sha);
    const evidence = validateCommitEvidence(record);
    errors.push(...evidence.errors.map((entry) => `${record.sha}: ${entry}`));
  }
  if (records.at(-1)?.sha !== expectedHead) errors.push(`last hosted commit is not exact PR head ${expectedHead}`);
  return result(errors);
}

function requireEnv(name) {
  const value = process.env[name];
  if (!value) throw new Error(`${name} is required`);
  return value;
}

function report(outcome) {
  if (outcome.ok) return;
  for (const entry of outcome.errors) process.stderr.write(`[contribution-policy] ${entry}\n`);
  process.exitCode = 1;
}

function main() {
  const command = process.argv[2];
  if (command === "pr") {
    const outcome = validatePullRequest({
      title: requireEnv("PR_TITLE"),
      body: process.env.PR_BODY ?? "",
      branch: requireEnv("PR_HEAD_REF"),
      actor: process.env.PR_ACTOR ?? "",
    });
    report(outcome);
    if (outcome.ok) process.stdout.write(`Contribution metadata valid: ${outcome.subject.type}(${outcome.subject.scope}).\n`);
    return;
  }
  if (command === "commits") {
    const outcome = validateCommitRange({
      repository: requireEnv("REPOSITORY_PATH"),
      base: requireEnv("BASE_SHA"),
      head: requireEnv("HEAD_SHA"),
      verifyOpenPgp: process.env.VERIFY_OPENPGP === "true",
    });
    report(outcome);
    if (outcome.ok) process.stdout.write(`Commit policy passed for ${outcome.commits.length} commit(s).\n`);
    return;
  }
  if (command === "hosted-commits") {
    const records = JSON.parse(readFileSync(requireEnv("COMMITS_FILE"), "utf8"));
    const outcome = validateHostedCommits(records, requireEnv("HEAD_SHA"));
    report(outcome);
    if (outcome.ok) process.stdout.write(`Hosted commit policy passed for ${records.length} commit(s).\n`);
    return;
  }
  throw new Error("Usage: contribution-policy.mjs <pr|commits|hosted-commits>");
}

if (path.resolve(process.argv[1] ?? "") === fileURLToPath(import.meta.url)) {
  try {
    main();
  } catch (error) {
    process.stderr.write(`[contribution-policy] ${error.message}\n`);
    process.exitCode = 2;
  }
}
