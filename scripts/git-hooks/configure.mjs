#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0

import { execFileSync } from "node:child_process";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");
const git = (args) => execFileSync("git", args, { cwd: root, encoding: "utf8", stdio: ["ignore", "pipe", "pipe"] }).trim();

export function auditGitConfiguration() {
  const name = git(["config", "--get", "user.name"]);
  const email = git(["config", "--get", "user.email"]);
  const signingKey = git(["config", "--get", "user.signingkey"]);
  const hooksPath = (() => { try { return git(["config", "--local", "--get", "core.hooksPath"]); } catch { return ""; } })();
  const signing = (() => { try { return git(["config", "--local", "--get", "commit.gpgsign"]); } catch { return ""; } })();
  const errors = [];
  if (!name || !email || !signingKey) errors.push("Git name, email and signing key must already be configured");
  if (hooksPath !== ".githooks") errors.push("core.hooksPath must be .githooks");
  if (signing !== "true") errors.push("commit.gpgsign must be true");
  return { ok: errors.length === 0, errors, name, email, signingKey, hooksPath, signing };
}

export function applyGitConfiguration({ execute = false } = {}) {
  if (!execute) throw new Error("Refusing to modify Git configuration without --execute");
  const name = git(["config", "--get", "user.name"]);
  const email = git(["config", "--get", "user.email"]);
  const signingKey = git(["config", "--get", "user.signingkey"]);
  if (!name || !email || !signingKey) throw new Error("Configure Git identity and signing key before repository hooks");
  git(["config", "--local", "core.hooksPath", ".githooks"]);
  git(["config", "--local", "commit.gpgsign", "true"]);
  return auditGitConfiguration();
}

function main() {
  const command = process.argv[2];
  const outcome = command === "apply"
    ? applyGitConfiguration({ execute: process.argv.includes("--execute") })
    : command === "audit" ? auditGitConfiguration() : null;
  if (!outcome) throw new Error("Usage: configure.mjs <audit|apply --execute>");
  if (!outcome.ok) {
    for (const error of outcome.errors) process.stderr.write(`[git-hooks] ${error}\n`);
    process.exitCode = 1;
  } else process.stdout.write("Repository Git hooks and signing policy are aligned.\n");
}

if (path.resolve(process.argv[1] ?? "") === fileURLToPath(import.meta.url)) {
  try { main(); } catch (error) {
    process.stderr.write(`[git-hooks] ${error.message}\n`);
    process.exitCode = 2;
  }
}
