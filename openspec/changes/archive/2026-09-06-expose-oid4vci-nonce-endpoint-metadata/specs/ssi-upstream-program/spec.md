## MODIFIED Requirements

### Requirement: IDR-023 begins with a focused Credential Offer transport slice

The canonical backlog SHALL keep `IDR-023` at `in_progress` and advance its
issue reference from completed child #131 to active child #133. Issue #133
SHALL deliver only optional Final `nonce_endpoint` interpretation inside the
bounded Credential Issuer Metadata core: exact HTTPS URL retention, the
existing shared endpoint byte budget, a redaction-safe public value/accessor,
and field-specific static diagnostics.

The slice SHALL NOT claim the full OID4VCI engine, metadata discovery or
retrieval, signed metadata, complete RFC 8414 conformance, trust, Nonce Request
construction, HTTP execution or status/header/cache validation, DPoP, Issuer
nonce generation or unpredictability, response provenance/correlation, nonce
freshness/expiry or replay safety, proof construction/verification, Credential
Requests/Responses, consumer adoption, publication or release.

#### Scenario: ledger points at the active bounded child

- **WHEN** issue #133 is implemented
- **THEN** `IDR-023` references #133 with `delivery_status=in_progress`
- **AND** #7 and #20 remain the open component/program parents

#### Scenario: endpoint metadata is not protocol-engine completion

- **WHEN** optional Final Nonce Endpoint parsing and diagnostic gates pass
- **THEN** the backlog keeps `IDR-023` in progress rather than delivered
