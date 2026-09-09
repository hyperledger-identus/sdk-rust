#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0

import { execFileSync } from "node:child_process";
import { createHash, randomBytes } from "node:crypto";
import {
  copyFileSync,
  existsSync,
  lstatSync,
  mkdirSync,
  readFileSync,
  readlinkSync,
  realpathSync,
  renameSync,
  rmSync,
  symlinkSync,
  writeFileSync,
} from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

export const PI_PACKAGE_CACHE_SCHEMA = 1;

function command(program, args, cwd) {
  return execFileSync(program, args, {
    cwd,
    encoding: "utf8",
    stdio: ["ignore", "pipe", "pipe"],
  }).trim();
}

function readJson(file) {
  return JSON.parse(readFileSync(file, "utf8"));
}

function sameJson(left, right) {
  return JSON.stringify(left) === JSON.stringify(right);
}

function isWithin(candidate, parent) {
  const relative = path.relative(parent, candidate);
  return relative === "" || (!relative.startsWith(`..${path.sep}`) && relative !== "..");
}

export function parseExactNpmSource(source) {
  if (typeof source !== "string") throw new Error("Pi package source must be a string");
  const match = /^npm:([a-z0-9-]+)@([0-9]+\.[0-9]+\.[0-9]+)$/u.exec(source);
  if (!match) throw new Error(`Pi package source is not an exact npm version: ${source}`);
  return { source, name: match[1], version: match[2], installSpec: `${match[1]}@${match[2]}` };
}

export function packageCacheIdentity(inputs) {
  if (!Array.isArray(inputs.packages) || inputs.packages.length === 0) {
    throw new Error("Pi package cache requires at least one exact package source");
  }
  const normalized = {
    schemaVersion: PI_PACKAGE_CACHE_SCHEMA,
    piVersion: String(inputs.piVersion),
    nodeVersion: String(inputs.nodeVersion),
    npmVersion: String(inputs.npmVersion),
    packageLockSha256: String(inputs.packageLockSha256),
    packages: inputs.packages.map((source) => parseExactNpmSource(source).source),
  };
  if (!/^v[0-9]+\.[0-9]+\.[0-9]+$/u.test(normalized.nodeVersion)) {
    throw new Error("Pi package cache requires an exact Node version");
  }
  for (const [name, version] of [["Pi", normalized.piVersion], ["npm", normalized.npmVersion]]) {
    if (!/^[0-9]+\.[0-9]+\.[0-9]+$/u.test(version)) {
      throw new Error(`Pi package cache requires an exact ${name} version`);
    }
  }
  if (new Set(normalized.packages).size !== normalized.packages.length) {
    throw new Error("Pi package cache does not permit duplicate package sources");
  }
  if (!/^[a-f0-9]{64}$/u.test(normalized.packageLockSha256)) {
    throw new Error("Pi package cache requires an exact package-lock SHA-256");
  }
  const digest = createHash("sha256").update(JSON.stringify(normalized)).digest("hex");
  return { digest, inputs: normalized };
}

export function deriveCacheLayout({ repositoryRoot, primaryWorktree, worktrees, digest }) {
  const root = realpathSync(repositoryRoot);
  const primary = realpathSync(primaryWorktree);
  const parent = realpathSync(path.dirname(primary));
  const externalRoot = path.join(parent, `.${path.basename(primary)}-factory`);
  const cacheRoot = path.join(externalRoot, "pi-packages", `v${PI_PACKAGE_CACHE_SCHEMA}`);
  const cachePath = path.join(cacheRoot, digest);
  const registered = worktrees.map((entry) => existsSync(entry) ? realpathSync(entry) : path.resolve(entry));

  for (const worktree of registered) {
    if (isWithin(cachePath, worktree)) {
      throw new Error(`Pi package cache must be outside registered worktrees: ${cachePath}`);
    }
  }
  if (!registered.includes(root)) {
    throw new Error(`current repository root is not a registered worktree: ${root}`);
  }
  return {
    repositoryRoot: root,
    primaryWorktree: primary,
    externalRoot,
    cacheRoot,
    cachePath,
    projectLink: path.join(root, ".pi", "npm"),
  };
}

function parseWorktreePaths(text) {
  return text
    .split("\n")
    .filter((line) => line.startsWith("worktree "))
    .map((line) => line.slice("worktree ".length));
}

export function resolveGitCacheContext(repositoryRoot) {
  const root = realpathSync(repositoryRoot);
  const worktrees = parseWorktreePaths(command("git", ["worktree", "list", "--porcelain"], root));
  if (worktrees.length === 0) throw new Error("Git did not report a primary worktree");
  return { repositoryRoot: root, primaryWorktree: worktrees[0], worktrees };
}

function ensureSafeDirectory(directory, existingBoundary) {
  const boundary = realpathSync(existingBoundary);
  if (!isWithin(directory, boundary) || directory === boundary) {
    throw new Error(`cache directory escapes its derived boundary: ${directory}`);
  }
  const relative = path.relative(boundary, directory);
  let current = boundary;
  for (const segment of relative.split(path.sep)) {
    current = path.join(current, segment);
    if (!existsSync(current)) {
      try {
        mkdirSync(current, { mode: 0o700 });
      } catch (error) {
        if (error?.code !== "EEXIST") throw error;
      }
    }
    const stat = lstatSync(current);
    if (stat.isSymbolicLink() || !stat.isDirectory()) {
      throw new Error(`cache ancestry must contain regular directories only: ${current}`);
    }
  }
}

function expectedMarker(identity) {
  return {
    schemaVersion: PI_PACKAGE_CACHE_SCHEMA,
    digest: identity.digest,
    inputs: identity.inputs,
    lifecycleScripts: "disabled",
  };
}

export function verifyPackageCache(cachePath, identity) {
  const stat = lstatSync(cachePath);
  if (stat.isSymbolicLink() || !stat.isDirectory()) {
    throw new Error(`Pi package cache must be a regular directory: ${cachePath}`);
  }
  const markerPath = path.join(cachePath, ".sdk-rust-pi-cache.json");
  if (!existsSync(markerPath) || lstatSync(markerPath).isSymbolicLink()) {
    throw new Error(`Pi package cache is incomplete; missing regular marker: ${markerPath}`);
  }
  const marker = readJson(markerPath);
  if (!sameJson(marker, expectedMarker(identity))) {
    throw new Error(`Pi package cache marker does not match its identity: ${cachePath}`);
  }
  for (const source of identity.inputs.packages) {
    const parsed = parseExactNpmSource(source);
    const modulesPath = path.join(cachePath, "node_modules");
    const packagePath = path.join(modulesPath, parsed.name);
    const manifestPath = path.join(packagePath, "package.json");
    for (const directory of [modulesPath, packagePath]) {
      if (!existsSync(directory)) throw new Error(`Pi package cache is missing directory: ${directory}`);
      const directoryStat = lstatSync(directory);
      if (directoryStat.isSymbolicLink() || !directoryStat.isDirectory()) {
        throw new Error(`Pi package cache contains an unsafe package path: ${directory}`);
      }
    }
    if (!existsSync(manifestPath) || lstatSync(manifestPath).isSymbolicLink()) {
      throw new Error(`Pi package cache is missing ${parsed.installSpec}`);
    }
    const manifest = readJson(manifestPath);
    if (manifest.name !== parsed.name || manifest.version !== parsed.version) {
      throw new Error(`Pi package cache has the wrong version for ${parsed.name}`);
    }
  }
  return expectedMarker(identity);
}

function validateProjectLink(projectLink, cachePath) {
  if (!lstatExists(projectLink)) return { state: "missing" };
  const stat = lstatSync(projectLink);
  if (!stat.isSymbolicLink()) {
    throw new Error(`refusing to replace operator-owned Pi package data: ${projectLink}`);
  }
  const linkTarget = path.resolve(path.dirname(projectLink), readlinkSync(projectLink));
  if (realpathSync(linkTarget) !== realpathSync(cachePath)) {
    throw new Error(`Pi package link points to an unexpected cache: ${projectLink}`);
  }
  return { state: "ready", linkTarget: realpathSync(linkTarget) };
}

function linkProjectStore(projectLink, cachePath) {
  const piDirectory = path.dirname(projectLink);
  const piStat = lstatSync(piDirectory);
  if (piStat.isSymbolicLink() || !piStat.isDirectory()) {
    throw new Error(`project .pi path must be a regular directory: ${piDirectory}`);
  }
  const current = validateProjectLink(projectLink, cachePath);
  if (current.state === "ready") return current;

  try {
    symlinkSync(cachePath, projectLink, "dir");
  } catch (error) {
    if (error?.code !== "EEXIST") throw error;
    const raced = validateProjectLink(projectLink, cachePath);
    if (raced.state !== "ready") throw error;
  }
  return validateProjectLink(projectLink, cachePath);
}

function lstatExists(candidate) {
  try {
    lstatSync(candidate);
    return true;
  } catch (error) {
    if (error?.code === "ENOENT") return false;
    throw error;
  }
}

function installExactPackages(stagingPath, identity, repositoryRoot) {
  const manifestRoot = path.join(repositoryRoot, ".pi", "package-runtime");
  copyFileSync(path.join(manifestRoot, "package.json"), path.join(stagingPath, "package.json"));
  copyFileSync(path.join(manifestRoot, "package-lock.json"), path.join(stagingPath, "package-lock.json"));
  execFileSync(
    "npm",
    ["ci", "--prefix", stagingPath, "--legacy-peer-deps", "--ignore-scripts", "--no-audit", "--no-fund"],
    { cwd: repositoryRoot, stdio: "inherit" },
  );
}

function publishCache({ layout, identity, install }) {
  if (lstatExists(layout.cachePath)) {
    verifyPackageCache(layout.cachePath, identity);
    return { populated: false };
  }

  const stagingPath = path.join(
    layout.cacheRoot,
    `.staging-${identity.digest}-${process.pid}-${randomBytes(6).toString("hex")}`,
  );
  mkdirSync(stagingPath);
  try {
    install(stagingPath, identity, layout.repositoryRoot);
    for (const source of identity.inputs.packages) {
      const parsed = parseExactNpmSource(source);
      const manifest = readJson(path.join(stagingPath, "node_modules", parsed.name, "package.json"));
      if (manifest.name !== parsed.name || manifest.version !== parsed.version) {
        throw new Error(`installed Pi package does not match ${parsed.installSpec}`);
      }
    }
    writeFileSync(
      path.join(stagingPath, ".sdk-rust-pi-cache.json"),
      `${JSON.stringify(expectedMarker(identity), null, 2)}\n`,
    );
    try {
      renameSync(stagingPath, layout.cachePath);
    } catch (error) {
      if (!lstatExists(layout.cachePath)) throw error;
      verifyPackageCache(layout.cachePath, identity);
      rmSync(stagingPath, { recursive: true, force: true });
      return { populated: false, converged: true };
    }
    verifyPackageCache(layout.cachePath, identity);
    return { populated: true };
  } catch (error) {
    const failure = error instanceof Error ? error : new Error(String(error));
    const retainedPath = lstatExists(stagingPath) ? stagingPath : layout.cachePath;
    failure.message = `${failure.message}; incomplete cache state retained for inspection: ${retainedPath}`;
    throw failure;
  }
}

export function runtimeInputs(repositoryRoot) {
  const settings = readJson(path.join(repositoryRoot, ".pi", "settings.json"));
  const packageRuntime = readPackageRuntime(repositoryRoot, settings.packages);
  return {
    piVersion: command("pi", ["--version"], repositoryRoot),
    nodeVersion: command("node", ["--version"], repositoryRoot),
    npmVersion: command("npm", ["--version"], repositoryRoot),
    packageLockSha256: packageRuntime.packageLockSha256,
    packages: settings.packages,
  };
}

export function readPackageRuntime(repositoryRoot, packageSources) {
  const manifestRoot = path.join(repositoryRoot, ".pi", "package-runtime");
  const packagePath = path.join(manifestRoot, "package.json");
  const lockPath = path.join(manifestRoot, "package-lock.json");
  for (const file of [packagePath, lockPath]) {
    if (!existsSync(file) || lstatSync(file).isSymbolicLink() || !lstatSync(file).isFile()) {
      throw new Error(`Pi package runtime requires a regular tracked file: ${file}`);
    }
  }
  const expectedDependencies = Object.fromEntries(
    packageSources.map((source) => {
      const parsed = parseExactNpmSource(source);
      return [parsed.name, parsed.version];
    }),
  );
  const manifest = readJson(packagePath);
  const lock = readJson(lockPath);
  if (manifest.private !== true || !sameJson(manifest.dependencies, expectedDependencies)) {
    throw new Error("Pi package runtime manifest does not match exact project package settings");
  }
  if (lock.lockfileVersion !== 3 || !sameJson(lock.packages?.[""]?.dependencies, expectedDependencies)) {
    throw new Error("Pi package lock does not match the exact runtime manifest");
  }
  for (const entry of Object.values(lock.packages ?? {})) {
    if (entry.resolved === undefined) continue;
    let resolved;
    try {
      resolved = new URL(entry.resolved);
    } catch {
      throw new Error(`Pi package lock contains an invalid resolved URL: ${entry.resolved}`);
    }
    if (resolved.protocol !== "https:" || resolved.hostname !== "registry.npmjs.org") {
      throw new Error(`Pi package lock contains a non-registry dependency: ${entry.resolved}`);
    }
    if (typeof entry.integrity !== "string" || !entry.integrity.startsWith("sha512-")) {
      throw new Error(`Pi package lock dependency lacks SHA-512 integrity: ${entry.resolved}`);
    }
  }
  const lockBytes = readFileSync(lockPath);
  return {
    manifestRoot,
    packageLockSha256: createHash("sha256").update(lockBytes).digest("hex"),
  };
}

export function inspectPiPackageCache({ repositoryRoot, inputs = runtimeInputs(repositoryRoot) }) {
  const identity = packageCacheIdentity(inputs);
  const gitContext = resolveGitCacheContext(repositoryRoot);
  const layout = deriveCacheLayout({ ...gitContext, digest: identity.digest });
  if (!lstatExists(layout.cachePath)) {
    if (lstatExists(layout.projectLink)) validateProjectLink(layout.projectLink, layout.cachePath);
    return { state: "absent", identity, layout };
  }
  verifyPackageCache(layout.cachePath, identity);
  const link = validateProjectLink(layout.projectLink, layout.cachePath);
  return { state: link.state === "ready" ? "ready" : "unlinked", identity, layout };
}

export function preparePiPackageCache({
  repositoryRoot,
  inputs = runtimeInputs(repositoryRoot),
  gitContext = resolveGitCacheContext(repositoryRoot),
  install = installExactPackages,
}) {
  const identity = packageCacheIdentity(inputs);
  const layout = deriveCacheLayout({ ...gitContext, digest: identity.digest });
  ensureSafeDirectory(layout.cacheRoot, path.dirname(layout.primaryWorktree));
  const publication = publishCache({ layout, identity, install });
  const link = linkProjectStore(layout.projectLink, layout.cachePath);
  return { state: "ready", identity, layout, publication, link };
}

async function main() {
  const repositoryRoot = realpathSync(path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../.."));
  const auditOnly = process.argv.includes("--audit");
  const result = auditOnly
    ? inspectPiPackageCache({ repositoryRoot })
    : preparePiPackageCache({ repositoryRoot });
  if (process.argv.includes("--json")) {
    process.stdout.write(`${JSON.stringify(result, null, 2)}\n`);
  } else if (result.state === "ready") {
    process.stdout.write(`factory: Pi package cache ready: ${result.layout.cachePath}\n`);
  } else {
    process.stdout.write(`factory: Pi package cache ${result.state}: ${result.layout.cachePath}\n`);
  }
}

if (path.resolve(process.argv[1] ?? "") === fileURLToPath(import.meta.url)) {
  main().catch((error) => {
    process.stderr.write(`[pi-cache] ${error.message}\n`);
    process.exitCode = 1;
  });
}
