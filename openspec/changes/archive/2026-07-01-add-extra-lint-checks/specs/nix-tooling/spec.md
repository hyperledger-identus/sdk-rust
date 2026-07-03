## MODIFIED Requirements

### Requirement: Reproducible development environment

The submodule SHALL provide a Nix flake at the repo root that yields a reproducible development shell containing a Rust toolchain, C toolchain, cargo dev tooling, Nix hygiene tooling, and file-hygiene/TOML tooling. The flake SHALL be self-contained (not requiring the workspace root flake at runtime).

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

#### Scenario: Devshell includes TOML and file-hygiene tooling

- **WHEN** the default devshell is active
- **THEN** the shell SHALL provide `taplo` (TOML formatter/linter), `markdownlint-cli2`, `yamllint`, `editorconfig-checker`, and `shellcheck`, so contributors can run and auto-fix the file-hygiene checks locally

### Requirement: Formatting apps

The flake SHALL expose `nix run .#format` (runs `cargo fmt`, then `taplo format` on `*.toml`, then `nixfmt` on nix files), `nix run .#format-nix` (runs `nixfmt` on nix files only), and `nix run .#format-toml` (runs `taplo format` on `*.toml` files only).

#### Scenario: format app formats Rust, TOML, and Nix

- **WHEN** a contributor runs `nix run .#format`
- **THEN** `cargo fmt` SHALL be applied to the workspace, `taplo format` SHALL be applied in place to every `*.toml` in the repo (excluding generated/vendored paths), and `nixfmt` SHALL be applied to `flake.nix` and `nix/**/*.nix`

#### Scenario: format-nix app formats only Nix

- **WHEN** a contributor runs `nix run .#format-nix`
- **THEN** `nixfmt` SHALL be applied to `flake.nix` and `nix/**/*.nix` and no Rust or TOML files SHALL be touched

#### Scenario: format-toml app formats only TOML

- **WHEN** a contributor runs `nix run .#format-toml`
- **THEN** `taplo format` SHALL be applied in place to every `*.toml` in the repo (excluding generated/vendored paths) and no Rust or Nix files SHALL be touched

## ADDED Requirements

### Requirement: Text hygiene check

The checks module SHALL include a `lint-text` check that runs `markdownlint-cli2`, `yamllint` (configured by `.yamllint.yml`), `editorconfig-checker` (configured by `.editorconfig` and `.editorconfig-checker.json`), and `shellcheck` against the repo, mirroring the file-hygiene checks enforced by the shared CI `lint-files.yml` reusable workflow. The check SHALL exclude generated and vendored paths (`target/`, `openspec/`, `.pi/`, `.claude/`, `node_modules/`, `.git`).

#### Scenario: Text hygiene check passes on a clean tree

- **WHEN** `nix flake check` is run and all markdown, yaml, shell, and editorconfig rules are satisfied
- **THEN** the `lint-text` check SHALL pass

#### Scenario: Text hygiene check fails on a markdown violation

- **WHEN** a markdown file violates a rule in `.markdownlint.yml`
- **THEN** the `lint-text` check SHALL fail

#### Scenario: Text hygiene check fails on an editorconfig violation

- **WHEN** a tracked file violates `.editorconfig` (e.g. trailing whitespace, wrong line ending, missing final newline) and is not excluded by `.editorconfig-checker.json`
- **THEN** the `lint-text` check SHALL fail

#### Scenario: Text hygiene check fails on a yaml violation

- **WHEN** a yaml file violates `.yamllint.yml`
- **THEN** the `lint-text` check SHALL fail

#### Scenario: Text hygiene check ignores generated and vendored paths

- **WHEN** `nix flake check` is run
- **THEN** the `lint-text` check SHALL NOT evaluate files under `target/`, `openspec/`, `.pi/`, `.claude/`, `node_modules/`, or `.git`

### Requirement: TOML hygiene check

The checks module SHALL include a `lint-toml` check that runs `taplo check` (schema/validation) and `taplo format --check` (formatting) against every `*.toml` in the repo, governed by the root `taplo.toml` formatting config. The check SHALL exclude generated/vendored paths (`target/`, `node_modules/`, `.git`).

#### Scenario: TOML hygiene check passes on formatted, valid TOML

- **WHEN** `nix flake check` is run and all `*.toml` files are valid and `taplo format`-clean per `taplo.toml`
- **THEN** the `lint-toml` check SHALL pass

#### Scenario: TOML hygiene check fails on malformed TOML

- **WHEN** a `*.toml` file fails `taplo check` validation
- **THEN** the `lint-toml` check SHALL fail

#### Scenario: TOML hygiene check fails on unformatted TOML

- **WHEN** a `*.toml` file is not formatted according to `taplo.toml`
- **THEN** the `lint-toml` check SHALL fail

#### Scenario: TOML hygiene check ignores generated and vendored paths

- **WHEN** `nix flake check` is run
- **THEN** the `lint-toml` check SHALL NOT evaluate files under `target/`, `node_modules/`, or `.git`

### Requirement: TOML formatting configuration

The submodule SHALL include a root `taplo.toml` that configures taplo formatting with `align_entries = true`, `column_width = 100`, `allowed_blank_lines = 1`, `indent_string = "  "`, `array_auto_collapse = false`, `array_auto_expand = false`, and `compact_arrays = false`, aligned with the neoprism submodule's taplo config for workspace consistency.

#### Scenario: taplo config governs the lint-toml and format-toml commands

- **WHEN** `taplo format` or `taplo format --check` is run in the repo
- **THEN** taplo SHALL read `taplo.toml` from the repo root and apply its formatting rules

### Requirement: Editorconfig-checker exclude configuration

The submodule SHALL include a root `.editorconfig-checker.json` that excludes `.git`, `target`, and any generated/vendored paths from editorconfig-checker, so the `lint-text` check passes on the existing tree.

#### Scenario: editorconfig-checker skips excluded paths

- **WHEN** `editorconfig-checker` is run during the `lint-text` check
- **THEN** paths matching the `Exclude` list in `.editorconfig-checker.json` SHALL NOT be evaluated