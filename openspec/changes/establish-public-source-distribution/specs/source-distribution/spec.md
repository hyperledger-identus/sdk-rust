# source-distribution Specification

## ADDED Requirements

### Requirement: Alpha Rust consumers use immutable public source

Before a registry candidate is accepted, the SDK SHALL document public HTTPS
Git source at an exact lowercase 40-hex commit as the only supported alpha
distribution channel for `identus-core`, `identus-derive`, `identus-crypto`,
`identus-did`, and `identus-did-resolver-http`. The source SHALL be readable
without repository credentials. A branch, tag, pull-request reference, short
revision, workspace version, or default branch SHALL NOT identify an alpha
artifact.

#### Scenario: Consumer pins an exact public commit

- **WHEN** a Rust consumer declares one of the five packages from the canonical
  public repository with a full exact `rev`
- **THEN** the consumer can resolve the source anonymously and can identify the
  exact SDK commit independently of branch movement

#### Scenario: Consumer selects a mutable reference

- **WHEN** a source-consumption example selects `develop`, another branch, a
  tag, a pull-request reference, or a short revision
- **THEN** the offline distribution check rejects the example

### Requirement: Consumers preserve resolution and integrity evidence

A Cargo consumer SHALL commit the resolved `Cargo.lock`, select features
explicitly, and record the SDK revision and validation evidence for an update.
A Nix consumer SHALL preserve that Cargo resolution and its ordinary locked or
fixed-output source hash. The guide SHALL distinguish these controls from a
registry checksum and SHALL state that crates.io publication of a dependent
package requires registry dependencies instead of Git dependencies.

#### Scenario: Cargo and Nix builds share one SDK identity

- **WHEN** a Nix-managed Rust consumer builds an SDK Git dependency
- **THEN** its manifest, Cargo lock and Nix source evidence resolve the same
  exact SDK commit without a personal credential

### Requirement: Source identity is not release identity

The guide SHALL state that version `0.0.0`, `publish = false`, and an exact
source revision carry no SemVer, support-lifetime, crates.io, binary, Nix
package, FFI or production-release promise. The current consumer compiler floor
SHALL remain Rust 1.98.1 until the separate release-candidate compatibility
decision replaces it.

#### Scenario: Source consumer evaluates a candidate

- **WHEN** a consumer adopts an exact SDK source revision
- **THEN** the adoption record describes a pre-release source dependency and
  does not call it a published or supported SDK release

### Requirement: Compatibility facades remain distinct

The distribution guide SHALL identify the Cargo package names exactly and SHALL
state that NeoPRISM's `identus-apollo` is a downstream compatibility facade,
not an alternate package name for `identus-crypto`.

#### Scenario: Consumer needs Apollo-shaped compatibility

- **WHEN** a consumer retains `identus-apollo` API or feature names
- **THEN** that facade remains in the consumer and delegates to the exact
  `identus-crypto` source dependency without creating an SDK package alias
