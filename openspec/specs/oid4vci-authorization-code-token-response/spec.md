# oid4vci-authorization-code-token-response Specification

## Purpose
TBD - created by archiving change bind-oid4vci-authorization-code-token-response. Update Purpose after archive.
## Requirements
### Requirement: One request binds one exclusive Token Endpoint outcome

The SDK SHALL consume exactly one `AuthorizationCodeTokenRequest` to bind one
caller-supplied final Token Endpoint HTTP response. Exact status `200` SHALL
select `TokenResponseCore`; exact status `400` or `401` SHALL select
`TokenErrorResponseCore`; every other status SHALL fail before any header or
body semantic is inspected. A `401` response SHALL be accepted only when the
parsed error code is exact `invalid_client`.

The result SHALL be exactly one request-bound success or OAuth error outcome.
Body shape SHALL NOT override the status-selected branch, and the transition
SHALL perform no HTTP.

#### Scenario: exact success is request bound

- **WHEN** a request receives status `200`, valid required headers and a valid
  bounded successful Token Response body
- **THEN** it returns the success branch with the existing parsed core

#### Scenario: OAuth errors are request bound

- **WHEN** a request receives status `400` or `401`, valid required headers and
  a valid bounded Token Error Response body
- **THEN** it returns the error branch with the existing parsed core

#### Scenario: status wins over JSON shape

- **WHEN** a `200` carries an error object or a `400`/`401` carries a success
  object
- **THEN** the status-selected parser rejects the mismatched body

#### Scenario: 401 remains specific to invalid client

- **WHEN** status `401` carries a valid Token Error Response whose exact error
  code is not `invalid_client`
- **THEN** binding fails with a static status/error mismatch diagnostic

### Requirement: HTTP status and headers are strict and independently bounded

`AuthorizationCodeTokenHttpResponseLimits` SHALL combine existing positive
success/error body limits with independent positive Content-Type,
Cache-Control and Pragma byte caps. Header defaults SHALL each be 1,024 bytes.

After valid status, the SDK SHALL validate in this exact order: Content-Type
bound and `application/json` grammar; Cache-Control bound, syntax and bare
`no-store`; Pragma bound, syntax and bare `no-cache`; then body parsing. Type,
subtype and directive names SHALL be ASCII case-insensitive. Valid media-type
parameters and additional valid cache directives SHALL interoperate. Missing,
malformed, argument-bearing required directives and duplicate media-type
parameter names SHALL fail closed.

#### Scenario: exact header bounds pass

- **WHEN** each effective header value equals its configured maximum and has
  valid required semantics
- **THEN** validation reaches the selected bounded body parser

#### Scenario: an oversized or invalid header short-circuits the body

- **WHEN** a header exceeds its limit or violates its required grammar
- **THEN** its static diagnostic is returned without inspecting body semantics

#### Scenario: invalid status short-circuits headers

- **WHEN** status is outside exact `200`, `400` and `401`
- **THEN** the status diagnostic is returned even if all supplied headers and
  body are malformed or oversized

### Requirement: Request secrets are erased and least-authority lineage survives

The consuming transition SHALL drop the zeroizing request form body before
validating or copying remote header/body content. Success and error outcomes
SHALL retain the exact validated Credential Issuer Metadata, selected
Authorization Server Metadata, selected Credential Configuration and unchanged
`AuthorizationResponseIssuerIdentification`, but no endpoint replacement,
request body, code, verifier, redirect, client ID or reusable request state.

Outcome Debug SHALL expose only branch-safe structural metadata and issuer
evidence. It SHALL NOT expose request, endpoint, issuer/server/configuration,
header, body, token, scope, description, URI or extension values.

#### Scenario: lineage and RFC 9207 evidence remain exact

- **WHEN** either response branch is returned
- **THEN** its public lineage queries match the consumed request and
  `VerifiedRfc9207` or `NotAdvertised` is unchanged

#### Scenario: request cannot be reused

- **WHEN** response binding begins and later status, header or body validation
  fails
- **THEN** the consumed typed request and its code/verifier body are not
  returned for another exchange attempt

### Requirement: Authority and compatibility remain narrow

The capability SHALL reuse the existing response cores and private HTTP field
parser. It SHALL add no dependency, feature, lockfile, unsafe/native, network,
storage, consumer, chain, product, release or publication behavior.

It SHALL NOT claim response origin, TLS, endpoint reachability, server or
issuer trust, authorization, token validity/freshness, cache compliance beyond
the supplied field syntax, retryability, client authentication, DPoP,
Authorization Details semantics, Credential Request readiness or successful
issuance.

#### Scenario: binding is transport neutral

- **WHEN** a response is bound
- **THEN** no DNS, TLS, HTTP, clock, entropy, persistence, retry or trust-policy
  effect occurs

### Requirement: Successful binding has one typed consuming correlation path

`RequestBoundAuthorizationCodeTokenResponse` SHALL expose a consuming
Authorization Details correlation transition under explicit positive limits.
The transition SHALL move the exact #360 lineage and Token Response core; it
SHALL NOT accept replacement metadata, configuration, response JSON or token
values. The OAuth error branch SHALL NOT expose this transition.

#### Scenario: only success can correlate

- **WHEN** callers hold the exclusive success or error outcome branch
- **THEN** only the success type can enter credential Authorization Details
  correlation

#### Scenario: failed correlation consumes the success

- **WHEN** response-local or lineage correlation fails
- **THEN** no reusable request-bound success or Token Response core is returned
