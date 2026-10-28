use identus_credentials::{
    CredentialStatusEvidence, CredentialStatusRevision, CredentialStatusValue,
};
use identus_crypto::{Ed25519PrivateKey, EncodeJwk, SecureRandom};
use identus_jose::{Ed25519Signer, JwsAlgorithm, JwsVerificationKey, SignatureSuiteRegistry};
use identus_sd_jwt::{IssuancePlan, SdJwtLimits};
use identus_sd_jwt_vc::{
    ClaimDisclosureRule, ClaimMetadata, Draft22SdJwtVc, IssuerEvidence, IssuerMetadataEvidence,
    MetadataClaimPathSegment, ProfileValidationInputs, SdJwtVcError, SdJwtVcLimits,
    StatusListBinding, StatusListEvidence, TypeMetadata, TypeMetadataChain, issue_draft22,
    parse_draft22_envelope,
};
use serde_json::json;

struct Entropy(u8);
impl SecureRandom for Entropy {
    fn fill_bytes(&mut self, output: &mut [u8]) -> Result<(), identus_crypto::Error> {
        for value in output {
            *value = self.0;
            self.0 = self.0.wrapping_add(1);
        }
        Ok(())
    }
}

#[test]
fn metadata_issuer_and_status_evidence_correlate_without_becoming_policy() {
    let base_limits = SdJwtLimits::default();
    let profile_limits = SdJwtVcLimits::default();
    let private = Ed25519PrivateKey::from_slice(&[5; 32]).unwrap();
    let public = private.to_public_key().encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).unwrap();
    let issued = issue_draft22(
        json!({
            "iss":"https://issuer.example",
            "vct":"urn:example:pid",
            "aka_vcts":["urn:example:identity"],
            "nbf":10,
            "exp":20,
            "family_name":"Lovelace",
            "nationalities":["GB","US"],
            "status":{"status_list":{"idx":7,"uri":"https://status.example/list"}}
        }),
        &IssuancePlan {
            disclose: vec![],
            decoy_locations: vec![],
            salt_bytes: 16,
        },
        &mut Entropy(1),
        &Ed25519Signer::new(&private),
        base_limits,
        profile_limits,
    )
    .unwrap();
    let envelope = issued.into_envelope().unwrap();
    let base = parse_draft22_envelope(&envelope, base_limits)
        .unwrap()
        .verify_issuer(&SignatureSuiteRegistry::recommended(), &key)
        .unwrap()
        .process_disclosures()
        .unwrap();
    let issuer = IssuerEvidence::Metadata(
        IssuerMetadataEvidence::inline_jwks(
            "https://issuer.example",
            vec![public.clone()],
            128,
            profile_limits,
        )
        .unwrap(),
    );
    let claim = ClaimMetadata::new(
        vec!["family_name".into()],
        true,
        ClaimDisclosureRule::Never,
        profile_limits,
    )
    .unwrap();
    let array_claim = ClaimMetadata::from_path(
        vec![
            MetadataClaimPathSegment::Member("nationalities".into()),
            MetadataClaimPathSegment::AnyIndex,
        ],
        false,
        ClaimDisclosureRule::Allowed,
        profile_limits,
    )
    .unwrap();
    let metadata = TypeMetadataChain::new(
        vec![
            TypeMetadata::new(
                "urn:example:pid",
                None,
                vec![claim, array_claim],
                256,
                profile_limits,
            )
            .unwrap(),
        ],
        profile_limits,
    )
    .unwrap();
    let binding = StatusListBinding::new(7, "https://status.example/list", profile_limits).unwrap();
    let evidence = StatusListEvidence::jwt(
        CredentialStatusEvidence::new(
            binding.generic().clone(),
            CredentialStatusValue::from_text("active").unwrap(),
            Some(CredentialStatusRevision::from_text("1").unwrap()),
            None,
            None,
        )
        .unwrap(),
    );
    assert!(matches!(
        Draft22SdJwtVc::validate(
            base,
            ProfileValidationInputs {
                require_key_binding: false,
                issuer_evidence: Some(&issuer),
                type_metadata: Some(&metadata),
                status_evidence: Some(&evidence),
            },
            SdJwtVcLimits {
                work_units: 15,
                ..profile_limits
            },
        ),
        Err(SdJwtVcError::ResourceLimit)
    ));
    let base = parse_draft22_envelope(&envelope, base_limits)
        .unwrap()
        .verify_issuer(&SignatureSuiteRegistry::recommended(), &key)
        .unwrap()
        .process_disclosures()
        .unwrap();
    assert!(matches!(
        Draft22SdJwtVc::validate(
            base,
            ProfileValidationInputs {
                issuer_evidence: Some(&issuer),
                ..ProfileValidationInputs::default()
            },
            SdJwtVcLimits {
                issuer_metadata_bytes: 64,
                ..profile_limits
            },
        ),
        Err(SdJwtVcError::ResourceLimit)
    ));
    let base = parse_draft22_envelope(&envelope, base_limits)
        .unwrap()
        .verify_issuer(&SignatureSuiteRegistry::recommended(), &key)
        .unwrap()
        .process_disclosures()
        .unwrap();
    assert!(matches!(
        Draft22SdJwtVc::validate(
            base,
            ProfileValidationInputs {
                type_metadata: Some(&metadata),
                ..ProfileValidationInputs::default()
            },
            SdJwtVcLimits {
                type_metadata_bytes: 128,
                ..profile_limits
            },
        ),
        Err(SdJwtVcError::ResourceLimit)
    ));
    let base = parse_draft22_envelope(&envelope, base_limits)
        .unwrap()
        .verify_issuer(&SignatureSuiteRegistry::recommended(), &key)
        .unwrap()
        .process_disclosures()
        .unwrap();
    let profile = Draft22SdJwtVc::validate(
        base,
        ProfileValidationInputs {
            require_key_binding: false,
            issuer_evidence: Some(&issuer),
            type_metadata: Some(&metadata),
            status_evidence: Some(&evidence),
        },
        profile_limits,
    )
    .unwrap();
    assert!(profile.matches_type("urn:example:identity"));
    assert!(profile.receipt().type_metadata_checked);
    assert!(profile.receipt().status_evidence_checked);

    let uri_metadata = IssuerEvidence::Metadata(
        IssuerMetadataEvidence::jwks_uri(
            "https://issuer.example",
            "https://issuer.example/jwks.json",
            vec![public.clone()],
            128,
            profile_limits,
        )
        .unwrap(),
    );
    let base = parse_draft22_envelope(&envelope, base_limits)
        .unwrap()
        .verify_issuer(&SignatureSuiteRegistry::recommended(), &key)
        .unwrap()
        .process_disclosures()
        .unwrap();
    assert!(
        Draft22SdJwtVc::validate(
            base,
            ProfileValidationInputs {
                issuer_evidence: Some(&uri_metadata),
                ..ProfileValidationInputs::default()
            },
            profile_limits,
        )
        .is_ok()
    );

    let unrelated = Ed25519PrivateKey::from_slice(&[6; 32])
        .unwrap()
        .to_public_key()
        .encode_jwk();
    let wrong_uri_metadata = IssuerEvidence::Metadata(
        IssuerMetadataEvidence::jwks_uri(
            "https://issuer.example",
            "https://issuer.example/jwks.json",
            vec![unrelated],
            128,
            profile_limits,
        )
        .unwrap(),
    );
    let base = parse_draft22_envelope(&envelope, base_limits)
        .unwrap()
        .verify_issuer(&SignatureSuiteRegistry::recommended(), &key)
        .unwrap()
        .process_disclosures()
        .unwrap();
    assert!(matches!(
        Draft22SdJwtVc::validate(
            base,
            ProfileValidationInputs {
                issuer_evidence: Some(&wrong_uri_metadata),
                ..ProfileValidationInputs::default()
            },
            profile_limits,
        ),
        Err(SdJwtVcError::EvidenceMismatch)
    ));
}

#[test]
fn exact_limits_and_metadata_cycles_fail_deterministically() {
    let limits = SdJwtVcLimits::default();
    assert!(limits.validate().is_ok());
    assert_eq!(
        SdJwtVcLimits {
            work_units: 0,
            ..limits
        }
        .validate(),
        Err(SdJwtVcError::InvalidLimits)
    );
    let first =
        TypeMetadata::new("urn:example:a", Some("urn:example:b"), vec![], 1, limits).unwrap();
    let second =
        TypeMetadata::new("urn:example:b", Some("urn:example:a"), vec![], 1, limits).unwrap();
    assert!(matches!(
        TypeMetadataChain::new(vec![first, second], limits),
        Err(SdJwtVcError::TypeMetadataCycle)
    ));

    let tight = SdJwtVcLimits {
        type_metadata_bytes: 32,
        ..limits
    };
    let oversized_claim = ClaimMetadata::new(
        vec!["x".repeat(32)],
        false,
        ClaimDisclosureRule::Allowed,
        tight,
    )
    .unwrap();
    assert!(matches!(
        TypeMetadata::new("urn:example:pid", None, vec![oversized_claim], 1, tight,),
        Err(SdJwtVcError::ResourceLimit)
    ));
}
