# Constraint readiness

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/207
Constraint blockers: none

## Existing entries affected

`SDK-SEC-003` requires explicit resource limits and redaction-safe errors at
changed input boundaries. `SDK-LIM-007` remains effective but narrows its
inherited-gap evidence after `DidMethod` and borrowed validated-string parsing
are bounded. `SDK-DELIVERY-001` requires the issue-linked specification,
distinct review and CI evidence. `SDK-COMPAT-001` through `SDK-COMPAT-005`,
`SDK-ARCH-001`, `SDK-ARCH-002`, `SDK-SEC-001` and `SDK-SEC-002` remain unchanged.

## Introduced or changed constraints

- A validated `String` newtype validator SHALL accept borrowed `&str` for the
  generated `parse` and `FromStr` paths.
- Borrowed validated-string parsing SHALL validate before allocating its owned
  success value.
- Standalone DID method names SHALL contain at most 2,042 UTF-8 bytes and SHALL
  be checked for length before ASCII grammar traversal.
- Errors produced by DID method validation SHALL contain no rejected input.

No compiler, dependency, feature, target, release, chain or product constraint
changes.

## Introduced or changed limitations

- Owned `TryFrom<String>` and serde necessarily receive already-allocated
  values; an outer transport/deserializer limit remains required.
- The generic DID resource ceiling is SDK policy, not a W3C maximum.
- External pre-release macro users with `fn(&String)` validators must migrate
  to `fn(&str)`.

## Consumer and product impact

Current workspace consumers preserve accepted values and APIs. Invalid
borrowed inputs are rejected before cloning and over-limit method errors become
static. Midnight, Cardano, wallet and method-specific policies stay downstream.

## Activation and rollback

The constraint activates only after issue #207's PR passes local and hosted
gates and merges into `develop`. Reverting the focused PR restores prior
behavior; there is no wire or data migration.

## Evidence

Issue #207, this change contract, ADR 0093, exact/one-over and redaction tests,
factory constraint checks, distinct review and hosted CI provide activation
evidence. Issue #168 remains the parent repository-wide audit.
