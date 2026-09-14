# Constraints and limitations

Impact class: routine
Decision status: not-required
Decision reference: not-required
Constraint blockers: none

## Existing entries affected

- `SDK-ARCH-001/002/003`: presentation ownership remains chain/product-neutral,
  crate-local, cohesive, and inward-only.
- `SDK-COMPAT-002/004/005`: Rust 1.98.1 and the temporary target evidence policy
  remain unchanged.
- `SDK-SEC-001/002`: no unsafe code is added and shared diagnostics remain
  static/redacted.
- `SDK-DELIVERY-001`: #271/#279, ADR 0117, planning receipt, exact review and
  protected-develop delivery govern the slice.
- `SDK-LIM-001/002/003/006/007`: unpublished status, FFI, runtime targets,
  consumers, and resource-bound limitations do not change.

## Introduced or changed constraints

No cross-cutting constraint changes. ADR 0117's two-field private record and
five catalogue groups are slice-level acceptance criteria, not a workspace
standard.

## Introduced or changed limitations

The implementation does not provide retryability, structured metadata, a wire
schema, localization, public introspection, bindings, a shared error framework,
runtime target proof, or downstream adoption. The golden covers only the 48
fieldless variants at the exact base.

## Consumer and product impact

No consumer action or presentation behavior change is expected. All current
constructors, validators, lifecycle logic and public diagnostics remain exact.
No consumer repository is inspected or modified.

## Activation and rollback

Activation is merge of one issue-linked PR to protected `develop` after green
CI and resolved review. A focused source/test revert restores the tuple match.
No data, wire, release, or consumer migration is required.

## Evidence

The exact base, 48-row immutable planning golden, ADR 0117, source inventory,
target-policy boundary and required implementation gates are recorded in this
change. Material constraint activation is neither requested nor permitted.
