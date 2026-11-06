## ADDED Requirements

### Requirement: Published API origins remain immutable compatibility inputs

The SDK SHALL preserve a published train's generated public-API origin and
compare later source to it without baseline initialization. Qualification MAY
accept additive current API, but SHALL fail when any published API occurrence
is removed or changed and SHALL emit bounded deterministic diff evidence.

#### Scenario: Current development is additive

- **WHEN** every published API occurrence remains and current source adds items
- **THEN** qualification retains the published origin and records the additions

#### Scenario: Published API history would be rewritten

- **WHEN** tooling requests initialization or baseline replacement for a train
  already recorded as published
- **THEN** qualification fails before modifying the origin
