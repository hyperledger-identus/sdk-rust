## 1. Stub Cargo workspace

- [ ] 1.1 Create root `Cargo.toml` with `[workspace]` (no package), `members = ["crates/*"]`, `resolver = "2"`, and a `[workspace.package]` block (edition 2021, rust-version stable) for shared metadata
- [ ] 1.2 Create `crates/identus-ssi/Cargo.toml` (lib crate, name `identus-ssi`, version `0.0.0`, inherits from `workspace.package`)
- [ ] 1.3 Create `crates/identus-ssi/src/lib.rs` as an empty placeholder (no items, or a single doc-comment line)
- [ ] 1.4 Add a workspace-level `[workspace.lints.rust]` and `[workspace.lints.clippy]` block (e.g. `warnings = "deny"`) and have the member crate inherit via `lints.workspace = true`, so clippy `-D warnings` is enforced at the cargo level too
- [ ] 1.5 Verify locally: `cargo build`, `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` all pass

## 2. Root flake scaffold

- [ ] 2.1 Create `flake.nix` using `flake-parts.lib.mkFlake`, with inputs `nixpkgs` (nixos-unstable), `flake-parts`, `devshell`, `rust-overlay`, `crane`
- [ ] 2.2 Set `systems = [ "x86_64-linux" "aarch64-darwin" ]`
- [ ] 2.3 In `perSystem`, set `_module.args.pkgs = import nixpkgs { inherit system; overlays = [ (import rust-overlay) ]; }` (mirror workspace root)
- [ ] 2.4 Import `inputs.devshell.flakeModule` and `./nix/rust-toolchain.nix`, `./nix/devshells`, `./nix/checks`, `./nix/apps` via flake-parts `imports`
- [ ] 2.5 Run `nix flake lock` to generate `flake.lock` (both linux and darwin paths)

## 3. Rust toolchain module

- [ ] 3.1 Create `nix/rust-toolchain.nix` exporting a `perSystem` module (or `_module.args`) that defines `toolchain.stable` = `rust-bin.stable.latest.default` with extensions `[rust-src rust-analyzer]` and targets `[wasm32-unknown-unknown]`
- [ ] 3.2 Expose a `craneLib` = `crane.mkLib pkgs` (using the stable toolchain) as a `_module.args` or per-system attr so checks and a future `buildPackage` share it

## 4. Devshells

- [ ] 4.1 Create `nix/devshells/default.nix` defining `devshells.default` (name `sdk-rust`) with: stable rust toolchain, `stdenv.cc`, `pkg-config`, `openssl`, `cargo-nextest`, `cargo-deny`, `cargo-audit`, `protobuf`, `just`, `git`, `jq`, `curl`, `which`, `gh`, `cacert`, `nix`, `nixfmt`, `deadnix`, `statix`
- [ ] 4.2 Set env in the default devshell: `SSL_CERT_FILE = "${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"`, `LANG = "C.utf8"`
- [ ] 4.3 Verify `nix develop -c cargo --version` works

## 5. Checks — Nix hygiene

- [ ] 5.1 Create `nix/checks/lint-nix.nix` mirroring the workspace root's `nix/checks/lint-nix.nix`: a `stdenv.mkDerivation` that runs `deadnix -f`, `statix check .`, and `nixfmt --check flake.nix` + `find nix -name '*.nix' | xargs nixfmt --check`, scoped to `flake.nix` and `nix/**`
- [ ] 5.2 Create `nix/checks/default.nix` wiring `checks.lint-nix`

## 6. Checks — Rust (crane)

- [ ] 6.1 Create `nix/checks/rust-fmt.nix` exposing `checks.rust-fmt = craneLib.cargoFmt { src = craneLib.cleanCargoSource ./.; }`
- [ ] 6.2 Create `nix/checks/rust-clippy.nix` exposing `checks.rust-clippy = craneLib.cargoClippy { ...; cargoClippyExtraArgs = "-- -D warnings"; }` (build deps once via `cargoArtifacts = craneLib.buildDepsOnly { ... }`)
- [ ] 6.3 Create `nix/checks/rust-test.nix` exposing `checks.rust-test` using `craneLib.cargoNextest` (with `cargo-nextest` in nativeBuildInputs) falling back to `craneLib.cargoTest`, sharing the same `cargoArtifacts` as clippy
- [ ] 6.4 Create `nix/checks/rust-deny.nix` exposing `checks.rust-deny = craneLib.cargoDeny { src = ./.; }` (uses `deny.toml`)
- [ ] 6.5 Create `nix/checks/rust-audit.nix` exposing `checks.rust-audit = craneLib.cargoAudit { src = ./.; advisory-db = <pinned input>; }` (pin `advisory-db` flake input)
- [ ] 6.6 Add `advisory-db` input to `flake.nix` (`github:rustsec/advisory-db`) and pass it into `cargoAudit`
- [ ] 6.7 Wire all five rust checks into `nix/checks/default.nix`
- [ ] 6.8 Verify `nix flake check` runs all six checks (lint-nix + 5 rust) and passes

## 7. cargo-deny policy

- [ ] 7.1 Create `deny.toml` with `[advisories]`, `[licenses]` (allow common permissive licenses), `[bans]` sections. Start permissive (deny only known-bad) so the stub workspace passes

## 8. Formatting apps

- [ ] 8.1 Create `nix/apps/format-nix.nix` as a `writeShellApplication` running `nixfmt flake.nix` + `find nix -name '*.nix' -print0 | xargs -0 -r nixfmt` (mirror workspace root `format-nix.nix`)
- [ ] 8.2 Create `nix/apps/format.nix` as a `writeShellApplication` running `cargo fmt` (from the stable toolchain) then the same nixfmt step
- [ ] 8.3 Create `nix/apps/default.nix` wiring `apps.format` and `apps.format-nix`
- [ ] 8.4 Verify `nix run .#format` and `nix run .#format-nix` both work

## 9. CI workflow

- [ ] 9.1 Create `.github/workflows/nix-checks.yml` triggering on `pull_request` and `push` to `main`
- [ ] 9.2 Add a `checks` job with `strategy.matrix.os = [ubuntu-latest, macos-latest]`, `runs-on: ${{ matrix.os }}`
- [ ] 9.3 Steps: checkout, install Nix via `DeterminateSystems/nix-installer-action`, enable `DeterminateSystems/magic-nix-cache-action`, run `nix flake check`
- [ ] 9.4 Pin all third-party GitHub Actions to commit SHAs with version comments (consistent with the existing DCO workflow style)

## 10. Final verification

- [ ] 10.1 Run `nix fmt` / `nix run .#format` to ensure all nix and rust files are formatted
- [ ] 10.2 Run `nix flake check` locally and confirm green (at minimum on linux; darwin if a mac host is available)
- [ ] 10.3 Confirm `nix develop -c cargo build` and `nix develop -c cargo test` succeed
- [ ] 10.4 Push the branch and confirm the `nix-checks` CI workflow runs green on both matrix legs
- [ ] 10.5 Update `README.md` development section to reflect the `nix develop` workflow (replace the bare `cargo build`/`cargo test` instructions)