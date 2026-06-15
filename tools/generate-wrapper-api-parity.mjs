#!/usr/bin/env node

import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const workspaceRoot = path.resolve(repoRoot, "..", "..");
const fixturePath = path.join(
  repoRoot,
  "fixtures/conformance/interop/wrapper-api-parity.json",
);

const coverage = {
  didPrism: [
    "fixtures/conformance/vector/did-prism/deterministic-master-key.json",
  ],
  didPrismVdr: [
    "fixtures/conformance/vector/did-prism/deterministic-master-key.json",
    "fixtures/conformance/vector/did-prism/prism-operation-vdr-lifecycle.json",
  ],
  didcomm: [
    "fixtures/conformance/transcript/didcomm/edge-mediation-and-pickup.json",
  ],
  didcommCredential: [
    "fixtures/conformance/transcript/didcomm/edge-mediation-and-pickup.json",
    "fixtures/conformance/transcript/didcomm/credential-presentation-revocation.json",
  ],
  connectionless: [
    "fixtures/conformance/transcript/didcomm/connectionless-oob-credential-proof.json",
    "fixtures/conformance/transcript/openid4vc/core-flows.json",
  ],
  credential: [
    "fixtures/conformance/vector/credential-verification/negative-cases.json",
  ],
  credentialOpenid: [
    "fixtures/conformance/vector/credential-verification/negative-cases.json",
    "fixtures/conformance/transcript/openid4vc/core-flows.json",
  ],
  pluginBacklog: ["T057"],
};

const languageConfigs = [
  {
    language: "typescript",
    package: "@hyperledger/identus-sdk",
    source_files: [
      "repos/sdk-ts/packages/lib/sdk/src/index.ts",
      "repos/sdk-ts/packages/shared/domain/src/index.ts",
      "repos/sdk-ts/packages/shared/domain/src/models/index.ts",
    ],
    exports: [
      tsModule("Apollo", "apollo", "identus-crypto", "cryptography", coverage.didPrism),
      tsModule("Castor", "castor", "identus-did", "did", coverage.didPrismVdr),
      tsModule("Mercury", "mercury", "identus-messaging", "messaging", coverage.didcommCredential),
      tsModule("Pluto", "pluto", "identus-wallet", "wallet", coverage.didcomm),
      {
        source_name: "edge-agent",
        source_kind: "module_export",
        source_path: "repos/sdk-ts/packages/lib/sdk/src/index.ts",
        owner_crate: "identus-agent",
        binding_surface: "agent",
        migration_disposition: "replace_with_rust_core_binding",
        fixture_coverage: coverage.connectionless,
        required_pattern: /export \* from ['"]\.\/edge-agent['"]/,
      },
      {
        source_name: "PluginManager",
        source_kind: "named_export",
        source_path: "repos/sdk-ts/packages/lib/sdk/src/index.ts",
        owner_crate: "identus-bindings",
        binding_surface: "extension",
        migration_disposition: "requires_plugin_compatibility_adr",
        fixture_coverage: coverage.pluginBacklog,
        required_pattern: /export \{ PluginManager \} from "\.\/plugins\/PluginManager";/,
      },
      {
        source_name: "Domain",
        source_kind: "namespace_export",
        source_path: "repos/sdk-ts/packages/lib/sdk/src/index.ts",
        owner_crate: "identus-bindings",
        binding_surface: "domain_dto",
        migration_disposition: "replace_with_generated_dto_binding",
        fixture_coverage: coverage.credential,
        required_pattern: /export \* as Domain from "@hyperledger\/identus-domain";/,
      },
      {
        source_name: "JWTCredential",
        source_kind: "named_export",
        source_path: "repos/sdk-ts/packages/lib/sdk/src/index.ts",
        owner_crate: "identus-credentials",
        binding_surface: "credential",
        migration_disposition: "replace_with_rust_core_binding",
        fixture_coverage: coverage.credential,
        required_pattern: /export \{ JWTCredential,/,
      },
      {
        source_name: "SDJWTCredential",
        source_kind: "named_export",
        source_path: "repos/sdk-ts/packages/lib/sdk/src/index.ts",
        owner_crate: "identus-credentials",
        binding_surface: "credential",
        migration_disposition: "replace_with_rust_core_binding",
        fixture_coverage: coverage.credential,
        required_pattern: /export \{ SDJWTCredential,/,
      },
    ],
  },
  {
    language: "swift",
    package: "EdgeAgentSDK",
    source_files: [
      "repos/sdk-swift/EdgeAgentSDK/Domain/Sources/BBs/Apollo.swift",
      "repos/sdk-swift/EdgeAgentSDK/Domain/Sources/BBs/Castor.swift",
      "repos/sdk-swift/EdgeAgentSDK/Domain/Sources/BBs/Mercury.swift",
      "repos/sdk-swift/EdgeAgentSDK/Domain/Sources/BBs/Pollux.swift",
      "repos/sdk-swift/EdgeAgentSDK/Domain/Sources/BBs/Pluto.swift",
      "repos/sdk-swift/EdgeAgentSDK/Domain/Sources/Models/Credentials/Credential.swift",
    ],
    exports: [
      swiftProtocol("Apollo", "identus-crypto", "cryptography", coverage.didPrism),
      swiftProtocol("Castor", "identus-did", "did", coverage.didPrismVdr),
      swiftProtocol("Mercury", "identus-messaging", "messaging", coverage.didcomm),
      swiftProtocol("Pollux", "identus-credentials", "credential", coverage.credential),
      swiftProtocol("Pluto", "identus-wallet", "wallet", coverage.didcomm),
      {
        source_name: "Credential",
        source_kind: "public_protocol",
        source_path: "repos/sdk-swift/EdgeAgentSDK/Domain/Sources/Models/Credentials/Credential.swift",
        owner_crate: "identus-credentials",
        binding_surface: "credential",
        migration_disposition: "replace_with_uniffi_binding",
        fixture_coverage: coverage.credential,
        required_pattern: /public protocol Credential\b/,
      },
    ],
  },
  {
    language: "kotlin",
    package: "org.hyperledger.identus.walletsdk",
    source_files: [
      "repos/sdk-kmp/sdk/src/commonMain/kotlin/org/hyperledger/identus/walletsdk/domain/buildingblocks/Apollo.kt",
      "repos/sdk-kmp/sdk/src/commonMain/kotlin/org/hyperledger/identus/walletsdk/domain/buildingblocks/Castor.kt",
      "repos/sdk-kmp/sdk/src/commonMain/kotlin/org/hyperledger/identus/walletsdk/domain/buildingblocks/Mercury.kt",
      "repos/sdk-kmp/sdk/src/commonMain/kotlin/org/hyperledger/identus/walletsdk/domain/buildingblocks/Pollux.kt",
      "repos/sdk-kmp/sdk/src/commonMain/kotlin/org/hyperledger/identus/walletsdk/domain/buildingblocks/Pluto.kt",
      "repos/sdk-kmp/sdk/src/commonMain/kotlin/org/hyperledger/identus/walletsdk/edgeagent/EdgeAgent.kt",
      "repos/sdk-kmp/sdk/src/commonMain/kotlin/org/hyperledger/identus/walletsdk/domain/models/DID.kt",
      "repos/sdk-kmp/sdk/src/commonMain/kotlin/org/hyperledger/identus/walletsdk/domain/models/Credential.kt",
      "repos/sdk-kmp/sdk/src/commonMain/kotlin/org/hyperledger/identus/walletsdk/domain/models/Message.kt",
    ],
    exports: [
      kotlinInterface("Apollo", "identus-crypto", "cryptography", coverage.didPrism),
      kotlinInterface("Castor", "identus-did", "did", coverage.didPrismVdr),
      kotlinInterface("Mercury", "identus-messaging", "messaging", coverage.didcomm),
      kotlinInterface("Pollux", "identus-credentials", "credential", coverage.credential),
      kotlinInterface("Pluto", "identus-wallet", "wallet", coverage.didcomm),
      {
        source_name: "EdgeAgent",
        source_kind: "class",
        source_path: "repos/sdk-kmp/sdk/src/commonMain/kotlin/org/hyperledger/identus/walletsdk/edgeagent/EdgeAgent.kt",
        owner_crate: "identus-agent",
        binding_surface: "agent",
        migration_disposition: "replace_with_uniffi_binding",
        fixture_coverage: coverage.connectionless,
        required_pattern: /class EdgeAgent\b/,
      },
      {
        source_name: "DID",
        source_kind: "data_class",
        source_path: "repos/sdk-kmp/sdk/src/commonMain/kotlin/org/hyperledger/identus/walletsdk/domain/models/DID.kt",
        owner_crate: "identus-did",
        binding_surface: "did",
        migration_disposition: "replace_with_uniffi_binding",
        fixture_coverage: coverage.didPrism,
        required_pattern: /data class DID\b/,
      },
      {
        source_name: "Credential",
        source_kind: "interface",
        source_path: "repos/sdk-kmp/sdk/src/commonMain/kotlin/org/hyperledger/identus/walletsdk/domain/models/Credential.kt",
        owner_crate: "identus-credentials",
        binding_surface: "credential",
        migration_disposition: "replace_with_uniffi_binding",
        fixture_coverage: coverage.credential,
        required_pattern: /interface Credential\b/,
      },
      {
        source_name: "Message",
        source_kind: "data_class",
        source_path: "repos/sdk-kmp/sdk/src/commonMain/kotlin/org/hyperledger/identus/walletsdk/domain/models/Message.kt",
        owner_crate: "identus-messaging",
        binding_surface: "messaging",
        migration_disposition: "replace_with_uniffi_binding",
        fixture_coverage: coverage.didcommCredential,
        required_pattern: /data class Message\b/,
      },
    ],
  },
];

function tsModule(name, module, ownerCrate, surface, fixtureCoverage) {
  return {
    source_name: name,
    source_kind: "module_export",
    source_path: "repos/sdk-ts/packages/lib/sdk/src/index.ts",
    owner_crate: ownerCrate,
    binding_surface: surface,
    migration_disposition: "replace_with_rust_core_binding",
    fixture_coverage: fixtureCoverage,
    required_pattern: new RegExp(`export \\* from ["']\\.\\/${module}["']`),
  };
}

function swiftProtocol(name, ownerCrate, surface, fixtureCoverage) {
  return {
    source_name: name,
    source_kind: "public_protocol",
    source_path: `repos/sdk-swift/EdgeAgentSDK/Domain/Sources/BBs/${name}.swift`,
    owner_crate: ownerCrate,
    binding_surface: surface,
    migration_disposition: "replace_with_uniffi_binding",
    fixture_coverage: fixtureCoverage,
    required_pattern: new RegExp(`public protocol ${name}\\b`),
  };
}

function kotlinInterface(name, ownerCrate, surface, fixtureCoverage) {
  return {
    source_name: name,
    source_kind: "interface",
    source_path: `repos/sdk-kmp/sdk/src/commonMain/kotlin/org/hyperledger/identus/walletsdk/domain/buildingblocks/${name}.kt`,
    owner_crate: ownerCrate,
    binding_surface: surface,
    migration_disposition: "replace_with_uniffi_binding",
    fixture_coverage: fixtureCoverage,
    required_pattern: new RegExp(`interface ${name}\\b`),
  };
}

function assertSourceExport(entry) {
  const sourcePath = path.join(workspaceRoot, entry.source_path);
  if (!fs.existsSync(sourcePath)) {
    throw new Error(`source file does not exist: ${entry.source_path}`);
  }
  const sourceText = fs.readFileSync(sourcePath, "utf8");
  if (!entry.required_pattern.test(sourceText)) {
    throw new Error(
      `${entry.source_path} does not contain expected export ${entry.source_name}`,
    );
  }
}

function cleanEntry(entry) {
  const {
    source_name,
    source_kind,
    source_path,
    owner_crate,
    binding_surface,
    migration_disposition,
    fixture_coverage,
  } = entry;
  return {
    source_name,
    source_kind,
    source_path,
    owner_crate,
    binding_surface,
    migration_disposition,
    fixture_coverage,
  };
}

function generate() {
  for (const language of languageConfigs) {
    for (const sourceFile of language.source_files) {
      const sourcePath = path.join(workspaceRoot, sourceFile);
      if (!fs.existsSync(sourcePath)) {
        throw new Error(`source file does not exist: ${sourceFile}`);
      }
    }
    for (const entry of language.exports) {
      assertSourceExport(entry);
    }
  }

  const languages = languageConfigs.map((language) => ({
    language: language.language,
    package: language.package,
    source_files: language.source_files,
    exports: language.exports.map(cleanEntry),
  }));

  const inventory = {
    id: "wrapper-api-parity-inventory",
    case_id: "wrapper-api-parity-inventory",
    schema_version: "interop-0.1",
    specification_ids: [
      "digital-credentials-api",
      "did-prism",
      "did-peer-1",
      "didcomm-v2",
      "didcomm-oob-2",
      "didcomm-issue-credential-3",
      "didcomm-present-proof-3",
      "oid4vci-1",
      "oid4vp-1",
      "vc-data-model",
      "jwt-vc",
      "sd-jwt-vc",
      "anoncreds-v1",
      "presentation-exchange",
      "bitstring-status-list",
    ],
    source: [
      "repos/sdk-ts/packages/lib/sdk/src/index.ts",
      "repos/sdk-ts/packages/shared/domain/src/index.ts",
      "repos/sdk-ts/packages/shared/domain/src/models/index.ts",
      "repos/sdk-swift/EdgeAgentSDK/Domain/Sources/BBs/*.swift",
      "repos/sdk-swift/EdgeAgentSDK/Domain/Sources/Models/Credentials/*.swift",
      "repos/sdk-kmp/sdk/src/commonMain/kotlin/org/hyperledger/identus/walletsdk/domain/buildingblocks/*.kt",
      "repos/sdk-kmp/sdk/src/commonMain/kotlin/org/hyperledger/identus/walletsdk/edgeagent/EdgeAgent.kt",
      "repos/sdk-kmp/sdk/src/commonMain/kotlin/org/hyperledger/identus/walletsdk/domain/models/*.kt",
    ],
    owner_crate: "identus-bindings",
    reference_implementation: "sdk-ts, sdk-swift, sdk-kmp public exports",
    description:
      "Machine-readable wrapper API parity inventory generated from local SDK public export sources.",
    generation: {
      date: "2026-06-13",
      method: "repeatable local source export scan",
      commands: [
        "node tools/generate-wrapper-api-parity.mjs --check",
        "node tools/generate-wrapper-api-parity.mjs --write",
      ],
    },
    languages,
    expected: {
      languages: ["typescript", "swift", "kotlin"],
      required_fields_per_entry: [
        "source_name",
        "source_kind",
        "source_path",
        "owner_crate",
        "binding_surface",
        "migration_disposition",
        "fixture_coverage",
      ],
      minimum_entries: 20,
      next_backlog: ["T057", "T083"],
    },
    redaction_policy:
      "Inventory contains public source paths and export names only. It must not contain secrets, test credentials, wallet seeds, access tokens, or private keys.",
  };

  return `${JSON.stringify(inventory, null, 2)}\n`;
}

function main() {
  const args = new Set(process.argv.slice(2));
  const generated = generate();

  if (args.has("--write")) {
    fs.writeFileSync(fixturePath, generated);
    return;
  }

  if (args.has("--check")) {
    const actual = fs.readFileSync(fixturePath, "utf8");
    if (actual !== generated) {
      console.error(
        "wrapper API parity fixture is stale; run node tools/generate-wrapper-api-parity.mjs --write",
      );
      process.exitCode = 1;
    }
    return;
  }

  process.stdout.write(generated);
}

main();
