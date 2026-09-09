## MODIFIED Requirements

### Requirement: IDR-023 begins with a focused Credential Offer transport slice

The canonical backlog SHALL keep `IDR-023` at `in_progress` and advance its
issue reference from completed child #237 to active child #239. Issue #239
SHALL deliver only a checked Final JWT Credential Request constructor that
selects one bounded Credential Dataset identifier from the #237 typed Token
Response state, confirms its configuration was offered, emits exactly
`credential_identifier` plus existing `proofs.jwt`, and reuses the current
Bearer/proof/body bounds, zeroizing ownership and redacted diagnostics.

The slice SHALL NOT claim the full OID4VCI engine, Authorization Request or
Code flow, HTTP execution/provenance, access-token or issuer trust, product
identifier ranking/choice, proof creation, response correlation changes,
replay/invalidation, credential verification/storage, format/chain extensions,
consumer adoption, publication, or release.

#### Scenario: ledger points at the active bounded child

- **WHEN** issue #239 is implemented
- **THEN** `IDR-023` references #239 with `delivery_status=in_progress`
- **AND** #7 and #20 remain the open component/program parents

#### Scenario: request encoding is not engine completion

- **WHEN** a selected authorized identifier is encoded without HTTP, trust,
  replay or product selection policy
- **THEN** the backlog keeps `IDR-023` in progress rather than delivered
