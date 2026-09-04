# ADR 0021: Use a fallible caller-owned entropy port

- Status: Accepted
- Date: 2026-09-04
- Issue: #67

## Context

The initial `SecureRandom` port returned `Vec<u8>` for an arbitrary requested
length and could not report failure. Production and key-generation code used
`expect` or `panic!`, making entropy failure unrecoverable and allowing the
port implementation to allocate based on caller input.

## Decision

`SecureRandom` fills caller-owned storage and returns the existing crypto
error:

```rust
fn fill_bytes(&mut self, output: &mut [u8]) -> Result<(), Error>;
```

Every random key, mnemonic, and seed constructor returns `Result`. Fixed-width
consumers allocate fixed 32-byte arrays. System-provider failure and bounded EC
scalar-exhaustion return `Error::SecureRandomFailure`, which bridges to the
stable redacted `crypto.secure_random_failure` code. Backend details and
candidate bytes are never included.

## Consequences

- Consumers must explicitly handle random-construction failure.
- Adapter implementations cannot choose an allocation proportional to a
  supplied length; they fill the caller's slice.
- Empty slices are valid and succeed.
- EC scalar rejection remains bounded at sixteen candidates and now ends in an
  error rather than panic.
- This is a source-breaking change to unpublished `0.0.0` APIs, with no wire,
  persistence, algorithm, dependency, target, or custody change.
