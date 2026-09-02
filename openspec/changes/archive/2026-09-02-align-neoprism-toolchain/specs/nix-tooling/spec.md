## RENAMED Requirements

- FROM: `### Requirement: Checks use the stable toolchain`
- TO: `### Requirement: Checks use the pinned NeoPRISM-aligned toolchain`

## MODIFIED Requirements

### Requirement: Reproducible development environment

The repository SHALL provide a self-contained Nix flake whose default devshell
uses the immutable NeoPRISM-etalon baseline: Rust nightly `2026-03-18`,
rust-overlay `f17186f52e82ec5cf40920b58eac63b78692ac7c` and nixpkgs
`c27cdad491a991b11ed731760aa2ef8db0cb0410`. The referenced nixpkgs SHALL
supply Nix package version `2.34.8`. The shell SHALL retain the SDK's C, Cargo,
Nix hygiene and file-hygiene tooling.

#### Scenario: Default devshell provides the pinned Rust toolchain

- **WHEN** a contributor runs `nix develop` from the repository root
- **THEN** the shell SHALL provide `cargo`, `rustc`, `rustfmt`,
  `rust-analyzer`, and `clippy` from Rust nightly `2026-03-18`
- **AND** the toolchain SHALL include the `wasm32-unknown-unknown` target

#### Scenario: Default devshell provides the pinned Nix package

- **WHEN** a contributor runs `nix develop --command nix --version`
- **THEN** the reported Nix version SHALL be `2.34.8`

#### Scenario: Devshell includes C toolchain and crypto build prerequisites

- **WHEN** the default devshell is active
- **THEN** the shell SHALL provide a C compiler (`stdenv.cc`), `pkg-config`, and
  `openssl`, so build scripts of transitive crypto dependencies can compile

#### Scenario: Devshell includes cargo quality tooling

- **WHEN** the default devshell is active
- **THEN** the shell SHALL provide `cargo-nextest`, `cargo-deny`, and
  `cargo-audit`

#### Scenario: Devshell includes Nix hygiene tooling

- **WHEN** the default devshell is active
- **THEN** the shell SHALL provide `nixfmt`, `deadnix`, and `statix`

#### Scenario: Devshell includes workspace-consistency tooling

- **WHEN** the default devshell is active
- **THEN** the shell SHALL provide `just`, `git`, `jq`, `curl`, `which`, `gh`,
  and `cacert`

#### Scenario: Devshell includes TOML and file-hygiene tooling

- **WHEN** the default devshell is active
- **THEN** the shell SHALL provide `taplo`, `markdownlint-cli2`, `yamllint`,
  `editorconfig-checker`, and `shellcheck`, so contributors can run and fix the
  file-hygiene checks locally

### Requirement: Checks use the pinned NeoPRISM-aligned toolchain

All crane-based Rust checks SHALL use Rust nightly `2026-03-18` from the locked
rust-overlay revision. This includes `rust-fmt`, `rust-clippy`, `rust-test`,
`rust-deny`, `rust-audit`, and `rust-doc`. The pin SHALL NOT authorize
nightly-only Rust features or change the workspace's declared MSRV.

#### Scenario: Checks build with the immutable Rust pin

- **WHEN** `nix flake check` is run
- **THEN** every crane-based Rust check SHALL compile with Rust nightly
  `2026-03-18`

#### Scenario: Toolchain updates are explicit

- **WHEN** NeoPRISM advances its Rust or Nix baseline
- **THEN** the SDK SHALL retain its recorded pins until a reviewed dependency
  update records both the old and new immutable NeoPRISM revisions
