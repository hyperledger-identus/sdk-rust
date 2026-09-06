## MODIFIED Requirements

### Requirement: IDR-023 begins with a focused Credential Offer transport slice

The canonical backlog SHALL keep `IDR-023` at `in_progress` and advance its
issue reference from completed child #133 to active child #135. Issue #135
SHALL deliver only a transport-neutral OID4VCI Final Credential Nonce Request
constructed from bounded issuer metadata with an advertised Nonce Endpoint:
exact endpoint ownership, static POST and empty-body guidance, explicit absence
of an access-token requirement, a redaction-safe public value/accessor, and a
field-specific missing-endpoint error.

The slice SHALL NOT claim the full OID4VCI engine, metadata discovery or
retrieval, signed metadata, complete RFC 8414 conformance, trust, HTTP
execution, DNS/TLS/redirect/private-network policy, request header policy,
response status/media/cache/DPoP validation, Issuer nonce generation or
unpredictability, response provenance/correlation, nonce freshness/expiry or
replay safety, proof construction/verification, Credential Requests/Responses,
consumer adoption, publication or release.

#### Scenario: ledger points at the active bounded child

- **WHEN** issue #135 is implemented
- **THEN** `IDR-023` references #135 with `delivery_status=in_progress`
- **AND** #7 and #20 remain the open component/program parents

#### Scenario: request description is not protocol-engine completion

- **WHEN** Final Nonce Request construction and diagnostic gates pass
- **THEN** the backlog keeps `IDR-023` in progress rather than delivered
