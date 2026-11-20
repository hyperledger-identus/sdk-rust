import assert from "node:assert/strict";
import { test } from "node:test";

import { CryptoError } from "../dist/public.js";
import { createRustKeyOperationsProvider } from "../dist/internal/crypto-provider.js";

function rawHandle({ publicKey, sign, derive }) {
  let disposed = false;
  let freeCount = 0;
  return {
    publicKeyBytes: () => new Uint8Array(publicKey),
    sign,
    deriveSharedSecret: derive,
    dispose() {
      disposed = true;
    },
    free() {
      freeCount += 1;
    },
    state: () => ({ disposed, freeCount }),
  };
}

test("imports private bytes through a copy and overwrites the caller buffer", () => {
  let received;
  const raw = rawHandle({ publicKey: new Uint8Array(32), sign: () => new Uint8Array(64) });
  const provider = createRustKeyOperationsProvider({
    cryptoBindingApiVersion: () => 1,
    importEd25519(value) {
      received = new Uint8Array(value);
      return raw;
    },
    generateEd25519: () => raw,
    verifyEd25519: () => true,
    generateX25519: () => raw,
    importX25519: () => raw,
  });
  const input = Uint8Array.from({ length: 32 }, (_, index) => index + 1);
  const handle = provider.importEd25519(input);
  assert.deepEqual(received, Uint8Array.from({ length: 32 }, (_, index) => index + 1));
  assert.deepEqual(input, new Uint8Array(32));
  assert.equal(handle.publicKey().algorithm, "Ed25519");
});

test("disposal is idempotent and use after disposal is a stable redacted error", () => {
  const raw = rawHandle({ publicKey: new Uint8Array(32), sign: () => new Uint8Array(64) });
  const provider = createRustKeyOperationsProvider({
    cryptoBindingApiVersion: () => 1,
    importEd25519: () => raw,
    generateEd25519: () => raw,
    verifyEd25519: () => true,
    generateX25519: () => raw,
    importX25519: () => raw,
  });
  const handle = provider.generateEd25519();
  handle.dispose();
  handle.dispose();
  assert.deepEqual(raw.state(), { disposed: true, freeCount: 1 });
  assert.throws(() => handle.sign(new Uint8Array()), (error) => {
    assert.equal(error instanceof CryptoError, true);
    assert.equal(error.code, "crypto.handle_disposed");
    assert.equal(error.stack, undefined);
    return true;
  });
});

test("copies public outputs and fails closed on contract drift", () => {
  const publicKey = Uint8Array.from({ length: 32 }, (_, index) => index);
  const raw = rawHandle({ publicKey, derive: () => new Uint8Array(32).fill(7) });
  const provider = createRustKeyOperationsProvider({
    cryptoBindingApiVersion: () => 1,
    importEd25519: () => raw,
    generateEd25519: () => raw,
    verifyEd25519: () => false,
    generateX25519: () => raw,
    importX25519: () => raw,
  });
  const handle = provider.generateX25519();
  const projected = handle.publicKey();
  publicKey.fill(255);
  assert.equal(projected.bytes[0], 0);
  assert.deepEqual(handle.deriveSharedSecret(new Uint8Array(32)), new Uint8Array(32).fill(7));

  assert.throws(
    () => createRustKeyOperationsProvider({ cryptoBindingApiVersion: () => 2 }),
    (error) => error instanceof CryptoError && error.code === "adapter.binding_contract",
  );
});
