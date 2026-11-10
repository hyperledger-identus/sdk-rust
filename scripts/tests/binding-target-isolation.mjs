#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0

import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";
import { aggregateFastTargets, createTargetReceipt, validateBindingPlan, validateTargetReceipt } from "../ci/aggregate-fast-targets.mjs";
import { bindingTargets, buildBindingImpactPlan, deriveCargoDependencyPaths } from "../ci/binding-impact-plan.mjs";

const headSha = "b".repeat(40);
const baseSha = "a".repeat(40);
const artifactSha256 = "c".repeat(64);
const repositoryRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");

function receipt(target, status = "success", applicability = "selected", overrides = {}) {
  return {
    schemaVersion: 1,
    repository: "hyperledger-identus/sdk-rust",
    headSha,
    target,
    applicability,
    status,
    artifactSha256: status === "success" ? artifactSha256 : null,
    durationSeconds: 1,
    ...overrides,
  };
}

function inapplicable(target) {
  return receipt(target, "inapplicable", "inapplicable");
}

function plan(paths) {
  return buildBindingImpactPlan({
    baseSha,
    headSha,
    paths,
    dependencyPaths: ["crates/did", "crates/wasm-did"],
  });
}

function receiptsFor(bindingPlan) {
  return {
    "core-rust": receipt("core-rust"),
    ...Object.fromEntries(bindingTargets.map((target) => [
      target,
      bindingPlan.targets[target].expectation === "success" ? receipt(target) : inapplicable(target),
    ])),
  };
}

test("Cargo dependency closure is derived transitively", () => {
  const metadata = {
    packages: [
      { name: "identus-wasm-did", manifest_path: "/repo/crates/wasm-did/Cargo.toml", dependencies: [{ name: "identus-did" }, { name: "wasm-bindgen" }] },
      { name: "identus-did", manifest_path: "/repo/crates/did/Cargo.toml", dependencies: [{ name: "identus-core" }] },
      { name: "identus-core", manifest_path: "/repo/crates/core/Cargo.toml", dependencies: [] },
    ],
  };
  assert.deepEqual(deriveCargoDependencyPaths(metadata, "identus-wasm-did", "/repo"), ["crates/core", "crates/did", "crates/wasm-did"]);
});

test("binding schema, dependency, locks, mappings, vectors and contracts select every target", () => {
  const fixtures = [
    "crates/wasm-did/api/typescript-api.d.ts",
    "crates/did/src/lib.rs",
    "Cargo.lock",
    "packages/identus-sdk/package-lock.json",
    "packages/identus-sdk/package.json",
    "docs/architecture/language-adapter-mappings.toml",
    "docs/conformance/fixtures/did/v1/vectors.json",
    "openspec/changes/add-sdk-ts-did-canary/specs/sdk-ts-did-facade/spec.md",
    "nix/devshells/typescript.nix",
    "scripts/assemble-sdk-ts-did-artifacts.mjs",
    "scripts/build-sdk-ts-did-artifacts.sh",
    "scripts/ci/aggregate-fast-targets.mjs",
  ];
  for (const candidate of fixtures) {
    const result = plan([candidate]);
    assert.deepEqual(bindingTargets.filter((target) => result.targets[target].selected), bindingTargets, candidate);
  }
});

test("target-specific source selects only its dependent executable targets", () => {
  const browser = plan(["packages/identus-sdk/src/browser/loader.ts"]);
  assert.equal(browser.targets["package-types"].selected, true);
  assert.equal(browser.targets.browser.selected, true);
  assert.equal(browser.targets["react-web"].selected, true);
  assert.equal(browser.targets.nodejs.selected, false);
  assert.equal(browser.targets["react-native"].selected, false);

  const node = plan(["packages/identus-sdk/src/node/loader.ts"]);
  assert.equal(node.targets.nodejs.selected, true);
  assert.equal(node.targets["react-web"].selected, true);
  assert.equal(node.targets.browser.selected, false);
});

test("unknown or empty impact selects every target fail-closed", () => {
  for (const paths of [[], ["unclassified.bin"], ["../escape"]]) {
    const result = plan(paths);
    assert.equal(result.unknownImpact, true);
    assert.deepEqual(bindingTargets.filter((target) => result.targets[target].selected), bindingTargets);
    assert.equal(result.targets["react-native"].expectation, "inapplicable");
  }
});

test("known unrelated changes select no binding runtime", () => {
  const result = plan(["docs/factory/README.md", "crates/jose/src/lib.rs"]);
  assert.equal(result.unknownImpact, false);
  assert.deepEqual(bindingTargets.filter((target) => result.targets[target].selected), []);
});

test("closed exact-head receipts aggregate successfully", () => {
  const bindingPlan = plan(["crates/wasm-did/src/lib.rs"]);
  const result = aggregateFastTargets({ headSha, plan: bindingPlan, receipts: receiptsFor(bindingPlan) });
  assert.equal(result.verdict, "success");
  assert.equal(result.coreResult, "success");
  assert.deepEqual(result.failures, []);
  assert.equal(result.targetResults["react-native"], "inapplicable");
});

test("receipt construction is closed and exact-head", () => {
  const built = createTargetReceipt({
    headSha,
    target: "browser",
    applicability: "selected",
    status: "success",
    artifactSha256,
    durationSeconds: 2,
  });
  assert.equal(Object.isFrozen(built), true);
  assert.deepEqual(validateTargetReceipt(built, { headSha, target: "browser" }), []);
});

test("stale, incomplete and inconsistent plans fail closed", () => {
  const valid = plan(["packages/identus-sdk/src/browser/loader.ts"]);
  assert.match(validateBindingPlan({ ...valid, headSha: "d".repeat(40) }, headSha).join(";"), /Git identity/u);
  const { browser: _browser, ...incompleteTargets } = valid.targets;
  assert.match(validateBindingPlan({ ...valid, targets: incompleteTargets }, headSha).join(";"), /target set is incomplete/u);
  assert.match(validateBindingPlan({ ...valid, targets: { ...valid.targets, browser: null } }, headSha).join(";"), /target browser is invalid/u);
  const inconsistent = structuredClone(plan(["unclassified.bin"]));
  inconsistent.targets.browser.selected = false;
  assert.match(validateBindingPlan(inconsistent, headSha).join(";"), /unknown impact must select every binding target/u);
});

test("stale, malformed and unknown receipt fields fail closed", () => {
  const valid = receipt("browser");
  assert.deepEqual(validateTargetReceipt({ ...valid, headSha: "d".repeat(40) }, { headSha, target: "browser" }), ["receipt head SHA is stale or invalid"]);
  assert.deepEqual(validateTargetReceipt({ ...valid, privateInput: "did:secret" }, { headSha, target: "browser" }), ["receipt has unknown or missing fields"]);
  assert.match(validateTargetReceipt({ ...valid, artifactSha256: null }, { headSha, target: "browser" }).join(";"), /requires an artifact/u);
});

test("missing and every non-success selected terminal state block aggregation", () => {
  const bindingPlan = plan(["packages/identus-sdk/src/node/loader.ts"]);
  const missing = receiptsFor(bindingPlan);
  delete missing.nodejs;
  assert.match(aggregateFastTargets({ headSha, plan: bindingPlan, receipts: missing }).failures.join(";"), /nodejs: missing receipt/u);

  for (const status of ["failed", "cancelled", "skipped", "timed-out"]) {
    const mutated = receiptsFor(bindingPlan);
    mutated.nodejs = receipt("nodejs", status);
    const result = aggregateFastTargets({ headSha, plan: bindingPlan, receipts: mutated });
    assert.equal(result.verdict, "failure", status);
    assert.match(result.failures.join(";"), /nodejs: selected target did not succeed/u, status);
  }
});

test("unselected targets must emit bounded inapplicable receipts", () => {
  const bindingPlan = plan(["docs/factory/README.md"]);
  const mutated = receiptsFor(bindingPlan);
  mutated.browser = receipt("browser");
  const result = aggregateFastTargets({ headSha, plan: bindingPlan, receipts: mutated });
  assert.equal(result.verdict, "failure");
  assert.match(result.failures.join(";"), /browser: target must emit an inapplicable receipt/u);
});

test("target failure preserves the independently green core result", () => {
  const bindingPlan = plan(["packages/identus-sdk/src/node/loader.ts"]);
  const mutated = receiptsFor(bindingPlan);
  mutated.nodejs = receipt("nodejs", "failed");
  const result = aggregateFastTargets({ headSha, plan: bindingPlan, receipts: mutated });
  assert.equal(result.verdict, "failure");
  assert.equal(result.coreResult, "success");
  assert.equal(result.targetResults.nodejs, "failed");
});

test("a non-success core result cannot be relabelled by green targets", () => {
  const bindingPlan = plan(["crates/wasm-did/src/lib.rs"]);
  const mutated = receiptsFor(bindingPlan);
  mutated["core-rust"] = receipt("core-rust", "cancelled");
  const result = aggregateFastTargets({ headSha, plan: bindingPlan, receipts: mutated });
  assert.equal(result.verdict, "failure");
  assert.equal(result.coreResult, "cancelled");
  assert.match(result.failures.join(";"), /core-rust: selected target did not succeed/u);
});

test("CLI ingestion rejects duplicate JSON fields", () => {
  const temporary = mkdtempSync(path.join(os.tmpdir(), "binding-target-isolation-"));
  try {
    const planPath = path.join(temporary, "plan.json");
    const corePath = path.join(temporary, "core.json");
    writeFileSync(planPath, `{"schemaVersion":1,"schemaVersion":1}\n`);
    writeFileSync(corePath, `${JSON.stringify(receipt("core-rust"))}\n`);
    const result = spawnSync(process.execPath, [
      path.join(repositoryRoot, "scripts/ci/aggregate-fast-targets.mjs"),
      "--head", headSha,
      "--plan", planPath,
      "--core", corePath,
    ], { cwd: repositoryRoot, encoding: "utf8" });
    assert.notEqual(result.status, 0);
    assert.match(result.stderr, /duplicate field/u);
  } finally {
    rmSync(temporary, { recursive: true, force: true });
  }
});

test("receipt writer emits one closed private exact-head record", () => {
  const temporary = mkdtempSync(path.join(os.tmpdir(), "binding-target-receipt-"));
  try {
    const output = path.join(temporary, "browser.json");
    const result = spawnSync(process.execPath, [
      path.join(repositoryRoot, "scripts/ci/write-target-receipt.mjs"),
      "--head", headSha,
      "--target", "browser",
      "--applicability", "selected",
      "--status", "success",
      "--artifact-sha256", artifactSha256,
      "--duration-seconds", "4",
      "--output", output,
    ], { cwd: repositoryRoot, encoding: "utf8" });
    assert.equal(result.status, 0, result.stderr);
    assert.deepEqual(JSON.parse(readFileSync(output, "utf8")), receipt("browser", "success", "selected", {
      durationSeconds: 4,
    }));
  } finally {
    rmSync(temporary, { recursive: true, force: true });
  }
});
