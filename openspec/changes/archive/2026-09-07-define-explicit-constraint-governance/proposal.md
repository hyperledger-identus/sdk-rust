## Why

ADR 0062 showed that a technically justified compatibility choice can still
surprise the product sponsor when its consumer consequence is distributed
across ADRs, support-policy files and implementation issues. The SDK needs one
visible constraint and limitation contract that prevents a future target from
being mistaken for an effective promise and makes material impact explicit
before implementation or integration.

## What Changes

- Define constraint and limitation terminology, materiality, authority,
  activation, exception, review and rollback rules in ADR 0063.
- Add a machine-readable index of cross-cutting SDK constraints and known
  limitations, seeded from already accepted repository sources.
- Require every active OpenSpec change to carry `constraints.md` and classify
  its impact as `none`, `routine` or `material`.
- Add an offline checker and factory command that reject incomplete records,
  unresolved material decisions, target/effective confusion and MSRV drift.
- Require pull requests and semantic review to disclose constraint and
  limitation impact without adding routine format approvals.

## Capabilities

### New Capabilities

- `constraint-governance`: Makes current constraints, future targets and known
  unsupported surfaces explicit and machine-checks change-level impact before
  implementation and integration.

### Modified Capabilities

None.

## Impact

This changes repository governance, factory tooling, OpenSpec intake, review
guidance and pull-request metadata. It does not change the effective MSRV,
supported targets, runtime dependencies, Rust APIs, wire behavior, release
state or any downstream repository. Issue #166 records the sponsor direction
for this governance refinement.
