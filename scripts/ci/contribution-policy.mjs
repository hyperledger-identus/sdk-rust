#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0

import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import {
  parseContributionPolicy,
  validateContributionPolicyMonotonicity,
  validateContributionPolicyShape,
} from "./contribution-policy-schema.mjs";
import { parseJsonWithoutDuplicates } from "../factory-tools/strict-json.mjs";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");
export const policy = Object.freeze(parseContributionPolicy(
  readFileSync(path.join(root, ".github/contribution-policy.json"), "utf8"),
));
const policyShapeErrors = validateContributionPolicyShape(policy);
if (policyShapeErrors.length > 0) throw new Error(`base contribution policy is invalid: ${policyShapeErrors.join("; ")}`);
const types = new Set(policy.types);
const scopes = new Set(policy.scopes);
const repositorySlug = "hyperledger-identus/sdk-rust";
const deferredAcceptancePathPattern = /^openspec\/changes\/archive\/\d{4}-\d{2}-\d{2}-[a-z0-9]+(?:-[a-z0-9]+)*\/post-merge-acceptance\.json$/u;
const maximumDeferredAcceptanceBytes = 16 * 1024;

function result(errors = [], values = {}) {
  return { ok: errors.length === 0, errors, ...values };
}

export function resolveSignatureEnvelopes(document) {
  const configured = document?.commit?.signatureEnvelopes;
  if (!Array.isArray(configured) || configured.length === 0) return null;
  if (!configured.every((entry) => typeof entry === "string" && entry.trim().length > 0)) return null;
  return configured;
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

function authoritativeMarkdownLines(body) {
  const source = String(body ?? "");
  let withoutComments = "";
  let comment = false;
  for (let index = 0; index < source.length;) {
    if (!comment && source.startsWith("<!--", index)) {
      comment = true;
      withoutComments += "    ";
      index += 4;
      continue;
    }
    if (comment && source.startsWith("-->", index)) {
      comment = false;
      withoutComments += "   ";
      index += 3;
      continue;
    }
    const character = source[index];
    withoutComments += comment && character !== "\n" && character !== "\r" ? " " : character;
    index += 1;
  }

  const lines = [];
  let fence = null;
  for (const line of withoutComments.split(/\r?\n/u)) {
    const marker = line.match(/^\s*(`{3,}|~{3,})/u)?.[1] ?? null;
    if (fence === null && marker !== null) {
      fence = { character: marker[0], length: marker.length };
      continue;
    }
    if (fence !== null) {
      const closing = line.match(/^\s*(`{3,}|~{3,})\s*$/u)?.[1] ?? null;
      if (closing !== null && closing[0] === fence.character && closing.length >= fence.length) fence = null;
      continue;
    }
    lines.push(line);
  }
  return lines;
}

function captures(lines, pattern) {
  return lines.flatMap((line) => {
    const match = line.match(pattern);
    return match ? [match.slice(1)] : [];
  });
}

export function containsGitHubClosingKeyword(text, issue, repository = repositorySlug) {
  const closingKeyword = new RegExp(
    `\\b(?:close|closes|closed|fix|fixes|fixed|resolve|resolves|resolved)\\s*:?\\s+(?:([A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+))?#${issue}(?![0-9])`,
    "giu",
  );
  return [...String(text ?? "").matchAll(closingKeyword)].some((match) => (
    match[1] === undefined || match[1].toLowerCase() === repository.toLowerCase()
  ));
}

export function validateDeferredAcceptance(document, expectedIssue) {
  const errors = [];
  const expectedKeys = ["acceptance", "disposition", "issue", "rationale", "schemaVersion"];
  if (!document || typeof document !== "object" || Array.isArray(document)) {
    return result(["deferred acceptance sidecar must be an object"]);
  }
  const actualKeys = Object.keys(document).sort();
  if (actualKeys.length !== expectedKeys.length || actualKeys.some((key, index) => key !== expectedKeys[index])) {
    errors.push("deferred acceptance sidecar has an unknown or missing field");
  }
  if (document.schemaVersion !== 1) errors.push("deferred acceptance sidecar schemaVersion must be 1");
  if (!Number.isSafeInteger(document.issue) || document.issue !== expectedIssue) {
    errors.push(`deferred acceptance sidecar must bind issue #${expectedIssue}`);
  }
  if (document.disposition !== "deferred") {
    errors.push("deferred acceptance sidecar disposition must be deferred");
  }
  if (typeof document.rationale !== "string" || document.rationale.trim().length === 0 || document.rationale.length > 1024) {
    errors.push("deferred acceptance sidecar rationale must be a non-empty bounded string");
  }
  if (!Array.isArray(document.acceptance) || document.acceptance.length === 0 || document.acceptance.length > 16) {
    errors.push("deferred acceptance sidecar must contain one to sixteen acceptance IDs");
  } else {
    const seen = new Set();
    for (const entry of document.acceptance) {
      if (typeof entry !== "string" || entry.length > 128 || !/^[a-z][a-z0-9]*(?:[.-][a-z0-9]+)*$/u.test(entry)) {
        errors.push("deferred acceptance sidecar contains an invalid acceptance ID");
      } else if (seen.has(entry)) {
        errors.push("deferred acceptance sidecar contains a duplicate acceptance ID");
      }
      seen.add(entry);
    }
  }
  return result(errors);
}

export function readDeferredAcceptanceAt({ repository = root, head, sidecarPath }) {
  if (!validSha(head)) throw new Error("deferred acceptance head must be an exact lowercase SHA");
  if (typeof sidecarPath !== "string" || !deferredAcceptancePathPattern.test(sidecarPath)) {
    throw new Error("deferred acceptance sidecar path is invalid");
  }
  const commit = git(repository, ["rev-parse", "--verify", `${head}^{commit}`]).trim();
  if (commit !== head) throw new Error("deferred acceptance head is not the exact commit object");
  const entry = git(repository, ["ls-tree", head, "--", sidecarPath]).trim();
  const match = entry.match(/^100644 blob ([0-9a-f]{40})\t(.+)$/u);
  if (!match || match[2] !== sidecarPath) {
    throw new Error("deferred acceptance sidecar must be one regular non-executable Git blob");
  }
  const [, blob] = match;
  const sizeText = git(repository, ["cat-file", "-s", blob]).trim();
  if (!/^[0-9]+$/u.test(sizeText)) throw new Error("deferred acceptance sidecar size is invalid");
  const size = Number(sizeText);
  if (!Number.isSafeInteger(size) || size > maximumDeferredAcceptanceBytes) {
    throw new Error("deferred acceptance sidecar exceeds 16384 bytes");
  }
  const text = git(repository, ["cat-file", "blob", blob]);
  if (Buffer.byteLength(text, "utf8") !== size) {
    throw new Error("deferred acceptance sidecar changed while reading");
  }
  return parseJsonWithoutDuplicates(text, { maximumDepth: 8, maximumNodes: 512 });
}

export function validateIssueLinkage(body, expectedIssue, {
  loadDeferredAcceptance = null,
  additionalClosingText = "",
} = {}) {
  const lines = authoritativeMarkdownLines(body);
  const errors = [];
  const issues = captures(lines, /^\s*-\s*Issue:\s*#([1-9][0-9]*)\s*$/iu).map(Number);
  const modes = captures(lines, /^\s*-\s*Issue linkage:\s*(closes|deferred)\s*$/iu).map(([mode]) => mode.toLowerCase());
  const closing = captures(
    lines,
    /^\s*(?:close|closes|closed|fix|fixes|fixed|resolve|resolves|resolved)\s*:?\s+#([1-9][0-9]*)\.?\s*$/iu,
  ).map(Number);
  const references = captures(lines, /^\s*(?:Refs|References)\s+#([1-9][0-9]*)\.?\s*$/iu).map(Number);
  const sidecars = captures(
    lines,
    /^\s*-\s*Post-merge acceptance:\s*(openspec\/changes\/archive\/\d{4}-\d{2}-\d{2}-[a-z0-9]+(?:-[a-z0-9]+)*\/post-merge-acceptance\.json)\s*$/u,
  ).map(([sidecarPath]) => sidecarPath);

  if (issues.length !== 1 || issues[0] !== expectedIssue) {
    errors.push(`body must contain exactly one authoritative Issue field for #${expectedIssue}`);
  }
  if (modes.length !== 1) errors.push("body must contain exactly one Issue linkage field");
  const mode = modes.length === 1 ? modes[0] : null;
  if (mode === "closes") {
    if (closing.filter((issue) => issue === expectedIssue).length !== 1) {
      errors.push(`closing linkage must contain exactly one standalone closing keyword for issue #${expectedIssue}`);
    }
    if (sidecars.length > 0) errors.push("closing linkage must not declare post-merge acceptance evidence");
  } else if (mode === "deferred") {
    if (containsGitHubClosingKeyword(lines.join("\n"), expectedIssue)
        || containsGitHubClosingKeyword(additionalClosingText, expectedIssue)) {
      errors.push(`deferred linkage must not close issue #${expectedIssue}`);
    }
    if (references.filter((issue) => issue === expectedIssue).length !== 1) {
      errors.push(`deferred linkage must contain exactly one standalone reference to issue #${expectedIssue}`);
    }
    if (sidecars.length !== 1) {
      errors.push("deferred linkage must contain exactly one post-merge acceptance sidecar path");
    } else if (typeof loadDeferredAcceptance !== "function") {
      errors.push("deferred linkage requires exact-head acceptance evidence");
    } else {
      try {
        const evidence = validateDeferredAcceptance(loadDeferredAcceptance(sidecars[0]), expectedIssue);
        errors.push(...evidence.errors);
      } catch {
        errors.push("deferred linkage acceptance evidence is unavailable or invalid");
      }
    }
  }
  return result(errors, { issue: expectedIssue, mode, sidecarPath: sidecars.length === 1 ? sidecars[0] : null });
}

export function validatePullRequest(
  { title, body = "", branch, actor = "" },
  { loadDeferredAcceptance = null } = {},
) {
  const subject = parseConventionalSubject(title, body);
  const branchResult = validateBranchName(branch, { expectedType: subject.type, actor });
  const errors = [
    ...subject.errors.map((entry) => `title: ${entry}`),
    ...branchResult.errors.map((entry) => `branch: ${entry}`),
  ];
  const linkage = branchResult.issue
    ? validateIssueLinkage(body, branchResult.issue, { loadDeferredAcceptance, additionalClosingText: title })
    : result([], { issue: null, mode: null, sidecarPath: null });
  errors.push(...linkage.errors.map((entry) => `linkage: ${entry}`));
  return result(errors, { subject, branch: branchResult, linkage });
}

export function validateSignatureProvenance({ verification = null, rawCommit = "" }, document = policy) {
  const envelopes = resolveSignatureEnvelopes(document);
  if (envelopes === null) return ["contribution policy requires a signature but declares no usable signature envelope set"];
  if (!verification) {
    return envelopes.some((envelope) => String(rawCommit ?? "").includes(`gpgsig ${envelope}`))
      ? []
      : ["commit does not contain an accepted signature envelope"];
  }
  if (!verification.verified || verification.reason !== "valid") {
    return [`GitHub does not verify this commit signature (reason: ${verification.reason ?? "missing"})`];
  }
  const signature = verification.signature;
  if (typeof signature !== "string" || signature.length === 0) {
    return ["verified commit is missing its signature envelope"];
  }
  if (envelopes.some((envelope) => signature.startsWith(envelope))) return [];
  const rejected = signature.split("\n", 1)[0].slice(0, 64);
  return [`commit signature envelope is not accepted by the contribution policy: ${rejected}`];
}

export function validateCommitEvidence(
  { message, authorName, authorEmail, rawCommit, verification = null, actor = "" },
  { authoredMetadata = true } = {},
) {
  const normalized = String(message ?? "").replace(/\n+$/u, "");
  const [subject = "", ...bodyLines] = normalized.split("\n");
  const subjectResult = parseConventionalSubject(subject, bodyLines.join("\n"));
  const errors = authoredMetadata
    ? [...subjectResult.errors.map((entry) => `subject: ${entry}`)]
    : [];
  if (authoredMetadata && policy.commit.requireDco && !policy.bots[actor]?.dcoAuthorNames?.includes(authorName)) {
    const expected = `Signed-off-by: ${authorName} <${authorEmail}>`;
    if (!bodyLines.includes(expected)) errors.push(`missing exact DCO trailer '${expected}'`);
  }
  if (policy.commit.requireSignature) errors.push(...validateSignatureProvenance({ verification, rawCommit }));
  return result(errors, { subject: subjectResult });
}

function validSha(value) {
  return typeof value === "string" && /^[0-9a-f]{40}$/u.test(value);
}

function gitAncestor(repository, ancestor, descendant) {
  if (ancestor === descendant) return true;
  try {
    git(repository, ["merge-base", "--is-ancestor", ancestor, descendant], {
      stdio: ["ignore", "ignore", "ignore"],
    });
    return true;
  } catch {
    return false;
  }
}

function gitMergeTree(repository, firstParent, secondParent) {
  try {
    const [tree = ""] = git(repository, ["merge-tree", "--write-tree", firstParent, secondParent])
      .trim()
      .split("\n");
    return validSha(tree) ? tree : "";
  } catch {
    return "";
  }
}

export function isGitHubSynchronizationMerge(
  record,
  previousSha,
  {
    baseSha = "",
    baseRef = "",
    headRef = "",
    repository = root,
    isAncestor = (ancestor, descendant) => gitAncestor(repository, ancestor, descendant),
    mergeTree = (firstParent, secondParent) => gitMergeTree(repository, firstParent, secondParent),
  } = {},
) {
  const parents = record?.parentShas;
  if (!Array.isArray(parents) || parents.length !== 2 || parents[0] === parents[1]) return false;
  if (!parents.every(validSha) || !validSha(previousSha) || !validSha(baseSha)) return false;
  if (typeof baseRef !== "string" || baseRef.length === 0 || typeof headRef !== "string" || headRef.length === 0) return false;
  if (record.committerName !== "GitHub" || record.committerEmail !== "noreply@github.com") return false;
  if (record.committerActor !== "web-flow") return false;
  if (record.verification?.verified !== true || record.verification?.reason !== "valid") return false;
  const [subject = ""] = String(record.message ?? "").split("\n");
  if (subject !== `Merge branch '${baseRef}' into ${headRef}`) return false;
  if (!validSha(record.treeSha) || parents[0] !== previousSha || !isAncestor(parents[1], baseSha)) return false;
  return mergeTree(parents[0], parents[1]) === record.treeSha;
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

export function readContributionPolicyAt({ repository = root, head }) {
  if (!validSha(head)) throw new Error("proposed policy head must be an exact lowercase SHA");
  const commit = git(repository, ["rev-parse", "--verify", `${head}^{commit}`]).trim();
  if (commit !== head) throw new Error("proposed policy head is not the exact commit object");
  const object = `${head}:.github/contribution-policy.json`;
  const blob = git(repository, ["rev-parse", "--verify", object]).trim();
  if (!validSha(blob) || git(repository, ["cat-file", "-t", blob]).trim() !== "blob") {
    throw new Error("proposed contribution policy is not a Git blob");
  }
  const sizeText = git(repository, ["cat-file", "-s", blob]).trim();
  if (!/^[0-9]+$/u.test(sizeText)) throw new Error("proposed contribution policy blob size is invalid");
  const size = Number(sizeText);
  if (!Number.isSafeInteger(size) || size > 64 * 1024) {
    throw new Error("proposed contribution policy exceeds 65536 bytes");
  }
  const text = git(repository, ["cat-file", "blob", blob]);
  if (Buffer.byteLength(text, "utf8") !== size) {
    throw new Error("proposed contribution policy blob size changed while reading");
  }
  return parseContributionPolicy(text);
}

// A failed verify-commit still interleaves success-status lines with the failure cause; only the non-status lines explain the exit.
const verifyCommitStatusLine = /^(?:Good "git" signature|(?:gpg: )?Good signature from|\[GNUPG:\] (?:NEWSIG|KEY_CONSIDERED|SIG_ID|GOODSIG|VALIDSIG|PLAINTEXT)\b)/u;

export function localVerificationFailureDetail(stderr) {
  const lines = String(stderr ?? "").split("\n").map((line) => line.trim()).filter(Boolean);
  const causes = lines.filter((line) => !verifyCommitStatusLine.test(line));
  const detail = (causes.length > 0 ? causes : lines).join("; ");
  return detail.length > 256 ? `${detail.slice(0, 253)}...` : detail;
}

export function validateCommitRange({ repository, base, head, verifySignature = false }) {
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
    if (verifySignature) {
      try {
        git(repository, ["verify-commit", "--raw", commit], { stdio: ["ignore", "ignore", "pipe"] });
      } catch (error) {
        const detail = localVerificationFailureDetail(error?.stderr);
        evidence.errors.push(`local signature verification failed${detail ? `: ${detail}` : ""}`);
        evidence.ok = false;
      }
    }
    return { commit, ...evidence };
  });
  return result(reports.flatMap((entry) => entry.errors.map((error) => `${entry.commit}: ${error}`)), { commits: reports });
}

export function validateHostedCommits(records, expectedHead, context = {}) {
  if (!Array.isArray(records) || records.length === 0) return result(["pull request has no commits"]);
  if (records.length > policy.commit.maximumRange) return result([`pull request exceeds ${policy.commit.maximumRange} commits`]);
  const errors = [];
  const seen = new Set();
  for (const [index, record] of records.entries()) {
    if (!/^[0-9a-f]{40}$/u.test(record?.sha ?? "") || seen.has(record.sha)) {
      errors.push("hosted commit records contain an invalid or repeated SHA");
      continue;
    }
    seen.add(record.sha);
    const synchronizationMerge = isGitHubSynchronizationMerge(
      record,
      index > 0 ? records[index - 1]?.sha : "",
      context,
    );
    const evidence = validateCommitEvidence(record, { authoredMetadata: !synchronizationMerge });
    errors.push(...evidence.errors.map((entry) => `${record.sha}: ${entry}`));
    if (context.deferredIssue
        && containsGitHubClosingKeyword(record.message, context.deferredIssue)) {
      errors.push(`${record.sha}: commit message must not close deferred issue #${context.deferredIssue}`);
    }
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
    const head = process.env.HEAD_SHA ?? "";
    const outcome = validatePullRequest({
      title: requireEnv("PR_TITLE"),
      body: process.env.PR_BODY ?? "",
      branch: requireEnv("PR_HEAD_REF"),
      actor: process.env.PR_ACTOR ?? "",
    }, {
      loadDeferredAcceptance: (sidecarPath) => readDeferredAcceptanceAt({
        repository: root,
        head,
        sidecarPath,
      }),
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
      verifySignature: process.env.VERIFY_SIGNATURES === "true" || process.env.VERIFY_OPENPGP === "true",
    });
    report(outcome);
    if (outcome.ok) process.stdout.write(`Commit policy passed for ${outcome.commits.length} commit(s).\n`);
    return;
  }
  if (command === "hosted-commits") {
    const records = JSON.parse(readFileSync(requireEnv("COMMITS_FILE"), "utf8"));
    const head = requireEnv("HEAD_SHA");
    const branch = validateBranchName(requireEnv("PR_HEAD_REF"), {
      actor: process.env.PR_ACTOR ?? "",
    });
    let linkage;
    if (!branch.ok) {
      linkage = result(branch.errors.map((entry) => `branch: ${entry}`));
    } else if (branch.exempt) {
      linkage = result([], { mode: null });
    } else if (branch.issue) {
      linkage = validateIssueLinkage(process.env.PR_BODY ?? "", branch.issue, {
        additionalClosingText: process.env.PR_TITLE ?? "",
        loadDeferredAcceptance: (sidecarPath) => readDeferredAcceptanceAt({
          repository: root,
          head,
          sidecarPath,
        }),
      });
    } else {
      linkage = result(["hosted commit evidence requires an issue branch"]);
    }
    report(linkage);
    if (!linkage.ok) return;
    const outcome = validateHostedCommits(records, requireEnv("HEAD_SHA"), {
      baseSha: requireEnv("PR_BASE_SHA"),
      baseRef: requireEnv("PR_BASE_REF"),
      headRef: requireEnv("PR_HEAD_REF"),
      repository: root,
      deferredIssue: linkage.mode === "deferred" ? branch.issue : null,
    });
    report(outcome);
    if (outcome.ok) process.stdout.write(`Hosted commit policy passed for ${records.length} commit(s).\n`);
    return;
  }
  if (command === "policy-monotonicity") {
    const proposed = readContributionPolicyAt({ head: requireEnv("HEAD_SHA") });
    const outcome = validateContributionPolicyMonotonicity(policy, proposed);
    report(outcome);
    if (outcome.ok) process.stdout.write("Proposed contribution policy is identical or stronger than the protected base.\n");
    return;
  }
  throw new Error("Usage: contribution-policy.mjs <pr|commits|hosted-commits|policy-monotonicity>");
}

if (path.resolve(process.argv[1] ?? "") === fileURLToPath(import.meta.url)) {
  try {
    main();
  } catch (error) {
    process.stderr.write(`[contribution-policy] ${error.message}\n`);
    process.exitCode = 2;
  }
}
