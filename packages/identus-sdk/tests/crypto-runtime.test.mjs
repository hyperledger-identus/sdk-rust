import { test } from "node:test";

import { createKeyOperationsProvider } from "../dist/node.js";
import { assertCryptoVectors } from "./crypto-vector-assertions.mjs";

test("Node executes the RFC 8032 and RFC 7748 key-operation vectors", async () => {
  const provider = await createKeyOperationsProvider();
  assertCryptoVectors(provider);
});
