## Context

The workspace is a 13-crate hexagonal ring (`crate-ring-layout`) with `identus-core` as the zero-workspace-dependency foundation owning the redaction-safe `IdentusError` contract (`core-error-conventions`). Domain primitives live in the `domain-primitives` layer (`identus-did`, `identus-trust`, …). Every domain crate is currently a `COMPONENT`-only stub. The recurring "newtype over `String`/`Vec<u8>`/numeric" pattern is already visible by hand in: `identus-core`'s `CapabilityId`/`ErrorCode` (`&'static str`, `const fn`), neoprism's `Uri`/`PublicKeyId`/`ServiceId`/`ServiceTypeValue` (owned `String` with validated `parse`/`FromStr`, `Display`, `Eq`/`Hash`, serde), and neoprism's `DidSuffix` (`Vec<u8>`, `as_bytes`/`into_bytes`, serde). Two in-flight changes (`add-crypto-capability`, `add-workspace-dependency-conventions`) establish the conventions this change follows: external deps declared once at workspace level and referenced via `.workspace = true`; a rich local error type bridged to the redaction-safe `IdentusError` via a hand-written `to_identus_error()`.

neoprism already pulls `lazybe::Newtype`, but that derive is database-oriented (it generates DB encoder/decoder glue, not domain-newtype accessors/`FromStr`/`Display`), so it does not satisfy the stated need; the user's framing ("define a common macro which packages the newtype pattern") and the workspace's "own the canonical source" ethos (cf. `add-crypto-capability` Decision 1) argue for a purpose-built owned derive rather than adopting `derive_more`/`nutype`.

## Goals / Non-Goals

**Goals:**
- One `#[derive(Newtype)]` derive that packages the newtype pattern for string/numeric/bytes inner types, generating exactly the category-appropriate boilerplate (constructors, accessors, `AsRef`/`From`, `Display`, fallible `parse`/`FromStr`, optional serde).
- Dogfood it on the first real domain types: `Url` (in `identus-core`), `DidMethod` + `Multihash` (in `identus-did`), plus one numeric example. `Multihash` is the canonical method-agnostic bytes identifier underlying `did:key` (and the wider multihash/multibase ecosystem); it fills the bytes-category slot previously held by the PRISM-specific `DidSuffix`, exercising the same `as_bytes`/`into_bytes` + hex `Display` + serde-string surface without tying the dogfood to a `did:prism`-only type. (neoprism's existing hand-written `DidSuffix` remains the pattern-evidence that motivates the macro; this change simply does not adopt a PRISM-specific type.)
- Make `identus-core` able to use the derive without breaking the `foundation → none` layer rule, via a clean, general proc-macro carve-out in the layer guard.
- Keep domain-specific error bridging hand-written (the macro generates generic boilerplate only), preserving the `core-error-conventions` two-surface split.

**Non-Goals:**
- Generating `const fn` constructors (proc-macros cannot reliably emit `const fn`). `CapabilityId`/`ErrorCode` and any future `const fn` newtypes stay hand-written.
- Generating domain-specific trait impls (e.g. a `DidMethodSpecific` trait). The macro implements only standard-library + serde traits; domain traits stay hand-written on top.
- Migrating neoprism to use `identus-derive`. Out of scope, same as `add-crypto-capability`'s non-goal.
- A FFI/UniFFI binding surface for the new types. That is the `add-bindings` change.
- Replacing `derive_more` where neoprism already uses it. This change owns a new SDK-internal macro; it does not touch neoprism.

## Decisions

### Decision 1: Build an owned proc-macro derive crate (`identus-derive`), not `macro_rules!` and not an adopted crate

A new foundation-layer crate `identus-derive` (`proc-macro = true`) exposes `#[derive(Newtype)]`. It inspects the single unnamed field's `syn::Type` and dispatches to the str/bytes/num category, then emits the category-appropriate impls. Helper attributes live under `#[newtype(...)]`.

**Rationale:** the user explicitly asked to *define* a macro, and the workspace's "own the canonical source that bindings port from" ethos (Decision 1 of `add-crypto-capability`) favors owning it. A proc-macro derive can branch on the inner type automatically (the key "if it makes sense" requirement) and integrates cleanly with `#[derive(Serialize, Deserialize)]` via attribute-gated opt-in. `macro_rules!` would force the caller to declare the category explicitly and makes conditional serde awkward; adopting `nutype`/`derive_more` would make an external crate the source of truth and add a supply-chain dep the SDK does not control.

**Alternatives considered:**
- *`macro_rules!` exported from `identus-core`*: rejected; can't inspect the inner type, conditional serde is ugly, and it would still need the user to state the category. Lightest option (zero deps), but loses the automatic "implement the trait if it makes sense" behavior the user asked for.
- *Adopt `nutype` (validation + serde) or `derive_more` (plain derives) + hand-written validation*: rejected against the own-the-source ethos; `derive_more` also does not generate validation/`parse`/accessors, so it would not "package the pattern" on its own.

### Decision 2: Three inner categories — string, bytes, numeric — each with a fixed trait set

The derive recognises the inner type and selects a category. **v1 ships only the owned forms** (`String`, `Vec<u8>`, numeric primitives); `Box<str>`, `&'static str`, `&[u8]`, and `[u8; N]` are recognised by the design but deferred to a follow-on (see Open Questions). The table shows the full intended category shape, with the v1-supported inner types in the first column bolded:

| Category | v1 inner types (deferred) | Generated accessor | `AsRef` | `From` | `Display` |
|---|---|---|---|---|---|
| string | **`String`** (`Box<str>`, `&'static str`) | `as_str()` | `AsRef<str>` | `From<String>`, `From<&str>` | pass-through |
| bytes | **`Vec<u8>`** (`&[u8]`, `[u8; N]`) | `as_bytes()` (+ `into_bytes()` for owned) | `AsRef<[u8]>` | `From<Vec<u8>>`, `From<&[u8]>` | hex **or** base64url (selectable, default hex) |
| numeric | `u8`..`u128`, `i8`..`i128`, `usize`, `f32`/`f64` | `get()` (+ `Into<Inner>`) | `AsRef<Inner>` not generated (trivially copyable) | `From<Inner>` | numeric formatting |

`Debug`/`Clone`/`Copy`/`PartialEq`/`Eq`/`Hash` are **not** generated by the derive — the caller composes standard `#[derive(...)]` on the struct. The macro only owns the boilerplate that standard derives cannot provide (accessors, `AsRef`/`From`, `Display`, `FromStr`, serde-transparent).

**Rationale:** a fixed, predictable per-category trait set is the "if it makes sense" behaviour the user asked for; making it attribute-driven per-trait would re-introduce the boilerplate the macro exists to remove.

### Decision 3: Fallible `parse` via a caller-supplied validation function + error type; rich error bridging stays hand-written

`#[newtype(parse = <path>, err = <type>)]` generates `FromStr` (with `Err = <type>`) and an inherent `parse(s: &str) -> Result<Self, <type>>` that calls `<path>(s) -> Result<(), <type>>`. The owning crate hand-writes `to_identus_error()` on `<type>` to map to a stable `ErrorCode` + `CapabilityId` with a redaction-safe `&'static str` message.

**Rationale:** the macro cannot invent domain-specific error variants or the stable `ErrorCode` catalogue (those are per-capability compatibility contracts per `core-error-conventions`). This matches the `add-crypto-capability` split: generic boilerplate macro-generated, rich local error + bridging hand-written.

**Alternatives considered:**
- *Macro generates the error type from a `code` attribute*: rejected; would force every newtype to use a single generic error variant and lose the stable-code catalogue contract.

### Decision 4: `Url` lives in `identus-core`; `identus-derive` gets a proc-macro carve-out from the layer rules

`Url` is a generic shared primitive consumed by `did`, `openid4vc`, and `trust`; placing it in foundation means those crates depend inward on `identus-core` (already allowed) rather than on `identus-did`. But `Url` is built with `#[derive(Newtype)]`, so `identus-core` depends on `identus-derive` — which `crate-ring-layout`'s `foundation → none` rule forbids. Resolution: `LAYER_RULES` members gain a `proc_macro: bool` flag; the dep-graph guard **exempts** any workspace crate flagged `proc_macro = true` from the inward-direction check, so any crate may depend on a proc-macro crate regardless of layer. `identus-derive` is a foundation member flagged `proc_macro = true`. As a `proc-macro = true` crate it ships no `pub const COMPONENT` — it is exempt from the `core-error-conventions` "Component metadata" obligation (proc-macro crates cannot export non-macro items to downstream crates, so a `COMPONENT` there would be unreadable by `identus-conformance` or any other crate), and the exemption keeps `identus-derive` from needing a runtime dependency on `identus-core` that would itself violate `foundation → none`.

**Rationale:** proc-macro crates are build-time tooling, not runtime domain dependencies; treating them as exempt is a clean, general rule (not a one-off `identus-core`-only hack) and keeps the foundation-is-dependency-free *runtime* invariant intact. This is the smallest change to `crate-ring-layout` that unblocks the chosen placement.

**Alternatives considered:**
- *Put `Url` in `identus-did` instead* (the original recommendation): rejected by the user; `Url` is shared and belongs in foundation.
- *Hand-write `Url` so core doesn't use the derive*: rejected; then `Url` doesn't dogfood the macro and the macro's own stated example type isn't built with it.
- *Allow `foundation → foundation` generally*: rejected; too broad a relaxation; the proc-macro carve-out is precise.

### Decision 5: `Url` validation backend — hand-rolled, no new external dep in `identus-core`

`Url`'s validation function is hand-rolled (scheme + `://` + authority/path structure check) and lives in `identus-core`'s `url` module. `identus-core` does **not** pull the `url` or `uriparse` crate; it stays external-dep-free (it already has zero deps, and `crate-ring-layout` treats foundation as dependency-free at runtime).

**Rationale:** introducing the `url` crate into foundation would be the first external dep in `identus-core` and a precedent for the zero-dep foundation; a hand-rolled validator is sufficient for the SDK's URL needs (DID service endpoints, issuer URLs) and keeps foundation clean. If richer RFC-3986 conformance is later required, a separate change can add a vetted `url` dep with an explicit compatibility decision.

**Alternatives considered:**
- *Use the `url` crate (servo, audited)*: rejected for now; adds a foundation external dep and a transitive dep surface (`idna`, `percent-encoding`) for a need that a small validator serves.

## Risks / Trade-offs

- **[Risk] proc-macro build cost** → `identus-derive` adds `syn`/`quote`/`proc-macro2` to the graph. These are ubiquitous, MIT/Apache-licensed, and `cargo-deny`/`rust-audit` will cover them; `multiple-versions = "warn"` (per current `deny.toml`; `add-supply-chain-audit-tooling` may tighten to `deny`) could surface a duplicate if another crate pulls `syn` at a different version. Mitigation: pin `syn` 2 at workspace level; if a duplicate appears, a scoped `[bans] skip` with justification is the documented escape hatch.
- **[Risk] proc-macro carve-out weakens the layer guard** → the exemption is `proc_macro = true`-gated, not blanket; a test asserts `identus-core → identus-did` (non-proc-macro) is still rejected. The carve-out is auditable in `LAYER_RULES`.
- **[Trade-off] hand-rolled URL validation is not full RFC-3986** → accepted for the SDK's URL scope; flagged as a future-change escape hatch.
- **[Risk] `trybuild` as a dev-dependency** → `trybuild` is MIT/Apache and dev-only (does not enter the production graph); declared at workspace level per `workspace-dependency-conventions`.

## Migration Plan

1. Land `identus-derive` (crate + `#[derive(Newtype)]` + category dispatch + attribute parsing) with macro unit tests (Phase 1).
2. Extend `LAYER_RULES` + guard with the `proc_macro` carve-out and `identus-derive` membership (Phase 2).
3. Adopt in `identus-core` (`Url` + `UrlError` + `to_identus_error()`) and `identus-did` (`DidMethod`, `Multihash`) (Phase 3).
4. Conformance + hardening (Phase 4): layer guard green (incl. the new carve-out scenario), fmt/clippy/test/`nix flake check`.

**Rollback:** the change is additive to stubs; reverting removes `crates/derive`, the `Url`/`DidMethod`/`Multihash` modules, and the `LAYER_RULES` `proc_macro` flag, returning the affected crates to their `COMPONENT`-only stubs.

## Open Questions

- **Bytes `Display` default**: hex (matches neoprism's `HexStr`) or base64url (matches neoprism's `Base64UrlStrNoPad`)? Default: hex, overridable via `#[newtype(display = "base64url")]`.
- **`&'static str` string category**: generate `const fn new` for the `&'static str` inner case only (where `const fn` *is* legal), or keep the macro uniform and leave `const fn` to hand-written newtypes like `CapabilityId`? Default: uniform (no `const fn`); `CapabilityId`/`ErrorCode` stay hand-written.
- **`Box<str>`/`[u8; N]` support in v1**: ship `String`/`Vec<u8>`/numeric first and add `Box<str>`/`[u8; N]` later, or support all listed inner types up front? Default (decided): `String`/`Vec<u8>`/numeric in v1; `Box<str>`/`&'static str`/`&[u8]`/`[u8; N]` deferred — the Decision 2 table marks these as deferred.
- **Bytes-category `serde` encoding**: when `#[newtype(serde)]` is set on a `Vec<u8>`-backed type, should transparent serde emit a JSON string (hex or base64url, matching `Display`) or a raw byte array (`[u8]` → array of numbers)? Default (decided): emit a string using the same encoding as `Display` (hex by default, base64url when `display = "base64url"`), so `Multihash` round-trips as a hex/base64url string over JSON rather than a number array. The `domain-newtype-macro` spec SHALL state this explicitly.