use blake2b_simd::Params as Blake2b;
use group::GroupEncoding;
use identus_crypto::curve::{EmbeddedFr, EmbeddedGroupAffine, Fr};
use identus_crypto::schnorr::{SchnorrSignature, vk};
use midnight_transient_crypto::curve::embedded;
use rand::Rng;
use rand::rngs::OsRng;
use wasm_bindgen::prelude::*;

// ---------------------------------------------------------------------------
// Return type for generate_key()
// ---------------------------------------------------------------------------

/// A generated keypair returned to JavaScript as an object with
/// `secret_key_hex` and `public_key_hex` string properties.
#[wasm_bindgen]
pub struct GeneratedKey {
    secret_key_hex: String,
    public_key_hex: String,
}

#[wasm_bindgen]
impl GeneratedKey {
    /// Hex-encoded 32-byte LE Jubjub secret (signing) key.
    #[wasm_bindgen(getter)]
    pub fn secret_key_hex(&self) -> String {
        self.secret_key_hex.clone()
    }

    /// Hex-encoded 32-byte compressed public (verifying) key.
    #[wasm_bindgen(getter)]
    pub fn public_key_hex(&self) -> String {
        self.public_key_hex.clone()
    }
}

// ---------------------------------------------------------------------------
// WASM-exported functions
// ---------------------------------------------------------------------------

/// Generate a random Schnorr keypair over the Jubjub embedded curve.
///
/// Returns a `GeneratedKey` object with `{ secret_key_hex, public_key_hex }`.
#[wasm_bindgen]
pub fn generate_key() -> GeneratedKey {
    let mut rng = OsRng;

    // Generate random secret key (Jubjub scalar).
    let sk: EmbeddedFr = rng.r#gen();

    // Derive the corresponding verifying key.
    let pk = vk(sk);

    // Encode as hex.
    let secret_key_hex = const_hex::encode(sk.as_le_bytes());
    let public_key_hex = const_hex::encode(pk.0.to_bytes());

    GeneratedKey {
        secret_key_hex,
        public_key_hex,
    }
}

/// Sign a message with the given secret key.
///
/// * `message` – raw bytes (will be hashed with Blake2b-512 internally).
/// * `secret_key_hex` – hex-encoded 32-byte LE Jubjub secret key.
///
/// Returns a hex-encoded 64-byte Schnorr signature
/// (32-byte compressed announcement ∥ 32-byte LE response).
#[wasm_bindgen(js_name = sign)]
pub fn schnorr_sign(message: &[u8], secret_key_hex: &str) -> String {
    use identus_crypto::schnorr::sign as schnorr_sign_inner;

    let mut rng = OsRng;

    // --- Parse secret key --------------------------------------------------
    let sk_bytes = const_hex::decode(secret_key_hex).expect("sign: invalid hex in secret_key_hex");
    let sk = EmbeddedFr::from_le_bytes(&sk_bytes).expect("sign: invalid secret key bytes");

    // --- Hash message to a scalar ------------------------------------------
    let msg_fr = hash_message_to_fr(message);

    // --- Sign --------------------------------------------------------------
    let sig = schnorr_sign_inner(&mut rng, sk, &[msg_fr]);

    // --- Encode signature as hex -------------------------------------------
    encode_signature(&sig)
}

/// Verify a Schnorr signature.
///
/// * `message` – raw bytes (hashed the same way as in `schnorr_sign`).
/// * `signature_hex` – hex-encoded 64-byte Schnorr signature.
/// * `public_key_hex` – hex-encoded 32-byte compressed public key.
///
/// Returns `true` if the signature is valid, `false` otherwise.
#[wasm_bindgen(js_name = verify)]
pub fn schnorr_verify(message: &[u8], signature_hex: &str, public_key_hex: &str) -> bool {
    use identus_crypto::schnorr::verify as schnorr_verify_inner;

    // --- Parse public key --------------------------------------------------
    let pk = match decode_public_key(public_key_hex) {
        Ok(pk) => pk,
        Err(_) => return false,
    };

    // --- Parse signature ---------------------------------------------------
    let sig = match decode_signature(signature_hex) {
        Ok(s) => s,
        Err(_) => return false,
    };

    // --- Hash message to a scalar ------------------------------------------
    let msg_fr = hash_message_to_fr(message);

    // --- Verify ------------------------------------------------------------
    schnorr_verify_inner(pk, &[msg_fr], &sig)
}

// ---------------------------------------------------------------------------
// Internal helpers
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
