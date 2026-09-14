# Encapsulate HD-key secret state

## Why

The unpublished `identus-crypto` candidate exposes the private-key and
chain-code arrays of `HDKey` and `EdHDKey` as public fields. Reading either
field creates an ordinary caller-owned copy outside the type's
`ZeroizeOnDrop` boundary. Issue #69 intentionally retained that compatibility
surface while it fixed debug redaction and owned-buffer erasure; issue #269 is
the pre-publication follow-up that removes the unsafe-by-default copy boundary.

## What changes

- Make the private-key and chain-code fields of both HD key types private.
- Add a narrow SDK-owned exposure value that owns exactly one 32-byte copy,
  erases it on drop, redacts `Debug`, and lends bytes only through an
  explicitly named method.
- Add explicitly named private-key and chain-code exposure methods on both HD
  key types.
- Preserve every BIP-32, SLIP-0010, Apollo-overlap, error, feature, and target
  result.
- Add compile-fail API regressions and update the unpublished
  `0.1.0-rc.1` public-API baseline intentionally.

## Capabilities

### Modified capabilities

- `crypto`: replace publicly copyable HD secret fields with an opaque,
  zeroizing, redaction-safe and explicitly borrowed exposure boundary.

## Public compatibility

This is an intentional Rust source-breaking change: field reads and struct
literals from external crates stop compiling. The candidate has never been
published, canonical packages remain `0.0.0` with `publish = false`, and no
downstream adoption is activated, so the safer API replaces the unpublished
`0.1.0-rc.1` baseline rather than carrying a deprecated leak-prone facade.
Field readers can move to the named exposure methods. Struct-literal callers
have no raw-state import replacement: they can reconstruct from the original
seed and supported derivation path, while private-key/chain-code/metadata
rehydration remains an explicit unsupported pre-release limitation deferred to
a separate security and API decision.

## Non-goals

- No new derivation algorithm, curve, path rule, custody backend, key handle,
  keystore, HSM, secure enclave, serialization, or FFI export.
- No stronger memory-erasure claim than best-effort ownership and `zeroize`.
- No publication, tag, release, `main` change, downstream adoption, or work on
  issues #7 or #168.

## Delivery

Issue #269 owns the change from protected
`develop@dbef9923e65d1a8332c4ba38f42532c8a12811f7`. The PR targets `develop`,
preserves the planning commit, and requires focused crypto/vector/API/target
evidence plus the existing protected checks.
