## Context

`#[derive(Newtype)]` (shipped by the archived `add-domain-newtype-macro` change) generates, for every category, an infallible `new(inner) -> Self` and infallible `From<Inner>` conversions that perform **no validation**, plus a fallible `parse`/`FromStr` only when `#[newtype(parse = …, err = …)]` is configured — where the validation function has signature `fn(&str) -> Result<(), Err>`. The shipped spec codifies this: *"Infallible `new` is always available … SHALL NOT validate"* and *"`new` and `parse` coexist."*

For grammar-bearing types this leaves three bypasses open:

1. **`new`** — explicit, greppable.
2. **`From<Inner>` / `From<&str>` / `From<&[u8]>`** — quiet; fire via `into()`, `?`, coercion.
3. **Generated `Deserialize`** — does `Self(inner)` directly, no validation. This is the untrusted input boundary (network, files, configs) and the most dangerous hole.

A second problem with the shipped design is the **validator signature itself**: `fn(&str) -> Result<(), Err>` forces every category through a string-shaped validation function, even when the inner type is not a string. The worst case is numeric: `validate_version(&str)` must internally `s.parse::<u8>()` just to recover the number the caller already held, so the natural number-in entry (`new(u8)`) is infallible and unvalidated while the validated entry (`parse(&str)`) is string-shaped and wasteful. For bytes, the validator runs on the *encoded* `&str` and is contractually coupled to the display encoding (it "must guarantee the string decodes"), which conflates two concerns (grammar over the decoded bytes vs. well-formedness of the encoding).

Current dogfood newtypes:

```
   type        category   validate_fn?  validator enforces            bypasses today
   ──────────  ─────────  ────────────  ─────────────────────────────  ─────────────────
   DidMethod   string     yes (parse)   W3C method-name grammar      new, From<String>,
                                                                       Deserialize
   Url         string     yes (parse)   URL grammar (scheme + ://)     new, From<String>,
                                                                       Deserialize
   Version     numeric    yes (parse)   u8 parse + non-zero           new(0), From<u8>,
                                                                       Deserialize
   Multihash   bytes      no            (none — deferred)             n/a (no validator)
```

`ErrorCode`/`CapabilityId` are hand-written `&'static str` newtypes, **not** `#[derive(Newtype)]`, and are unaffected. All current call sites of the removed constructors and all definitions of the validator functions are in-crate (verified by repo-wide grep); no production call sites exist, so migration is mechanical now. Once these types are adopted across cloud-agent adapters and bindings, removal becomes a breaking change — this is the window.

## Goals / Non-Goals

**Goals:**
- Make a `validate_fn`-configured newtype's invariant hold across **every** construction path: inherent constructors, trait conversions, and `Deserialize`.
- Replace the string-shaped `fn(&str)` validator with one invoked by the macro as `<path>(&inner)` over the inner type, so the validator reasons about the type's actual domain representation (a number, a byte buffer, a string), not a serialization artifact. The function's parameter may be `&Inner` or any type `&Inner` derefs to (e.g. `&str` for `String`, `&[u8]` for `Vec<u8>`, `&u8` for numeric), letting each validator pick its idiomatic borrowed form.
- Replace the quiet infallible `From<Inner>` bypasses with a fallible `TryFrom<Inner>` that validates (move semantics, no realloc on the owned path that `FromStr` cannot provide cheaply).
- Provide a uniform, discoverable inherent validated constructor `try_new(Inner) -> Result<Self, Err>` for every category (delegates to `TryFrom<Inner>`), so callers need not import a trait and the constructor reads as a constructor, not a conversion.
- Retain a fast trusted-construction path via a named, greppable `new_unchecked` hatch with fixed `pub(crate)` visibility — enforced at the crate boundary, only the defining crate can bypass.
- Drop the string-shaped `parse`/`FromStr` entry for numeric and bytes (it was a validator-shaped artifact, not a natural input); keep it for string where `&str` is the natural input.
- Keep non-`validate_fn` types (`Multihash`) exactly as they are — forbidding construction when no validator exists would make a type unconstructable.

**Non-Goals:**
- Adding a validator to `Multihash` (deferred follow-on; out of scope).
- Changing `ErrorCode`/`CapabilityId` (not `#[derive(Newtype)]` types).
- Revising `Display`, `as_str`/`as_bytes`/`get` accessors, `Serialize`, or the bytes hex/base64url encoding.
- A richer bytes-decode error type — `nt_decode_` continues to return `Result<_, &'static str>`; only its errors are now surfaced as `D::Error::custom(...)` instead of `expect`-ed.
- Const construction: `new_unchecked` is not `const` (the existing `new` was not either); out of scope.
- Cross-crate trusted construction: the derive provides no `pub` hatch; a type needing it would hand-write the constructor.

## Decisions

### Decision 1 — Goal is the full invariant (Option D), not accident-prevention (Option B)

Forbid `new`/`From` **and** make `Deserialize` validate. Rationale: forbidding `new` while leaving `Deserialize` doing `Self(inner)` is cosmetic — it closes the greppable path and leaves the untrusted boundary wide open, creating a false sense of safety. The serde path is *where invalid values enter the system*; closing it is the entire point.

**Alternatives considered:**
- **B (rename `new`, leave serde transparent)**: cheaper, but overclaims nothing — "speed bump" not invariant. Rejected.
- **A (validate_fn only, remove all infallible) / C (TryFrom + validate_fn, serde transparent)**: both leave the serde hole. Rejected.

### Decision 2 — The rule keys on `validate_fn` being configured, not on category

Apply the full contract (remove `new`/`From`, add `TryFrom<Inner>` + `try_new` + `new_unchecked`, validate serde) to **every category that has `validate_fn`** — string, bytes, numeric. The trigger is "a validator exists," not "the inner is a string." `Multihash` (bytes, no `validate_fn`) is the sole exception and keeps infallible construction.

**Evidence that drove this:** `validate_version` enforces **non-zero**, a semantic rule `u8`'s type does not guarantee. `Version::new(0)` and `Version::from(0)` already construct a value the validator explicitly rejects — the existing test `version_infallible_new_does_not_validate` documents this as current behavior. Under D, that test is a bug report this change fixes. The exploration's initial lean ("numeric keeps `new`; bounds ≠ grammar") assumed the numeric validator was pure bounds-checking `new` couldn't violate; it isn't.

**Alternatives considered:**
- Numeric keeps `new` (bounds ≠ grammar): invalidated by `validate_version`'s non-zero rule. Rejected.

### Decision 3 — Validator is invoked as `<path>(&inner)`; parameter is `&Inner` or any type `&Inner` derefs to

The macro emits a single call site `validate_fn(&inner)` where `inner: Inner`, and the compiler's deref coercion at that call site does the rest. The validator's first parameter may therefore be `&Inner` **or** any type `&Inner` derefs to — `&str` for `String`, `&[u8]` for `Vec<u8>`, `&u8` for numeric — and the macro places no further constraint on it (proc-macros work on tokens and cannot resolve types anyway). This removes the string-round-trip problem for numeric (the validator sees `&u8` directly, no `s.parse::<u8>()`), decouples the bytes validator from the display encoding (it can take `&[u8]`, not the encoded string), and lets each validator pick its idiomatic borrowed form — in particular `&str`/`&[u8]` rather than `&String`/`&Vec<u8>`, which would trip `clippy::ptr_arg` (this project gates on clippy via `nix flake check`). The attribute is renamed `parse`/`err` → `validate_fn`/`validate_err` to reflect that it names a validator over the inner type, not a string parser. `validate_fn` and `validate_err` are required together; `validate_err` must match `validate_fn`'s return type (the macro names it in `FromStr::Err`/`TryFrom::Error`/`Deserialize`).

**Alternatives considered:**
- Keep `fn(&str)`, add a separate number-in path: leaves the bytes-encoding coupling and the numeric round-trip; two validator shapes. Rejected.
- Mandate a single `fn(&Inner)` signature per category (`&String`/`&Vec<u8>`/`&u8`): fights Rust idiom for the growable categories — `&String` and `&Vec<u8>` both trigger `clippy::ptr_arg`, forcing either an `#[allow]` or unidiomatic signatures. The macro cannot enforce such a mandate anyway (proc-macros don't resolve types); prescribing it in the spec would be aspirational prose the compiler doesn't back. Rejected in favor of the permissive “callable with `&inner`” contract, which lets each validator use `&str`/`&[u8]`/`&u8` idiomatically.
- Infer `validate_err` from `validate_fn`'s return type at expansion time: proc-macros work on tokens, not resolved types, so the return type cannot be reliably extracted from a bare path. Rejected — explicit `validate_err` is required.

### Decision 4 — `try_new(Inner)` is uniform across all categories

Every `validate_fn`-configured type gets an inherent `try_new(inner: Inner) -> Result<Self, Err>` that delegates to `TryFrom<Inner>` (one calls the other). Rationale: (1) an inherent named constructor is the thing users reach for first and is greppable, whereas `TryFrom` requires a trait import and reads as a conversion rather than a constructor; (2) the `&Inner` validator signature already removed the *reason* `try_new` was numeric-only in the prior iteration (the `u8 -> str -> u8` round-trip), so the only remaining argument for asymmetry is "less surface," which is weak given each `try_new` is one line delegating to `TryFrom`. Uniform also makes the spec simpler: every `validate_fn`-configured type gets `try_new`, `TryFrom<Inner>`, `new_unchecked`, validating `Deserialize`; string additionally gets `parse`/`FromStr`.

**Alternatives considered:**
- `try_new` numeric-only: adds a category special-case with no remaining justification. Rejected.

### Decision 5 — `TryFrom<Inner>` (owned, move semantics) replaces `From<Inner>`; no `TryFrom<&str>`/`TryFrom<&[u8]>`

`TryFrom<Inner>` validates `&inner` then moves `Self(inner)` — no realloc on the owned path — and is the direct fallible replacement for the removed `From<Inner>` (`s.into()` → `s.try_into()?`). No `TryFrom<&str>`/`TryFrom<&[u8]>`: for string, `FromStr` already covers the borrowed validated path (`parse(&str)`, one `to_owned()` on success); for numeric and bytes, the borrowed string path is dropped entirely (see Decision 6). This makes the rule **uniform across categories**: `validate_fn`-configured → `From<Inner>` replaced by `TryFrom<Inner>`; non-`validate_fn` → `From<Inner>` stays.

**Alternatives considered:**
- Emit `TryFrom<&str>`/`TryFrom<&[u8]>` too: duplicates `FromStr` for string and resurrects the string-shaped path for numeric/bytes that Decision 6 removes. Rejected.

### Decision 6 — `parse`/`FromStr` is string-only; dropped for numeric and bytes

`parse(&str)`/`FromStr` remains for the **string category** only, where `&str` is the natural input and `&str → String::to_owned()` is lossless: `parse(s)` does `let owned = s.to_owned(); validate_fn(&owned)?; Ok(Self(owned))`. It is **dropped for numeric and bytes**:

- **numeric:** the string-shaped entry existed only because the old `validate(&str)` signature forced one. With `validate(&u8)`, the natural entry is `try_new(u8)`/`TryFrom<u8>`; a caller with a string does `"5".parse::<u8>()?` then `Version::try_new(n)?` — two explicit steps with two distinct error types (parse-failure vs. validation-failure), which is clearer than mushing both into one `FromStr::Err`.
- **bytes:** the hex/base64url string path was always awkward and coupled the validator to the display encoding. With a bytes validator (`&[u8]` or `&Vec<u8>`), the validator reasons about bytes; the wire-format string is the job of serde (which decodes then validates — see Decision 7), and explicit construction is `try_from(Vec<u8>)` / `try_new(Vec<u8>)`. There is no `FromStr`/`parse` for bytes.

**Alternatives considered:**
- Keep `FromStr` for all categories: keeps a string-shaped entry that the new validator signature makes unnatural for numeric/bytes, and forces the bytes validator/decode coupling back. Rejected.
- Keep `FromStr` for bytes only (since serde is string-shaped): serde already covers the wire boundary; an explicit `FromStr` would duplicate it and re-couple construction to the display encoding. Rejected.

### Decision 7 — Hatch is `new_unchecked`, fixed `pub(crate)`, no attribute

- **Name:** `new_unchecked` — std-idiomatic (`NonZeroU8::new_unchecked`, `MaybeUninit::assume_init`); the `_unchecked` suffix is the established reviewer signal for "caller owes a proof the type can't provide."
- **Fixed visibility `pub(crate)`:** the invariant is enforced at the crate boundary — only the defining crate can bypass, and it owns the invariant. There is **no `hatch` attribute and no opt-in to `pub`**: offering a greppable widening path is still offering a path that, if taken, silently weakens the invariant — better not to offer it. If a future type genuinely needs cross-crate trusted construction, the derive cannot provide it; the type would hand-write that constructor (a deliberate, reviewable act, not a one-attribute flip).
- **No `From`-shaped hatch:** single named constructor; the removed `From<Inner>`/`From<&str>`/`From<&[u8]>` do not return in unchecked form.

**Alternatives considered:**
- `pub` default with `hatch = "pub(crate)"` to narrow: analyzed and rejected as "Option B with a different name" — a gentlemen's agreement any downstream crate can violate.
- `pub(crate)` default with opt-in `pub` via `#[newtype(hatch = "pub")]`: the prior design's choice. Rejected here in favor of fixed `pub(crate)` — simpler (one fewer attribute, one fewer branch) and strictly safer (no widening knob to be tempted into later).
- No hatch at all (strictest D): impractical — the owning crate needs a fast path for internal construction (e.g. building a `DidMethod` from a string just regex-matched). Rejected.

### Decision 8 — Validated `Deserialize`; bytes inverts to decode-then-validate with a real decode error

When `validate_fn` is configured, generated `Deserialize` validates:

- **string / numeric:** deserialize the inner value (`String` / the number), then call `validate_fn(&inner)`; on `Err`, return `Err(<D::Error>::custom(<validate_err>))` (or its `Display`); on `Ok`, construct `Self(inner)`.
- **bytes:** the wire format is the hex/base64url string (unchanged). Deserialize that string, then **decode** it via `nt_decode_` → on `Err`, return `Err(<D::Error>::custom(<decode-err>))`; on `Ok(bytes)`, call `validate_fn(&bytes)` → on `Err`, return `Err(<D::Error>::custom(<validate_err>))`; on `Ok`, `Self(bytes)`. This **inverts** the old flow (`validate(&str)` → `decode` with `expect`) and makes decode failure a first-class `D::Error` rather than an `expect`/panic. The old contract — "validation function must guarantee the string decodes in the display encoding" — is **deleted**: the validator now reasons about bytes and bears no responsibility for hex/base64url well-formedness; the macro owns decode and its errors.

When `validate_fn` is **not** configured, `Deserialize` stays transparent (`Self(inner)`).

**Alternatives considered:**
- Keep `expect`-on-decode for bytes (validator guarantees decodability): impossible once the validator sees the decoded bytes (`&[u8]`/`&Vec<u8>`) — it runs after decode and cannot pre-guarantee decodability. Rejected.
- Surface a richer decode error type: out of scope; `&'static str` (what `nt_decode_` returns today) suffices.

## Risks / Trade-offs

- **[Breaking change to `new`/`From<Inner>`]** → Mitigation: all current call sites are in-crate tests/docs (grep-verified); migration is mechanical (`new(x)` → `try_new(x)?`/`try_from(x)?`/`parse(&x)?` (string only), or `new_unchecked(x)` for trusted). Do it now before downstream adoption locks the surface.
- **[Validator signature change]** → The macro's contract relaxes from a mandated `fn(&str)` to “callable as `<path>(&inner)`”; dogfood validators move to `fn(&str)` (string) / `fn(&u8)` (numeric). Mitigation: the three `validate_*` functions are private to their modules; rewriting them is in-crate and mechanical. No external caller exists.
- **[Breaking attribute rename `parse`/`err` → `validate_fn`/`validate_err`]** → Mitigation: in-crate dogfood types only; mechanical rename.
- **[Breaking removal of `FromStr`/`parse` for numeric and bytes]** → Mitigation: numeric callers with a string do `s.parse::<u8>()?` then `try_new(n)?`; bytes callers use serde (wire) or `try_from(Vec<u8>)` (explicit). No in-crate call site relies on bytes/numeric `FromStr` beyond tests.
- **[Serde error surface changes for `validate_fn`-configured types]** → Mitigation: invalid input now fails deserialization with the type's error (and, for bytes, a decode error) instead of silently constructing; callers relying on "serde always accepts" must handle the error. This is the intended behavior, not a regression.
- **[Validation cost on every deserialization]** → Mitigation: validators are cheap (short grammar check / non-zero check); correctness outweighs micro-perf at the untrusted boundary.
- **[`pub(crate)` hatch — no cross-crate trusted construction]** → Mitigation: no current type needs it; the defining crate's internal fast path is preserved. A future need is hand-written, which is a deliberate reviewable act.
- **[Bytes decode failure now a `D::Error`, no longer impossible-by-contract]** → Mitigation: this is strictly safer than the prior `expect`; the only behavioral change is that malformed hex/base64url now fails deserialization instead of panicking under a violated contract.

## Migration Plan

1. Update `crates/derive/src/attr.rs`: drop `parse`/`err`/`hatch` parsing; add `validate_fn`/`validate_err` (required together); remove hatch plumbing.
2. Update macro expansion in `crates/derive/src/{str,bytes,num}.rs` to emit the conditional contract: when `validate_fn` is present, emit `try_new` + `TryFrom<Inner>` + `pub(crate) new_unchecked` + validating `Deserialize` (bytes: decode-then-validate), drop `new`/`From<Inner>`/`From<&…>`; for string additionally emit `parse`/`FromStr`; drop `parse`/`FromStr` for numeric and bytes. When `validate_fn` is absent, leave `new`/`From<Inner>`/transparent `Deserialize` unchanged.
3. Update dogfood types: rewrite `validate_did_method`/`validate_url` as `fn(&str) -> Result<(), _>` (idiomatic borrowed form; `&String` would trip `clippy::ptr_arg`) and `validate_version` as `fn(&u8) -> Result<(), _>` (drop the internal `s.parse::<u8>()`); migrate attributes `parse = …, err = …` → `validate_fn = …, validate_err = …`; replace `Type::new(x)` / `From`-based construction in tests/docs with `try_new(x)?`/`try_from(x)?`/`parse(&x)?` (string) / `new_unchecked(x)` (deliberately invalid); delete/rewrite `version_infallible_new_does_not_validate` to assert `Version::new` no longer exists, `try_new(0)` returns `Err`, and `new_unchecked(0)` constructs.
4. `cargo test` + `cargo clippy` + `cargo fmt`; `nix flake check` for full validation.

**Rollback:** revert the macro + dogfood changes; the old infallible constructors, the `&str` validator signature, and the `parse`/`err` attribute return. No data/format migration involved.

## Open Questions

None — all decisions were resolved: goal = D (full invariant); validator signature = callable as `<path>(&inner)` (`&Inner` or any `&Inner` derefs to), attribute = `validate_fn`/`validate_err`; `try_new` = uniform across categories; `TryFrom<Inner>` = yes, no `TryFrom<&…>`; `parse`/`FromStr` = string-only; hatch = `new_unchecked` fixed `pub(crate)`, no attribute; bytes serde = decode-then-validate with real decode error, `expect` contract deleted; decode error type = `&'static str` (richer out of scope); `ErrorCode`/`CapabilityId` = unaffected; migration = trivial now (in-crate only).