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

- **string** (`String`): `as_str(&self) -> &str`, `AsRef<str>`, and (when `display` is requested) `Display` rendering the inner string. The conversion traits depend on `validate_fn`: when `validate_fn` is **not** configured, `From<String>` and `From<&str>` SHALL be generated (infallible, no validation); when `validate_fn` **is** configured, `From<String>` and `From<&str>` SHALL NOT be generated, and `TryFrom<String>` SHALL be generated instead (fallible, validating — see the "Generated constructors" requirement). `TryFrom<&str>` SHALL NOT be generated (the borrowed validated path is covered by `FromStr`, which string retains). The string category additionally retains `parse(s: &str) -> Result<Self, Err>` / `FromStr` when `validate_fn` is configured (see the "Fallible `parse`" requirement).
- **bytes** (`Vec<u8>`): `as_bytes(&self) -> &[u8]`, `into_bytes(self) -> Vec<u8>`, `AsRef<[u8]>`, and (when `display` is requested) `Display` rendering the bytes as hex (default) or base64url. The conversion traits depend on `validate_fn`: when `validate_fn` is **not** configured, `From<Vec<u8>>` and `From<&[u8]>` SHALL be generated (infallible, no validation); when `validate_fn` **is** configured, `From<Vec<u8>>` and `From<&[u8]>` SHALL NOT be generated, and `TryFrom<Vec<u8>>` SHALL be generated instead (fallible, validating). `TryFrom<&[u8]>` SHALL NOT be generated. The bytes category SHALL NOT generate `parse`/`FromStr` under any configuration.
- **numeric** (an integer or float primitive): `get(&self) -> Inner` (copying) and (when `display` is requested) `Display` rendering the number. The conversion traits depend on `validate_fn`: when `validate_fn` is **not** configured, `From<Inner>` SHALL be generated (infallible, no validation); when `validate_fn` **is** configured, `From<Inner>` SHALL NOT be generated, and `TryFrom<Inner>` SHALL be generated instead (fallible, validating). The numeric category SHALL NOT generate `parse`/`FromStr` under any configuration.

#### Scenario: String category generates string accessor and conversions without validate_fn

- **WHEN** `#[derive(Newtype)]` is applied to a `String`-backed struct with `display` and **without** `validate_fn`
- **THEN** `as_str()`, `AsRef<str>`, `From<String>`, `From<&str>`, and `Display` SHALL be available on the type

#### Scenario: String category generates TryFrom instead of From when validate_fn is configured

- **WHEN** `#[derive(Newtype)]` is applied to a `String`-backed struct with `display` and `validate_fn`
- **THEN** `as_str()`, `AsRef<str>`, `Display`, `FromStr`, `parse`, `TryFrom<String>`, `try_new`, and `new_unchecked` SHALL be available on the type
- **AND** `From<String>` and `From<&str>` SHALL NOT be available

#### Scenario: Bytes category generates bytes accessors and conversions without validate_fn

- **WHEN** `#[derive(Newtype)]` is applied to a `Vec<u8>`-backed struct with `display` and **without** `validate_fn`
- **THEN** `as_bytes()`, `into_bytes()`, `AsRef<[u8]>`, `From<Vec<u8>>`, `From<&[u8]>`, and `Display` (hex) SHALL be available on the type

#### Scenario: Bytes category generates TryFrom but no FromStr when validate_fn is configured

- **WHEN** `#[derive(Newtype)]` is applied to a `Vec<u8>`-backed struct with `display` and `validate_fn`
- **THEN** `as_bytes()`, `into_bytes()`, `AsRef<[u8]>`, `Display`, `TryFrom<Vec<u8>>`, `try_new`, and `new_unchecked` SHALL be available on the type
- **AND** `From<Vec<u8>>`, `From<&[u8]>`, `FromStr`, and `parse` SHALL NOT be available

#### Scenario: Numeric category generates numeric accessor and conversion without validate_fn

- **WHEN** `#[derive(Newtype)]` is applied to a `u16`-backed struct with `display` and **without** `validate_fn`
- **THEN** `new(inner) -> Self`, `get()`, `From<u16>`, and `Display` SHALL be available on the type

#### Scenario: Numeric category generates TryFrom and try_new but no FromStr when validate_fn is configured

- **WHEN** `#[derive(Newtype)]` is applied to a `u16`-backed struct with `display` and `validate_fn`
- **THEN** `get()`, `Display`, `TryFrom<u16>`, `try_new`, and `new_unchecked` SHALL be available on the type
- **AND** `new(inner) -> Self`, `From<u16>`, `FromStr`, and `parse` SHALL NOT be available

#### Scenario: Unrecognised inner type is rejected

- **WHEN** `#[derive(Newtype)]` is applied to a struct whose field type is not one of the supported string/bytes/numeric inner types
- **THEN** the derive SHALL emit a compile error listing the supported inner types

### Requirement: Opt-in `display` and `serde` via `#[newtype(...)]` attributes

The derive SHALL accept `#[newtype(...)]` helper attributes on the struct. `display` SHALL opt into `Display` generation. `serde` SHALL opt into `Serialize`/`Deserialize` implemented over the inner type: string- and numeric-backed types serialize as the underlying JSON string/number; bytes-backed types serialize as a JSON **string** in the same encoding as `Display` (hex by default, base64url when `display = "base64url"`), not as a JSON array of byte numbers and not as a wrapper object. Bytes-category `display` SHALL default to hex and SHALL accept `display = "base64url"` to select base64url encoding.

The `Deserialize` implementation's validation behavior depends on `validate_fn`: when `validate_fn` is **not** configured, `Deserialize` SHALL be transparent (deserialize the inner value and construct `Self(inner)` with no validation); when `validate_fn` **is** configured, `Deserialize` SHALL run the validation function (see the "Generated constructors" and "Fallible `parse`" requirements) and SHALL return `Err` on invalid input rather than constructing an invalid value. For the bytes category, `Deserialize` SHALL first decode the hex/base64url string and SHALL return `Err` on decode failure (rather than panicking), then validate the decoded bytes.

#### Scenario: `display` opts into Display

- **WHEN** a `String`-backed struct is derived with `#[newtype(display)]`
- **THEN** `Display` SHALL be implemented and render the inner string

#### Scenario: `serde` opts into transparent serde without validate_fn

- **WHEN** a `String`-backed struct is derived with `#[newtype(serde)]` and **without** `validate_fn`
- **THEN** `Serialize` and `Deserialize` SHALL be implemented such that the value serializes as a plain JSON string and round-trips, and `Deserialize` SHALL NOT validate

#### Scenario: `serde` validates on deserialization when validate_fn is configured (string/numeric)

- **WHEN** a `String`-backed struct is derived with `#[newtype(serde, validate_fn = <path>, validate_err = <type>)]`
- **AND** a value is deserialized whose inner value fails the validation function
- **THEN** `Deserialize` SHALL return `Err` and SHALL NOT construct an instance of the type
- **AND WHEN** a value is deserialized whose inner value passes the validation function
- **THEN** `Deserialize` SHALL construct `Self(inner)`

#### Scenario: `serde` decodes then validates on deserialization when validate_fn is configured (bytes)

- **WHEN** a `Vec<u8>`-backed struct is derived with `#[newtype(serde, validate_fn = <path>, validate_err = <type>)]`
- **AND** a value is deserialized whose wire string cannot be decoded in the display encoding
- **THEN** `Deserialize` SHALL return `Err` (mapping the decode error into `D::Error`) and SHALL NOT construct an instance, SHALL NOT panic, and SHALL NOT call the validation function
- **AND WHEN** a value is deserialized whose wire string decodes but the decoded bytes fail the validation function
- **THEN** `Deserialize` SHALL return `Err` (mapping `validate_err` into `D::Error`) and SHALL NOT construct an instance
- **AND WHEN** a value is deserialized whose wire string decodes and the decoded bytes pass the validation function
- **THEN** `Deserialize` SHALL construct `Self(bytes)`

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

### Requirement: Fallible `parse` via caller-supplied validation function and error type (string category only)

The derive SHALL accept `#[newtype(validate_fn = <path>, validate_err = <type>)]`. `validate_fn` names a function the macro invokes as `<path>(&inner)` where `inner: Inner` (the validation function, operating on the inner type by reference). For the string category, the function's first parameter SHALL accept `&str`; for bytes and numeric categories, it SHALL be `&Inner` or any type `&Inner` derefs to (e.g. `&[u8]` for `Vec<u8>` or `&u8` for numeric). `validate_fn` and `validate_err` SHALL be specified together, and `validate_err` SHALL match `validate_fn`'s return type. The derive SHALL NOT generate the error type itself; the owning crate SHALL define `<type>` and its `to_identus_error()` mapping by hand.

For the **string category only**, when `validate_fn` is configured the derive SHALL implement `FromStr` with `Err = <type>` and an inherent `parse(s: &str) -> Result<Self, <type>>`. Both SHALL call `<path>(s)` before allocating, return `Err(<type>)` on validation failure without constructing an owned copy, and construct `Self(s.to_owned())` on success. The numeric and bytes categories SHALL NOT generate `parse` or `FromStr` under any configuration. Owned `TryFrom<String>` and validating serde SHALL continue to invoke the validator against their already-owned inner value.

#### Scenario: `parse` validates borrowed input before allocation (string category)

- **WHEN** `#[derive(Newtype)]` with `#[newtype(validate_fn = validate_did_method, validate_err = did::Error)]` is applied to a `String`-backed struct
- **THEN** inherent `parse` and `<Type as FromStr>::from_str` SHALL call `validate_did_method(s)` before constructing an owned `String`
- **AND** validation failure SHALL return `did::Error` without cloning the rejected input
- **AND** validation success SHALL construct `Self(s.to_owned())`

#### Scenario: Validation function is invoked with the category-appropriate reference

- **WHEN** `validate_fn = <path>` is configured for a validated `String` newtype
- **THEN** its validator SHALL accept `&str` so borrowed parsing can validate before allocation and owned paths can use deref coercion
- **AND WHEN** `validate_fn = <path>` is configured for a bytes or numeric newtype
- **THEN** the validation function SHALL be invoked as `<path>(&inner)` where `inner: Inner`
- **AND** the validator SHALL NOT be invoked with a by-value `Inner` or with extra parameters

#### Scenario: Numeric and bytes categories do not generate parse or FromStr

- **WHEN** `#[derive(Newtype)]` with `#[newtype(validate_fn = <path>, validate_err = <type>)]` is applied to a numeric- or `Vec<u8>`-backed struct
- **THEN** neither `parse` nor `FromStr` SHALL be available on the type

#### Scenario: validate_fn and validate_err are required together

- **WHEN** a struct is derived with `validate_fn` but without `validate_err`, or with `validate_err` but without `validate_fn`
- **THEN** the derive SHALL emit a compile error

#### Scenario: The error type and its core bridging are owned by the caller

- **WHEN** the derived type's `FromStr::Err` (string category) or `TryFrom::Error` is inspected
- **THEN** it SHALL be the caller-supplied `<type>`, and the derive SHALL NOT emit a `to_identus_error()` impl for it

### Requirement: Generated constructors

The constructor surface the derive SHALL generate depends on whether `validate_fn` is configured.

When `validate_fn` is **not** configured (no validator exists), the derive SHALL generate an infallible `new(inner) -> Self` constructor (taking ownership for `String`/`Vec<u8>`, by value for numeric) that performs no validation, together with the infallible `From<Inner>` conversions (and `From<&str>`/`From<&[u8]>` for string/bytes). Forbidding infallible construction when no validator exists would make the type unconstructable, so the infallible constructors SHALL remain.

When `validate_fn` **is** configured, the derive SHALL NOT generate the infallible `new` or the infallible `From<Inner>`/`From<&str>`/`From<&[u8]>` conversions. Instead it SHALL generate:

- A uniform inherent `try_new(inner: Inner) -> Result<Self, <validate_err>>` for **every** category, that calls the validation function on `&inner` and constructs `Self(inner)` only when validation passes. `try_new` and `TryFrom<Inner>::try_from` SHALL be equivalent (one SHALL delegate to the other).
- A `TryFrom<Inner>` that validates `&inner` and constructs `Self(inner)` on success. `TryFrom<Inner>` SHALL take ownership of the inner value (move semantics); on validation success it SHALL move the inner value into `Self` without reallocating. `TryFrom<&str>`/`TryFrom<&[u8]>` SHALL NOT be generated.
- A `new_unchecked(inner) -> Self` escape hatch that constructs `Self(inner)` **without** validation. `new_unchecked` SHALL have fixed `pub(crate)` visibility; there SHALL be no attribute to widen it to `pub`. No `From`-shaped unchecked conversion SHALL be generated; `new_unchecked` is the sole bypass.
- For the **string category only**, the fallible `parse(s: &str) -> Result<Self, <validate_err>>` and `FromStr` constructors (as in the "Fallible `parse`" requirement). Numeric and bytes SHALL NOT generate `parse` or `FromStr`.
- A `Deserialize` that runs the validation function and returns `Err` on invalid input (see the "Opt-in `display` and `serde`" requirement). For bytes, `Deserialize` SHALL decode the wire string first and return `Err` on decode failure before validating.

#### Scenario: Infallible `new` is available only when validate_fn is not configured

- **WHEN** a `String`-backed struct is derived **without** `validate_fn`
- **THEN** `Type::new(s: String) -> Self` SHALL be available and SHALL NOT validate
- **AND** `From<String>` and `From<&str>` SHALL be available

#### Scenario: Infallible `new` and From are removed when validate_fn is configured

- **WHEN** a `String`-backed struct is derived **with** `validate_fn`
- **THEN** `Type::new` SHALL NOT be available
- **AND** `From<String>` and `From<&str>` SHALL NOT be available
- **AND** `Type::parse(s)` / `FromStr` (string only), `TryFrom<String>` (fallible, validated, move), `Type::try_new(inner)`, and `Type::new_unchecked(inner)` SHALL be available

#### Scenario: try_new is generated for every category when validate_fn is configured

- **WHEN** a struct of any category (string, bytes, numeric) is derived **with** `validate_fn`
- **THEN** `Type::try_new(inner: Inner) -> Result<Self, <validate_err>>` SHALL be available
- **AND** `try_new` SHALL delegate to `TryFrom<Inner>::try_from` (or vice versa)

#### Scenario: `new_unchecked` skips validation with fixed pub(crate) visibility

- **WHEN** `Type::new_unchecked(inner)` is called
- **THEN** it SHALL construct `Self(inner)` without calling the validation function
- **AND** it SHALL be callable from within the defining crate
- **AND** it SHALL NOT be callable from outside the defining crate
- **AND** no `#[newtype(...)]` attribute SHALL exist that widens its visibility

#### Scenario: `TryFrom<Inner>` validates and moves without reallocating

- **WHEN** a `String`-backed struct is derived **with** `validate_fn` and `TryFrom<String>::try_from(s)` is called with an owned `String` whose content passes validation
- **THEN** the validation function SHALL be called on `&s`
- **AND** on success `Self` SHALL be constructed by moving the owned `String` (no reallocation of the inner string)

#### Scenario: `TryFrom<Inner>` errors on invalid input

- **WHEN** `TryFrom<Inner>::try_from(inner)` is called with an inner value whose content fails the validation function
- **THEN** it SHALL return `Err(<validate_err>)` and SHALL NOT construct an instance

#### Scenario: Validation failure does not construct the value

- **WHEN** the validation function returns `Err` (via `try_new`, `TryFrom<Inner>`, `parse`/`FromStr`, or `Deserialize`)
- **THEN** no instance of the newtype SHALL be constructed
