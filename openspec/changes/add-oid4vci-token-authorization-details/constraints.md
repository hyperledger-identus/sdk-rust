# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/237
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001` and `SDK-ARCH-002` keep generic protocol ownership and external
types inside the correct crate. `SDK-SEC-001` through `SDK-SEC-003` require safe
code, static redacted errors and bounded untrusted inputs. `SDK-COMPAT-001`
through `SDK-COMPAT-005` retain Rust 1.98.1 and honest target evidence.
`SDK-DELIVERY-001`, `SDK-LIM-001`, `SDK-LIM-005`, `SDK-LIM-006` and
`SDK-LIM-007` preserve issue-first delivery, unpublished state and incomplete
product/resource claims.

## Introduced or changed constraints

- The existing Token Response core parse contract does not become stricter.
- Authorization Details validation requires positive caller limits and a
  non-empty array.
- Every recognized entry requires exact `openid_credential`, one bounded
  configuration identifier and one or more bounded unique dataset identifiers.
- Unknown fields/types are traversed only inside existing byte/depth/node
  budgets and are not retained.
- Identifiers and retained response content never enter formatting or errors.

## Introduced or changed limitations

The canonical Token Response limitation narrows only for the new explicit
validated state. Access-token trust, issuer authorization, metadata agreement,
identifier selection, Credential Request construction, outer transport
allocation and the repository-wide inherited-boundary audit remain limited.

## Consumer and product impact

The API is additive and unpublished. Existing callers keep the presence-only
core. No consumer, product, wire producer, stored data or downstream tree is
changed.

## Activation and rollback

Activation occurs only when issue #237's PR passes local and hosted gates and
merges to `develop`. Rollback removes the additive capability atomically; no
migration is required.

## Evidence

Boundary, duplicate, unknown-extension, redaction and compatibility tests plus
factory, full Nix, API review and hosted CI are required before activation.
