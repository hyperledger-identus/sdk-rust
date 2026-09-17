# `identus-core`

`identus-core` holds small chain-neutral values and capability contracts that
multiple SDK components need without depending on a protocol or product.

## It owns

- stable component metadata;
- capability identifiers and error codes;
- redaction-safe `IdentusError`, `ErrorKind`, and `IdentusResult` contracts;
- bounded generic URLs;
- wall and monotonic clock ports plus millisecond time/duration values.

The public error value stores static public fields. Its display form contains a
stable code and public message, not secrets or internal debugging context.

## It does not own

- a catch-all prelude or umbrella SDK facade;
- cryptographic algorithms or secret-bearing key types;
- DID/VC/protocol wire models;
- network clients, persistence, telemetry, trust, custody, or wallet policy;
- chain or runtime integration.

## Dependency role

`identus-core` uses `identus-derive` for shared type conventions and Serde for
the bounded values that require serialization. Higher crates depend inward on
core; core must never depend outward on those components.

## Example: public error boundary

```rust
use identus_core::{CapabilityId, ErrorCode, ErrorKind, IdentusError};

let error = IdentusError::public(
    ErrorCode::new("invalid_input"),
    ErrorKind::InvalidInput,
    CapabilityId::new("example"),
    "input is invalid",
);

assert_eq!(error.to_string(), "invalid_input: input is invalid");
```

[Source](https://github.com/hyperledger-identus/sdk-rust/tree/develop/crates/core)
