# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/239
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001`/`002`, `SDK-SEC-001` through `003`, `SDK-COMPAT-001` through
`005`, `SDK-DELIVERY-001`, and `SDK-LIM-001`, `005`, `006`, `007` remain
effective. The protocol crate owns this generic transition; bounds, redaction,
Rust 1.98.1, issue-first delivery and honest support claims do not change.

## Introduced or changed constraints

- The new constructor accepts only `TokenResponseWithAuthorizationDetails`.
- Both selection indices are checked before token/proof/body construction.
- The selected authorization configuration must equal an offered configuration.
- The body contains exactly `credential_identifier`, never both Final selectors.
- Existing Bearer/proof/Authorization/body bounds and redaction are reused.
- The existing configuration-ID constructor remains source/wire compatible.

## Introduced or changed limitations

No existing effective limitation is removed. The new state does not establish
token/issuer/identifier trust, product selection policy, replay protection,
HTTP execution, downstream adoption or release support.

## Consumer and product impact

Additive unpublished API only. No existing caller, wire producer, stored data,
product policy or downstream repository changes.

## Activation and rollback

Activation requires issue #239's PR to pass local and hosted gates and merge to
`develop`. Rollback removes the sibling constructor/errors/tests/spec without
migrating the unchanged configuration-ID branch.

## Evidence

Exact wire, selector, mismatch, escaping, bound, redaction, compatibility,
factory, Nix and hosted review/CI evidence are mandatory.
