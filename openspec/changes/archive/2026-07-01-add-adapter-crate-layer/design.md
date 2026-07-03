## Context

`crate-ring-layout` places `identus-adapters` alone in `outer-boundary`. The `wallet-key-management` exploration already anticipates `identus-adapters` accumulating `InMemorySecureStorageAdapter`, SQLite, Secure Enclave, Android Keystore, HSM-KMS, WebCrypto, and OS keychain adapters — feature-gated, incremental. A review of `add-crypto-capability` surfaced that the `ring`-backed `SecureRandom` adapter belongs in `outer-boundary`, not in `identus-crypto` (domain) — co-locating it in the domain crate couples the domain to `ring`'s platform limitations (no `wasm32`) and sets the precedent that adapters live in domain crates. But `outer-boundary` currently has no entropy adapter home, and stuffing every adapter into one crate is the latent structural debt this change addresses head-on: feature unification across the graph leaks every enabled adapter's dep tree into every consumer, a change to any adapter recompiles the whole crate and its downstream, and the crate has no single responsibility.

## Goals / Non-Goals

**Goals:**
- Make `outer-boundary` a *layer* of cohesive adapter crates, one per port family, so each adapter's dependency graph is isolated and the layer scales without a monolithic crate.
- Establish the conventions (naming, `default = []`, no cross-family coupling, composition-root-only consumption) that keep adapter crates from rotting as they multiply.
- Create `identus-adapters-entropy` as the first adapter-family crate, ready for `add-crypto-capability` to fill with the `ring`-backed `SecureRandom` adapter.
- Make the conformance guard admit new adapter-family crates mechanically (derive counts/outer-set from `LAYER_RULES`), so adding one does not require editing a hard-coded literal.
- Keep the change non-breaking: the existing `identus-adapters` stub is retained; no production crate gains an outward edge.

**Non-Goals:**
- Filling `identus-adapters-entropy` with the `ring` adapter implementation — that is `add-crypto-capability` (the consumer). This change creates the crate skeleton + the rules.
- Splitting the existing `identus-adapters` stub into per-family crates (storage/transport/resolver/openid4vc). Those land incrementally in their own changes as each port gains adapters; this change only establishes the pattern and the first instance.
- The `getrandom`-backed wasm `SecureRandom` adapter — deferred to the future TS/wasm binding change, where it lands in `identus-adapters-entropy` behind a `wasm`/`getrandom` feature alongside `ring`.
- Redefining the composition root as a dedicated crate. The inward-direction policy already ensures only binaries/examples/`identus-bindings` consume adapter crates; no new "app" crate is introduced here.
- The proc-macro / `identus-derive` modifications — those belong to `add-domain-newtype-macro`; this change does not restate them.

## Decisions

### Decision 1: One crate per port family, not one per adapter and not one grand crate

`outer-boundary` SHALL contain `identus-adapters-<family>` crates where `<family>` is the port family (entropy, storage, transport, resolver, openid4vc). Each crate owns all concrete adapters for that family. This is the middle ground: coarser than one-crate-per-adapter (too many crates, weak cohesion per crate) and finer than one-grand-`identus-adapters` (the monolith this change avoids).

**Rationale:** a port family has a coherent single responsibility ("implement the storage ports", "implement the entropy port") and a shared dependency footprint (e.g., the storage family pulls `rusqlite`/HSM SDKs; the entropy family pulls `ring`/`getrandom`). Grouping by family keeps each crate cohesive and isolates each family's heavy deps, while avoiding crate explosion. The Rust ecosystem norm ("many small crates", e.g. tokio/RustCrypto) favours this granularity.

**Alternatives considered:**
- *One grand `identus-adapters` crate, feature-gated*: rejected; feature unification across the graph leaks every enabled adapter's dep tree into every consumer, a change to any adapter recompiles the whole crate and its downstream, and the crate has no single responsibility.
- *One crate per individual adapter* (`identus-adapters-ring`, `identus-adapters-sqlite`, `identus-adapters-hsm`, …): rejected; too fine — proliferates crates for adapters that share a port family and a dep footprint, with no cohesion payoff over the family grouping.

### Decision 2: `default = []` for adapter crates

Every `identus-adapters-<family>` crate SHALL ship with `default = []` — no adapter enabled by default. Each concrete adapter is behind its own feature (`ring`, `getrandom`, `sqlite`, `hsm`, …).

**Rationale:** adapters are opt-in by definition; a consumer that does not name an adapter feature must not compile its dependency tree. This is the opposite of `identus-crypto`'s `default = all` (crypto's *capability* is the default surface; an *adapter* is infrastructure the consumer must choose). Default-off is what keeps the layer's compile cost proportional to what a target actually uses.

### Decision 3: No cross-family coupling; adapters are leaf crates

An `identus-adapters-<family>` crate SHALL NOT depend on another `identus-adapters-<family'>` crate. Adapter crates depend only inward (`identus-core` through `identus-wallet`/`identus-agent` as needed to implement their ports) plus external crates. No production lib crate (foundation through orchestration) depends on any adapter crate (already enforced by the inward-direction policy); adapter crates are consumed only by binaries, examples, and `identus-bindings` (the composition root).

**Rationale:** cross-adapter coupling is the nucleus of adapter-crate rot (a shared `util` two adapters grow to depend on becomes an undeclared coupling point). Forbidding it keeps each family crate independently extractable and reviewable. The composition-root-only consumption is already a consequence of the ring's inward rule; restating it for adapter-family crates makes the intent explicit.

### Decision 4: `identus-adapters-entropy` is the first family crate; `ring` is its first adapter

`identus-adapters-entropy` is created here as a stub and filled by `add-crypto-capability` with the `ring`-backed `SecureRandom` adapter (implementing `identus_crypto::SecureRandom`). The future `getrandom`-backed wasm adapter lands in the same crate behind a `wasm`/`getrandom` feature.

**Rationale:** entropy is the adapter the wasm story pivots on, and the one `add-crypto-capability` is ready to fill — making it the natural wedge for the family-crate pattern. Co-locating the future `getrandom` adapter with `ring` in the same family crate keeps the entropy port's adapters together.

### Decision 5: Guard derives crate count and outer-set from `LAYER_RULES`

The conformance guard's crate-count assertions and its production-must-not-depend-on-outer check SHALL derive from `LAYER_RULES` membership rather than hard-coded literals (`13`) or a hand-maintained `OUTER_CRATES` list. Adding an `identus-adapters-<family>` crate is then a two-line change (add to `LAYER_RULES` + root `[workspace.dependencies]`) with no guard-logic edit.

**Rationale:** a hard-coded count would require a spec edit + guard edit for every new adapter crate; deriving from `LAYER_RULES` makes the pattern scale mechanically. This composes with `add-domain-newtype-macro`, which already moves the guard toward "derive from `LAYER_RULES`" for the `identus-derive` addition; both changes converge on the same derived-count guard, so whichever applies first, the other reconciles trivially.

### Decision 6: Non-literal crate counts in the spec text; guard-relevance splits the mechanism

The spec's two count-pinning requirements are made non-literal and tolerant of future adapter-family crates, via two *different* mechanisms chosen by guard-relevance:

- `### Requirement: Workspace dependency map` (guard-relevant — the guard reads `[workspace.dependencies]` to identify workspace-internal edges) is MODIFIED to state that `[workspace.dependencies]` maps every `LAYER_RULES` member, with the count derived from `LAYER_RULES` (no literal `13`/`14`/`15`). This matches Decision 5's guard derivation, so spec and guard share one source of truth.
- `### Requirement: Crate stubs at minimal code depth` (NOT guard-relevant — the guard reads only `Cargo.toml` manifests and never inspects `lib.rs` content or counts stubs) is RENAMED from `Twelve crate stubs at minimal code depth` and its body rewritten to enumerate the founding runtime stubs plus `identus-adapters-entropy`, admitting further `identus-adapters-<family>` stubs by reference to the adapter-family convention. No fixed total count is normative; no `stub: bool` flag is added to `LAYER_RULES` (stub-ness is a content/placeholder property the guard does not test, so a flag in the guard's data structure would be dead). The "Twelve" is dropped from the title because the total stub population is now dynamic via by-reference admission; the founding twelve remain a fixed, named set.

**Rationale:** the conformance guard tests dependency soundness (layer membership + inward edges + workspace-dep form), not stub content; "stub-ness" is a placeholder/content property independent of layer and not tracked by `LAYER_RULES`. Deriving the stub count from `LAYER_RULES` would be a false derivation (a filled crate stays a `LAYER_RULES` member but stops being a stub), and a `stub` flag would be dead data the guard never reads. The honest split is: derive the guard-relevant count (dep map) from `LAYER_RULES`; keep the non-guard stub set as an enumeration-plus-by-reference population that grows with the adapter-family convention. This makes the spec tolerant of each new adapter crate without a spec edit, on both requirements.

## Risks / Trade-offs

- **[Trade-off] more crates in the workspace** → accepted; the per-family granularity pays for itself in compile isolation and cohesion. The ring-layout's previously-fixed count is relaxed deliberately.
- **[Risk] two in-flight changes touch the conformance guard and the `crate-ring-layout` count requirements** (this change and `add-domain-newtype-macro`) → this change RENAMES `Twelve crate stubs at minimal code depth` → `Crate stubs at minimal code depth` and MODIFIES it and `Workspace dependency map` to non-literal/derived counts; `add-domain-newtype-macro` MODIFIES the same two headers (adding the `identus-derive` exclusion and the proc-macro carve-out). The collision is resolved by archive ordering: **`add-adapter-crate-layer` archives first** (its RENAMED + MODIFIED blocks are self-contained and carry no `identus-derive` content), then `add-domain-newtype-macro` archives second with its MODIFIED blocks retargeted to the post-rename header (`Crate stubs at minimal code depth`) and carrying the full combined content (founding runtime stubs + `identus-adapters-entropy` + by-reference admission + `identus-derive` exclusion; derived workspace-dep set including both `identus-adapters-entropy` and `identus-derive`). Because `add-domain-newtype-macro`'s retargeted MODIFIED header does not exist in the current main spec until this change archives, `add-domain-newtype-macro` is **archive-blocked-by this change** and not standalone-`openspec validate`-able in the interim — a known, documented coupling, not a design conflict. Apply-time reconciliation at the guard source (both replace the hard-coded `13` with the derived form) remains a two-line merge.
- **[Risk] the retained `identus-adapters` stub looks vestigial beside family crates** → accepted for now; it remains the cross-cutting/facade home for adapter concerns that don't belong to a single port family, and may be repurposed or removed in a future change once the family split is complete.

## Migration Plan

1. Relax the ring-layout spec: RENAME `Twelve crate stubs at minimal code depth` → `Crate stubs at minimal code depth` (non-literal body: founding runtime stubs + `identus-adapters-entropy` + by-reference admission of future adapter-family stubs); MODIFY `Workspace dependency map` to a `LAYER_RULES`-derived count (no literal); ADDED requirements (adapter-family conventions, `identus-adapters-entropy`, composition-root-only dependency, guard derives count from `LAYER_RULES`).
2. Add `ring` + `identus-adapters-entropy` to root `[workspace.dependencies]`; create the `identus-adapters-entropy` stub.
3. Add `identus-adapters-entropy` to `LAYER_RULES` `outer-boundary` membership; switch the guard's two count assertions and the outer-set check to derive from `LAYER_RULES`.
4. Archive `add-adapter-crate-layer` **before** `add-domain-newtype-macro`. When `add-domain-newtype-macro` archives next, its MODIFIED blocks for `Crate stubs at minimal code depth` and `Workspace dependency map` SHALL be retargeted to the renamed/derived headers and carry the combined content including `identus-derive`.
5. `add-crypto-capability` (consumer) fills the `ring` adapter and defines the `SecureRandom` port in `identus-crypto`.

**Rollback:** reverting removes the `identus-adapters-entropy` stub, restores the hard-coded count literals / `OUTER_CRATES` list, and removes the ADDED spec requirements. No production crate depends on the new crate at merge time.

## Open Questions

- **`OUTER_CRATES` disposition:** remove the const entirely in favour of a `layer_of`-derived outer set, or keep it as a derived helper cached at test start. To finalize during apply (purely an implementation choice; the contract is "derived from `LAYER_RULES`").