# SDK-TS HD-derivation quality evidence

Issue #612 adds a private, opt-in, profile-explicit TypeScript facade over
SDK-Rust hierarchical derivation. It does not activate a default provider,
publish an artifact, acquire mnemonics, export private material, add custody or
promise React Native support.

## Profile and secret review

- `secp256k1-bip32-hardened-v1` accepts 16–64 seed bytes and hardened private
  axes only; its public projection is compressed SEC1.
- `ed25519-slip10-hardened-v1` accepts 16–64 seed bytes and hardened private
  axes only; its public projection is a 32-byte Ed25519 key.
- `ed25519-cardano-v2-v1` accepts exactly the 64-byte Apollo-compatible seed
  representation, permits hard/soft private derivation, and exposes a distinct
  soft-only extended-public handle.
- The TypeScript adapter copies seed input into the generated binding and
  overwrites the caller-supplied buffer synchronously on success and failure.
  Other aliases, mnemonic material, and prior copies remain caller-owned.
- No public interface exports private or chain-code bytes. Every public
  projection is copied before generated ownership can be released.
- Paths require exact lowercase `m`, at most 4,096 UTF-8 bytes and 255 axes;
  every axis remains below the hardened offset before its profile rule runs.
- Explicit disposal is idempotent and frees generated ownership once;
  finalization is defense in depth. Errors and observations contain only
  closed codes plus bounded operation/profile/depth metadata.

## Local evidence

The following checks passed on 2026-11-20 under pinned Rust 1.99.1, Node 24,
wasm-pack 0.15.0, and wasm-bindgen-cli 0.2.121:

- Rust fmt, clippy with warnings denied, seven focused `identus-wasm-crypto`
  tests, and five canonical derivation-path tests passed.
- `npm --prefix packages/identus-sdk test`: 63 package tests passed.
- Focused HD provider coverage reached 89.50% lines and 73.77% branches, above
  the 80%/70% acceptance floor.
- The isolated package/types, browser Chromium, Node ESM/CommonJS, React SSR,
  and hydrated Chromium lanes passed at exact head
  `87156727348f6b78a0708be0e8d6b87d8409e32e` with artifact SHA-256
  `ea48c356b6c2089396f092ee22fe22e81af93a90a70aeac9115acc32f7096281`.
- One neutral assertion body exercised BIP-32, SLIP-0010, and Cardano V2 public
  results in every applicable runtime.
- `scripts/fuzz-crypto.sh replay hd_derivation` compiled with AddressSanitizer
  and replayed the checked-in seed without a finding.
- The cross-language catalog passed with 43 sources, 20 packets, and 94 vectors;
  all three HD cases are pinned to immutable normative or consumer sources.

The private crypto artifacts measured 337,359 Web bytes and 330,434 Node.js
bytes before compression, with deterministic gzip sums of 156,945 and 155,514
bytes. On this host, 101 warm batches spanning all three roots, three private
axes, and Cardano soft public derivation had a 0.739 ms median and 0.835 ms 95th
percentile. These are measurement-only baselines, not support budgets.

## Residual limits

WASM cannot prove physical erasure of engine copies, allocator history,
snapshots, or compromised hosts. Mnemonic acquisition, wallet path allocation,
platform keystores, KMS/HSM, backup/recovery, external SDK-TS activation,
accepted runtime budgets, React Native, publication, stable support,
certification, release, and mdoc remain outside this slice.
