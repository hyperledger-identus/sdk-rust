//! Ed25519 — full lifecycle (public key, private key, keypair, generate, sign,
//! strict verify). Completes neoprism's verify-only port.

use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};

#[cfg(feature = "cose")]
use crate::cose::{CoseCurve, EncodeCose, PublicKeyCose};
use crate::enc::{EncodeArray, EncodeVec, Verifiable};
use crate::error::Error;
use crate::jwk::{EncodeJwk, JwkCurve, PublicKeyJwk};
use crate::securerandom::SecureRandom;

const KEY_SIZE: usize = 32;

fn key_type<T>() -> &'static str {
    std::any::type_name::<T>()
}

/// Ed25519 public (verifying) key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ed25519PublicKey(pub VerifyingKey);

/// Ed25519 private (signing) key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ed25519PrivateKey(pub SigningKey);

/// An Ed25519 keypair (private + public).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ed25519KeyPair {
    private: Ed25519PrivateKey,
    public: Ed25519PublicKey,
}

impl Ed25519PublicKey {
    /// Parse a 32-byte Ed25519 public key.
    pub fn from_slice(slice: &[u8]) -> Result<Self, Error> {
        let arr: [u8; KEY_SIZE] = slice.try_into().map_err(|_| Error::InvalidKeySize {
            expected: KEY_SIZE,
            actual: slice.len(),
            key_type: key_type::<Self>(),
        })?;
        let key = VerifyingKey::from_bytes(&arr)?;
        Ok(Self(key))
    }

    /// The underlying verifying key.
    pub fn as_verifying_key(&self) -> &VerifyingKey {
        &self.0
    }
}

impl Ed25519PrivateKey {
    /// Parse a 32-byte Ed25519 private key.
    pub fn from_slice(slice: &[u8]) -> Result<Self, Error> {
        let arr: [u8; KEY_SIZE] = slice.try_into().map_err(|_| Error::InvalidKeySize {
            expected: KEY_SIZE,
            actual: slice.len(),
            key_type: key_type::<Self>(),
        })?;
        Ok(Self(SigningKey::from_bytes(&arr)))
    }

    /// The 32-byte raw private key material.
    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.to_bytes()
    }

    /// Derive the corresponding public key.
    #[must_use]
    pub fn to_public_key(&self) -> Ed25519PublicKey {
        Ed25519PublicKey(self.0.verifying_key())
    }

    /// Sign `message`, returning the 64-byte Ed25519 signature.
    #[must_use]
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        let sig: Signature = self.0.sign(message);
        sig.to_bytes().to_vec()
    }
}

impl Ed25519KeyPair {
    /// Generate a fresh keypair using the injected [`SecureRandom`] entropy port.
    pub fn generate(rng: &mut impl SecureRandom) -> Self {
        let seed: [u8; KEY_SIZE] = rng
            .generate_seed(KEY_SIZE)
            .try_into()
            .expect("SecureRandom must return the requested number of bytes");
        let private = Ed25519PrivateKey(SigningKey::from_bytes(&seed));
        let public = private.to_public_key();
        Self { private, public }
    }

    /// The public half of the keypair.
    pub fn public(&self) -> &Ed25519PublicKey {
        &self.public
    }

    /// The private half of the keypair.
    pub fn private(&self) -> &Ed25519PrivateKey {
        &self.private
    }

    /// Sign `message`, returning the 64-byte Ed25519 signature.
    #[must_use]
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        self.private.sign(message)
    }
}

impl EncodeVec for Ed25519PublicKey {
    fn encode_vec(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }
}

impl EncodeArray<32> for Ed25519PublicKey {
    fn encode_array(&self) -> [u8; 32] {
        self.0.to_bytes()
    }
}

impl Verifiable for Ed25519PublicKey {
    fn verify(&self, message: &[u8], signature: &[u8]) -> bool {
        let Ok(sig) = Signature::from_slice(signature) else {
            return false;
        };
        self.0.verify_strict(message, &sig).is_ok()
    }
}

impl EncodeJwk for Ed25519PublicKey {
    fn encode_jwk(&self) -> PublicKeyJwk {
        PublicKeyJwk::new_okp(JwkCurve::Ed25519, self.encode_array())
            .expect("Ed25519 is a supported OKP JWK profile")
    }
}

#[cfg(feature = "cose")]
impl EncodeCose for Ed25519PublicKey {
    fn encode_cose(&self) -> PublicKeyCose {
        PublicKeyCose::new_okp(CoseCurve::Ed25519, self.encode_array())
            .expect("Ed25519 is a supported OKP COSE profile")
    }
}
