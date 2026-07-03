## 1. Seed the workspace dependency

- [x] 1.1 Add `toml = "0.8"` to root `Cargo.toml` `[workspace.dependencies]`
- [x] 1.2 Change `crates/conformance/Cargo.toml` `[dev-dependencies]` from `toml = "0.8"` to `toml.workspace = true`
- [x] 1.3 Run `cargo build --workspace` and `cargo test -p identus-conformance`; confirm the existing layer-rule guard still passes

## 2. Extend the conformance guard

- [x] 2.1 In `crates/conformance/src/lib.rs` `guard` module, add a helper that, given a parsed manifest, enumerates the external entries across the top-level `[dependencies]`, `[dev-dependencies]`, `[build-dependencies]` tables and every target-specific `[target.'cfg(...)'.dependencies]`, `[target.'cfg(...)'.dev-dependencies]`, and `[target.'cfg(...)'.build-dependencies]` table (identifying "external" as non-`workspace.internal`, non-`path`)
- [x] 2.2 Have the helper expose each enumerated entry's inline fields (presence of `version`, presence of `workspace = true`, `optional`)
- [x] 2.3 Add a `#[test]` (e.g. `no_inline_external_dependency_versions`) that asserts no enumerated external entry in any crate manifest pins `version = "..."` inline; every external entry SHALL carry `workspace = true` resolving to a root `[workspace.dependencies]` entry
- [x] 2.4 Confirm the new test reads no `.json`, invokes no subprocess, and adds no `serde_json` dependency; the guard stays `toml`-dev-dep-only

## 3. Validate the guard against fixtures

- [x] 3.1 Confirm the new test passes on the migrated tree (the seed `toml` now uses `.workspace = true`)
- [x] 3.2 Temporarily revert `crates/conformance/Cargo.toml` to `toml = "0.8"` inline and confirm the new test fails (then restore the workspace form)
- [x] 3.3 Temporarily inject an inline external version under `[build-dependencies]`, under a `[target.'cfg(...)'.dependencies]` section, and under a `[target.'cfg(...)'.dev-dependencies]` section in a scratch crate and confirm each is caught; remove the scratch after

## 4. Lint, checks, and spec hygiene

- [x] 4.1 Run `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo test --workspace`; all pass clean
- [x] 4.2 Run `nix flake check`; passes (the guard runs through the existing crane `rust-test` check with no nix config change)
- [x] 4.3 Verify no new CI job, no Node, and no `serde_json` were introduced; the only manifest delta is the root `[workspace.dependencies]` seed entry plus the conformance reference change