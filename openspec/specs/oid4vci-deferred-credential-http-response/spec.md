# oid4vci-deferred-credential-http-response Specification

## Purpose
TBD - created by archiving change bind-oid4vci-deferred-http-response. Update Purpose after archive.
## Requirements
### Requirement: A deferred request validates both successful Final HTTP outcomes

The SDK SHALL validate an unencrypted Deferred Credential Endpoint response
through the originating `DeferredCredentialRequest`. Status `200` SHALL parse
with the existing bounded immediate response parser and return an issued
outcome. Status `202` SHALL parse with the existing bounded deferred response
parser and return a pending outcome. Every other status SHALL fail with a
static unsupported-status error before body parsing.

The validator SHALL perform no HTTP, token, TLS, retry, timer, transaction
invalidation, credential verification or storage operation.

#### Scenario: issued response becomes a typed outcome

- **WHEN** the request validates a status `200` Final response with a valid
  unencrypted immediate Credential Response body
- **THEN** it returns an issued outcome containing the bounded parsed core

#### Scenario: pending response becomes a typed outcome

- **WHEN** the request validates a status `202` Final response with a valid
  deferred body whose transaction identifier equals the request identifier
- **THEN** it returns a pending outcome containing the bounded parsed core

#### Scenario: another status is not a successful outcome

- **WHEN** the request validates any status other than `200` or `202`
- **THEN** validation returns a static unsupported-status error without parsing
  the body

### Requirement: Successful deferred HTTP responses require bounded JSON media type

`DeferredCredentialHttpResponseLimits` SHALL compose exactly one positive
complete Content-Type byte ceiling with existing immediate and deferred body
limits. An invalid zero Content-Type ceiling SHALL return a static
invalid-limits error. An absent, oversized or syntactically non-JSON
Content-Type SHALL fail before body parsing.

The existing JSON media-type semantics SHALL accept `application/json`
case-insensitively with optional parameters and SHALL reject other media types.
Exact Content-Type limits SHALL succeed and one-byte-over inputs SHALL fail.

#### Scenario: parameterized JSON is accepted at the exact boundary

- **WHEN** a successful response has a parameterized `application/json`
  Content-Type whose bytes exactly match the configured ceiling
- **THEN** validation proceeds to the status-selected body parser

#### Scenario: missing or oversized media type fails first

- **WHEN** Content-Type is absent or exceeds the configured byte ceiling
- **THEN** validation returns a static media-type error without parsing the
  response body

### Requirement: A pending response is correlated without exposing identifiers

`DeferredCredentialRequest` SHALL privately retain its originating decoded
transaction identifier under zeroizing ownership. A parsed `202` response
SHALL be returned only when its transaction identifier exactly equals that
retained value. A mismatch SHALL return a static fieldless error.

The retained identifier SHALL have no public accessor and SHALL NOT appear in
Debug, Display, errors or generic serialization. Validation SHALL remain
repeatable and SHALL NOT imply retry timing, freshness, terminal use or
invalidation policy. The issued branch SHALL NOT claim proof-count correlation.

#### Scenario: substituted pending transaction is rejected

- **WHEN** a valid `202` response carries a different transaction identifier
- **THEN** validation returns the static mismatch error and no identifier
  appears in diagnostics

#### Scenario: repeated validation remains policy-neutral

- **WHEN** the same request validates the same correlated response more than
  once
- **THEN** each structural result is equivalent
- **AND** the SDK makes no timing, replay, freshness or invalidation claim
