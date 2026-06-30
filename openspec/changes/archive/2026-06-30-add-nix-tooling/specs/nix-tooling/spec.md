## ADDED Requirements

### Requirement: Reproducible development environment

The submodule SHALL provide a Nix flake at the repo root that yields a reproducible development shell containing a Rust toolchain, C toolchain, cargo dev tooling, and Nix hygiene tooling. The flake SHALL be self-contained (not requiring the workspace root flake at runtime).

#### Scenario: Default devshell provides a stable Rust toolchain

- **WHEN** a contributor runs `nix develop` (or `nix develop .#default`) from the submodule root
- **THEN** the shell SHALL provide `cargo`, `rustc`, `rustfmt`, `rust-analyzer`, and `clippy` from the stable Rust toolchain, plus the `wasm32-unknown-unknown` rustc target

#### Scenario: Devshell includes C toolchain and crypto build prerequisites

- **WHEN** the default devshell is active
- **THEN** the shell SHALL provide a C compiler (`stdenv.cc`), `pkg-config`, and `openssl`, so that build scripts of transitive crypto dependencies can compile

#### Scenario: Devshell includes cargo quality tooling

- **WHEN** the default devshell is active
- **THEN** the shell SHALL provide `cargo-nextest`, `cargo-deny`, and `cargo-audit`

#### Scenario: Devshell includes Nix hygiene tooling

- **WHEN** the default devshell is active
- **THEN** the shell SHALL provide `nixfmt`, `deadnix`, and `statix`

#### Scenario: Devshell includes workspace-consistency tooling

- **WHEN** the default devshell is active
- **THEN** the shell SHALL provide `just`, `git`, `jq`, `curl`, `which`, `gh`, and `cacert`

### Requirement: Multi-system support

The flake SHALL target both `x86_64-linux` and `aarch64-darwin` systems, with per-system outputs (devshells, checks, apps) evaluated for each.

#### Scenario: Flake evaluates for both supported systems

- **WHEN** `nix flake show` is run against the flake
- **THEN** outputs SHALL be present for both `x86_64-linux` and `aarch64-darwin`

#### Scenario: Default devshell is enterable on darwin

- **WHEN** a contributor runs `nix develop` on an `aarch64-darwin` host
- **THEN** the shell SHALL enter successfully and provide the same toolset as the linux devshell, built for darwin

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

#### Scenario: Workspace targets edition 2024 with a pinned MSRV

- **WHEN** the root `Cargo.toml` and member crate manifest are inspected
- **THEN** the workspace SHALL declare `edition = "2024"` and `rust-version = "1.85.0"` (the edition 2024 floor), and `resolver = "3"` SHALL be set explicitly at the workspace level

### Requirement: Nix hygiene check

The checks module SHALL include a `lint-nix` check that runs `deadnix -f`, `statix check .`, and `nixfmt --check` against `flake.nix` and `nix/**/*.nix`, mirroring the workspace root's `nix/checks/lint-nix.nix` idiom.

#### Scenario: Nix hygiene check passes on well-formed nix

- **WHEN** `nix flake check` is run
- **THEN** the `lint-nix` check SHALL pass when nix files are formatted and free of dead code and statix warnings

#### Scenario: Nix hygiene check fails on unformatted nix

- **WHEN** a nix file under `nix/` is not `nixfmt`-formatted
- **THEN** the `lint-nix` check SHALL fail

### Requirement: Rust format check

The checks module SHALL include a `rust-fmt` check that runs `cargo fmt --check` across the workspace via crane's `cargoFmt`.

#### Scenario: Rust format check passes on formatted code

- **WHEN** `nix flake check` is run and all Rust source is `cargo fmt`-formatted
- **THEN** the `rust-fmt` check SHALL pass

#### Scenario: Rust format check fails on unformatted code

- **WHEN** any Rust source file is not `cargo fmt`-formatted
- **THEN** the `rust-fmt` check SHALL fail

### Requirement: Rust clippy check

The checks module SHALL include a `rust-clippy` check that runs `cargo clippy -- -D warnings` across the workspace via crane's `cargoClippy`, with dependencies built once and shared with other crane-based checks.

#### Scenario: Clippy check passes on warning-free code

- **WHEN** `nix flake check` is run and the workspace is clippy-clean
- **THEN** the `rust-clippy` check SHALL pass

#### Scenario: Clippy check fails on any warning

- **WHEN** `cargo clippy` emits any warning
- **THEN** the `rust-clippy` check SHALL fail

### Requirement: Rust test check

The checks module SHALL include a `rust-test` check that runs the workspace's test suite via crane (using `cargoNextest` when available, falling back to `cargoTest`).

#### Scenario: Test check passes when tests pass

- **WHEN** `nix flake check` is run and `cargo test` passes
- **THEN** the `rust-test` check SHALL pass

#### Scenario: Test check fails when a test fails

- **WHEN** any test in the workspace fails
- **THEN** the `rust-test` check SHALL fail

### Requirement: Cargo deny check

The checks module SHALL include a `rust-deny` check that runs `cargo deny` with policy from `deny.toml` covering advisories, licenses, and bans, via crane's `cargoDeny`.

#### Scenario: Deny check passes on policy-compliant deps

- **WHEN** `nix flake check` is run and the dependency graph satisfies `deny.toml`
- **THEN** the `rust-deny` check SHALL pass

#### Scenario: Deny check fails on a policy violation

- **WHEN** the dependency graph contains a banned crate, a disallowed license, or a known advisory
- **THEN** the `rust-deny` check SHALL fail

### Requirement: Cargo audit check

The checks module SHALL include a `rust-audit` check that runs `cargo audit` against a pinned RustSec advisory database via crane's `cargoAudit`.

#### Scenario: Audit check passes with no known vulnerabilities

- **WHEN** `nix flake check` is run and no dependency has a known RustSec advisory
- **THEN** the `rust-audit` check SHALL pass

#### Scenario: Audit check fails on a known vulnerability

- **WHEN** a dependency has a known, unignored RustSec advisory
- **THEN** the `rust-audit` check SHALL fail

### Requirement: Formatting apps

The flake SHALL expose `nix run .#format` (runs `cargo fmt` then `nixfmt` on nix files) and `nix run .#format-nix` (runs `nixfmt` on nix files only).

#### Scenario: format app formats Rust and Nix

- **WHEN** a contributor runs `nix run .#format`
- **THEN** `cargo fmt` SHALL be applied to the workspace and `nixfmt` SHALL be applied to `flake.nix` and `nix/**/*.nix`

#### Scenario: format-nix app formats only Nix

- **WHEN** a contributor runs `nix run .#format-nix`
- **THEN** `nixfmt` SHALL be applied to `flake.nix` and `nix/**/*.nix` and no Rust files SHALL be touched

### Requirement: CI enforcement of flake checks

The submodule SHALL include a `.github/workflows/nix-checks.yml` workflow that installs Nix and runs `nix flake check` on a matrix of `ubuntu-latest` and `macos-latest`, on pull requests and on pushes to `main`.

#### Scenario: CI runs flake check on pull requests

- **WHEN** a pull request is opened or updated
- **THEN** the `nix-checks` workflow SHALL run `nix flake check` on both `ubuntu-latest` and `macos-latest`

#### Scenario: CI runs flake check on push to main

- **WHEN** a commit is pushed to `main`
- **THEN** the `nix-checks` workflow SHALL run `nix flake check` on both matrix legs

#### Scenario: CI blocks merge on a failing check

- **WHEN** any check in `nix flake check` fails on either matrix leg
- **THEN** the `nix-checks` workflow SHALL report a failing status for that leg

### Requirement: Checks use the stable toolchain

All crane-based Rust checks (`rust-fmt`, `rust-clippy`, `rust-test`, `rust-deny`, `rust-audit`) SHALL build with the stable Rust toolchain.

#### Scenario: Checks build on stable

- **WHEN** `nix flake check` is run
- **THEN** every crane-based Rust check SHALL compile against the stable Rust toolchain

### Requirement: Crane dependency caching

The crane-based checks SHALL share a single dependency derivation so that dependencies are compiled once and reused across `rust-clippy`, `rust-test`, and any future `cargoDoc`/`buildPackage` consumers, rather than recompiling per check.

#### Scenario: Dependencies are not recompiled per check

- **WHEN** `nix flake check` runs multiple crane-based checks
- **THEN** the shared dependency derivation SHALL be built once and reused across checks