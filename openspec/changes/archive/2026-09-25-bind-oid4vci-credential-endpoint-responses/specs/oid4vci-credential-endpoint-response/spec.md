## ADDED Requirements

### Requirement: request-bound Credential Endpoint response classification

The SDK SHALL consume one `JwtCredentialRequest` and classify exactly one
unencrypted Final Credential Endpoint HTTP response as immediate issuance,
deferred issuance, or Credential payload error using status before media/body.

#### Scenario: exact success and error statuses

- **WHEN** the status is 200, 202, or 400 with valid selected JSON content
- **THEN** only the corresponding existing bounded parser runs and the result
  retains the originating request proof count

#### Scenario: unsupported or authorization status

- **WHEN** status is not 200, 202, or 400
- **THEN** classification fails statically before media type or body parsing

### Requirement: one-shot authority and proof-count binding

The transition SHALL consume the request, erase its zeroizing transport secrets
before response parsing, accept no replacement request authority, and reject an
immediate credential count above the exact originating proof count.

#### Scenario: immediate over-issuance

- **WHEN** HTTP 200 contains more credentials than the request JWT proof count
- **THEN** the existing proof-count error is returned without exposing values

### Requirement: compatibility and explicit non-scope

The existing borrowed immediate-only validator SHALL remain behavior-compatible.
The new API SHALL NOT perform HTTP, parse RFC 6750 authorization errors, accept
encrypted responses, or establish trust, retry, polling, validation or storage.

#### Scenario: compatibility path remains exact

- **WHEN** an existing caller uses the borrowed immediate-only validator
- **THEN** its status, media, body and proof-count behavior remains unchanged
