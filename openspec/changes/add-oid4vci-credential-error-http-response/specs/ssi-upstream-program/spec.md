## MODIFIED Requirements

### Requirement: IDR-023 begins with a focused Credential Offer transport slice

The canonical backlog SHALL keep `IDR-023` at `in_progress` and advance its
issue reference from completed child #145 to active child #147. Issue #147
SHALL deliver only bounded validation of the OID4VCI Final Credential Request
payload-error HTTP envelope over the existing Credential Error Response core.
It SHALL require exact status 400, one bounded RFC-shaped `application/json`
effective media value, explicit exclusion of exact generic `invalid_request`,
extension-compatible body semantics, no retention of envelope data, and
fieldless redaction-safe diagnostics.

The slice SHALL NOT claim the full OID4VCI engine, HTTP execution or
provenance, generic header collection, example-only Cache-Control behavior,
RFC 6750 Authorization errors or authentication challenges, request
correlation, issuer truth, retries/remediation/blame/UI policy, deferred
issuance/polling, response encryption, proof/token/credential verification or
trust, credential storage/disclosure, Notification Endpoint execution,
format/chain extensions, consumer adoption, publication, or release.

#### Scenario: ledger points at the active bounded child

- **WHEN** issue #147 is implemented
- **THEN** `IDR-023` references #147 with `delivery_status=in_progress`
- **AND** #7 and #20 remain the open component/program parents

#### Scenario: validated Credential payload-error envelope is not engine completion

- **WHEN** exact status/media and the bounded payload-error body are validated
  without transport, authorization, correlation, or recovery semantics
- **THEN** the backlog keeps `IDR-023` in progress rather than delivered
