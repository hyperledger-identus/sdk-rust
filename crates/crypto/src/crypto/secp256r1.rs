//! secp256r1 (P-256) — full lifecycle (public key, private key, keypair,
//! generate, DER sign, verify, JWK). New (parity gap vs neoprism, which had no
//! P-256).

use p256::NistP256;
use p256::ecdsa::signature::{SignerMut, Verifier};
use p256::elliptic_curve::sec1::{EncodedPoint, ToEncodedPoint};
use p256::{
    PublicKey, SecretKey,
    ecdsa::{Signature, SigningKey},
};
use zeroize::Zeroizing;

#[cfg(feature = "cose")]
use crate::cose::{CoseCurve, EncodeCose, PublicKeyCose};
use crate::enc::{EncodeArray, EncodeVec, Verifiable};
use crate::error::Error;
use crate::jwk::{EncodeJwk, JwkCurve, PublicKeyJwk};
use crate::securerandom::SecureRandom;

const PRIV_SIZE: usize = 32;
const COMPRESSED_SIZE: usize = 33;
const UNCOMPRESSED_SIZE: usize = 65;
const COORD_SIZE: usize = 32;

/// P-256 public key (compressed 33-byte / uncompressed 65-byte encodings).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct P256PublicKey(pub PublicKey);

/// P-256 private key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct P256PrivateKey(pub SecretKey);

/// A P-256 keypair (private + public).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct P256KeyPair {
    private: P256PrivateKey,
    public: P256PublicKey,
}

impl P256PublicKey {
    /// Parse a compressed (33-byte) or uncompressed (65-byte) P-256 public key.
    pub fn from_slice(slice: &[u8]) -> Result<Self, Error> {
        Ok(Self(PublicKey::from_sec1_bytes(slice)?))
    }

    /// Compressed (33-byte) SEC1 encoding.
    #[must_use]
    pub fn encode_compressed(&self) -> [u8; COMPRESSED_SIZE] {
        let bytes: EncodedPoint<NistP256> = self.0.to_encoded_point(true);
        bytes
            .as_bytes()
            .try_into()
            .expect("compressed SEC1 point is 33 bytes")
    }

    /// Uncompressed (65-byte) SEC1 encoding.
    #[must_use]
    pub fn encode_uncompressed(&self) -> [u8; UNCOMPRESSED_SIZE] {
        let bytes: EncodedPoint<NistP256> = self.0.to_encoded_point(false);
        bytes
            .as_bytes()
            .try_into()
            .expect("uncompressed SEC1 point is 65 bytes")
    }

    /// The curve point (x, y) coordinates.
    #[must_use]
    pub fn curve_point(&self) -> ([u8; COORD_SIZE], [u8; COORD_SIZE]) {
        let uncompressed = self.encode_uncompressed();
        let mut x = [0u8; COORD_SIZE];
        let mut y = [0u8; COORD_SIZE];
        x.copy_from_slice(&uncompressed[1..33]);
        y.copy_from_slice(&uncompressed[33..65]);
        (x, y)
    }
}

impl P256PrivateKey {
    /// Parse a 32-byte P-256 private key.
    pub fn from_slice(slice: &[u8]) -> Result<Self, Error> {
        let sk = SecretKey::from_slice(slice)?;
        Ok(Self(sk))
    }

    /// Derive the corresponding public key.
    #[must_use]
    pub fn to_public_key(&self) -> P256PublicKey {
        P256PublicKey(self.0.public_key())
    }

    /// Sign `message` (deterministic RFC 6979 nonce), returning a DER-encoded signature.
    #[must_use]
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        let mut signing_key = SigningKey::from(&self.0);
        let signature: Signature = signing_key.sign(message);
        signature.to_der().as_bytes().to_vec()
    }
}

impl P256KeyPair {
    /// Generate a fresh keypair using the injected [`SecureRandom`] entropy port.
    pub fn generate(rng: &mut impl SecureRandom) -> Result<Self, Error> {
        for _ in 0..16 {
            let mut bytes = Zeroizing::new([0u8; PRIV_SIZE]);
            rng.fill_bytes(bytes.as_mut())?;
            if let Ok(secret) = SecretKey::from_slice(bytes.as_ref()) {
                let private = P256PrivateKey(secret);
                let public = private.to_public_key();
                return Ok(Self { private, public });
            }
        }
        Err(Error::SecureRandomFailure)
    }

    /// The public half of the keypair.
    pub fn public(&self) -> &P256PublicKey {
        &self.public
    }

    /// The private half of the keypair.
    pub fn private(&self) -> &P256PrivateKey {
        &self.private
    }

    /// Sign `message`, returning a DER-encoded signature.
    #[must_use]
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        self.private.sign(message)
    }
}

impl EncodeVec for P256PublicKey {
    fn encode_vec(&self) -> Vec<u8> {
        self.encode_compressed().to_vec()
    }
}

impl EncodeArray<33> for P256PublicKey {
    fn encode_array(&self) -> [u8; 33] {
        self.encode_compressed()
    }
}

impl EncodeArray<65> for P256PublicKey {
    fn encode_array(&self) -> [u8; 65] {
        self.encode_uncompressed()
    }
}

impl Verifiable for P256PublicKey {
    fn verify(&self, message: &[u8], signature: &[u8]) -> bool {
        let verifying_key: p256::ecdsa::VerifyingKey = self.0.into();
        let Ok(signature) = Signature::from_der(signature) else {
            return false;
        };
        if verifying_key.verify(message, &signature).is_ok() {
            return true;
        }
        // normalized-s fallback (cheap robustness against high-s signatures
        // produced by other libraries); no bitcoin-transcode for P-256.
        let Some(normalized) = signature.normalize_s() else {
            return false;
        };
        verifying_key.verify(message, &normalized).is_ok()
    }
}

impl EncodeJwk for P256PublicKey {
    fn encode_jwk(&self) -> PublicKeyJwk {
        let (x, y) = self.curve_point();
        PublicKeyJwk::new_ec(JwkCurve::P256, x, y).expect("P-256 is a supported EC JWK profile")
    }
}

#[cfg(feature = "cose")]
impl EncodeCose for P256PublicKey {
    fn encode_cose(&self) -> PublicKeyCose {
        let (x, y) = self.curve_point();
        PublicKeyCose::new_ec(CoseCurve::P256, x, y).expect("P-256 is a supported EC2 COSE profile")
    }
}
