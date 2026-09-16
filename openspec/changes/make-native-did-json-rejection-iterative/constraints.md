# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/297
Constraint blockers: none

## Existing entries affected

- `SDK-SEC-003` requires explicit resource ownership for the changed hostile
  input rejection path.
- `SDK-LIM-007` remains effective but loses only native DID owned-JSON
  rejection cleanup from its summary, scope, rationale, consumer impact, and
  enforcement evidence.
- Outer preallocation, caller-budgeted work, and the Multihash, crypto-codec,
  and JOSE retained-input compatibility residuals remain unchanged.

## Introduced or changed constraints

After a native DID constructor takes ownership of recursive JSON, every
validation `Err` SHALL dismantle that owned tree without recursive destruction.
The SDK does not claim to bound or reverse caller/deserializer allocation that
occurred before entry.

## Introduced or changed limitations

No new limitation is introduced. Removing the named cleanup clause does not
remove `SDK-LIM-007` or imply end-to-end hostile-input safety before typed SDK
entry. Breadth already allocated by the caller can require a proportional
iterative worklist during cleanup.

## Consumer and product impact

Native Rust consumers no longer need a pre-entry depth bound solely to make a
rejected DID JSON value safe to destroy. They still own preallocation and must
use bounded wire-slice parsers at untrusted byte boundaries. No product,
chain, release, certification, or downstream adoption outcome changes.

## Activation and rollback

Activation requires complete hostile-depth constructor-family evidence,
focused and workspace gates, signed PR, green exact-head CI, and merge to
`develop`. Rollback restores the former cleanup clause atomically with removal
of the guard and regressions; accepted values and wire data require no
migration.

## Evidence

Issue #297 and ADR 0125 provide exact material authority. The OpenSpec delta,
constructor inventory, tests, updated machine inventory/limitation index, and
review receipt provide implementation evidence.
