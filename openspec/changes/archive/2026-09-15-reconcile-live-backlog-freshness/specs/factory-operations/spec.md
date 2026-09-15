## ADDED Requirements

### Requirement: Supervisor backlog freshness is explicit and non-mutating

The factory SHALL expose a dedicated live backlog audit for supervisor work
selection. The command SHALL use the repository identity from tracked policy,
request only bounded issue-state metadata, deduplicate issue lookups and make
GitHub or malformed-response failures visible. It SHALL NOT modify issues,
roadmap state, repository settings or worker configuration.

A strict snapshot input MAY reproduce and test the decision without network
access. Snapshot use SHALL be reported distinctly from a live result and SHALL
validate exact schema, repository identity, unique issue numbers and closed
state values.

#### Scenario: Supervisor prepares a Pi task

- **WHEN** the supervisor is about to select a canonical `in_progress` work
  item
- **THEN** it runs the live backlog audit before preparing the issue-bound Pi
  invocation envelope

#### Scenario: Fast CI validates repository structure

- **WHEN** required pull-request CI runs without GitHub coordination access
- **THEN** it continues to run the offline backlog checker and does not claim
  live freshness

#### Scenario: Audit receives a fixture snapshot

- **WHEN** a test or operator supplies a valid exact-repository issue-state
  snapshot
- **THEN** the same delivery-state rules run without invoking GitHub and the
  output identifies snapshot mode
