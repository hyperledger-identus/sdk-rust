#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0

import { execFileSync } from "node:child_process";
import { mkdirSync, readFileSync, renameSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { randomUUID } from "node:crypto";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");
const policy = JSON.parse(readFileSync(path.join(root, ".factory-policy.json"), "utf8"));
const allowedKeys = new Set([
  "schemaVersion", "repository", "issue", "pullRequest", "headSha", "profile",
  "startedAt", "completedAt", "durationSeconds", "retryCount", "toolCalls",
  "inputTokens", "outputTokens", "coveragePercent", "diskBytes", "outcome",
]);
const nullableCounters = ["durationSeconds", "retryCount", "toolCalls", "inputTokens", "outputTokens", "coveragePercent", "diskBytes"];
const forbiddenPattern = /(?:prompt|message|secret|tokenValue|credential|privateKey|mnemonic|seed|authorization)/iu;

function fail(message) { throw new Error(message); }
function git(args) {
  return execFileSync("git", args, { cwd: root, encoding: "utf8", stdio: ["ignore", "pipe", "pipe"] }).trim();
}
function exactSha(value) { return /^[0-9a-f]{40}$/u.test(value ?? ""); }
function positiveInteger(value) { return Number.isSafeInteger(value) && value > 0; }
function nullableNonnegative(value) { return value === null || (Number.isFinite(value) && value >= 0); }
function isoDate(value) { return typeof value === "string" && Number.isFinite(Date.parse(value)); }

export function validateMetric(record, { requireCurrentHead = false } = {}) {
  const errors = [];
  if (!record || typeof record !== "object" || Array.isArray(record)) return { ok: false, errors: ["record must be an object"] };
  for (const key of Object.keys(record)) {
    if (!allowedKeys.has(key)) errors.push(`unknown field: ${key}`);
    if (forbiddenPattern.test(key)) errors.push(`forbidden field: ${key}`);
  }
  for (const key of allowedKeys) if (!Object.hasOwn(record, key)) errors.push(`missing field: ${key}`);
  if (record.schemaVersion !== 1 || record.repository !== policy.repository) errors.push("schema or repository identity is invalid");
  if (!positiveInteger(record.issue)) errors.push("issue must be a positive integer");
  if (record.pullRequest !== null && !positiveInteger(record.pullRequest)) errors.push("pullRequest must be null or a positive integer");
  if (!exactSha(record.headSha)) errors.push("headSha must be an exact lowercase SHA");
  if (!["prototype", "production-ready", "integration"].includes(record.profile)) errors.push("profile is invalid");
  if (!isoDate(record.startedAt) || !isoDate(record.completedAt)) errors.push("timestamps must be ISO date-times");
  if (isoDate(record.startedAt) && isoDate(record.completedAt) && Date.parse(record.completedAt) < Date.parse(record.startedAt)) errors.push("completedAt precedes startedAt");
  for (const key of nullableCounters) if (!nullableNonnegative(record[key])) errors.push(`${key} must be null or non-negative`);
  if (record.coveragePercent !== null && record.coveragePercent > 100) errors.push("coveragePercent exceeds 100");
  if (!["merged", "closed", "blocked", "in-progress"].includes(record.outcome)) errors.push("outcome is invalid");
  if (requireCurrentHead && exactSha(record.headSha) && record.headSha !== git(["rev-parse", "HEAD"])) errors.push("headSha is not the current exact HEAD");
  return { ok: errors.length === 0, errors };
}

export function renderMetric(record) {
  const result = validateMetric(record);
  if (!result.ok) fail(result.errors.join("; "));
  const value = (candidate, suffix = "") => candidate === null ? "unknown" : `${candidate}${suffix}`;
  const summary = [
    "### Factory metrics",
    "",
    `- Exact head: \`${record.headSha}\``,
    `- Profile/outcome: \`${record.profile}\` / \`${record.outcome}\``,
    `- Duration/retries/tool calls: ${value(record.durationSeconds, "s")} / ${value(record.retryCount)} / ${value(record.toolCalls)}`,
    `- Tokens (input/output): ${value(record.inputTokens)} / ${value(record.outputTokens)}`,
    `- Coverage/disk: ${value(record.coveragePercent, "%")} / ${value(record.diskBytes, " bytes")}`,
  ].join("\n");
  const payload = Buffer.from(JSON.stringify(record), "utf8").toString("base64url");
  const rendered = `${summary}\n\n<!-- ${policy.metrics.publicMarker}:${payload} -->\n`;
  if (Buffer.byteLength(rendered) > policy.metrics.maximumPublicBytes) fail("public metrics payload exceeds configured byte bound");
  return rendered;
}

export function privateStoreRoot() {
  const common = path.resolve(root, git(["rev-parse", "--git-common-dir"]));
  return path.join(common, policy.metrics.privateStore);
}

function readRecord(file) {
  const content = readFileSync(path.resolve(file), "utf8");
  if (Buffer.byteLength(content) > 32768) fail("metric record exceeds 32 KiB");
  if (forbiddenPattern.test(content)) fail("metric record contains a forbidden privacy term");
  return JSON.parse(content);
}

function writePrivate(record) {
  const result = validateMetric(record, { requireCurrentHead: true });
  if (!result.ok) fail(result.errors.join("; "));
  const directory = path.join(privateStoreRoot(), `issue-${record.issue}`);
  mkdirSync(directory, { recursive: true, mode: 0o700 });
  const target = path.join(directory, `${record.headSha}.json`);
  const temporary = `${target}.${process.pid}.${randomUUID()}.tmp`;
  writeFileSync(temporary, `${JSON.stringify(record, null, 2)}\n`, { flag: "wx", mode: 0o600 });
  renameSync(temporary, target);
  process.stdout.write(`${target}\n`);
}

function publish(record, issue, execute) {
  if (!execute) fail("publication requires --execute");
  if (record.issue !== issue) fail("metric issue does not match publication target");
  const body = renderMetric(record);
  const login = execFileSync("gh", ["api", "user", "--jq", ".login"], { encoding: "utf8" }).trim();
  const comments = JSON.parse(execFileSync("gh", ["api", `repos/${policy.repository}/issues/${issue}/comments`, "--paginate"], { encoding: "utf8", maxBuffer: 16 * 1024 * 1024 }));
  const marker = `<!-- ${policy.metrics.publicMarker}:`;
  const owned = comments.filter((comment) => comment?.user?.login === login && String(comment.body).includes(marker));
  if (owned.length > 1) fail("multiple owned metrics comments exist; refusing ambiguous update");
  if (owned.length === 1) {
    if (!positiveInteger(owned[0].id)) fail("owned metrics comment has an invalid identity");
    execFileSync("gh", ["api", "--method", "PATCH", `repos/${policy.repository}/issues/comments/${owned[0].id}`, "-f", `body=${body}`], { stdio: "inherit" });
  }
  else execFileSync("gh", ["issue", "comment", String(issue), "--repo", policy.repository, "--body", body], { stdio: "inherit" });
}

function argument(argv, name, required = true) {
  const index = argv.indexOf(name);
  const value = index >= 0 ? argv[index + 1] : null;
  if (required && !value) fail(`${name} is required`);
  return value;
}

function template(issue) {
  const now = new Date().toISOString();
  return {
    schemaVersion: 1, repository: policy.repository, issue, pullRequest: null,
    headSha: git(["rev-parse", "HEAD"]), profile: policy.delivery.defaultProfile,
    startedAt: now, completedAt: now, durationSeconds: null, retryCount: null,
    toolCalls: null, inputTokens: null, outputTokens: null, coveragePercent: null,
    diskBytes: null, outcome: "in-progress",
  };
}

function main() {
  const [command, ...argv] = process.argv.slice(2);
  if (command === "template") {
    const issue = Number(argument(argv, "--issue"));
    if (!positiveInteger(issue)) fail("--issue must be a positive integer");
    process.stdout.write(`${JSON.stringify(template(issue), null, 2)}\n`);
  } else if (["validate", "write", "render", "publish"].includes(command)) {
    const record = readRecord(argument(argv, "--file"));
    if (command === "validate") {
      const result = validateMetric(record, { requireCurrentHead: argv.includes("--current-head") });
      if (!result.ok) fail(result.errors.join("; "));
      process.stdout.write("factory: metric record is valid\n");
    } else if (command === "write") writePrivate(record);
    else if (command === "render") process.stdout.write(renderMetric(record));
    else publish(record, Number(argument(argv, "--issue")), argv.includes("--execute"));
  } else fail("Usage: metrics.mjs <template|validate|write|render|publish> [options]");
}

if (path.resolve(process.argv[1] ?? "") === fileURLToPath(import.meta.url)) {
  try { main(); } catch (error) { process.stderr.write(`[factory-metrics] ${error.message}\n`); process.exitCode = 1; }
}
