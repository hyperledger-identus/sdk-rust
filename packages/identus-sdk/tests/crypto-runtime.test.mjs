import assert from "node:assert/strict";
import { test } from "node:test";

import { createKeyOperationsProvider } from "../dist/node.js";

const bytes = (hex) => Uint8Array.from(Buffer.from(hex, "hex"));

test("Node executes the RFC 8032 and RFC 7748 key-operation vectors", async () => {
  const provider = await createKeyOperationsProvider();
  const edSeed = bytes("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60");
  const ed = provider.importEd25519(edSeed);
  const signature = ed.sign(new Uint8Array());
  assert.equal(Buffer.from(ed.publicKey().bytes).toString("hex"), "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a");
  assert.equal(Buffer.from(signature).toString("hex"), "e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e065224901555fb8821590a33bacc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b");
  assert.equal(provider.verifyEd25519(ed.publicKey().bytes, new Uint8Array(), signature), true);
  assert.equal(edSeed.every((byte) => byte === 0), true);
  ed.dispose();

  const xSeed = bytes("77076d0a7318a57d3c16c17251b26645df4c2f87ebc0992ab177fba51db92c2a");
  const x = provider.importX25519(xSeed);
  assert.equal(Buffer.from(x.publicKey().bytes).toString("hex"), "8520f0098930a754748b7ddcb43ef75a0dbf3a0d26381af4eba4a98eaa9b4e6a");
  assert.equal(Buffer.from(x.deriveSharedSecret(bytes("de9edb7d7b7dc1b4d35b61c2ece435373f8343c85b78674dadfc7e146f882b4f"))).toString("hex"), "4a5d9d5ba4ce2de1728e3bf480350f25e07e21c947d19e3376f09b3c1e161742");
  assert.equal(xSeed.every((byte) => byte === 0), true);
  x.dispose();
});
