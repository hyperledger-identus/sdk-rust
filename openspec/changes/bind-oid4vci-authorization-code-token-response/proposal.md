# Bind the OID4VCI Authorization Code Token Response

## Why

Issue #358 constructs a bounded Authorization Code Token Request and retains
the exact selected Authorization Server, Credential Issuer, configuration and
issuer-identification evidence. Consumers still have to classify HTTP status,
validate response headers, select the success or OAuth error parser and keep
that result attached to the request that produced it. Reimplementing that seam
can parse an error as success, accept a response from an unsupported status,
lose lineage, retain the authorization code and verifier after exchange, or
process unbounded headers and bodies.

## What changes

- Consume one `AuthorizationCodeTokenRequest` exactly once when binding a
  caller-supplied final Token Endpoint response.
- Classify exact HTTP `200` as success and exact `400` or `401` as OAuth error
  before any body parsing; require a parsed `401` error to be exact
  `invalid_client`.
- Require bounded `application/json`, `Cache-Control` containing a bare
  `no-store`, and `Pragma` containing a bare `no-cache`.
- Parse the body with the existing bounded `TokenResponseCore` or
  `TokenErrorResponseCore` parser selected by the status class.
- Return an exclusive success/error outcome retaining the exact public
  request lineage and unchanged issuer-identification evidence.
- Erase the request form body before remote body parsing and keep every Debug,
  Display and public error surface free of request, token and remote values.
- Add static diagnostics, exact status/header/body precedence evidence, ADR
  0145, architecture evidence and a focused successor.

## What does not change

No HTTP execution, redirect/decompression policy, origin or TLS proof, client
authentication, DPoP, retry, token cryptographic validation, token storage,
Authorization Details policy, Credential Request construction, issuer trust,
consumer, chain, product, release or publication behavior is added.

## Capabilities

### New capabilities

- `oid4vci-authorization-code-token-response`: bounded, one-shot,
  request-bound success/error Token Endpoint response classification.

### Modified capabilities

- `oid4vci-authorization-code-token-request`: make response binding the only
  typed consuming continuation and document early request-secret erasure.
- `oid4vci-error-contracts`: append static HTTP response-limit and validation
  diagnostics without changing prior entries.
- `ssi-upstream-program`: retain IDR-023 in progress under the next focused
  child after #360 integrates.

## Authority

Issue #360, OpenID4VCI 1.0 Final sections 6.2 and 6.3, RFC 6749 sections 5.1
and 5.2, RFC 9110 sections 8.3 and 15, and the standing SDK delivery mandate.
