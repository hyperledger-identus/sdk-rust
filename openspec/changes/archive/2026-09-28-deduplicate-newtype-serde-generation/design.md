# Design

## Ownership boundary

A new crate-private module owns only the serde token templates shared by the
string and numeric categories. Its input is the already parsed derive context;
it returns a `TokenStream2`. The category modules decide whether serde is
requested and append the helper output. Bytes remain separate because their
wire representation performs explicit hex/base64url encoding and decoding.

## Generated behavior

The helper always emits the existing transparent `Serialize` implementation.
For `Deserialize`, it emits one of two existing shapes:

- deserialize `Inner` and construct `Self(inner)` when no validator exists;
- deserialize `Inner`, call the configured validator on `&inner`, map its
  error through `serde::de::Error::custom`, then construct on success.

The helper does not consume `validate_err`: deserialization already maps the
validator's concrete error into the serde error type. Attribute parsing still
requires `validate_fn` and `validate_err` together.

## Compatibility and cohesion

String parsing, `FromStr`, borrowed validation-before-allocation, numeric
copy/value access, constructors, and conversions stay category-owned. The
shared module has one reason to change: scalar serde template evolution. It is
not exported and does not create a runtime dependency or public extension
point.

## Verification

Existing runtime expansion tests prove exact string/number wire forms and
validated rejection. A positive trybuild case compiles all four scalar
combinations. Generated-output safety, strict Clippy, full tests, code-health,
factory, Nix, and protected CI prove the refactor remains behavior-neutral.
The baseline duplicate disposition is removed only after the generated copies
are gone and the refreshed report passes its canonical contract.
