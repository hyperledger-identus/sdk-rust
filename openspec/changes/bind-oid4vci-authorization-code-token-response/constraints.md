# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/360
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001`/`002`, `SDK-SEC-001` through `003`, `SDK-COMPAT-001` through
`005`, `SDK-DELIVERY-001`, and `SDK-LIM-001`, `005`, `006`, `007`, `009`
remain effective. This is chain-neutral bounded OID4VCI protocol behavior; the
Rust 1.98.1, zeroizing ownership, no-I/O, issue-first and honest-target rules
remain unchanged.

## Introduced or changed constraints

- Only `AuthorizationCodeTokenRequest` may bind this response and the request
  is consumed exactly once on every return path.
- Exact status `200` selects success; exact `400` or `401` selects OAuth error;
  every other status fails before header or body parsing.
- Content-Type, Cache-Control and Pragma each have positive independent byte
  limits and strict static validation before the existing bounded body parser.
- Both outcome branches require JSON, a bare `no-store`, and a bare `no-cache`.
- The request's zeroizing secret body is dropped before remote parsing.
- Outcomes retain only public issuer/server/configuration lineage and the exact
  issuer-identification evidence alongside the existing bounded response core.
- Debug, Display and public errors expose no request, header, body, token,
  endpoint, metadata or remote value.
- No dependency, feature, unsafe/native code, network or downstream behavior
  is introduced.

## Introduced or changed limitations

The outcome proves bounded status/header/body syntax and request lineage only.
It does not prove response origin, TLS, server/issuer trust, authorization,
token validity/freshness, cache compliance outside the observed fields,
retryability, DPoP, Authorization Details semantics, storage or issuance.
`NotAdvertised` remains an explicit lack of RFC 9207 mix-up evidence.

## Consumer and product impact

Additive unpublished Rust API only. Consumers can replace local status/header
classification and keep success/error data attached to the originating public
client request. No stored data, migration, target, product, chain,
credential-format or release promise changes.

## Activation and rollback

Activation requires issue #360's PR to pass local and hosted gates and merge
to `develop`. Rollback removes the additive response envelope, limits,
diagnostics, tests, ADR and private request decomposition without changing
existing request or response-core behavior.

## Evidence

Exact status branches, JSON/cache header grammar and limits, success/error
shape mismatch, body bounds, one-shot ownership, early secret erasure, retained
lineage/evidence, redaction, stable error contract, portable compilation,
factory, Nix, signed+DCO and exact-head hosted CI evidence are mandatory.
