## MODIFIED Requirements

### Requirement: IDR-023 begins with a focused Credential Offer transport slice

The canonical backlog SHALL keep `IDR-023` at `in_progress` and advance its
issue reference from completed child #123 to active child #125. Issue #125
SHALL deliver only a consuming state transition that constructs the mandatory
Pre-Authorized Code Token Request fields, deterministic bounded UTF-8 form
body, validated Token Endpoint, and static transport guidance under a
zeroizing, redaction-safe lifecycle.

The slice SHALL NOT claim the full OID4VCI engine, metadata discovery or
retrieval, signed metadata, complete RFC 8414 conformance, trust, server
ranking/fallback, HTTP execution, client identity/authentication, optional
authorization selectors, Transaction Code correctness, token/credential
responses, retries/replay safety, consumer adoption, publication, or release.

#### Scenario: ledger points at the active bounded child

- **WHEN** issue #125 is implemented
- **THEN** `IDR-023` references #125 with `delivery_status=in_progress`
- **AND** #7 and #20 remain the open component/program parents

#### Scenario: mandatory request construction is not protocol-engine completion

- **WHEN** deterministic form construction and sensitive-lifecycle gates pass
- **THEN** the backlog keeps `IDR-023` in progress rather than delivered
