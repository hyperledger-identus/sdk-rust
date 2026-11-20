# SDK-TS key-operations quality evidence

Issue #611 adds a private, opt-in Ed25519/X25519 facade over the existing
`identus-crypto` semantics. It does not activate a default provider, publish an
artifact, or add custody, persistence, derivation, protocol signing, or mdoc.

## Secret-handling review

- Private keys enter only through exact 32-byte mutable buffers or system
  entropy and remain in Rust-owned opaque WASM handles.
- The TypeScript adapter copies import input into the generated binding and
  overwrites the caller-supplied buffer synchronously on both success and
  failure. Other aliases and earlier copies remain caller-owned.
- No public interface exports private bytes. Public keys, signatures, and
  shared secrets are copied out of generated ownership; errors and observations
  contain only closed codes and bounded operation metadata.
- Explicit disposal is idempotent, unregisters finalization, and frees the
  generated owner once. Finalization is defense in depth when explicit
  lifecycle authority is abandoned.
- X25519 rejects malformed, noncanonical, and non-contributory peer values.
  Ed25519 verification rejects malformed lengths and returns `false` only for a
  well-shaped cryptographic mismatch.
- TypeScript rejects messages above 1,048,576 bytes before invoking WASM. Rust
  independently enforces the same bound.

## Local evidence

The following checks passed on 2026-11-20 under the pinned Rust 1.99.1 and
TypeScript development shells:

- `cargo test -p identus-wasm-crypto`: four focused Rust tests passed.
- `cargo check -p identus-wasm-crypto --target wasm32-unknown-unknown` passed.
- `npm --prefix packages/identus-sdk test`: 58 package tests passed.
- Focused Node coverage for the strict crypto provider reached 84.92% lines,
  89.09% branches, and 90.91% functions, above the 80%/70% acceptance floor.
- Browser ESM executed both normative crypto vectors in Chromium.
- React SSR and hydrated Chromium reused the same assertion body and passed.
- Node ESM and CommonJS reused the same assertion body and passed.
- `scripts/fuzz-crypto.sh replay key_operations` compiled under AddressSanitizer
  and replayed the checked-in seed without a finding.
- The isolated crypto target runner passed package/types, browser, Node.js, and
  React Web modes while producing separate exact-head target receipts.

Measured private artifact sizes were 157,729 bytes for Web and 151,999 bytes
for Node.js before compression. The deterministic gzip sums were 80,080 and
78,733 bytes. On this host, 101 warm combined Ed25519 sign/verify plus X25519
derive samples over a 1,024-byte message had a 0.296 ms median and 0.319 ms
95th percentile. These are measurement-only baselines, not support budgets.

## Residual limits

WASM cannot prove physical erasure of engine copies, allocator history,
snapshots, or compromised hosts. React Native, platform keystores, KMS/HSM,
backup/recovery, external adoption, publication, stable support, certification,
release, and mdoc remain outside this slice.
