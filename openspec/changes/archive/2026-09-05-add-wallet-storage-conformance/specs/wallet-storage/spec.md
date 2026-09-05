## ADDED Requirements

### Requirement: Successful replacement invalidates the preceding revision

Every successful replacement SHALL return a revision different from the exact
revision it replaced. A later write or delete using the invalidated revision
SHALL return `StorageError::Conflict` and SHALL preserve the current record.
This rule SHALL NOT imply global revision uniqueness, ordering, generation or
backend representation.

#### Scenario: Previous reader attempts a stale mutation

- **WHEN** one caller replaces a record after another caller observed its prior
  revision and the earlier caller attempts a conditional write or delete
- **THEN** both stale operations fail with Conflict and the replacement remains
  current
