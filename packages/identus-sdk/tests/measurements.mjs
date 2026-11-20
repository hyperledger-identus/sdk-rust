#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0

import { execFile } from "node:child_process";
import { readFile, writeFile } from "node:fs/promises";
import path from "node:path";
import { performance } from "node:perf_hooks";
import { promisify } from "node:util";
import { fileURLToPath, pathToFileURL } from "node:url";
import { gzipSync } from "node:zlib";

const runFile = promisify(execFile);
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const output = path.resolve(
  process.env.SDK_TS_DID_MEASUREMENTS
    ?? path.join(root, "../../target/sdk-ts-did-measurements.json"),
);
const baselineRoot = requireAbsoluteDirectory("SDK_TS_DID_BASELINE_PACKAGE_ROOT");
const baselineRevision = requireRevision("SDK_TS_DID_BASELINE_REVISION");
const candidateRevision = requireRevision("SDK_TS_DID_CANDIDATE_REVISION");
const warmup = 21;
const samples = 101;
const coldSamples = 11;
const execOptions = Object.freeze({ encoding: "utf8", maxBuffer: 64 * 1_024 });

function requireAbsoluteDirectory(name) {
  const value = process.env[name];
  if (typeof value !== "string" || !path.isAbsolute(value)) {
    throw new Error(`${name} must be an absolute package root`);
  }
  return path.resolve(value);
}

function requireRevision(name) {
  const value = process.env[name];
  if (typeof value !== "string" || !/^[0-9a-f]{40}$/u.test(value)) {
    throw new Error(`${name} must be an exact lowercase Git SHA`);
  }
  return value;
}

function percentile(values, fraction) {
  const sorted = [...values].sort((left, right) => left - right);
  return sorted[Math.min(sorted.length - 1, Math.floor(sorted.length * fraction))];
}

function summary(durations, measuredSamples, measuredWarmup) {
  return Object.freeze({
    samples: measuredSamples,
    warmup: measuredWarmup,
    aggregation: "nearest-rank over ascending milliseconds",
    p50Milliseconds: percentile(durations, 0.5),
    p95Milliseconds: percentile(durations, 0.95),
  });
}

function measure(operation) {
  for (let index = 0; index < warmup; index += 1) operation();
  const durations = [];
  for (let index = 0; index < samples; index += 1) {
    const start = performance.now();
    operation();
    durations.push(performance.now() - start);
  }
  return summary(durations, samples, warmup);
}

async function measureAsync(operation) {
  for (let index = 0; index < warmup; index += 1) await operation();
  const durations = [];
  for (let index = 0; index < samples; index += 1) {
    const start = performance.now();
    await operation();
    durations.push(performance.now() - start);
  }
  return summary(durations, samples, warmup);
}

async function artifactMeasurement(packageRoot) {
  const manifest = JSON.parse(
    await readFile(path.join(packageRoot, "artifacts/manifest.json"), "utf8"),
  );
  const targets = {};
  for (const target of ["web", "nodejs"]) {
    const artifacts = [...manifest.targets[target].artifacts]
      .sort((left, right) => left.path.localeCompare(right.path));
    let uncompressedBytes = 0;
    let gzipBytes = 0;
    for (const artifact of artifacts) {
      const bytes = await readFile(path.join(packageRoot, "artifacts", artifact.path));
      if (bytes.byteLength !== artifact.bytes) {
        throw new Error(`${target} artifact byte mismatch: ${artifact.path}`);
      }
      uncompressedBytes += bytes.byteLength;
      gzipBytes += gzipSync(bytes, { level: 9, mtime: 0 }).byteLength;
    }
    targets[target] = Object.freeze({
      targetSha256: manifest.targets[target].targetSha256,
      paths: Object.freeze(artifacts.map((artifact) => artifact.path)),
      uncompressedBytes,
      gzipBytes,
    });
  }
  return Object.freeze({
    assemblySha256: manifest.assemblySha256,
    bindingApiVersion: manifest.bindingApiVersion,
    generator: Object.freeze({ ...manifest.generator }),
    targets: Object.freeze(targets),
  });
}

async function verifyPackageRevision(packageRoot, expectedRevision, label) {
  const repositoryRoot = path.resolve(packageRoot, "../..");
  const { stdout: head } = await runFile(
    "git",
    ["-C", repositoryRoot, "rev-parse", "HEAD"],
    execOptions,
  );
  if (head.trim() !== expectedRevision) {
    throw new Error(`${label} package root does not match its declared revision`);
  }
  const { stdout: status } = await runFile(
    "git",
    ["-C", repositoryRoot, "status", "--porcelain=v1", "--untracked-files=no"],
    execOptions,
  );
  if (status !== "") {
    throw new Error(`${label} package root has tracked modifications`);
  }
}

function artifactDelta(baseline, candidate) {
  const targets = {};
  for (const target of ["web", "nodejs"]) {
    if (JSON.stringify(baseline.targets[target].paths)
      !== JSON.stringify(candidate.targets[target].paths)) {
      throw new Error(`${target} baseline and candidate artifact inventories differ`);
    }
    targets[target] = Object.freeze({
      uncompressedBytes: candidate.targets[target].uncompressedBytes - baseline.targets[target].uncompressedBytes,
      gzipBytes: candidate.targets[target].gzipBytes - baseline.targets[target].gzipBytes,
    });
  }
  return Object.freeze(targets);
}

const coldScript = `
  import { performance } from "node:perf_hooks";
  const start = performance.now();
  const module = await import(process.argv[1]);
  await module.createDidProvider();
  process.stdout.write(JSON.stringify({ milliseconds: performance.now() - start }));
`;

async function measureColdInitialization(moduleUrl) {
  const durations = [];
  for (let index = 0; index < coldSamples; index += 1) {
    const { stdout } = await runFile(
      process.execPath,
      ["--input-type=module", "--eval", coldScript, moduleUrl],
      execOptions,
    );
    const value = JSON.parse(stdout);
    if (typeof value.milliseconds !== "number" || !Number.isFinite(value.milliseconds) || value.milliseconds < 0) {
      throw new Error("cold initialization child returned an invalid duration");
    }
    durations.push(value.milliseconds);
  }
  return summary(durations, coldSamples, 0);
}

async function toolVersion(command, args = ["--version"]) {
  const { stdout } = await runFile(command, args, execOptions);
  return stdout.trim();
}

async function measureInitialization(packageRoot) {
  const moduleUrl = pathToFileURL(path.join(packageRoot, "dist/node.js")).href;
  const coldImportAndProvider = await measureColdInitialization(moduleUrl);
  const module = await import(moduleUrl);
  await module.createDidProvider();
  const warmImportAndProvider = await measureAsync(async () => {
    const cached = await import(moduleUrl);
    await cached.createDidProvider();
  });
  const providerConstruction = await measureAsync(() => module.createDidProvider());
  return Object.freeze({ module, coldImportAndProvider, warmImportAndProvider, providerConstruction });
}

await verifyPackageRevision(baselineRoot, baselineRevision, "baseline");
await verifyPackageRevision(root, candidateRevision, "candidate");
const baselineArtifact = await artifactMeasurement(baselineRoot);
const candidateArtifact = await artifactMeasurement(root);
const artifact = Object.freeze({
  definition: "sum of identical manifest-ordered regular files; gzip is per-file RFC 1952 level 9 with mtime 0",
  baseline: baselineArtifact,
  candidate: candidateArtifact,
  candidateMinusBaseline: artifactDelta(baselineArtifact, candidateArtifact),
});
const baselineInitialization = await measureInitialization(baselineRoot);
const candidateInitialization = await measureInitialization(root);
const { createDidProvider, createDidQueryProvider } = candidateInitialization.module;
const provider = await createDidProvider();
const ordinary = measure(() => provider.parseDid("did:example:measurement"));
const nearLimitValue = `did:example:${"a".repeat(131_000)}`;
const nearLimit = measure(() => provider.parseDid(nearLimitValue));
const ordinaryDocumentValue = '{"id":"did:example:measurement","profile":{"enabled":true}}';
const ordinaryDocument = measure(() => provider.parseDidDocument(ordinaryDocumentValue));
const nearLimitDocumentValue = JSON.stringify({
  id: "did:example:measurement-near-limit",
  ...Object.fromEntries(Array.from({ length: 60 }, (_, index) => [
    `extension-${index.toString().padStart(2, "0")}`,
    "a".repeat(4_000),
  ])),
});
const nearLimitDocument = measure(() => provider.parseDidDocument(nearLimitDocumentValue));
const ordinaryDocumentOutputBytes = Buffer.byteLength(provider.parseDidDocument(ordinaryDocumentValue).serialized);
const nearLimitDocumentOutputBytes = Buffer.byteLength(provider.parseDidDocument(nearLimitDocumentValue).serialized);
const ordinaryPeerInput = Object.freeze({ hello: "world" });
const nearLimitPeerInput = Object.freeze({ claim: "a".repeat(16_372) });
const ordinaryPeer = measure(() => provider.createPeerDidNumalgo4(ordinaryPeerInput));
const nearLimitPeer = measure(() => provider.createPeerDidNumalgo4(nearLimitPeerInput));
const ordinaryPeerOutputBytes = Buffer.byteLength(
  JSON.stringify(provider.createPeerDidNumalgo4(ordinaryPeerInput)),
);
const nearLimitPeerOutputBytes = Buffer.byteLength(
  JSON.stringify(provider.createPeerDidNumalgo4(nearLimitPeerInput)),
);
const prismPublicKey = "AuRZmOGBFxE4NblytDeE-2tkcLciyy3BccX7oAJlvIDu";
const ordinaryPrismInput = Object.freeze({
  keys: Object.freeze([Object.freeze({
    id: "master0",
    usage: "master",
    curve: "secp256k1",
    publicKey: prismPublicKey,
  })]),
});
const maxProfilePrismInput = Object.freeze({
  keys: Object.freeze(Array.from({ length: 50 }, (_, index) => Object.freeze({
    id: `master-${index}`,
    usage: "master",
    curve: "secp256k1",
    publicKey: prismPublicKey,
  }))),
});
const ordinaryPrismCreate = measure(() => provider.createPrismDid(ordinaryPrismInput));
const maxProfilePrismCreate = measure(() => provider.createPrismDid(maxProfilePrismInput));
const ordinaryPrismCreateOutputBytes = Buffer.byteLength(
  JSON.stringify(provider.createPrismDid(ordinaryPrismInput)),
);
const maxProfilePrismCreateOutputBytes = Buffer.byteLength(
  JSON.stringify(provider.createPrismDid(maxProfilePrismInput)),
);
const prismCanonicalDid = "did:prism:44fb82ad3544a8c8acb763c01a6fc7d92d81c38104096319345a4618d1a4275c";
const prismSignedCreate = "CgdtYXN0ZXIwEkgwRgIhALjle4EjdeqePDrnlFnIaVoUlBm_aMLp4YoYXd3OSwivAiEAql-m3H3ufA0Lkc4FwgRSfoS7qe-H4nhwhRLQyRrCQDwaQQo_Cj0SOwoHbWFzdGVyMBABSi4KCXNlY3AyNTZrMRIhAuRZmOGBFxE4NblytDeE-2tkcLciyy3BccX7oAJlvIDu";
const prismQueryProvider = await createDidQueryProvider({
  history: { history: async () => [prismSignedCreate] },
});
const ordinaryPrismHistory = await measureAsync(() =>
  prismQueryProvider.resolvePrismDid(prismCanonicalDid));
const ordinaryPrismHistoryOutputBytes = Buffer.byteLength(
  JSON.stringify(await prismQueryProvider.resolvePrismDid(prismCanonicalDid)),
);
const resolutionDid = "did:example:measurement";
const ordinaryResolutionValue = JSON.stringify({
  didResolutionMetadata: { contentType: "application/did+json" },
  didDocument: { id: resolutionDid },
  didDocumentMetadata: { versionId: "ordinary" },
});
const nearLimitResolutionValue = JSON.stringify({
  didResolutionMetadata: {
    contentType: "application/did+json",
    measurementPadding: "a".repeat((64 * 1_024) - 64),
  },
  didDocument: { id: resolutionDid },
  didDocumentMetadata: { versionId: "near-limit" },
});
const ordinaryQueryProvider = await createDidQueryProvider({
  resolver: { resolve: async () => ordinaryResolutionValue },
});
const nearLimitQueryProvider = await createDidQueryProvider({
  resolver: { resolve: async () => nearLimitResolutionValue },
});
const ordinaryResolution = await measureAsync(() =>
  ordinaryQueryProvider.resolveDid(resolutionDid));
const nearLimitResolution = await measureAsync(() =>
  nearLimitQueryProvider.resolveDid(resolutionDid));
const ordinaryResolutionOutputBytes = Buffer.byteLength(
  (await ordinaryQueryProvider.resolveDid(resolutionDid)).serialized,
);
const nearLimitResolutionOutputBytes = Buffer.byteLength(
  (await nearLimitQueryProvider.resolveDid(resolutionDid)).serialized,
);

function initializationReport(measurement) {
  return Object.freeze({
    coldImportAndProvider: measurement.coldImportAndProvider,
    warmImportAndProvider: measurement.warmImportAndProvider,
    providerConstruction: measurement.providerConstruction,
  });
}

const report = Object.freeze({
  schemaVersion: 2,
  status: "measurement-only",
  measuredTargets: Object.freeze(["package:identus-typescript-sdk", "runtime:nodejs"]),
  revisions: Object.freeze({ baseline: baselineRevision, candidate: candidateRevision }),
  environment: Object.freeze({
    node: process.version,
    platform: process.platform,
    architecture: process.arch,
    cpu: process.env.SDK_TS_DID_CPU_DESCRIPTION ?? "unrecorded",
  }),
  toolchain: Object.freeze({
    rustc: await toolVersion("rustc", ["--version", "--verbose"]),
    wasmPack: await toolVersion("wasm-pack"),
    wasmBindgen: await toolVersion("wasm-bindgen"),
  }),
  artifact,
  initialization: Object.freeze({
    definition: "cold includes dynamic import plus first provider in an isolated Node process; warm repeats cached dynamic import plus provider in one process; provider construction follows initialized binding",
    baseline: initializationReport(baselineInitialization),
    candidate: initializationReport(candidateInitialization),
  }),
  operations: Object.freeze({
    parseDid: Object.freeze({ ordinary, nearLimit }),
    document: Object.freeze({
      ordinary: Object.freeze({
        ...ordinaryDocument,
        inputBytes: Buffer.byteLength(ordinaryDocumentValue),
        outputBytes: ordinaryDocumentOutputBytes,
      }),
      nearLimit: Object.freeze({
        ...nearLimitDocument,
        inputBytes: Buffer.byteLength(nearLimitDocumentValue),
        outputBytes: nearLimitDocumentOutputBytes,
      }),
    }),
    peer: Object.freeze({
      profile: "numalgo4",
      ordinary: Object.freeze({
        ...ordinaryPeer,
        inputBytes: Buffer.byteLength(JSON.stringify(ordinaryPeerInput)),
        outputBytes: ordinaryPeerOutputBytes,
      }),
      nearLimit: Object.freeze({
        ...nearLimitPeer,
        inputBytes: Buffer.byteLength(JSON.stringify(nearLimitPeerInput)),
        outputBytes: nearLimitPeerOutputBytes,
      }),
    }),
    prism: Object.freeze({
      create: Object.freeze({
        ordinary: Object.freeze({
          ...ordinaryPrismCreate,
          inputBytes: Buffer.byteLength(JSON.stringify(ordinaryPrismInput)),
          outputBytes: ordinaryPrismCreateOutputBytes,
        }),
        maxProfile: Object.freeze({
          ...maxProfilePrismCreate,
          inputBytes: Buffer.byteLength(JSON.stringify(maxProfilePrismInput)),
          outputBytes: maxProfilePrismCreateOutputBytes,
        }),
      }),
      signedHistory: Object.freeze({
        ordinary: Object.freeze({
          ...ordinaryPrismHistory,
          inputBytes: Buffer.byteLength(prismSignedCreate),
          outputBytes: ordinaryPrismHistoryOutputBytes,
        }),
      }),
    }),
    resolution: Object.freeze({
      ordinary: Object.freeze({
        ...ordinaryResolution,
        inputBytes: Buffer.byteLength(ordinaryResolutionValue),
        outputBytes: ordinaryResolutionOutputBytes,
      }),
      nearLimit: Object.freeze({
        ...nearLimitResolution,
        inputBytes: Buffer.byteLength(nearLimitResolutionValue),
        outputBytes: nearLimitResolutionOutputBytes,
      }),
    }),
  }),
  rssBytes: process.memoryUsage().rss,
  limitations: Object.freeze([
    "Directional local Node measurement only; browser and React receipts are functional evidence, not performance measurements.",
    "No accepted support, regression budget, external adoption, publication, stable API, or release claim.",
    "Cold timing starts immediately before module import and excludes Node process launch.",
  ]),
});

await writeFile(output, `${JSON.stringify(report, null, 2)}\n`, { mode: 0o600 });
process.stdout.write(`${JSON.stringify(report)}\n`);
