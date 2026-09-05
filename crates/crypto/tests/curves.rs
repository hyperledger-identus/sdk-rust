//! Curve lifecycle tests: Ed25519, X25519, secp256k1, P-256, plus JWK
//! roundtrips. Each `generate` injects the in-test deterministic
//! `SecureRandom`.

mod common;

use identus_crypto::ed25519::{Ed25519KeyPair, Ed25519PrivateKey, Ed25519PublicKey};
use identus_crypto::secp256k1::{Secp256k1KeyPair, Secp256k1PrivateKey, Secp256k1PublicKey};
use identus_crypto::secp256r1::{P256KeyPair, P256PrivateKey};
use identus_crypto::x25519::{X25519KeyPair, X25519PublicKey};
use identus_crypto::{
    EncodeArray, EncodeJwk, EncodeVec, Error, JwkCurve, JwkKeyType, PublicKeyJwk, SecureRandom,
    Verifiable,
};

use common::DetRandom;

const SAMPLE_32: [u8; 32] = [
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
    0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20,
];

struct FailingRandom;

impl SecureRandom for FailingRandom {
    fn fill_bytes(&mut self, _output: &mut [u8]) -> Result<(), Error> {
        Err(Error::SecureRandomFailure)
    }
}

#[derive(Default)]
struct ZeroRandom {
    calls: usize,
}

impl SecureRandom for ZeroRandom {
    fn fill_bytes(&mut self, output: &mut [u8]) -> Result<(), Error> {
        self.calls += 1;
        output.fill(0);
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Ed25519
// ---------------------------------------------------------------------------

#[test]
fn ed25519_generate_sign_verify_roundtrip() {
    let mut rng = DetRandom::new();
    let kp = Ed25519KeyPair::generate(&mut rng).unwrap();
    let message = b"ed25519 message";
    let signature = kp.sign(message);
    assert!(kp.public().verify(message, &signature));
    assert!(!kp.public().verify(b"tampered", &signature));
}

#[test]
fn ed25519_from_slice_invalid_size() {
    let err = Ed25519PublicKey::from_slice(&[0u8; 31]).unwrap_err();
    let identus = err.to_identus_error();
    assert_eq!(identus.code().as_str(), "crypto.invalid_key_size");
    assert!(!identus.to_string().contains("31"));
    assert!(!identus.to_string().contains("32"));
}

#[test]
fn ed25519_encode_array_and_vec() {
    let mut rng = DetRandom::new();
    let kp = Ed25519KeyPair::generate(&mut rng).unwrap();
    let arr: [u8; 32] = kp.public().encode_array();
    let vec = kp.public().encode_vec();
    assert_eq!(vec.as_slice(), arr);
    let recovered = Ed25519PublicKey::from_slice(&arr).unwrap();
    assert_eq!(recovered, *kp.public());
}

#[test]
fn ed25519_private_key_bytes_roundtrip() {
    let sk = Ed25519PrivateKey::from_slice(&SAMPLE_32).unwrap();
    let bytes = sk.to_bytes();
    let recovered = Ed25519PrivateKey::from_slice(&bytes).unwrap();
    assert_eq!(recovered.to_bytes(), bytes);
}

#[test]
fn ed25519_jwk() {
    let mut rng = DetRandom::new();
    let kp = Ed25519KeyPair::generate(&mut rng).unwrap();
    let jwk = kp.public().encode_jwk();
    assert_eq!(jwk.kty(), JwkKeyType::Okp);
    assert_eq!(jwk.crv(), JwkCurve::Ed25519);
    assert_eq!(jwk.x().to_bytes(), kp.public().encode_array());
    assert!(jwk.y().is_none());
}

// ---------------------------------------------------------------------------
// X25519
// ---------------------------------------------------------------------------

#[test]
fn x25519_dh_yields_shared_secret() {
    let mut rng1 = DetRandom::new();
    let mut rng2 = DetRandom::new();
    rng2.cursor = 64; // distinct keys
    let alice = X25519KeyPair::generate(&mut rng1).unwrap();
    let bob = X25519KeyPair::generate(&mut rng2).unwrap();
    let s1 = alice.derive_shared(bob.public());
    let s2 = bob.derive_shared(alice.public());
    assert_eq!(s1, s2);
}

#[test]
fn x25519_from_slice_rejects_wrong_length() {
    assert!(X25519PublicKey::from_slice(&[0u8; 31]).is_err());
    assert!(X25519PublicKey::from_slice(&[0u8; 33]).is_err());
    assert!(X25519PublicKey::from_slice(&[0xAB; 32]).is_ok());
}

#[test]
fn x25519_jwk() {
    let mut rng = DetRandom::new();
    let kp = X25519KeyPair::generate(&mut rng).unwrap();
    let jwk = kp.public().encode_jwk();
    assert_eq!(jwk.kty(), JwkKeyType::Okp);
    assert_eq!(jwk.crv(), JwkCurve::X25519);
    assert_eq!(jwk.x().to_bytes(), kp.public().encode_array());
    assert!(jwk.y().is_none());
}

#[test]
fn x25519_encode_array_roundtrip() {
    let pk = X25519PublicKey::from_slice(&SAMPLE_32).unwrap();
    let arr: [u8; 32] = pk.encode_array();
    assert_eq!(arr, SAMPLE_32);
}

// ---------------------------------------------------------------------------
// secp256k1
// ---------------------------------------------------------------------------

#[test]
fn secp256k1_generate_sign_verify_roundtrip() {
    let mut rng = DetRandom::new();
    let kp = Secp256k1KeyPair::generate(&mut rng).unwrap();
    let message = b"secp256k1 message";
    let signature = kp.sign(message);
    assert_eq!(signature[0], 0x30); // DER SEQUENCE tag
    assert!(kp.public().verify(message, &signature));
    assert!(!kp.public().verify(b"other", &signature));
}

#[test]
fn secp256k1_compressed_uncompressed_roundtrip() {
    let sk = Secp256k1PrivateKey::from_slice(&SAMPLE_32).unwrap();
    let pk = sk.to_public_key();
    let compressed: [u8; 33] = pk.encode_array();
    let uncompressed: [u8; 65] = pk.encode_array();
    assert_eq!(compressed[0], 0x02);
    assert_eq!(uncompressed[0], 0x04);
    let from_c = Secp256k1PublicKey::from_slice(&compressed).unwrap();
    let from_u = Secp256k1PublicKey::from_slice(&uncompressed).unwrap();
    assert_eq!(from_c, from_u);
    assert_eq!(from_c, pk);
}

#[test]
fn secp256k1_jwk() {
    let sk = Secp256k1PrivateKey::from_slice(&SAMPLE_32).unwrap();
    let jwk = sk.to_public_key().encode_jwk();
    assert_eq!(jwk.kty(), JwkKeyType::Ec);
    assert_eq!(jwk.crv(), JwkCurve::Secp256k1);
    let point = sk.to_public_key().curve_point();
    assert_eq!(jwk.x().to_bytes(), point.x);
    assert_eq!(jwk.y().expect("EC y").to_bytes(), point.y);
}

// ---------------------------------------------------------------------------
// P-256
// ---------------------------------------------------------------------------

#[test]
fn p256_generate_sign_verify_roundtrip() {
    let mut rng = DetRandom::new();
    let kp = P256KeyPair::generate(&mut rng).unwrap();
    let message = b"p256 message";
    let signature = kp.sign(message);
    assert_eq!(signature[0], 0x30); // DER SEQUENCE tag
    assert!(kp.public().verify(message, &signature));
    assert!(!kp.public().verify(b"other", &signature));
}

#[test]
fn p256_fixed_signature_preserves_der_api_and_rejects_tampering() {
    let private = P256PrivateKey::from_slice(&SAMPLE_32).expect("P-256 private key");
    let public = private.to_public_key();
    let message = b"fixed-width p256 message";
    let fixed = private.sign_fixed(message);
    let der = private.sign(message);

    assert_eq!(fixed.len(), 64);
    assert_eq!(der.first(), Some(&0x30));
    assert!(public.verify_fixed(message, &fixed));
    assert!(!public.verify_fixed(b"tampered", &fixed));
    assert!(public.verify(message, &der));
}

#[test]
fn p256_jwk() {
    let sk = P256PrivateKey::from_slice(&SAMPLE_32).unwrap();
    let jwk = sk.to_public_key().encode_jwk();
    assert_eq!(jwk.kty(), JwkKeyType::Ec);
    assert_eq!(jwk.crv(), JwkCurve::P256);
    let (x, y) = sk.to_public_key().curve_point();
    assert_eq!(jwk.x().to_bytes(), x);
    assert_eq!(jwk.y().expect("EC y").to_bytes(), y);
}

// ---------------------------------------------------------------------------
// JWK shape shared checks
// ---------------------------------------------------------------------------

#[test]
fn jwk_is_cloneable_and_eq() {
    let mut rng = DetRandom::new();
    let jwk1 = Ed25519KeyPair::generate(&mut rng)
        .unwrap()
        .public()
        .encode_jwk();
    let jwk2: PublicKeyJwk = jwk1.clone();
    assert_eq!(jwk1, jwk2);
}

#[test]
fn key_generation_propagates_entropy_failure() {
    assert!(matches!(
        Ed25519KeyPair::generate(&mut FailingRandom),
        Err(Error::SecureRandomFailure)
    ));
    assert!(matches!(
        X25519KeyPair::generate(&mut FailingRandom),
        Err(Error::SecureRandomFailure)
    ));
    assert!(matches!(
        Secp256k1KeyPair::generate(&mut FailingRandom),
        Err(Error::SecureRandomFailure)
    ));
    assert!(matches!(
        P256KeyPair::generate(&mut FailingRandom),
        Err(Error::SecureRandomFailure)
    ));
}

#[test]
fn ec_key_generation_bounds_invalid_scalar_retries() {
    let mut secp_rng = ZeroRandom::default();
    assert!(matches!(
        Secp256k1KeyPair::generate(&mut secp_rng),
        Err(Error::SecureRandomFailure)
    ));
    assert_eq!(secp_rng.calls, 16);

    let mut p256_rng = ZeroRandom::default();
    assert!(matches!(
        P256KeyPair::generate(&mut p256_rng),
        Err(Error::SecureRandomFailure)
    ));
    assert_eq!(p256_rng.calls, 16);
}
