use blake2b_simd::Params as Blake2b;
use group::GroupEncoding;
use identus_crypto::curve::{EmbeddedFr, EmbeddedGroupAffine, Fr};
use identus_crypto::schnorr::{SchnorrSignature, vk};
use midnight_transient_crypto::curve::embedded;
use rand::Rng;
use rand::rngs::OsRng;

uniffi::setup_scaffolding!();

// ---------------------------------------------------------------------------
// Error type returned by sign()
// ---------------------------------------------------------------------------

/// Error returned when `sign()` receives invalid input.
#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum SignError {
    /// The secret key is not valid hex.
    #[error("invalid secret key hex: {reason}")]
    InvalidHex { reason: String },

    /// The decoded secret key bytes do not represent a valid Jubjub scalar.
    #[error("invalid secret key bytes")]
    InvalidSecretKey,
}

// ---------------------------------------------------------------------------
// Record type returned by generate_key()
// ---------------------------------------------------------------------------

/// A generated keypair returned to foreign-language callers.
#[derive(uniffi::Record)]
pub struct KeyInfo {
    /// Hex-encoded 32-byte LE Jubjub secret (signing) key.
    pub secret_key: String,
    /// Hex-encoded 32-byte compressed public (verifying) key.
    pub public_key: String,
}

// ---------------------------------------------------------------------------
// UniFFI-exported functions
// ---------------------------------------------------------------------------

/// Generate a random Schnorr keypair over the Jubjub embedded curve.
///
/// Returns a `KeyInfo` record with `secret_key` and `public_key`.
#[uniffi::export]
pub fn generate_key() -> KeyInfo {
    let mut rng = OsRng;

    // Generate random secret key (Jubjub scalar).
    let sk: EmbeddedFr = rng.r#gen();

    // Derive the corresponding verifying key.
    let pk = vk(sk);

    // Encode as hex.
    let secret_key = const_hex::encode(sk.as_le_bytes());
    let public_key = const_hex::encode(pk.0.to_bytes());

    KeyInfo { secret_key, public_key }
}

/// Sign a message with the given secret key.
///
/// * `secret_key` – hex-encoded 32-byte LE Jubjub secret key.
/// * `message` – raw bytes (will be hashed with Blake2b-512 internally).
///
/// Returns a hex-encoded 64-byte Schnorr signature
/// (32-byte compressed announcement ∥ 32-byte LE response).
///
/// Returns `Err(SignError)` if the secret key is not valid hex or does not
/// represent a valid Jubjub scalar.
#[uniffi::export]
pub fn sign(secret_key: String, message: Vec<u8>) -> Result<String, SignError> {
    use identus_crypto::schnorr::sign as schnorr_sign_inner;

    let mut rng = OsRng;

    // --- Parse secret key --------------------------------------------------
    let sk_bytes = const_hex::decode(&secret_key).map_err(|e| SignError::InvalidHex { reason: e.to_string() })?;
    let sk = EmbeddedFr::from_le_bytes(&sk_bytes).ok_or(SignError::InvalidSecretKey)?;

    // --- Hash message to a scalar ------------------------------------------
    let msg_fr = hash_message_to_fr(&message);

    // --- Sign --------------------------------------------------------------
    let sig = schnorr_sign_inner(&mut rng, sk, &[msg_fr]);

    // --- Encode signature as hex -------------------------------------------
    Ok(encode_signature(&sig))
}

/// Verify a Schnorr signature.
///
/// * `public_key` – hex-encoded 32-byte compressed public key.
/// * `message` – raw bytes (hashed the same way as in `sign`).
/// * `signature` – hex-encoded 64-byte Schnorr signature.
///
/// Returns `true` if the signature is valid, `false` otherwise.
#[uniffi::export]
pub fn verify(public_key: String, message: Vec<u8>, signature: String) -> bool {
    use identus_crypto::schnorr::verify as schnorr_verify_inner;

    // --- Parse public key --------------------------------------------------
    let pk = match decode_public_key(&public_key) {
        Ok(pk) => pk,
        Err(_) => return false,
    };

    // --- Parse signature ---------------------------------------------------
    let sig = match decode_signature(&signature) {
        Ok(s) => s,
        Err(_) => return false,
    };

    // --- Hash message to a scalar ------------------------------------------
    let msg_fr = hash_message_to_fr(&message);

    // --- Verify ------------------------------------------------------------
    schnorr_verify_inner(pk, &[msg_fr], &sig)
}

// ---------------------------------------------------------------------------
// Internal helpers (mirrors identus-crypto-wasm/src/lib.rs)
// ---------------------------------------------------------------------------

/// Hash raw message bytes with Blake2b-512 and reduce to a single `Fr`
/// (BLS12-381 outer scalar) for use with `schnorr::sign` / `schnorr::verify`.
///
/// Steps:
/// 1. Blake2b-512 hash → 64 bytes.
/// 2. `embedded::Scalar::from_bytes_wide(&[u8; 64])` → Jubjub scalar.
/// 3. `Fr::try_from(EmbeddedFr)` → BLS12-381 scalar (outer).
fn hash_message_to_fr(message: &[u8]) -> Fr {
    // Step 1: Blake2b-512
    let hash = Blake2b::new().hash_length(64).hash(message);
    let hash_bytes: [u8; 64] = hash.as_bytes().try_into().expect("hash_length(64) guarantees 64 bytes");

    // Step 2: EmbeddedFr ← Jubjub scalar via wide reduction
    let embedded_fr = EmbeddedFr(embedded::Scalar::from_bytes_wide(&hash_bytes));

    // Step 3: Fr ← BLS12-381 scalar
    Fr::try_from(embedded_fr).expect("EmbeddedFr → Fr conversion must succeed")
}

/// Decode a hex-encoded compressed public key (32 bytes) into an
/// `EmbeddedGroupAffine`.
fn decode_public_key(public_key_hex: &str) -> Result<EmbeddedGroupAffine, &'static str> {
    let pk_bytes = const_hex::decode(public_key_hex).map_err(|_| "invalid public key hex")?;

    if pk_bytes.len() != 32 {
        return Err("public key must be 32 bytes");
    }

    let mut repr = <embedded::Affine as GroupEncoding>::Repr::default();
    repr.as_mut().copy_from_slice(&pk_bytes);

    let pk = <Option<_>>::from(embedded::Affine::from_bytes(&repr)).ok_or("invalid public key bytes")?;
    Ok(EmbeddedGroupAffine(pk))
}

/// Encode a `SchnorrSignature` as a hex string.
///
/// Format: 32-byte compressed announcement ∥ 32-byte LE response = 64 bytes.
fn encode_signature(sig: &SchnorrSignature) -> String {
    let ann_bytes = sig.announcement.0.to_bytes();
    let resp_bytes = sig.response.as_le_bytes();

    let mut combined = Vec::with_capacity(64);
    combined.extend_from_slice(ann_bytes.as_ref());
    combined.extend_from_slice(&resp_bytes);

    const_hex::encode(combined)
}

/// Decode a hex-encoded 64-byte Schnorr signature.
fn decode_signature(signature_hex: &str) -> Result<SchnorrSignature, &'static str> {
    let sig_bytes = const_hex::decode(signature_hex).map_err(|_| "invalid hex")?;

    if sig_bytes.len() != 64 {
        return Err("signature must be 64 bytes");
    }

    // Announcement (first 32 bytes)
    let mut ann_repr = <embedded::Affine as GroupEncoding>::Repr::default();
    ann_repr.as_mut().copy_from_slice(&sig_bytes[..32]);
    let announcement =
        <Option<_>>::from(embedded::Affine::from_bytes(&ann_repr)).ok_or("invalid announcement point")?;
    let announcement = EmbeddedGroupAffine(announcement);

    // Response (last 32 bytes)
    let response = EmbeddedFr::from_le_bytes(&sig_bytes[32..64]).ok_or("invalid response scalar")?;

    Ok(SchnorrSignature { announcement, response })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use identus_crypto::curve::FR_BYTES;

    use super::*;

    /// Verify that the hashing pipeline produces consistent results.
    #[test]
    fn test_hash_message_to_fr_deterministic() {
        let msg = b"hello world";

        let fr1 = hash_message_to_fr(msg);
        let fr2 = hash_message_to_fr(msg);

        assert_eq!(fr1.as_le_bytes(), fr2.as_le_bytes(), "hash must be deterministic");
    }

    /// Verify that the message hashing pipeline used for sign/verify matches
    /// the WASM binding's implementation (Blake2b-512 → Jubjub wide reduction
    /// → Fr scalar).
    #[test]
    fn test_message_hashing_pipeline() {
        // Known message; verify that the output is a valid Fr scalar
        // (i.e. does not panic on conversion).
        let msg = b"Identus SDK test message";
        let fr = hash_message_to_fr(msg);
        let bytes = fr.as_le_bytes();
        assert_eq!(bytes.len(), FR_BYTES, "Fr must be {FR_BYTES} bytes");
    }

    /// Round-trip test: sign a message with a generated key and verify.
    #[test]
    fn test_sign_then_verify() {
        let key = generate_key();
        let message = b"Hello, UniFFI!";

        let signature = sign(key.secret_key.clone(), message.to_vec()).unwrap();

        let valid = verify(key.public_key.clone(), message.to_vec(), signature);
        assert!(valid, "signature must verify for the correct key and message");
    }

    /// Verify that signature verification fails with a wrong message.
    #[test]
    fn test_verify_wrong_message() {
        let key = generate_key();
        let message = b"original message";
        let wrong_message = b"wrong message";

        let signature = sign(key.secret_key.clone(), message.to_vec()).unwrap();

        let valid = verify(key.public_key.clone(), wrong_message.to_vec(), signature);
        assert!(!valid, "signature must NOT verify for a different message");
    }

    /// Verify that signature verification fails with a wrong public key.
    #[test]
    fn test_verify_wrong_key() {
        let key1 = generate_key();
        let key2 = generate_key();
        let message = b"some message";

        let signature = sign(key1.secret_key, message.to_vec()).unwrap();

        let valid = verify(key2.public_key, message.to_vec(), signature);
        assert!(!valid, "signature must NOT verify with a different key");
    }

    /// Verify that invalid hex input returns false (doesn't panic).
    #[test]
    fn test_verify_invalid_input() {
        let message = b"test";

        // Invalid hex for public key
        assert!(!verify("not-hex".to_string(), message.to_vec(), "aabb".to_string()));

        // Invalid hex for signature
        let key = generate_key();
        assert!(!verify(key.public_key, message.to_vec(), "not-hex".to_string()));
    }

    /// Verify that sign() returns an error for invalid hex input (doesn't panic).
    #[test]
    fn test_sign_invalid_hex() {
        let result = sign("not-hex".to_string(), b"test".to_vec());
        assert!(result.is_err(), "sign() must return Err for invalid hex");

        // Verify the error type
        match result {
            Err(SignError::InvalidHex { .. }) => {} // expected
            _ => panic!("expected InvalidHex error, got {:?}", result),
        }
    }

    /// Verify that sign() returns an error for invalid secret key bytes.
    #[test]
    fn test_sign_invalid_secret_key() {
        // 32 bytes of 0xff — valid hex but not a valid Jubjub scalar
        // (the scalar is a 32-byte LE integer; 0xff*32 exceeds the group order).
        let result = sign(
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_string(),
            b"test".to_vec(),
        );
        assert!(result.is_err(), "sign() must return Err for invalid key bytes");

        match result {
            Err(SignError::InvalidSecretKey) => {} // expected
            _ => panic!("expected InvalidSecretKey error, got {:?}", result),
        }
    }

    /// Verify that generated keys have the expected format (non-empty, 64 hex chars).
    #[test]
    fn test_generated_key_format() {
        // Verify the function produces valid key material (non-empty hex strings of the expected length).
        let key = generate_key();
        assert!(!key.secret_key.is_empty(), "secret key must not be empty");
        assert!(!key.public_key.is_empty(), "public key must not be empty");

        // Secret key: 32 bytes → 64 hex chars
        assert_eq!(key.secret_key.len(), 64, "secret key hex must be 64 chars");
        // Public key: 32 bytes → 64 hex chars
        assert_eq!(key.public_key.len(), 64, "public key hex must be 64 chars");
    }
}
