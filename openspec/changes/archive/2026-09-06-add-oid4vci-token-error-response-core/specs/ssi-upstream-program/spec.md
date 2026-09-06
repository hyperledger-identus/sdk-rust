## MODIFIED Requirements

### Requirement: IDR-023 begins with a focused Credential Offer transport slice

The canonical backlog SHALL keep `IDR-023` at `in_progress` and advance its
issue reference from completed child #127 to active child #129. Issue #129
SHALL deliver only a bounded partial Token Error Response core: an exact open
error code with closed RFC classification, optional untrusted description and
validated URI-reference, aggregate and independent limits,
extension-tolerant strict JSON, zeroizing retained strings, and redacted
diagnostics.

The slice SHALL NOT claim the full OID4VCI engine, metadata discovery or
retrieval, signed metadata, complete RFC 8414 conformance, trust, HTTP
execution or status/header validation, response provenance/correlation,
retry/remediation policy, successful Token Responses, Authorization Details or
Credential Dataset validation, token cryptographic or time validity, refresh,
nonce, Credential Requests/Responses, replay safety, consumer adoption,
publication, or release.

#### Scenario: ledger points at the active bounded child

- **WHEN** issue #129 is implemented
- **THEN** `IDR-023` references #129 with `delivery_status=in_progress`
- **AND** #7 and #20 remain the open component/program parents

#### Scenario: error response core is not protocol-engine completion

- **WHEN** bounded RFC error parsing and untrusted-metadata gates pass
- **THEN** the backlog keeps `IDR-023` in progress rather than delivered
