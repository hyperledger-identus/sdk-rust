## 1. Stub Cargo workspace

- [x] 1.1 Create root `Cargo.toml` with `[workspace]` (no package), `members = ["crates/*"]`, `resolver = "3"` (kept explicit; edition 2024 implies it but explicitness aids readers), and a `[workspace.package]` block (`edition = "2024"`, `rust-version = "1.85.0"` — the edition 2024 floor) for shared metadata
- [x] 1.2 Create `crates/identus-ssi/Cargo.toml` (lib crate, name `identus-ssi`, version `0.0.0`, inherits from `workspace.package`)
- [x] 1.3 Create `crates/identus-ssi/src/lib.rs` as an empty placeholder (no items, or a single doc-comment line)
- [x] 1.4 Add a workspace-level `[workspace.lints.rust]` and `[workspace.lints.clippy]` block (e.g. `warnings = "deny"`) and have the member crate inherit via `lints.workspace = true`, so clippy `-D warnings` is enforced at the cargo level too
- [x] 1.5 Verify locally: `cargo build`, `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` all pass

## 2. Root flake scaffold

- [x] 2.1 Create `flake.nix` using `flake-parts.lib.mkFlake`, with inputs `nixpkgs` (nixos-unstable), `flake-parts`, `devshell`, `rust-overlay`, `crane`
- [x] 2.2 Set `systems = [ "x86_64-linux" "aarch64-darwin" ]`
- [x] 2.3 In `perSystem`, set `_module.args.pkgs = import nixpkgs { inherit system; overlays = [ (import rust-overlay) ]; }` (mirror workspace root)
- [x] 2.4 Import `inputs.devshell.flakeModule` and `./nix/rust-toolchain.nix`, `./nix/devshells`, `./nix/checks`, `./nix/apps` via flake-parts `imports`
- [x] 2.5 Run `nix flake lock` to generate `flake.lock` (both linux and darwin paths)

## 3. Rust toolchain module

- [x] 3.1 Create `nix/rust-toolchain.nix` exporting a `perSystem` module (or `_module.args`) that defines `toolchain.stable` = `rust-bin.stable.latest.default` with extensions `[rust-src rust-analyzer]` and targets `[wasm32-unknown-unknown]`
- [x] 3.2 Expose a `craneLib` = `crane.mkLib pkgs` (using the stable toolchain) as a `_module.args` or per-system attr so checks and a future `buildPackage` share it

## 4. Devshells

- [x] 4.1 Create `nix/devshells/default.nix` defining `devshells.default` (name `sdk-rust`) with: stable rust toolchain, `stdenv.cc`, `pkg-config`, `openssl`, `cargo-nextest`, `cargo-deny`, `cargo-audit`, `protobuf`, `just`, `git`, `jq`, `curl`, `which`, `gh`, `cacert`, `nix`, `nixfmt`, `deadnix`, `statix`
- [x] 4.2 Set env in the default devshell: `SSL_CERT_FILE = "${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"`, `LANG = "C.utf8"`
- [x] 4.3 Verify `nix develop -c cargo --version` works

## 5. Checks — Nix hygiene

- [x] 5.1 Create `nix/checks/lint-nix.nix` mirroring the workspace root's `nix/checks/lint-nix.nix`: a `stdenv.mkDerivation` that runs `deadnix -f`, `statix check .`, and `nixfmt --check flake.nix` + `find nix -name '*.nix' | xargs nixfmt --check`, scoped to `flake.nix` and `nix/**`
- [x] 5.2 Create `nix/checks/default.nix` wiring `checks.lint-nix`

## 6. Checks — Rust (crane)

- [x] 6.1 Create `nix/checks/rust-fmt.nix` exposing `checks.rust-fmt = craneLib.cargoFmt { src = craneLib.cleanCargoSource ./.; }`
- [x] 6.2 Create `nix/checks/rust-clippy.nix` exposing `checks.rust-clippy = craneLib.cargoClippy { ...; cargoClippyExtraArgs = "-- -D warnings"; }` (build deps once via `cargoArtifacts = craneLib.buildDepsOnly { ... }`)
- [x] 6.3 Create `nix/checks/rust-test.nix` exposing `checks.rust-test` using `craneLib.cargoNextest` (with `cargo-nextest` in nativeBuildInputs) falling back to `craneLib.cargoTest`, sharing the same `cargoArtifacts` as clippy
- [x] 6.4 Create `nix/checks/rust-deny.nix` exposing `checks.rust-deny = craneLib.cargoDeny { src = ./.; }` (uses `deny.toml`)
- [x] 6.5 Create `nix/checks/rust-audit.nix` exposing `checks.rust-audit = craneLib.cargoAudit { src = ./.; advisory-db = <pinned input>; }` (pin `advisory-db` flake input)
- [x] 6.6 Add `advisory-db` input to `flake.nix` (`github:rustsec/advisory-db`) and pass it into `cargoAudit`
- [x] 6.7 Wire all five rust checks into `nix/checks/default.nix`
- [x] 6.8 Verify `nix flake check` runs all six checks (lint-nix + 5 rust) and passes

## 7. cargo-deny policy

- [x] 7.1 Create `deny.toml` with `[advisories]`, `[licenses]` (allow common permissive licenses), `[bans]` sections. Start permissive (deny only known-bad) so the stub workspace passes

## 8. Formatting apps

- [x] 8.1 Create `nix/apps/format-nix.nix` as a `writeShellApplication` running `nixfmt flake.nix` + `find nix -name '*.nix' -print0 | xargs -0 -r nixfmt` (mirror workspace root `format-nix.nix`)
- [x] 8.2 Create `nix/apps/format.nix` as a `writeShellApplication` running `cargo fmt` (from the stable toolchain) then the same nixfmt step
- [x] 8.3 Create `nix/apps/default.nix` wiring `apps.format` and `apps.format-nix`
- [x] 8.4 Verify `nix run .#format` and `nix run .#format-nix` both work

## 9. CI workflow

- [x] 9.1 Create `.github/workflows/nix-checks.yml` triggering on `pull_request` and `push` to `main`
- [x] 9.2 Add a `checks` job with `strategy.matrix.os = [ubuntu-latest, macos-latest]`, `runs-on: ${{ matrix.os }}`
- [x] 9.3 Steps: checkout, install Nix via `DeterminateSystems/nix-installer-action`, enable `DeterminateSystems/magic-nix-cache-action`, run `nix flake check`
- [x] 9.4 Pin all third-party GitHub Actions to commit SHAs with version comments (consistent with the existing DCO workflow style)

## 10. Final verification

- [x] Run `nix fmt` / `nix run .#format` to ensure all nix and rust files are formatted
- [x] Run `nix flake check` locally and confirm green (at minimum on linux; darwin if a mac host is available)
- [x] Confirm `nix develop -c cargo build` and `nix develop -c cargo test` succeed
- [x] 10.4 Push the branch and confirm the `nix-checks` CI workflow runs green on both matrix legs
- [x] Update `README.md` development section to reflect the `nix develop` workflow (replace the bare `cargo build`/`cargo test` instructions)