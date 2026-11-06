# prism-did-method Specification

## ADDED Requirements

### Requirement: portable PRISM method semantics are independently selectable

The SDK SHALL package PRISM short- and long-form syntax in
`identus-prism-did`, with pinned protobuf wire messages, operation construction and validation, authorization lineage,
deterministic state reduction, and DID-document projection in a focused method
crate. The crate SHALL depend inward on generic DID and crypto contracts and
SHALL NOT own Cardano, network, persistence, service, or node-runtime effects.

#### Scenario: a valid ordered operation sequence is reduced

- **WHEN** create, update, or deactivate operations satisfy the pinned method
  rules and resource limits
- **THEN** reduction produces deterministic state and document projection
  independently of a ledger client or database

#### Scenario: concrete chain access is needed

- **WHEN** an application observes or submits PRISM operations on Cardano
- **THEN** the concrete source or sink remains a downstream adapter and the
  method crate consumes only bounded portable input

### Requirement: long-form identifiers have derived finite limits

PRISM long-form parsing and construction SHALL use the generic DID resource
policy established by #524. It SHALL validate embedded create semantics and
SHALL NOT weaken unrelated generic JSON or crypto-text limits.

#### Scenario: an embedded create operation exceeds its budget

- **WHEN** encoded or decoded operation work exceeds the derived PRISM or DID
  boundary
- **THEN** construction or parsing fails before retaining or projecting the
  oversized operation
