import { readFile, readdir } from "node:fs/promises";
import path from "node:path";
import { performance } from "node:perf_hooks";
import { gzipSync } from "node:zlib";

import { createKeyOperationsProvider } from "../dist/node.js";

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
const ed = provider.generateEd25519();
const x = provider.generateX25519();
const message = new Uint8Array(1024);
const peer = provider.generateX25519();
const peerPublic = peer.publicKey().bytes;
const samples = [];
for (let index = 0; index < 122; index += 1) {
  const started = performance.now();
  const signature = ed.sign(message);
  if (!provider.verifyEd25519(ed.publicKey().bytes, message, signature)) {
    throw new Error("measurement verification failed");
  }
  x.deriveSharedSecret(peerPublic);
  const durationMs = performance.now() - started;
  if (index >= 21) samples.push(durationMs);
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
})}\n`);
