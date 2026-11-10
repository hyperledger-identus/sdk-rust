#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0

import { lstatSync, readFileSync, readdirSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const packageRoot = path.join(root, "packages/identus-sdk");
const sourceRoot = path.join(packageRoot, "src");
const legacyName = /apollo|castor|mercury|pluto|pollux|edge[ _-]?agent/iu;
const importPattern = /(?:from\s+|import\s*\()(["'])([^"']+)\1/gu;

function fail(message) {
  process.stderr.write(`sdk-ts-did-boundary: ${message}\n`);
  process.exitCode = 1;
}

function sourceFiles(directory) {
  const files = [];
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    const absolute = path.join(directory, entry.name);
    const info = lstatSync(absolute);
    if (info.isSymbolicLink()) {
      fail(`source path must not be a symlink: ${path.relative(root, absolute)}`);
    } else if (info.isDirectory()) {
      files.push(...sourceFiles(absolute));
    } else if (info.isFile() && /\.(?:ts|tsx)$/u.test(entry.name)) {
      files.push(absolute);
    }
  }
  return files.sort();
}

for (const file of sourceFiles(sourceRoot)) {
  const relative = path.relative(root, file).replaceAll(path.sep, "/");
  const text = readFileSync(file, "utf8");
  if (legacyName.test(text)) fail(`${relative} contains a historical public component name`);
  for (const match of text.matchAll(importPattern)) {
    const specifier = match[2];
    if (!specifier.startsWith(".")) {
      if (specifier !== "react") fail(`${relative} imports unapproved runtime package ${specifier}`);
      continue;
    }
    const resolved = path.resolve(path.dirname(file), specifier);
    if (!resolved.startsWith(`${packageRoot}${path.sep}`)) {
      fail(`${relative} escapes the TypeScript package through ${specifier}`);
    }
    if (specifier.includes("artifacts/") && !/[\\/](?:browser|node)\.tsx?$/u.test(file)) {
      fail(`${relative} reaches generated artifacts outside an explicit target loader`);
    }
  }
}

const reactSource = readFileSync(path.join(sourceRoot, "react.tsx"), "utf8");
for (const forbidden of ["./internal/", "../artifacts/", "parseDid(", "parseDidUrl("]) {
  if (reactSource.includes(forbidden)) fail(`React lifecycle source contains forbidden domain dependency ${forbidden}`);
}

for (const manifest of ["Cargo.toml", ...readdirSync(path.join(root, "crates"), { withFileTypes: true })
  .filter((entry) => entry.isDirectory())
  .map((entry) => `crates/${entry.name}/Cargo.toml`)]) {
  const absolute = path.join(root, manifest);
  try {
    const text = readFileSync(absolute, "utf8");
    if (text.includes("packages/identus-sdk") || text.includes("@hyperledger/identus-sdk")) {
      fail(`${manifest} reverses the Rust-to-TypeScript dependency direction`);
    }
  } catch (error) {
    if (error?.code !== "ENOENT") throw error;
  }
}

if (process.exitCode === undefined) {
  process.stdout.write("sdk-ts-did-boundary: one-way Rust/WASM/TypeScript/React graph passed\n");
}
