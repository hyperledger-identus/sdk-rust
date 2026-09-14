# Design

## Context and goals

The current presentation contract is behaviorally sound but combines five
ownership domains in one 190-line function. The goal is smaller review units
and exact characterization, while preserving its already-single message
decision per variant.

## Decisions

### Lean private record

Add a crate-private record containing `ErrorCode` and `&'static str` message.
Its const conversion supplies the uniform `InvalidInput` kind and
`presentation` capability centrally. It allocates nothing and is neither
public, serialized, bound, nor returned.

### Five ownership modules

Add private request/query, candidate, disclosure, artifact, and lifecycle
catalogues with counts 13/9/13/9/4. Keep public enum and constants explicit.
One private macro list maps all variants to records and produces only a
test-only variant inventory; its match has no wildcard.

### Independent immutable golden

Capture 48 rows from the exact base before implementation. Each row contains
error type, variant, constant name/visibility, code, kind, capability, local
display, public message, full public display and source. Copy the bytes to a
stable presentation test fixture only after the planning receipt.

The planning artifact's immutable SHA-256 is
`3941cbdb1b3eedb26243b5caf1b8a11c4646c3789a3cab1415834c48d3a8ba49`.

Extend the existing factory checker as a bounded multi-binding validator. Do
not copy the path-security implementation. Each binding independently requires
exact provenance/hash, byte identity, one active-or-archived planning copy,
complete active state, unique archive state, root confinement, and no symlinked
component. Mutation tests cover both binding selection and presentation drift.

### Compatibility and measurement

Use a compile-time const probe and base/head public API diff. Require empty
manifest/lock/feature diffs. The regression enumerates all variants and exact
rows, and checks `Error::source() == None`.

Success is a largest router at or below 60 lines and five independently
reviewable catalogues. Behavioral decision count must be reported as 48 to 48,
not as duplication reduction. Report total physical production lines even if
they increase; no line-count budget is imposed.

## Risks and mitigations

- Golden and implementation could drift together: fixed hash/provenance and a
  planning-only commit prevent same-change regeneration.
- Test inventory could omit new variants: router and cfg(test) inventory share
  one private macro input; golden comparison rejects an added row gap.
- Kind/capability could be repeated inconsistently: keep both outside records.
- Private files could fragment the crate: use only the five existing change
  axes and keep the router with the enum.
- Target claims could be overstated: run direct compile checks and explicitly
  exclude runtime/device/binding promises.

## Migration and rollback

1. Commit ADR, OpenSpec and golden as planning-only evidence.
2. Write and validate the durable preimplementation receipt.
3. Copy the exact fixture, add the record/catalogues/router and regression.
4. Extend the existing golden checker/configuration and Nix stable-fixture
   filter.
5. Run exact compatibility, quality, target, factory and Nix evidence.
6. Resolve independent architecture/adversarial review, archive, open the PR,
   and merge only after hosted gates pass.

Rollback removes the private modules and stable fixture and restores the old
match. No external state changes.
