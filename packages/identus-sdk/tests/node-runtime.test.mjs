import assert from "node:assert/strict";
import { createRequire } from "node:module";
import { test } from "node:test";

import { assertAllVectors } from "./vector-assertions.mjs";
import { loadVectors } from "./vector-loader.mjs";
import { assertAllDocumentVectors } from "./document-vector-assertions.mjs";
import { loadDocumentVectors } from "./document-vector-loader.mjs";
import { assertAllResolutionVectors } from "./resolution-vector-assertions.mjs";
import { loadResolutionVectors } from "./resolution-vector-loader.mjs";
import { assertAllPeerVectors } from "./peer-vector-assertions.mjs";
import { loadPeerVectors } from "./peer-vector-loader.mjs";

test("Node ESM executes every canonical DID vector", async () => {
  const { createDidProvider, createDidQueryProvider } = await import("../dist/node.js");
  const provider = await createDidProvider();
  assertAllVectors(provider, await loadVectors());
  assertAllDocumentVectors(provider, await loadDocumentVectors());
  assertAllPeerVectors(provider, await loadPeerVectors());
  await assertAllResolutionVectors(createDidQueryProvider, await loadResolutionVectors());
});

test("Node CommonJS shares the ESM facade and executes every canonical DID vector", async () => {
  const require = createRequire(import.meta.url);
  const entry = require("../cjs/node.cjs");
  assert.deepEqual(Object.keys(entry), [
    "createDidProvider",
    "selectDidProvider",
    "createDidQueryProvider",
  ]);
  const peerless = Object.freeze({
    parseDid: (value) => Object.freeze({
      serialized: value,
      method: "example",
      methodSpecificId: "selected",
    }),
    parseDidUrl: () => { throw new Error("unused peerless URL path"); },
    parseDidDocument: () => { throw new Error("unused peerless document path"); },
  });
  assert.strictEqual(
    await entry.selectDidProvider({ kind: "injected", provider: peerless }),
    peerless,
  );
  const provider = await entry.createDidProvider();
  assertAllVectors(provider, await loadVectors());
  assertAllDocumentVectors(provider, await loadDocumentVectors());
  assertAllPeerVectors(provider, await loadPeerVectors());
  await assertAllResolutionVectors(entry.createDidQueryProvider, await loadResolutionVectors());
});
