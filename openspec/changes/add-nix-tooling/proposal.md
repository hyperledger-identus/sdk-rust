## Why

`sdk-rust` is a greenfield submodule with no `flake.nix` and no `Cargo.toml`. The workspace `AGENTS.md` already declares that `sdk-rust` owns its own flake (`cd sdk-rust && nix develop -c <command>`), but that flake does not exist, so contributors have neither a reproducible development environment nor any automated checks for the Rust code that will implement the Identus SSI capability. Bootstrapping the Nix tooling now — before any feature code lands — establishes the reproducible dev environment and the check/CI guardrails that every subsequent proposal will rely on, and gives later changes a place to land code.

## What Changes

- Add a `flake.nix` at the submodule root, built with `flake-parts` and mirroring the workspace root flake's input set (`nixpkgs` unstable, `flake-parts`, `devshell`, `rust-overlay`) plus a new `crane` input.
- Add a modular `nix/` directory: `nix/rust-toolchain.nix`, `nix/devshells/{default,nightly}.nix`, `nix/checks/{default,lint-nix,rust-fmt,rust-clippy,rust-test,rust-deny,rust-audit}.nix`, `nix/apps/{default,format,format-nix}.nix`.
- Add a stub Cargo workspace: root `Cargo.toml` (`[workspace]` with no package of its own) and one member crate `crates/identus-ssi/` (`Cargo.toml` + placeholder `src/lib.rs`) so the crane-based checks have a real cargo project to exercise and so future proposals have a landing zone.
- Add `deny.toml` for `cargo-deny` policy (advisories, licenses, bans).
- Add a CI workflow `.github/workflows/nix-checks.yml` that installs Nix and runs `nix flake check` on a matrix of `ubuntu-latest` and `macos-latest`.
- Systems supported: `x86_64-linux` and `aarch64-darwin`.
- Toolchain: stable default (with `wasm32-unknown-unknown` target included); a `.#nightly` devshell attribute is provided as an opt-in but is **not** wired into checks.

## Capabilities

### New Capabilities

- `nix-tooling`: Reproducible Nix-based development environment (devshells), automated checks (nix hygiene + Rust fmt/clippy/test/deny/audit via crane), formatting apps, and CI enforcement for the `sdk-rust` submodule, plus the stub Cargo workspace the checks operate on.

### Modified Capabilities

<!-- None — this is a greenfield submodule with no existing specs. -->

## Impact

- **New files**: `flake.nix`, `flake.lock`, `Cargo.toml`, `crates/identus-ssi/{Cargo.toml,src/lib.rs}`, `deny.toml`, the entire `nix/` tree, `.github/workflows/nix-checks.yml`.
- **New flake inputs**: `crane` (new to this submodule; not added to the workspace root flake). `nixpkgs`, `flake-parts`, `devshell`, `rust-overlay` mirror the workspace root's choices.
- **Systems matrix**: `flake.lock` will contain both linux and darwin paths; CI runs a two-runner matrix. The darwin toolchain must build clean from day one (the stub crate has no platform-sensitive deps, so this is trivial now but becomes a real constraint as deps are added).
- **No runtime/API impact**: this change introduces build/dev tooling and an empty library stub only. No Identus SSI functionality is implemented.
- **Workspace relationship**: the workspace root flake is untouched. `sdk-rust` remains an independent repo with its own flake; the workspace `AGENTS.md` already documents the `cd sdk-rust && nix develop` workflow.
- **CI**: adds the first build/test CI to a repo that currently only has DCO, CodeQL, and file-hygiene workflows.