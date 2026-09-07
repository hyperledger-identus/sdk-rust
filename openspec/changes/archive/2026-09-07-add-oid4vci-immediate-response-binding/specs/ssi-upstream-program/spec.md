## MODIFIED Requirements

### Requirement: IDR-023 begins with a focused Credential Offer transport slice

The canonical backlog SHALL keep `IDR-023` at `in_progress` and advance its
issue reference from completed child #141 to active child #143. Issue #143
SHALL deliver only bounded, request-bound validation of the unencrypted
OID4VCI Final immediate Credential Response. It SHALL require exact HTTP 200,
an RFC-shaped case-insensitive `application/json` media type, the existing
bounded body-core parser, and a response credential count no greater than the
JWT Credential Request proof count. It SHALL return an owned request-bound
state with fieldless redaction-safe errors while explicitly disclaiming proof
key uniqueness and credential-key correlation.

The slice SHALL NOT claim the full OID4VCI engine, HTTP execution or generic
headers, Cache-Control, TLS/network/endpoint provenance, DPoP, Credential Error
Response parsing, deferred issuance/polling, response encryption,
Authorization Details/Credential identifiers, proof/token trust/freshness,
credential format decoding/verification/trust/status/storage/disclosure,
Notification Endpoint execution, retries/replay, format/chain extensions,
consumer adoption, publication or release.

#### Scenario: ledger points at the active bounded child

- **WHEN** issue #143 is implemented
- **THEN** `IDR-023` references #143 with `delivery_status=in_progress`
- **AND** #7 and #20 remain the open component/program parents

#### Scenario: request-bound immediate response is not protocol-engine completion

- **WHEN** immediate response HTTP metadata, bounded body and necessary proof
  count upper bound are validated through a JWT Credential Request
- **THEN** the backlog keeps `IDR-023` in progress rather than delivered
