## Context

`add-adapter-crate-layer` split `outer-boundary` into per-port-family `identus-adapters-<family>` crates and explicitly rejected the monolithic single-`identus-adapters` crate on cohesion and compile-leakage grounds (Decision 1). It retained the original `identus-adapters` crate only as a non-breaking concession — "the cross-cutting/facade home for adapter concerns that don't belong to a single port family" — and recorded in its Risks that it "may be repurposed or removed in a future change once the family split is complete." Current state:

- `crates/adapters/` is a pure stub: `Cargo.toml` (deps on core/did/messaging/openid4vc/trust/wallet) + `src/lib.rs` (a doc-comment and a `pub const COMPONENT`).
- Its `lib.rs` doc-comment claims it "owns concrete transport, storage, and resolver adapters" — exactly the concerns the family crates now own, i.e. the comment already contradicts the spec.
- No production crate, `identus-bindings` (the composition root), binary, or example declares `identus-adapters` as a dependency. `identus-bindings` depends only on `identus-core` + `identus-wallet`.
- The conformance guard derives workspace crate count and the outer/verification set from `LAYER_RULES` membership (per `add-adapter-crate-layer`), so removing a `Member` entry needs no literal edit.
- The guard has one test assertion naming `identus-adapters` as the example outer crate to prove outward-dep rejection (`crates/conformance/src/lib.rs`, the `Guard fails on an outward dependency to a non-proc-macro crate` scenario mirrored in the spec).
- The in-flight `add-crypto-capability` change (complete, 29/29) has `design.md` prose referring to "platform adapters in `identus-adapters`" conceptually; it is prose, not a dependency, and the change is about to archive as historical.

The hexagonal architecture already names `identus-bindings` as the composition root (the "Adapter-family crates are composition-root-only dependencies" requirement: adapter-family crates "SHALL be consumed only by binaries, examples, and `identus-bindings` (the composition root)"). The retained facade is the one element of the original monolithic `identus-adapters` that the split left behind.

## Goals / Non-Goals

**Goals:**
- Remove the dead `identus-adapters` crate, its workspace-dep entry, and its `LAYER_RULES` membership, completing the architecture `add-adapter-crate-layer` intended rather than leaving a vestigial crate.
- Restate the `outer-boundary` layer so its dual character — adapter-family leaf crates + the composition root (`identus-bindings`) — is legible at the layer's defining requirement, not scattered across requirements.
- Keep the conformance guard green by deriving counts/outer-set from `LAYER_RULES` (no literal edits) and retargeting the one outward-dep-rejection test to a surviving outer crate.

**Non-Goals:**
- Filling, splitting, or otherwise modifying any `identus-adapters-<family>` crate — those are unaffected.
- Introducing a replacement cross-cutting crate. The exploration concluded every candidate cross-cutting concern (facade re-export, composition wiring, cross-family shared types, cross-family test fakes) belongs elsewhere (`identus-bindings`, orchestration, `identus-core`, `identus-conformance`); no permanent crate is warranted.
- Editing the `add-crypto-capability` `design.md` prose. It is a complete, about-to-archive change; its "platform adapters in `identus-adapters`" line is a historical prose inaccuracy, not a dependency. Archives are left untouched.
- Touching the `identus-adapters-entropy crate` requirement, the `Adapter-family crates are composition-root-only dependencies` requirement, or the `Conformance guard derives workspace crate count and outer-set from LAYER_RULES` requirement — all remain valid as-is. Removing a member is already admitted by the derived-count mechanism.

## Decisions

### Decision 1: Remove `identus-adapters` entirely; do not repurpose

`identus-adapters` is deleted, not narrowed. The retained-stub rationale ("cross-cutting/facade home") has not materialized and, on inspection, every candidate cross-cutting role belongs in an existing crate:

| Candidate | Belongs in |
|---|---|
| Facade re-export of all family adapters | nowhere — it walks back the dep-isolation the family split exists to provide; consumers depend on the families they want |
| Composition/wiring helper | `identus-bindings` (composition root) or orchestration |
| Cross-family shared error/conversion types | `identus-core` (foundation) if they materialize |
| Cross-family test fakes | `identus-conformance` |

**Rationale:** a catch-all "adapters" crate is exactly the "monolith with no single responsibility" Decision 1 of `add-adapter-crate-layer` rejected. Keeping it would require either duplicating family-crate content (bad) or shrinking the doc-comment to match the thin "cross-cutting" claim, at which point the crate has no concrete reason to exist. The doc-comment drift ("owns concrete transport, storage, and resolver adapters") is itself evidence the crate has lost its reason. Removal completes the intended architecture.

**Alternatives considered:**
- *Keep as a narrow cross-cutting crate:* rejected — no cross-cutting surface has materialized and none is expected to belong here rather than in `identus-bindings`/`identus-core`/orchestration/conformance.
- *Keep as a gated facade re-export:* rejected — reintroduces the compile-leakage and feature-unification costs the family split exists to eliminate.

### Decision 2: Restate `outer-boundary` as family leaves + composition root (Option B)

The "Adapter-family crates in the outer-boundary layer" requirement is rewritten so the layer's defining sentence names both halves of its character: the `identus-adapters-<family>` leaf crates (one per port family) plus `identus-bindings`, the composition root that wires selected adapter families into runnable artifacts. The "retained `identus-adapters` cross-cutting/facade" clause is dropped.

**Rationale:** today the layer's composition is spread across two requirements — the family-crate requirement describes only the adapter side, and `identus-bindings`' composition-root role lives in a separate "composition-root-only" requirement. Option B makes the layer legible at its defining requirement, so a reader sees "adapter leaves + composition root" in one place. This also aligns the requirement with the Purpose bullet that already names `identus-bindings` as the composition root.

**Alternatives considered:**
- *Option A (minimal):* state the layer as `identus-adapters-<family>` crates + `identus-bindings` without naming the composition-root role. Rejected in favour of B because it leaves the layer's dual character implicit; B makes it legible at a glance with negligible extra normative weight.

### Decision 3: Retarget the outward-dep-rejection test to a surviving outer crate

The guard scenario "Guard fails on an outward dependency to a non-proc-macro crate" (and the corresponding `#[test]` assertion in `crates/conformance/src/lib.rs`) names `identus-adapters` as the example outer-boundary crate. On removal it is retargeted to `identus-adapters-entropy` (a surviving `outer-boundary` family crate). The scenario's intent — a domain crate depending on an outer crate is rejected — is unchanged.

**Rationale:** the test exists to exercise the inward-direction policy against the outer layer, not to assert the existence of any specific outer crate. Any surviving `outer-boundary` non-proc-macro member satisfies it; `identus-adapters-entropy` is the natural choice since it is the established first family crate. `identus-bindings` would also work but is conceptually the composition root, so the adapter-family crate is the cleaner example for an "adapter is outer" assertion.

### Decision 4: Leave the in-flight `add-crypto-capability` prose untouched

`add-crypto-capability/design.md` says "platform adapters in `identus-adapters`." It is prose in a complete (29/29), about-to-archive change. This change does not edit it and is not blocked by it.

**Rationale:** editing a complete change's artifacts is out of scope, and once `add-crypto-capability` archives it is a historical snapshot that must not be retroactively edited. The prose inaccuracy is harmless (no code depends on the wording) and self-resolves into the archive.

## Risks / Trade-offs

- **[Risk] the Purpose prose becomes stale** → the spec's `## Purpose` layer table and the "13-crate" / "`identus-adapters`, `identus-bindings`" bullet reference `identus-adapters`. The Purpose section is informational prose, not a requirement, so it is not modified via the delta mechanism; it is reconciled as a prose edit at archive/sync time (note: it is already stale relative to `identus-adapters-entropy`, which the requirements admit but the table does not list). The normative source of truth is `LAYER_RULES` and the requirements, both of which this change updates. Mitigation: the design calls out the Purpose prose reconciliation explicitly so it is not lost at archive.
- **[Risk] a future cross-cutting adapter need appears** → if a genuinely cross-cutting concern later emerges that belongs nowhere else, a new crate can be added to `LAYER_RULES` + `[workspace.dependencies]` under the derived-count mechanism; nothing in this removal forecloses that. The removal only asserts that *no such need exists today* and that the catch-all crate was not the right home for the candidates examined.
- **[Risk] concurrent in-flight change `add-port-adapter-naming-conventions` also edits `crates/conformance/src/lib.rs`** → that change restructures `conformance/src/lib.rs` into modules and adds a `syn`-based naming guard, moving the dep-graph guard (including the outward-dep-rejection assertion this change retargets) but leaving its behavior unchanged; it does not touch `crate-ring-layout`'s layer/dep-direction requirements or `identus-adapters`. Spec-level the two changes are independent (`openspec validate` passes for both), so this is an apply-time reconciliation, not a spec conflict. Recommended apply order: **`remove-adapter-facade-crate` applies first** (retargets the assertion and removes the `LAYER_RULES` member in the current flat layout), then `add-port-adapter-naming-conventions` applies and moves the already-retargeted assertion into its new module. If the order reverses, `add-port-adapter-naming-conventions` must locate the moved assertion in its new module location to retarget it — a mechanical find-and-edit, not a design conflict. The `identus-adapters-entropy` struct renames in that change are in `crates/adapters-entropy/`, which this change does not touch, so there is no collision there.
- **[Trade-off] breaking change to a workspace-internal crate name** → accepted; the only consumer surface is the crate name, and zero consumers reference it. No production crate, composition root, binary, or example breaks.

## Migration Plan

1. Spec: MODIFY the `crate-ring-layout` requirements — "Adapter-family crates in the outer-boundary layer" (drop facade, restate as family leaves + `identus-bindings` composition root, add a composition-root scenario), "Crate stubs at minimal code depth" (drop `identus-adapters` from the founding runtime stub enumeration), "Rust dep-graph guard enforces layer rules" (retarget the outward-dep-rejection scenario example to `identus-adapters-entropy`).
2. Code: delete `crates/adapters/`; drop `identus-adapters` from root `Cargo.toml` `[workspace.dependencies]`; remove the `identus-adapters` `Member` from the `outer-boundary` `LayerRule` in `LAYER_RULES`; retarget the `identus-did → identus-adapters` test assertion to `identus-did → identus-adapters-entropy`.
3. Prose: at archive/sync, reconcile the `## Purpose` layer table and the "13-crate" / "`identus-adapters`, `identus-bindings`" bullet to drop `identus-adapters` (and, while there, the already-stale absence of `identus-adapters-entropy`).

**Rollback:** reverting restores `crates/adapters/` (the stub), the workspace-dep entry, the `LAYER_RULES` member, and the original test target. No production crate depends on the removed crate at merge time, so rollback is a clean no-op for consumers.

## Open Questions

- None blocking. The retarget crate choice (`identus-adapters-entropy` vs `identus-bindings`) is settled in Decision 3; if a reviewer prefers `identus-bindings` as the example it is a one-line swap with no spec-level consequence.