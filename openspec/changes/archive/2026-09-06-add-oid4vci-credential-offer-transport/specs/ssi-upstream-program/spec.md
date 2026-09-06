## ADDED Requirements

### Requirement: IDR-023 begins with a focused Credential Offer transport slice

The canonical backlog SHALL move `IDR-023` from `specified` under parent #7 to
`in_progress` under focused issue #111 when the bounded Credential Offer
transport is implemented. Priority, target gate, component outcome,
acceptance evidence, consumer dependency, commitment, predecessors, normative
source, and source repositories SHALL remain unchanged.

The row SHALL remain short of `delivered` because metadata, authorization,
token, nonce, credential, deferred, notification, protocol-state, and complete
conformance behavior are not supplied by this first slice.

#### Scenario: first protocol slice lands

- **WHEN** issue #111 adds the focused crate and its transport evidence
- **THEN** `IDR-023` references #111 with `delivery_status=in_progress`

#### Scenario: partial OID4VCI does not overclaim completion

- **WHEN** transport parsing passes but the remaining parent #7 capabilities
  are absent
- **THEN** the backlog keeps `IDR-023` in progress rather than delivered
