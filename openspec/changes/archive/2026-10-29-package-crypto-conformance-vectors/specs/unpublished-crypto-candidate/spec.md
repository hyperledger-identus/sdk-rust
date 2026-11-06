## ADDED Requirements

### Requirement: Declared candidate tests are archive-self-contained

Every test input required by a candidate package's declared test targets SHALL
be a regular file inside that package archive. Candidate verification SHALL
extract the archive and run its all-feature test suite without copying files
from the source repository or resolving paths above the extracted package.

#### Scenario: Crypto compatibility test runs after extraction

- **WHEN** the `identus-crypto` candidate archive is extracted into the
  independent verification workspace
- **THEN** its compatibility packet is present at the crate-local path and the
  all-feature compatibility test executes without repository files

#### Scenario: A declared test input is absent

- **WHEN** Cargo's package listing or the extracted archive omits a test input
  referenced by a declared target
- **THEN** candidate preparation fails before emitting success evidence
