# Decompose DID document model and validation

## Why

`crates/did/src/document.rs` contains 1,083 authored nonblank lines and owns
wire-cardinality preservation, JSON-LD context and extension budgets,
verification methods, services, aggregate document validation, native
construction, serde construction, and rejection cleanup. Discussion #399 and
issue #408 identify it as the last production module above the 1,000-line
attention threshold. Its adversarial corpus permits a private ownership move
without changing DID Core behavior.

## What changes

- Keep `document.rs` as a private facade and preserve every crate-root export.
- Isolate scalar-or-array cardinality representation.
- Isolate bounded JSON-LD contexts and generic extension-tree validation.
- Isolate verification methods, public-material checks, and relationships.
- Isolate service types, endpoints, validation, and cleanup projection.
- Keep `DidDocument`, its builder, cross-document validation, serde boundary,
  and aggregate cleanup in one document owner.
- Preserve limits, validation order, errors, duplicate detection, canonical
  multibase rules, redaction, semantic round trips, and iterative cleanup.

## Capability

### Added capability

- `did-document-module-ownership`: cohesive private ownership and stable
  compatibility boundaries for the bounded DID document model.

## Non-goals

No new DID method, verification suite, service type, JSON-LD interpretation,
wire shape, error, dependency, feature, target, persistence, authorization, or
resolution behavior is introduced.

## Delivery

Issue #408 owns this slice. Planning is committed before implementation and is
rebased onto the merged issue #407 `develop` tip before exact-head preflight.
DID characterization, public/error/source contracts, cleanup hardening,
code-health evidence, portable targets, and protected CI prove compatibility.
