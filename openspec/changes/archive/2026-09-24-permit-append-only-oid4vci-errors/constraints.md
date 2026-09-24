# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/346
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-002`, `SDK-SEC-002`, `SDK-COMPAT-001`, `SDK-DELIVERY-001`, and
`SDK-LIM-001` remain effective. Crate-local ownership, static redaction-safe
errors, additive compatibility and issue-first delivery do not change.

## Introduced or changed constraints

- The 171 v1 variants remain the exact ordered prefix of the live inventory.
- New variants and router mappings append only after that prefix.
- The complete live inventory remains unique and the router remains exhaustive
  and wildcard-free.
- Every appended error requires independent feature-owned contract tests.

## Introduced or changed limitations

The immutable v1 fixture does not characterize future suffix variants. It is
historical compatibility evidence, not a complete forever-current catalogue.

## Consumer and product impact

None. This issue changes no production or public item.

## Activation and rollback

Activation requires issue #346's PR to pass local and hosted gates and merge to
`develop`. Rollback restores the accidental live-inventory ceiling.

## Evidence

Exact fixture hash, all existing OID4VCI tests, focused prefix/uniqueness tests,
factory, signed+DCO and exact-head hosted evidence are mandatory.
