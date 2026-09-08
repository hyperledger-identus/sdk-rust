# Design

## Context

The two parser implementations have private backing strings and identical
decode-then-canonicalize shapes. A shared root constant and one early check per
`FromStr` path establish a coherent crate-level input budget without wrapping
or replacing the existing codec dependencies.

## Decisions

### One public encoded-text parser budget

`pub const MAX_CRYPTO_TEXT_BYTES: usize = 4_096` is exported from
`identus-crypto`. The value applies to UTF-8 byte length presented to
`HexStr::from_str` and `Base64UrlStrNoPad::from_str`. The formats are ASCII,
but byte wording makes the resource semantics explicit for malformed Unicode.

### Preserve the public error category

A private error value carries a static codec label, maximum and actual byte
counts and implements `Error`. Oversize rejection wraps it in the existing
`Error::KeyParsing` variant. This retains the public enum, source chain and
stable redacted core bridge while giving local diagnostics useful lengths
without input contents.

### Check resource limits before syntax

Each `FromStr` checks `s.len()` before invoking `hex::decode` or
`base64::Engine::decode`. A malformed 4,097-byte input therefore reports the
same length source as a syntactically valid oversized value. In-budget decode
and canonical re-encoding are unchanged.

### Keep trusted encoding infallible

`From<B: AsRef<[u8]>>` remains unrestricted because it receives caller-owned
bytes and is used by fixed-width constructors. Its documentation states that
the parser limit is not a universal instance-size invariant. `to_bytes` remains
infallible for every SDK-constructible instance.

### Inherit the bound at JWK without changing width errors

`parse_coordinate` continues to call the base64url parser and then check the
decoded 32-byte length. This bounds its codec allocation at 4 KiB and preserves
`InvalidCoordinateEncoding` versus `InvalidCoordinateLength` for current
in-budget inputs. Exact 43-character preflight is deferred because it would
change that observable precedence; JSON allocation still needs outer limits.

### Narrow evidence, not the parent audit

The crypto specification gains the codec budget and trusted-construction
caveat. `SDK-LIM-007` names codec tests as additional evidence but remains
effective for every inherited surface not yet audited under #168.

## Compatibility

The constant is additive and the `Error` variant/code are unchanged. Valid
inputs through 4,096 bytes preserve output. Longer parsed inputs newly fail,
which is the material compatibility event accepted by #197. No dependency,
feature, target, MSRV, FFI or wire representation changes for accepted values.

## Security and privacy

The early byte check prevents decoder and canonical-output allocation for
oversized borrowed text. It does not prevent the caller, transport or
deserializer from allocating the source. Error display may contain only the
static codec name and numeric byte counts; bridged public display stays static.

## Test design

- Hex: accept 4,096 lowercase/uppercase characters and preserve lowercase
  canonicalization; reject 4,097 malformed characters before odd-length
  decoding and reject 4,098 otherwise-valid characters.
- Base64url: accept a canonical 4,096-character encoding; reject 4,097
  characters before invalid-length decoding and reject an otherwise-valid
  4,098-character encoding.
- Both: prove errors contain lengths but not an input sentinel, and bridge to
  the existing stable code/capability/kind/static message.
- JWK: reject a coordinate above the shared ceiling as invalid encoding and
  retain all existing 42/43/44-character canonicality and decoded-width cases.
- Preserve existing donor compatibility vectors and all feature profiles.

## Rollback

Revert constant, checks, tests, specification and limitation evidence together.
Do not leave documentation claiming a bound if either parser check is removed.
