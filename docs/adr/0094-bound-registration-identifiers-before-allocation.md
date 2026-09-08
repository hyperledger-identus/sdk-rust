# ADR 0094: bound registration identifiers before allocation

- **Status:** Accepted for implementation
- **Date:** 2026-09-09
- **Issue:** [#209](https://github.com/hyperledger-identus/sdk-rust/issues/209)
- **Parent:** [#168](https://github.com/hyperledger-identus/sdk-rust/issues/168)
- **Decision authority:** SDK resource-bound guardrails

## Context

The DID Registration private newtype macro clones borrowed identifier input
before applying an existing 256-byte or 1,024-byte ceiling. Grammar, owned
construction, and redacted diagnostics are otherwise correct.

## Decision

Every generated borrowed `parse(&str)` SHALL validate the original slice before
calling `to_owned()`. The owned `try_new(String)` path remains validate-and-move.
Existing ceilings and grammar remain unchanged, and tests cover every generated
type at its exact and one-over boundary.

Do not add a dependency. A library cannot correct this local macro ordering or
own these SDK-specific limits and would unnecessarily widen the dependency cone.
Do not add an allocation-counting global allocator; direct source review avoids
unsafe, global test state for a four-line ordering property.

## Consequences

- Oversized borrowed identifiers are rejected before their retained-value allocation.
- Accepted input, public APIs, owned construction, and lifecycle behavior are stable.
- Outer transports may still allocate before invoking the SDK, so `SDK-LIM-007`
  and issue #168 remain active.
- No dependency, target, chain, product, protocol, or certification claim changes.

## Alternatives rejected

Delegating through `try_new(value.to_owned())` preserves the defect. A general
validated-newtype abstraction adds indirection without a second consumer. An
unsafe test allocator expands risk rather than improving the direct evidence.

## Verification and rollback

Exact/one-over, precedence, redaction, and owned/borrowed tests plus distinct
source review prove the property. Full local, Nix, and hosted gates are required.
Reverting the focused PR restores the prior order without migration.
