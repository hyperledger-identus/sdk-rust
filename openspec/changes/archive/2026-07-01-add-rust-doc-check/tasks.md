## 1. Flake check

- [x] 1.1 Add `nix/checks/rust-doc.nix` defining `checks.rust-doc = craneLib.cargoDoc { inherit cargoArtifacts; src = rustSrc; cargoDocExtraArgs = "--no-deps"; }`
- [x] 1.2 Add `./rust-doc.nix` to the `imports` list in `nix/checks/default.nix`
- [x] 1.3 `git add` the new/modified nix files so Nix can see them (Nix requires tracked paths)

## 2. Verification

- [x] 2.1 `nix build .#checks.x86_64-linux.rust-doc` succeeds on the current (doc-fix-applied) tree
- [x] 2.2 Negative test: temporarily reintroduce a private intra-doc link (e.g. `` [`validate_url`] `` in `crates/core/src/url.rs`), confirm `nix build .#checks.x86_64-linux.rust-doc` fails, then revert
- [x] 2.3 `nix flake check` is green (incl. the new `rust-doc` check) on `x86_64-linux`
- [x] 2.4 Confirm `nix develop -c cargo doc --no-deps` matches the check's behavior (same warning-as-error promotion via workspace lints)

## 3. Spec sync

- [x] 3.1 Sync the `nix-tooling` delta to the main spec via the openspec sync workflow (or confirm deltas are correct for archive) so the main spec gains the "Rust doc check" requirement and the extended "Checks use the stable toolchain" enumeration
- [x] 3.2 `openspec validate add-rust-doc-check --strict` passes