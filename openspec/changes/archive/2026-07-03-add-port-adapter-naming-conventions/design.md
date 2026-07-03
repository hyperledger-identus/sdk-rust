## Context

The `sdk-rust` workspace is hexagonal: domain/protocol crates define *ports* (traits) and the outer-boundary `identus-adapters-<family>` crates provide *adapters* (concrete structs implementing those ports). Today the only port is `SecureRandom` (in `identus-crypto`), with two adapters in `identus-adapters-entropy`: `RingSystemRandom` and `DeterministicRandom`. The port side already follows the desired convention (no `Port` suffix), but the adapter side does not (no `*Adapter` suffix), and nothing in the workspace encodes the convention or stops it from drifting. The `storage`, `transport`, and `resolver` port families are planned, so the convention must be fixed and enforced before the inconsistency multiplies.

Enforcing it requires a *second* guard in `identus-conformance`, which today is a single ~470-line `lib.rs` holding both the invariant data (`LAYER_RULES`, layer types) and the dep-graph guard test. Adding a second guard plus `syn` parsing plus source-walking into the same file would turn it into a mudball, so the crate's internal structure must be imposed as part of this change.

Constraints (from landed specs): dependency direction is inward and enforced (`crate-ring-layout`); external deps are workspace-level (`workspace-dependency-conventions`); the conformance guard reads files and is `#[cfg(test)]`; `syn` is already a workspace dependency (used by `identus-derive`); proc-macro crates (`identus-derive`) are exempt from the inward-direction policy — any crate may depend on them; version is `0.0.0` so public-API breaks are free.

## Goals / Non-Goals

**Goals:**
- Codify a single port/adapter naming convention: ports are bare capability nouns (no `Port` suffix, and `Port` suffix forbidden); adapters are `<Backend><Capability>Adapter`.
- Make port-ness self-documenting and machine-discoverable from source, with no hand-maintained parallel registry that can drift.
- Enforce the port-side naming rule at **compile time** (fail fast on any `cargo build` in any crate), not only at conformance-test time.
- Enforce the adapter-side naming rule in `cargo test` via a fast, hermetic guard — no external server, no indexing, no per-adapter annotation.
- Retrofit the two existing adapters to the convention.
- Impose a reusable module structure on `identus-conformance` so future guards slot in without restructuring.

**Non-Goals:**
- Renaming the adapter-**family crates** (`identus-adapters-entropy`, etc.) — their `identus-adapters-<family>` naming is already covered by `crate-ring-layout` and is not in scope.
- Changing the layer rules, crate membership, or dependency-direction policy.
- Adding new ports or adapter families (`storage`/`transport`/`resolver`); this change only governs *how* existing and future ones are named.
- Runtime enforcement; the convention is enforced at compile/test time, matching the existing guard posture.
- Key-management ports (`KeyHandle`/`KeyStore`/`SecretResolver`) — those are an `identus-wallet` concern per `add-crypto-capability`, not naming-convention scope.
- Marking adapter structs (e.g. an `#[identus::adapter]` attribute). Adapter-ness is structurally derivable (a struct in an `identus-adapters-*` crate implementing a port), so per-adapter annotation is neither required nor wanted.

## Decisions

### Decision 1: An inert `#[identus::port]` attribute declares port-ness, not a marker supertrait

An earlier draft of this change chose a `pub trait Port {}` marker in `identus-core` with port traits declaring `: Port` as a supertrait (`pub trait SecureRandom: Port`). That approach is **rejected** because a Rust supertrait bound is an implicit `Self:` bound on every implementer: `trait SecureRandom: Port` requires `RingSystemRandomAdapter: Port` to hold, so every adapter must add a vacuous `impl Port for RingSystemRandomAdapter {}`. Worse, the marker's meaning collapses — `Port` no longer means "this is a port," it means "a port, or anything that implements a port" — which defeats the self-documenting property that was the marker's whole justification. (Verified: `impl SecureRandom for RingAdapter` against `pub trait SecureRandom: Port` fails to compile with `the trait Port is not implemented for RingAdapter`.)

The fix is to tag port-ness on the trait **item** via an inert attribute `#[identus::port]`, not as a `Self:` bound. An attribute is attached to the trait item and is *not* propagated to implementers, so adapters need no extra impl and the attribute means exactly "this trait is a port."

The attribute lives in `identus-derive` (the existing workspace proc-macro crate, which already uses `syn`). It is registered as a `#[proc_macro_attribute]` that, in its inert form, validates the trait name (see Decision 2) and otherwise emits the trait unchanged. Proc-macro crates are exempt from the layer inward-direction policy (`crate-ring-layout`), so any inner-ring port crate may depend on `identus-derive` without a layer violation.

**Cost — one new build-time edge:** the port-owning crate (`identus-crypto`, and future `storage`/`transport`/`resolver` crates) gains a dependency on `identus-derive`. `identus-crypto` does not currently depend on it (only `identus-core` and `identus-did` do), so this is a new, layer-legal (proc-macro exemption), build-time-only edge. This is the one cost the marker-in-core approach avoided ("every port crate already depends on core"); it is judged acceptable because the edge is layer-legal and build-time only, and it avoids the marker-trait propagation flaw entirely.

**Why not a hand-maintained port registry** in `identus-conformance` (the original Option A)? Both the registry and the attribute are opt-in (forgotten = unenforced), but the attribute is self-documenting at the definition site (a reader sees `#[identus::port]` and knows the role) and has a single drift mode (forgotten attribute), while the registry has two (forgotten line + dangling reference for a removed port) and is not self-documenting. The attribute therefore dominates the registry on both axes the original design cited against the registry.

### Decision 2: Compile-time port-name enforcement + syn-based discovery via attributes

Two enforcement layers, deliberately split:

**Port side — compile time.** The `#[identus::port]` proc-macro receives the trait's tokens, parses it as `syn::ItemTrait`, reads `ident`, and emits `compile_error!` if the name ends in `Port`. This fires on any `cargo build` in any crate that defines a port, before the conformance test run. Because the attribute is attached to the very item whose name is being checked, a compile-time name check is free here.

**Adapter side — test time.** The conformance naming guard parses `crates/adapters-*/src/**/*.rs` with `syn`, finds `syn::ItemImpl` whose implemented trait is a discovered port, and asserts the implementing `pub struct`'s name ends in `Adapter`. This cannot be compile time without per-adapter opt-in annotation: the type system cannot see type *names* on stable Rust (no `Self: NamedAdapter` bound exists; `std::any::type_name` is runtime), and no macro is attached to a plain `impl` "by virtue of being an adapter." The test-time scan needs no per-adapter ceremony and catches every adapter.

**The asymmetry is inherent and justified.** A compile-time name check requires a proc-macro syntactically attached to the item being named. The port has a natural attachment point (its own port-ness attribute), so the check is free. The adapter is an ordinary `impl` with no attachment point, so only an external scan can read its name without inventing per-adapter annotation (`#[identus::adapter]`) — which would be opt-in (forgotten = unenforced) and would still require the test-time guard as a backstop, a net negative.

**Discovery for the test-time guard:** the guard reads `syn::ItemTrait.attrs` to find traits carrying `#[identus::port]` (the port set), then scans `ItemImpl`s of those ports. Because the attribute is an unambiguous token on the item (not a bound that can also appear in a `where` clause), the `where`-clause-vs-supertrait disambiguation that motivated the original `syn`-over-regex choice **simply disappears** — there is nothing to confuse it with.

**Why `syn` over regex:** `syn::parse_file` gives a real syntax tree (`attrs`, `ItemImpl`, `ItemTrait` as structured items) at milliseconds per file, with no supply-chain cost (`syn` is already a workspace dependency via `identus-derive`). **Why not rust-analyzer/LSP:** rejected for the same four reasons as before — the accurate `hir` API is internal to the ra binary (not a published crate); the stable LSP interface exposes references (locations), not "this trait has the port attribute"; spinning up an ra server inside a `#[cfg(test)]` unit test indexes the workspace (seconds–tens of seconds, hundreds of MB), breaking the hermetic, fast, reproducible `cargo test`/`nix flake check` posture; and ra is not a devshell build input for crate tests today, so adopting it changes the toolchain contract for every contributor. `syn` captures the intent (accurate structural detection) without a server.

### Decision 3: Adapter name form is `<Backend><Capability>Adapter` (Form A)

Adapter struct names are `<Backend><Capability>Adapter` — e.g. `RingSystemRandomAdapter`, `DeterministicRandomAdapter`, and (future) `GetrandomSystemRandomAdapter`.

**Why over `<Backend>Adapter` (Form B):** the bare backend (`RingAdapter`) loses the capability word, which is the most legible part at an injection site (`fn generate(&mut impl SecureRandom)` callers see the adapter type). It also collides if one backend serves two ports (e.g. a hypothetical `ring`-backed storage adapter).

**Why over `<Backend><PortFamily>Adapter` (Form C):** tying the struct to the `adapters-<family>` crate name (`RingEntropyAdapter`) introduces a *second* vocabulary — the family name (`entropy`) and the port name (`SecureRandom`) — that the spec would have to keep in sync. The capability word (`SystemRandom`) is already what the existing names use, so Form A is the smallest diff (`RingSystemRandom` → `RingSystemRandomAdapter`, suffix-only) and avoids a new naming axis.

### Decision 4: Retrofit-rename the two existing adapters

`RingSystemRandom` and `DeterministicRandom` are renamed to add the `Adapter` suffix as part of this change, rather than grandfathered as exceptions.

**Why:** grandfathering leaves two public names that violate the convention the spec just codified, which both defeats the "no exceptions" stance and forces the guard to carry an allow-list. Since the workspace version is `0.0.0`, the public-API break is free; the only consumers are tests and the composition root (`identus-bindings`), updated in the same change. The deferred wasm adapter, when it lands, will be `GetrandomSystemRandomAdapter` from the start.

### Decision 5: `syn` is a dev-dependency of the conformance guard, guard is `#[cfg(test)]`

The naming guard runs only in `cargo test`, matching the existing dep-graph guard's `#[cfg(test)]` posture. `syn` is therefore a `[dev-dependencies]` entry in `identus-conformance`, not a runtime dependency — the production crate's dependency graph is unchanged. (The compile-time port-name check uses `syn` inside `identus-derive`, which is a normal build dependency of port crates — not a dev-dependency — but that is build-time proc-macro tooling, not a runtime dependency of the port crate.)

### Decision 6: Conformance crate structure — rulebook vs guards, one module per guard

`identus-conformance/src/` is restructured into a thin `lib.rs`, a `rulebook.rs` (the invariant data, `pub(crate)`, runtime-available), and a `guard/` directory (`#[cfg(test)]`) with `mod.rs` (shared helpers), `dep_graph.rs` (the existing guard, moved verbatim), and `naming.rs` (the new guard). See the `conformance-crate-structure` spec for the rules; this design only records the rationale: separating data (source of truth, reused by every guard) from logic (test-only) is what keeps the crate navigable as guards accumulate, and "one module per guard" means adding a guard never touches another.

## Risks / Trade-offs

- **[New edge `identus-crypto → identus-derive`]** The attribute approach introduces a build-time dependency from a port crate to the proc-macro crate. Mitigation: proc-macro crates are exempt from the layer inward-direction policy (`crate-ring-layout`), so the edge is layer-legal; it is build-time only (no runtime dependency), workspace-internal, and already pinned at the workspace level via `identus-derive`. No external supply-chain surface is added. Future port crates incur the same allowed edge.
- **[Compile-time port check is opt-in]** The `#[identus::port]` attribute fires only on annotated traits; a port trait written without the attribute is neither compile-checked for its name nor discovered by the test-time guard (so its adapters also go unchecked). Mitigation: this is the same trust model as any opt-in convention — the spec requires the attribute on every port, and code review catches omissions. It is strictly no worse than the rejected marker-trait approach (which also depended on an opt-in `: Port` bound) and avoids that approach's propagation flaw.
- **[`syn` parse robustness]** `syn` parses the edition's grammar; a future edition change or unusual macro-generated trait could in principle parse differently. Mitigation: `syn` is the canonical Rust parser and is kept current at the workspace level; the workspace uses stable edition 2024 with no exotic trait-define macros in port crates. The guard fails closed (a parse error fails the test, surfacing the issue rather than silently passing).
- **[Breaking rename of public adapters]** Renaming `RingSystemRandom`/`DeterministicRandom` is a public-API break. Mitigation: version `0.0.0`; the only consumers are in-tree tests and (future) `identus-bindings`, all updated in this change; no external consumer exists at this version.
- **[Guard scan cost]** The guard parses every `crates/*/src/**/*.rs` file (for port discovery) and every `crates/adapters-*/src/**/*.rs` file (for adapter enforcement). Mitigation: the workspace is ~15 small crates; `syn::parse_file` is milliseconds per file, run only in `cargo test`. Far below the cost of an LSP index.

## Migration Plan

1. Add the inert `#[identus::port]` attribute helper to `identus-derive` (`#[proc_macro_attribute]` that parses the input as `syn::ItemTrait`, emits `compile_error!` if `ident` ends in `Port`, and otherwise passes the trait through unchanged). Add `identus-derive` as a dependency of `identus-crypto`. Annotate `SecureRandom` with `#[identus::port]`. No `Port` trait is added to `identus-core`.
2. Restructure `identus-conformance` into `lib.rs` + `rulebook.rs` + `guard/{mod,dep_graph,naming}.rs`; move the existing dep-graph guard verbatim; add `syn` dev-dep.
3. Implement the naming guard in `guard/naming.rs`: discover the port set by reading `#[identus::port]` off `syn::ItemTrait.attrs`; scan `crates/adapters-*/src/**/*.rs` for `syn::ItemImpl` of a discovered port and assert the implementing `pub struct`'s name ends in `Adapter`; fail closed on parse errors. The guard MAY redundantly re-assert that discovered port names do not end in `Port` (compile-time already enforces this for annotated ports).
4. Rename `RingSystemRandom` → `RingSystemRandomAdapter` and `DeterministicRandom` → `DeterministicRandomAdapter` in `identus-adapters-entropy`, updating struct defs, `impl SecureRandom for …` blocks, and the module doc-comment.
5. Run `cargo build --workspace` (the `#[identus::port]` compile-time check is exercised wherever a port is defined) and `cargo test -p identus-conformance` (both guards green); confirm no other crate references the renamed types.

**Rollback:** revert the change; no data migration, no persisted state. The attribute, the rename, and the crate restructure are all source-only.

## Open Questions

- Should the test-time guard re-assert the port-name rule given the attribute already enforces it at compile time? **Resolved: yes, as harmless redundancy** — the guard already reads the attribute to discover ports, so an extra `assert!(!name.ends_with("Port"))` is one line and guards against a hypothetical attribute-macro regression. Revisit only if it adds noise to guard output.
- Should the naming guard also enforce that adapter structs live *only* in `identus-adapters-*` crates (a port implemented by a struct in a domain crate would be a layer violation already caught by the dep-graph guard)? Out of scope; the dep-graph guard already rejects domain→adapter-family edges, and a domain crate implementing a port for its own struct is not an adapter by definition. Leave to the layer guard.