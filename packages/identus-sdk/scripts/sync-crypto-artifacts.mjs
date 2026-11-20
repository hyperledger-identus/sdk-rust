#!/usr/bin/env node
import { cp, lstat, readdir, rm } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const packageRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const repositoryRoot = path.resolve(packageRoot, "../..");
const source = path.resolve(
  process.env.SDK_TS_CRYPTO_ARTIFACTS
    ?? path.join(repositoryRoot, "target/sdk-ts-crypto-artifacts"),
);
const output = path.join(packageRoot, "artifacts/crypto");
const expected = [
  "identus_wasm_crypto.d.ts",
  "identus_wasm_crypto.js",
  "identus_wasm_crypto_bg.wasm",
  "identus_wasm_crypto_bg.wasm.d.ts",
  "package.json",
];

async function assertDirectory(directory) {
  const info = await lstat(directory);
  if (!info.isDirectory() || info.isSymbolicLink()) {
    throw new Error(`artifact path is not a regular directory: ${directory}`);
  }
  const entries = (await readdir(directory)).sort();
  if (JSON.stringify(entries) !== JSON.stringify(expected)) {
    throw new Error(`crypto artifact inventory mismatch: ${directory}`);
  }
}

await assertDirectory(path.join(source, "web"));
await assertDirectory(path.join(source, "nodejs"));
await rm(output, { recursive: true, force: true });
await cp(source, output, { recursive: true, dereference: false, errorOnExist: true });
process.stdout.write("synced private crypto artifacts\n");
