//! secp256k1 — full lifecycle (public key, private key, keypair, generate,
//! DER sign, JVM-compat verify). Ported from neoprism, with the sign/generate
//! side completed here.

use k256::Secp256k1;
use k256::ecdsa::signature::{SignerMut, Verifier};
use k256::elliptic_curve::sec1::{EncodedPoint, ToEncodedPoint};
use k256::{
    PublicKey, SecretKey,
    ecdsa::{Signature, SigningKey},
};

use crate::enc::{EncodeArray, EncodeVec, Verifiable};
use crate::error::Error;
use crate::jwk::{EncodeJwk, JwkCurve, PublicKeyJwk};
use crate::securerandom::SecureRandom;

const PRIV_SIZE: usize = 32;

/// secp256k1 public key (compressed 33-byte / uncompressed 65-byte encodings).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Secp256k1PublicKey(pub PublicKey);

/// secp256k1 private key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Secp256k1PrivateKey(pub SecretKey);

/// A secp256k1 curve point as two 32-byte coordinates.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CurvePoint {
    /// The x coordinate.
    pub x: [u8; 32],
    /// The y coordinate.
    pub y: [u8; 32],
}

/// A secp256k1 keypair (private + public).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Secp256k1KeyPair {
    private: Secp256k1PrivateKey,
    public: Secp256k1PublicKey,
}

impl Secp256k1PublicKey {
    /// Parse a compressed (33-byte) or uncompressed (65-byte) secp256k1 public key.
    pub fn from_slice(slice: &[u8]) -> Result<Self, Error> {
        Ok(Self(PublicKey::from_sec1_bytes(slice)?))
    }

    /// Compressed (33-byte) SEC1 encoding.
    #[must_use]
    pub fn encode_compressed(&self) -> [u8; 33] {
        let bytes: EncodedPoint<Secp256k1> = self.0.to_encoded_point(true);
        bytes
            .as_bytes()
            .try_into()
            .expect("compressed SEC1 point is 33 bytes")
    }

    /// Uncompressed (65-byte) SEC1 encoding.
    #[must_use]
    pub fn encode_uncompressed(&self) -> [u8; 65] {
        let bytes: EncodedPoint<Secp256k1> = self.0.to_encoded_point(false);
        bytes
            .as_bytes()
            .try_into()
            .expect("uncompressed SEC1 point is 65 bytes")
    }

    /// The curve point (x, y) coordinates.
    #[must_use]
    pub fn curve_point(&self) -> CurvePoint {
        let uncompressed = self.encode_uncompressed();
        let mut x = [0u8; 32];
        let mut y = [0u8; 32];
        x.copy_from_slice(&uncompressed[1..33]);
        y.copy_from_slice(&uncompressed[33..65]);
        CurvePoint { x, y }
    }
}

impl Secp256k1PrivateKey {
    /// Parse a 32-byte secp256k1 private key.
    pub fn from_slice(slice: &[u8]) -> Result<Self, Error> {
        let sk = SecretKey::from_slice(slice)?;
        Ok(Self(sk))
    }

    /// Derive the corresponding public key.
    #[must_use]
    pub fn to_public_key(&self) -> Secp256k1PublicKey {
        Secp256k1PublicKey(self.0.public_key())
    }

    /// Sign `message` (deterministic RFC 6979 nonce), returning a DER-encoded signature.
    #[must_use]
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        let mut signing_key = SigningKey::from(&self.0);
        let signature: Signature = signing_key.sign(message);
        signature.to_der().as_bytes().to_vec()
    }
}

impl Secp256k1KeyPair {
    /// Generate a fresh keypair using the injected [`SecureRandom`] entropy port.
    pub fn generate(rng: &mut impl SecureRandom) -> Self {
        // A uniform random 32-byte scalar is almost always a valid secp256k1
        // private key; retry on the astronomically rare out-of-range draw.
        for _ in 0..16 {
            let bytes = rng.generate_seed(PRIV_SIZE);
            if let Ok(arr) = <[u8; PRIV_SIZE]>::try_from(bytes.as_slice()) {
                if let Ok(secret) = SecretKey::from_slice(&arr) {
                    let private = Secp256k1PrivateKey(secret);
                    let public = private.to_public_key();
                    return Self { private, public };
                }
            }
        }
        panic!("SecureRandom failed to produce a valid secp256k1 private key after 16 draws")
    }

    /// The public half of the keypair.
    pub fn public(&self) -> &Secp256k1PublicKey {
        &self.public
    }

    /// The private half of the keypair.
    pub fn private(&self) -> &Secp256k1PrivateKey {
        &self.private
    }

    /// Sign `message`, returning a DER-encoded signature.
    #[must_use]
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        self.private.sign(message)
    }
}

impl EncodeVec for Secp256k1PublicKey {
    fn encode_vec(&self) -> Vec<u8> {
        self.encode_compressed().to_vec()
    }
}

impl EncodeArray<33> for Secp256k1PublicKey {
    fn encode_array(&self) -> [u8; 33] {
        self.encode_compressed()
    }
}

impl EncodeArray<65> for Secp256k1PublicKey {
    fn encode_array(&self) -> [u8; 65] {
        self.encode_uncompressed()
    }
}

impl Verifiable for Secp256k1PublicKey {
    /// JVM-compat verification (preserved verbatim from the JVM PRISM node
    /// `Secp256k1Lib`): raw verify → normalize-s verify → bitcoin-transcode
    /// verify. Signatures produced by bouncycastle/bitcoinj are not always
    /// verifiable by a vanilla `k256` verify, so the fallbacks are a
    /// cross-node compatibility contract.
    fn verify(&self, message: &[u8], signature: &[u8]) -> bool {
        let verifying_key: k256::ecdsa::VerifyingKey = self.0.into();

        let Ok(signature) = Signature::from_der(signature) else {
            return false;
        };

        // vanilla verification
        if verifying_key.verify(message, &signature).is_ok() {
            return true;
        }

        // normalized-s verification. The JVM (`Secp256k1Lib.verify`) always
        // proceeds to the transcode step regardless of whether `s` was already
        // low, so we must NOT early-return here when `normalize_s` yields `None`
        // — doing so would skip the transcode path for exactly the low-s
        // LE-encoded signatures it is meant to recover. Fall through to the
        // transcode path using the normalized-or-original signature.
        let normalized = signature.normalize_s().unwrap_or(signature);
        if verifying_key.verify(message, &normalized).is_ok() {
            return true;
        }

        // transcoded (bitcoin byte-order) verification: re-interpret `r` and
        // `s` with their byte order reversed, matching the JVM PRISM node's
        // bitcoin-transcode compatibility path. The JVM feeds a compact
        // (raw `r||s`) reversed signature to acinq's `Secp256k1.verify`; the
        // k256 equivalent is to rebuild a `Signature` from the reversed
        // scalars via `from_scalars` (NOT `from_der`, which expects DER
        // framing and would always reject the raw reversed bytes).
        let (r_bytes, s_bytes) = normalized.split_bytes();
        let transcoded = transcode_scalars_to_bitcoin(&r_bytes, &s_bytes);
        let Ok(transcoded) = Signature::from_scalars(transcoded.0, transcoded.1) else {
            return false;
        };
        verifying_key.verify(message, &transcoded).is_ok()
    }
}

/// Reverse the byte order of each 32-byte scalar half (`r`, then `s`),
/// matching the JVM PRISM node's bitcoin-transcode compatibility path.
/// Returns the reversed halves as owned arrays ready for `Signature::from_scalars`.
fn transcode_scalars_to_bitcoin(r: &[u8], s: &[u8]) -> ([u8; 32], [u8; 32]) {
    let mut r_rev = [0u8; 32];
    let mut s_rev = [0u8; 32];
    r_rev.copy_from_slice(r);
    s_rev.copy_from_slice(s);
    r_rev.reverse();
    s_rev.reverse();
    (r_rev, s_rev)
}

impl EncodeJwk for Secp256k1PublicKey {
    fn encode_jwk(&self) -> PublicKeyJwk {
        let point = self.curve_point();
        PublicKeyJwk::new_ec(JwkCurve::Secp256k1, point.x, point.y)
            .expect("secp256k1 is a supported EC JWK profile")
    }
}
