# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/358
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001`/`002`, `SDK-SEC-001` through `003`, `SDK-COMPAT-001` through
`005`, `SDK-DELIVERY-001`, and `SDK-LIM-001`, `005`, `006`, `007`, `009`
remain effective. This is chain-neutral bounded OID4VCI protocol behavior; the
Rust 1.98.1, zeroizing ownership, no-I/O, issue-first and honest-target rules
remain unchanged.

## Introduced or changed constraints

- Only `CorrelatedAuthorizationCode` may construct the request and it is
  consumed once.
- This slice implements an unauthenticated public-client profile and always
  emits the exact retained `client_id`.
- Exact retained code, redirect URI and PKCE verifier are emitted with exact
  `grant_type=authorization_code` in a deterministic fixed order.
- The selected server must expose a Token Endpoint and the complete endpoint
  and encoded body must each satisfy positive request-specific limits.
- The result retains non-secret server/offer/configuration lineage and exact
  issuer-identification evidence, but no separately reusable code or verifier.
- Debug, Display and public errors remain static and redact the body, code,
  verifier, redirect, client, endpoint and remote values.
- No dependency, feature, unsafe/native code, network or downstream behavior
  is introduced.

## Introduced or changed limitations

The request does not support confidential/authenticated client methods,
Authorization Details narrowing, DPoP, HTTP, retry, response binding, token
trust or storage. `NotAdvertised` remains an explicit lack of RFC 9207 issuer
evidence and is not upgraded into a mix-up-protection claim.

## Consumer and product impact

Additive unpublished Rust API only. Consumers can delete local form
construction for the supported public-client profile. No wire parser, stored
data, migration, target, product, chain, credential-format or release promise
changes.

## Activation and rollback

Activation requires issue #358's PR to pass local and hosted gates and merge
to `develop`. Rollback removes the additive module, internal consuming
decomposition, diagnostics, tests, ADR and capability without changing any
existing request/response behavior.

## Evidence

Normative and exact-byte vectors, endpoint/body limit boundaries, missing
endpoint, one-shot ownership, retained lineage/evidence, redaction, stable
error-contract, portable compilation, factory, Nix, signed+DCO and exact-head
hosted CI evidence are mandatory.
