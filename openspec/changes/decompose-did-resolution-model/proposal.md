# Decompose DID resolution model and wire responsibilities

## Why

`crates/did/src/resolution.rs` contains 1,345 authored nonblank lines and owns
five distinct change axes: scalar value syntax, standard problem metadata,
document metadata, resolution result envelopes, and dereferencing content plus
raw-wire preflight. Discussion #399 and issue #405 identify this as the largest
remaining SDK domain hotspot. The current public contract is mature and well
tested, so this slice improves ownership without redesigning DID resolution.

## What changes

- Keep `resolution.rs` as a private facade and preserve every crate-root export.
- Move validated scalar types and their parsers to one private value module.
- Move standard error and operation metadata models to one private operation
  metadata module.
- Move DID document metadata and its builder to one private module.
- Separate resolution and dereferencing result envelopes by lifecycle.
- Isolate duplicate-name and bounded raw-JSON preflight in one private wire
  module.
- Remove only mechanically duplicated documentation/serde annotations and the
  second identical pure validation call in dereferencing construction.
- Preserve errors, validation ordering, cleanup behavior, cache/cancellation
  contracts, object safety, runtime neutrality, public paths, and wire forms.

## Capability

### Added capability

- `did-resolution-module-ownership`: records cohesive private ownership and
  compatibility invariants for DID resolution and dereferencing result data.

## Non-goals

This change does not implement or alter resolver dispatch, cache single-flight,
network transport, executor policy, DID method behavior, W3C wire semantics,
limits, public API, error taxonomy, dependencies, features, or targets. Issue
#50 remains the sole owner of any future request-coalescing decision.

## Delivery

Issue #405 owns this slice. Planning and exact-head preflight precede code
moves. The complete DID test corpus, public API inventory, error goldens,
source-distribution checks, portable target gates, code-health evidence, and
protected exact-head CI prove compatibility.
