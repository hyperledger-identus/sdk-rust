# oid4vci-authorization-code-token-request Specification

## Purpose
TBD - created by archiving change construct-oid4vci-authorization-code-token-request. Update Purpose after archive.
## Requirements
### Requirement: A correlated success constructs one public-client Token Request

The SDK SHALL consume exactly one `CorrelatedAuthorizationCode` to construct
an `AuthorizationCodeTokenRequest` for an unauthenticated public client. The
request SHALL contain exactly these form fields in this order:
`grant_type=authorization_code`, exact `code`, exact `redirect_uri`, exact
`client_id`, and exact `code_verifier`.

Construction SHALL use only values retained by the correlated Authorization
Request lineage. It SHALL NOT accept caller replacements, add client
credentials or authentication assertions, or perform HTTP.

#### Scenario: exact retained values become a deterministic form body

- **WHEN** a correlated success has a selected server Token Endpoint and all
  exact output fits the supplied limits
- **THEN** the transition returns the fixed five-field encoded form body for
  the retained unauthenticated public client

#### Scenario: encoded punctuation is exact

- **WHEN** retained code, redirect or client values require form encoding
- **THEN** the output follows the existing form codec and exact fixed order
  without normalization or lossy conversion

### Requirement: Endpoint and body limits fail before output

`AuthorizationCodeTokenRequestLimits` SHALL require positive independent
maximum Token Endpoint and complete form-body byte counts. Construction SHALL
reject a missing Token Endpoint, an endpoint above its request-specific cap,
checked-size overflow, or a body above its cap before returning a request.

#### Scenario: absent selected Token Endpoint is rejected

- **WHEN** the correlated lineage's selected server has no Token Endpoint
- **THEN** construction returns the existing static Token Endpoint required
  diagnostic

#### Scenario: each exact maximum is accepted

- **WHEN** the endpoint and form body each equal their respective maximum
- **THEN** construction succeeds

#### Scenario: either maximum plus one is rejected

- **WHEN** the endpoint or complete encoded form body exceeds its respective
  maximum by one byte
- **THEN** construction returns the matching static oversized diagnostic

### Requirement: The request owns least-authority lineage and secrets

After construction, the request SHALL retain the validated Credential Issuer
Metadata, selected Authorization Server Metadata, selected Credential
Configuration, and unchanged `AuthorizationResponseIssuerIdentification`. It
SHALL discard the consumed Credential Offer and retain no separately
accessible code, verifier, state, redirect URI, client
identifier or Authorization Request URI outside the encoded zeroizing body.

The request SHALL expose the sensitive body only through an explicitly named
borrowed accessor for immediate transport. Debug, Display and public errors
SHALL expose none of the body, code, verifier, redirect, client, endpoint or
remote lineage values.

#### Scenario: RFC 9207 evidence is preserved without upgrade

- **WHEN** a correlated success reports `VerifiedRfc9207` or `NotAdvertised`
- **THEN** the Token Request reports that exact same evidence value

#### Scenario: Debug is redacted

- **WHEN** the request is formatted with Debug
- **THEN** it exposes bounded structural metadata but none of its remote or
  sensitive values

### Requirement: Compatibility and authority remain narrow

The request SHALL reuse the existing private form codec and existing POST and
form-media-type constants. It SHALL add no dependency, feature, lockfile,
unsafe/native, network, storage, consumer, chain or product behavior.

It SHALL NOT claim confidential-client support, Token Endpoint authentication,
Authorization Details narrowing, DPoP, endpoint reachability, server trust,
mix-up protection when RFC 9207 is not advertised, authorization, token
provenance or successful issuance.

#### Scenario: request construction is transport neutral

- **WHEN** the request is constructed
- **THEN** no DNS, TLS, HTTP, clock, entropy, persistence or retry effect occurs
