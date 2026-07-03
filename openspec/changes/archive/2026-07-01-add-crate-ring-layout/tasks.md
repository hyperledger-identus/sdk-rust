## 1. Workspace dependency map

- [x] 1.1 Add `[workspace.dependencies]` to root `Cargo.toml` with all 13 entries: `identus-core = { path = "crates/core" }`, `identus-crypto = { path = "crates/crypto" }`, … `identus-conformance = { path = "crates/conformance" }`
- [x] 1.2 Verify `cargo metadata` resolves all workspace deps without error

## 2. Twelve crate stubs (minimal code depth)

For each of `crypto`, `did`, `trust`, `credentials`, `presentations`, `messaging`, `openid4vc`, `wallet`, `agent`, `adapters`, `bindings`, `conformance`:

- [x] 2.1 Create `crates/<name>/Cargo.toml`: `name = "identus-<name>"`, inherit `version`/`edition`/`rust-version`/`license` from workspace (matching `crates/core/Cargo.toml`), `[dependencies]` = the crate's intended inward edges (per the layer table), `identus-X.workspace = true` form, `[lints] workspace = true`
- [x] 2.2 Create `crates/<name>/src/lib.rs`: module doc-comment + `pub const COMPONENT: identus_core::Component { name: "identus-<name>", summary: "<one-line>" }` only
- [x] 2.3 Verify `cargo build --workspace` passes (unused inward deps stay silent)

Intended inward edges (port from seed):
- `crypto` → core
- `did` → core, crypto
- `trust` → core, crypto, did
- `credentials` → core, crypto, did, trust
- `presentations` → core, credentials, trust
- `messaging` → core, crypto, did
- `openid4vc` → core, credentials, presentations, trust
- `wallet` → core, credentials, crypto, did, messaging, openid4vc, presentations, trust
- `agent` → core
- `adapters` → core, did, messaging, openid4vc, trust, wallet
- `bindings` → core, wallet
- `conformance` → core (prod); `toml` (dev)

## 3. In-source layer rulebook

- [x] 3.1 Add a `pub(crate) const LAYER_RULES` (typed Rust data) to `crates/conformance/src/lib.rs` encoding the 7 layers (foundation, domain-primitives, credential-semantics, protocol-semantics, orchestration, outer-boundary, verification), each layer's `identus-*` crate membership, and each source layer's allowed inward target layers — ported from the seed's `layer_rules` + `allowed_target_layers_by_source_layer` (no generated-snapshot fields)
- [x] 3.2 Ensure every `identus-*` crate appears in exactly one layer's membership in `LAYER_RULES`

## 4. Rust dep-graph guard

- [x] 4.1 Add `toml` to `crates/conformance/Cargo.toml` `[dev-dependencies]` (no `serde_json`)
- [x] 4.2 Add a `#[cfg(test)]` guard module in `crates/conformance/src/lib.rs` that: resolves the workspace root from `env!("CARGO_MANIFEST_DIR")` joined with `../..`; reads the root `Cargo.toml` via `toml` to build the set of 13 workspace crate names from `[workspace.dependencies]`; reads each `crates/*/Cargo.toml` via `toml`; for each crate inspects `[dependencies]`, keeps only targets that are workspace members, maps source and target to layers via `LAYER_RULES`, and asserts each edge's target layer is in the source layer's allowed-inward list; asserts `identus-core` has no `identus-*` dependencies; asserts no production crate (all except `identus-adapters`, `identus-bindings`, `identus-conformance`) depends on `identus-adapters`/`identus-bindings`/`identus-conformance`. No `serde_json`, no `.json` read, no `cargo metadata` subprocess
- [x] 4.3 Add a negative test path (or manual check) confirming the guard fails when an outward dependency is inserted into a stub's `[dependencies]`, then remove it
- [x] 4.4 Verify `cargo test -p identus-conformance` passes

## 5. Whole-workspace verification

- [x] 5.1 `cargo build --workspace`, `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test --workspace` all pass
- [x] 5.2 `nix flake check` green (guard runs under the existing `rust-test` check reading only `.toml` files preserved by `cleanCargoSource`; 12 new stubs covered by `members = ["crates/*"]`; no nix config change)

## 6. OpenSpec artifacts

- [x] 6.1 Finalize `proposal.md`, `design.md`, `specs/crate-ring-layout/spec.md`, `tasks.md`
- [x] 6.2 Run `openspec validate add-crate-ring-layout --strict` and resolve any findings