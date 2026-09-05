# Add wallet storage adapter conformance

## Why

Issue #89 established policy-neutral wallet storage ports, but downstream
implementations cannot yet produce comparable behavioral evidence. Repeating
ad hoc tests in Oxid, midnight-identity and future consumers would allow the
same contract to drift in several repositories.

## What changes

- Add `identus-wallet-conformance` as a verification-layer test-support crate.
- Exercise each actual `identus-wallet` port through executor-neutral async
  checks and consumer-owned fixture types.
- Cover exact-record conditional mutation for every store and bounded recovery
  pagination for the three list-capable stores.
- Emit only static failure classes and aggregate receipt counts.
- Strengthen the storage contract so every successful replacement invalidates
  the preceding record revision.
- Prove the kit with a test-only memory implementation and a release diagnostic.

## Non-goals

This change does not add a production storage adapter, database, encryption,
codec, migration, runtime, synchronization policy, consumer repository edit,
FFI, chain behavior or product authorization. It does not advance `IDR-010`
to delivered without two independent downstream receipts.

## Impact

- **Issue:** #91, child of #20 / `IDR-010`; predecessor #89.
- **Owner:** new verification-layer `identus-wallet-conformance` crate.
- **Compatibility:** additive unreleased test-support API plus a clarification
  of the already intended compare-and-swap revision semantics.
- **Dependencies:** runtime `identus-wallet` only; no executor or external
  dependency.
- **Rollback:** revert the focused change before publication.
