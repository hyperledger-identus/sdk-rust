use identus_crypto::{Ed25519PrivateKey, EncodeJwk, PublicKeyJwk, SecureRandom};
use identus_jose::{
    Ed25519Signer, JwsAlgorithm, JwsKeyReference, JwsLimits, JwsSigningInput, JwsVerificationKey,
    ProtectedHeader, SignatureSuiteRegistry,
};
use identus_sd_jwt::{
    ClaimPath, IssuanceOptions, IssuancePlan, PathSegment, SdJwtLimits, issue_with_options,
};
use identus_sd_jwt_vc::{
    DRAFT_DATE, DRAFT_NAME, DRAFT_SHA256, DRAFT_SOURCE, Draft22SdJwtVc, ENVELOPE_FORMAT,
    IssuerEvidence, IssuerMetadataEvidence, ProfileValidationInputs, SdJwtVcError, SdJwtVcLimits,
    issue_draft22, parse_draft22_envelope,
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

#[test]
fn canonical_profile_round_trips_with_exact_identity() {
    assert_eq!(DRAFT_NAME, "draft-ietf-oauth-sd-jwt-vc-22");
    assert_eq!(DRAFT_DATE, "2026-10-27");
    assert_eq!(
        DRAFT_SOURCE,
        "https://www.ietf.org/archive/id/draft-ietf-oauth-sd-jwt-vc-22.txt"
    );
    assert_eq!(
        DRAFT_SHA256,
        "da2ade6128e9088e28e482866aa744f8287ac7eb6f11a5713bb2166827299124"
    );

    let base_limits = SdJwtLimits::default();
    let private = Ed25519PrivateKey::from_slice(&[3; 32]).unwrap();
    let public = private.to_public_key().encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).unwrap();
    let issued = issue_draft22(
        json!({"iss":"https://issuer.example", "vct":"urn:example:pid", "given_name":"Ada"}),
        &IssuancePlan {
            disclose: vec![],
            decoy_locations: vec![],
            salt_bytes: 16,
        },
        &mut DeterministicEntropy(1),
        &Ed25519Signer::new(&private),
        base_limits,
        SdJwtVcLimits::default(),
    )
    .unwrap();
    let envelope = issued.into_envelope().unwrap();
    assert_eq!(envelope.format().as_str(), ENVELOPE_FORMAT);
    let base = parse_draft22_envelope(&envelope, base_limits)
        .unwrap()
        .verify_issuer(&SignatureSuiteRegistry::recommended(), &key)
        .unwrap()
        .process_disclosures()
        .unwrap();
    let profile = Draft22SdJwtVc::validate(
        base,
        ProfileValidationInputs::default(),
        SdJwtVcLimits::default(),
    )
    .unwrap();
    assert!(profile.matches_type("urn:example:pid"));
    assert!(profile.receipt().profile_validated);
}

#[test]
fn appendix_a_pid_profile_claims_validate_against_the_pinned_draft() {
    let base_limits = SdJwtLimits::default();
    let private = Ed25519PrivateKey::from_slice(&[4; 32]).unwrap();
    let public = private.to_public_key().encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).unwrap();
    let issued = issue_draft22(
        json!({
            "iss":"https://pid-issuer.aendgard.example",
            "iat":1683000000,
            "exp":1883000000,
            "vct":"urn:example:eudi:pid:aendgard:1",
            "cnf":{"jwk":{
                "kty":"EC",
                "crv":"P-256",
                "x":"TCAER19Zvu3OHF4j4W4vfSVoHIP1ILilDls7vCeGemc",
                "y":"ZxjiWWbZMQGHVWKVQ4hbSIirsVfuecCE6t4jT9F2HZQ"
            }},
            "given_name":"Astrid"
        }),
        &IssuancePlan {
            disclose: vec![],
            decoy_locations: vec![],
            salt_bytes: 16,
        },
        &mut DeterministicEntropy(1),
        &Ed25519Signer::new(&private),
        base_limits,
        SdJwtVcLimits::default(),
    )
    .unwrap();
    let base = parse_draft22_envelope(&issued.into_envelope().unwrap(), base_limits)
        .unwrap()
        .verify_issuer(&SignatureSuiteRegistry::recommended(), &key)
        .unwrap()
        .process_disclosures()
        .unwrap();
    let profile = Draft22SdJwtVc::validate(
        base,
        ProfileValidationInputs::default(),
        SdJwtVcLimits::default(),
    )
    .unwrap();
    assert!(profile.matches_type("urn:example:eudi:pid:aendgard:1"));
}

#[test]
fn fractional_numeric_dates_and_collision_resistant_types_are_enforced() {
    let base_limits = SdJwtLimits::default();
    let profile_limits = SdJwtVcLimits::default();
    let private = Ed25519PrivateKey::from_slice(&[6; 32]).unwrap();
    let public = private.to_public_key().encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).unwrap();
    let issued = issue_draft22(
        json!({
            "iss":"https://issuer.example",
            "vct":"urn:example:fractional-date",
            "iat":9.75,
            "nbf":10.25,
            "exp":10.5
        }),
        &IssuancePlan {
            disclose: vec![],
            decoy_locations: vec![],
            salt_bytes: 16,
        },
        &mut DeterministicEntropy(1),
        &Ed25519Signer::new(&private),
        base_limits,
        profile_limits,
    )
    .unwrap();
    let base = parse_draft22_envelope(&issued.into_envelope().unwrap(), base_limits)
        .unwrap()
        .verify_issuer(&SignatureSuiteRegistry::recommended(), &key)
        .unwrap()
        .process_disclosures()
        .unwrap();
    assert!(
        Draft22SdJwtVc::validate(base, ProfileValidationInputs::default(), profile_limits).is_ok()
    );

    let invalid = issue_draft22(
        json!({"iss":"https://issuer.example", "vct":"local-name"}),
        &IssuancePlan {
            disclose: vec![],
            decoy_locations: vec![],
            salt_bytes: 16,
        },
        &mut DeterministicEntropy(1),
        &Ed25519Signer::new(&private),
        base_limits,
        profile_limits,
    );
    assert!(matches!(invalid, Err(SdJwtVcError::CredentialType)));

    let path_shaped = issue_draft22(
        json!({"iss":"https://issuer.example", "vct":"example.com/path"}),
        &IssuancePlan {
            disclose: vec![],
            decoy_locations: vec![],
            salt_bytes: 16,
        },
        &mut DeterministicEntropy(1),
        &Ed25519Signer::new(&private),
        base_limits,
        profile_limits,
    );
    assert!(matches!(path_shaped, Err(SdJwtVcError::CredentialType)));

    let reversed_large_integers = issue_draft22(
        json!({
            "iss":"https://issuer.example",
            "vct":"urn:example:large-date",
            "nbf":9_007_199_254_740_993_u64,
            "exp":9_007_199_254_740_992_u64
        }),
        &IssuancePlan {
            disclose: vec![],
            decoy_locations: vec![],
            salt_bytes: 16,
        },
        &mut DeterministicEntropy(1),
        &Ed25519Signer::new(&private),
        base_limits,
        profile_limits,
    );
    assert!(matches!(
        reversed_large_integers,
        Err(SdJwtVcError::NumericDate)
    ));
}

#[test]
fn aggregate_profile_output_and_retained_jwks_are_measured_independently() {
    let private = Ed25519PrivateKey::from_slice(&[5; 32]).unwrap();
    let signer = Ed25519Signer::new(&private);
    let compact_limits = SdJwtVcLimits {
        credential_type_bytes: 24,
        issuer_metadata_bytes: 24,
        type_metadata_bytes: 24,
        retained_output_bytes: 24,
        ..SdJwtVcLimits::default()
    };
    let result = issue_draft22(
        json!({
            "iss":"https://issuer.example",
            "vct":"urn:example:one",
            "aka_vcts":["urn:example:two"]
        }),
        &IssuancePlan {
            disclose: vec![],
            decoy_locations: vec![],
            salt_bytes: 16,
        },
        &mut DeterministicEntropy(1),
        &signer,
        SdJwtLimits::default(),
        compact_limits,
    );
    assert!(matches!(result, Err(SdJwtVcError::ResourceLimit)));

    let mut key_value = serde_json::to_value(private.to_public_key().encode_jwk()).unwrap();
    key_value
        .as_object_mut()
        .unwrap()
        .insert("label".into(), json!("x".repeat(512)));
    let key: PublicKeyJwk = serde_json::from_value(key_value).unwrap();
    let metadata_limits = SdJwtVcLimits {
        issuer_metadata_bytes: 128,
        ..SdJwtVcLimits::default()
    };
    assert!(matches!(
        IssuerMetadataEvidence::inline_jwks(
            "https://issuer.example",
            vec![key],
            1,
            metadata_limits,
        ),
        Err(SdJwtVcError::ResourceLimit)
    ));
}

#[test]
fn differential_sdk_ts_legacy_type_and_protected_claim_disclosure_fail_closed() {
    let base_limits = SdJwtLimits::default();
    let private = Ed25519PrivateKey::from_slice(&[3; 32]).unwrap();
    let public = private.to_public_key().encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).unwrap();
    let claims = json!({"iss":"https://issuer.example", "vct":"urn:example:pid"});
    let legacy = issue_with_options(
        claims.clone(),
        &IssuancePlan {
            disclose: vec![],
            decoy_locations: vec![],
            salt_bytes: 16,
        },
        IssuanceOptions {
            protected_type: Some("vc+sd-jwt"),
        },
        &mut DeterministicEntropy(1),
        &Ed25519Signer::new(&private),
        base_limits,
    )
    .unwrap();
    let base = identus_sd_jwt::UnverifiedSdJwt::parse(legacy.as_str(), base_limits)
        .unwrap()
        .verify_issuer(&SignatureSuiteRegistry::recommended(), &key)
        .unwrap()
        .process_disclosures()
        .unwrap();
    assert!(matches!(
        Draft22SdJwtVc::validate(
            base,
            ProfileValidationInputs::default(),
            SdJwtVcLimits::default()
        ),
        Err(SdJwtVcError::ProtectedType)
    ));

    let vct = ClaimPath::new(vec![PathSegment::Member("vct".into())], base_limits).unwrap();
    let result = issue_draft22(
        claims,
        &IssuancePlan {
            disclose: vec![vct],
            decoy_locations: vec![],
            salt_bytes: 16,
        },
        &mut DeterministicEntropy(1),
        &Ed25519Signer::new(&private),
        base_limits,
        SdJwtVcLimits::default(),
    );
    assert!(matches!(result, Err(SdJwtVcError::ClaimPlacement)));

    let nested_status = ClaimPath::new(
        vec![
            PathSegment::Member("status".into()),
            PathSegment::Member("status_list".into()),
            PathSegment::Member("idx".into()),
        ],
        base_limits,
    )
    .unwrap();
    let generic = issue_with_options(
        json!({
            "iss":"https://issuer.example",
            "vct":"urn:example:pid",
            "status":{"status_list":{"idx":7,"uri":"https://status.example/list"}}
        }),
        &IssuancePlan {
            disclose: vec![nested_status],
            decoy_locations: vec![],
            salt_bytes: 16,
        },
        IssuanceOptions {
            protected_type: Some("dc+sd-jwt"),
        },
        &mut DeterministicEntropy(1),
        &Ed25519Signer::new(&private),
        base_limits,
    )
    .unwrap();
    let base = identus_sd_jwt::UnverifiedSdJwt::parse(generic.as_str(), base_limits)
        .unwrap()
        .verify_issuer(&SignatureSuiteRegistry::recommended(), &key)
        .unwrap()
        .process_disclosures()
        .unwrap();
    assert!(matches!(
        Draft22SdJwtVc::validate(
            base,
            ProfileValidationInputs::default(),
            SdJwtVcLimits::default()
        ),
        Err(SdJwtVcError::ClaimPlacement)
    ));
}

#[test]
fn errors_and_debug_are_value_free() {
    let secret = "SECRET-CREDENTIAL-TYPE";
    let result = identus_sd_jwt_vc::TypeMetadata::new(
        secret,
        None,
        vec![],
        1,
        SdJwtVcLimits {
            credential_type_bytes: 4,
            ..SdJwtVcLimits::default()
        },
    )
    .unwrap_err();
    let rendered = format!("{result} {result:?}");
    assert!(!rendered.contains(secret));
    assert_eq!(result, SdJwtVcError::CredentialType);
}

#[test]
fn issuance_rejects_malformed_confirmation_and_status() {
    let base_limits = SdJwtLimits::default();
    let profile_limits = SdJwtVcLimits::default();
    let private = Ed25519PrivateKey::from_slice(&[3; 32]).unwrap();
    let signer = Ed25519Signer::new(&private);
    let plan = IssuancePlan {
        disclose: vec![],
        decoy_locations: vec![],
        salt_bytes: 16,
    };

    for claims in [
        json!({"iss":"https://issuer.example", "vct":"urn:example:pid", "cnf":{"jwk":null}}),
        json!({"iss":"https://issuer.example", "vct":"urn:example:pid", "cnf":{"jwk":{"unexpected":1}}}),
        json!({"iss":"https://issuer.example", "vct":"urn:example:pid", "cnf":{"kid":""}}),
        json!({"iss":"https://issuer.example", "vct":"urn:example:pid", "sub":42}),
        json!({"iss":"https://issuer.example", "vct":"urn:example:pid", "vct#integrity":"not-sri"}),
        json!({
            "iss":"https://issuer.example",
            "vct":"urn:example:pid",
            "status":{"status_list":{"idx":"7","uri":"https://status.example/list"}}
        }),
    ] {
        assert!(
            issue_draft22(
                claims,
                &plan,
                &mut DeterministicEntropy(1),
                &signer,
                base_limits,
                profile_limits,
            )
            .is_err()
        );
    }

    assert!(
        issue_draft22(
            json!({
                "iss":"https://issuer.example",
                "vct":"urn:example:pid",
                "sub":"holder-123",
                "vct#integrity":"sha256-5qAmRHgxiSHIPqS+mLzrVKVWliOBm955Oj86pMrc5g4="
            }),
            &plan,
            &mut DeterministicEntropy(1),
            &signer,
            base_limits,
            profile_limits,
        )
        .is_ok()
    );

    let bounded_limits = SdJwtVcLimits {
        issuer_metadata_bytes: 128,
        status_bytes: 128,
        ..profile_limits
    };
    let oversized_confirmation = json!({
        "iss":"https://issuer.example",
        "vct":"urn:example:pid",
        "cnf":{"jwk":{
            "kty":"OKP", "crv":"Ed25519",
            "x":"AQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQE",
            "label":"x".repeat(512)
        }}
    });
    assert!(
        issue_draft22(
            oversized_confirmation,
            &plan,
            &mut DeterministicEntropy(1),
            &signer,
            base_limits,
            bounded_limits,
        )
        .is_err()
    );
    let oversized_status = json!({
        "iss":"https://issuer.example",
        "vct":"urn:example:pid",
        "status":{"status_list":{"idx":7,"uri":format!("https://status.example/{}", "x".repeat(512))}}
    });
    assert!(matches!(
        issue_draft22(
            oversized_status,
            &plan,
            &mut DeterministicEntropy(1),
            &signer,
            base_limits,
            bounded_limits,
        ),
        Err(SdJwtVcError::ResourceLimit)
    ));
}

#[test]
fn inline_x509_evidence_matches_authenticated_header_and_exact_verification_key() {
    let private = Ed25519PrivateKey::from_slice(&[9; 32]).unwrap();
    let public = private.to_public_key().encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).unwrap();
    let chain = vec!["AQID".to_owned()];
    let limits = JwsLimits::default();
    let header = ProtectedHeader::with_key_reference(
        "Ed25519",
        Some("dc+sd-jwt"),
        Some(JwsKeyReference::x5c(chain.clone(), limits).unwrap()),
        limits,
    )
    .unwrap();
    let payload = serde_json::to_vec(&json!({
        "iss":"https://issuer.example", "vct":"urn:example:pid", "_sd_alg":"sha-256"
    }))
    .unwrap();
    let compact = JwsSigningInput::new(header, payload, limits)
        .unwrap()
        .sign_with(&Ed25519Signer::new(&private))
        .unwrap();
    let combined = format!("{}~", compact.compact());
    let base = identus_sd_jwt::UnverifiedSdJwt::parse(&combined, SdJwtLimits::default())
        .unwrap()
        .verify_issuer(&SignatureSuiteRegistry::recommended(), &key)
        .unwrap()
        .process_disclosures()
        .unwrap();
    let evidence = IssuerEvidence::InlineX509 {
        certificate_chain: chain,
        extracted_key: public.clone(),
    };
    assert!(
        Draft22SdJwtVc::validate(
            base,
            ProfileValidationInputs {
                issuer_evidence: Some(&evidence),
                ..ProfileValidationInputs::default()
            },
            SdJwtVcLimits::default(),
        )
        .is_ok()
    );

    let wrong = IssuerEvidence::InlineX509 {
        certificate_chain: vec!["BAUG".to_owned()],
        extracted_key: public.clone(),
    };
    let base = identus_sd_jwt::UnverifiedSdJwt::parse(&combined, SdJwtLimits::default())
        .unwrap()
        .verify_issuer(&SignatureSuiteRegistry::recommended(), &key)
        .unwrap()
        .process_disclosures()
        .unwrap();
    assert!(matches!(
        Draft22SdJwtVc::validate(
            base,
            ProfileValidationInputs {
                issuer_evidence: Some(&wrong),
                ..ProfileValidationInputs::default()
            },
            SdJwtVcLimits::default(),
        ),
        Err(SdJwtVcError::EvidenceMismatch)
    ));
}
