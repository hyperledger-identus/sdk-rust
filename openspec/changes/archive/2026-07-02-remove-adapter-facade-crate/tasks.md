## 1. Code removal

- [x] 1.1 Delete `crates/adapters/` (the `identus-adapters` crate: `Cargo.toml` + `src/lib.rs`)
- [x] 1.2 Drop `identus-adapters = { path = "crates/adapters" }` from root `Cargo.toml` `[workspace.dependencies]`

## 2. Conformance guard

- [x] 2.1 Remove the `identus-adapters` `Member` entry from the `outer-boundary` `LayerRule` in `LAYER_RULES` (`crates/conformance/src/lib.rs`)
- [x] 2.2 Retarget the outward-dependency-rejection test assertion that names `identus-adapters` (the `Guard fails on an outward dependency to a non-proc-macro crate` scenario's `#[test]`) to `identus-adapters-entropy`
- [x] 2.3 Run `cargo test -p identus-conformance` and confirm green (counts derive from `LAYER_RULES`, so no literal edit is needed)

## 3. Workspace verification

- [x] 3.1 Run `cargo build --workspace` and confirm no crate references the removed `identus-adapters` name
- [x] 3.2 Run `cargo clippy --workspace -- -D warnings` and confirm clean
- [x] 3.3 Run `nix flake check` (or the crane `rust-test` check) and confirm the dep-graph guard + lint + format pass

## 4. Spec delta reconciliation (at archive/sync)

- [x] 4.1 Reconcile the `crate-ring-layout` `## Purpose` layer table: drop `identus-adapters` from the `outer-boundary` row (and, while there, address the already-stale absence of `identus-adapters-entropy`)
- [x] 4.2 Reconcile the `## Purpose` bullet "Production crates ... SHALL NOT depend on `identus-adapters`, `identus-bindings`, or `identus-conformance`" to drop `identus-adapters` (replacing with the `identus-adapters-<family>` framing as appropriate)
- [x] 4.3 Update the `## Purpose` "13-crate" count prose to reflect the post-removal crate count
- [x] 4.4 Add `identus-derive` to the `## Purpose` layer table's `foundation` row and reframe the count prose to distinguish the 13-crate runtime ring from the build-time `proc-macro` member, so the table matches `LAYER_RULES` membership (per "In-source layer rulebook")