## Purpose

The `identus-derive` crate (a foundation-layer `proc-macro = true` build-time crate) provides a single `#[derive(Newtype)]` proc-macro that packages the boilerplate for domain newtypes — the `DidMethod`, `DidSuffix`, and similar wrapper types used across the Identus Rust SDK. A domain newtype is a tuple struct with exactly one unnamed field; the derive inspects the inner field's type and generates category-appropriate constructors, accessors, conversions, and `Display`, with optional `serde` transparency and fallible `parse` wired to a caller-supplied validation function.

## Requirements

### Requirement: Single `#[derive(Newtype)]` packages the newtype boilerplate

The `identus-derive` crate (a foundation-layer `proc-macro = true` crate) SHALL expose a single `#[derive(Newtype)]` proc-macro derive applicable to a tuple struct with exactly one unnamed field. The derive SHALL inspect the field's type and dispatch to one of three categories — string, bytes, numeric — generating the category-appropriate constructors, accessors, conversions, and `Display`. The derive SHALL compose with standard `#[derive(...)]` (e.g. `Debug`, `Clone`, `PartialEq`, `Eq`, `Hash`, `Copy`) on the same struct and SHALL NOT itself emit those derives.

#### Scenario: Derive applies to a one-field tuple struct

- **WHEN** `#[derive(Newtype)]` is applied to `pub struct DidMethod(String)`
- **THEN** the derive SHALL compile and generate category-appropriate inherent methods and trait impls for `DidMethod`

#### Scenario: Derive rejects a struct with no fields or more than one field

- **WHEN** `#[derive(Newtype)]` is applied to a struct with zero fields or with two or more fields
- **THEN** the derive SHALL emit a compile error identifying the offending struct

#### Scenario: Derive rejects a named-field struct

- **WHEN** `#[derive(Newtype)]` is applied to a struct with a named field (e.g. `pub struct Foo { inner: String }`)
- **THEN** the derive SHALL emit a compile error stating that only a single unnamed field is supported

### Requirement: Three inner categories with category-appropriate trait impls

The derive SHALL recognise the inner field's type and select a category, implementing exactly the standard traits that make sense for that category:

- **string** (`String`): `as_str(&self) -> &str`, `AsRef<str>`, `From<String>`, `From<&str>`, and (when `display` is requested) `Display` rendering the inner string.
- **bytes** (`Vec<u8>`): `as_bytes(&self) -> &[u8]`, `into_bytes(self) -> Vec<u8>`, `AsRef<[u8]>`, `From<Vec<u8>>`, `From<&[u8]>`, and (when `display` is requested) `Display` rendering the bytes as hex (default) or base64url.
- **numeric** (an integer or float primitive): `new(inner) -> Self` (infallible), `get(&self) -> Inner` (copying), `From<Inner>`, and (when `display` is requested) `Display` rendering the number.

#### Scenario: String category generates string accessor and conversions

- **WHEN** `#[derive(Newtype)]` is applied to a `String`-backed struct with `display`
- **THEN** `as_str()`, `AsRef<str>`, `From<String>`, `From<&str>`, and `Display` SHALL be available on the type

#### Scenario: Bytes category generates bytes accessors and conversions

- **WHEN** `#[derive(Newtype)]` is applied to a `Vec<u8>`-backed struct with `display`
- **THEN** `as_bytes()`, `into_bytes()`, `AsRef<[u8]>`, `From<Vec<u8>>`, `From<&[u8]>`, and `Display` (hex) SHALL be available on the type

#### Scenario: Numeric category generates numeric accessor and conversion

- **WHEN** `#[derive(Newtype)]` is applied to a `u16`-backed struct with `display`
- **THEN** `get()`, `From<u16>`, and `Display` SHALL be available on the type

#### Scenario: Unrecognised inner type is rejected

- **WHEN** `#[derive(Newtype)]` is applied to a struct whose field type is not one of the supported string/bytes/numeric inner types
- **THEN** the derive SHALL emit a compile error listing the supported inner types

### Requirement: Opt-in `display` and `serde` via `#[newtype(...)]` attributes

The derive SHALL accept `#[newtype(...)]` helper attributes on the struct. `display` SHALL opt into `Display` generation. `serde` SHALL opt into `Serialize`/`Deserialize` implemented transparently over the inner type: string- and numeric-backed types serialize as the underlying JSON string/number; bytes-backed types serialize as a JSON **string** in the same encoding as `Display` (hex by default, base64url when `display = "base64url"`), not as a JSON array of byte numbers and not as a wrapper object. Bytes-category `display` SHALL default to hex and SHALL accept `display = "base64url"` to select base64url encoding.

#### Scenario: `display` opts into Display

- **WHEN** a `String`-backed struct is derived with `#[newtype(display)]`
- **THEN** `Display` SHALL be implemented and render the inner string

#### Scenario: `serde` opts into transparent serde

- **WHEN** a `String`-backed struct is derived with `#[newtype(serde)]`
- **THEN** `Serialize` and `Deserialize` SHALL be implemented such that the value serializes as a plain JSON string and round-trips

#### Scenario: Bytes-category serde emits the display encoding, not a byte array

- **WHEN** a `Vec<u8>`-backed struct is derived with `#[newtype(serde)]` (and no `display` override)
- **THEN** `Serialize` and `Deserialize` SHALL emit and accept a JSON **string** in hex (matching the default `Display` encoding), not a JSON array of byte numbers
- **AND WHEN** the same struct is derived with `#[newtype(display = "base64url", serde)]`
- **THEN** serde SHALL emit and accept a base64url (no padding) JSON string matching the `Display` encoding

#### Scenario: Bytes display defaults to hex and is selectable

- **WHEN** a `Vec<u8>`-backed struct is derived with `#[newtype(display)]`
- **THEN** `Display` SHALL render hex
- **AND WHEN** the same struct is derived with `#[newtype(display = "base64url")]`
- **THEN** `Display` SHALL render base64url (no padding)

### Requirement: Fallible `parse` via caller-supplied validation function and error type

The derive SHALL accept `#[newtype(parse = <path>, err = <type>)]`. When present, the derive SHALL implement `FromStr` with `Err = <type>` and an inherent `parse(s: &str) -> Result<Self, <type>>`, both delegating to `<path>(s) -> Result<(), <type>>` (the validation function) and constructing `Self` only when validation passes. The derive SHALL NOT generate the error type itself; the owning crate SHALL define `<type>` and its `to_identus_error()` mapping by hand.

#### Scenario: `parse` wires FromStr to a validation function

- **WHEN** `#[derive(Newtype)]` with `#[newtype(parse = did_method_validate, err = did::Error)]` is applied to a `String`-backed struct
- **THEN** `<Type as FromStr>::from_str` SHALL call `did_method_validate(s)`, return `Err(did::Error)` on validation failure, and construct `Self(s.to_owned())` on success

#### Scenario: Validation failure does not construct the value

- **WHEN** the validation function returns `Err`
- **THEN** no instance of the newtype SHALL be constructed

#### Scenario: The error type and its core bridging are owned by the caller

- **WHEN** the derived type's `FromStr::Err` is inspected
- **THEN** it SHALL be the caller-supplied `<type>`, and the derive SHALL NOT emit a `to_identus_error()` impl for it

### Requirement: Generated constructors

For all categories, the derive SHALL generate an infallible `new(inner) -> Self` constructor (taking ownership for `String`/`Vec<u8>`, by value for numeric) that performs no validation. When `parse` is configured, the fallible `parse`/`FromStr` constructors SHALL be generated in addition to `new`.

#### Scenario: Infallible `new` is always available

- **WHEN** a `String`-backed struct is derived without `parse`
- **THEN** `Type::new(s: String) -> Self` SHALL be available and SHALL NOT validate

#### Scenario: `new` and `parse` coexist

- **WHEN** a `String`-backed struct is derived with `parse`
- **THEN** both `Type::new(inner)` (infallible, no validation) and `Type::parse(s)` / `FromStr` (fallible, validated) SHALL be available