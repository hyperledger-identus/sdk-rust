## MODIFIED Requirements

### Requirement: IDR-023 begins with a focused Credential Offer transport slice

The canonical backlog SHALL keep `IDR-023` at `in_progress` and advance its
issue reference from completed child #121 to active child #123. Issue #123
SHALL deliver only a consuming state transition that binds optional
caller-owned Transaction Code input to the Final Credential Offer's exact
`tx_code` object presence under an independent byte bound and a zeroizing,
redaction-safe lifecycle.

The slice SHALL NOT claim the full OID4VCI engine, metadata discovery or
retrieval, signed metadata, complete RFC 8414 conformance, trust, server
ranking/fallback, authorization or Token Request construction/serialization,
client identity/authentication, Transaction Code correctness, token/credential
responses, replay safety, consumer adoption, publication, or release.

#### Scenario: ledger points at the active bounded child

- **WHEN** issue #123 is implemented
- **THEN** `IDR-023` references #123 with `delivery_status=in_progress`
- **AND** #7 and #20 remain the open component/program parents

#### Scenario: input presence binding is not protocol-engine completion

- **WHEN** Transaction Code input presence and lifecycle gates pass
- **THEN** the backlog keeps `IDR-023` in progress rather than delivered
