# ssi-upstream-program Specification

## MODIFIED Requirements

### Requirement: IDR-023 begins with a focused Credential Offer transport slice

The canonical backlog SHALL keep `IDR-023` at `in_progress` and advance its
issue reference from completed child #139 to active child #141. Issue #141
SHALL deliver only bounded, transport-neutral parsing of the unencrypted
OID4VCI Final immediate Credential Response JSON body. It SHALL require a
non-empty ordered `credentials` array, accept only string or object
`credential` values, retain exact values in zeroizing ownership, expose decoded
string values, accept an optional opaque `notification_id`, tolerate bounded
unique extensions, reject duplicate members, and distinguish the unsupported
deferred `transaction_id` branch with fieldless redaction-safe errors.

The slice SHALL NOT claim the full OID4VCI engine, HTTP status/media/cache or
network handling, endpoint/issuer provenance, request correlation, Credential
Error Response parsing, deferred issuance/polling, response encryption,
credential format decoding/verification/trust/status/storage/disclosure,
Notification Endpoint execution, retries/replay, format/chain extensions,
consumer adoption, publication or release.

#### Scenario: ledger points at the active bounded child

- **WHEN** issue #141 is implemented
- **THEN** `IDR-023` references #141 with `delivery_status=in_progress`
- **AND** #7 and #20 remain the open component/program parents

#### Scenario: immediate response parsing is not protocol-engine completion

- **WHEN** the bounded Final immediate Credential Response core is parsed
- **THEN** the backlog keeps `IDR-023` in progress rather than delivered
