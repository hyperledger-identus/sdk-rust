## ADDED Requirements

### Requirement: IDR-010 storage ports have focused ownership

The canonical backlog SHALL move `IDR-010` from the program queue to
`specified` under focused issue #89 when the policy-neutral port contract
enters implementation. It SHALL remain short of delivered until independent
in-memory and encrypted consumer adapters pass conformance. Concrete product
storage SHALL remain downstream.

#### Scenario: Generic storage contract enters implementation

- **WHEN** issue #89 activates the `identus-wallet` storage-port surface
- **THEN** the IDR-010 row references #89 with status `specified` and preserves
  its existing priority, gate, predecessors, outcome and acceptance evidence

#### Scenario: SDK ports exist without consumer conformance

- **WHEN** all generic storage ports and SDK-local test doubles pass but no
  immutable consumer adapter conformance receipt exists
- **THEN** IDR-010 remains `specified` rather than `delivered`
