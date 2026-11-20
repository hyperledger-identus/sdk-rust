import { test } from "node:test";

import { createHdDerivationProvider, createKeyOperationsProvider } from "../dist/node.js";
import { assertCryptoVectors } from "./crypto-vector-assertions.mjs";
import { assertHdDerivationVectors } from "./hd-vector-assertions.mjs";

test("Node executes the RFC 8032 and RFC 7748 key-operation vectors", async () => {
  const provider = await createKeyOperationsProvider();
  assertCryptoVectors(provider);
});

test("Node executes the BIP-32, SLIP-0010, and Cardano V2 HD vectors", async () => {
  assertHdDerivationVectors(await createHdDerivationProvider());
});
