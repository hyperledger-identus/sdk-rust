# release-candidate-trains Specification

## Purpose
TBD - created by archiving change define-reusable-release-candidate-manifest. Update Purpose after archive.
## Requirements
### Requirement: Independent train identities are closed and collision-free

The SDK SHALL maintain one closed ordered train index. Every train SHALL have a
unique stable identifier, lifecycle, descriptor, primary exact Cargo package,
package ownership and immutable tag. Every new independent train after the
first historical release SHALL use tag `<primary-package>-v<version>` and SHALL
NOT reuse or move an existing tag.

#### Scenario: Second train shares the first version

- **WHEN** the DID train selects version `0.1.0-rc.1`
- **THEN** its identity is `identus-did-v0.1.0-rc.1` and the existing
  `v0.1.0-rc.1` crypto release remains unchanged

#### Scenario: Train identity collides

- **WHEN** IDs, descriptors, tags or package ownership are duplicated or a new
  tag omits the primary exact package prefix
- **THEN** validation fails before candidate assembly

### Requirement: Candidate staging does not activate publication

A candidate-only descriptor SHALL select exact ordered package names, paths,
version, metadata, dependency classes, profiles and resource bounds. Candidate
assembly MAY render release-shaped staged manifests, but canonical package
manifests SHALL retain workspace version `0.0.0` and `publish = false` until a
separate release decision.

#### Scenario: DID candidate is assembled

- **WHEN** the valid candidate-only DID descriptor is selected
- **THEN** exactly `identus-did` then `identus-did-resolver-http` enter isolated
  staging and no canonical publication state changes

#### Scenario: Canonical DID package is activated early

- **WHEN** either selected package overrides the workspace version or publish
  denial before release authorization
- **THEN** the candidate checker fails closed

### Requirement: Candidate archives are deterministic and bounded

The candidate assembler SHALL require an exact current clean Git revision,
build twice in separate scratch outside every Git worktree, compare archive
bytes, reject unsafe or unbounded archive members, validate normalized metadata
and exact dependency requirements, and atomically expose completed evidence.

#### Scenario: Two assemblies agree

- **WHEN** both isolated assemblies complete from one exact revision
- **THEN** every corresponding archive is byte-identical and its SHA-256,
  byte size and bounded file list enter the receipt

#### Scenario: Archive or staging input is unsafe

- **WHEN** a link, special file, path traversal, duplicate member, unknown file,
  oversize input, retained Git/path dependency or non-deterministic archive is
  observed
- **THEN** assembly fails without exposing a completed output

### Requirement: Candidate closure is verified without registry claims

The assembler SHALL extract the exact archives and verify their ordered local
closure through explicit crates.io patches, including declared feature
profiles. The receipt SHALL state that this is not registry resolution,
publication, certification or a stable support promise.

#### Scenario: Local candidate closure passes

- **WHEN** archive manifests carry exact candidate/internal and published
  foundation requirements and every declared profile builds/tests
- **THEN** the receipt reports local patched-closure evidence and no registry
  availability claim

### Requirement: Candidate tooling cannot mutate release state

Candidate tooling SHALL accept no credential and SHALL contain no upload,
publish, tag, release, repository-setting, protected-environment, branch or
consumer mutation operation.

#### Scenario: Candidate succeeds

- **WHEN** deterministic package and closure evidence completes
- **THEN** only local review artifacts exist and every remote/release state
  remains unchanged

