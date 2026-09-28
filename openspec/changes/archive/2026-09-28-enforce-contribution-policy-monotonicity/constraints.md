# Constraint readiness

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/339
Constraint blockers: none

## Existing entries affected

ADR 0166, the base-owned pull-request policy requirement, and contribution
policy version 1 remain authoritative. Issue #341's trust root is a prerequisite
and is complete.

## Introduced or changed constraints

- The exact base policy and proposed exact-head policy are parsed by base-owned
  code under one closed schema and bounded JSON/resource policy.
- Equality and strict strengthening pass; any relaxation, ambiguity, unknown
  field, duplicate, invalid type, deletion, oversize blob, or unsupported
  version fails closed.
- Proposed allowlists and exemptions may only shrink; enforcement booleans may
  only strengthen; numeric ceilings may only decrease.
- The proposed tree is data only and SHALL NOT be checked out or executed.
- There is no self-authorizing waiver in this slice.

## Introduced or changed limitations

The partial order covers only declared version-1 policy semantics. Descriptive
branch arrays must remain set-equivalent. An intentional future relaxation
needs a separately bootstrapped base-owned approval mechanism.

## Consumer and product impact

No SDK, SSI, cryptographic, target, wire, storage, package, or consumer behavior
changes. Maintainers gain deterministic diagnostics before policy weakening can
reach `develop`.

## Activation and rollback

Activation requires issue-bound planning, focused mutation tests, workflow
lint, full factory gates, signed/DCO review, natural base-owned CI, and protected
merge. Rollback is a repository revert.

## Evidence

Every version-1 field class has equality, strengthening, and relaxation
evidence where meaningful. Malformed/duplicate/oversize/deleted head policy,
exact-head binding, no-head execution, actionlint, factory, and hosted evidence
are required.
