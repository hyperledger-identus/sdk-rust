# Narrow the DID crypto feature cone

## Why

`identus-did` currently activates the complete default feature set of
`identus-crypto`, even though neither its production source nor its tests use
that crate. A DID-only consumer therefore compiles key algorithms, derivation,
hashing, JWK conversion and COSE support that the DID domain model neither owns
nor invokes. The edge also obscures the intended boundary between structural
public-key material in a DID document and algorithm-specific key validation.

## What changes

- Remove the unused `identus-did -> identus-crypto` dependency instead of
  replacing it with another speculative feature selection.
- Keep DID `publicKeyJwk` handling structural, bounded and public-only; callers
  that perform cryptographic operations remain responsible for converting it
  to an algorithm-bound key.
- Add an executable manifest guard for the exact current internal DID runtime
  dependency cone.
- Prove that DID default and no-default builds have the same crypto-free graph
  and retain current behavior on supported hosts and compile-only targets.
- Correct the current architecture inventory and dependency contract to match
  the already accepted DID and JOSE surfaces.

## Non-goals

This change does not alter DID syntax, documents, resolution, dereferencing,
registration, caching, JWK wire validation, algorithms, JOSE verification,
method-specific behavior, public APIs, donor repositories or publication. It
does not introduce DID crate features merely to represent absent behavior.

## Impact

- **Issue:** #101 under #9, related to #5 and #98.
- **Owner:** `identus-did` manifest and repository conformance evidence.
- **Compatibility:** behavior-neutral and unreleased; the public Rust and wire
  surfaces do not change.
- **Dependencies:** one unused internal dependency is removed; no dependency is
  added and no lockfile package is expected to change at workspace scope.
- **Rollback:** restore the manifest edge and remove its regression assertion;
  no data, API or consumer migration is involved.
