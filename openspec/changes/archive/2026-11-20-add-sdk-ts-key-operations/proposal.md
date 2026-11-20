# Add the SDK-TS key operations train

## Why

SDK-Rust already owns portable Ed25519 and X25519 key generation, parsing,
public encoding, signing, strict verification, and contributory shared-secret
semantics. The private TypeScript package has no capability-named key facade,
so TS1 cannot exercise those Rust semantics in Browser, Node.js, or React Web
without retaining the historical SDK-TS crypto implementation.

## What Changes

- Add a separate `identus-wasm-crypto` artifact with opaque Rust-owned private
  handles, public-key values, system-entropy generation, import, signing,
  verification, shared-secret derivation, and deterministic disposal.
- Add strict TypeScript Ed25519 and X25519 values and operations without the
  historical component names, generic key bags, private export, or implicit
  curve conversion.
- Freeze applicable SDK-TS donor journeys and cross-language positive,
  negative, resource, mutation, and secret-redaction vectors.
- Run one neutral contract across Browser, Node.js, React SSR/hydration, with
  explicit React Native inapplicability and isolated target receipts.
- Reconcile adapter mappings, quality declarations, resource boundaries,
  capability status, consumer-change evidence, artifact assembly, and CI
  impact routing.

## Capabilities

### New Capabilities

- `sdk-ts-key-operations`: define the private key-operation facade, opaque
  secret lifecycle, strict public projection, target evidence, and rollback.

## Impact

The change adds one private WASM crypto crate and extends the private
TypeScript package, deterministic artifact assembly, target tests, vectors,
and governance registries. It composes only in-tree `identus-crypto` and the
reviewed system entropy adapter.

HD derivation, mnemonic/seed APIs, secp256k1, P-256, private-key export,
non-exportable platform keystores, hardware/KMS, persistence, wallet custody,
default SDK-TS activation, React Native, publication, stable support, release,
and mdoc remain outside this train.
