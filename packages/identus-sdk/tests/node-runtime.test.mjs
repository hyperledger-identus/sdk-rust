import assert from "node:assert/strict";
import { createRequire } from "node:module";
import { test } from "node:test";

import { assertAllVectors } from "./vector-assertions.mjs";
import { loadVectors } from "./vector-loader.mjs";
import { assertAllDocumentVectors } from "./document-vector-assertions.mjs";
import { loadDocumentVectors } from "./document-vector-loader.mjs";

test("Node ESM executes every canonical DID vector", async () => {
  const { createDidProvider } = await import("../dist/node.js");
  const provider = await createDidProvider();
  assertAllVectors(provider, await loadVectors());
  assertAllDocumentVectors(provider, await loadDocumentVectors());
});

test("Node CommonJS shares the ESM facade and executes every canonical DID vector", async () => {
  const require = createRequire(import.meta.url);
  const entry = require("../cjs/node.cjs");
  assert.deepEqual(Object.keys(entry), ["createDidProvider"]);
  const provider = await entry.createDidProvider();
  assertAllVectors(provider, await loadVectors());
  assertAllDocumentVectors(provider, await loadDocumentVectors());
});
