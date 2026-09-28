# Decompose presentation model responsibilities

## Why

`crates/presentations/src/model.rs` contains 1,134 authored nonblank lines and
owns scalar syntax, semantic requests, candidate matching, disclosure plans,
generated artifacts, and receipt projection. Discussion #399 and issue #407
identify it as a remaining domain hotspot. Its public behavior is extensively
characterized, so this slice improves private ownership without changing
presentation semantics.

## What changes

- Keep `model.rs` as a private facade and preserve every crate-root export.
- Isolate bounded presentation scalar values and opaque identifiers.
- Isolate request/query/filter/claim models and their invariants.
- Isolate candidate and disclosure-selection validation.
- Isolate generated artifact coverage, byte budgets, and receipt projection.
- Preserve limits, validation order, errors, value-free semantics, redaction,
  format orthogonality, and public API.

## Capability

### Added capability

- `presentation-module-ownership`: cohesive private ownership and stable
  compatibility boundaries for format-neutral presentation semantics.

## Non-goals

No new format, protocol, discovery, ranking, consent, proof, trust, lifecycle,
persistence, I/O, wire representation, error, dependency, feature, or target
behavior is introduced.

## Delivery

Issue #407 owns this slice. The planning-only contract was rebased onto the
current protected `develop` tip before exact-head preflight or code changes.
Presentation characterization, public/error/source contracts, code-health
evidence, portable targets, and protected CI prove compatibility.
