## Why

The workspace already mandates its *internal* `identus-*` dependencies at the workspace level (per `crate-ring-layout`'s "Workspace dependency map" requirement), but **external / third-party dependencies are unregulated**: the one external dep in the tree (`toml = "0.8"` in `identus-conformance`'s `[dev-dependencies]`) is pinned inline, per-crate, and is invisible to the existing dep-graph guard (which reads only `[dependencies]` and filters to workspace-internal crates). With `add-crypto-capability` about to introduce ~8 real third-party crypto crates, ad-hoc inline versioning would fragment immediately — version drift across crates, no single source of truth for upgrades, and no mechanical way to prevent two crates pinning different versions of the same primitive. This change establishes that *all* external dependencies live in `[workspace.dependencies]` and are referenced via `X.workspace = true`, enforced by extending the existing conformance guard — ahead of the crypto workload, so those deps enter an already-single-source graph.

## What Changes

- Mandate every external (non-`workspace.internal`, non-`path`) dependency be declared in root `[workspace.dependencies]` and referenced from crates via `<dep>.workspace = true`. Per-crate `features` are still allowed on top of the workspace entry; the `version` (and `default-features`) is declared once, at workspace level.
- Declare `default-features` at workspace level when it is set (single source for the off-state too), with a documented per-crate override escape hatch for the rare crate that needs a different default-feature set.
- Extend the `identus-conformance` dep-graph guard (from `crate-ring-layout`) to enumerate the top-level `[dependencies]`, `[dev-dependencies]`, and `[build-dependencies]` sections plus their target-specific `[target.'cfg(...)'.dependencies|dev-dependencies|build-dependencies]` variants, and to reject any external entry that pins `version = "..."` inline (including `optional = true` entries) instead of resolving to a root `[workspace.dependencies]` entry via `.workspace = true`.
- Migrate the existing seed case — `toml = "0.8"` in `identus-conformance`'s `[dev-dependencies]` — to root `[workspace.dependencies]` and reference it via `toml.workspace = true`. This is the only external dep in the tree today, so the migration is one entry.
- No change to the internal `identus-*` workspace-dependency map (already mandated by `crate-ring-layout`); this governs only external crates.

## Capabilities

### New Capabilities

- `workspace-dependency-conventions`: the policy that every external dependency is declared once in root `[workspace.dependencies]` and referenced from crates via `.workspace = true` (version and `default-features` single-sourced at workspace level; per-crate `features` permitted on top), plus how optional / feature-gated and target-specific external entries conform.

### Modified Capabilities

- `crate-ring-layout`: the in-source `identus-conformance` dep-graph guard gains a requirement to reject inline external dependency versions and to enumerate every dependency-section kind (the top-level `[dependencies]`, `[dev-dependencies]`, `[build-dependencies]` tables and their target-specific `[target.'cfg(...)'.dependencies|dev-dependencies|build-dependencies]` variants), so the guard enforces both the inward-direction layer rules (existing) and the external single-source rule (new) in the same manifest-reading pass.

## Impact

- **`Cargo.toml` (root)**: `[workspace.dependencies]` gains `toml = "0.8"` (the seed entry); future external deps (`add-crypto-capability`'s crypto crates) are added here as they land.
- **`crates/conformance/Cargo.toml`**: the inline `toml = "0.8"` in `[dev-dependencies]` becomes `toml.workspace = true`.
- **`crates/conformance/src/lib.rs`**: the `guard` test module gains (a) a helper that enumerates every dependency-section kind (top-level and target-scoped) from a parsed manifest, and (b) a new `#[test]` asserting no external entry in any of those sections pins `version = "..."` inline. Runs through the existing crane `rust-test` nix check; no Node, no `serde_json`, no nix config change, no new CI job.
- **No production crate manifests change** today (none declare external deps yet). The first real consumers are `add-crypto-capability` and `add-supply-chain-audit-tooling`, both of which assume this change landed first.
- **Assumes prior changes landed**: `crate-ring-layout` (the guard and the workspace-dependency map already exist). Sequenced **before** `add-supply-chain-audit-tooling` (single-source declaration is what makes that change's planned `multiple-versions = "deny"` trivially satisfiable) and **before** `add-crypto-capability` (the first change to pull real third-party crates into the graph).