use std::collections::BTreeMap;
use std::hint::black_box;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use identus_core::IdentusError;
use identus_crypto::{
    Ed25519PrivateKey, EncodeJwk, JwkCurve, JwkKeyType, P256PrivateKey, PublicKeyJwk,
};
use identus_jose::{
    Ed25519SignatureSuite, Ed25519Signer, Es256SignatureSuite, Es256Signer, JoseError,
    JwsAlgorithm, JwsLimits, JwsSignatureSuite, JwsSigner, JwsSigningInput, JwsVerificationKey,
    LegacyEdDsaSignatureSuite, ProtectedHeader, SignatureSuiteRegistry, SignerFailure,
    UnverifiedCompactJws,
};
use serde_json::{Value, json};

const SAMPLE_PRIVATE: [u8; 32] = [
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
    0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20,
];

fn header(algorithm: &str) -> ProtectedHeader {
    ProtectedHeader::new(
        algorithm,
        Some("proof+jwt"),
        Some("did:example:holder#key-1"),
        JwsLimits::default(),
    )
    .expect("accepted header")
}

fn prepared(algorithm: &str) -> JwsSigningInput {
    JwsSigningInput::new(
        header(algorithm),
        br#"{"nonce":"fresh","aud":"https://issuer.example"}"#.to_vec(),
        JwsLimits::default(),
    )
    .expect("bounded signing input")
}

fn compact_from_raw(header: &[u8], payload: &[u8], signature: &[u8]) -> String {
    format!(
        "{}.{}.{}",
        URL_SAFE_NO_PAD.encode(header),
        URL_SAFE_NO_PAD.encode(payload),
        URL_SAFE_NO_PAD.encode(signature)
    )
}

#[test]
fn recommended_registry_verifies_fully_specified_ed25519() {
    let private = Ed25519PrivateKey::from_slice(&SAMPLE_PRIVATE).expect("Ed25519 private key");
    let public = private.to_public_key().encode_jwk();
    let compact = prepared("Ed25519")
        .sign_with(&Ed25519Signer::new(&private))
        .expect("software signature");
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).expect("bound key");
    let mut registry = SignatureSuiteRegistry::recommended();

    assert!(registry.contains(JwsAlgorithm::Ed25519));
    assert!(registry.contains(JwsAlgorithm::Es256));
    assert!(!registry.contains(JwsAlgorithm::LegacyEdDsa));
    let verified = registry.verify(&compact, &key).expect("verified JWS");
    assert_eq!(verified.algorithm(), JwsAlgorithm::Ed25519);
    assert_eq!(verified.as_compact().compact(), compact.compact());

    registry
        .register(LegacyEdDsaSignatureSuite)
        .expect("recommended registry remains explicitly extensible");
    assert!(registry.contains(JwsAlgorithm::LegacyEdDsa));
}

#[test]
fn rfc_8037_legacy_vector_requires_explicit_suite() {
    let compact = "eyJhbGciOiJFZERTQSJ9.RXhhbXBsZSBvZiBFZDI1NTE5IHNpZ25pbmc.hgyY0il_MGCjP0JzlnLWG1PPOt7-09PGcvMg3AIbQR6dWbhijcNR4ki4iylGjg5BhVsPt9g7sVvpAr_MuM0KAg";
    let parsed = UnverifiedCompactJws::parse(compact, JwsLimits::default()).expect("RFC JWS");
    let public: PublicKeyJwk = serde_json::from_str(
        r#"{"kty":"OKP","crv":"Ed25519","x":"11qYAYKxCrfVS_7TyWQHOg7hcvPapiMlrwIaaPcHURo"}"#,
    )
    .expect("RFC public JWK");
    let key = JwsVerificationKey::new(JwsAlgorithm::LegacyEdDsa, &public).expect("legacy binding");

    assert_eq!(
        SignatureSuiteRegistry::recommended().verify(&parsed, &key),
        Err(JoseError::AlgorithmNotAllowed)
    );
    let mut legacy = SignatureSuiteRegistry::new(1).expect("registry");
    legacy
        .register(LegacyEdDsaSignatureSuite)
        .expect("explicit legacy suite");
    assert!(legacy.verify(&parsed, &key).is_ok());
}

#[test]
fn es256_uses_raw_fixed_width_signature_and_rejects_der() {
    let private = P256PrivateKey::from_slice(&SAMPLE_PRIVATE).expect("P-256 private key");
    let public = private.to_public_key().encode_jwk();
    let input = prepared("ES256");
    let compact = input
        .sign_with(&Es256Signer::new(&private))
        .expect("raw ES256 signature");
    let key = JwsVerificationKey::new(JwsAlgorithm::Es256, &public).expect("bound key");
    let registry = SignatureSuiteRegistry::recommended();

    assert_eq!(compact.signature().len(), 64);
    assert!(registry.verify(&compact, &key).is_ok());

    let der = input
        .clone()
        .attach_signature(private.sign(input.as_bytes()))
        .expect("DER remains valid opaque compact bytes");
    assert_ne!(der.signature().len(), 64);
    assert_eq!(
        registry.verify(&der, &key),
        Err(JoseError::InvalidSignatureLength)
    );
}

#[test]
fn algorithm_and_key_confusion_fail_closed() {
    let ed_private = Ed25519PrivateKey::from_slice(&SAMPLE_PRIVATE).expect("Ed25519 key");
    let p256_private = P256PrivateKey::from_slice(&SAMPLE_PRIVATE).expect("P-256 key");
    let ed_public = ed_private.to_public_key().encode_jwk();
    let p256_public = p256_private.to_public_key().encode_jwk();

    assert!(matches!(
        JwsVerificationKey::new(JwsAlgorithm::Es256, &ed_public),
        Err(JoseError::InvalidVerificationKey)
    ));
    assert!(matches!(
        JwsVerificationKey::new(JwsAlgorithm::Ed25519, &p256_public),
        Err(JoseError::InvalidVerificationKey)
    ));

    let compact = prepared("Ed25519")
        .sign_with(&Ed25519Signer::new(&ed_private))
        .expect("signature");
    let legacy_key =
        JwsVerificationKey::new(JwsAlgorithm::LegacyEdDsa, &ed_public).expect("legacy key");
    assert_eq!(
        SignatureSuiteRegistry::recommended().verify(&compact, &legacy_key),
        Err(JoseError::AlgorithmMismatch)
    );
    assert_eq!(
        prepared("ES256").sign_with(&Ed25519Signer::new(&ed_private)),
        Err(JoseError::AlgorithmMismatch)
    );

    let unknown = JwsSigningInput::new(header("RS256"), b"payload".to_vec(), JwsLimits::default())
        .expect("codec accepts policy-neutral algorithm");
    assert_eq!(
        unknown.sign_with(&Ed25519Signer::new(&ed_private)),
        Err(JoseError::UnsupportedAlgorithm)
    );
}

#[test]
fn optional_jwk_algorithm_must_match_exactly() {
    let private = Ed25519PrivateKey::from_slice(&SAMPLE_PRIVATE).expect("Ed25519 key");
    let encoded = private.to_public_key().encode_jwk();
    let with_algorithm = |algorithm: Value| {
        PublicKeyJwk::from_parts(
            JwkKeyType::Okp,
            JwkCurve::Ed25519,
            encoded.x().as_str(),
            None,
            BTreeMap::from([("alg".to_owned(), algorithm)]),
        )
        .expect("structural JWK")
    };

    let exact = with_algorithm(json!("Ed25519"));
    assert!(JwsVerificationKey::new(JwsAlgorithm::Ed25519, &exact).is_ok());
    for invalid in [json!("EdDSA"), json!(7)] {
        assert!(matches!(
            JwsVerificationKey::new(JwsAlgorithm::Ed25519, &with_algorithm(invalid)),
            Err(JoseError::AlgorithmMismatch)
        ));
    }
}

#[test]
fn registry_capacity_and_duplicates_are_bounded() {
    for invalid in [0, 17, usize::MAX] {
        assert!(matches!(
            SignatureSuiteRegistry::new(invalid),
            Err(JoseError::InvalidRegistryCapacity)
        ));
    }
    let mut registry = SignatureSuiteRegistry::new(2).expect("bounded registry");
    assert!(registry.is_empty());
    registry
        .register(Ed25519SignatureSuite)
        .expect("first suite");
    assert_eq!(
        registry.register(Ed25519SignatureSuite),
        Err(JoseError::DuplicateAlgorithm)
    );
    registry
        .register(LegacyEdDsaSignatureSuite)
        .expect("second suite");
    assert_eq!(registry.len(), 2);
    assert_eq!(
        registry.register(Es256SignatureSuite),
        Err(JoseError::RegistryFull)
    );
}

struct StatefulAlgorithmSuite {
    algorithm_calls: Arc<AtomicUsize>,
}

impl JwsSignatureSuite for StatefulAlgorithmSuite {
    fn algorithm(&self) -> JwsAlgorithm {
        match self.algorithm_calls.fetch_add(1, Ordering::SeqCst) {
            0 => JwsAlgorithm::Ed25519,
            _ => JwsAlgorithm::Es256,
        }
    }

    fn verify(
        &self,
        _signing_input: &[u8],
        _signature: &[u8],
        _public_key: &PublicKeyJwk,
    ) -> Result<(), JoseError> {
        Ok(())
    }
}

#[test]
fn registry_captures_a_suite_algorithm_once_at_registration() {
    let calls = Arc::new(AtomicUsize::new(0));
    let mut registry = SignatureSuiteRegistry::new(1).expect("registry");
    registry
        .register(StatefulAlgorithmSuite {
            algorithm_calls: Arc::clone(&calls),
        })
        .expect("suite");

    let private = Ed25519PrivateKey::from_slice(&SAMPLE_PRIVATE).expect("Ed25519 key");
    let public = private.to_public_key().encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).expect("key");
    let compact = prepared("Ed25519")
        .attach_signature(vec![0x5a; 64])
        .expect("opaque signature");

    assert!(registry.contains(JwsAlgorithm::Ed25519));
    registry.verify(&compact, &key).expect("captured dispatch");
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

struct RecordingSigner {
    seen: Arc<Mutex<Vec<u8>>>,
    result: Result<[u8; 64], SignerFailure>,
}

impl JwsSigner for RecordingSigner {
    fn algorithm(&self) -> JwsAlgorithm {
        JwsAlgorithm::Ed25519
    }

    fn sign(&self, signing_input: &[u8]) -> Result<[u8; 64], SignerFailure> {
        *self.seen.lock().expect("recording lock") = signing_input.to_vec();
        self.result
    }
}

#[test]
fn external_signer_receives_exact_bytes_and_failures_are_static() {
    let input = prepared("Ed25519");
    let expected = input.as_bytes().to_vec();
    for (result, expected_error) in [
        (Err(SignerFailure::Rejected), JoseError::SigningRejected),
        (
            Err(SignerFailure::Unavailable),
            JoseError::SignerUnavailable,
        ),
    ] {
        let seen = Arc::new(Mutex::new(Vec::new()));
        let signer = RecordingSigner {
            seen: Arc::clone(&seen),
            result,
        };
        assert_eq!(input.sign_with(&signer), Err(expected_error));
        assert_eq!(*seen.lock().expect("recording lock"), expected);
    }

    let seen = Arc::new(Mutex::new(Vec::new()));
    let signer = RecordingSigner {
        seen: Arc::clone(&seen),
        result: Ok([0x5a; 64]),
    };
    let compact = input
        .sign_with(&signer)
        .expect("accepted external signature");
    assert_eq!(*seen.lock().expect("recording lock"), expected);
    assert_eq!(compact.signing_input(), expected);
}

#[test]
fn signing_preflight_avoids_external_work_when_fixed_output_cannot_fit() {
    let payload = br#"{"nonce":"fresh","aud":"https://issuer.example"}"#.to_vec();
    let cases = [
        (
            JwsLimits::new(65_536, 4_096, 49_152, 63, 2_048).expect("limits"),
            JoseError::SignatureTooLarge,
        ),
        {
            let signing_input_length = prepared("Ed25519").as_bytes().len();
            (
                JwsLimits::new(signing_input_length + 3, 4_096, 49_152, 64, 2_048).expect("limits"),
                JoseError::CompactTooLarge,
            )
        },
    ];

    for (limits, expected_error) in cases {
        let input = JwsSigningInput::new(header("Ed25519"), payload.clone(), limits)
            .expect("one-byte minimum still fits");
        let seen = Arc::new(Mutex::new(Vec::new()));
        let signer = RecordingSigner {
            seen: Arc::clone(&seen),
            result: Ok([0x5a; 64]),
        };

        assert_eq!(input.sign_with(&signer), Err(expected_error));
        assert!(seen.lock().expect("recording lock").is_empty());
    }
}

struct RecordingSuite {
    seen: Arc<Mutex<Vec<u8>>>,
}

impl JwsSignatureSuite for RecordingSuite {
    fn algorithm(&self) -> JwsAlgorithm {
        JwsAlgorithm::Ed25519
    }

    fn verify(
        &self,
        signing_input: &[u8],
        _signature: &[u8],
        _public_key: &PublicKeyJwk,
    ) -> Result<(), JoseError> {
        *self.seen.lock().expect("recording lock") = signing_input.to_vec();
        Ok(())
    }
}

#[test]
fn verifier_capability_receives_exact_parsed_signing_input() {
    let private = Ed25519PrivateKey::from_slice(&SAMPLE_PRIVATE).expect("Ed25519 key");
    let public = private.to_public_key().encode_jwk();
    let raw_header = br#"{ "kid":"did:example:holder#key-1", "alg":"Ed25519", "typ":"proof+jwt" }"#;
    let payload = br#"{"aud":"https://issuer.example"}"#;
    let received = compact_from_raw(raw_header, payload, &[0x5a; 64]);
    let reparsed =
        UnverifiedCompactJws::parse(&received, JwsLimits::default()).expect("parsed compact");
    let expected = reparsed.signing_input().to_vec();
    assert_eq!(
        expected,
        format!(
            "{}.{}",
            URL_SAFE_NO_PAD.encode(raw_header),
            URL_SAFE_NO_PAD.encode(payload)
        )
        .as_bytes()
    );
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).expect("key");
    let seen = Arc::new(Mutex::new(Vec::new()));
    let mut registry = SignatureSuiteRegistry::new(1).expect("registry");
    registry
        .register(RecordingSuite {
            seen: Arc::clone(&seen),
        })
        .expect("suite");

    registry.verify(&reparsed, &key).expect("accepted");
    assert_eq!(*seen.lock().expect("recording lock"), expected);
}

#[test]
fn malformed_points_tampering_and_lengths_are_rejected() {
    let invalid_point =
        PublicKeyJwk::new_ec(JwkCurve::P256, [0; 32], [0; 32]).expect("structurally valid JWK");
    let key = JwsVerificationKey::new(JwsAlgorithm::Es256, &invalid_point).expect("bound shape");
    let compact = prepared("ES256")
        .attach_signature(vec![0x5a; 64])
        .expect("opaque signature");
    assert_eq!(
        SignatureSuiteRegistry::recommended().verify(&compact, &key),
        Err(JoseError::InvalidVerificationKey)
    );

    let private = Ed25519PrivateKey::from_slice(&SAMPLE_PRIVATE).expect("Ed25519 key");
    let public = private.to_public_key().encode_jwk();
    let valid = prepared("Ed25519")
        .sign_with(&Ed25519Signer::new(&private))
        .expect("signature");
    let mut tampered_signature = valid.signature().to_vec();
    tampered_signature[0] ^= 1;
    let tampered = prepared("Ed25519")
        .attach_signature(tampered_signature)
        .expect("opaque tampered signature");
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).expect("key");
    assert_eq!(
        SignatureSuiteRegistry::recommended().verify(&tampered, &key),
        Err(JoseError::SignatureInvalid)
    );
}

#[test]
fn errors_and_debug_are_redaction_safe() {
    let private = Ed25519PrivateKey::from_slice(&SAMPLE_PRIVATE).expect("Ed25519 key");
    let public = private.to_public_key().encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).expect("key");
    let compact = prepared("Ed25519")
        .sign_with(&Ed25519Signer::new(&private))
        .expect("signature");
    let verified = SignatureSuiteRegistry::recommended()
        .verify(&compact, &key)
        .expect("verified");
    let key_debug = format!("{key:?}");
    let verified_debug = format!("{verified:?}");
    let canaries = [
        public.x().as_str(),
        "did:example:holder#key-1",
        "nonce",
        compact.compact(),
    ];
    for canary in canaries {
        assert!(!key_debug.contains(canary));
        assert!(!verified_debug.contains(canary));
    }

    for error in [
        JoseError::UnsupportedAlgorithm,
        JoseError::AlgorithmMismatch,
        JoseError::InvalidVerificationKey,
        JoseError::InvalidRegistryCapacity,
        JoseError::RegistryFull,
        JoseError::DuplicateAlgorithm,
        JoseError::AlgorithmNotAllowed,
        JoseError::InvalidSignatureLength,
        JoseError::SigningRejected,
        JoseError::SignerUnavailable,
        JoseError::SignatureInvalid,
    ] {
        let bridged: IdentusError = error.into();
        let rendered = format!("{error:?} {error} {bridged:?} {bridged}");
        for canary in canaries {
            assert!(!rendered.contains(canary));
        }
        assert!(bridged.code().as_str().starts_with("jose."));
    }
}

#[test]
fn capabilities_are_send_sync_and_algorithm_names_are_case_sensitive() {
    fn assert_send_sync<T: Send + Sync>() {}

    assert_send_sync::<SignatureSuiteRegistry>();
    assert_send_sync::<Ed25519SignatureSuite>();
    assert_send_sync::<Es256SignatureSuite>();
    assert_eq!(JwsAlgorithm::Ed25519.as_str(), "Ed25519");
    assert_eq!(JwsAlgorithm::Es256.as_str(), "ES256");
    assert_eq!(JwsAlgorithm::LegacyEdDsa.as_str(), "EdDSA");

    let private = Ed25519PrivateKey::from_slice(&SAMPLE_PRIVATE).expect("Ed25519 key");
    assert_eq!(
        prepared("ed25519").sign_with(&Ed25519Signer::new(&private)),
        Err(JoseError::UnsupportedAlgorithm)
    );
}

#[test]
#[ignore = "release-only diagnostic; informational, not a correctness threshold"]
fn verification_throughput_diagnostic() {
    let private = Ed25519PrivateKey::from_slice(&SAMPLE_PRIVATE).expect("Ed25519 key");
    let public = private.to_public_key().encode_jwk();
    let compact = prepared("Ed25519")
        .sign_with(&Ed25519Signer::new(&private))
        .expect("signature");
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).expect("key");
    let registry = SignatureSuiteRegistry::recommended();
    let iterations = 10_000_u32;
    let started = Instant::now();
    for _ in 0..iterations {
        black_box(
            registry
                .verify(black_box(&compact), black_box(&key))
                .expect("verify"),
        );
    }
    let elapsed = started.elapsed();
    let operations_per_second = f64::from(iterations) / elapsed.as_secs_f64();
    eprintln!(
        "jws Ed25519 verify: {iterations} iterations in {elapsed:?} ({operations_per_second:.0} ops/s)"
    );
}
