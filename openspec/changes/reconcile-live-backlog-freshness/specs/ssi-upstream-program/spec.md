## ADDED Requirements

### Requirement: Active backlog ownership is live before autonomous selection

The repository SHALL provide a network-explicit, read-only audit that validates
every canonical backlog issue reference against the configured GitHub
repository before a supervisor selects an `in_progress` row for autonomous
implementation. Every referenced issue SHALL exist and be visible, and every
`in_progress` row SHALL reference an open durable component or focused issue.

The required offline backlog checker SHALL remain network-free. A `specified`
or `delivered` row MAY retain a closed delivery issue as durable evidence; it
MUST receive a new focused open issue and become `in_progress` before further
implementation begins.

#### Scenario: Active owner is open

- **WHEN** the live audit resolves every referenced issue and each
  `in_progress` owner is open
- **THEN** it reports the ledger fresh without claiming that any component is
  complete, conformant or correctly prioritized

#### Scenario: Active owner is closed

- **WHEN** an `in_progress` row references a closed issue
- **THEN** the live audit fails with the row and issue identifiers before a
  supervisor delegates implementation

#### Scenario: Coordination state is unavailable

- **WHEN** GitHub, authentication or issue visibility cannot resolve any
  referenced issue
- **THEN** the live audit fails closed while the deterministic offline checker
  and required fast CI remain independently usable

#### Scenario: Specified evidence is closed

- **WHEN** a `specified` row references the closed issue that established its
  current reusable contract
- **THEN** the live audit accepts the evidence reference but does not treat the
  row as selectable active implementation
