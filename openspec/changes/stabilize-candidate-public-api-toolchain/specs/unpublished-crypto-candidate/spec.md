## MODIFIED Requirements

### Requirement: Supply-chain and API evidence bind the exact candidate

The candidate SHALL emit checksums, package sizes, source revision, Rust/Cargo
and release-tool versions, CycloneDX SBOM, public-API baseline/check, feature
profiles, and explicit limitations in deterministic machine-readable evidence.

Public-API evidence SHALL be produced in two explicit stages. The primary
Nix-pinned Rust 1.98.1 Cargo/rustdoc SHALL generate one all-feature
`identus_crypto.json` in disposable candidate scratch with
`RUSTC_BOOTSTRAP=1` scoped only to that command. Locked `cargo-public-api`
SHALL parse that completed JSON file without selecting or installing a rustup
toolchain. A missing or differently identified JSON file SHALL fail closed.

#### Scenario: Evidence cannot be tied to an archive

- **WHEN** an SBOM, API result, or receipt package identity/version differs from
  the corresponding archive
- **THEN** the candidate gate fails closed

#### Scenario: Clean runner has no rustup nightly

- **WHEN** the Nix application prepares public-API evidence without an ambient
  rustup nightly installation
- **THEN** Rust 1.98.1 generates the JSON, the locked parser renders it, and no
  toolchain is installed or selected through rustup

#### Scenario: Rustdoc JSON is absent or ambiguous

- **WHEN** explicit JSON generation does not produce the exact expected
  `identus_crypto.json` regular file
- **THEN** candidate preparation fails before API comparison or receipt output
