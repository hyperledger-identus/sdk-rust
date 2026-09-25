# release-candidate-trains

## ADDED Requirements

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
