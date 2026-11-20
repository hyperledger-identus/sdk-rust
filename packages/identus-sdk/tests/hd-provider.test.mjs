import assert from "node:assert/strict";
import { test } from "node:test";

import { CryptoError } from "../dist/public.js";
import { createRustHdDerivationProvider } from "../dist/internal/hd-provider.js";

function rawHandle(profile, depth, length) {
  let disposed = false;
  let freeCount = 0;
  return {
    profile: () => profile,
    depth: () => depth,
    publicKeyBytes: () => new Uint8Array(length).fill(depth + 1),
    derive: () => rawHandle(profile, depth + 1, length),
    cardanoPublicHandle: () => rawHandle("ed25519-cardano-v2-v1", depth, 64),
    dispose: () => { disposed = true; },
    free: () => { freeCount += 1; },
    state: () => ({ disposed, freeCount }),
  };
}

test("creates profile-typed roots through copied seed input and erases caller custody", () => {
  let received;
  const provider = createRustHdDerivationProvider({
    hdDerivationBindingApiVersion: () => 1,
    createRoot(profile, seed) {
      received = new Uint8Array(seed);
      return rawHandle(profile, 0, profile === "secp256k1-bip32-hardened-v1" ? 33 : 32);
    },
  });
  const seed = Uint8Array.from({ length: 16 }, (_, index) => index);
  const root = provider.createRoot("secp256k1-bip32-hardened-v1", seed);
  assert.deepEqual(received, Uint8Array.from({ length: 16 }, (_, index) => index));
  assert.deepEqual(seed, new Uint8Array(16));
  assert.equal(root.profile, "secp256k1-bip32-hardened-v1");
  assert.equal(root.depth, 0);
});

test("copies every public projection and disposes generated owners exactly once", () => {
  const raw = rawHandle("ed25519-slip10-hardened-v1", 0, 32);
  const provider = createRustHdDerivationProvider({
    hdDerivationBindingApiVersion: () => 1,
    createRoot: () => raw,
  });
  const root = provider.createRoot("ed25519-slip10-hardened-v1", new Uint8Array(16));
  const projected = root.publicKey();
  projected.bytes.fill(255);
  assert.deepEqual(root.publicKey().bytes, new Uint8Array(32).fill(1));
  root.dispose();
  root.dispose();
  assert.deepEqual(raw.state(), { disposed: true, freeCount: 1 });
  assert.throws(
    () => root.publicKey(),
    (error) => error instanceof CryptoError
      && error.code === "crypto.handle_disposed"
      && error.stack === undefined,
  );
});

test("Cardano public handles preserve profile, depth, and bounded observations", () => {
  const observations = [];
  const provider = createRustHdDerivationProvider({
    hdDerivationBindingApiVersion: () => 1,
    createRoot: () => rawHandle("ed25519-cardano-v2-v1", 0, 64),
  }, { observer: (observation) => observations.push(observation) });
  const root = provider.createRoot("ed25519-cardano-v2-v1", new Uint8Array(64));
  const publicRoot = root.publicDerivation();
  const child = publicRoot.derive("m/0");
  assert.equal(child.depth, 1);
  assert.equal(child.profile, "ed25519-cardano-v2-v1");
  child.dispose();
  publicRoot.dispose();
  root.dispose();
  assert.deepEqual(
    Object.keys(observations[0]).sort(),
    ["depth", "durationMs", "errorCode", "operation", "profile", "result", "route"],
  );
  assert.equal(JSON.stringify(observations).includes("seed"), false);
});

test("binding drift and observer failures fail closed without secret diagnostics", () => {
  assert.throws(
    () => createRustHdDerivationProvider({ hdDerivationBindingApiVersion: () => 2 }),
    (error) => error instanceof CryptoError && error.code === "adapter.binding_contract",
  );
  const raw = rawHandle("ed25519-slip10-hardened-v1", 0, 32);
  const provider = createRustHdDerivationProvider({
    hdDerivationBindingApiVersion: () => 1,
    createRoot: () => raw,
  }, { observer: () => { throw new Error("secret"); } });
  const seed = new Uint8Array(16).fill(7);
  assert.throws(
    () => provider.createRoot("ed25519-slip10-hardened-v1", seed),
    (error) => error instanceof CryptoError
      && error.code === "crypto.observer_failure"
      && error.stack === undefined,
  );
  assert.deepEqual(seed, new Uint8Array(16));
  assert.deepEqual(raw.state(), { disposed: true, freeCount: 1 });
});
