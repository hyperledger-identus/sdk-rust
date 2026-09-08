//! Cryptographic primitive operations for the Identus Rust SDK.
//!
//! `identus-crypto` is the canonical cryptographic capability — the reference
//! implementation that the TypeScript, Swift, and KMP bindings are ported
//! *from*. It owns **primitive crypto operations on key material** (bytes-in,
//! bytes-out): concrete curve key types (Ed25519, X25519, secp256k1, P-256)
//! with full sign/verify/generate, encoding/JWK/COSE trait polymorphism, SHA-2
//! hashing, hierarchical derivation (BIP32/SLIP-0010/BIP39), and Ed25519→X25519
//! conversion. Sits in the `domain-primitives` layer and depends only on
//! `identus-core` (plus workspace-level external crates).
//!
//! The only infrastructure **port** is [`SecureRandom`] (entropy), the one
//! place a known second backend exists (`ring` breaks on `wasm32`, so a
//! `getrandom`-backed cross-platform adapter is used instead). The port is
//! defined here; its concrete adapters live in the outer-boundary
//! `identus-adapters-entropy` crate and are injected into key generation and
//! mnemonic creation via dependency injection. The crate has **no `ring`
//! dependency**, so it builds on `wasm32` with default features.
//!
//! Key management (`KeyHandle`, `KeyStore`, non-exportable signing, hardware/
//! `KMS`-bound signers) is **not** in scope here — that is an `identus-wallet`
//! concern per the secure-storage boundary. This crate operates on key
//! *material*; it does not decide where the key lives.

use identus_core::Component;

/// Maximum UTF-8 byte length accepted by the public hex and base64url text
/// parsers.
///
/// This is a parser resource boundary, not a universal instance-size limit.
/// Infallible encoding from caller-owned bytes remains caller-budgeted.
pub const MAX_CRYPTO_TEXT_BYTES: usize = 4_096;

pub mod enc;
pub mod error;
pub mod securerandom;

#[cfg(feature = "base64")]
pub mod base64;
#[cfg(feature = "cose")]
pub mod cose;
#[cfg(feature = "hash")]
pub mod hash;
#[cfg(feature = "hex")]
pub mod hex;
#[cfg(feature = "jwk")]
pub mod jwk;

#[cfg(all(feature = "x25519", feature = "hash"))]
pub mod convert;

pub mod crypto;

#[cfg(any(feature = "derivation", feature = "cardano-bip32"))]
pub mod derivation;

pub use enc::{EncodeArray, EncodeVec, Verifiable};
pub use error::Error;
pub use securerandom::SecureRandom;

#[cfg(feature = "base64")]
pub use base64::Base64UrlStrNoPad;
#[cfg(feature = "cose")]
pub use cose::{
    CoseCoordinate, CoseCurve, CoseEcY, CoseKeyError, CoseKeyType, EncodeCose,
    MAX_COSE_ADDITIONAL_PARAMETERS, MAX_COSE_KEY_BYTES, MAX_COSE_NESTING_DEPTH, PublicKeyCose,
};
#[cfg(feature = "hash")]
pub use hash::{Sha256Digest, Sha512Digest, sha256, sha512};
#[cfg(feature = "hex")]
pub use hex::HexStr;
#[cfg(feature = "jwk-thumbprint")]
pub use jwk::JwkThumbprint;
#[cfg(feature = "jwk")]
pub use jwk::{EncodeJwk, JwkCoordinate, JwkCurve, JwkError, JwkKeyType, PublicKeyJwk};

#[cfg(all(feature = "x25519", feature = "hash"))]
pub use convert::ConvertEd25519;

#[cfg(feature = "ed25519")]
pub use crypto::ed25519;
#[cfg(feature = "secp256k1")]
pub use crypto::secp256k1;
#[cfg(feature = "secp256r1")]
pub use crypto::secp256r1;
#[cfg(feature = "x25519")]
pub use crypto::x25519;

#[cfg(feature = "ed25519")]
pub use crypto::ed25519::{Ed25519KeyPair, Ed25519PrivateKey, Ed25519PublicKey};
#[cfg(feature = "secp256k1")]
pub use crypto::secp256k1::{
    CurvePoint, Secp256k1KeyPair, Secp256k1PrivateKey, Secp256k1PublicKey,
};
#[cfg(feature = "secp256r1")]
pub use crypto::secp256r1::{P256KeyPair, P256PrivateKey, P256PublicKey};
#[cfg(feature = "x25519")]
pub use crypto::x25519::{X25519KeyPair, X25519PrivateKey, X25519PublicKey};

#[cfg(any(feature = "derivation", feature = "cardano-bip32"))]
pub use derivation::path;
#[cfg(feature = "cardano-bip32")]
pub use derivation::{CardanoV2ExtendedPrivateKey, CardanoV2ExtendedPublicKey};
#[cfg(feature = "derivation")]
pub use derivation::{EdHDKey, HDKey, MnemonicHelper};
#[cfg(any(feature = "derivation", feature = "cardano-bip32"))]
pub use derivation::{
    MAX_DERIVATION_PATH_AXES, MAX_DERIVATION_PATH_BYTES, MAX_HD_SEED_BYTES, MIN_HD_SEED_BYTES,
};

/// Metadata for the `identus-crypto` crate.
pub const COMPONENT: Component = Component {
    name: "identus-crypto",
    summary: "Cryptographic primitive operations for the Identus Rust SDK.",
};
