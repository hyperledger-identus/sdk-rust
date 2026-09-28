# Decompose DID registration lifecycle responsibilities

## Why

`crates/did/src/registration.rs` combines 1,163 authored nonblank lines across
public extension-data validation, mutation requests, asynchronous lifecycle
states, result validation, and the registrar port. Discussion #399 and issue
#406 identify it as a remaining domain hotspot. The public contract is already
characterized, so this slice improves private ownership without redesigning DID
registration.

## What changes

- Keep `registration.rs` as a private facade and preserve every crate-root export.
- Isolate bounded public JSON data and hostile-value cleanup.
- Isolate request, job, action, secret-policy, and document-operation models.
- Isolate lifecycle results and their method/job/document invariants.
- Isolate the runtime-neutral future and object-safe registrar port.
- Preserve validation order, limits, redaction, cleanup, errors, and public API.

## Capability

### Added capability

- `did-registration-module-ownership`: cohesive private ownership and stable
  compatibility boundaries for chain-neutral DID registration.

## Non-goals

No DID method, chain adapter, custody, persistence, HTTP representation,
executor, cancellation, finality, wire-format, error-taxonomy, dependency,
feature, or target behavior changes.

## Delivery

Issue #406 owns this slice. Planning and exact-head preflight precede code
moves. DID characterization, public/error/source contracts, portable targets,
code-health evidence, and protected exact-head CI prove compatibility.
