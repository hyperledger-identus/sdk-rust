# unpublished-crypto-candidate Specification

## ADDED Requirements

### Requirement: Candidate scope and brand are explicit

The SDK SHALL identify `identus-crypto` as the durable Rust cryptography package
and SHALL prepare `identus-derive`, `identus-core`, and `identus-crypto` together
at `0.1.0-rc.1`. It SHALL NOT generate `identus-apollo`, version unrelated
workspace members, or change canonical `publish = false` policy.

#### Scenario: Candidate descriptor is evaluated

- **WHEN** the candidate gate reads the committed descriptor
- **THEN** exactly the three approved package identities and exact internal
  prerelease requirements enter staging

### Requirement: Package archives are deterministic and self-describing

The SDK SHALL assemble each candidate twice from the same exact source revision
and SHALL reject differing bytes, unexpected/unbounded contents, missing
metadata/license/README, Git dependencies, path-only normalized dependencies,
or checksum drift.

#### Scenario: Same source is packaged twice

- **WHEN** both independent staging runs complete
- **THEN** every corresponding `.crate` archive has the same SHA-256 digest and
  bounded byte size

### Requirement: Unpublished dependency closure is verified honestly

The gate SHALL extract the exact candidate archives and build/test the closure
and a minimal `identus-crypto` consumer using local patches only for the three
unpublished package versions. It SHALL record that this is not registry
resolution or `cargo publish --dry-run` evidence.

#### Scenario: Normalized archive retains a workspace path

- **WHEN** an internal dependency lacks exact `=0.1.0-rc.1` registry metadata or
  a Git/path selector survives normalized inspection
- **THEN** candidate verification fails before producing a success receipt

### Requirement: Supply-chain and API evidence bind the exact candidate

The candidate SHALL emit checksums, package sizes, source revision, Rust/Cargo
and release-tool versions, CycloneDX SBOM, public-API baseline/check, feature
profiles, and explicit limitations in deterministic machine-readable evidence.

#### Scenario: Evidence cannot be tied to an archive

- **WHEN** an SBOM, API result, or receipt package identity/version differs from
  the corresponding archive
- **THEN** the candidate gate fails closed

### Requirement: Preparation cannot publish

Candidate tooling SHALL contain no registry credential, upload, publish, tag,
release, or `main` mutation operation. Rust 1.98.1 evidence SHALL be labeled as
candidate preparation only; a consumer-driven compiler decision and human
release controls remain required before publication.

#### Scenario: Candidate succeeds

- **WHEN** every local candidate check and protected PR gate passes
- **THEN** the result is an unpublished review artifact and no public release
  lifecycle state changes
