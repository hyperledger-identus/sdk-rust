# Constraints and limitations

Impact class: routine
Decision status: not-required
Decision reference: not-required
Constraint blockers: none

## Existing entries affected

- `SDK-ARCH-001/002/003`: JOSE ownership remains chain/product-neutral,
  crate-local, cohesive, and inward-only.
- `SDK-COMPAT-002/004/005`: Rust 1.98.1 and target evidence policy remain.
- `SDK-SEC-001/002`: no unsafe code is added; diagnostics remain static/redacted.
- `SDK-DELIVERY-001`: #271/#280, ADR 0118, receipt, review and protected
  `develop` delivery govern the slice.
- `SDK-LIM-001/002/003/006/007`: unpublished, FFI, runtime-target, consumer,
  and resource-bound limitations do not change.

## Introduced or changed constraints

No cross-cutting constraint changes. ADR 0118's three-field record and four
groups are slice acceptance criteria, not a workspace standard.

## Introduced or changed limitations

No new algorithms, encryption, MAC, key agreement, retryability, structured
metadata, localization, wire schema, bindings, runtime target proof, or
downstream adoption is supplied. The golden covers only the exact 51 fieldless
variants at the assessed base.

## Consumer and product impact

No consumer action or JOSE/OID4VCI behavior change is expected. No consumer
repository is inspected or modified.

## Activation and rollback

Activation is merge of one issue-linked PR to protected `develop` after green
CI and resolved review. A focused source/test revert restores both explicit
matches. No data, wire, release, or consumer migration is required.

## Evidence

The exact base, immutable golden, ADR 0118, source inventory, target-policy
boundary and implementation gates are recorded here. No material constraint
activation is requested or permitted.
