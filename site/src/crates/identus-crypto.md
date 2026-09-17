# `identus-crypto`

`identus-crypto` provides reusable primitive operations on key material. Its
contract is bytes-in/bytes-out cryptography with typed boundaries—not custody,
wallet orchestration, or protocol policy.

## Capability surface

- Ed25519 signing and verification;
- X25519 key agreement material and Ed25519-to-X25519 conversion;
- secp256k1 and P-256 keys, signing, and verification;
- SHA-256/SHA-512 and supporting HMAC/PBKDF2 paths;
- BIP-39, secp256k1 BIP-32, SLIP-0010, and Cardano V2 Ed25519-BIP32;
- bounded hex, Base64URL, public JWK, JWK thumbprint, and public COSE Key
  representations;
- caller-injected `SecureRandom` for key generation and mnemonic creation.

## Feature policy

Algorithms and encodings are feature-gated. Consumers should disable defaults
and select the smallest reviewed capability cone when they do not need the
full suite.

| Feature | Adds |
| --- | --- |
| `hash` | SHA-256 and SHA-512 digest values/functions |
| `ed25519` | Ed25519 key and signature operations; public JWK support |
| `x25519` | X25519 keys; conversion also requires `hash` |
| `secp256k1` | K-256 key and ECDSA operations |
| `secp256r1` | P-256 key and ECDSA operations |
| `jwk` / `jwk-thumbprint` | Bounded public JWK values and RFC 7638 thumbprints |
| `cose` | Bounded public COSE Key values |
| `derivation` | BIP-39, BIP-32/SLIP-0010, HMAC, and required curves |
| `cardano-bip32` | Cardano V2 extended Ed25519 keys |
| `kmp-compat` | Explicit legacy KMP/Apollo interop path |

## Security boundary

Secret-bearing SDK types redact diagnostics and use zeroization where their
owned representation permits it. The workspace forbids first-party unsafe
code. Entropy is explicit and fallible; the production adapter lives outside
the primitive crate.

Consumers still own:

- non-exportable key handles and hardware/KMS integration;
- secure storage, backup, recovery, access control, and consent;
- algorithm/profile negotiation and protocol verification policy;
- side-channel and platform hardening appropriate to their threat model;
- rate, message-size, and work budgets at the calling boundary.

## Example: minimal hashing

```rust
use identus_crypto::sha256;

let digest = sha256(b"bounded public input");
assert_eq!(digest.as_array().len(), 32);
```

Before registry publication, evaluation uses a full immutable Git revision and
an explicit feature set. See [Adoption](../adoption.md).

[Source](https://github.com/hyperledger-identus/sdk-rust/tree/develop/crates/crypto)
