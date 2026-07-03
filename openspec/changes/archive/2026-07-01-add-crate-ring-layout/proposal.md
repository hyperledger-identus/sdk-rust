## Why

`add-core-error-conventions` (in flight) establishes `identus-core` as the foundation crate. Before any domain behavior can land, the workspace needs the full crate ring — the named boundaries and the dependency-direction rules that keep adapters, bindings, and conformance outside domain semantics — *and* an executable guard that enforces those rules from the moment any real code lands. The Identus seed branch (`sdk-rust-seed`) defines a 13-crate hexagonal ring with a checked dependency-graph fixture. This change ports that ring as 12 minimal crate stubs (plus the existing `identus-core`), establishes the full intended dependency edges in the manifests, and adds a Rust test that enforces the layer rules against an in-source `LAYER_RULES` const. It is the structural substrate every later domain change fills.

## What Changes

- Create 12 crate stubs at minimal *code* depth (module doc-comment + `COMPONENT` const only): `identus-crypto`, `identus-did`, `identus-trust`, `identus-credentials`, `identus-presentations`, `identus-messaging`, `identus-openid4vc`, `identus-wallet`, `identus-agent`, `identus-adapters`, `identus-bindings`, `identus-conformance`. Short directory names (`crates/crypto`, `crates/did`, …).
- Declare each stub's full intended *inward* dependency edges in its `Cargo.toml` (porting the seed's manifest deps), so the guard has a real ring to enforce — even though the minimal `lib.rs` only uses `identus-core` (via `COMPONENT`). Unused crate deps stay silent because `unused_crate_dependencies` is allow-by-default.
- Add the full 13-entry `[workspace.dependencies]` block to the root `Cargo.toml` (`identus-core = { path = "crates/core" }`, …) so `identus-X.workspace = true` resolves in every stub.
- Encode the layer rulebook as an in-source `pub(crate) const LAYER_RULES` in `identus-conformance`, porting the seed's `layer_rules` + `allowed_target_layers_by_source_layer` + `constraints` (the 7 layers, their `identus-*` crate membership, and each source layer's allowed inward target layers). The seed's generated-snapshot fields (`production_edges`, `dev_edges`, `expected_policy_violations`, `generated_by`, `source_command`) are dropped — this change has no generator and no drift-snapshot job.
- Add a Rust `#[test]` in `identus-conformance` (the guard) that reads `crates/*/Cargo.toml` and the root `Cargo.toml` (via `toml`), identifies workspace-internal dependencies by membership in `[workspace.dependencies]`, and asserts: every workspace-internal `[dependencies]` edge obeys `LAYER_RULES` (no outward dependencies); `identus-core` has no `identus-*` deps; no production crate depends on `identus-adapters`, `identus-bindings`, or `identus-conformance`. The guard reads no `.json` and invokes no subprocess. It runs through the existing crane `rust-test` nix check — no Node, no `serde_json`, no new nix check, no subprocess.
- Add `toml` as `identus-conformance`'s only dev-dependency. No `serde_json`, no cross-crate dev-dependencies (the rulebook is an in-source `const`; the domain crates are stubs with no public contracts to drift against yet).

## Capabilities

### New Capabilities

- `crate-ring-layout`: the 13-crate hexagonal ring (12 stubs plus `identus-core`), the full intended inward dependency edges, the workspace dependency map, and an executable Rust guard that enforces the layer rules — encoded as an in-source `LAYER_RULES` const — against the workspace manifests.

### Modified Capabilities

- `nix-tooling`: no change. The guard is a `#[test]` that piggybacks on the existing crane `rust-test` check and reads only `.toml` files (preserved by `cleanCargoSource`); the `members = ["crates/*"]` glob already covers the new stubs.
- `core-error-conventions`: no change. The stubs only use `identus_core::Component`, already provided by `add-core-error-conventions`.

## Impact

- **New files**: 12 crate stubs (`Cargo.toml` + `src/lib.rs` each) under `crates/`, and a guard `#[test]` module plus the `LAYER_RULES` const in `crates/conformance/src/lib.rs`. No `fixtures/` file is created.
- **Root `Cargo.toml`**: adds the 13-entry `[workspace.dependencies]` block. No version, resolver, or lint change.
- **Dependency**: assumes `add-core-error-conventions` has landed (`identus-core` exists with `Component`).
- **Toolchain**: no Node, no `serde_json`. The guard is pure Rust + `toml`, reading only `.toml` files (preserved by crane's `cleanCargoSource`), running under the existing crane `rust-test` check with no nix config change.
- **No `docs/architecture/` files**: the ring rules live in the OpenSpec spec's `## Purpose` + requirements; per-change decisions in `design.md`.