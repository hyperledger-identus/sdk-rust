# factory-operations

## ADDED Requirements

### Requirement: PR metadata matching is deterministic for bounded bodies

Required-field matching in the shared pull-request policy checker SHALL be
independent of body size and match position for every input inside the accepted
bound. A successful match SHALL NOT be converted into failure by producer
`SIGPIPE` or pipeline-status composition. The accepted metadata syntax,
extracted issue identity, diagnostics and input bound SHALL remain unchanged.

#### Scenario: Long bounded PR body matches near the beginning

- **WHEN** every required metadata line is valid near the beginning of a body
  that remains inside the delivery bound
- **THEN** local and hosted policy validation pass independently of unread
  filler after the match

#### Scenario: Long bounded PR body matches near the end

- **WHEN** every required metadata line is valid near the end of a body that
  remains inside the delivery bound
- **THEN** local and hosted policy validation pass with the same extracted
  issue and diagnostics contract

#### Scenario: Required metadata is absent

- **WHEN** a bounded body omits or malforms any required metadata line
- **THEN** the checker fails with the same field-specific diagnostic
