import { readFile, readdir } from "node:fs/promises";
import path from "node:path";
import { performance } from "node:perf_hooks";
import { gzipSync } from "node:zlib";

import { createHdDerivationProvider, createKeyOperationsProvider } from "../dist/node.js";

const packageRoot = path.resolve(new URL("..", import.meta.url).pathname);

function percentile(values, fraction) {
  const sorted = [...values].sort((left, right) => left - right);
  return sorted[Math.min(sorted.length - 1, Math.floor(sorted.length * fraction))];
}

async function artifactSizes(target) {
  const root = path.join(packageRoot, "artifacts/crypto", target);
  const files = await readdir(root);
  let bytes = 0;
  let gzipBytes = 0;
  for (const file of files.sort()) {
    const value = await readFile(path.join(root, file));
    bytes += value.length;
    gzipBytes += gzipSync(value, { level: 9 }).length;
  }
  return Object.freeze({ bytes, gzipBytes });
}

const provider = await createKeyOperationsProvider();
const hdProvider = await createHdDerivationProvider();
const ed = provider.generateEd25519();
const x = provider.generateX25519();
const message = new Uint8Array(1024);
const peer = provider.generateX25519();
const peerPublic = peer.publicKey().bytes;
const samples = [];
const hdSamples = [];
for (let index = 0; index < 122; index += 1) {
  const started = performance.now();
  const signature = ed.sign(message);
  if (!provider.verifyEd25519(ed.publicKey().bytes, message, signature)) {
    throw new Error("measurement verification failed");
  }
  x.deriveSharedSecret(peerPublic);
  const durationMs = performance.now() - started;
  if (index >= 21) samples.push(durationMs);

  const hdStarted = performance.now();
  const secp = hdProvider.createRoot(
    "secp256k1-bip32-hardened-v1",
    Uint8Array.from({ length: 32 }, (_, offset) => offset),
  );
  const secpChild = secp.derive("m/0'/1'/2'");
  secpChild.publicKey();
  secpChild.dispose();
  secp.dispose();
  const slip = hdProvider.createRoot(
    "ed25519-slip10-hardened-v1",
    Uint8Array.from({ length: 32 }, (_, offset) => offset),
  );
  const slipChild = slip.derive("m/0'/1'/2'");
  slipChild.publicKey();
  slipChild.dispose();
  slip.dispose();
  const cardano = hdProvider.createRoot("ed25519-cardano-v2-v1", new Uint8Array(64).fill(7));
  const cardanoChild = cardano.derive("m/0/1/2");
  cardanoChild.publicKey();
  const cardanoPublic = cardano.publicDerivation();
  const cardanoPublicChild = cardanoPublic.derive("m/0/1/2");
  cardanoPublicChild.publicKey();
  cardanoPublicChild.dispose();
  cardanoPublic.dispose();
  cardanoChild.dispose();
  cardano.dispose();
  const hdDurationMs = performance.now() - hdStarted;
  if (index >= 21) hdSamples.push(hdDurationMs);
}
ed.dispose();
x.dispose();
peer.dispose();

process.stdout.write(`${JSON.stringify({
  schemaVersion: 1,
  artifact: {
    web: await artifactSizes("web"),
    nodejs: await artifactSizes("nodejs"),
  },
  operation: {
    samples: samples.length,
    messageBytes: message.length,
    p50Milliseconds: percentile(samples, 0.5),
    p95Milliseconds: percentile(samples, 0.95),
  },
  hdDerivation: {
    samples: hdSamples.length,
    profiles: 3,
    privatePathDepth: 3,
    publicPathDepth: 3,
    p50Milliseconds: percentile(hdSamples, 0.5),
    p95Milliseconds: percentile(hdSamples, 0.95),
  },
})}\n`);
