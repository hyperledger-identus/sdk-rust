## MODIFIED Requirements

### Requirement: Archive success proves the requested state transition

The factory archive facade SHALL report success only after the requested active
change is absent, exactly one new dated archive entry ending in the requested
change name exists relative to the pre-mutation snapshot, that entry is a
regular non-symlink directory, every mandatory change artifact is preserved,
and the resulting OpenSpec store passes the factory validation gate. It SHALL
derive completion from repository state, SHALL NOT predict the completed
archive from the host-local calendar date, and SHALL NOT parse human-readable
OpenSpec output as a machine contract.

#### Scenario: OpenSpec exits zero without archiving

- **WHEN** the pinned OpenSpec archive command returns zero but leaves the
  requested change active or creates no new matching archive entry
- **THEN** `scripts/factory archive` exits non-zero and does not print its safe
  archive success marker

#### Scenario: Archive date differs from the host date

- **WHEN** OpenSpec removes the active change and creates one complete matching
  archive under a date different from the host-local date
- **THEN** `scripts/factory archive` validates that newly created directory and
  reports that the named change was archived safely

#### Scenario: Archive result is ambiguous or indirect

- **WHEN** OpenSpec creates multiple new matching entries, creates a matching
  symlink, or only a pre-existing matching entry is present
- **THEN** `scripts/factory archive` exits non-zero without accepting any entry
  as the completed archive

#### Scenario: Requested archive transition completes

- **WHEN** OpenSpec removes the active change, creates exactly one new regular
  matching archive directory, preserves every mandatory artifact and the
  resulting store validates
- **THEN** `scripts/factory archive` reports that the named change was archived
  safely
