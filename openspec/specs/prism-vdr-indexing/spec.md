# prism-vdr-indexing Specification

## Purpose
TBD - created by archiving change separate-peer-prism-method-ownership. Update Purpose after archive.
## Requirements
### Requirement: PRISM VDR and indexing cores are effect-isolated

The SDK SHALL place PRISM storage-entry/state rules and bounded root traversal
in `identus-prism-vdr`, which depends on `identus-prism-did`, and SHALL place
deterministic ordered ingestion, batching, replay, and cursor semantics in
`identus-prism-indexer`, which depends on the VDR crate. These crates MAY
define bounded source, sink, store, transaction-batch, and repository ports
but SHALL NOT implement Cardano clients, SQL, HTTP, CLI, deployment, or node
orchestration.

#### Scenario: an ordered batch is indexed

- **WHEN** a bounded source supplies a complete ordered batch
- **THEN** the indexer validates and commits deterministic state atomically
  through caller-provided ports with stable replay and idempotency behavior

#### Scenario: a concrete effect is required

- **WHEN** indexing requires Blockfrost, db-sync, Oura, a wallet, SQL, or a
  service runtime
- **THEN** that implementation remains downstream and is not added to the SDK
  dependency cone

#### Scenario: crate dependency direction is validated

- **WHEN** the method, VDR, and indexer crates enter the workspace
- **THEN** the existing family deny is replaced atomically by a machine guard
  that permits only indexer-to-VDR, VDR-to-method, and method-to-generic inward
  edges, rejects every generic-to-method edge, and rejects prohibited effects
  throughout each transitive dependency cone

#### Scenario: method packages are planned but absent

- **WHEN** the method, VDR, and indexer crates have not entered the workspace
- **THEN** the dependency guard rejects the complete `identus-prism-*` family

### Requirement: ancestry and replay work are bounded

Root traversal SHALL apply finite limits to ancestry depth, pages, batch size,
channels, retries, and retained state. Cycles, missing parents, rollback,
depth exhaustion, and duplicate replay SHALL produce deterministic redacted
outcomes.

#### Scenario: ancestry is cyclic or too deep

- **WHEN** VDR-root traversal encounters a cycle or exceeds its configured
  depth
- **THEN** processing terminates without partial state commitment and returns
  the corresponding stable bounded-work outcome

