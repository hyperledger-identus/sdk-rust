## MODIFIED Requirements

### Requirement: IDR-023 begins with a focused Credential Offer transport slice

The canonical backlog SHALL keep `IDR-023` at `in_progress` and advance its
issue reference from completed child #147 to active child #149. Issue #149
SHALL deliver only the bounded unencrypted deferred Credential Response body
core from OpenID4VCI Final section 8.3. It SHALL require a non-empty bounded
`transaction_id` and positive exact JSON-number `interval`, forbid immediate-
only members, traverse and discard bounded unique extensions, retain sensitive
values in zeroizing storage, and keep diagnostics fieldless and redaction-safe.

The slice SHALL NOT claim the full OID4VCI engine, HTTP status/media/execution
or provenance, request correlation, interval conversion or polling policy,
Deferred Credential Request construction, transaction freshness/invalidation,
RFC 6750 Authorization Errors, response encryption, proof/token/credential
verification or trust, credential storage/disclosure, Notification Endpoint
execution, format/chain extensions, consumer adoption, publication, or release.

#### Scenario: ledger points at the active bounded child

- **WHEN** issue #149 is implemented
- **THEN** `IDR-023` references #149 with `delivery_status=in_progress`
- **AND** #7 and #20 remain the open component/program parents

#### Scenario: deferred body syntax is not engine completion

- **WHEN** the bounded transaction and interval body is parsed without HTTP,
  request correlation, polling, or credential verification semantics
- **THEN** the backlog keeps `IDR-023` in progress rather than delivered
