## MODIFIED Requirements

### Requirement: IDR-023 begins with a focused Credential Offer transport slice

The canonical backlog SHALL keep `IDR-023` at `in_progress` and advance its
issue reference from completed child #125 to active child #127. Issue #127
SHALL deliver only a bounded partial successful Token Response core: mandatory
OAuth token fields, optional expiry/refresh/scope, aggregate and independent
limits, extension-tolerant strict JSON, zeroizing secret ownership, redacted
diagnostics, and explicit unvalidated Authorization Details presence.

The slice SHALL NOT claim the full OID4VCI engine, metadata discovery or
retrieval, signed metadata, complete RFC 8414 conformance, trust, HTTP
execution, response provenance/correlation, Token Error Responses,
Authorization Details or Credential Dataset validation, token cryptographic or
time validity, refresh, nonce, Credential Requests/Responses, retries/replay
safety, consumer adoption, publication, or release.

#### Scenario: ledger points at the active bounded child

- **WHEN** issue #127 is implemented
- **THEN** `IDR-023` references #127 with `delivery_status=in_progress`
- **AND** #7 and #20 remain the open component/program parents

#### Scenario: successful response core is not protocol-engine completion

- **WHEN** bounded OAuth core parsing and sensitive-lifecycle gates pass
- **THEN** the backlog keeps `IDR-023` in progress rather than delivered
