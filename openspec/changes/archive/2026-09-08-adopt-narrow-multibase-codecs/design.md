## Context

`VerificationMethod` stores suite-defined properties in an Identus-owned
`BTreeMap<String, Value>`. It recognizes `publicKeyJwk` and
`publicKeyMultibase`, rejects their simultaneous use and preserves the exact
wire value. JWK input receives structural/private-material checks; multibase
input receives only printable-string and 16 KiB checks.

Controlled Identifiers 1.0 defines `z` (base58-btc) and `u`
(base64url-no-pad) as common normative multibase encodings and allows other
registry entries with weaker interoperability. did:key v0.9 uses the same two
prefixes. This SDK needs a conservative interoperable subset, not the entire
multibase registry.

## Goals

- Reuse maintained base algorithm implementations rather than write Base58.
- Keep prefix allow-list, bounds, canonicality and error mapping SDK-owned.
- Add only the dependency surface required by a current DID document consumer.
- Preserve exact valid JSON and public accessor compatibility.
- Leave key meaning to later method/cryptosuite capabilities.

## Decisions

### Compose narrow engines

Pin `bs58 = "=0.5.1"` with `default-features = false` and
`features = ["alloc"]`. Reuse the workspace's existing `base64 0.22`
dependency with allocation support. A private DID module dispatches:

- `z...` to the default Bitcoin Base58 alphabet;
- `u...` to URL-safe Base64 without padding;
- all other prefixes to the existing invalid-document error.

The dispatcher rejects an encoded value larger than 4 KiB before calling a
codec, decodes to owned bounded bytes, rejects an empty payload,
re-encodes with the same selected engine and compares the complete input.
Upstream types and detailed errors are discarded.

### Reject the umbrella implementation for this seam

`multibase 0.9.3` with default features disabled still resolves support for
Base2/8/10/16/32/36/45/58/64 and Base256Emoji. Relative to the current SDK
lock it adds nine package names: `multibase`, `base-x`, `base45`,
`base256emoji`, `const-str`, `match-lookup`, `data-encoding`,
`data-encoding-macro` and `data-encoding-macro-internal`. Existing
proc-macro support packages are reused, but unused runtime/build capabilities
remain coupled.

The narrow composition adds only `bs58`; `base64` is already locked. The
SDK-owned dispatcher is protocol policy rather than a reimplementation of
either base algorithm.

### Preserve the current facade

`public_key_multibase()` continues to return `Option<&str>`; serialization
retains the exact accepted string and `VerificationMethod` remains an open
suite-property model. Invalid known material continues to map to
`DocumentError::InvalidString`, so no attacker-controlled value or upstream
diagnostic enters errors.

The recognized-carrier ceiling is tightened from 16 KiB to 4 KiB before
decoding. A release-mode probe on the local aarch64-Darwin host measured
decode-plus-re-encode of repeated non-zero Base58 payloads at approximately
1 ms for 1,024 bytes, 27 ms for 4,096 bytes and 332 ms for 16,383 bytes. The
smaller limit bounds the quadratic codec cost while remaining above currently
named W3C key encodings and common post-quantum public-key sizes. The existing
256 KiB whole-document and item-count budgets still apply. Empty decoded bytes
are rejected because the property claims public key material.

### Keep key semantics out

Successful multibase decoding proves only carrier syntax and canonical text.
It does not validate the leading multicodec varint, supported key type, curve,
key length or proof purpose. A later did:key or Multikey issue must add those
rules with exact normative vectors; it may reuse this private seam.

## Dependency and security shape

`bs58 0.5.1` is MIT OR Apache-2.0, has no required normal dependency when
`alloc` alone is enabled and compiles without `std`. It does not declare an
MSRV; integrated Rust 1.98.1 and supported-target gates are therefore required.
The source repository is unarchived but last changed in March 2024, so the
exact pin and maintenance trigger are explicit.

The crate denies unsafe code except for one locally allowed conversion used by
its public mutable-`str` output target. This SDK calls only owned
`String`/`Vec<u8>` paths, does not expose `EncodeTarget`, and adds no
unsafe block. There is no FFI, native library, network, runtime, clock or secret
handling.

## Rollback

Revert the focused PR, restore the printable-string check and remove `bs58`
from the DID/workspace manifests and lock. Previously accepted malformed
values could become valid again after rollback, but no accepted value changes
representation and no storage migration is required.
