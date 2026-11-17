#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0

import { mkdir, rename, writeFile } from "node:fs/promises";
import path from "node:path";
import { randomUUID } from "node:crypto";
import { fileURLToPath } from "node:url";
import { createTargetReceipt } from "./aggregate-fast-targets.mjs";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");

function parseArguments(argv) {
  const values = new Map();
  for (let index = 0; index < argv.length; index += 2) {
    const option = argv[index];
    const value = argv[index + 1];
    if (!option?.startsWith("--") || value === undefined) {
      throw new Error("receipt arguments must be --name value pairs");
    }
    if (values.has(option)) throw new Error(`duplicate argument: ${option}`);
    values.set(option, value);
  }
  const allowed = new Set([
    "--head",
    "--target",
    "--applicability",
    "--status",
    "--artifact-sha256",
    "--duration-seconds",
    "--output",
  ]);
  for (const key of values.keys()) if (!allowed.has(key)) throw new Error(`unknown argument: ${key}`);
  for (const key of allowed) if (!values.has(key)) throw new Error(`missing argument: ${key}`);
  const durationSeconds = Number(values.get("--duration-seconds"));
  return Object.freeze({
    headSha: values.get("--head"),
    target: values.get("--target"),
    applicability: values.get("--applicability"),
    status: values.get("--status"),
    artifactSha256: values.get("--artifact-sha256") === "none" ? null : values.get("--artifact-sha256"),
    durationSeconds,
    output: path.resolve(root, values.get("--output")),
  });
}

async function main() {
  const options = parseArguments(process.argv.slice(2));
  const receipt = createTargetReceipt(options);
  await mkdir(path.dirname(options.output), { recursive: true });
  const temporary = `${options.output}.${process.pid}.${randomUUID()}.tmp`;
  await writeFile(temporary, `${JSON.stringify(receipt, null, 2)}\n`, {
    flag: "wx",
    mode: 0o600,
  });
  await rename(temporary, options.output);
  process.stdout.write(`${JSON.stringify(receipt)}\n`);
}

main().catch((error) => {
  process.stderr.write(`[write-target-receipt] ${error.message}\n`);
  process.exitCode = 1;
});
