use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use identus_crypto::P256PrivateKey;
use identus_jose::{
    CompactJwe, JweContentEncryptionAlgorithm, JweEncryptionError, JweEncryptionRequest,
    JweKeyManagementAlgorithm, JweLimits, JweRecipientPublicKey,
};

fn valid_compact() -> String {
    let point = P256PrivateKey::from_slice(&[9; 32])
        .expect("private key")
        .to_public_key()
        .encode_uncompressed();
    let header = serde_json::json!({
        "alg": "ECDH-ES",
        "enc": "A128GCM",
        "epk": {
            "kty": "EC",
            "crv": "P-256",
            "x": URL_SAFE_NO_PAD.encode(&point[1..33]),
            "y": URL_SAFE_NO_PAD.encode(&point[33..65])
        }
    });
    format!(
        "{}..{}.{}.{}",
        URL_SAFE_NO_PAD.encode(serde_json::to_vec(&header).expect("header")),
        URL_SAFE_NO_PAD.encode([1_u8; 12]),
        URL_SAFE_NO_PAD.encode([2_u8; 4]),
        URL_SAFE_NO_PAD.encode([3_u8; 16])
    )
}

fn recipient(limits: JweLimits) -> JweRecipientPublicKey {
    let key = P256PrivateKey::from_slice(&[7; 32])
        .expect("private key")
        .to_public_key()
        .encode_uncompressed();
    JweRecipientPublicKey::p256(&key, Some("recipient-key"), limits).expect("recipient")
}

#[test]
fn request_is_closed_bounded_and_redacted() {
    let limits = JweLimits::new(16, 64, 16).expect("limits");
    let request = JweEncryptionRequest::new(
        recipient(limits),
        JweKeyManagementAlgorithm::EcdhEs,
        JweContentEncryptionAlgorithm::A128Gcm,
        b"secret payload",
        limits,
    )
    .expect("request");

    assert_eq!(request.key_management_algorithm().as_str(), "ECDH-ES");
    assert_eq!(request.content_encryption_algorithm().as_str(), "A128GCM");
    assert_eq!(request.recipient().expose_key_id(), Some("recipient-key"));
    assert_eq!(request.expose_sensitive_plaintext(), b"secret payload");
    let debug = format!("{request:?}");
    assert!(!debug.contains("secret payload"));
    assert!(!debug.contains("recipient-key"));
}

#[test]
fn bounded_request_matrix_preserves_only_explicitly_revealed_values() {
    for key_id_len in 1..=16 {
        let limits = JweLimits::new(32, 1_024, key_id_len).expect("limits");
        let key_id = "Z".repeat(key_id_len);
        let key = P256PrivateKey::from_slice(&[7; 32])
            .expect("private key")
            .to_public_key()
            .encode_uncompressed();
        let recipient =
            JweRecipientPublicKey::p256(&key, Some(&key_id), limits).expect("bounded recipient");
        for plaintext_len in [1, 2, 8, 16, 32] {
            let plaintext = vec![0x5a; plaintext_len];
            let request = JweEncryptionRequest::new(
                recipient_for_matrix(&key, &key_id, limits),
                JweKeyManagementAlgorithm::EcdhEs,
                JweContentEncryptionAlgorithm::A128Gcm,
                &plaintext,
                limits,
            )
            .expect("bounded request");
            assert_eq!(request.recipient().expose_key_id(), Some(key_id.as_str()));
            assert_eq!(request.expose_sensitive_plaintext(), plaintext);
            assert!(!format!("{request:?}").contains(&key_id));
        }
        assert_eq!(recipient.expose_key_id(), Some(key_id.as_str()));
    }
}

fn recipient_for_matrix(key: &[u8; 65], key_id: &str, limits: JweLimits) -> JweRecipientPublicKey {
    JweRecipientPublicKey::p256(key, Some(key_id), limits).expect("recipient")
}

#[test]
fn recipient_rejects_compressed_invalid_and_unbounded_material() {
    let limits = JweLimits::new(16, 64, 4).expect("limits");
    assert_eq!(
        JweRecipientPublicKey::p256(&[2; 33], None, limits).unwrap_err(),
        JweEncryptionError::InvalidRecipientKey
    );
    assert_eq!(
        JweRecipientPublicKey::p256(&[4; 65], None, limits).unwrap_err(),
        JweEncryptionError::InvalidRecipientKey
    );
    let key = P256PrivateKey::from_slice(&[7; 32])
        .expect("private key")
        .to_public_key()
        .encode_uncompressed();
    assert_eq!(
        JweRecipientPublicKey::p256(&key, Some("large"), limits).unwrap_err(),
        JweEncryptionError::InvalidKeyId
    );
}

#[test]
fn compact_direct_key_agreement_grammar_is_exact() {
    let compact = valid_compact();
    let valid = CompactJwe::new(compact.clone(), 1_024).expect("compact");
    assert_eq!(valid.len(), compact.len());
    assert!(!format!("{valid:?}").contains(&compact));

    for invalid in [
        "a.b.c.d.e",
        "a...d.e",
        "a..c..e",
        "a..c.d.",
        "a..c.d.e.f",
        "a=..c.d.e",
    ] {
        assert_eq!(
            CompactJwe::new(invalid.to_owned(), 64).unwrap_err(),
            JweEncryptionError::InvalidCompactStructure
        );
    }
    assert_eq!(
        CompactJwe::new("a..c.d.e".to_owned(), 4).unwrap_err(),
        JweEncryptionError::CompactTooLarge
    );
}

#[test]
fn failures_are_static_and_redacted() {
    for error in [
        JweEncryptionError::InvalidLimits,
        JweEncryptionError::InvalidRecipientKey,
        JweEncryptionError::InvalidKeyId,
        JweEncryptionError::PlaintextTooLarge,
        JweEncryptionError::CompactTooLarge,
        JweEncryptionError::InvalidCompactStructure,
        JweEncryptionError::EncryptionRejected,
        JweEncryptionError::EncrypterUnavailable,
    ] {
        let error = error.to_identus_error();
        assert!(error.code().as_str().starts_with("jose."));
        assert!(!error.to_string().contains("secret"));
    }
}

#[test]
fn limits_cannot_admit_key_ids_rejected_by_compact_validation() {
    assert_eq!(
        JweLimits::new(1, 1, identus_jose::DEFAULT_MAX_JWE_KEY_ID_BYTES + 1).unwrap_err(),
        JweEncryptionError::InvalidLimits
    );
}
