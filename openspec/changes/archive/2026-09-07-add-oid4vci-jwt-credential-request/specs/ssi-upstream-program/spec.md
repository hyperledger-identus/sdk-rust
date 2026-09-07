## MODIFIED Requirements

### Requirement: IDR-023 begins with a focused Credential Offer transport slice

The canonical backlog SHALL keep `IDR-023` at `in_progress` and advance its
issue reference from completed child #137 to active child #139. Issue #139
SHALL deliver only bounded, transport-neutral construction of the unencrypted
OID4VCI Final Credential Request configuration-ID/JWT-proof path from matched
offer/metadata, a successful Bearer Token Response without unvalidated
Authorization Details, one offered configuration index and non-empty
holder-produced proof JWTs. It SHALL use positive proof/body/authorization
limits, deterministic JSON, zeroizing ownership and fieldless redaction-safe
errors.

The slice SHALL NOT claim the full OID4VCI engine, metadata discovery or trust,
HTTP execution, TLS/DNS/redirect/private-network policy, DPoP, token trust,
freshness, scope or refresh, Authorization Details or Credential identifiers,
proof construction/verification/trust/freshness/replay, proofless or non-JWT
requests, format/chain extensions, request/response encryption, Credential
Response/error/deferred processing, consumer adoption, publication or release.

#### Scenario: ledger points at the active bounded child

- **WHEN** issue #139 is implemented
- **THEN** `IDR-023` references #139 with `delivery_status=in_progress`
- **AND** #7 and #20 remain the open component/program parents

#### Scenario: request construction is not protocol-engine completion

- **WHEN** the bounded Final configuration-ID/JWT-proof request is constructed
- **THEN** the backlog keeps `IDR-023` in progress rather than delivered
