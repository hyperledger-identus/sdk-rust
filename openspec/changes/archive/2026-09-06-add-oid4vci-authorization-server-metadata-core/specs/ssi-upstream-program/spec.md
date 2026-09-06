## MODIFIED Requirements

### Requirement: IDR-023 begins with a focused Credential Offer transport slice

The canonical backlog SHALL keep `IDR-023` at `in_progress` and advance its
issue reference from completed child #117 to active child #119. Issue #119
SHALL deliver only a bounded partial Authorization Server Metadata core with
exact issuer binding, safe optional endpoints, grant advertisement/defaults,
and the OID4VCI anonymous Pre-Authorized Code flag.

The slice SHALL NOT claim the full OID4VCI engine, metadata retrieval, signed
metadata, complete RFC 8414 conformance, trust, grant/server selection,
authorization/token/credential messages, replay safety, consumer adoption,
publication, or release.

#### Scenario: ledger points at the active bounded child

- **WHEN** issue #119 is implemented
- **THEN** `IDR-023` references #119 with `delivery_status=in_progress`
- **AND** #7 and #20 remain the open component/program parents

#### Scenario: partial metadata delivery is not engine completion

- **WHEN** Authorization Server Metadata Core gates pass
- **THEN** the backlog keeps `IDR-023` in progress rather than delivered
