# ADR 0129: bound public crypto text encoders

- **Status:** Accepted for implementation
- **Date:** 2026-09-17
- **Decision authority:** issue #298 and the `harden-crypto-text-encoders` OpenSpec change
- **Related:** ADR 0125; `SDK-SEC-003`; `SDK-LIM-007`

## Context

`HexStr` and `Base64UrlStrNoPad` already reject encoded text above
`MAX_CRYPTO_TEXT_BYTES` (4,096 bytes), but their blanket
`From<B: AsRef<[u8]>>` implementations could encode and retain arbitrarily large
caller-owned byte slices. This was the final crypto-codec compatibility exception
in the repository input-resource inventory.

The crates remain unpublished at version `0.0.0`, and the inspected Oxid,
Lace ID Portal, Midnight Identity, and NeoPRISM worktrees do not directly call
these sdk-rust constructors. This is therefore the least costly point to remove
the source-level bypass without changing the canonical wire representation.

## Decision

1. Remove the blanket infallible `From<B: AsRef<[u8]>>` implementations.
2. Expose `try_from_bytes` plus `TryFrom` for byte slices, vectors, arrays, and
   borrowed arrays. Every public byte path rejects before the retained canonical
   text would exceed 4,096 bytes.
3. Derive raw-input ceilings from that text budget: 2,048 bytes for lowercase
   hex and 3,072 bytes for unpadded Base64url.
4. Keep canonical encoding unchanged and use the existing redacted
   `Error::KeyParsing` bridge on rejection.
5. Keep a crate-private trusted encoder only for already-bounded parser output
   and fixed-width 32-byte JWK coordinates. It is not a public escape hatch.
6. Narrow only the codec clause of `SDK-LIM-007`. Caller allocation before SDK
   entry and all other named compatibility exceptions remain disclosed.

## Consumer migration

Replace infallible construction with an explicit fallible boundary:

```rust
let hex = HexStr::try_from_bytes(bytes)?;
let base64url = Base64UrlStrNoPad::try_from(bytes)?;
```

Code handling a fixed byte array may use `TryFrom<[u8; N]>` or
`TryFrom<&[u8; N]>`. Parsing existing encoded text remains
`"...".parse::<HexStr>()` or `"...".parse::<Base64UrlStrNoPad>()`.

## Consequences

- Publicly reachable encoded values now share the parser's retained-text
  ceiling and fail before encoding an oversized input.
- Existing in-budget text, serialization, and JWK output remain byte-for-byte
  compatible.
- Source callers must handle `Result`; this is an intentional unpublished API
  migration rather than a silent compatibility promise.
- The boundary does not retroactively limit memory allocated by a caller,
  transport, FFI bridge, JavaScript engine, or deserializer.

## Rejected alternatives

- Retaining the blanket `From` would preserve the audited resource bypass.
- Panicking or truncating at the boundary would make oversized input unsafe or
  ambiguous.
- A public unchecked constructor would recreate the same bypass under another
  name.
- Raising the shared 4,096-byte budget is unnecessary for canonical key and
  digest encodings and would weaken the existing parser contract.

## Rollback

Reintroduce the blanket conversions, reverse callers and tests, and restore the
codec exception in the boundary inventory and `SDK-LIM-007` atomically.
