## MODIFIED Requirements

### Requirement: IDR-023 begins with a focused Credential Offer transport slice

The canonical backlog SHALL keep `IDR-023` at `in_progress` and advance its
issue reference from completed child #149 to active child #237. Issue #237
SHALL deliver only the explicit bounded validation state for
`authorization_details` in an OpenID4VCI Final successful Token Response. It
SHALL preserve the existing presence-only core, require a non-empty RFC 9396
array and at least one recognized `openid_credential` entry, require one
bounded configuration identifier and non-empty bounded unique Credential
Dataset identifiers per recognized entry, traverse and discard bounded
unknown fields/types, retain sensitive values in zeroizing storage, and keep
diagnostics fieldless and redaction-safe.

The slice SHALL NOT claim the full OID4VCI engine, Authorization Request or
Code flow, HTTP execution or provenance, access-token or issuer trust,
Credential Issuer Metadata agreement, identifier selection or Credential
Request construction, request/response correlation, polling, RFC 6750
Authorization Errors, response encryption, proof/token/credential verification
or trust, credential storage/disclosure, format/chain extensions, consumer
adoption, publication, or release.

#### Scenario: ledger points at the active bounded child

- **WHEN** issue #237 is implemented
- **THEN** `IDR-023` references #237 with `delivery_status=in_progress`
- **AND** #7 and #20 remain the open component/program parents

#### Scenario: typed authorization details are not engine completion

- **WHEN** bounded Credential Dataset identifiers are validated without
  metadata correlation, selection, request construction, HTTP or trust
- **THEN** the backlog keeps `IDR-023` in progress rather than delivered
