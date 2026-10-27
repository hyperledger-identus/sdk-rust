# Upgrade the RustCrypto security stack

## Why

Current RustSec evidence rejects `crypto-bigint 0.5.5` for timing variability
and Tokio 1.53.1 for an I/O-safety defect. The first is reached through the
workspace `k256`/`p256` 0.13 stack; the second is used by the HTTP resolver test
runtime. The findings now block every honest audit and the bounded fuzz lanes.

## What changes

- Upgrade `k256` and `p256` to 0.14 and accept the corresponding RustCrypto
  trait/encoding migration only behind the existing `identus-crypto` facade.
- Resolve Tokio to 1.54.1 or newer within the accepted 1.x contract.
- Preserve every public SDK type, byte encoding, signature behavior, JVM
  compatibility fallback, feature boundary, target claim, and test vector.
- Require clean root and standalone-fuzz `cargo audit --deny warnings` evidence.

## Capability

### Modified capability

- `crypto`: make an advisory-clean RustCrypto dependency closure an explicit
  maintained invariant.

## Non-goals

No new algorithm, API redesign, RustSec suppression, target expansion, Tokio
runtime ownership, downstream mutation, publication, or unrelated dependency
refresh belongs to this change.

## Delivery

Issue #525 owns this prerequisite security correction. It lands independently
before issue #522 is replayed and its Linux fuzz receipt is rerun.
