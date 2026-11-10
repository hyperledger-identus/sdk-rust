#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0

import { performance } from "node:perf_hooks";
import { readFile, writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

import { createDidProvider } from "../dist/node.js";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const output = path.resolve(process.env.SDK_TS_DID_MEASUREMENTS ?? path.join(root, "../../target/sdk-ts-did-measurements.json"));
const warmup = 21;
const samples = 101;

function percentile(values, fraction) {
  const sorted = [...values].sort((left, right) => left - right);
  return sorted[Math.min(sorted.length - 1, Math.floor(sorted.length * fraction))];
}

function measure(operation) {
  for (let index = 0; index < warmup; index += 1) operation();
  const durations = [];
  for (let index = 0; index < samples; index += 1) {
    const start = performance.now();
    operation();
    durations.push(performance.now() - start);
  }
  return Object.freeze({
    samples,
    warmup,
    p50Milliseconds: percentile(durations, 0.5),
    p95Milliseconds: percentile(durations, 0.95),
  });
}

const coldStart = performance.now();
const provider = await createDidProvider();
const initializationMilliseconds = performance.now() - coldStart;
const ordinary = measure(() => provider.parseDid("did:example:measurement"));
const nearLimitValue = `did:example:${"a".repeat(131_000)}`;
const nearLimit = measure(() => provider.parseDid(nearLimitValue));
const manifest = JSON.parse(await readFile(new URL("../artifacts/manifest.json", import.meta.url), "utf8"));

const report = Object.freeze({
  schemaVersion: 1,
  status: "target-not-supported",
  environment: Object.freeze({
    node: process.version,
    platform: process.platform,
    architecture: process.arch,
  }),
  artifact: Object.freeze({
    assemblySha256: manifest.assemblySha256,
    webBytes: manifest.targets.web.artifacts.reduce((total, entry) => total + entry.bytes, 0),
    nodejsBytes: manifest.targets.nodejs.artifacts.reduce((total, entry) => total + entry.bytes, 0),
  }),
  initializationMilliseconds,
  ordinary,
  nearLimit,
  rssBytes: process.memoryUsage().rss,
  limitations: "Directional local measurement only; no accepted support or regression budget.",
});

await writeFile(output, `${JSON.stringify(report, null, 2)}\n`, { mode: 0o600 });
process.stdout.write(`${JSON.stringify(report)}\n`);
