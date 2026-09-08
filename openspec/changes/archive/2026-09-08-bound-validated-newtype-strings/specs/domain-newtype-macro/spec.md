## MODIFIED Requirements

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
