## ADDED Requirements

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
