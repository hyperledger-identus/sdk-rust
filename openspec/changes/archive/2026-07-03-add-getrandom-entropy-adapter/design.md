## Context

`identus-adapters-entropy` is the outer-boundary crate owning concrete backends
for the `identus_crypto::SecureRandom` port (the only infrastructure port in the
domain layer). It currently ships one production adapter —
`RingSystemRandomAdapter` (feature = `ring"`) — plus a deterministic test
adapter (feature = `"deterministic"`).

The `crypto` spec defines `SecureRandom` as a *port* precisely because a known
second backend must exist: `ring` does not build on `wasm32-unknown-unknown`,
and the upcoming uniffi **Kotlin + WASM** bindings need entropy on the WASM
path. The spec already names the second backend — "a `getrandom`-backed wasm
adapter ... deferred to that future binding change" — and the crate's own
`lib.rs` doc-comment pre-commits the name `GetrandomSystemRandomAdapter`.
This change realizes that deferred adapter.

Current state confirmed by codebase investigation:

- `ring` is a workspace dependency consumed **only** by `adapters-entropy`.
  `identus-crypto` deliberately excludes `ring` (its Cargo.toml says so), and
  no other crate depends on it.
- `sdk-ts`'s WASM build pipeline (`externals/run.sh`) standardizes on
  `wasm-pack build --target=web` (browser WASM) and on
  `getrandom = { features = ["js"] }` (see `externals/anoncreds/wasm/Cargo.toml`,
  `externals/didcomm/wasm`). The workspace `AGENTS.md` pins the sdk-ts devshell
  to the `wasm32-unknown-unknown` target. **No WASI target exists anywhere in
  the workspace or `sdk-ts`.**
- Both `ring::rand::SystemRandom` and `getrandom` delegate to the same OS
  CSPRNG (`getrandom(2)` / `CCRandomGenerateBytes` / `BCryptGenRandom` /
  `crypto.getRandomValues()`). `getrandom` is not a security downgrade.

## Goals / Non-Goals

**Goals:**

- Provide a production entropy adapter that compiles on every uniffi target —
  Kotlin/JVM, Android, native, and browser WASM
  (`wasm32-unknown-unknown`).
- Reuse the established `sdk-ts` WASM convention (`getrandom` + `js` feature)
  so the Rust SDK's entropy path matches the rest of the workspace.
- Formalize the `adapters-entropy` capability with a spec (none existed; the
  ring adapter landed without one via the consumed
  `add-adapter-crate-layer` change).
- Resolve the disposition of the legacy `ring` adapter as a gated decision
  before implementation.

**Non-Goals:**

- **WASI** (`wasm32-wasi` / `wasip1`) support. Out of scope and out of the
  workspace's target set. The `getrandom` `rdrand`-on-WASI complication is
  explicitly ignored.
- Changing the `SecureRandom` *port* signature or any behavior in
  `identus-crypto`. The port is unchanged; only its backends move.
- Touching the `DeterministicRandomAdapter` — it is orthogonal and unchanged.
- The uniffi Kotlin+WASM *bindings* themselves. This change only lands the
  entropy adapter the bindings will consume; the binding work is a separate
  change.

## Decisions

### D1 — Backend: `getrandom`, not `ring`

**Choice:** Add a `getrandom`-backed `GetrandomSystemRandomAdapter` as the
cross-platform adapter. Drop `ring` from the entropy crate (subject to Q3
gate).

**Why over alternatives:** `getrandom` is the only option in the candidate
set that compiles on `wasm32-unknown-unknown`. `ring` cannot serve the WASM
path at all. Security is equivalent (same OS CSPRNG). `getrandom` is lighter
(maintained by RustCrypto, a transitive dep of `rand`, audited).

**Alternative considered — keep both `ring` and `getrandom` adapters:** More
flexible, but `ring` adds `boringssl` weight for zero benefit since `getrandom`
covers every target `ring` did. With nothing else in the tree depending on
`ring`, keeping it is pure maintenance surface. Rejected (pending Q3 gate
confirmation).

### D2 — Feature wiring: `getrandom` feature enables `dep:getrandom` with `js`

**Choice:** A single `getrandom` cargo feature on `adapters-entropy` enables
`dep:getrandom` with the `js` feature enabled unconditionally.

**Why:** `getrandom`'s `js` feature is only active on `wasm32-unknown-unknown`
(where it `import`s `crypto.getRandomValues`); on native targets it is a no-op.
Enabling it unconditionally means one feature flag works on every target
without per-target `cfg` gymnastics in the manifest, and it mirrors exactly
what `sdk-ts`'s `externals/anoncreds/wasm` does.

**Alternative considered — separate `getrandom` / `getrandom-wasm` features
selected by the consumer:** More explicit, but pushes target awareness onto
every consumer and diverges from the `sdk-ts` precedent. Rejected.

### D3 — Naming: `GetrandomSystemRandomAdapter`

**Choice:** Name the new adapter `GetrandomSystemRandomAdapter`.

**Why:** The crate's `lib.rs` doc-comment already commits to this name, and it
follows the existing `<Backend><Capability>Adapter` convention
(`RingSystemRandomAdapter`, `DeterministicRandomAdapter`). Symmetric and
discoverable.

### D4 — Q3 disposition of `ring`: recommend clean cut, gated

**Choice:** Recommend removing `RingSystemRandomAdapter` and the `ring`
feature (and the workspace `ring` dependency line) entirely. This is gated
behind task 1, a consumer-impact check.

**Why:** `ring` is consumed only by this crate; `getrandom` covers every
target `ring` did with identical OS-CSPRNG security; removing it is a net
dependency reduction and removes a non-WASM-capable code path from a
WASM-capable crate.

**Gate:** If task 1 finds real published consumers of
`identus-adapters-entropy` with `features = ["ring"]` and a stability
commitment, the decision flips to **deprecate-then-remove**
(`#[deprecated]` on the `ring` feature + adapter for one release, both
adapters coexisting temporarily). In that case the `adapters-entropy` spec
delta is amended to retain a `RingSystemRandomAdapter` requirement marked
deprecated. See Open Questions.

**Resolved Q3 outcome (task 1):** `identus-adapters-entropy` is **not
published** to crates.io (workspace `version = "0.0.0"`, pre-release), and
no in-tree or external consumer enables it with `features = ["ring"]`.
The `ring` workspace dependency is consumed **only** by this single crate,
and nothing depends on `identus-adapters-entropy` as a Cargo dependency with
the `ring` feature (the only `identus-adapters-entropy` references in-tree
are the workspace dep map, doc comments, and the `conformance` guard that
asserts domain crates do *not* depend on it). There is no published-API
stability commitment to preserve. **The gate passes: clean cut.** The
`ring` adapter, the `ring` cargo feature, and the workspace `ring`
dependency line are removed; the spec deltas stand as written (clean-cut
end state); tasks 3.x / 4.x proceed with the clean-cut branches.

### D5 — WASI explicitly excluded

**Choice:** No WASI target, no `rdrand`/WASI fallback. Browser WASM only.

**Why:** The workspace target set is `wasm32-unknown-unknown` only (confirmed
via `sdk-ts`'s `wasm-pack --target=web` pipeline and the workspace devshell
config). Adding WASI would introduce the messy `getrandom`-on-WASI fallback
for a target nobody consumes.

## Risks / Trade-offs

- **[Q3 misjudged — breaking a hidden consumer]** → Mitigation: task 1 is an
  explicit consumer-impact gate *before* any breaking edit. If consumers
  exist, flip to deprecate-then-remove (D4 fallback). No breaking edit lands
  until the gate passes.
- **[Spec bakes in clean-cut before the gate resolves]** → The
  `adapters-entropy` and `crypto` spec deltas are written to the recommended
  end state (clean cut). The `crypto` delta contains two MODIFIED
  requirements — `SecureRandom — the single infrastructure port` and
  `Wasm-clean by default` — both written to the clean-cut end state. If the
  gate flips to deprecate, task 1's output includes amending the spec deltas
  to retain a deprecated `RingSystemRandomAdapter` requirement *and*
  re-amending the `Wasm-clean by default` MODIFIED entry to restore the
  localization contrast (under deprecate the `ring` adapter stays in
  `identus-adapters-entropy`, so "only the adapter crate is wasm-incompatible"
  is true again). This is a documented task (6.1), not a silent drift.
- **[`getrandom` `js` feature surprises a native consumer]** → Mitigation:
  the `js` feature is a documented no-op on non-wasm targets (upstream
  `getrandom` contract). The `sdk-ts` externals already rely on this. No
  native behavior changes.
- **[Entropy quality regression]** → Not a real risk: `getrandom` calls the
  identical OS CSPRNG that `ring` called. Documented in D1; no mitigation
  needed beyond the design record.
- **[Workspace `ring` dep line left dangling]** → If Q3 = clean cut, the
  workspace `[workspace.dependencies]` `ring = "0.17"` line must be removed,
  not left as an unused workspace dep (workspace lints would otherwise flag
  it). Tracked in tasks.

## Migration Plan

This is a library-internal adapter change with no persistent state and no
runtime data to migrate.

- **If Q3 resolves to clean cut:** consumers enabling
  `features = ["ring"]` switch to `features = ["getrandom"]` and replace
  `RingSystemRandomAdapter` with `GetrandomSystemRandomAdapter` (same
  `SecureRandom` interface, drop-in). The composition root's injected
  adapter changes; nothing else does.
- **If Q3 resolves to deprecate:** `ring` feature stays buildable, marked
  `#[deprecated]`, with a migration note pointing to `getrandom`; removal
  happens in a follow-up release.
- **Rollback:** revert the change; `RingSystemRandomAdapter` and the `ring`
  feature are restored from git. No data migration to undo.

## Open Questions

- **Q3 (the gate):** Are there published / external consumers of
  `identus-adapters-entropy` enabling `features = ["ring"]` with a stability
  expectation? Resolved by task 1. Recommendation: clean cut (D4). If the
  answer is "yes, with stability commitment," flip to deprecate-then-remove
  and amend the spec deltas accordingly.