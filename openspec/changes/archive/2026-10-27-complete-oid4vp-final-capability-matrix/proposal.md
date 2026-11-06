# Complete the OpenID4VP Final capability matrix

## Why

`identus-oid4vp` now has several independently reviewed protocol slices, but
the repository has no closed artifact that says which OpenID4VP 1.0 Final
wallet responsibilities are implemented, partial, unsupported, or not
applicable. Issue #541 cannot be closed from issue count or README prose.

## What changes

- Add a machine-readable Final capability matrix and offline fail-closed
  validator.
- Publish a human report that reconciles exact evidence and limitations.
- Execute clean-room Final vectors through one reference harness.
- Prove TypeScript- and Swift-shaped adapter inputs can use Rust-native APIs
  without importing donor DTOs or error contracts.
- Assign each missing required capability to focused issues #544 through #547.
- Reconcile the SSI upstream backlog without prematurely marking IDR-024
  delivered.

## Capabilities

### Added capabilities

- `oid4vp-final-conformance`: closed Final traceability, gap ownership, and
  reference execution evidence for the bounded wallet protocol core.

### Modified capabilities

- `ssi-upstream-program`: require machine evidence before IDR-024 completion
  and reconcile the delivered SIOPv2 row.

## Non-goals

No new response mode, trust policy, clock, replay store, client-identifier
resolver, transaction-data semantics, credential-format engine, network I/O,
consumer mutation, publication, certification, SIOPv2, DC API, or HAIP feature
is implemented by this evidence slice.

## Delivery

Issue #541 owns this additive repository-evidence change. Missing protocol
behavior remains visible under #544, #545, #546, and #547.
