# ADR 0129: make native DID JSON rejection cleanup iterative

- **Status:** Accepted for implementation
- **Date:** 2026-09-17
- **Decision authority:** sponsor-approved issue
  [#297](https://github.com/hyperledger-identus/sdk-rust/issues/297)
- **Related:** ADR 0125, `SDK-SEC-003`, `SDK-LIM-007`
- **Assessed revision:** sdk-rust `508917b0416147668b9d12949eec987f0b82946b`

## Context

ADR 0125 separated pre-entry allocation from typed SDK resource enforcement but
left one avoidable residual: rejecting an already-owned, hostile-depth
`serde_json::Value` through a native DID constructor could recursively destroy
the tree and exhaust the thread stack. Bounded wire-slice parsers avoided this
path, but native Rust callers had to pre-bound depth solely to make rejection
safe.

The affected public family spans DID documents, verification methods, services,
resolution and dereferencing metadata/content, document metadata builders,
resolution and dereferencing options, and registration public data. A partial
fix would make narrowing `SDK-LIM-007` misleading.

## Decision

The DID crate owns one private generic rejection guard and one private iterative
JSON dismantler. Every audited native constructor or builder that accepts owned
recursive JSON arms the guard around the complete candidate before the first
fallible validation branch. Success returns that exact candidate without
cleanup. Rejection destructures the complete candidate and sends every raw JSON
root through the shared worklist; per-type adapters contain no traversal logic.

Each constructor family has a 32,768-level hostile-depth regression, including
paths where a non-JSON field or reserved key fails before JSON resource
validation. Existing validation order, public APIs, accepted limits, error
variants, wire representations, and successful allocations remain unchanged.

`SDK-LIM-007` therefore loses only its native DID rejection-cleanup clause.
Pre-entry allocation, caller-budgeted primitive/adapter work, and the named
unbounded compatibility surfaces remain explicit limitations. Bounded slice
parsers remain the correct hostile-byte boundary because cleanup after ownership
cannot retroactively constrain allocation or lexical parsing.

## Consequences

- Native Rust callers do not need a depth cap solely to make rejected owned DID
  JSON stack-safe after the SDK takes ownership.
- A new owned-JSON field is exposed by exhaustive adapter destructuring; a new
  constructor or rejection path must use the same guard and add hostile-depth
  evidence.
- Cleanup uses memory proportional to already-owned breadth for its explicit
  worklist. It does not add a second semantic JSON limit.
- The implementation remains crate-private and introduces no dependency,
  feature, target, wire, storage, release, or downstream-adoption change.

## Alternatives rejected

- Lowering accepted JSON depth would be a compatibility change and would not
  protect validation failures that occur before the depth check.
- Catching stack overflow is neither portable nor a safe recovery mechanism.
- Duplicating traversal in every constructor increases drift and makes complete
  review harder.
- Keeping the caller cleanup obligation after complete coverage would preserve
  a limitation that the SDK no longer has.

## Verification and rollback

Focused all-feature and no-default-feature DID tests, strict Clippy, workspace
gates, compatible Nix evaluation, the input-boundary mutation suite, and a
fresh security/architecture review provide release evidence. Rollback removes
the guard integrations, regressions, inventory wording, and this narrowing
together, then restores the native cleanup clause in `SDK-LIM-007`.
