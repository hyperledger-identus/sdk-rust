## Why

The bounded JWS Compact codec now carries reusable proof-JWT traffic for Oxid
and NeoPRISM-facing consumers. Deterministic conformance covers known cases but
does not search hostile byte combinations and caller-selected limit tuples for
panics, noncanonical acceptance, signing-input drift or inconsistent encoding.

Issue #100 closes that assurance gap without changing production behavior. It
extends the existing independent SDK fuzz workspace and keeps sanitizer/runtime
dependencies outside every published crate.

## What Changes

- Add one arbitrary-byte JWS Compact target which exercises default limits and
  bounded limit tuples derived from the same input.
- Assert static errors, canonical unpadded base64url, exact received signing
  input, byte-preserving reparsing and semantic builder round trips.
- Add independently authored RFC-, Oxid- and Lace-shaped seeds plus every
  existing negative boundary family and a JWS dictionary.
- Add a pinned replay/smoke/soak wrapper and path-scoped sanitizer workflow.
- Document provenance, resource ceilings, triage, residual risk and performance
  evidence; update the canonical JWS contract after review.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `jws-compact`: makes bounded sanitizer/property search part of the reusable
  codec conformance contract.

## Impact

- **Issue:** #100, child of #8 / `IDR-004`.
- **Affected surface:** independent `fuzz/` workspace, JWS fuzz wrapper,
  path-scoped workflow, documentation, ADR 0039 and OpenSpec evidence.
- **Public/wire compatibility:** unchanged; the target consumes public APIs.
- **Dependencies:** fuzz-only `base64` and `identus-jose`; no published crate or
  root lock change.
- **Consumers:** no downstream repository changes in this slice.
- **Rollback:** one focused revert; no release, data, chain, product, protected
  branch or repository setting changes.
