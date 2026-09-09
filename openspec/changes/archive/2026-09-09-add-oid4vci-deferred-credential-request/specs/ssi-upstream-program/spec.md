## MODIFIED Requirements

### Requirement: IDR-023 begins with a focused Credential Offer transport slice

The canonical backlog SHALL keep `IDR-023` at `in_progress` and advance its
issue reference from completed child #241 to active child #250. Issue #250
SHALL deliver only bounded construction of the unencrypted Final Deferred
Credential Request from existing typed response and metadata state.

The slice SHALL NOT claim the full OID4VCI engine, HTTP execution, token
ownership/validation, TLS, interval scheduling, transaction lifecycle,
request/response encryption, extension parameters, error response handling,
response correlation, credential verification/storage, format/chain
extensions, consumer adoption, publication or release.

#### Scenario: ledger points at the active bounded child

- **WHEN** issue #250 is implemented
- **THEN** `IDR-023` references #250 with `delivery_status=in_progress`
- **AND** #7 and #20 remain the open component/program parents

#### Scenario: request construction is not engine completion

- **WHEN** a bounded Deferred Credential Request is constructed without HTTP,
  token, timing, lifecycle, encryption or correlation behavior
- **THEN** the backlog keeps `IDR-023` in progress rather than delivered
