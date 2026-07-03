# newtype-validation-construction-contract

> **Status: Exploring** — pre-change exploration, not tracked by `openspec` as
> a change or spec, and not apply-able. When ready, promote to
> `openspec/changes/` via `openspec new change`. Picks up a thread left open by
> the completed `add-domain-newtype-macro` change; would **amend** that
> change's shipped spec rather than stand alone.

## Why

`#[derive(Newtype)]` (shipped by `add-domain-newtype-macro`, status complete
35/35) generates an infallible `new(inner) -> Self` and `From<Inner>`/`From<&…>`
conversions that perform **no validation**, alongside the fallible
`parse`/`FromStr` generated only when `#[newtype(parse = …, err = …)]` is
configured. The spec explicitly codifies this: *"Infallible `new` is always
available … SHALL NOT validate"* and *"`new` and `parse` coexist."*

For string-category types with a real grammar — `DidMethod` (W3C `method-name`),
`Url` — this means an invalid value is trivially constructible:

```rust
let m = DidMethod::new("UPPER NOT VALID".to_owned()); // compiles, invalid
```

The instinct to fix this is sound: if a type has a grammar, *every* value of
that type should satisfy it ("make invalid states unrepresentable"). But
forbidding `new` alone does **not** achieve that goal — it closes one of three
bypasses. The generated `Deserialize` (in `crates/derive/src/str.rs` and
`bytes.rs`) does `Self(inner)` directly and runs **no validation**, so the
untrusted deserialization boundary — network, files, configs — is a wider hole:

```rust
let bad: DidMethod = serde_json::from_str("\"BAD\"").unwrap(); // succeeds, invalid
```

`new` is at least an explicit, greppable call site; the serde path is the more
dangerous one because it's where untrusted data enters the system. So the real
question is not "forbid `new`" but *"what is the validation contract across
every construction path?"*

## What (sketch)

A change that, when `parse` is configured, revises the generated surface so the
type's invariant holds across all construction paths. Shape options (not all
required; to be narrowed during the change):

- **Drop the quiet infallible bypasses** — `From<String>`/`From<&str>` (string),
  `From<Vec<u8>>`/`From<&[u8]>` (bytes). These fire implicitly via `into()`/`?`/
  coercion and are the insidious ones.
- **Replace bare `new` with a named escape hatch** — e.g. `new_unchecked` /
  `new_assume_valid`. The name is the safety feature: reviewers grep for the
  trust assumption. Trusted internal construction keeps a fast path; external
  callers are steered to `parse`.
- **Make generated `Deserialize` run the validator** — when `parse` is
  configured, `Deserialize` calls the validation function and returns
  `Err(<type>)` on invalid input. This is what actually closes the untrusted
  boundary; without it, forbidding `new` is cosmetic.
- **Category-scoped rule** — apply the restriction to string and
  bytes-with-`parse` categories only; numeric (e.g. `Version(u8)`,
  `parse = validate_version`, non-zero) keeps `new`, because bounds ≠ grammar
  and `Version::new(2)` is cheap/meaningful where `parse("2")` would force a
  string allocation to re-validate a known-good literal.
- **No public infallible constructor at all** is a stricter variant (force
  `pub(crate)` on any hatch) — considered but likely too restrictive for trusted
  downstream construction.

Compared options (from the exploration session):

```
   Option  Constructor          From<…>?      Serde validates?  Invariant holds?
   ──────  ───────────          ────────      ────────────────  ─────────────────
   A hard  parse only           removed       no (gap)          NO
   B hatch  new_unchecked +     removed       no (gap)          NO
            parse
   C TryFrom parse +            TryFrom<…>    no (gap)          NO
            TryFrom<…>          (fallible)
   D full   parse +             removed       YES               YES
            new_unchecked       (quiet ones)
```

Only **D** delivers "no `DidMethod` holds an invalid string" across every path.
B is the cheaper version that stops accidental bypass but leaves the serde
hole; A/C leave the serde hole too. The choice hinges on whether the goal is
"make invalid states unrepresentable" (→ D) or merely "stop accidental bypass"
(→ B).

## Open Questions (to resolve before promoting to a change)

- **Goal framing — invariant vs accident-prevention**: is the aim that *no
  instance* can ever hold an invalid value (→ Option D, serde must validate),
  or only that the quiet/implicit bypasses are removed (→ Option B, serde stays
  transparent)? This is the deciding question.
- **Serde scope**: is making `Deserialize` validate in-scope for this change, or
  deferred to a follow-on? If deferred, the change must explicitly state
  "invalid states remain representable via serde" so it doesn't overclaim the
  invariant.
- **Category scope**: forbid `new` for `Version` (numeric) too, or restrict the
  rule to string + bytes-with-`parse`? Default leaning: numeric keeps `new`
  (bounds ≠ grammar; cheap trusted literals).
- **Escape-hatch name and visibility**: `new_unchecked` vs `new_assume_valid`
  vs `from_assume_valid`; `pub` vs `pub(crate)`. The stricter `pub(crate)`
  variant forces all external construction through `parse`.
- **`TryFrom<String>` in addition to / instead of `From`**: keep a fallible
  owned-conversion trait for ergonomics, or only `parse`/`FromStr`?
- **Migration**: today `DidMethod::new`/`Url::new`/`Version::new` and the `From`
  impls are used **only in tests and doc examples** (verified by repo-wide grep),
  and the dogfood types are the only consumers — so the migration is trivial
  *now*. Once these types get adopted across `cloud-agent` adapters / bindings,
  the infallible constructors become a breaking change to remove. This is the
  moment to decide; that window is why this exploration is time-sensitive.

## Notes

- This **amends** `add-domain-newtype-macro`'s shipped spec — specifically the
  "Generated constructors" requirement ("Infallible `new` is always available …
  SHALL NOT validate" and the "`new` and `parse` coexist" scenario). The change
  is not additive; it revises an existing contract. The proposal should name
  that explicitly and update `crates/derive/src/{str,bytes,num}.rs` expansion +
  the dogfood types' docs/tests (`crates/did/src/{method,version,multihash}.rs`,
  `crates/core/src/url.rs`) accordingly.
- `Multihash` currently has **no** `parse` configured (validated multihash
  structure is a deferred follow-on per the existing design), so the rule is
  conditional on `parse` being present — forbidding infallible construction when
  no validator exists would make the type unconstructable. Any change must keep
  `new`/`From` when `parse` is absent.
- Grounding evidence: generated `Deserialize` bypass confirmed in
  `crates/derive/src/str.rs` and `bytes.rs` (both do `Self(inner)`); bypass-only
  usage confirmed by grep (`DidMethod::new`/`Url::new`/`Version::new` and `From`
  impls appear solely in tests/docs, no production call sites as of this
  exploration).