//! X25519 — full lifecycle (public key, private key, keypair, generate, and
//! Diffie-Hellman shared-secret derivation). Completes neoprism's public-key-only
//! port.

use x25519_dalek::{PublicKey, StaticSecret};

use crate::enc::{EncodeArray, EncodeVec};
use crate::error::Error;
use crate::jwk::{EncodeJwk, JwkCurve, PublicKeyJwk};
use crate::securerandom::SecureRandom;

const KEY_SIZE: usize = 32;

fn key_type<T>() -> &'static str {
    std::any::type_name::<T>()
}

/// X25519 public key.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct X25519PublicKey(pub PublicKey);

/// X25519 private key.
#[derive(Clone)]
pub struct X25519PrivateKey(pub StaticSecret);

/// An X25519 keypair (private + public).
#[derive(Clone)]
pub struct X25519KeyPair {
    private: X25519PrivateKey,
    public: X25519PublicKey,
}

impl X25519PublicKey {
    /// Parse a 32-byte X25519 public key.
    pub fn from_slice(slice: &[u8]) -> Result<Self, Error> {
        let arr: [u8; KEY_SIZE] = slice.try_into().map_err(|_| Error::InvalidKeySize {
            expected: KEY_SIZE,
            actual: slice.len(),
            key_type: key_type::<Self>(),
        })?;
        Ok(Self(PublicKey::from(arr)))
    }
}

impl X25519PrivateKey {
    /// Parse a 32-byte X25519 private key.
    pub fn from_slice(slice: &[u8]) -> Result<Self, Error> {
        let arr: [u8; KEY_SIZE] = slice.try_into().map_err(|_| Error::InvalidKeySize {
            expected: KEY_SIZE,
            actual: slice.len(),
            key_type: key_type::<Self>(),
        })?;
        Ok(Self(StaticSecret::from(arr)))
    }

    /// The 32-byte raw private key material.
    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.to_bytes()
    }

    /// Derive the corresponding public key.
    #[must_use]
    pub fn to_public_key(&self) -> X25519PublicKey {
        X25519PublicKey(PublicKey::from(&self.0))
    }

    /// Derive a Diffie-Hellman shared secret with `their_public`.
    #[must_use]
    pub fn derive_shared(&self, their_public: &X25519PublicKey) -> Vec<u8> {
        self.0.diffie_hellman(&their_public.0).to_bytes().to_vec()
    }
}

impl X25519KeyPair {
    /// Generate a fresh keypair using the injected [`SecureRandom`] entropy port.
    pub fn generate(rng: &mut impl SecureRandom) -> Self {
        let seed: [u8; KEY_SIZE] = rng
            .generate_seed(KEY_SIZE)
            .try_into()
            .expect("SecureRandom must return the requested number of bytes");
        let private = X25519PrivateKey(StaticSecret::from(seed));
        let public = private.to_public_key();
        Self { private, public }
    }

    /// The public half of the keypair.
    pub fn public(&self) -> &X25519PublicKey {
        &self.public
    }

    /// The private half of the keypair.
    pub fn private(&self) -> &X25519PrivateKey {
        &self.private
    }

    /// Derive a Diffie-Hellman shared secret with `their_public`.
    #[must_use]
    pub fn derive_shared(&self, their_public: &X25519PublicKey) -> Vec<u8> {
        self.private.derive_shared(their_public)
    }
}

impl EncodeVec for X25519PublicKey {
    fn encode_vec(&self) -> Vec<u8> {
        self.0.as_bytes().to_vec()
    }
}

impl EncodeArray<32> for X25519PublicKey {
    fn encode_array(&self) -> [u8; 32] {
        self.0.to_bytes()
    }
}

impl EncodeJwk for X25519PublicKey {
    fn encode_jwk(&self) -> PublicKeyJwk {
        PublicKeyJwk::new_okp(JwkCurve::X25519, self.encode_array())
            .expect("X25519 is a supported OKP JWK profile")
    }
}
