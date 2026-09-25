## MODIFIED Requirements

### Requirement: one-shot authority and proof-count binding

The transition SHALL consume the request and accept no replacement request
authority. For non-202 statuses it SHALL erase all zeroizing transport secrets
before response parsing. For HTTP 202 it SHALL erase the proof body and obsolete
Credential Endpoint before parsing while retaining only the exact validated
issuer, optional Deferred Credential Endpoint, bearer Authorization and proof
count needed for continuation. It SHALL reject an immediate credential count
above the exact originating proof count.

The transition SHALL consume the request, accept no replacement request
authority, and reject an immediate credential count above the exact originating
proof count. The preceding branch-specific lifetime rule replaces the earlier
requirement to erase every transport secret before response parsing, because an
exact HTTP 202 continuation must retain its bearer authority.

#### Scenario: exact HTTP 202 retains only continuation authority

- **WHEN** a Credential Request is consumed by a valid HTTP 202 response
- **THEN** its request-bound deferred state retains the exact issuer, advertised
  Deferred Credential Endpoint, bearer Authorization and proof count
- **AND** its proof body and Credential Endpoint are erased before parsing

#### Scenario: non-202 response retains no bearer authority

- **WHEN** the consumed response status is 200, 400 or unsupported
- **THEN** the entire request is erased before any remote field is parsed

#### Scenario: immediate over-issuance

- **WHEN** HTTP 200 contains more credentials than the request JWT proof count
- **THEN** the existing proof-count error is returned without exposing values
