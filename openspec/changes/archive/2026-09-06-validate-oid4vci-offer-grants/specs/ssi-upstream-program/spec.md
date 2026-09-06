## MODIFIED Requirements

### Requirement: IDR-023 begins with a focused Credential Offer transport slice

The canonical backlog SHALL keep `IDR-023` at `in_progress` and advance its
focused issue from completed semantic-offer issue #113 to grant-shape issue
#115 when bounded known-grant validation is implemented. Priority, target gate,
component outcome, acceptance evidence, consumer dependency, commitment,
predecessors, normative source, and source repositories SHALL remain unchanged.

The row SHALL remain short of `delivered` because metadata, authorization,
token, nonce, credential, deferred, notification, protocol-state, and complete
conformance behavior are not supplied by the first three slices.

#### Scenario: third protocol slice lands

- **WHEN** issue #115 adds bounded known-grant semantics after #111 transport
  and #113 core-offer semantics
- **THEN** `IDR-023` references #115 with `delivery_status=in_progress`

#### Scenario: partial OID4VCI does not overclaim completion

- **WHEN** transport, core-offer, and grant-shape parsing pass but the remaining
  parent #7 capabilities are absent
- **THEN** the backlog keeps `IDR-023` in progress rather than delivered
