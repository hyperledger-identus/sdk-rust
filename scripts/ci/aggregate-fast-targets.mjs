#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0

import { readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { bindingTargets } from "./binding-impact-plan.mjs";
import { parseJsonWithoutDuplicates } from "../factory-tools/strict-json.mjs";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");
const repository = "hyperledger-identus/sdk-rust";
const shaPattern = /^[0-9a-f]{40}$/u;
const hashPattern = /^[0-9a-f]{64}$/u;
const receiptTargets = Object.freeze(["core-rust", ...bindingTargets]);
const planSignals = new Set([
  "binding-api-schema",
  "browser-source",
  "cargo-dependency-closure",
  "cargo-lock-or-toolchain",
  "ci-contract",
  "language-mapping",
  "node-source",
  "npm-lock",
  "package-metadata",
  "react-native-contract",
  "react-web-source",
  "shared-typescript-source",
  "shared-vectors",
  "target-contract",
  "unknown-impact",
]);
const receiptKeys = Object.freeze([
  "applicability",
  "artifactSha256",
  "durationSeconds",
  "headSha",
  "repository",
  "schemaVersion",
  "status",
  "target",
]);
const planKeys = Object.freeze([
  "baseSha",
  "cargoDependencyPaths",
  "changedPaths",
  "headSha",
  "repository",
  "schemaVersion",
  "signals",
  "targets",
  "unknownImpact",
]);

function exactKeys(value, expected) {
  return value && typeof value === "object" && !Array.isArray(value)
    && JSON.stringify(Object.keys(value).sort()) === JSON.stringify([...expected].sort());
}

export function validateTargetReceipt(receipt, { headSha, target }) {
  const errors = [];
  if (!exactKeys(receipt, receiptKeys)) return ["receipt has unknown or missing fields"];
  if (receipt.schemaVersion !== 1 || receipt.repository !== repository) errors.push("receipt identity is invalid");
  if (receipt.headSha !== headSha || !shaPattern.test(receipt.headSha)) errors.push("receipt head SHA is stale or invalid");
  if (receipt.target !== target || !receiptTargets.includes(receipt.target)) errors.push("receipt target is invalid");
  if (!["selected", "inapplicable"].includes(receipt.applicability)) errors.push("receipt applicability is invalid");
  if (!["success", "failed", "cancelled", "skipped", "timed-out", "inapplicable"].includes(receipt.status)) errors.push("receipt status is invalid");
  if (!Number.isSafeInteger(receipt.durationSeconds) || receipt.durationSeconds < 0 || receipt.durationSeconds > 86_400) errors.push("receipt duration is invalid");
  if (receipt.artifactSha256 !== null && (typeof receipt.artifactSha256 !== "string" || !hashPattern.test(receipt.artifactSha256))) {
    errors.push("receipt artifact SHA-256 is invalid");
  }
  if (receipt.applicability === "inapplicable" && (receipt.status !== "inapplicable" || receipt.artifactSha256 !== null)) {
    errors.push("inapplicable receipt must have inapplicable status and no artifact");
  }
  if (receipt.applicability === "selected" && receipt.status === "inapplicable") errors.push("selected receipt cannot be inapplicable");
  if (receipt.status === "success" && receipt.artifactSha256 === null) errors.push("successful receipt requires an artifact SHA-256");
  return errors;
}

export function createTargetReceipt({ headSha, target, applicability, status, artifactSha256 = null, durationSeconds }) {
  const receipt = {
    schemaVersion: 1,
    repository,
    headSha,
    target,
    applicability,
    status,
    artifactSha256,
    durationSeconds,
  };
  const errors = validateTargetReceipt(receipt, { headSha, target });
  if (errors.length > 0) throw new Error(errors.join("; "));
  return Object.freeze(receipt);
}

function isClosedStringList(value, { maximum = 2000, allowed = null } = {}) {
  return Array.isArray(value)
    && value.length <= maximum
    && value.every((entry) => typeof entry === "string" && entry.length > 0 && entry.length <= 4096 && (allowed === null || allowed.has(entry)))
    && JSON.stringify(value) === JSON.stringify([...new Set(value)].sort());
}

export function validateBindingPlan(plan, headSha) {
  const errors = [];
  if (!exactKeys(plan, planKeys)) return ["binding plan has unknown or missing fields"];
  if (plan.schemaVersion !== 1 || plan.repository !== repository) errors.push("binding plan identity is invalid");
  if (plan.headSha !== headSha || !shaPattern.test(plan.headSha) || !shaPattern.test(plan.baseSha)) errors.push("binding plan Git identity is stale or invalid");
  if (!isClosedStringList(plan.changedPaths)
      || !isClosedStringList(plan.cargoDependencyPaths)
      || !isClosedStringList(plan.signals, { maximum: planSignals.size, allowed: planSignals })
      || typeof plan.unknownImpact !== "boolean") {
    errors.push("binding plan evidence collections are invalid");
  }
  if (plan.unknownImpact !== plan.signals?.includes("unknown-impact")) errors.push("binding plan unknown-impact signal is inconsistent");
  if (!plan.targets || JSON.stringify(Object.keys(plan.targets).sort()) !== JSON.stringify([...bindingTargets].sort())) {
    errors.push("binding plan target set is incomplete");
  } else {
    for (const target of bindingTargets) {
      const entry = plan.targets[target];
      if (!exactKeys(entry, ["expectation", "reasons", "selected"])
          || typeof entry.selected !== "boolean"
          || !["success", "inapplicable"].includes(entry.expectation)
          || !isClosedStringList(entry.reasons, { maximum: planSignals.size + 1 })) {
        errors.push(`binding plan target ${target} is invalid`);
      }
      if (entry?.reasons?.some((reason) => reason !== "adr-0163-native-binding-deferred" && (!planSignals.has(reason) || !plan.signals.includes(reason)))) {
        errors.push(`binding plan target ${target} has an unknown reason`);
      }
      if (entry?.expectation === "success" && !entry.selected) errors.push(`binding plan target ${target} cannot require success while unselected`);
      if (target === "react-native" && entry?.expectation !== "inapplicable") errors.push("React Native must remain structurally inapplicable");
    }
  }
  if (plan.unknownImpact === true && bindingTargets.some((target) => plan.targets?.[target]?.selected !== true)) {
    errors.push("unknown impact must select every binding target");
  }
  return errors;
}

export function aggregateFastTargets({ headSha, plan, receipts }) {
  if (!shaPattern.test(headSha)) throw new Error("aggregation requires an exact head SHA");
  const failures = validateBindingPlan(plan, headSha);
  const outcomes = {};
  const expected = {
    "core-rust": { expectation: "success", selected: true },
    ...plan.targets,
  };
  for (const target of receiptTargets) {
    const receipt = receipts[target];
    if (!receipt) {
      failures.push(`${target}: missing receipt`);
      outcomes[target] = "missing";
      continue;
    }
    const receiptErrors = validateTargetReceipt(receipt, { headSha, target });
    for (const error of receiptErrors) failures.push(`${target}: ${error}`);
    const targetExpectation = expected[target]?.expectation ?? "inapplicable";
    if (targetExpectation === "success") {
      if (receipt.applicability !== "selected" || receipt.status !== "success") failures.push(`${target}: selected target did not succeed`);
    } else if (receipt.applicability !== "inapplicable" || receipt.status !== "inapplicable") {
      failures.push(`${target}: target must emit an inapplicable receipt`);
    }
    outcomes[target] = receipt.status;
  }
  return {
    schemaVersion: 1,
    repository,
    headSha,
    verdict: failures.length === 0 ? "success" : "failure",
    coreResult: outcomes["core-rust"] ?? "missing",
    targetResults: Object.fromEntries(bindingTargets.map((target) => [target, outcomes[target] ?? "missing"])),
    failures: [...new Set(failures)].sort(),
  };
}

function readBoundedJson(relative, label) {
  const absolute = path.resolve(root, relative);
  const bytes = readFileSync(absolute);
  if (bytes.length === 0 || bytes.length > 128 * 1024) throw new Error(`${label} must be between 1 byte and 128 KiB`);
  return parseJsonWithoutDuplicates(bytes.toString("utf8"));
}

function parseArgs(argv) {
  const values = { receipts: {} };
  for (let index = 0; index < argv.length; index += 1) {
    const option = argv[index];
    const value = argv[index + 1];
    if (value === undefined) throw new Error(`missing value for ${option}`);
    if (option === "--receipt") {
      const separator = value.indexOf("=");
      if (separator < 1) throw new Error("--receipt requires TARGET=FILE");
      const target = value.slice(0, separator);
      if (values.receipts[target] !== undefined) throw new Error(`duplicate receipt argument: ${target}`);
      values.receipts[target] = value.slice(separator + 1);
    } else if (["--head", "--plan", "--core", "--output"].includes(option)) {
      values[option.slice(2)] = value;
    } else {
      throw new Error(`unknown argument: ${option}`);
    }
    index += 1;
  }
  if (!values.head || !values.plan || !values.core) {
    throw new Error("Usage: aggregate-fast-targets.mjs --head SHA --plan FILE --core FILE --receipt TARGET=FILE... [--output FILE]");
  }
  return values;
}

function main() {
  const options = parseArgs(process.argv.slice(2));
  const receipts = { "core-rust": readBoundedJson(options.core, "core receipt") };
  for (const [target, file] of Object.entries(options.receipts)) {
    if (!bindingTargets.includes(target)) throw new Error(`unknown receipt target: ${target}`);
    receipts[target] = readBoundedJson(file, `${target} receipt`);
  }
  const result = aggregateFastTargets({ headSha: options.head, plan: readBoundedJson(options.plan, "binding plan"), receipts });
  const rendered = `${JSON.stringify(result, null, 2)}\n`;
  if (options.output) writeFileSync(path.resolve(root, options.output), rendered, { mode: 0o600 });
  process.stdout.write(rendered);
  if (result.verdict !== "success") process.exitCode = 1;
}

if (path.resolve(process.argv[1] ?? "") === fileURLToPath(import.meta.url)) {
  try {
    main();
  } catch (error) {
    process.stderr.write(`[aggregate-fast-targets] ${error.message}\n`);
    process.exitCode = 1;
  }
}
