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

### Requirement: A request-bound deferred endpoint response is classified once

The SDK SHALL consume one `RequestBoundDeferredCredentialRequest` and classify
exactly one unencrypted Final Deferred Credential Endpoint HTTP response as
issued credentials, a correlated pending transaction, or a deferred payload
error. Status 200, 202 or 400 SHALL select only its existing bounded parser
before Content-Type or body inspection. Every other status SHALL erase the
request and return the existing static unsupported-status error.

#### Scenario: exact supported statuses select one parser

- **WHEN** a request-bound deferred request consumes status 200, 202 or 400
  with valid selected JSON content
- **THEN** exactly the issued, pending or deferred-error outcome is returned
  with the exact originating proof count

#### Scenario: unsupported status is terminal before remote parsing

- **WHEN** the consumed response has any other status and adversarial media and
  body values
- **THEN** the complete request is erased and the static status error wins

### Requirement: deferred continuation authority has branch-specific lifetime

For status 200, 400 and unsupported statuses, the SDK SHALL erase the complete
request, including Authorization, transaction and JSON body, before parsing
remote fields. An HTTP 200 result SHALL be rejected when its credential count
exceeds the exact originating proof count. An HTTP 400 result SHALL retain only
the bounded deferred error response and proof count.

For status 202, the SDK SHALL erase the serialized request body before parsing
and retain the exact issuer, Deferred Credential Endpoint, zeroizing
Authorization and proof count only when the parsed transaction exactly equals
the request transaction. Mismatch or any parse failure SHALL erase all retained
authority. The successful pending result SHALL reuse the existing consuming
transition into the next authorized request without replacement inputs.

#### Scenario: issued credentials remain proof-count bounded

- **WHEN** HTTP 200 contains more credentials than the originating request's
  exact proof count
- **THEN** the existing proof-count error is returned after request erasure

#### Scenario: only exact pending transaction continues

- **WHEN** HTTP 202 contains a bounded transaction equal to the consumed request
- **THEN** the result alone can construct the next request with exact authority
- **AND** a different transaction or malformed response retains no authority

#### Scenario: deferred error is terminal evidence

- **WHEN** HTTP 400 contains a valid deferred payload-error response
- **THEN** the result retains its bounded classification and proof count but no
  issuer, endpoint, Authorization, transaction or request body

### Requirement: consuming classification is additive and effect-free

The composite response policy SHALL contain only the existing successful
Deferred Credential HTTP limits and Credential Error HTTP limits. The
consuming outcome and its bound states SHALL be non-Clone, redaction-safe and
free of generic Serde. No replacement endpoint, bearer, issuer, transaction or
proof count SHALL be accepted.

Existing borrowed success and error validators SHALL remain behavior-compatible.
The transition SHALL perform no HTTP, origin/TLS proof, token validation,
timing, retry, polling, credential verification/storage or product behavior.

#### Scenario: borrowed validation remains compatible

- **WHEN** an existing caller uses either borrowed deferred validator
- **THEN** its status, media, body, correlation and classification behavior is
  unchanged

#### Scenario: diagnostics redact request and response content

- **WHEN** outcomes, errors and bridged errors are rendered
- **THEN** bearer, endpoint, issuer, transaction, request body, credential and
  untrusted description values do not appear
