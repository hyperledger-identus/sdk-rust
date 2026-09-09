## ADDED Requirements

### Requirement: Crypto line coverage meets the pinned Apollo baseline

The repository SHALL produce one reproducible `identus-crypto` line-coverage
campaign on Rust 1.98.1 with the pinned coverage tool. It SHALL execute default,
all-feature, no-default-feature and `kmp-compat` surfaces into one clean profile
set and count only executable first-party Rust lines below `crates/crypto/src`.
Covered lines SHALL be at least 74.82% of that denominator. Lowering the
accepted threshold SHALL require a superseding ADR.

#### Scenario: A feature surface is omitted

- **WHEN** the coverage runner or manifest omits one declared feature surface
- **THEN** validation SHALL fail before publishing parity evidence

#### Scenario: Coverage falls below Apollo

- **WHEN** covered first-party lines are less than 74.82% of the denominator
- **THEN** the slow campaign SHALL fail and SHALL NOT publish a passing summary

### Requirement: Coverage artifacts are reviewable and path-confined

The slow campaign SHALL emit deterministic JSON and Markdown summaries plus
LCOV line evidence. The summaries SHALL identify the exact SDK revision, Rust
version, coverage-tool version, feature profiles, exclusions, covered lines,
uncovered lines, total lines and percentage. Dependencies, tests, examples and
generated code SHALL remain outside the denominator; no production source file,
error path or secret-handling path may be excluded.

#### Scenario: An unrelated or missing path enters the denominator

- **WHEN** input names a file outside `crates/crypto/src`, a missing file, a
  symlink escape, a non-Rust file or a duplicate file
- **THEN** normalization SHALL fail without emitting accepted evidence

#### Scenario: Artifact metadata is incomplete or drifts

- **WHEN** revision, Rust version, coverage-tool version, profile set or
  denominator fields are missing or differ from the pinned policy
- **THEN** validation SHALL fail with the mismatched field

### Requirement: Coverage remains subordinate to vectors and negative tests

Every executable `identus-crypto` production source file SHALL map in the
Apollo parity ledger to one or more audited capability rows and, when shared
behavior has vector evidence, to existing vector rows. Coverage percentage
SHALL NOT waive a missing shared vector, public error-path test, redaction test,
resource-bound test or explicit safety-critical negative test.

#### Scenario: Source coverage lacks semantic evidence mapping

- **WHEN** an executable crypto source file has no known capability mapping or
  references an unknown capability/vector ID
- **THEN** parity validation SHALL fail before the M2 report is accepted

#### Scenario: Global percentage is high but a critical selector disappears

- **WHEN** a mapped Apollo selector or explicit negative-path selector is
  removed while aggregate line coverage remains above threshold
- **THEN** the existing parity selector validation SHALL still fail

### Requirement: Coverage stays outside the fast pull-request lane

Coverage instrumentation and reporting SHALL run only in the weekly/manual
slow workflow during the ADR 0081 temporary phase. The pull-request and
`develop` fast job SHALL remain unchanged.

#### Scenario: Coverage is added to fast CI

- **WHEN** workflow validation detects the coverage runner in the fast job
- **THEN** policy validation SHALL fail the change
