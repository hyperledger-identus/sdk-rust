## ADDED Requirements

### Requirement: Rust doc check

The checks module SHALL include a `rust-doc` check that builds the workspace's rustdoc via crane's `cargoDoc` with `--no-deps` (documenting only workspace crates, not dependencies), reusing the shared `cargoArtifacts` dependency derivation and `rustSrc`. The check SHALL fail on any rustdoc warning; because the workspace sets `[workspace.lints.rust] warnings = "deny"`, rustdoc lints (including `rustdoc::private_intra_doc_links`) are promoted to errors without additional `RUSTDOCFLAGS`.

#### Scenario: Doc check passes on warning-free documentation

- **WHEN** `nix flake check` is run and the workspace's rustdoc builds with no warnings
- **THEN** the `rust-doc` check SHALL pass

#### Scenario: Doc check fails on a private intra-doc link

- **WHEN** a public doc comment contains an intra-doc link to a private item (e.g. a link to a private `validate_*` helper function)
- **THEN** the `rust-doc` check SHALL fail with a `rustdoc::private_intra_doc_links` error

#### Scenario: Doc check reuses the shared dependency derivation

- **WHEN** `nix flake check` runs the `rust-doc` check alongside the other crane-based checks
- **THEN** the `rust-doc` check SHALL consume the shared `cargoArtifacts` derivation rather than recompiling dependencies

#### Scenario: Doc check documents only workspace crates

- **WHEN** the `rust-doc` check runs `cargoDoc`
- **THEN** it SHALL pass `--no-deps` so dependency crates are not documented

## MODIFIED Requirements

### Requirement: Checks use the stable toolchain

All crane-based Rust checks (`rust-fmt`, `rust-clippy`, `rust-test`, `rust-deny`, `rust-audit`, `rust-doc`) SHALL build with the stable Rust toolchain.

#### Scenario: Checks build on stable

- **WHEN** `nix flake check` is run
- **THEN** every crane-based Rust check SHALL compile against the stable Rust toolchain