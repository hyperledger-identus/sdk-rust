## Why

The format-neutral credential envelope landed in #72, but callers still have
no generic way to distinguish structurally valid, cryptographically invalid,
or incompletely checked credentials. Oxid supplies a useful staged pattern,
while Midnight and NeoPRISM status systems prove that evidence producers must
remain adapters. Oxid's report also includes a product trust stage and accepts
a caller-supplied aggregate outcome; neither belongs in generic verification.

## What Changes

- Add six fixed, policy-neutral verification stage names.
- Add bounded machine reason codes and invariant-preserving stage values.
- Add a complete, canonical fixed-array report whose aggregate outcome is
  derived rather than caller supplied.
- Keep trust, evidence payloads, resolvers, crypto, status/schema execution,
  codecs, storage, and product decisions outside the model.
- Extend stable credential construction errors and focused tests.
- Record a release-mode throughput observation for the bounded construction
  path without imposing a machine-dependent timing assertion.

## Capabilities

### New Capabilities

- `credential-verification`: policy-neutral staged evidence and aggregate
  outcome semantics.

### Modified Capabilities

- `sdk-governance-evidence`: inventory the expanded experimental surface and
  retain explicit non-scope.

## Impact

- **Issue:** #73, child of #6 / `IDR-009` and #20.
- **Code:** one focused module plus the existing credential error catalogue.
- **Dependencies:** none; the existing crate dependency graph is unchanged.
- **Compatibility:** additive, experimental and unreleased; no wire contract.
- **Consumers:** Oxid, midnight-identity, Lace ID Portal, and NeoPRISM remain
  read-only evidence sources.
