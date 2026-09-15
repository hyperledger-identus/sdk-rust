# unpublished-crypto-candidate Specification

## MODIFIED Requirements

### Requirement: Package archives are deterministic and self-describing

The SDK SHALL assemble each candidate twice from the same exact source revision
and SHALL reject differing bytes, unexpected/unbounded contents, missing
metadata/license/README, Git dependencies, path-only normalized dependencies,
or checksum drift.

The independent Cargo assembly workspaces SHALL resolve outside the canonical
source repository so caller-selected output paths and stage names cannot enter
Cargo VCS metadata. Completed evidence SHALL be staged on the requested output
filesystem and SHALL become visible only through the existing atomic rename.

#### Scenario: Same source is packaged twice

- **WHEN** both independent staging runs complete
- **THEN** every corresponding `.crate` archive has the same SHA-256 digest and
  bounded byte size

#### Scenario: Build scratch resolves beneath the source repository

- **WHEN** the selected temporary build root is inside the canonical checkout
- **THEN** candidate preparation fails before invoking Cargo or creating the
  completed output

#### Scenario: Requested output is inside the source repository

- **WHEN** a caller requests the normal ignored `artifacts/` destination
- **THEN** Cargo stages remain outside VCS discovery while verified completed
  evidence is atomically renamed at that destination
