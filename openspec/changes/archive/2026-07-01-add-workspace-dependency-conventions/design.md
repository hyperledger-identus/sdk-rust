## Context

`crate-ring-layout` already established a `[workspace.dependencies]` map for the 13 internal `identus-*` crates and an in-source `identus-conformance` guard (`#[test]` in `crates/conformance/src/lib.rs`) that reads `crates/*/Cargo.toml` and the root `Cargo.toml` via the `toml` crate, identifies workspace-internal dependencies by membership in the root map, and asserts every workspace-internal `[dependencies]` edge obeys the `LAYER_RULES` inward-direction policy. The guard reads no `.json`, invokes no subprocess, and runs through the existing crane `rust-test` nix check.

Two gaps make this insufficient as the first real third-party crates arrive:

1. **External deps are unregulated.** The only external dep in the tree — `toml = "0.8"` in `identus-conformance`'s `[dev-dependencies]` — is pinned inline, and the guard never sees it: `inward_workspace_deps()` reads only the `[dependencies]` table and filters to workspace-internal crate names. There is no single source of truth for external versions.
2. **The guard reads one section kind.** Even for internal edges, only `[dependencies]` is enumerated; `[dev-dependencies]`, `[build-dependencies]`, and the target-specific `[target.'cfg(...)'.dependencies]` / `[target.'cfg(...)'.dev-dependencies]` / `[target.'cfg(...)'.build-dependencies]` variants are never inspected.

`add-crypto-capability` is the first change to pull real third-party crates into the graph (~8 crypto crates, several feature-gated `optional = true`, with `default-features = false` desired for `k256`/`p256`). Its design defers the `default-features` placement to this change, and its proposal assumes this change landed first. `add-supply-chain-audit-tooling` (sequenced after this one) wants `multiple-versions = "deny"` in `deny.toml`, which is only sustainably satisfiable when versions are single-sourced.

## Goals / Non-Goals

**Goals:**
- Every external (non-`workspace.internal`, non-`path`) dependency is declared once in root `[workspace.dependencies]` and referenced from crates via `<dep>.workspace = true`, so versions (and the `default-features` off-state) are single-sourced.
- The conformance guard mechanically enforces this across every dependency-section kind (the top-level `[dependencies]`, `[dev-dependencies]`, `[build-dependencies]` tables and their target-specific `[target.'cfg(...)'.dependencies|dev-dependencies|build-dependencies]` variants), including `optional = true` entries, in the same manifest-reading pass that already enforces layer direction.
- The existing seed case (`toml` in `identus-conformance`) is migrated to the new form as the proof-of-concept.

**Non-Goals:**
- Changing the internal `identus-*` workspace-dependency map or the layer-direction rules (owned by `crate-ring-layout`).
- Curating *which* external crates are allowed or forbidding specific crates (owned by `add-supply-chain-audit-tooling`'s `deny.toml` policy).
- Choosing the crypto crates or their features (owned by `add-crypto-capability`).
- A new CI job, a new nix check, or any Node/`serde_json` dependency. The guard stays a `#[test]` reading manifests via the existing `toml` dev-dep.

## Decisions

### Decision 1: Policy as a new capability; enforcement as a new requirement on `crate-ring-layout`

The declarative policy ("all external deps are workspace-declared, `.workspace = true`") is a new capability `workspace-dependency-conventions`. The *enforcement* is a new requirement added to `crate-ring-layout`, because the guard that must run it already lives there (same `#[test]` module, same parsed manifests, same `toml`-dev-dep, same crane `rust-test` nix check). Splitting policy (what) from guard location (where it runs) keeps each spec cohesive: `workspace-dependency-conventions` states the contract every crate manifest must satisfy; `crate-ring-layout` states that the guard checks it.

**Alternatives considered:**
- *Own the guard inside `workspace-dependency-conventions`.* Rejected: it would duplicate the manifest-reading pass and the "runs through crane `rust-test`, no Node, no `serde_json`" constraints already encoded for the layer guard, fragmenting the single conformance test module into two capabilities.
- *Make the whole thing a `crate-ring-layout` modification with no new capability.* Rejected: external-dep declaration is a distinct, independently-reasonable contract (a future crate could satisfy it without the ring, e.g. a non-ring workspace) and deserves its own spec so consumers like `add-crypto-capability` can reference it by name.

### Decision 2: `default-features` declared at workspace level

When `default-features = false` is needed, it is set on the root `[workspace.dependencies]` entry, not per-crate. The workspace entry becomes the single source for both `version` and the default-feature off-state; per-crate manifests add only `.workspace = true` plus any additional `features = [...]` they need. A per-crate override (`default-features = true` in the referencing crate) remains legal Cargo and is the documented escape hatch for the rare crate that needs the defaults on — but it is an exception, not the norm.

**Rationale:** `add-crypto-capability` wants the same off-by-default for both `k256` and `p256` and explicitly defers this decision here; workspace-level keeps the off-state single-sourced, consistent with this change's "version declared once" principle, and prevents two crates from silently disagreeing about whether defaults are on. The escape hatch preserves the one legitimate reason to localise the decision (a single quirky crate).

**Alternatives considered:**
- *`default-features` per-crate.* Rejected: reintroduces exactly the drift this change exists to prevent on the off-state, and `add-crypto-capability` (the first consumer) wants the shared off-state, not per-crate variation.

### Decision 3: The guard enumerates every dependency-section kind, including target-scoped variants

The new guard enumerates the top-level `[dependencies]`, `[dev-dependencies]`, and `[build-dependencies]` tables from each parsed manifest, plus their target-specific `[target.'cfg(...)'.dependencies]`, `[target.'cfg(...)'.dev-dependencies]`, and `[target.'cfg(...)'.build-dependencies]` variants, and applies the external-version rule to each. This is forced: the seed case (`toml`) lives in `[dev-dependencies]`, so a guard that did not enumerate dev-deps would not even catch the migration's own proof-of-concept. Covering `[build-dependencies]` and the target-scoped variants proactively (none exist today) is cheap — one helper iterating a fixed section list — and avoids the same gap being reopened when the first build-script or `cfg`-gated external dep lands. The target-scoped dev/build variants are included so a `cfg`-gated external dep cannot evade the rule by being placed under `[target.'cfg(...)'.dev-dependencies]` instead of `[target.'cfg(...)'.dependencies]`.

### Decision 4: `optional = true` and feature-gated external entries are treated identically

An external dependency marked `optional = true` (required for Cargo feature-gating, e.g. `ed25519 = ["ed25519-dalek"]`) still must be declared at workspace level and referenced via `.workspace = true`. The guard treats `optional = true` as orthogonal to the version-source rule: it checks the version-source form, not the optionality. This is forced by `add-crypto-capability`, whose feature-gated crypto crates are exactly the crates this convention exists to govern.

## Risks / Trade-offs

- **[Risk] A legitimate transitive duplicate forces a workspace entry to differ across crates** → `default-features`/`features` differences are still expressible per-crate on top of the shared version; if a *version* truly must differ (rare, and usually a sign of a stale dep), the escape hatch is to not list it as a shared workspace dep and accept a `[bans] skip` in `add-supply-chain-audit-tooling` instead. The guard rejects inline `version` regardless, so the divergence is surfaced as a deliberate, reviewable exception rather than silent drift.
- **[Risk] The guard reads manifests with the `toml` crate, which parses only what is written** → The guard checks syntactic form (presence of `version` inline vs `workspace = true`), not resolved versions; that is the intended contract. Resolution correctness is `cargo`'s job. `add-supply-chain-audit-tooling`'s `cargo-deny` covers resolved-graph concerns (`multiple-versions`).
- **[Risk] Enumerating `[build-dependencies]`/target sections today finds nothing, inviting bit-rot** → Mitigated by the seed `toml` case in `[dev-dependencies]`, which keeps the section-enumeration helper exercised on every run; the unused branches are dead-simple table lookups, not complex logic.
- **[Trade-off] Workspace-level `default-features` slightly reduces per-crate locality** → Accepted; the escape hatch exists, and the dominant consumer (`add-crypto-capability`) wants the shared state.

## Migration Plan

1. Add `toml = "0.8"` to root `[workspace.dependencies]`.
2. Change `crates/conformance/Cargo.toml` `[dev-dependencies]` from `toml = "0.8"` to `toml.workspace = true`.
3. Extend the `guard` module in `crates/conformance/src/lib.rs`: add a helper enumerating every dependency-section kind (top-level and target-scoped) from a parsed manifest, and a new `#[test]` asserting no external entry in any section pins `version = "..."` inline (i.e. every external entry carries `workspace = true` and resolves to a root entry).
4. Run `cargo test -p identus-conformance` and `nix flake check`; both pass on the migrated tree.

**Rollback:** revert the three edits; the guard reverts to layer-direction-only and `toml` returns to inline. No data, no generated files, no CI plumbing to undo.

## Open Questions

None remaining. The four questions raised in the originating exploration are resolved by Decisions 2–4 (section coverage; `default-features` placement; optional/feature-gated handling) and by the sequencing noted in the proposal (interaction with `add-supply-chain-audit-tooling`). Decision 2 (`default-features` at workspace level) is the one judgment call with a real trade-off; it is flagged here and in the spec so it can be revisited if `add-crypto-capability`'s apply reveals a crate that genuinely needs per-crate default-feature divergence.