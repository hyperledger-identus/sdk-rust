use identus_crypto::curve::{EmbeddedFr, EmbeddedGroupAffine, FR_BYTES, Fr};
use identus_crypto::schnorr::{SchnorrSignature, sign, verify, vk};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Deterministic RNG seeded with a known value for reproducible tests.
fn test_rng() -> ChaCha8Rng {
    ChaCha8Rng::seed_from_u64(42)
}

/// Verify that exported curve types exist and have the expected sizes.
#[test]
fn test_curve_types_exported() {
    // These compile-checks verify the re-exports (AC #3).
    let _fr: Fr = test_rng().r#gen();
    let _sk: EmbeddedFr = test_rng().r#gen();
    let _pk: EmbeddedGroupAffine = test_rng().r#gen();

    // FR_BYTES must be positive and sensible (JubJub scalar is ~252 bits = 32 bytes).
    const _: () = assert!(FR_BYTES > 0);
    const _: () = assert!(FR_BYTES <= 64, "FR_BYTES should be at most 64");
}

/// Verify that Schnorr types and functions are exported (AC #4).
#[test]
fn test_schnorr_types_exported() {
    let _sig = SchnorrSignature {
        announcement: test_rng().r#gen(),
        response: test_rng().r#gen(),
    };
    // sign, verify, vk are function items; they will be exercised in round-trip tests.
}

/// AC #5: Sign/verify round-trip works.
#[test]
fn test_schnorr_roundtrip() {
    let mut rng = test_rng();

    // Generate a signing key.
    let sk: EmbeddedFr = rng.r#gen();

    // Compute the verifying key.
    let pk = vk(sk);

    // Create a message consisting of field elements.
    let msg = vec![rng.r#gen::<Fr>(), rng.r#gen::<Fr>(), rng.r#gen::<Fr>()];

    // Sign the message.
    let sig = sign(&mut rng, sk, &msg);

    // Verify the signature with the correct key and message.
    assert!(verify(pk, &msg, &sig), "sign/verify round-trip should succeed");
}

/// AC #6: verify() correctly rejects a signature made with a different signing key.
#[test]
fn test_wrong_key_rejected() {
    let mut rng = test_rng();

    let sk1: EmbeddedFr = rng.r#gen();
    let sk2: EmbeddedFr = rng.r#gen();
    let pk1 = vk(sk1);
    let pk2 = vk(sk2);

    // Ensure the keys are actually different.
    assert_ne!(pk1, pk2, "randomly generated keys should be different");

    let msg = vec![rng.r#gen::<Fr>(), rng.r#gen::<Fr>()];

    // Sign with sk1.
    let sig = sign(&mut rng, sk1, &msg);

    // Verify with pk2 — should fail.
    assert!(
        !verify(pk2, &msg, &sig),
        "signature made with sk1 should NOT verify with pk2"
    );
}

/// Verify that a signature made with a different message fails.
#[test]
fn test_wrong_message_rejected() {
    let mut rng = test_rng();

    let sk: EmbeddedFr = rng.r#gen();
    let pk = vk(sk);

    let msg1 = vec![rng.r#gen::<Fr>()];
    let msg2 = vec![rng.r#gen::<Fr>()];

    let sig = sign(&mut rng, sk, &msg1);

    // Verify with a different message — should fail.
    assert!(
        !verify(pk, &msg2, &sig),
        "signature for msg1 should NOT verify with msg2"
    );
}

/// Edge case: empty message slice.
#[test]
fn test_empty_message() {
    let mut rng = test_rng();

    let sk: EmbeddedFr = rng.r#gen();
    let pk = vk(sk);
    let msg: &[Fr] = &[];

    let sig = sign(&mut rng, sk, msg);

    // Should verify with the same (empty) message.
    assert!(verify(pk, msg, &sig), "sign/verify with empty message should succeed");
}

/// Edge case: sign with zero signing key.
/// A zero signing key produces the identity verifying key,
/// which verify() should reject (it checks pk.is_identity()).
#[test]
fn test_zero_key_rejected() {
    let mut rng = test_rng();

    // Zero signing key — EmbeddedFr::from(0u64) is the additive identity.
    let sk = EmbeddedFr::from(0u64);
    let pk = vk(sk);

    // The verifying key should be the identity element.
    assert!(pk.is_identity(), "zero key should produce identity verifying key");

    let msg = vec![rng.r#gen::<Fr>()];
    let sig = sign(&mut rng, sk, &msg);

    // verify() returns false when the public key is the identity element.
    assert!(
        !verify(pk, &msg, &sig),
        "signature with zero signing key should NOT verify"
    );
}
