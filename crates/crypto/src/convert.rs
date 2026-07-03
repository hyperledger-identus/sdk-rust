//! `ConvertEd25519` — convert an Ed25519 private key to an X25519 private key
//! (SHA-512 of the first 32 bytes + clamping). Ported from the KMP
//! `ConvertEd25519`.

use crate::crypto::x25519::X25519PrivateKey;
use crate::error::Error;

/// Ed25519 → X25519 private-key conversion.
pub struct ConvertEd25519;

impl ConvertEd25519 {
    /// Convert an Ed25519 private key (`secret_key`, 32 bytes) into an X25519
    /// private key by hashing the first 32 bytes with SHA-512 and clamping the
    /// result to a valid X25519 scalar.
    pub fn convert_secret_key_to_x25519(secret_key: &[u8]) -> Result<X25519PrivateKey, Error> {
        if secret_key.len() < 32 {
            return Err(Error::InvalidKeySize {
                expected: 32,
                actual: secret_key.len(),
                key_type: std::any::type_name::<Self>(),
            });
        }
        let digest = crate::hash::sha512(&secret_key[0..32]);
        let mut hashed = [0u8; 32];
        hashed.copy_from_slice(&digest.as_array()[0..32]);
        // Clamp to a valid Curve25519 scalar.
        hashed[0] &= 248;
        hashed[31] &= 127;
        hashed[31] |= 64;
        X25519PrivateKey::from_slice(&hashed)
    }
}
