## MODIFIED Requirements

### Requirement: IDR-010 storage ports have focused ownership

The canonical backlog SHALL keep `IDR-010` at `specified` under focused issue
#91 after the port contract from #89 gains reusable behavioral conformance.
It SHALL remain short of delivered until independent in-memory and encrypted
consumer adapters publish immutable conformance receipts. Concrete product
storage SHALL remain downstream.

#### Scenario: Reusable conformance enters implementation

- **WHEN** issue #91 adds the wallet storage test-support crate
- **THEN** the IDR-010 row references #91 with status `specified` and preserves
  its existing priority, gate, predecessors, outcome and acceptance evidence

#### Scenario: SDK-local harness passes without consumer receipts

- **WHEN** all generic checks and SDK-local test doubles pass but fewer than two
  independent immutable consumer receipts exist
- **THEN** IDR-010 remains `specified` rather than `delivered`
