# ADR 0085: compose narrow codecs for multibase key carriers

- **Status:** Accepted for implementation
- **Date:** 2026-09-08
- **Issue:** [#156](https://github.com/hyperledger-identus/sdk-rust/issues/156)
- **Supersedes:** ADR 0061's conditional production disposition for
  `multibase 0.9.3`
- **Decision authority:** IDR-040 and the current DID document
  `publicKeyMultibase` consumer

## Context

The DID document model recognizes `publicKeyMultibase` but currently proves
only that it is a printable, non-empty string within 16 KiB. The earlier
portfolio selected `multibase 0.9.3` conditionally because its transitive
`base45 3.2.0` needed a newer compiler than the then-effective Rust 1.85.
The SDK now uses exact Rust 1.98.1, so compiler compatibility is no longer the
blocking question.

Focused reassessment found a cohesion problem. Even without default features,
`multibase 0.9.3` imports every implemented base, including Base45 and
experimental Base256Emoji, plus build-time encoding macros. Nine package names
would be new to the current lock. The current verification-material standards
and planned did:key seam need only base58-btc and unpadded base64url.

## Decision

Do not adopt `multibase 0.9.3` for this boundary. Compose:

1. exact `bs58 0.5.1`, defaults disabled with `alloc`, for `z`
   base58-btc;
2. the existing workspace `base64 0.22` engine for `u`
   base64url-no-pad; and
3. a small private Identus dispatcher that owns the prefix allow-list, 16 KiB
   encoded precheck, non-empty decoded rule, canonical re-encoding comparison
   and stable redacted error mapping.

Only `z` and `u` are accepted. Upstream types and errors remain private.
The existing `Option<&str>` accessor and exact JSON representation remain
unchanged. Successful carrier validation does not interpret multicodec
varints, key type, curve, key length or cryptographic validity.

## Evidence

`bs58 0.5.1` adds one package and no required normal transitive dependency
under the selected feature. It is MIT OR Apache-2.0 and compiles without
`std`. Its crates.io checksum is
`bf88ba1141d185c399bee5288d850d63b8369520c1eafc32a0430b5b6c287bf4`;
the annotated tag object is
`54fd9a73d03b0d62921b27d47e5a0288485cba58`, release commit
`7d3c9282d2595612e5474df93dd0e017db9b684f`, and packaged
`src/lib.rs` SHA-256 is
`eee508c54fc64a0ffb58d6db610559671e3a3d23f6a98a9a003f399701ab4217`.

The repository is unarchived but its last source commit is from March 2024.
The crate declares no MSRV. Exact pinning plus integrated Rust 1.98.1,
supported-target, advisory and license checks are therefore required.

The crate denies unsafe code except for one scoped implementation that encodes
directly into a caller's mutable `str`. The SDK uses only the owned
`String` and `Vec<u8>` paths, which do not dispatch through that target,
adds no unsafe block and exposes no upstream trait.

`multibase 0.9.3` remains a standards implementation and potential future
candidate, but its current nine-name incremental cone is disproportionate.
Its release checksum is
`7e0e4a371cbf1dfd666b658ba137763edb23c45beb43cfe369b5593cd6b437b6`
and release commit is
`cdeda567ce45c8d37ced0fd4eac7f7bac5a445c3`.

## Consequences

- The SDK reuses both closed base algorithms without importing unused
  registry implementations.
- Prefix and interoperability policy stays explicit and cohesive in DID Core.
- Existing malformed placeholders become invalid and must be replaced with
  canonical fixtures.
- Future DID/VC capabilities can request another prefix through a focused
  normative decision rather than inheriting blanket support.
- The project owns a small dispatch/canonicality layer, but not Base58 or
  Base64 mechanics.

## Alternatives rejected

### Adopt the generic multibase crate now

It is technically capable and now compiler-compatible, but its feature model
cannot remove unrelated algorithms. Passing MSRV does not justify the
incremental cone for two prefixes.

### Implement Base58 locally

This would recreate a closed, error-prone conversion algorithm to avoid one
small dependency. The narrow crate has better conformance value.

### Keep printable-string validation

This preserves maximum extension tolerance but misrepresents malformed data as
recognized public key material. Unknown suite properties remain open; a known
verification-material property should fail closed.

## Verification and rollback

Official `z`/`u` vectors, malformed alphabet/padding/prefix/empty/oversize
cases, native/serde parity and all repository target/dependency gates must
pass. Revert the focused PR to restore loose validation and remove `bs58`;
accepted values require no data migration.
