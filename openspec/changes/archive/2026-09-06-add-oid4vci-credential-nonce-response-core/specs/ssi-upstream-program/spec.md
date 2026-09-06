## MODIFIED Requirements

### Requirement: IDR-023 begins with a focused Credential Offer transport slice

The canonical backlog SHALL keep `IDR-023` at `in_progress` and advance its
issue reference from completed child #129 to active child #131. Issue #131
SHALL deliver only a bounded partial Credential Nonce Response core: required
opaque non-empty `c_nonce`, aggregate and independent limits,
extension-tolerant strict JSON, zeroizing retention, an explicitly sensitive
accessor, and content-free diagnostics.

The slice SHALL NOT claim the full OID4VCI engine, metadata discovery or
retrieval, `nonce_endpoint` exposure, signed metadata, complete RFC 8414
conformance, trust, Nonce Request construction, HTTP execution or
status/header/cache validation, DPoP, Issuer nonce generation or
unpredictability, response provenance/correlation, nonce freshness/expiry or
replay safety, proof construction/verification, Credential Requests/Responses,
consumer adoption, publication, or release.

#### Scenario: ledger points at the active bounded child

- **WHEN** issue #131 is implemented
- **THEN** `IDR-023` references #131 with `delivery_status=in_progress`
- **AND** #7 and #20 remain the open component/program parents

#### Scenario: nonce response core is not protocol-engine completion

- **WHEN** bounded Final `c_nonce` parsing and diagnostic gates pass
- **THEN** the backlog keeps `IDR-023` in progress rather than delivered
