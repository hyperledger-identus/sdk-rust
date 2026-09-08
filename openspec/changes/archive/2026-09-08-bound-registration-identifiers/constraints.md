# Constraint readiness

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/209
Constraint blockers: none

## Existing entries affected

`SDK-SEC-003` requires bounded changed input surfaces. `SDK-LIM-007` remains
effective but gains explicit evidence for DID Registration identifier borrowed
parsing. `SDK-DELIVERY-001` requires issue-linked specification, review, and CI.
Compatibility, architecture, unsafe, dependency, target, release, chain, and
product constraints remain unchanged.

## Introduced or changed constraints

- Borrowed parsing for all six registration opaque identifier types SHALL enforce
  the existing type ceiling before allocating the retained success value.
- Existing grammar, limits, owned move semantics, APIs, and redaction SHALL remain.

No dependency, compiler, feature, target, release, chain, or product constraint
changes.

## Introduced or changed limitations

- Outer transport/deserializer allocations remain consumer-budgeted; #168 and
  `SDK-LIM-007` are not closed by this slice.

## Consumer and product impact

Current SDK consumers preserve accepted identifiers, APIs, and lifecycle
behavior. Midnight, Cardano, wallet, and method-specific policy remain downstream.

## Activation and rollback

The constraint activates only after #209 passes local and hosted gates and its
focused PR merges into `develop`. A focused revert restores prior behavior;
there is no wire or data migration.

## Evidence

Issue #209, this OpenSpec contract, ADR 0094, all-type boundary tests, focused
source review, factory checks, Nix gates, and hosted CI are required evidence.
