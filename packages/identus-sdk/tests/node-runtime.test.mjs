import assert from "node:assert/strict";
import { createRequire } from "node:module";
import { test } from "node:test";

import { assertAllVectors } from "./vector-assertions.mjs";
import { loadVectors } from "./vector-loader.mjs";

test("Node ESM executes every canonical DID vector", async () => {
  const { createDidProvider } = await import("../dist/node.js");
  assertAllVectors(await createDidProvider(), await loadVectors());
});

test("Node CommonJS shares the ESM facade and executes every canonical DID vector", async () => {
  const require = createRequire(import.meta.url);
  const entry = require("../cjs/node.cjs");
  assert.deepEqual(Object.keys(entry), ["createDidProvider"]);
  assertAllVectors(await entry.createDidProvider(), await loadVectors());
});
