## 1. `#[identus::port]` attribute + port declaration (Phase 1 — foundation)

> **Assumes `add-crypto-capability` landed**: `SecureRandom` port and the `RingSystemRandom`/`DeterministicRandom` adapters exist in their current (un-suffixed) form.

- [x] 1.1 `crates/derive/src/`: add the inert `#[identus::port]` attribute as a `#[proc_macro_attribute]` (`fn port(attr: TokenStream, item: TokenStream) -> TokenStream`). Parse the input as `syn::ItemTrait`; if `ident` ends in `Port`, emit `compile_error!`, otherwise emit the trait unchanged. Add a doc-comment stating the attribute is an inert port-ness marker discovered by the `identus-conformance` naming guard and carries no runtime semantics. `identus-derive` already uses `syn` — add no new dependency.
- [x] 1.2 `crates/crypto/Cargo.toml`: add `identus-derive.workspace = true` under `[dependencies]` (a new, layer-legal proc-macro edge per `crate-ring-layout`; `identus-crypto` does not currently depend on `identus-derive`).
- [x] 1.3 `crates/crypto/src/securerandom.rs`: annotate `SecureRandom` with `#[identus::port]` (`#[identus::port] pub trait SecureRandom { ... }`); update the module doc-comment to note the attribute declares port-ness. Behavior unchanged.
- [x] 1.4 Verify `cargo build -p identus-derive` and `cargo build -p identus-crypto` succeed; verify a temporary `#[identus::port] pub trait StoragePort {}` in a scratch file emits a compile error (then remove it).

## 2. Conformance crate restructure (Phase 2 — structure)

- [x] 2.1 Create `crates/conformance/src/rulebook.rs`: move `Layer`, `Member`, `LayerRule`, the `Layer::as_str` impl, and the `pub(crate) const LAYER_RULES` (lines ~21–196 of the current `lib.rs`) verbatim into it. Keep them `pub(crate)` and runtime-available (NOT `#[cfg(test)]`). Add `#[cfg_attr(not(test), allow(dead_code))]` on items only used by tests, mirroring today's posture.
- [x] 2.2 Create `crates/conformance/src/guard/mod.rs` declared as `#[cfg(test)] mod guard;` from `lib.rs`. Move the shared helpers currently inside the single `mod guard { … }` block (`layer_of`, `is_proc_macro`, `is_outer_or_verification`, `allowed_target_layers`, `check_dep_edge`, `workspace_crate_names`, root-manifest loading, file walking) into `guard/mod.rs`. Add a shared `syn` parse helper (`fn parse_rs(path) -> syn::File`) and a shared source-file walker (`fn walk_src_files() -> Vec<PathBuf>` enumerating `crates/*/src/**/*.rs`).
- [x] 2.3 Create `crates/conformance/src/guard/dep_graph.rs`: move the dep-graph guard test body (layer-membership + dependency-direction assertions) verbatim from the current `mod guard`, retargeted to use the shared helpers in `guard/mod.rs` and the rulebook items re-exported from `rulebook.rs`. The guard's required behavior SHALL be unchanged.
- [x] 2.4 Slim `crates/conformance/src/lib.rs` to a thin root: crate doc-comment, `pub const COMPONENT`, `mod rulebook;` (with `pub(crate) use rulebook::*;` re-export for guards), `#[cfg(test)] mod guard;`, and any needed re-exports. No invariant data, no guard logic inline.
- [x] 2.5 `crates/conformance/Cargo.toml`: add `[dev-dependencies] syn.workspace = true` (NO `[dependencies]` entry for `syn`). Verify the root `Cargo.toml` `[workspace.dependencies]` already has `syn = "2"` (it does, via `identus-derive`).
- [x] 2.6 Verify `cargo test -p identus-conformance` passes with the dep-graph guard green after the move; verify `cargo build -p identus-conformance` (no `--tests`) does not pull `syn` into the production graph.

## 3. Naming guard (Phase 3 — enforcement)

- [x] 3.1 `crates/conformance/src/guard/naming.rs`: implement the naming guard. PASS 1 — derive the port set: walk `crates/*/src/**/*.rs` via the shared walker, `syn::parse_file` each, find `Item::Trait` whose `attrs` (the `Vec<Attribute>` field) contains an attribute whose path is `identus::port`; collect trait names; MAY redundantly assert none ends in `Port` (the authoritative check is compile-time via the attribute). PASS 2 — adapter suffix: walk `crates/adapters-*/src/**/*.rs`, find `Item::Impl` whose implemented trait path matches a discovered port and whose self type is a path to a `pub struct`; assert the struct's last segment ends in `Adapter`. Fail closed on any `syn::parse_file` error.
- [x] 3.2 Guard tests: a positive scenario asserting `SecureRandom` is discovered via `#[identus::port]` on `ItemTrait.attrs` and its name lacks the `Port` suffix; a positive scenario asserting `RingSystemRandomAdapter`/`DeterministicRandomAdapter` (after Phase 4 rename) are found as `impl SecureRandom for …` and end in `Adapter`; a negative scenario with `impl SecureRandom for BadName` asserting the guard fails; a negative scenario asserting a `syn::parse_file` error on a scanned file fails the guard closed.
- [x] 3.3 Verify `cargo test -p identus-conformance` runs both guards (`dep_graph` + `naming`) and both pass.

## 4. Adapter retrofit rename (Phase 4 — retrofit)

- [x] 4.1 `crates/adapters-entropy/src/lib.rs` (behind `ring` feature): rename `pub struct RingSystemRandom` → `pub struct RingSystemRandomAdapter`; rename `impl SecureRandom for RingSystemRandom` → `impl SecureRandom for RingSystemRandomAdapter`. Update the module doc-comment to state the `<Backend><Capability>Adapter` convention and that the deferred wasm adapter will be `GetrandomSystemRandomAdapter`.
- [x] 4.2 `crates/adapters-entropy/src/lib.rs` (behind `deterministic` feature): rename `pub struct DeterministicRandom` → `pub struct DeterministicRandomAdapter`; rename its `impl SecureRandom for …` block accordingly.
- [x] 4.3 Update any in-tree references to the renamed adapters: search `crates/` for `RingSystemRandom`/`DeterministicRandom` identifiers (tests, examples, `identus-bindings` composition root) and retarget to the `*Adapter` names. No un-suffixed struct/impl name SHALL remain.
- [x] 4.4 Verify `cargo build --workspace` succeeds and `cargo test --workspace` passes with both conformance guards green.

## 5. Hardening (Phase 5 — gates)

- [x] 5.1 `cargo fmt --check` passes across the workspace.
- [x] 5.2 `cargo clippy --workspace -- -D warnings` passes (workspace `clippy::all = deny`).
- [x] 5.3 `nix flake check` passes (includes the conformance guard via nextest/crane per `nix-tooling`).
- [x] 5.4 `openspec validate --change add-port-adapter-naming-conventions` passes; the change is ready for `/opsx-verify-change` then archive.