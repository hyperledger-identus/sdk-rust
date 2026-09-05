# Add wallet storage ports

## Why

The SDK now owns reusable DID, credential, presentation and verification
semantics, but every consumer still invents persistence interfaces. Oxid has
separate credential, DID and wallet repositories; Midnight carries private
controller state; Lace persists protocol sessions. Their concrete storage,
encryption and product identifiers remain downstream, while their shared need
for bounded, asynchronous, conflict-aware capabilities belongs in SDK-Rust.

## What changes

- Activate `identus-wallet` with policy-neutral storage port contracts.
- Add bounded opaque revision, cursor and pagination vocabulary.
- Add explicit conditional write/delete and redacted receipt/outcome types.
- Add five distinct object-safe async capabilities for secrets, credentials,
  DIDs, protocol state and status cache entries.
- Keep consumer record/key/scope/index types behind associated types.
- Prove the contracts with consumer-shaped in-memory test doubles and measure
  trait-object dispatch overhead.

## Non-goals

This change does not define raw key formats, key generation or signing;
implement encryption, codecs, databases, files, keychains or cloud storage;
promise cross-record transactions; select synchronization, retention, backup,
consent or authorization policy; add product identifiers, chain checkpoints,
protocol engines, FFI or production adapters; release a crate; or modify a
consumer repository.

## Impact

- **Issue:** #89, child of #20 / `IDR-010`.
- **Owner:** existing orchestration-layer `identus-wallet` crate, activated
  from quarantine for this bounded experimental surface.
- **Compatibility:** additive unreleased API; no wire or stored-data format.
- **Dependencies:** runtime `identus-core` plus build-time `identus-derive`;
  no domain, protocol, external, adapter or consumer dependency.
- **Rollback:** revert the focused change before publication.
