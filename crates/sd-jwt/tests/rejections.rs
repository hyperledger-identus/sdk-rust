use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use identus_crypto::{Ed25519PrivateKey, EncodeJwk, PublicKeyJwk, SecureRandom};
use identus_jose::{
    Ed25519Signer, JwsAlgorithm, JwsLimits, JwsSigningInput, JwsVerificationKey, ProtectedHeader,
    SignatureSuiteRegistry, UnverifiedCompactJws,
};
use identus_sd_jwt::{
    ClaimPath, DisclosureHasher, IssuancePlan, PathSegment, SdJwtError, SdJwtLimits,
    UnverifiedSdJwt, issue, issue_with_hasher,
};
use serde_json::json;

struct DeterministicEntropy(u8);
impl SecureRandom for DeterministicEntropy {
    fn fill_bytes(&mut self, output: &mut [u8]) -> Result<(), identus_crypto::Error> {
        for byte in output {
            *byte = self.0;
            self.0 = self.0.wrapping_add(1);
        }
        Ok(())
    }
}

struct CollidingHasher;
impl DisclosureHasher for CollidingHasher {
    fn hash(&self, _bytes: &[u8]) -> [u8; 32] {
        [0; 32]
    }
}

struct RepeatingEntropy;
impl SecureRandom for RepeatingEntropy {
    fn fill_bytes(&mut self, output: &mut [u8]) -> Result<(), identus_crypto::Error> {
        output.fill(7);
        Ok(())
    }
}

fn signed(payload: &[u8]) -> (String, Ed25519PrivateKey) {
    let private = Ed25519PrivateKey::from_slice(&[7; 32]).unwrap();
    let limits = JwsLimits::new(524_288, 4_096, 524_288, 1_024, 2_048).unwrap();
    let header = ProtectedHeader::new("Ed25519", None, None, limits).unwrap();
    let compact = JwsSigningInput::new(header, payload.to_vec(), limits)
        .unwrap()
        .sign_with(&Ed25519Signer::new(&private))
        .unwrap()
        .compact()
        .to_owned();
    (compact, private)
}

fn process(
    compact: &str,
    private: &Ed25519PrivateKey,
    limits: SdJwtLimits,
) -> Result<(), SdJwtError> {
    let public = private.to_public_key().encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).unwrap();
    UnverifiedSdJwt::parse(compact, limits)?
        .verify_issuer(&SignatureSuiteRegistry::recommended(), &key)?
        .process_disclosures()?;
    Ok(())
}

#[test]
fn duplicate_json_and_unused_disclosure_fail_closed() {
    let (issuer, private) = signed(br#"{"_sd_alg":"sha-256","private":"one","private":"two"}"#);
    assert_eq!(
        process(&format!("{issuer}~"), &private, SdJwtLimits::default()),
        Err(SdJwtError::Json)
    );
    let (issuer, private) = signed(br#"{"_sd_alg":"sha-256","public":true}"#);
    let disclosure = URL_SAFE_NO_PAD.encode(br#"["AAAAAAAAAAAAAAAAAAAAAA","private","secret"]"#);
    let combined = format!("{issuer}~{disclosure}~");
    assert_eq!(
        process(&combined, &private, SdJwtLimits::default()),
        Err(SdJwtError::UnusedDisclosure)
    );
    let duplicated = format!("{issuer}~{disclosure}~{disclosure}~");
    assert_eq!(
        process(&duplicated, &private, SdJwtLimits::default()),
        Err(SdJwtError::DuplicateDisclosure)
    );
}

#[test]
fn resource_and_error_surface_are_static() {
    let (issuer, private) = signed(br#"{"_sd_alg":"sha-256","public":true}"#);
    let limits = SdJwtLimits {
        json_nodes: 1,
        ..SdJwtLimits::default()
    };
    assert_eq!(
        process(&format!("{issuer}~"), &private, limits),
        Err(SdJwtError::ResourceLimit)
    );
    assert!(matches!(
        UnverifiedSdJwt::parse(&"x".repeat(1_048_577), SdJwtLimits::default()),
        Err(SdJwtError::ResourceLimit)
    ));
    for error in [
        SdJwtError::Json,
        SdJwtError::UnusedDisclosure,
        SdJwtError::KeyBindingHash,
    ] {
        let rendered = format!("{error} {error:?}");
        assert!(rendered.contains("sd_jwt."));
        assert!(!rendered.contains("private"));
        assert!(!rendered.contains("secret"));
    }
}

#[test]
fn retained_issuer_verification_key_is_charged() {
    let (issuer, private) = signed(br#"{"_sd_alg":"sha-256","public":true}"#);
    let public = private.to_public_key().encode_jwk();
    let key_bytes = serde_json::to_vec(&public).unwrap().len();
    let claims_bytes = serde_json::to_vec(&json!({"public":true})).unwrap().len();
    let jws_limits = JwsLimits::new(524_288, 4_096, 524_288, 1_024, 2_048).unwrap();
    let header_bytes = serde_json::to_vec(
        UnverifiedCompactJws::parse(&issuer, jws_limits)
            .unwrap()
            .protected_header(),
    )
    .unwrap()
    .len();
    let retained = issuer.len() * 2 + 1 + claims_bytes + key_bytes + header_bytes;
    let limits = SdJwtLimits {
        retained_output_bytes: retained,
        ..SdJwtLimits::default()
    };
    let compact = format!("{issuer}~");
    let normal = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).unwrap();
    assert!(
        UnverifiedSdJwt::parse(&compact, limits)
            .unwrap()
            .verify_issuer(&SignatureSuiteRegistry::recommended(), &normal)
            .unwrap()
            .process_disclosures()
            .is_ok()
    );

    let insufficient = SdJwtLimits {
        retained_output_bytes: retained - 1,
        ..SdJwtLimits::default()
    };
    assert!(matches!(
        UnverifiedSdJwt::parse(&compact, insufficient)
            .unwrap()
            .verify_issuer(&SignatureSuiteRegistry::recommended(), &normal)
            .unwrap()
            .process_disclosures(),
        Err(SdJwtError::ResourceLimit)
    ));

    let mut extended = serde_json::to_value(public).unwrap();
    extended
        .as_object_mut()
        .unwrap()
        .insert("label".into(), json!("x".repeat(128)));
    let extended: PublicKeyJwk = serde_json::from_value(extended).unwrap();
    let extended = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &extended).unwrap();
    assert!(matches!(
        UnverifiedSdJwt::parse(&compact, limits)
            .unwrap()
            .verify_issuer(&SignatureSuiteRegistry::recommended(), &extended)
            .unwrap()
            .process_disclosures(),
        Err(SdJwtError::ResourceLimit)
    ));
}

#[test]
fn injected_digest_collision_is_rejected() {
    let limits = SdJwtLimits::default();
    let path =
        |name: &str| ClaimPath::new(vec![PathSegment::Member(name.to_owned())], limits).unwrap();
    let private = Ed25519PrivateKey::from_slice(&[7; 32]).unwrap();
    let result = issue_with_hasher(
        json!({"one":1,"two":2}),
        &IssuancePlan {
            disclose: vec![path("one"), path("two")],
            decoy_locations: Vec::new(),
            salt_bytes: 16,
        },
        &mut DeterministicEntropy(1),
        &Ed25519Signer::new(&private),
        limits,
        &CollidingHasher,
    );
    assert!(matches!(result, Err(SdJwtError::DigestCollision)));
}

#[test]
fn issuance_rejects_reused_salts_even_when_disclosures_do_not_collide() {
    let limits = SdJwtLimits::default();
    let path =
        |name: &str| ClaimPath::new(vec![PathSegment::Member(name.to_owned())], limits).unwrap();
    let private = Ed25519PrivateKey::from_slice(&[7; 32]).unwrap();
    let result = identus_sd_jwt::issue(
        json!({"one":1,"two":2}),
        &IssuancePlan {
            disclose: vec![path("one"), path("two")],
            decoy_locations: Vec::new(),
            salt_bytes: 16,
        },
        &mut RepeatingEntropy,
        &Ed25519Signer::new(&private),
        limits,
    );
    assert!(matches!(result, Err(SdJwtError::DuplicateDisclosure)));
}

#[test]
fn verifier_accepts_rfc_salt_strings_that_are_not_base64url() {
    let disclosure = URL_SAFE_NO_PAD.encode(br#"["not+base64/salt!!","private","secret"]"#);
    let digest = URL_SAFE_NO_PAD.encode(identus_crypto::sha256(disclosure.as_bytes()).as_bytes());
    let payload = format!(r#"{{"_sd_alg":"sha-256","_sd":["{digest}"]}}"#);
    let (issuer, private) = signed(payload.as_bytes());
    let combined = format!("{issuer}~{disclosure}~");
    assert_eq!(process(&combined, &private, SdJwtLimits::default()), Ok(()));
}

#[test]
fn maximum_configured_issuer_salt_round_trips() {
    let limits = SdJwtLimits::default();
    let private = Ed25519PrivateKey::from_slice(&[7; 32]).unwrap();
    let issued = issue(
        json!({"private":"secret"}),
        &IssuancePlan {
            disclose: vec![
                ClaimPath::new(vec![PathSegment::Member("private".to_owned())], limits).unwrap(),
            ],
            decoy_locations: Vec::new(),
            salt_bytes: limits.salt_max_bytes,
        },
        &mut DeterministicEntropy(1),
        &Ed25519Signer::new(&private),
        limits,
    )
    .unwrap();
    assert_eq!(process(issued.as_str(), &private, limits), Ok(()));
}

#[test]
fn resolved_graph_enforces_aggregate_member_limit() {
    let first = URL_SAFE_NO_PAD.encode(br#"["sixteen-byte-salt","one",{"a":1,"b":2}]"#);
    let second = URL_SAFE_NO_PAD.encode(br#"["another-16-bytes","two",{"c":3,"d":4}]"#);
    let first_digest = URL_SAFE_NO_PAD.encode(identus_crypto::sha256(first.as_bytes()).as_bytes());
    let second_digest =
        URL_SAFE_NO_PAD.encode(identus_crypto::sha256(second.as_bytes()).as_bytes());
    let payload = format!(r#"{{"_sd_alg":"sha-256","_sd":["{first_digest}","{second_digest}"]}}"#);
    let (issuer, private) = signed(payload.as_bytes());
    let compact = format!("{issuer}~{first}~{second}~");
    let limits = SdJwtLimits {
        json_members: 4,
        ..SdJwtLimits::default()
    };
    assert_eq!(
        process(&compact, &private, limits),
        Err(SdJwtError::ResourceLimit)
    );
}

#[test]
fn issuance_rechecks_json_limits_after_sd_transformation() {
    let limits = SdJwtLimits {
        json_members: 1,
        ..SdJwtLimits::default()
    };
    let private = Ed25519PrivateKey::from_slice(&[7; 32]).unwrap();
    let result = issue(
        json!({"public":true}),
        &IssuancePlan {
            disclose: Vec::new(),
            decoy_locations: Vec::new(),
            salt_bytes: 16,
        },
        &mut DeterministicEntropy(1),
        &Ed25519Signer::new(&private),
        limits,
    );
    assert!(matches!(result, Err(SdJwtError::ResourceLimit)));
}

#[test]
fn error_codes_are_stable_and_redacted() {
    let golden = [
        (SdJwtError::InvalidLimits, "sd_jwt.invalid_limits"),
        (SdJwtError::ResourceLimit, "sd_jwt.resource_limit"),
        (SdJwtError::CompactStructure, "sd_jwt.invalid_compact"),
        (SdJwtError::Encoding, "sd_jwt.encoding"),
        (SdJwtError::Json, "sd_jwt.json"),
        (SdJwtError::ReservedClaim, "sd_jwt.reserved_claim"),
        (
            SdJwtError::UnsupportedHashAlgorithm,
            "sd_jwt.unsupported_hash_algorithm",
        ),
        (SdJwtError::DisclosureShape, "sd_jwt.disclosure_shape"),
        (
            SdJwtError::DuplicateDisclosure,
            "sd_jwt.duplicate_disclosure",
        ),
        (SdJwtError::DigestCollision, "sd_jwt.digest_collision"),
        (SdJwtError::AmbiguousReference, "sd_jwt.ambiguous_reference"),
        (SdJwtError::UnusedDisclosure, "sd_jwt.unused_disclosure"),
        (SdJwtError::InvalidSelection, "sd_jwt.invalid_selection"),
        (SdJwtError::IssuerSignature, "sd_jwt.issuer_signature"),
        (
            SdJwtError::KeyBindingStructure,
            "sd_jwt.key_binding_structure",
        ),
        (
            SdJwtError::KeyBindingSignature,
            "sd_jwt.key_binding_signature",
        ),
        (SdJwtError::KeyBindingType, "sd_jwt.key_binding_type"),
        (SdJwtError::KeyBindingHash, "sd_jwt.key_binding_hash"),
        (
            SdJwtError::KeyBindingAudience,
            "sd_jwt.key_binding_audience",
        ),
        (SdJwtError::KeyBindingNonce, "sd_jwt.key_binding_nonce"),
        (SdJwtError::KeyBindingTime, "sd_jwt.key_binding_time"),
        (SdJwtError::EffectRejected, "sd_jwt.effect_rejected"),
        (SdJwtError::EffectUnavailable, "sd_jwt.effect_unavailable"),
        (SdJwtError::InternalInvariant, "sd_jwt.internal_invariant"),
    ];
    for (error, code) in golden {
        assert_eq!(error.code(), code);
        assert_eq!(error.to_string(), code);
    }
}
