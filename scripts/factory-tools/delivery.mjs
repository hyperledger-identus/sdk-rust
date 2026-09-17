#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0

import { spawnSync } from "node:child_process";
import {
  closeSync,
  constants,
  fstatSync,
  lstatSync,
  openSync,
  readSync,
} from "node:fs";
import path from "node:path";
import { TextDecoder } from "node:util";
import { fileURLToPath } from "node:url";
import { validatePullRequest } from "../ci/contribution-policy.mjs";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");
const pullRequestPolicy = path.join(root, "scripts", "check-pr-policy.sh");
export const maximumPullRequestBodyBytes = 64 * 1024;

function fail(message) {
  throw new Error(message);
}

function requireText(value, label) {
  if (typeof value !== "string" || value.length === 0 || value.includes("\0")) {
    fail(`${label} must be an explicit non-empty string`);
  }
  if (value.length > 4096) fail(`${label} exceeds its input bound`);
  return value;
}

export function readPullRequestBody(file) {
  requireText(file, "--body-file");
  const resolved = path.resolve(file);
  let before;
  try {
    before = lstatSync(resolved);
  } catch {
    fail("pull request body file is unavailable");
  }
  if (!before.isFile() || before.isSymbolicLink()) {
    fail("pull request body must be a regular non-symlink file");
  }
  if (before.size > maximumPullRequestBodyBytes) {
    fail("pull request body file exceeds its 64 KiB byte bound");
  }

  let descriptor;
  try {
    const flags = constants.O_RDONLY | (constants.O_NOFOLLOW ?? 0) | (constants.O_NONBLOCK ?? 0);
    descriptor = openSync(resolved, flags);
  } catch {
    fail("pull request body file could not be opened safely");
  }

  let bytes;
  try {
    const opened = fstatSync(descriptor);
    if (!opened.isFile() || opened.dev !== before.dev || opened.ino !== before.ino) {
      fail("pull request body file changed during validation");
    }
    if (opened.size > maximumPullRequestBodyBytes) {
      fail("pull request body file exceeds its 64 KiB byte bound");
    }
    const bounded = Buffer.allocUnsafe(maximumPullRequestBodyBytes + 1);
    let length = 0;
    while (length < bounded.length) {
      const count = readSync(descriptor, bounded, length, bounded.length - length, null);
      if (count === 0) break;
      length += count;
    }
    if (length > maximumPullRequestBodyBytes) {
      fail("pull request body file exceeds its 64 KiB byte bound");
    }
    bytes = bounded.subarray(0, length);
  } finally {
    closeSync(descriptor);
  }

  let body;
  try {
    body = new TextDecoder("utf-8", { fatal: true }).decode(bytes);
  } catch {
    fail("pull request body file must contain valid UTF-8");
  }
  if (body.includes("\0")) fail("pull request body file contains an invalid null byte");
  return body;
}

export function runPullRequestPolicy(environment) {
  const childEnvironment = { ...process.env, ...environment };
  delete childEnvironment.GITHUB_OUTPUT;
  const outcome = spawnSync(pullRequestPolicy, [], {
    cwd: root,
    encoding: "utf8",
    env: childEnvironment,
    maxBuffer: 1024 * 1024,
    stdio: ["ignore", "pipe", "pipe"],
  });
  if (outcome.error) fail("pull-request policy checker could not be executed");
  return outcome;
}

class PullRequestPolicyError extends Error {
  constructor(diagnostics) {
    super("pull request metadata policy failed");
    this.name = "PullRequestPolicyError";
    this.diagnostics = diagnostics;
  }
}

function policyDiagnostics(stderr) {
  const diagnostics = String(stderr ?? "").split(/\r?\n/u).filter(Boolean);
  return diagnostics.length > 0
    ? diagnostics
    : ["pull-request-policy: checker failed without diagnostics"];
}

export function preflightPullRequest({ title, bodyFile, headRef, baseRef, draft }, {
  runPolicy = runPullRequestPolicy,
} = {}) {
  requireText(title, "--title");
  requireText(headRef, "--head-ref");
  requireText(baseRef, "--base-ref");
  if (typeof draft !== "boolean") fail("--draft must be explicitly true or false");
  const body = readPullRequestBody(bodyFile);
  const environment = Object.freeze({
    PR_BASE_REF: baseRef,
    PR_DRAFT: String(draft),
    PR_BODY: body,
  });
  const shellOutcome = runPolicy(environment);
  if (!shellOutcome || shellOutcome.status !== 0) {
    throw new PullRequestPolicyError(policyDiagnostics(shellOutcome?.stderr));
  }

  const contribution = validatePullRequest({ title, body, branch: headRef });
  if (!contribution.ok) {
    throw new PullRequestPolicyError(
      contribution.errors.map((entry) => `[contribution-policy] ${entry}`),
    );
  }
  return {
    ok: true,
    issue: contribution.branch.issue,
    type: contribution.subject.type,
    scope: contribution.subject.scope,
  };
}

export function parseDeliveryArguments(argv) {
  const [command, ...rest] = argv;
  if (command !== "pr-preflight") fail("delivery requires the pr-preflight command");
  const names = new Set(["--title", "--body-file", "--head-ref", "--base-ref", "--draft"]);
  const values = new Map();
  for (let index = 0; index < rest.length; index += 2) {
    const name = rest[index];
    const value = rest[index + 1];
    if (!names.has(name)) fail(`unknown delivery argument: ${name ?? "<missing>"}`);
    if (values.has(name)) fail(`duplicate delivery argument: ${name}`);
    if (value === undefined || value.startsWith("--")) fail(`${name} requires an explicit value`);
    values.set(name, value);
  }
  for (const name of names) if (!values.has(name)) fail(`pr-preflight requires ${name}`);
  const draftValue = values.get("--draft");
  if (draftValue !== "true" && draftValue !== "false") {
    fail("--draft must be explicitly true or false");
  }
  return {
    title: values.get("--title"),
    bodyFile: values.get("--body-file"),
    headRef: values.get("--head-ref"),
    baseRef: values.get("--base-ref"),
    draft: draftValue === "true",
  };
}

function usage() {
  return [
    "Usage: scripts/factory delivery pr-preflight [options]",
    "",
    "Options:",
    "  --title <title>       Explicit pull-request title",
    "  --body-file <path>    Bounded regular non-symlink pull-request body file",
    "  --head-ref <ref>      Explicit pull-request head branch",
    "  --base-ref <ref>      Explicit pull-request base branch",
    "  --draft <true|false>  Explicit pull-request draft state",
  ].join("\n");
}

function main() {
  if (["help", "--help", "-h"].includes(process.argv[2])) {
    process.stdout.write(`${usage()}\n`);
    return;
  }
  const options = parseDeliveryArguments(process.argv.slice(2));
  const outcome = preflightPullRequest(options);
  process.stdout.write(`factory: pull request metadata preflight passed for issue #${outcome.issue}\n`);
}

if (path.resolve(process.argv[1] ?? "") === fileURLToPath(import.meta.url)) {
  try {
    main();
  } catch (error) {
    if (Array.isArray(error?.diagnostics)) {
      for (const diagnostic of error.diagnostics) process.stderr.write(`${diagnostic}\n`);
    } else {
      process.stderr.write(`[factory-delivery] ${error.message}\n`);
    }
    process.exitCode = 1;
  }
}
