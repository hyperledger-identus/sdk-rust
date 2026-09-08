## MODIFIED Requirements

### Requirement: Stub Cargo workspace

The submodule SHALL contain a Cargo workspace rooted at `Cargo.toml` (with `[workspace]` and no package of its own) and one member crate `crates/identus-ssi/` whose `src/lib.rs` is an empty placeholder. The workspace SHALL compile and pass `cargo fmt --check`, `cargo clippy`, and `cargo test` with zero warnings.

#### Scenario: Workspace compiles

- **WHEN** `cargo build` is run in the submodule root
- **THEN** the build SHALL succeed with no warnings

#### Scenario: Workspace is clippy-clean

- **WHEN** `cargo clippy -- -D warnings` is run in the submodule root
- **THEN** clippy SHALL exit zero

#### Scenario: Workspace tests pass

- **WHEN** `cargo test` is run in the submodule root
- **THEN** the test suite SHALL pass (the placeholder crate MAY have no tests)

#### Scenario: Workspace targets edition 2024 with a pinned compiler floor

- **WHEN** the root `Cargo.toml` and member crate manifest are inspected
- **THEN** the workspace SHALL declare `edition = "2024"` and temporary
  `rust-version = "1.98.1"`, and `resolver = "3"` SHALL be set explicitly at
  the workspace level
