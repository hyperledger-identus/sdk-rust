## ADDED Requirements

### Requirement: Archive success proves the requested state transition

The factory archive facade SHALL report success only after the requested active
change is absent, its deterministic dated archive exists, every mandatory
change artifact is preserved, and the resulting OpenSpec store passes the
factory validation gate. It SHALL derive completion from repository state and
SHALL NOT parse human-readable OpenSpec output as a machine contract.

#### Scenario: OpenSpec exits zero without archiving

- **WHEN** the pinned OpenSpec archive command returns zero but leaves the
  requested change active or does not create its expected dated archive
- **THEN** `scripts/factory archive` exits non-zero and does not print its safe
  archive success marker

#### Scenario: Dated archive destination already exists

- **WHEN** the deterministic dated destination for the requested change exists
  before mutation
- **THEN** `scripts/factory archive` exits non-zero before invoking OpenSpec

#### Scenario: Requested archive transition completes

- **WHEN** OpenSpec removes the active change, creates its expected dated
  archive, preserves every mandatory artifact and the resulting store validates
- **THEN** `scripts/factory archive` reports that the named change was archived
  safely
