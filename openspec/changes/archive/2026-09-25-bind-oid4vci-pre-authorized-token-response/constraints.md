# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/375
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001`/`002`, `SDK-SEC-001` through `003`, `SDK-COMPAT-001` through
`005`, `SDK-DELIVERY-001`, and `SDK-LIM-001`, `005`, `006`, `007`, `009`
remain effective. This is chain-neutral bounded OID4VCI protocol behavior; the
Rust 1.98.1, zeroizing ownership, no-I/O, issue-first and honest-target rules
remain unchanged.

## Introduced or changed constraints

- Only `PreAuthorizedTokenRequest` binds the response and is consumed once.
- The request retains exact public issuer/server metadata and ordered offered
  configuration IDs, but not Credential Offer JSON or grant/input secrets.
- Exact `200` selects success and exact `400` selects OAuth error; every other
  status fails before header or body semantics.
- Positive Content-Type, Cache-Control and Pragma limits precede strict JSON,
  bare `no-store`, bare `no-cache`, then bounded body parsing.
- The zeroizing form body is dropped before remote response validation.
- No dependency, feature, unsafe/native code, network or downstream behavior
  is introduced.

## Introduced or changed limitations

The outcome does not prove HTTP origin, TLS, server/issuer trust,
authorization, token validity/freshness, external cache compliance,
retryability, client authentication, DPoP, credential-request readiness,
storage or issuance.

## Consumer and product impact

This is additive unpublished Rust API. Consumers can replace local response
classification without adopting SDK transport, storage or trust policy. No
stored data, migration, target, chain, product or release promise changes.

## Activation and rollback

Activation requires issue #375's exact-head PR to pass local and hosted gates
and merge to `develop`. Rollback removes the additive response module, limits,
diagnostics, tests, ADR and request-lineage fields without changing existing
core parsers.

## Evidence

Mandatory evidence covers status/header/body precedence,
inclusive bounds, lineage, secret erasure, redaction, stable errors, portable
compilation, factory/Nix gates, signed+DCO history and hosted CI.
