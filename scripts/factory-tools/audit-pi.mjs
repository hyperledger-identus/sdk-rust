#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0

import { execFileSync } from "node:child_process";
import { readFileSync, realpathSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { checkUserPolicy } from "./pi-policy.mjs";
import { inspectPiPackageCache, readPackageRuntime } from "./pi-package-cache.mjs";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");

function readJson(relative) { return JSON.parse(readFileSync(path.join(root, relative), "utf8")); }
function command(program, args) { return execFileSync(program, args, { cwd: root, encoding: "utf8", stdio: ["ignore", "pipe", "pipe"] }).trim(); }
function check(condition, message, findings) { if (!condition) findings.push(message); }

export async function auditPi({ configOnly = false, enforceConfig = false } = {}) {
  const factory = readJson(".factory-policy.json");
  const settings = readJson(".pi/settings.json");
  const profiles = readJson(".pi/delivery-profiles.json");
  const subagents = readJson(".pi/subagent-policy.json");
  const devloops = readFileSync(path.join(root, ".devloops"), "utf8");
  const findings = [];
  check(factory.runtime.configurationIsGuidance === true, "vault configuration must remain guidance", findings);
  check(factory.cache?.piPackages?.schemaVersion === 1, "Pi package cache schema must remain explicit", findings);
  check(factory.cache?.piPackages?.rootKind === "repository-sibling", "Pi package cache must remain outside worktrees", findings);
  check(factory.cache?.piPackages?.projectLink === ".pi/npm", "Pi project package link is inconsistent", findings);
  check(factory.cache?.piPackages?.manifest === ".pi/package-runtime/package.json", "Pi package manifest path is inconsistent", findings);
  check(factory.cache?.piPackages?.lock === ".pi/package-runtime/package-lock.json", "Pi package lock path is inconsistent", findings);
  check(factory.cache?.piPackages?.lifecycleScripts === "disabled", "Pi package lifecycle scripts must remain disabled", findings);
  check(factory.cache?.piPackages?.automaticPruning === false, "Pi package cache pruning must remain explicit", findings);
  check(factory.integrationBranch === "develop" && factory.reservedBranches.includes("main"), "branch authority is inconsistent", findings);
  check(!Object.hasOwn(settings, "provider") && !Object.hasOwn(settings, "model"), "tracked Pi settings must not choose a personal provider/model", findings);
  check(Array.isArray(settings.packages) && settings.packages.length > 0 && settings.packages.every((entry) => /^npm:[a-z0-9-]+@[0-9]+\.[0-9]+\.[0-9]+$/u.test(entry)), "Pi packages must be exact npm versions", findings);
  let packageRuntime = null;
  try {
    packageRuntime = readPackageRuntime(root, settings.packages);
  } catch (error) {
    check(false, error.message, findings);
  }
  check(profiles.defaultProfile === factory.delivery.defaultProfile, "delivery profile defaults disagree", findings);
  check(profiles.profiles?.[profiles.defaultProfile]?.qualityBudget?.targetPercent === factory.delivery.qualityTargetPercent, "quality target disagrees", findings);
  check(profiles.profiles?.[profiles.defaultProfile]?.qualityBudget?.mandatoryInvariantsPercent === factory.delivery.mandatoryInvariantPercent, "mandatory invariant target disagrees", findings);
  check(subagents.globalConcurrencyLimit <= factory.delivery.managedWorktrees, "subagent concurrency exceeds managed worktree capacity", findings);
  check(subagents.maxSubagentSpawnsPerSession <= 1 && subagents.maxSubagentDepth <= 2, "subagent bounds exceed bootstrap policy", findings);
  check(/preApproval:[\s\S]*?requireCi:\s*true/u.test(devloops), "pre-approval review must require hosted CI", findings);
  check(/humanMergeOnly:\s*false/u.test(devloops), "dev-loop merge authority disagrees with ADRs 0003/0004", findings);

  let userPolicy = null;
  if (enforceConfig) {
    userPolicy = await checkUserPolicy();
    check(userPolicy.ok, `user Pi policy is not aligned: ${userPolicy.mismatches.map((item) => item.field).join(", ")}`, findings);
  }

  let runtime = null;
  if (!configOnly) {
    const nodeVersion = command("node", ["--version"]);
    const piPath = command("which", ["pi"]);
    const piVersionOutput = command("pi", ["--version"]);
    const npmVersion = command("npm", ["--version"]);
    runtime = { nodeVersion, npmVersion, piPath: realpathSync(piPath), piVersionOutput };
    check(Number(nodeVersion.match(/^v([0-9]+)/u)?.[1]) === factory.runtime.nodeMajor, `Node ${factory.runtime.nodeMajor} is required`, findings);
    check(runtime.piPath.startsWith("/nix/store/"), "Pi must resolve from the pinned Nix store", findings);
    check(piVersionOutput.includes(factory.runtime.piVersion), `Pi ${factory.runtime.piVersion} is required`, findings);
    check(process.env.RUSTC_WRAPPER === "sccache", "RUSTC_WRAPPER must be sccache inside the devshell", findings);
    let hooksPath = "";
    let signing = "";
    try { hooksPath = command("git", ["config", "--local", "--get", "core.hooksPath"]); } catch { /* reported below */ }
    try { signing = command("git", ["config", "--local", "--get", "commit.gpgsign"]); } catch { /* reported below */ }
    check(hooksPath === ".githooks", "repository Git hooks are not configured; run ./bootstrap.sh --configure-git", findings);
    check(signing === "true", "repository commit signing is not enabled", findings);
    const commonRoot = path.dirname(path.resolve(root, command("git", ["rev-parse", "--git-common-dir"])));
    const managedRoot = path.join(commonRoot, factory.worktrees.rootRelativeToCommonCheckout);
    const worktreeText = command("git", ["worktree", "list", "--porcelain"]);
    const activeManaged = worktreeText.split("\n").filter((line) => line.startsWith(`worktree ${managedRoot}${path.sep}`)).length;
    check(activeManaged <= factory.delivery.managedWorktrees, "managed worktree capacity is exceeded", findings);
    runtime.hooksPath = hooksPath;
    runtime.managedWorktrees = activeManaged;
    try {
      const cache = inspectPiPackageCache({
        repositoryRoot: root,
        inputs: {
          piVersion: piVersionOutput,
          nodeVersion,
          npmVersion,
          packageLockSha256: packageRuntime?.packageLockSha256 ?? "invalid",
          packages: settings.packages,
        },
      });
      runtime.piPackageCache = { state: cache.state, path: cache.layout.cachePath };
    } catch (error) {
      check(false, `Pi package cache is unsafe: ${error.message}`, findings);
    }
  }

  return { ok: findings.length === 0, findings, runtime, userPolicy };
}

async function main() {
  const argv = process.argv.slice(2);
  const result = await auditPi({ configOnly: argv.includes("--config-only"), enforceConfig: argv.includes("--enforce-config") });
  if (argv.includes("--json")) process.stdout.write(`${JSON.stringify(result, null, 2)}\n`);
  else if (result.ok) process.stdout.write("factory: Pi runtime and policy audit passed\n");
  else for (const finding of result.findings) process.stderr.write(`[pi-audit] ${finding}\n`);
  process.exitCode = result.ok ? 0 : 1;
}

if (path.resolve(process.argv[1] ?? "") === fileURLToPath(import.meta.url)) {
  main().catch((error) => { process.stderr.write(`[pi-audit] ${error.message}\n`); process.exitCode = 1; });
}
