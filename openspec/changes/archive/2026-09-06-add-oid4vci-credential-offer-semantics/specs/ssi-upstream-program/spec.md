## MODIFIED Requirements

### Requirement: IDR-023 begins with a focused Credential Offer transport slice

The canonical backlog SHALL keep `IDR-023` at `in_progress` and advance its
focused issue from completed transport issue #111 to semantic-offer issue #113
when bounded Credential Offer core-member validation is implemented. Priority,
target gate, component outcome, acceptance evidence, consumer dependency,
commitment, predecessors, normative source, and source repositories SHALL
remain unchanged.

The row SHALL remain short of `delivered` because grant-member semantics,
metadata, authorization, token, nonce, credential, deferred, notification,
protocol-state, and complete conformance behavior are not supplied by the first
two slices.

#### Scenario: second protocol slice lands

- **WHEN** issue #113 adds bounded core-member semantics after #111 transport
- **THEN** `IDR-023` references #113 with `delivery_status=in_progress`

#### Scenario: partial OID4VCI does not overclaim completion

- **WHEN** transport and core-offer parsing pass but the remaining parent #7
  capabilities are absent
- **THEN** the backlog keeps `IDR-023` in progress rather than delivered
