# Bound public crypto text codecs before decode

## Why

`identus-crypto::HexStr::from_str` and
`Base64UrlStrNoPad::from_str` currently decode an arbitrarily long borrowed
string into a new byte buffer and then allocate its canonical encoding. Issue
#197 isolates this inherited resource gap as a child of the repository-wide
audit in #168 and the Apollo parity program in #9.

The codecs are generic helpers rather than protocol-specific values, so the
bound must preserve ordinary key, signature, digest and derivation-vector
uses. A 4,096-byte encoded-text ceiling matches the crate's existing public
COSE Key byte budget and is over twenty times the largest current codec-shaped
literal in sdk-rust, Apollo or NeoPRISM.

## What Changes

- Export `MAX_CRYPTO_TEXT_BYTES` with the value 4,096.
- Reject longer `HexStr` and `Base64UrlStrNoPad` parser input before decoding
  or canonical re-encoding.
- Preserve infallible byte-to-text construction for trusted caller-owned bytes.
- Make the public JWK coordinate parser inherit the codec ceiling without
  changing its existing canonical-encoding and decoded-width error semantics.
- Prove exact-limit, one-byte-over, valid-over-limit, canonicalization,
  precedence and redaction behavior.
- Add the accepted boundary to the canonical crypto specification and narrow
  the corresponding evidence in `SDK-LIM-007` without closing #168.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `crypto`: define the public text-codec parser budget and trusted-encoding
  asymmetry.

## Impact

- **Issues:** #197, child of #168 and #9.
- **Public API:** adds `MAX_CRYPTO_TEXT_BYTES`; the existing `Error::KeyParsing`
  variant remains the local error category and keeps its stable core bridge.
- **Accepted behavior:** text longer than 4,096 UTF-8 bytes is newly rejected
  by `FromStr`, even when it is otherwise a valid encoding.
- **JWK behavior:** coordinates inherit the ceiling after JSON has already
  materialized its strings; supported coordinates remain exactly 43 ASCII
  characters encoding 32 bytes.
- **Dependencies and features:** unchanged.
- **Rollback:** remove the parser ceiling, tests and specification together and
  restore the codec clause in `SDK-LIM-007`; changing the limit later requires
  a new compatibility/resource decision.
