#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0

import { execFileSync } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");
const repository = "hyperledger-identus/sdk-rust";
const shaPattern = /^[0-9a-f]{40}$/u;

export const bindingTargets = Object.freeze([
  "package-types",
  "browser",
  "nodejs",
  "react-web",
  "react-native",
]);

const bindingPackage = /^(?:packages|bindings)\/(?:identus-sdk|typescript-sdk|sdk-typescript)(?:\/|$)/u;
const knownNeutral = /^(?:\.github\/|bindings\/|crates\/|docs\/|examples\/|fuzz\/|nix\/|openspec\/|packages\/|scripts\/|site\/|tests\/|tools\/|Cargo\.(?:toml|lock)|flake\.(?:nix|lock)|rust-toolchain(?:\.toml)?|README(?:\.md)?|LICENSE(?:\.md)?|CONTRIBUTING\.md|GOVERNANCE\.md)/u;

function git(args) {
  return execFileSync("git", args, {
    cwd: root,
    encoding: "utf8",
    maxBuffer: 16 * 1024 * 1024,
    stdio: ["ignore", "pipe", "pipe"],
  });
}

function cargoMetadata() {
  return JSON.parse(execFileSync("cargo", ["metadata", "--locked", "--no-deps", "--format-version", "1"], {
    cwd: root,
    encoding: "utf8",
    maxBuffer: 32 * 1024 * 1024,
    stdio: ["ignore", "pipe", "pipe"],
  }));
}

function resolveCommit(revision, label) {
  let sha;
  try {
    sha = git(["rev-parse", "--verify", `${revision}^{commit}`]).trim();
  } catch {
    throw new Error(`${label} is not an available commit: ${revision}`);
  }
  if (!shaPattern.test(sha)) throw new Error(`${label} did not resolve to an exact SHA`);
  return sha;
}

function normalizeRelative(candidate) {
  if (typeof candidate !== "string" || candidate.length === 0 || candidate.length > 4096 || candidate.includes("\0")) return null;
  const normalized = candidate.replaceAll("\\", "/");
  if (normalized.startsWith("/") || normalized.split("/").includes("..") || normalized !== path.posix.normalize(normalized)) return null;
  return normalized;
}

export function deriveCargoDependencyPaths(metadata, targetPackage = "identus-wasm-did", repositoryRoot = root) {
  if (!metadata || !Array.isArray(metadata.packages)) throw new Error("Cargo metadata packages are missing");
  const packages = new Map(metadata.packages.map((entry) => [entry.name, entry]));
  if (!packages.has(targetPackage)) throw new Error(`Cargo metadata is missing ${targetPackage}`);
  const visited = new Set();
  const queue = [targetPackage];
  while (queue.length > 0) {
    const name = queue.shift();
    if (visited.has(name)) continue;
    visited.add(name);
    const entry = packages.get(name);
    for (const dependency of entry?.dependencies ?? []) {
      if (packages.has(dependency.name) && !visited.has(dependency.name)) queue.push(dependency.name);
    }
  }
  return [...visited].map((name) => {
    const manifest = packages.get(name).manifest_path;
    const relative = path.relative(repositoryRoot, path.dirname(manifest)).replaceAll(path.sep, "/");
    if (!relative || relative.startsWith("../")) throw new Error(`Cargo package ${name} is outside the repository`);
    return relative;
  }).sort();
}

function addSignal(signals, targets, signal, selected = bindingTargets) {
  signals.add(signal);
  for (const target of selected) targets.get(target).add(signal);
}

export function buildBindingImpactPlan({ baseSha, headSha, paths, dependencyPaths = ["crates/did", "crates/wasm-did"] }) {
  if (!shaPattern.test(baseSha) || !shaPattern.test(headSha)) throw new Error("binding plan requires exact base and head SHAs");
  if (!Array.isArray(paths) || paths.length > 2000) throw new Error("binding plan paths are invalid or exceed 2,000 entries");
  if (!Array.isArray(dependencyPaths) || dependencyPaths.some((entry) => normalizeRelative(entry) === null)) {
    throw new Error("binding plan dependency paths are invalid");
  }
  const targetSignals = new Map(bindingTargets.map((target) => [target, new Set()]));
  const signals = new Set();
  const normalizedPaths = [];
  let unknownImpact = paths.length === 0;

  for (const candidate of paths) {
    const relative = normalizeRelative(candidate);
    if (relative === null) {
      unknownImpact = true;
      continue;
    }
    normalizedPaths.push(relative);
    const inCargoClosure = dependencyPaths.some((prefix) => relative === prefix || relative.startsWith(`${prefix}/`));
    if (inCargoClosure) addSignal(signals, targetSignals, "cargo-dependency-closure");
    if (/^crates\/wasm-did\/api\//u.test(relative)) addSignal(signals, targetSignals, "binding-api-schema");
    if (/^(?:Cargo\.(?:toml|lock)|rust-toolchain(?:\.toml)?|flake\.(?:nix|lock))$/u.test(relative)) addSignal(signals, targetSignals, "cargo-lock-or-toolchain");
    if (/^(?:package-lock\.json|npm-shrinkwrap\.json|pnpm-lock\.yaml|yarn\.lock)$/u.test(relative)
        || (bindingPackage.test(relative) && /\/(?:package-lock\.json|npm-shrinkwrap\.json|pnpm-lock\.yaml|yarn\.lock)$/u.test(relative))) {
      addSignal(signals, targetSignals, "npm-lock");
    }
    if (bindingPackage.test(relative) && /(?:^|\/)package\.json$/u.test(relative)) addSignal(signals, targetSignals, "package-metadata");
    if (relative === "docs/architecture/language-adapter-mappings.toml") addSignal(signals, targetSignals, "language-mapping");
    if (relative === "docs/conformance/cross-language-vector-catalog.toml" || relative.startsWith("docs/conformance/fixtures/did/")) {
      addSignal(signals, targetSignals, "shared-vectors");
    }
    if (relative.startsWith("openspec/specs/sdk-ts-parity-conformance/")
        || relative.startsWith("openspec/changes/add-sdk-ts-did-canary/")
        || relative === "docs/adr/0199-use-target-specific-wasm-behind-the-typescript-did-facade.md") {
      addSignal(signals, targetSignals, "target-contract");
    }
    if (/^(?:\.github\/workflows\/factory-contract\.yml|scripts\/(?:assemble-sdk-ts-did-artifacts\.mjs|build-sdk-ts-did-artifacts\.sh)|scripts\/ci\/(?:binding-impact-plan|aggregate-fast-targets|write-target-receipt)\.mjs|scripts\/check-sdk-ts-did-(?:boundaries\.mjs|target\.sh))$/u.test(relative)) {
      addSignal(signals, targetSignals, "ci-contract");
    }
    if (relative === "scripts/tests/binding-target-isolation.mjs"
        || /^nix\/(?:apps|checks|devshells)\/.*(?:sdk-ts|typescript)/u.test(relative)) {
      addSignal(signals, targetSignals, "ci-contract");
    }
    if (bindingPackage.test(relative)) {
      if (/\/(?:browser|web)(?:\/|\.|-)/u.test(relative)) addSignal(signals, targetSignals, "browser-source", ["package-types", "browser", "react-web"]);
      else if (/\/node(?:js)?(?:\/|\.|-)/u.test(relative)) addSignal(signals, targetSignals, "node-source", ["package-types", "nodejs", "react-web"]);
      else if (/\/react-native(?:\/|\.|-)/u.test(relative)) addSignal(signals, targetSignals, "react-native-contract", ["package-types"]);
      else if (/\/react(?:\/|\.|-)/u.test(relative)) addSignal(signals, targetSignals, "react-web-source", ["package-types", "react-web"]);
      else addSignal(signals, targetSignals, "shared-typescript-source");
    }
    if (!knownNeutral.test(relative)) unknownImpact = true;
  }

  if (unknownImpact) addSignal(signals, targetSignals, "unknown-impact");
  const targets = Object.fromEntries(bindingTargets.map((target) => {
    const reasons = [...targetSignals.get(target)].sort();
    const selected = reasons.length > 0;
    return [target, {
      selected,
      expectation: target === "react-native" ? "inapplicable" : selected ? "success" : "inapplicable",
      reasons: target === "react-native" && selected ? [...new Set([...reasons, "adr-0163-native-binding-deferred"])].sort() : reasons,
    }];
  }));
  return {
    schemaVersion: 1,
    repository,
    baseSha,
    headSha,
    changedPaths: [...new Set(normalizedPaths)].sort(),
    cargoDependencyPaths: [...new Set(dependencyPaths)].sort(),
    signals: [...signals].sort(),
    unknownImpact,
    targets,
  };
}

function parseArgs(argv) {
  const values = {};
  for (let index = 0; index < argv.length; index += 1) {
    const option = argv[index];
    if (!["--base", "--head", "--output", "--metadata"].includes(option)) throw new Error(`unknown argument: ${option}`);
    if (argv[index + 1] === undefined) throw new Error(`missing value for ${option}`);
    values[option.slice(2)] = argv[index + 1];
    index += 1;
  }
  if (!values.base || !values.head) throw new Error("Usage: binding-impact-plan.mjs --base REF --head REF [--metadata FILE] [--output FILE]");
  return values;
}

function main() {
  const options = parseArgs(process.argv.slice(2));
  const baseSha = resolveCommit(options.base, "base");
  const headSha = resolveCommit(options.head, "head");
  const paths = git(["diff", "--name-only", "-z", `${baseSha}...${headSha}`]).split("\0").filter(Boolean);
  const metadata = options.metadata ? JSON.parse(readFileSync(path.resolve(root, options.metadata), "utf8")) : cargoMetadata();
  const plan = buildBindingImpactPlan({ baseSha, headSha, paths, dependencyPaths: deriveCargoDependencyPaths(metadata) });
  const rendered = `${JSON.stringify(plan, null, 2)}\n`;
  if (options.output) writeFileSync(path.resolve(root, options.output), rendered, { mode: 0o600 });
  process.stdout.write(rendered);
}

if (path.resolve(process.argv[1] ?? "") === fileURLToPath(import.meta.url)) {
  try {
    main();
  } catch (error) {
    process.stderr.write(`[binding-impact-plan] ${error.message}\n`);
    process.exitCode = 1;
  }
}
