# Constraints and limitations

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/609
Constraint blockers: none

## Existing entries affected

- `SDK-ARCH-001`: canonical portable semantics remain Rust-owned.
- `SDK-LIM-002`: the TypeScript package remains private and unpublished.
- `SDK-LIM-009`: isolated TypeScript target evidence remains independent from
  the Rust core result.

## Introduced or changed constraints

The rehearsal is bound to exact SDK-Rust and SDK-TS revisions. One assertion
body must run against both provider routes wherever behavior is applicable.
Provider choice is terminal before initialization; a failure never activates
the other route. Platform-owned pairwise effects remain excluded.

## Introduced or changed limitations

The repository-local consumer fixture proves composition and rollback of the
private facade, not external SDK-TS adoption, default activation, support,
publication, budget acceptance, React Native, DIDComm processing, credentials,
or mdoc.

## Consumer and product impact

No external consumer behavior changes. TS1 evidence gains one pinned composed
rehearsal while the legacy SDK-TS route remains unchanged and selectable.

## Activation and rollback

Activation requires issue-bound planning, focused local evidence, exact target
receipts, distinct review, signed+DCO commits, green CI, and protected merge.
Rollback removes the fixture and evidence records without changing any Rust
semantic core or external SDK-TS behavior.

## Evidence

Issue #609, the pinned donor ledgers, current provider-selection contract,
canonical vectors, target runners, parity matrix, and consumer-change ledger
are the atomic evidence owners.
