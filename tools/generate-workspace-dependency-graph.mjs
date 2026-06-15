#!/usr/bin/env node

import { execFileSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const fixturePath = path.join(
  repoRoot,
  "fixtures/conformance/static-model/workspace-dependency-graph.json",
);

const layerByCrate = new Map([
  ["identus-core", "foundation"],
  ["identus-crypto", "domain-primitives"],
  ["identus-did", "domain-primitives"],
  ["identus-trust", "domain-primitives"],
  ["identus-credentials", "credential-semantics"],
  ["identus-presentations", "credential-semantics"],
  ["identus-messaging", "protocol-semantics"],
  ["identus-openid4vc", "protocol-semantics"],
  ["identus-wallet", "orchestration"],
  ["identus-agent", "orchestration"],
  ["identus-adapters", "outer-boundary"],
  ["identus-bindings", "outer-boundary"],
  ["identus-conformance", "verification"],
]);

const layerRules = [
  {
    layer: "foundation",
    crates: ["identus-core"],
    rule: "No workspace dependencies and no dependency on product, protocol, adapter, binding, or conformance crates.",
  },
  {
    layer: "domain-primitives",
    crates: ["identus-crypto", "identus-did", "identus-trust"],
    rule: "Own type-safe SSI primitives and depend only inward on foundation or other domain primitive crates.",
  },
  {
    layer: "credential-semantics",
    crates: ["identus-credentials", "identus-presentations"],
    rule: "Own credential and presentation semantics over identity, crypto, and trust primitives.",
  },
  {
    layer: "protocol-semantics",
    crates: ["identus-messaging", "identus-openid4vc"],
    rule: "Own DIDComm and OpenID4VC state boundaries without infrastructure dependencies.",
  },
  {
    layer: "orchestration",
    crates: ["identus-wallet", "identus-agent"],
    rule: "Compose domain and protocol capabilities without becoming an infrastructure adapter.",
  },
  {
    layer: "outer-boundary",
    crates: ["identus-adapters", "identus-bindings"],
    rule: "Expose infrastructure and language boundaries over stable core semantics.",
  },
  {
    layer: "verification",
    crates: ["identus-conformance"],
    rule: "Validate specifications, fixtures, docs, and public contract drift; production crates must not depend on it.",
  },
];

const allowedProductionTargetLayers = {
  foundation: [],
  "domain-primitives": ["foundation", "domain-primitives"],
  "credential-semantics": [
    "foundation",
    "domain-primitives",
    "credential-semantics",
  ],
  "protocol-semantics": [
    "foundation",
    "domain-primitives",
    "credential-semantics",
    "protocol-semantics",
  ],
  orchestration: [
    "foundation",
    "domain-primitives",
    "credential-semantics",
    "protocol-semantics",
    "orchestration",
  ],
  "outer-boundary": [
    "foundation",
    "domain-primitives",
    "credential-semantics",
    "protocol-semantics",
    "orchestration",
  ],
  verification: ["foundation"],
};

function workspaceMetadata() {
  return JSON.parse(
    execFileSync("cargo", ["metadata", "--format-version", "1", "--no-deps"], {
      cwd: repoRoot,
      encoding: "utf8",
    }),
  );
}

function workspaceCrateNames(metadata) {
  return new Set(metadata.packages.map((pkg) => pkg.name));
}

function packageDependencies(pkg, workspaceNames, kind) {
  return pkg.dependencies
    .filter((dep) => dep.source === null)
    .filter((dep) => workspaceNames.has(dep.name))
    .filter((dep) => (kind === "production" ? dep.kind === null : dep.kind === "dev"))
    .map((dep) => dep.name)
    .sort();
}

function edge(source, target) {
  return `${source}->${target}`;
}

function collectProductionPolicyViolations(crates) {
  const crateByName = new Map(crates.map((crate) => [crate.crate, crate]));
  return crates.flatMap((crate) => {
    const allowedTargets = allowedProductionTargetLayers[crate.layer];
    if (!allowedTargets) {
      return [
        {
          edge: `${crate.crate}-><unknown>`,
          source_layer: crate.layer,
          target_layer: "<unknown>",
          reason: "source crate has no allowed target layer policy",
        },
      ];
    }

    return crate.production_dependencies.flatMap((targetName) => {
      const target = crateByName.get(targetName);
      if (!target) {
        return [
          {
            edge: edge(crate.crate, targetName),
            source_layer: crate.layer,
            target_layer: "<missing>",
            reason: "target crate is not part of the workspace graph",
          },
        ];
      }

      if (allowedTargets.includes(target.layer)) {
        return [];
      }

      return [
        {
          edge: edge(crate.crate, targetName),
          source_layer: crate.layer,
          target_layer: target.layer,
          reason: "production dependency points outside the allowed hexagonal layer boundary",
        },
      ];
    });
  });
}

function collectProductionCycleViolations(crates) {
  const graph = new Map(
    crates.map((crate) => [crate.crate, crate.production_dependencies]),
  );
  const visiting = new Set();
  const visited = new Set();
  const stack = [];
  const cycles = new Set();

  function visit(crateName) {
    if (visiting.has(crateName)) {
      const cycleStart = stack.indexOf(crateName);
      if (cycleStart >= 0) {
        cycles.add([...stack.slice(cycleStart), crateName].join("->"));
      }
      return;
    }
    if (visited.has(crateName)) {
      return;
    }

    visiting.add(crateName);
    stack.push(crateName);
    for (const target of graph.get(crateName) ?? []) {
      visit(target);
    }
    stack.pop();
    visiting.delete(crateName);
    visited.add(crateName);
  }

  for (const crate of crates) {
    visit(crate.crate);
  }

  return [...cycles].sort();
}

function buildFixture(metadata) {
  const workspaceNames = workspaceCrateNames(metadata);
  const crates = metadata.packages
    .map((pkg) => {
      const layer = layerByCrate.get(pkg.name);
      if (!layer) {
        throw new Error(`missing architecture layer for ${pkg.name}`);
      }

      return {
        crate: pkg.name,
        layer,
        manifest_path: path.relative(repoRoot, pkg.manifest_path),
        production_dependencies: packageDependencies(pkg, workspaceNames, "production"),
        dev_dependencies: packageDependencies(pkg, workspaceNames, "dev"),
      };
    })
    .sort((left, right) => left.crate.localeCompare(right.crate));

  const productionEdges = crates.flatMap((crate) =>
    crate.production_dependencies.map((target) => edge(crate.crate, target)),
  );
  const devEdges = crates.flatMap((crate) =>
    crate.dev_dependencies.map((target) => edge(crate.crate, target)),
  );
  const productionPolicyViolations = collectProductionPolicyViolations(crates);
  const productionCycleViolations = collectProductionCycleViolations(crates);

  return {
    schema_version: "2026-06-15.workspace-dependency-graph.v1",
    generated_by: "tools/generate-workspace-dependency-graph.mjs",
    source_command: "cargo metadata --format-version 1 --no-deps",
    architecture_style: "hexagonal",
    arrow_semantics: "depends_on",
    layer_rules: layerRules,
    production_dependency_policy: {
      direction: "outer crates may depend inward; domain and protocol crates must not depend outward",
      allowed_target_layers_by_source_layer: allowedProductionTargetLayers,
      expected_policy_violations: 0,
      expected_cycle_violations: 0,
    },
    constraints: {
      core_crate: "identus-core",
      core_has_no_workspace_dependencies: true,
      domain_and_protocol_crates_do_not_depend_on: [
        "identus-adapters",
        "identus-bindings",
        "identus-conformance",
      ],
      outer_boundary_crates: ["identus-adapters", "identus-bindings"],
      verification_crate: "identus-conformance",
      production_crates_must_not_depend_on_verification: true,
    },
    crates,
    production_edges: productionEdges.sort(),
    dev_edges: devEdges.sort(),
    production_policy_violations: productionPolicyViolations,
    production_cycle_violations: productionCycleViolations,
  };
}

function stableJson(value) {
  return `${JSON.stringify(value, null, 2)}\n`;
}

function main() {
  const args = new Set(process.argv.slice(2));
  const fixture = buildFixture(workspaceMetadata());
  const next = stableJson(fixture);

  if (args.has("--write")) {
    fs.writeFileSync(fixturePath, next);
    return;
  }

  if (args.has("--check")) {
    const current = fs.existsSync(fixturePath)
      ? fs.readFileSync(fixturePath, "utf8")
      : "";
    if (current !== next) {
      throw new Error(
        `${path.relative(repoRoot, fixturePath)} is out of date; run node tools/generate-workspace-dependency-graph.mjs --write`,
      );
    }
    return;
  }

  process.stdout.write(next);
}

main();
