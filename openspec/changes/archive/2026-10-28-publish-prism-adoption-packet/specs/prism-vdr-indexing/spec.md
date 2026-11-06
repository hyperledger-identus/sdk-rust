## ADDED Requirements

### Requirement: The portable stack retains a readable end-to-end scenario

The repository SHALL run one deterministic integration scenario through the
public PRISM DID, VDR, and indexer APIs using only caller-owned in-memory ports.
The scenario SHALL cover construction and signing, ordered atomic commit, DID
projection, VDR lookup/update, and exact replay without introducing a
production store, runtime, network, or custody implementation.

#### Scenario: an adopter runs the documented E2E command

- **WHEN** the focused integration test executes
- **THEN** the complete portable flow commits deterministically, resolves the expected DID and VDR state, and treats exact replay as idempotent without a second commit
