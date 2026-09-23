# identus-crypto

Reusable, chain-neutral cryptographic primitives for Hyperledger Identus Rust
SDK consumers.

The current surface includes Ed25519, X25519, secp256k1, P-256, SHA-2/HMAC,
BIP-39, secp256k1 BIP-32, SLIP-0010, Cardano V2 Ed25519-BIP32, public JWK, and
public COSE Key support. Feature selection and exact evidence are documented in
the repository's Apollo parity ledger.

Version `0.1.0-rc.1` is the release-activated first cryptography train. It is
experimental and carries no production, certification, or long-term
compatibility promise. Verify the immutable release receipt before depending on
the crates.io package; merging package metadata alone does not prove upload.
Secret-bearing SDK types redact diagnostics; consumers still own custody,
storage, policy, and platform hardening.

See the [SDK repository](https://github.com/hyperledger-identus/sdk-rust),
[Apollo parity report](https://github.com/hyperledger-identus/sdk-rust/blob/develop/docs/architecture/apollo-crypto-parity.md),
[feature definitions](https://github.com/hyperledger-identus/sdk-rust/blob/develop/crates/crypto/Cargo.toml),
[release policy](https://github.com/hyperledger-identus/sdk-rust/blob/develop/RELEASING.md),
and the Apache-2.0 license.
