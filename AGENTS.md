# AGENTS.md — Identus SDK for Rust

## Project Overview

This is the **Identus SDK for Rust** — a Rust SDK for building decentralized identity solutions with the [Identus](https://github.com/hyperledger-identus) ecosystem.

Workspace members:

- `lib/identus-core/` — Core types, traits, and utilities (foundational crate)

## Development Environment

**Nix is recommended.** All commands can be run inside `nix develop`:

```bash
nix develop                    # Enter dev shell
nix develop --unset PATH       # Enter pure dev shell
```

The dev shell provides: `just`, `nixfmt`, `taplo`, text linters, `cargo-llvm-cov`, and the Rust toolchain (managed via `rust-overlay`).

## Rust Guidelines

### Build Commands

- Build: `just build` or `cargo build --all-features`
- Clean: `just clean` or `cargo clean`

### Test Commands

- Run all tests: `just test` or `cargo test --all-features`
- Run single test in crate: `cargo test -p <crate> <test_name>`
- Run single test with full path: `cargo test -p <crate> <module>::test_fn`
- Run integration tests: `cargo test --all-features --test <test_file>`
- Coverage: `just coverage` (LCOV) or `just coverage-html` (HTML report)

### Lint and Format Commands

- Format all: `just format` (Rust, Nix, TOML)
- Format Rust only: `cargo fmt`
- Clippy: `cargo clippy --all-targets -- -D warnings`
- Full check: `just check` (format + build + test)

### Text File Linting

This project uses **nix-provided linters** instead of `npx`. All linter
binaries are declared in `nix/devShells/default.nix` and available inside
`nix develop`. This ensures reproducible versions across all developers
without requiring Node.js, Python, or Homebrew for linting.

**Commands** (run inside `nix develop`):

- Lint all text files: `just lint-text`
- Auto-fix markdown: *(not yet configured; run markdownlint-cli2 manually)*

**Tools and config files:**

| Tool | Config | Purpose |
| --- | --- | --- |
| markdownlint-cli2 | `.markdownlint-cli2.yaml` | Markdown formatting |
| yamllint | `.yamllint.yml` | YAML validation |
| editorconfig-checker | — *(no project config yet)* | Charset (UTF-8 no BOM), line endings (LF), indent |
| shellcheck | — | Shell script analysis |

**Managing linter dependencies:**

Linter packages are declared in `nix/devShells/default.nix` (under the
`# text linters` block). Their versions come from the nixpkgs input pinned
in `flake.lock`.

To check current nix versions:

```bash
nix develop --command bash -c "markdownlint-cli2 --help | head -1; yamllint --version; editorconfig-checker --version; shellcheck --version | head -2"
```

### Code Style

#### Imports

Use `StdExternalCrate` grouping with module-level granularity:

```rust
// Standard library
use std::collections::HashMap;
// External crates
use serde::Deserialize;
use tokio::sync::Mutex;
// Local crates
use crate::error::Error;
```

Run `cargo fmt` to auto-format imports (configured in `rustfmt.toml`).

#### Formatting

- Line width: 120 characters max (configured in `rustfmt.toml`)
- Edition: Rust 2024
- Format doc comments: enabled
- Imports granularity: `Module`
- Import grouping: `StdExternalCrate`

#### Naming Conventions

- Identifiers: `snake_case`
- Types: `CamelCase`
- Choose descriptive, unambiguous names

#### Dependency Versions

In workspace member `Cargo.toml`, always use workspace dependencies:

```toml
[dependencies]
serde = { workspace = true }
tokio = { workspace = true }
```

Do not specify versions in member crates. Add new dependencies to the
`[workspace.dependencies]` section of the root `Cargo.toml`.

#### Error Handling

- Return `Result`/`Option` in public APIs
- Define custom error types in `error.rs` at crate or module level
- Use `thiserror` for error types (preferred over `derive_more` for this project)
- Map errors with context using `.map_err()`; log via `tracing`

Example error definition:

```rust
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("did suffix {suffix} is invalid")]
    InvalidSuffix { suffix: String },
    #[error("failed to parse key data for id {id}")]
    KeyParseError { id: PublicKeyId },
}
```

#### Error Message Style

- **Tone**: short, factual, lowercase start, no trailing period (e.g., "did is not found")
- **Placeholders**: use named placeholders (`{id}`, `{did}`, `{limit}`, `{actual}`, `{expected}`, `{location}`)

**User-facing errors** (public API and service layers):

- Use `#[error("...")]` with descriptive placeholders
- BadRequest (400): preserve error message to help users fix their input, debug formatting (`{:?}`) allowed
- Internal (500): show generic "internal server error", log full error chain via `tracing::error!`

**Developer-facing errors** (library layer, internal processing):

- Debug formatting (`{:?}`) allowed for hashes, binary data, internal IDs
- These appear in logs but are masked before reaching HTTP clients

**Examples**:

```rust
// User-facing (public API errors)
#[error("public key id {id} is invalid")]           // ✅ descriptive placeholder
#[error("did {did} is not found")]                   // ✅ descriptive placeholder

// Developer-facing (library errors, logged only)
#[error("entry with hash {initial_hash:?} exists")] // ✅ {:?} ok for hashes in logs
#[error("block {block_hash:?} tx {tx_idx:?}")]       // ✅ {:?} ok for internal IDs
```

**Quick checklist**:

- starts lowercase
- no trailing period
- placeholder names are descriptive
- `{:?}` only in developer-facing errors (library layer)

#### Logging

Use structured logging via `tracing`:

```rust
tracing::info!(did = %did, "resolving did document");
tracing::error!(error = %e, "failed to connect");
```

Control verbosity with `RUST_LOG=debug` environment variable.

#### Tests

- Place tests in `tests/` directory or next to modules with `#[cfg(test)]`
- Use descriptive test names: `test_create_did_with_valid_input`
- For async tests: `#[tokio::test]`
- Run single test: `cargo test -p identus-core storage_operation`

## General Guidelines

### Commit Conventions

- Format: Conventional Commits without scopes
- Limit: 72 characters
- No secrets in commits
- Examples:
  - `add identus-core crate with basic types`
  - `fix did resolution for revoked keys`
  - `update cargo dependencies`

### Pre-Commit Checklist

Before submitting a PR:

1. `just format` — format all sources
2. `just lint-text` — lint markdown, YAML, editorconfig, shell scripts
3. `just test` — run all tests
4. `cargo clippy --all-targets -- -D warnings` — lint Rust
5. `just check` — full validation (optional but recommended)
