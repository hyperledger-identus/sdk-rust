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

### Requirement: First candidate establishes an explicit API origin

The candidate descriptor SHALL bind one committed public-API baseline per
ordered package and exact evidence-tool versions. An initial candidate SHALL
record SemVer comparison as `not-applicable` and SHALL NOT compare against the
unpublished workspace identity or itself.

#### Scenario: First DID candidate is qualified

- **WHEN** staged all-feature rustdoc JSON is rendered for both packages
- **THEN** exact public-API text matches committed baselines that become the
  comparison origin for later DID candidates

#### Scenario: A meaningless compatibility check is proposed

- **WHEN** the baseline is the same candidate or workspace `0.0.0`
- **THEN** qualification fails rather than reporting a misleading green check

### Requirement: Candidate supply-chain evidence is bounded and identified

The assembler SHALL emit exactly one validated CycloneDX JSON document per
candidate package. Each SHALL bind exact name/version/license, contain no Git
dependency source, remain within configured resource bounds and enter the
atomic receipt by SHA-256 and byte size.

#### Scenario: Evidence matches the candidate

- **WHEN** both SBOMs identify the ordered `0.1.0-rc.1` packages and pass
  structural/dependency/resource validation
- **THEN** the atomic candidate receipt includes both immutable artifact
  identities and the exact generating tools

#### Scenario: Supply-chain evidence drifts

- **WHEN** an SBOM is missing, duplicated, oversized, malformed, names another
  component/version/license, or contains a Git source
- **THEN** qualification fails without exposing completed output

### Requirement: Evidence qualification preserves candidate archives

API and supply-chain evidence SHALL NOT alter staged package inputs or archive
assembly. Normal qualification SHALL reproduce the candidate archive SHA-256
values accepted by the preceding train contract.

#### Scenario: Evidence tools are added

- **WHEN** the qualified candidate is assembled from the same source
- **THEN** both package archive hashes equal the retained #382 values

### Requirement: Local supply-chain evidence is not release attestation

The receipt SHALL distinguish repository license/advisory policy, local SBOM,
API origin, platform evidence, signed provenance and registry availability.
Only the first three are in scope for this slice.

#### Scenario: Qualification succeeds

- **WHEN** local API/SBOM evidence passes
- **THEN** no stable compatibility, vulnerability-free, platform, signed
  provenance, publication or certification claim is made

