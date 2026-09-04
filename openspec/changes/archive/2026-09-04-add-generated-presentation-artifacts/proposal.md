# Add generated presentation artifacts and receipt inputs

## Why

`identus-presentations` now validates a holder's credential and claim choices,
but format adapters still have no shared boundary for returning generated
presentation bytes or producing a privacy-conscious receipt input. Without
that boundary, OID4VP, Midnight and Oxid would each recreate artifact-to-plan
correlation and persistence-facing disclosure summaries.

## What changes

- Bind each disclosure plan to the exact request used during construction.
- Add an opaque bounded presentation artifact that can represent one or more
  same-format disclosure selections.
- Add a generated-presentation aggregate that validates exact request, format,
  binding uniqueness and complete selection coverage.
- Derive an owner-private, value-free receipt input from the validated result
  without retaining challenge or artifact bytes.
- Extend static redacted errors, inventories and release-mode performance
  diagnostics for the new contracts.

## Non-goals

This change does not generate or verify proofs; authorize a holder; parse or
serialize artifacts; encode a VP Token, DCQL response or transport; decide
consent, trust, receipt timing or retention; claim delivery or verifier
acceptance; store lifecycle state; add persistence, FFI, chain or product
behavior; publish the crate; or modify a downstream repository.

## Impact

- **Issue:** #83, under `IDR-008`, #20 and predecessor #81.
- **Owner:** existing experimental `identus-presentations` crate.
- **Compatibility:** additive unreleased API plus private disclosure-plan
  request state; no serialized form.
- **Dependencies:** no new crate, external package or feature edge.
- **Rollback:** revert the focused change before publication.
