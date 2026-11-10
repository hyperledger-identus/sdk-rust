#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0

import { cp, lstat, readFile, readdir, rm } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const packageRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const repositoryRoot = path.resolve(packageRoot, "../..");
const source = path.resolve(
  process.env.SDK_TS_DID_ARTIFACTS
    ?? path.join(repositoryRoot, "target/sdk-ts-did-artifacts"),
);
const output = path.join(packageRoot, "artifacts");
const expectedRoot = ["manifest.json", "nodejs", "web"];
const expectedTarget = [
  "identus_wasm_did.d.ts",
  "identus_wasm_did.js",
  "identus_wasm_did_bg.wasm",
  "identus_wasm_did_bg.wasm.d.ts",
  "package.json",
];

async function assertDirectory(directory, expected) {
  const info = await lstat(directory);
  if (!info.isDirectory() || info.isSymbolicLink()) {
    throw new Error(`artifact path is not a regular directory: ${directory}`);
  }
  const entries = (await readdir(directory)).sort();
  if (JSON.stringify(entries) !== JSON.stringify(expected)) {
    throw new Error(`artifact inventory mismatch: ${directory}`);
  }
}

await assertDirectory(source, expectedRoot);
await assertDirectory(path.join(source, "web"), expectedTarget);
await assertDirectory(path.join(source, "nodejs"), expectedTarget);
const manifest = JSON.parse(await readFile(path.join(source, "manifest.json"), "utf8"));
if (manifest.layout !== "identus.sdk-ts-did.private-artifacts.v1"
    || manifest.bindingApiVersion !== 1) {
  throw new Error("artifact manifest identity mismatch");
}

await rm(output, { recursive: true, force: true });
await cp(source, output, { recursive: true, dereference: false, errorOnExist: true });
process.stdout.write(`synced private DID artifacts: ${manifest.assemblySha256}\n`);
