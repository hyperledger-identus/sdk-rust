## Why

`sdk-rust` already enforces Rust and Nix hygiene locally via `nix flake check`, but the additional file-hygiene checks that CI runs (markdown-lint, editorconfig, yaml lint) are only enforced through the shared remote `hyperledger-identus/.github` reusable workflow. Contributors therefore discover those failures only after pushing, cannot self-verify them locally, and have no way to auto-fix them. Additionally, TOML files in the repo (`Cargo.toml`, `deny.toml`, future manifests) are not formatted or linted by any local tool, so formatting drift goes unnoticed until review. Bringing these checks into the flake closes the gap between local `nix flake check` and real CI, and matches the in-workspace pattern already proven by `neoprism` (taplo + text linters in the devshell).

## What Changes

- Add a `lint-text` flake check that runs markdownlint-cli2, yamllint, and editorconfig-checker against the repo, mirroring the shared `lint-files.yml` CI hygiene job so `nix flake check` reproduces CI locally.
- Add a `lint-toml` flake check that runs `taplo check` (validation) and `taplo format --check` (formatting) against all `*.toml` files, using a new `taplo.toml` config aligned with neoprism's settings.
- Extend the `format` app to also run `taplo format` on `*.toml` files (in-place), and add a `format-toml` app for TOML-only formatting, paralleling the existing `format-nix` app.
- Add the required tools (`taplo`, `markdownlint-cli2`, `yamllint`, `editorconfig-checker`, `shellcheck`) to the default devshell so contributors can run and fix these checks locally.
- Add a `taplo.toml` config at the repo root governing TOML formatting.
- Add an `.editorconfig-checker.json` exclude file (excluding `.git`, `target`, generated/vendored paths) so editorconfig-checker passes on the existing tree.
- Reconcile existing root lint configs (`.markdownlint.yml`, `.markdownlint-cli2.yaml`, `.yamllint.yml`, `.editorconfig`) with neoprism's where needed so the local checks match what CI enforces.
- No changes to the existing Rust or Nix checks; this is purely additive.

## Capabilities

### New Capabilities

(none)

### Modified Capabilities

- `nix-tooling`: Adds new flake checks (`lint-text`, `lint-toml`), extends the devshell toolset with text/TOML lint tools, extends the `format` app with TOML formatting and adds a `format-toml` app, and introduces the `taplo.toml` and `.editorconfig-checker.json` config files. Existing requirements for the devshell, formatting apps, and the checks surface change to include non-Rust/non-Nix file hygiene.

## Impact

- **Nix flake**: `nix/checks/` gains two new check modules; `nix/apps/` gains a `format-toml` app and the `format` app is updated; `nix/devshells/default.nix` gains new packages.
- **New config files**: `taplo.toml`, `.editorconfig-checker.json` at repo root.
- **Existing config files**: `.markdownlint.yml`, `.markdownlint-cli2.yaml`, `.yamllint.yml`, `.editorconfig` may receive minor adjustments to align with CI/neoprism and to ensure the new local checks pass on the current tree.
- **TOML files**: `Cargo.toml`, `deny.toml`, and any other `*.toml` will be reformatted by `taplo format` as part of implementation; formatting changes only.
- **CI**: `.github/workflows/nix-checks.yml` is unchanged (it already runs `nix flake check`); `.github/workflows/file-hygiene.yml` is unchanged (the shared remote job remains the source of truth for CI, now mirrored locally). No new CI jobs are required.
- **Dependencies**: New nixpkgs packages pulled into the flake's devshell/checks closure (`taplo`, `markdownlint-cli2`, `yamllint`, `editorconfig-checker`, `shellcheck`). All are already available in nixpkgs and used by neoprism.
- **No API/runtime impact**: this is tooling-only; no Rust crate behavior changes.