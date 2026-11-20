# Add the SDK-TS HD derivation train

## Why

SDK-Rust already owns bounded path parsing and three distinct deterministic
derivation profiles. The private TypeScript package cannot use them without
either retaining the historical SDK-TS implementation or collapsing different
algorithms behind one ambiguous HD-key name.

## What Changes

- Extend the private crypto WASM artifact with opaque Rust-owned derivation
  handles for hardened-only secp256k1 BIP-32, hardened-only Ed25519 SLIP-0010,
  and Cardano/IOG Ed25519-BIP32 V2.
- Add strict TypeScript profile, path, public-projection, and disposable handle
  values without the historical component names, generic key bags, mnemonic
  acquisition, or private export.
- Accept only bounded caller-owned seed buffers, overwrite the supplied mutable
  buffer synchronously, and retain derived secret state only inside Rust-owned
  handles.
- Freeze applicable SDK-TS donor journeys plus normative BIP-32, SLIP-0010,
  and Cardano V2 vectors, including profile-specific hardened/soft and
  public/private derivation rejection cases.
- Run one neutral contract across Browser, Node.js, and React Web, with
  deterministic artifacts, isolated CI receipts, quality evidence, capability
  status, consumer-change evidence, and explicit legacy rollback.

## Capabilities

### New Capabilities

- `sdk-ts-hd-derivation`: define the private profile-explicit derivation
  facade, opaque secret lifecycle, path contract, target evidence, and rollback.

## Impact

The change extends the unpublished `identus-wasm-crypto` artifact and private
TypeScript package over existing in-tree `identus-crypto` primitives. It adds
no new cryptographic dependency, algorithm, host entropy source, storage, or
consumer repository mutation.

Mnemonic generation/validation, BIP-39 seed creation, X25519 conversion,
private export, keystore/KMS/hardware custody, persistence, wallet policy,
default SDK-TS activation, React Native, publication, stable support, release,
and mdoc remain outside this train.
