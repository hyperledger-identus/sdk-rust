# unpublished-crypto-candidate

## ADDED Requirements

### Requirement: Release publication workspace is VCS-independent

The release-eligible candidate SHALL construct and lock its publication
workspace outside every Git worktree, then copy only the clean source workspace
into evidence. Repackaging SHALL reproduce the reviewed archive checksums even
when the requested evidence directory is inside the canonical checkout.

#### Scenario: Workflow writes evidence below the checkout

- **WHEN** candidate preparation targets an in-repository `artifacts` directory
- **THEN** the copied publication workspace contains no ambient Cargo VCS
  metadata or build output and publisher verification reproduces all reviewed
  archive bytes

## MODIFIED Requirements

### Requirement: Candidate scope and brand are explicit

The SDK SHALL identify `identus-crypto` as the durable Rust cryptography package
and SHALL prepare `identus-derive`, `identus-core`, and `identus-crypto` together
at `0.1.0-rc.1`. It SHALL NOT generate `identus-apollo` or version/publish any
unrelated workspace member. After explicit release activation, the three
selected canonical manifests SHALL carry their reviewed version and package
metadata while the workspace default remains `0.0.0` and unpublished.

#### Scenario: Candidate descriptor is evaluated

- **WHEN** the candidate gate reads the committed descriptor and canonical
  manifests
- **THEN** exactly the three approved package identities and exact internal
  prerelease requirements enter staging

### Requirement: Unpublished dependency closure is verified honestly

Before namespace creation, the gate SHALL extract the exact candidate archives
and build/test the closure and a minimal `identus-crypto` consumer using local
patches only for the three not-yet-indexed package versions. It SHALL record
that this is archive-closure evidence rather than registry resolution. The
protected publisher SHALL then let Cargo verify each package against crates.io
in dependency order as its prerequisites become available.

#### Scenario: Normalized archive retains a workspace path

- **WHEN** an internal dependency lacks exact `=0.1.0-rc.1` registry metadata
  or a Git/path selector survives normalized inspection
- **THEN** candidate verification fails before producing a success receipt

### Requirement: Preparation cannot publish

Candidate tooling SHALL contain no registry credential, upload, publish, tag,
release, or `main` mutation operation. A successful candidate SHALL become
release-eligible only through the separately tested protected publisher and
human controls defined by `crate-release-trains`.

#### Scenario: Candidate succeeds

- **WHEN** every local candidate check and protected PR gate passes
- **THEN** the result is a review artifact and no public release lifecycle
  state changes until the protected publisher is independently authorized
