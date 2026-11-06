# `identus-crypto` RustCrypto 0.14 migration

Issue [#525](https://github.com/hyperledger-identus/sdk-rust/issues/525)
removes advisory-affected RustCrypto 0.13 dependencies before the next
`identus-crypto` release candidate. The canonical change-ledger identifier is
`crypto.secp256k1.rustcrypto-security-stack.v1`.

## Compatibility

Key bytes, SEC1 encodings, DER and fixed-width signatures, JVM compatibility
verification, JWK/COSE output, SDK error behavior, features and supported
targets do not change. All SDK-owned constructors, encoders, signers and
verifiers keep their signatures.

The `0.1.0-rc.1` API snapshot also records four public tuple fields whose field
types come directly from RustCrypto 0.13:

- `Secp256k1PublicKey(pub k256::PublicKey)`;
- `Secp256k1PrivateKey(pub k256::SecretKey)`;
- `P256PublicKey(pub p256::PublicKey)`; and
- `P256PrivateKey(pub p256::SecretKey)`.

The next candidate changes those field identities to the corresponding
`k256 0.14` and `p256 0.14` types. That is a Rust source break for code that
directly constructs a wrapper or reads its `.0` field. It is not a wire,
cryptographic-behavior or persisted-data break.

## Consumer action

Preferred migration: treat the wrappers as SDK values. Construct them with
`from_slice`, derive public keys with `to_public_key`, and use the SDK encoding
traits or `encode_compressed`/`encode_uncompressed` methods. This keeps
application code independent of a RustCrypto release family.

Consumers that intentionally use the tuple fields must update their direct
`k256`/`p256` dependency and imports to 0.14 when adopting the next candidate.
No 0.13 compatibility alias is provided because it would keep the rejected
`crypto-bigint 0.5.5` closure reachable.

## Rollback

Restoring RustCrypto 0.13 is not an acceptable release rollback. A consumer
that is not ready for the source migration can remain on `0.1.0-rc.1` only as
an explicitly risk-accepted, isolated transition while it moves to SDK byte
boundaries; it must not interpret that candidate as security-current.
