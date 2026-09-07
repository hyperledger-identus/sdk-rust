## MODIFIED Requirements

### Requirement: IDR-023 begins with a focused Credential Offer transport slice

The canonical backlog SHALL keep `IDR-023` at `in_progress` and advance its
issue reference from completed child #143 to active child #145. Issue #145
SHALL deliver only bounded parsing of the OID4VCI Final Credential Error
Response body. It SHALL require a strict top-level object, required exact
extension-compatible error code, optional explicitly untrusted description,
positive aggregate/structural/value limits, duplicate-safe traversal,
zeroizing retention, and fieldless redaction-safe diagnostics. It SHALL
classify the seven Final values without assigning recovery policy to them.

The slice SHALL NOT claim the full OID4VCI engine, HTTP status/media/header or
transport validation, RFC 6750 Authorization errors, authentication
challenges, request correlation, issuer truth, retries/remediation/blame/UI
policy, legacy `c_nonce` semantics, error URI, deferred issuance/polling,
response encryption, proof/token/credential verification or trust,
credential storage/disclosure, Notification Endpoint execution, format/chain
extensions, consumer adoption, publication or release.

#### Scenario: ledger points at the active bounded child

- **WHEN** issue #145 is implemented
- **THEN** `IDR-023` references #145 with `delivery_status=in_progress`
- **AND** #7 and #20 remain the open component/program parents

#### Scenario: parsed Credential Error body is not protocol-engine completion

- **WHEN** the bounded error code and optional description are parsed and
  classified without transport or recovery semantics
- **THEN** the backlog keeps `IDR-023` in progress rather than delivered
