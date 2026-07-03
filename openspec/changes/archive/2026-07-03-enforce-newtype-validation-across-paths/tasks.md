## 1. Attribute parsing (`crates/derive/src/attr.rs`)

- [x] 1.1 Drop parsing of `parse`/`err`/`hatch` helper attributes. Remove all `hatch`-related fields/plumbing.
- [x] 1.2 Add `validate_fn: Option<syn::Path>` and `validate_err: Option<syn::Path>` fields to the attribute struct.
- [x] 1.3 Parse `#[newtype(validate_fn = <path>, validate_err = <type>)]`. Enforce that `validate_fn` and `validate_err` are specified together — emit a compile error if exactly one is present.
- [x] 1.4 Expose `validate_fn`/`validate_err` to the category expanders (replacing the old `parse`/`err` access). Remove the hatch visibility selector entirely (the expanders will hardcode `pub(crate)`).

## 2. Macro expansion — string category (`crates/derive/src/str.rs`)

- [x] 2.1 When `validate_fn` is configured, stop emitting the infallible `new(inner) -> Self` constructor; emit `new_unchecked(inner) -> Self` instead with fixed `pub(crate)` visibility (no attribute-driven widening).
- [x] 2.2 When `validate_fn` is configured, stop emitting `From<String>` and `From<&str>`; emit `TryFrom<String>` instead (calls `validate_fn(&s)`, moves the owned `String` into `Self` on success, no realloc). Do NOT emit `TryFrom<&str>`.
- [x] 2.3 When `validate_fn` is configured, emit a uniform inherent `try_new(inner: String) -> Result<Self, #validate_err>` that validates `&inner` and constructs on success; `try_new` SHALL delegate to `TryFrom<String>::try_from` (or vice versa).
- [x] 2.4 When `validate_fn` is configured, change the generated `Deserialize` to call `validate_fn(&inner)` after deserializing the `String` and return `Err(D::Error::custom(<validate_err>))` on validation failure; keep the transparent `Self(inner)` `Deserialize` when `validate_fn` is absent.
- [x] 2.5 When `validate_fn` is configured, keep emitting `parse(s: &str)` / `FromStr`: convert to owned `String`, call `validate_fn(&owned)`, construct `Self(owned)` on success. (String is the only category that retains `parse`/`FromStr`.)
- [x] 2.6 When `validate_fn` is NOT configured, leave `new`, `From<String>`, `From<&str>`, and transparent `Deserialize` unchanged.

## 3. Macro expansion — bytes category (`crates/derive/src/bytes.rs`)

- [x] 3.1 When `validate_fn` is configured, stop emitting `new`; emit `new_unchecked(inner: Vec<u8>) -> Self` with fixed `pub(crate)` visibility.
- [x] 3.2 When `validate_fn` is configured, stop emitting `From<Vec<u8>>` and `From<&[u8]>`; emit `TryFrom<Vec<u8>>` instead (calls `validate_fn(&v)`, moves on success). Do NOT emit `TryFrom<&[u8]>`.
- [x] 3.3 When `validate_fn` is configured, emit a uniform inherent `try_new(inner: Vec<u8>) -> Result<Self, #validate_err>` that validates `&inner`; delegate to `TryFrom<Vec<u8>>::try_from` (or vice versa).
- [x] 3.4 When `validate_fn` is configured, make the generated `Deserialize` (the hex/base64url string path) **decode then validate**: decode via `nt_decode_` → on `Err` return `Err(D::Error::custom(<decode-err>))` (no `expect`, no panic) → on `Ok(bytes)` call `validate_fn(&bytes)` → on `Err` return `Err(D::Error::custom(<validate_err>))` → on `Ok` construct `Self(bytes)`. Delete the old `expect("validation function must guarantee the string decodes in the display encoding")` contract from both `Deserialize` and `parse_impl` (the latter is being removed anyway — see 3.6).
- [x] 3.5 When `validate_fn` is configured, SHALL NOT emit `parse` or `FromStr` (bytes drops the string-shaped entry entirely).
- [x] 3.6 When `validate_fn` is NOT configured, leave `new`, `From<Vec<u8>>`, `From<&[u8]>`, and transparent `Deserialize` unchanged (this is the `Multihash` path — no validator exists). Remove the now-unused `parse_impl` for bytes if no remaining code path references it.

## 4. Macro expansion — numeric category (`crates/derive/src/num.rs`)

- [x] 4.1 When `validate_fn` is configured, stop emitting the infallible `new(inner) -> Self` and `From<Inner>`; emit `new_unchecked(inner: Inner) -> Self` with fixed `pub(crate)` visibility.
- [x] 4.2 When `validate_fn` is configured, emit an inherent `try_new(inner: Inner) -> Result<Self, #validate_err>` that calls `validate_fn(&inner)` directly (no string round-trip) and constructs `Self(inner)` only on success.
- [x] 4.3 When `validate_fn` is configured, emit `TryFrom<Inner>` whose `try_from` calls `validate_fn(&inner)` and is equivalent to `try_new` (one delegates to the other).
- [x] 4.4 When `validate_fn` is configured, make the generated `Deserialize` validate: deserialize the number directly, then call `validate_fn(&inner)`, return `Err(D::Error::custom(<validate_err>))` on failure; keep transparent `Deserialize` when `validate_fn` is absent.
- [x] 4.5 When `validate_fn` is configured, SHALL NOT emit `parse` or `FromStr` (numeric drops the string-shaped entry entirely). Remove the now-unused numeric `parse`/`FromStr` expansion.
- [x] 4.6 When `validate_fn` is NOT configured, leave `new`, `From<Inner>`, and transparent `Deserialize` unchanged.

## 5. Derive crate integration tests (`crates/derive/tests/expand.rs`)

- [x] 5.1 `Tag` (string, `validate_fn`): migrate the attribute from `#[newtype(display, serde, parse = validate_nonempty, err = EmptyError)]` to `#[newtype(display, serde, validate_fn = validate_nonempty, validate_err = EmptyError)]`.
- [x] 5.2 Leave `validate_nonempty` as `fn(s: &str) -> Result<(), EmptyError>` (idiomatic borrowed form; `&String` would trip `clippy::ptr_arg`). The macro invokes `validate_nonempty(&inner)` where `inner: String`, and `&String` deref-coerces to `&str`; the `s.is_empty()` body is unchanged.
- [x] 5.3 In `str_newtype_accessors_and_conversions`: replace `Tag::new("hello".to_owned())` with `Tag::try_new("hello".to_owned()).unwrap()`; replace `Tag::from("world".to_owned())` with `Tag::try_from("world".to_owned()).unwrap()`; replace `Tag::from("bye")` with `Tag::parse("bye").unwrap()` (`From<String>`/`From<&str>` are removed for `validate_fn` string types; `parse`/`FromStr` are retained).
- [x] 5.4 Leave `str_newtype_parse_and_fromstr` unchanged — `parse`/`FromStr` are retained for the string category.
- [x] 5.5 In `str_newtype_serde_roundtrip`: replace `Tag::new("hello".to_owned())` with `Tag::try_new("hello".to_owned()).unwrap()`; keep the roundtrip (`"hello"` is valid, so validating `Deserialize` still accepts it). Add an assertion that `serde_json::from_str::<Tag>("\"\"").is_err()` (empty string fails `validate_nonempty`), exercising the new validating `Deserialize` for the string category.
- [x] 5.6 `BoundedPort` (numeric, `validate_fn`): migrate the attribute from `#[newtype(display, serde, parse = validate_bounded_port, err = PortError)]` to `#[newtype(display, serde, validate_fn = validate_bounded_port, validate_err = PortError)]`.
- [x] 5.7 Rewrite `validate_bounded_port` from `fn(s: &str) -> Result<(), PortError>` (which internally `s.parse::<u16>()`) to `fn(n: &u16) -> Result<(), PortError>` with body `if *n > 0 { Ok(()) } else { Err(PortError) }` — drop the internal string parse; the validator now reasons about the inner type directly.
- [x] 5.8 Rewrite `num_newtype_parse_validates` (numeric drops `parse`/`FromStr`): replace `BoundedPort::parse("8080")` and `"443".parse::<BoundedPort>()` with `BoundedPort::try_new(8080).unwrap()` and `BoundedPort::try_from(443).unwrap()` (asserting `.get()` equals the input); assert `BoundedPort::try_new(0).is_err()`; drop the `from_str("notanumber")` case (no string entry exists for numeric). Optionally assert `BoundedPort::new_unchecked(0).get() == 0` to exercise the `pub(crate)` hatch (callable since the test is in the defining crate).
- [x] 5.9 Add a `BoundedPort` serde-validates test: `serde_json::from_str::<BoundedPort>("8080")` succeeds (`.get() == 8080`) and `serde_json::from_str::<BoundedPort>("0").is_err()` (numeric `Deserialize` now validates), exercising the new validating `Deserialize` for the numeric category.
- [x] 5.10 Update the module doc comment: "fallible `parse` for the string, bytes, and numeric categories" → "fallible `parse` for the string category". Leave `Hash`, `B64`, and `Port` and their tests unchanged (no `validate_fn`; `new`/`From`/transparent serde retained per 2.6/3.6/4.6). Keep `use std::str::FromStr;` (the string category still uses it).

## 6. Dogfood types — `DidMethod` (`crates/did/src/method.rs`)

- [x] 6.1 Leave `validate_did_method` signature as `fn(s: &str) -> Result<(), Error>` (idiomatic borrowed form; `&String` would trip `clippy::ptr_arg`). The macro invokes `validate_did_method(&inner)` where `inner: String`, and `&String` deref-coerces to `&str`; the body (`s.is_empty()` / `s.chars()`) is unchanged.
- [x] 6.2 Migrate the attribute from `#[newtype(parse = validate_did_method, err = Error)]` to `#[newtype(validate_fn = validate_did_method, validate_err = Error)]`.
- [x] 6.3 Update doc comments: describe `parse`/`FromStr`/`TryFrom<String>`/`try_new` as the validated construction paths and `new_unchecked` as the `pub(crate)` trusted hatch (note there is no `pub` widening).
- [x] 6.4 Update tests: replace `DidMethod::new(...)` and `From`-based construction with `DidMethod::parse(...)?`/`DidMethod::try_from(...)?`/`DidMethod::try_new(...)?` (or `new_unchecked` where a deliberately-invalid value is needed, e.g. the "UPPER NOT VALID" assertion); remove any reliance on the removed infallible constructors.

## 7. Dogfood types — `Url` (`crates/core/src/url.rs`)

- [x] 7.1 Leave `validate_url` signature as `fn(s: &str) -> Result<(), UrlError>` (idiomatic borrowed form; `&String` would trip `clippy::ptr_arg`). The macro invokes `validate_url(&inner)` where `inner: String`, deref-coercing `&String → &str`.
- [x] 7.2 Migrate the attribute from `#[newtype(parse = validate_url, err = UrlError)]` to `#[newtype(validate_fn = validate_url, validate_err = UrlError)]`.
- [x] 7.3 Update doc comments for the validated construction paths and `new_unchecked` hatch.
- [x] 7.4 Update tests: replace `Url::new(...)`/`From` construction with `Url::parse(...)?`/`Url::try_from(...)?`/`Url::try_new(...)?` (or `new_unchecked` for the deliberately-invalid "not a url" case).

## 8. Dogfood types — `Version` (`crates/did/src/version.rs`)

- [x] 8.1 Rewrite `validate_version` signature from `fn(s: &str) -> Result<(), Error>` to `fn(n: &u8) -> Result<(), Error>`. Drop the internal `s.parse::<u8>()`; the body becomes a direct `if *n == 0 { Err(...) } else { Ok(()) }`.
- [x] 8.2 Migrate the attribute from `#[newtype(parse = validate_version, err = Error)]` to `#[newtype(validate_fn = validate_version, validate_err = Error)]`.
- [x] 8.3 Update doc comments: `try_new(u8)` / `TryFrom<u8>` are the validated number-in paths; `new_unchecked` the trusted hatch. Note that `parse`/`FromStr` are no longer available for `Version`.
- [x] 8.4 Delete the `version_infallible_new_does_not_validate` test (it asserted `Version::new(0)` succeeds — now a bug this change fixes) and replace with tests asserting: `Version::new` does not exist (compile-fail / not callable), `Version::try_new(0)` returns `Err(InvalidVersion)`, `Version::try_new(1)` returns `Ok`, and `Version::new_unchecked(0)` constructs without validating.
- [x] 8.5 Update remaining `Version::new(...)`/`From<u8>` test sites to `Version::try_new(...)?`/`Version::try_from(...)?` (or `new_unchecked` for known-good literals where a fast path is wanted). Replace any `Version::from_str`/`Version::parse` test sites with `try_new`/`try_from` since `FromStr` is dropped for numeric.

## 9. Dogfood types — `Multihash` (`crates/did/src/multihash.rs`)

- [x] 9.1 Confirm `Multihash` (bytes, no `validate_fn`) is unchanged — `new`, `From<Vec<u8>>`, `From<&[u8]>`, transparent `Deserialize` all remain. No `try_new`/`TryFrom`/`new_unchecked`/`parse`/`FromStr` are generated. No edits expected beyond verifying tests still pass; update docs only if they imply validation that doesn't exist.

## 10. Validation

- [x] 10.1 `cargo fmt`
- [x] 10.2 `cargo clippy` (fix any new warnings)
- [x] 10.3 `cargo test` (all derive + dogfood tests pass)
- [x] 10.4 `nix flake check` (full lint + nextest + deny + audit)