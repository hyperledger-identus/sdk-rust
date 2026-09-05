# Add credential verifier execution

## Why

`identus-credentials` owns bounded envelopes and canonical staged verification
evidence, but consumers still have no shared execution boundary. Oxid proves
that credential verification is asynchronous and adapter-backed. Midnight and
Cardano prove that proof, DID, status and schema dependencies must remain in
chain- or format-specific adapters rather than the generic credential crate.

## What changes

- Add an object-safe asynchronous credential-verifier capability.
- Add a borrowed request that deliberately excludes format-private holder
  material.
- Separate completed invalid evidence from operational execution failures.
- Add a bounded immutable registry that dispatches one exact credential format
  to one verifier.
- Prove the extension seam with unrelated dummy-format adapters and record a
  release-mode dispatch observation.

## Non-goals

This change does not implement a concrete credential format; parse or verify
proofs; resolve DIDs; fetch or interpret status; validate schemas; decide
trust; define stage scheduling; add clocks, retries, cancellation tokens,
caching, codecs, storage, FFI or telemetry; create an adapter crate; publish a
release; or modify a downstream repository.

## Impact

- **Issue:** #87, child of #6 / `IDR-009` and #20.
- **Owner:** existing experimental `identus-credentials` crate.
- **Compatibility:** additive unreleased API; no wire or persistence contract.
- **Dependencies:** build-time-only `identus-derive` edge required by the
  existing port-declaration convention; no runtime or external dependency.
- **Rollback:** revert the focused change before publication.
