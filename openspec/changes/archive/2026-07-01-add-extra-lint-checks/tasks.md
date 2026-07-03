## 1. Config files

- [x] 1.1 Add root `taplo.toml` with `align_entries=true`, `column_width=100`, `allowed_blank_lines=1`, `indent_string="  "`, `array_auto_collapse=false`, `array_auto_expand=false`, `compact_arrays=false` (aligned with neoprism).
- [x] 1.2 Add root `.editorconfig-checker.json` with an `Exclude` list covering `.git`, `target`, `node_modules`, and any generated/vendored paths present in the tree.
- [x] 1.3 Review and, if needed, reconcile `.markdownlint.yml` and `.markdownlint-cli2.yaml` with neoprism's rule set and the current tree (ensure `openspec/`, `.pi/`, `.claude/`, `target/`, `node_modules/` are ignored).
- [x] 1.4 Review `.yamllint.yml` and `.editorconfig` for compatibility with the current tree; adjust only where a local check would otherwise fail.

## 2. Devshell tooling

- [x] 2.1 Add `taplo`, `markdownlint-cli2`, `yamllint`, `editorconfig-checker`, and `shellcheck` to `nix/devshells/default.nix` package list.
- [x] 2.2 Verify the devshell still evaluates and enters on `x86_64-linux` (and `aarch64-darwin` where possible) with `nix develop`.

## 3. TOML checks and formatting

- [x] 3.1 Create `nix/checks/lint-toml.nix` as a `stdenv.mkDerivation` mirroring `lint-nix.nix`: filter `src` to `*.toml` files (excluding `target/`, `node_modules/`, `.git`), `nativeBuildInputs = [ taplo ]`, `checkPhase` runs `taplo check` then `taplo format --check` against all matched files, `installPhase = "touch $out"`.
- [x] 3.2 Register `lint-toml` in `nix/checks/default.nix`.
- [x] 3.3 Add `nix/apps/format-toml.nix` as a `writeShellApplication` that runs `taplo format` over all `*.toml` files (excluding generated/vendored paths).
- [x] 3.4 Register `format-toml` in `nix/apps/default.nix`.
- [x] 3.5 Update `nix/apps/format.nix` to run `taplo format` over `*.toml` files between `cargo fmt` and `nixfmt`.

## 4. Text hygiene check

- [x] 4.1 Create `nix/checks/lint-text.nix` as a `stdenv.mkDerivation` mirroring `lint-nix.nix`: filter `src` to relevant file extensions (`.md`, `.yaml`/`.yml`, `.sh`, and editorconfig-relevant text files), `nativeBuildInputs = [ markdownlint-cli2 yamllint editorconfig-checker shellcheck ]`, `checkPhase` runs `markdownlint-cli2`, `yamllint -c .yamllint.yml`, `editorconfig-checker`, and `shellcheck` on shell scripts, excluding generated/vendored paths.
- [x] 4.2 Register `lint-text` in `nix/checks/default.nix`.

## 5. Baseline the tree

- [x] 5.1 Run `nix run .#format` to apply `taplo format` to all `*.toml` files (`Cargo.toml`, `deny.toml`, etc.); commit cosmetic formatting changes.
- [x] 5.2 Fix any markdown violations reported by `markdownlint-cli2` (run `markdownlint-cli2 --fix` from the devshell, then hand-fix the rest).
- [x] 5.3 Fix any editorconfig violations reported by `editorconfig-checker` (trailing whitespace, missing final newline, line endings) or add excludes in `.editorconfig-checker.json` for generated/vendored paths.
- [x] 5.4 Fix any yaml violations reported by `yamllint` against `.yamllint.yml`.
- [x] 5.5 Fix any shellcheck warnings (severity `warning`+) in shell scripts.

## 6. Verification

- [x] 6.1 Run `nix flake check` on `x86_64-linux` and confirm `lint-text`, `lint-toml`, and all pre-existing checks pass.
- [x] 6.2 Confirm `nix run .#format`, `nix run .#format-nix`, and `nix run .#format-toml` all behave as specified (composite + scoped formatting).
- [x] 6.3 Confirm the flake evaluates for both `x86_64-linux` and `aarch64-darwin` (`nix flake show`).
- [x] 6.4 Run `nix run .#format` one final time to confirm the tree is baseline-clean (no-op).