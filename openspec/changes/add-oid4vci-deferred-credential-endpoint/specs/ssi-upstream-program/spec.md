## MODIFIED Requirements

### Requirement: IDR-023 begins with a focused Credential Offer transport slice

The canonical backlog SHALL keep `IDR-023` at `in_progress` and advance its
issue reference from completed child #239 to active child #241. Issue #241
SHALL deliver only the optional bounded Final `deferred_credential_endpoint`
in Credential Issuer Metadata with exact HTTPS retention, omission semantics,
independent shared endpoint-byte limits and redacted diagnostics.

The slice SHALL NOT claim the full OID4VCI engine, Deferred Credential Request,
Authorization Request or Code flow, HTTP execution/provenance, interval
scheduling, access-token/transaction/issuer trust, encryption, response
correlation, retry/recovery, credential verification/storage, format/chain
extensions, consumer adoption, publication or release.

#### Scenario: ledger points at the active bounded child

- **WHEN** issue #241 is implemented
- **THEN** `IDR-023` references #241 with `delivery_status=in_progress`
- **AND** #7 and #20 remain the open component/program parents

#### Scenario: endpoint discovery is not engine completion

- **WHEN** a bounded Deferred Credential Endpoint is exposed without request,
  HTTP, scheduling, trust or transaction lifecycle behavior
- **THEN** the backlog keeps `IDR-023` in progress rather than delivered
