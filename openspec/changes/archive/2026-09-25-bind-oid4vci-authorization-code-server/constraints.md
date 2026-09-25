# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/350
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001`/`002`, `SDK-SEC-001` through `003`, `SDK-COMPAT-001` through
`005`, `SDK-DELIVERY-001`, and `SDK-LIM-001`, `005`, `006`, `007`, `009`
remain effective. Generic OID4VCI state transitions belong in sdk-rust;
bounded inputs, explicit sensitive ownership, Rust 1.98.1, issue-first delivery
and honest target claims do not change.

## Introduced or changed constraints

- The transition consumes existing matched offer and bounded server metadata
  objects and performs no parsing or I/O.
- Authorization Code grant presence takes precedence over server capability
  checks.
- The selected metadata issuer must be an effective Authorization Server and
  must exactly match any grant-level hint.
- Effective grant support follows the existing RFC 8414 default; an explicit
  list must contain exact `authorization_code`.
- A validated Authorization Endpoint is required; a Token Endpoint is not.
- The result retains optional `issuer_state` only through its existing
  zeroizing owner and exposes remote data only through existing borrowed APIs.
- New diagnostics are static, redacted and appended to the error contract.

## Introduced or changed limitations

No effective limitation is removed. The state does not prove metadata
provenance, server identity/control/trust, endpoint reachability, client or
redirect eligibility, PKCE/CSRF safety, callback issuer binding, authorization,
token exchange, persistence or successful issuance.

## Consumer and product impact

Additive unpublished API only. Consumers can delete server-selection glue and
receive a stable predecessor for later Authorization Request construction. No
wire, stored data, migration, product, chain or credential-format behavior
changes.

## Activation and rollback

Activation requires issue #350's PR to pass local and hosted gates and merge
to `develop`. Rollback removes the additive state, method, diagnostics, tests,
ADR and capability while preserving all existing offer and metadata APIs.

## Evidence

Grant absence, advertisement mismatch, hint mismatch, explicit and defaulted
grant support, missing endpoint, success ownership, issuer-state preservation,
redaction, stable error-contract, compatibility, factory, Nix, signed+DCO and
exact-head hosted CI evidence is mandatory.
