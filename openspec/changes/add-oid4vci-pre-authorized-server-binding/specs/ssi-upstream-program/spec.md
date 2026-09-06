## MODIFIED Requirements

### Requirement: IDR-023 begins with a focused Credential Offer transport slice

The canonical backlog SHALL keep `IDR-023` at `in_progress` and advance its
issue reference from completed child #119 to active child #121. Issue #121
SHALL deliver only a consuming cross-document binding from a matched Credential
Offer/Issuer Metadata state to one caller-selected Authorization Server that is
effectively advertised, agrees with any Pre-Authorized Code grant hint,
explicitly supports that grant, and exposes a Token Endpoint.

The slice SHALL NOT claim the full OID4VCI engine, metadata discovery or
retrieval, signed metadata, complete RFC 8414 conformance, trust, server
ranking/fallback, authorization or Token Request construction, client
authentication, Transaction Code input, token/credential responses, replay
safety, consumer adoption, publication, or release.

#### Scenario: ledger points at the active bounded child

- **WHEN** issue #121 is implemented
- **THEN** `IDR-023` references #121 with `delivery_status=in_progress`
- **AND** #7 and #20 remain the open component/program parents

#### Scenario: server binding is not protocol-engine completion

- **WHEN** Pre-Authorized server binding gates pass
- **THEN** the backlog keeps `IDR-023` in progress rather than delivered
