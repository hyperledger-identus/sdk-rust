# `identus-derive`

`identus-derive` is compile-time tooling for repetitive, security-sensitive
domain-type structure.

## It owns

- `#[derive(Newtype)]` for supported single-field string, byte, and numeric
  tuple structs;
- generated constructors, accessors, conversions, display behavior, optional
  Serde integration, and fallible parsing appropriate to each category;
- `#[identus::port]`, an inert capability marker that enforces the SDK's
  port-trait naming convention;
- validation of generated output against forbidden unsafe constructs before
  tokens leave the macro.

## It does not own

- business or protocol validation rules;
- a runtime object model;
- cryptography, serialization formats, storage, transport, or chains;
- arbitrary code generation for downstream convenience.

The consuming type declares its domain constraint. The macro standardizes the
safe shape; it does not decide what a valid DID, credential, key, or wallet
policy means.

## Why it is a separate crate

Rust procedural macros compile for the build host and require a dedicated
`proc-macro` package. Keeping that compiler surface separate also prevents
`syn`, `quote`, and `proc-macro2` from becoming runtime dependencies of SDK
consumers.

## Review focus

- Diagnostics and generated public APIs are compatibility surfaces.
- Generated code must inherit the repository's no-unsafe policy.
- Compile-fail fixtures are part of the contract, not incidental tests.
- Runtime crates should expose explicit types rather than re-exporting the
  macro crate as a broad facade.

[Source](https://github.com/hyperledger-identus/sdk-rust/tree/develop/crates/derive)
