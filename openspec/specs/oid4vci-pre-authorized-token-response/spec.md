# oid4vci-pre-authorized-token-response Specification

## Purpose
TBD - created by archiving change bind-oid4vci-pre-authorized-token-response. Update Purpose after archive.
## Requirements
### Requirement: One request binds one exclusive Token Endpoint outcome

The SDK SHALL consume exactly one `PreAuthorizedTokenRequest` to bind a
caller-supplied final Token Endpoint response. Exact status `200` SHALL select
`TokenResponseCore`, exact status `400` SHALL select `TokenErrorResponseCore`,
and every other status SHALL fail before header or body semantics. Body shape
SHALL NOT override the status-selected branch and the transition SHALL perform
no HTTP.

#### Scenario: exact success and error are request bound

- **WHEN** exact `200` or `400` carries valid required headers and the matching
  bounded response body
- **THEN** the exclusive success or error branch retains the request lineage

#### Scenario: status wins over JSON shape

- **WHEN** `200` carries an error body, `400` carries a success body, or any
  unsupported status carries otherwise valid input
- **THEN** the status-selected static or core parser error is returned

### Requirement: Headers and bodies are strictly bounded

`PreAuthorizedTokenHttpResponseLimits` SHALL combine existing success/error
body limits with positive Content-Type, Cache-Control and Pragma caps,
defaulting each header cap to 1,024 bytes. Validation order SHALL be status,
Content-Type bound and JSON grammar, Cache-Control bound and bare `no-store`,
Pragma bound and bare `no-cache`, then the selected body parser.

#### Scenario: inclusive limits interoperate

- **WHEN** each supplied value equals its positive limit and is semantically valid
- **THEN** binding succeeds through the selected existing core

#### Scenario: earlier metadata failure short-circuits later input

- **WHEN** status or one ordered header is invalid
- **THEN** its static diagnostic is returned without parsing later semantics

### Requirement: Only least-authority public lineage survives

The request and both outcomes SHALL retain exact validated Credential Issuer
Metadata, selected Authorization Server Metadata, and the ordered offered
Credential Configuration IDs. They SHALL NOT retain Credential Offer JSON,
the Pre-Authorized Code, Transaction Code or reusable request state. Response
binding SHALL drop the zeroizing form before remote validation. Debug and
public errors SHALL reveal no issuer, server, configuration, endpoint, header,
body, grant, transaction code or token value.

#### Scenario: exact offer scope remains queryable

- **WHEN** either response branch is returned
- **THEN** its lineage exposes the same ordered configuration IDs and matched
  issuer/server metadata that produced the consumed request

#### Scenario: invalid response cannot recover request authority

- **WHEN** status, header or body validation fails
- **THEN** the consumed request and its secrets are not returned

### Requirement: Authority remains narrow and additive

The capability SHALL reuse the existing response cores and private HTTP field
grammar and SHALL add no dependency, feature, unsafe/native code, transport,
trust, storage, retry, consumer, chain, release or product behavior.

#### Scenario: response metadata is caller supplied

- **WHEN** a response is bound
- **THEN** the result makes no claim about HTTP origin, TLS, token trust or
  external cache behavior
