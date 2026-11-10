import assert from "node:assert/strict";
import { createRequire } from "node:module";
import { test } from "node:test";

import { assertAllVectors } from "./vector-assertions.mjs";

test("Node ESM executes every canonical DID vector", async () => {
  const { createDidProvider } = await import("../dist/node.js");
  await assertAllVectors(await createDidProvider());
});

test("Node CommonJS shares the ESM facade and executes every canonical DID vector", async () => {
  const require = createRequire(import.meta.url);
  const entry = require("../cjs/node.cjs");
  assert.deepEqual(Object.keys(entry), ["createDidProvider"]);
  await assertAllVectors(await entry.createDidProvider());
});
