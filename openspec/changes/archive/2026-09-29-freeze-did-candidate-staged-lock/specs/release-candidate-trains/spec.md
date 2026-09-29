# release-candidate-trains

## ADDED Requirements

### Requirement: Candidate lanes consume one reviewed staged dependency lock

A release-candidate train SHALL bind one bounded repository-owned lockfile and
exact digest to its staged manifest identity before archive or compiler/target
qualification. Every independent lane SHALL verify and consume those exact
bytes with locked Cargo operations and SHALL NOT generate, update, or select a
different dependency resolution. An intentional manifest or dependency change
SHALL refresh the staged lock through a separately reviewable diff.

#### Scenario: Four DID lanes qualify one candidate

- **WHEN** Linux/macOS primary/MSRV lanes render the same candidate manifests
- **THEN** every lane copies the descriptor-bound lock and reports its exact
  digest without consulting registry state for a newer resolution

#### Scenario: Lock bytes or manifest identity drift

- **WHEN** the lock is missing, modified, malformed, stale for the staged
  manifests, or differs from the descriptor digest
- **THEN** candidate preparation fails before a lane can create passing evidence

#### Scenario: A compatible dependency is published during the matrix

- **WHEN** registry state changes after the first lane starts
- **THEN** later lanes retain the reviewed lock versions and the aggregate
  result is determined by the repository candidate rather than execution timing
