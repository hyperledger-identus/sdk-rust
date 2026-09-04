# Add validated presentation disclosure plans

## Why

`identus-presentations` can express bounded requests and candidates, but a
holder-side adapter still has no reusable value for handing an already-made
credential and claim choice to a format-specific proof implementation. Without
that boundary, Oxid, Midnight and OID4VP adapters would each recreate the same
cross-object validation or pass loosely related query IDs, credential handles
and claim paths.

## What changes

- Add a value-free selected-claim type that preserves one requested path and
  reveal-or-predicate intent.
- Add a bounded credential selection that identifies one request query, one
  candidate handle and its selected claims.
- Add a bounded disclosure plan that validates all selections against both the
  request and candidate set, including full query coverage and multiplicity.
- Extend the static presentation error and redaction contracts for the new
  cross-object failures.
- Record a release-mode validation diagnostic without making host timing a
  correctness gate.

## Non-goals

This change does not choose, rank or discover candidates; decide user consent,
trust or disclosure policy; model DCQL claim/credential alternatives; carry
claim values, openings or predicate parameters; perform holder binding or
proofs; encode protocol wire data; create a presentation artifact or receipt;
store lifecycle state; add persistence, FFI, chain or product behavior; publish
the crate; or modify a downstream repository.

## Impact

- **Issue:** #81, under `IDR-008` and #20.
- **Owner:** existing experimental `identus-presentations` crate.
- **Compatibility:** additive, unreleased Rust API with no serialized form.
- **Dependencies:** no new crate, external package or feature edge.
- **Rollback:** revert the focused change before publication.
