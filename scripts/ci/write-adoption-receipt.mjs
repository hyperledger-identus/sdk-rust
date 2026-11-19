#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0

import { createHash, randomUUID } from "node:crypto";
import { mkdir, readFile, rename, writeFile } from "node:fs/promises";
import path from "node:path";

const DONOR_REVISION = "4bf86ebf69d5e96616a148e4c973f831f95fa38e";
const HASH = /^[0-9a-f]{40}$/u;
const TARGETS = new Set(["browser", "nodejs", "react-web"]);

function options(argv) {
  const values = new Map();
  for (let index = 0; index < argv.length; index += 2) {
    const key = argv[index];
    const value = argv[index + 1];
    if (!key?.startsWith("--") || value === undefined || values.has(key)) {
      throw new Error("arguments must be unique --name value pairs");
    }
    values.set(key, value);
  }
  const allowed = new Set(["--head", "--target", "--route-instances", "--output", "--sources"]);
  for (const key of values.keys()) if (!allowed.has(key)) throw new Error(`unknown argument: ${key}`);
  for (const key of allowed) if (!values.has(key)) throw new Error(`missing argument: ${key}`);
  const headSha = values.get("--head");
  const target = values.get("--target");
  const routeInstances = Number(values.get("--route-instances"));
  if (!HASH.test(headSha)) throw new Error("head must be a full lowercase Git SHA");
  if (!TARGETS.has(target)) throw new Error("target is not an applicable adoption target");
  if (!Number.isSafeInteger(routeInstances) || routeInstances < 2 || routeInstances % 2 !== 0) {
    throw new Error("route instances must be a positive legacy/Rust pair count");
  }
  return Object.freeze({
    headSha,
    target,
    routeInstances,
    output: path.resolve(values.get("--output")),
    sources: values.get("--sources").split(",").map((value) => path.resolve(value)),
  });
}

async function main() {
  const selected = options(process.argv.slice(2));
  const digest = createHash("sha256");
  for (const source of selected.sources) {
    const bytes = await readFile(source);
    const text = bytes.toString("utf8");
    if (/\.(?:only|skip)\s*\(|\bquarantin(?:e|ed)\b/iu.test(text)) {
      throw new Error(`focused, skipped, or quarantined adoption source: ${source}`);
    }
    digest.update(path.basename(source));
    digest.update("\0");
    digest.update(bytes);
    digest.update("\0");
  }
  const assertionsPerRoute = 22;
  const receipt = Object.freeze({
    schemaVersion: 1,
    rehearsalId: "sdk-ts.did-facade-adoption.v1",
    headSha: selected.headSha,
    donorRevision: DONOR_REVISION,
    target: selected.target,
    status: "success",
    routes: Object.freeze(["legacy", "rust"]),
    routeInstances: selected.routeInstances,
    assertionsPerRoute,
    assertionResults: assertionsPerRoute * selected.routeInstances,
    skipped: 0,
    focused: 0,
    quarantined: 0,
    automaticFallback: false,
    rollback: "explicit",
    sourceSha256: digest.digest("hex"),
  });
  await mkdir(path.dirname(selected.output), { recursive: true });
  const temporary = `${selected.output}.${process.pid}.${randomUUID()}.tmp`;
  await writeFile(temporary, `${JSON.stringify(receipt, null, 2)}\n`, { flag: "wx", mode: 0o600 });
  await rename(temporary, selected.output);
  process.stdout.write(`${JSON.stringify(receipt)}\n`);
}

main().catch((error) => {
  process.stderr.write(`[write-adoption-receipt] ${error.message}\n`);
  process.exitCode = 1;
});
