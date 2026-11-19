# Constraints and limitations

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/607
Constraint blockers: none

## Existing entries affected

- `SDK-ARCH-001`: Rust remains the semantic owner; measurements do not change
  the TypeScript adapter boundary.
- `SDK-LIM-002`: the package remains private and unpublished.
- `SDK-LIM-009`: isolated TypeScript evidence remains independent from Rust
  core integration evidence.

## Introduced or changed constraints

Comparative artifact results must use the same file inventory, toolchain,
compression algorithm/settings, host, and aggregation for both revisions.
Cold initialization must include module import in an isolated process; warm
initialization must use the cached import path. Provider construction and peer
operation latency remain separate fields.

## Introduced or changed limitations

The benchmark applies only to the private package and Node runtime. Browser
and React receipts remain functional evidence only. Directional measurements
do not establish an accepted regression threshold, supported runtime, external
adoption, publication, or stable API.

## Consumer and product impact

No consumer behavior changes. Existing private browser, Node.js, and React web
functional receipts stay valid, while their roadmap states are corrected to
show that adoption and product gates remain open.

## Activation and rollback

Activation requires issue-bound planning, exact-revision local measurement,
registry and parity validation, full local gates, distinct review,
signed+DCO commits, green CI, and protected merge. Rollback removes only the
measurement/reporting changes and evidence labels.

## Evidence

Issue #607 supplies directed authority. PR #604's immutable base/candidate
revisions, the assembly manifest, measurement harness, quality declaration,
peer quality record, and parity ledger are the atomic evidence owners.
