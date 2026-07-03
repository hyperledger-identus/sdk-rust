## 1. `identus-derive` crate + `#[derive(Newtype)]` core (Phase 1)

- [x] 1.1 Add `crates/derive/` (`identus-derive`, `proc-macro = true`); add `identus-derive = { path = "crates/derive" }` to root `[workspace.dependencies]`; add `syn` 2, `quote` 1, `proc-macro2` 1 to root `[workspace.dependencies]` (per `workspace-dependency-conventions`)
- [x] 1.2 `crates/derive/Cargo.toml`: reference `syn`/`quote`/`proc-macro2` via `<dep>.workspace = true`; `[lib] proc-macro = true`; workspace lints
- [x] 1.3 `src/lib.rs`: expose `#[proc_macro_derive(Newtype, attributes(newtype))]`; parse the `DeriveInput`, assert a tuple struct with exactly one unnamed field (else compile error); SHALL NOT declare a `pub const COMPONENT` (proc-macro crate, exempt per the `core-error-conventions` "Component metadata" carve-out)
- [x] 1.4 `src/attr.rs`: parse `#[newtype(...)]` attributes — `display`, `display = "hex"|"base64url"`, `serde`, `parse = <path>, err = <type>`
- [x] 1.5 `src/category.rs`: inspect the single field's `syn::Type`; classify as string (`String`), bytes (`Vec<u8>`), or numeric (integer/float primitives); compile error on unrecognised inner type
- [x] 1.6 `src/str.rs`: for `String` category, emit `new`, `as_str`, `AsRef<str>`, `From<String>`, `From<&str>`; emit `Display` when `display`; emit transparent `Serialize`/`Deserialize` when `serde`; emit `FromStr` + inherent `parse` when `parse` configured
- [x] 1.7 `src/bytes.rs`: for `Vec<u8>` category, emit `new`, `as_bytes`, `into_bytes`, `AsRef<[u8]>`, `From<Vec<u8>>`, `From<&[u8]>`; `Display` hex (default) or base64url (when `display = "base64url"`); serde transparent (bytes); `FromStr` when `parse`
- [x] 1.8 `src/num.rs`: for numeric category, emit `new`, `get`, `From<Inner>`; `Display` when `display`; serde transparent (number); `parse` delegates to the validation fn
- [x] 1.9 Macro tests: `trybuild` (dev-dep, workspace-declared) positive + compile-fail cases (zero/two fields, named field, unrecognised inner type); per-category expansion unit tests

## 2. Layer-rule carve-out for proc-macro crates (Phase 2)

- [x] 2.1 In `crates/conformance/src/lib.rs`, extend `LAYER_RULES` member entries with a `proc_macro: bool` field; set `identus-derive` (foundation) `proc_macro = true`, all others `false`
- [x] 2.2 Update the dep-graph guard: a dependency edge whose target is `proc_macro = true` in `LAYER_RULES` is exempt from the inward-direction check (always permitted)
- [x] 2.3 Update the "Foundation has no workspace dependencies" assertion to treat `proc_macro = true` targets as exempt
- [x] 2.4 Guard scenarios: `identus-core → identus-derive` passes; `identus-core → identus-did` still fails; a production crate → `identus-conformance` is still rejected; conforming ring passes
- [x] 2.4a The guard's workspace-crate-count assertions SHALL be derived from `LAYER_RULES` membership (no hard-coded literal) — this change converges with `add-adapter-crate-layer`, which already switches the guard from the hard-coded `13` to a `LAYER_RULES`-derived count. Adding `identus-derive` to `LAYER_RULES` (task 2.1) makes the derived count include it automatically; no `13`→`14` literal bump remains. Update the stale "exactly the 13 crates" comment in `workspace_crate_names` to reflect the derived count.
- [x] 2.5 `cargo test -p identus-conformance` green
- [x] 2.6 Add `specs/core-error-conventions/spec.md` delta (MODIFIED "Foundation crate has no workspace dependencies"): `identus-core` SHALL declare no `identus-*` runtime dependencies, with a carve-out permitting a build-time dependency on a workspace crate whose `Cargo.toml` declares `[lib] proc-macro = true` (e.g. `identus-derive`); scenarios "Core is runtime-dependency-free" and "Core may depend on identus-derive". This mirrors the `crate-ring-layout` carve-out so the `identus-core → identus-derive` build-time edge does not violate `core-error-conventions`'s foundation invariant.
- [x] 2.6b Add `specs/core-error-conventions/spec.md` delta (MODIFIED "Component metadata"): narrow "Each crate" to "Each runtime crate" and exempt crates whose `Cargo.toml` declares `[lib] proc-macro = true` (e.g. `identus-derive`) from exposing `COMPONENT`, noting proc-macro crates cannot export non-macro items downstream; keep the "identus-core self-describes" scenario; add scenario "Proc-macro crates are exempt from COMPONENT" (verified by inspecting the crate's `src/lib.rs` for absence of `pub const COMPONENT`). This mirrors the dependency carve-out so `identus-derive` need not depend on `identus-core` for a `COMPONENT` it could not export anyway.
- [x] 2.7 Verify the `core-error-conventions` "Foundation crate has no workspace dependencies" scenarios stay green: `crates/core/Cargo.toml` has no `identus-*` dependency except `identus-derive` (proc-macro), so "Core is runtime-dependency-free" and "Core may depend on identus-derive" both hold

## 3. First adoptions (Phase 3)

- [x] 3.1 `crates/core/Cargo.toml`: add `identus-derive.workspace = true` (exempt per carve-out)
- [x] 3.2 `crates/core/src/url.rs`: `Url(String)` via `#[derive(Clone, Debug, PartialEq, Eq, Hash, Newtype)]` with `#[newtype(display, serde)]` and `#[newtype(parse = validate_url, err = UrlError)]`; hand-rolled `validate_url` (scheme + `://` + authority/path structure, no external dep)
- [x] 3.3 `crates/core/src/url.rs`: `UrlError` rich local error + hand-written `to_identus_error()` mapping to a stable `ErrorCode` (e.g. `core.invalid_url`) + `CapabilityId("core")`; assert `IdentusError` `Display` carries no runtime detail
- [x] 3.4 `crates/core/src/lib.rs`: wire `mod url;` re-exports; keep `COMPONENT` and existing error contract untouched
- [x] 3.5 `crates/did/Cargo.toml`: add `identus-derive.workspace = true`
- [x] 3.6 `crates/did/src/method.rs`: `DidMethod(String)` via the derive (str category, `display`, `serde`, `parse` with `did::Error`); hand-written `to_identus_error()` bridging
- [x] 3.7 `crates/did/src/multihash.rs`: `Multihash(Vec<u8>)` via the derive (bytes category, `display = "hex"`, `serde`); `as_bytes`/`into_bytes` exercised by `did:key`-style multihash identifier handling (method-agnostic, not PRISM-specific). No `parse`/`FromStr` in this adoption (matches the prior `DidSuffix` shape); a validated `parse` with a multihash code/length/digest structure check is a follow-on, not a v1 dogfood requirement
- [x] 3.8 One numeric newtype (e.g. a `Version`/`Port` in `identus-did` or `identus-core`) via the derive (num category, `display`, `serde`, bounds-checked `parse`) to exercise the numeric path

## 4. Conformance + hardening (Phase 4)

- [x] 4.1 Adoption tests: `Url`/`DidMethod`/`Multihash` round-trips; invalid-input rejection; serde round-trips (string/bytes/number); bytes `Display` hex vs base64url
- [x] 4.2 Error-bridging tests: each adopted newtype's `FromStr::Err` → `to_identus_error()` yields the stable `ErrorCode` + correct `CapabilityId`; `IdentusError` `Display` carries no runtime detail
- [x] 4.2a Confirm `crates/derive/src/lib.rs` contains no `pub const COMPONENT` and that no test or guard requires one of `identus-derive` (the proc-macro exemption from `core-error-conventions` "Component metadata" holds)
- [x] 4.3 Layer guard green incl. the new `proc_macro` carve-out scenarios; `identus-core` runtime-dependency-free assertion holds
- [x] 4.4 `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test --workspace` pass
- [x] 4.5 `nix flake check` green (incl. `deny.toml`: `syn`/`quote`/`proc-macro2`/`trybuild` licenses + duplicate-version policy; if `multiple-versions = "deny"`, confirm `syn` is single-version or apply a justified scoped `[bans] skip`)
- [x] 4.6 Confirm `identus-core` has no external runtime dep (the `Url` validator is hand-rolled; no `url`/`uriparse` pulled in)

## 5. OpenSpec artifacts & cross-change archive ordering

- [x] 5.1 Finalize `proposal.md`, `design.md`, `specs/crate-ring-layout/spec.md`, `specs/core-error-conventions/spec.md`, `tasks.md`. The `crate-ring-layout` delta MODIFIES the same two headers `add-adapter-crate-layer` touches — `Crate stubs at minimal code depth` (retargeted from `Twelve crate stubs at minimal code depth` after `add-adapter-crate-layer`'s RENAMED) and `Workspace dependency map`; both bodies are the `LAYER_RULES`-derived / non-literal combined form carrying both `identus-derive` and `identus-adapters-entropy`.
- [x] 5.2 `openspec validate add-domain-newtype-macro --strict` passes structurally. This change is **archive-blocked-by `add-adapter-crate-layer`**: its MODIFIED header `Crate stubs at minimal code depth` does not exist in the current main spec until `add-adapter-crate-layer` archives (which renames `Twelve crate stubs at minimal code depth` → `Crate stubs at minimal code depth`). Archive this change only after `add-adapter-crate-layer` has archived; at archive time `buildUpdatedSpec` resolves the retargeted MODIFIED against the renamed header. (`add-adapter-crate-layer` archived 2026-07-01; the blocker is resolved and the retargeted MODIFIED headers now exist in the main spec.)