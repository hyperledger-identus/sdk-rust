use identus_credentials::{CredentialEnvelope, CredentialFormat, CredentialPayload};
use identus_crypto::{Ed25519PrivateKey, EncodeJwk, PublicKeyJwk, SecureRandom};
use identus_jose::{Ed25519Signer, JwsAlgorithm, JwsVerificationKey, SignatureSuiteRegistry};
use identus_sd_jwt::{IssuanceOptions, IssuancePlan, SdJwtLimits, issue_with_options};
use identus_sd_jwt_vc::{
    Draft22SdJwtVc, LegacyValidationInputs, LegacyVcSdJwtDraft03Adapter, ProfileValidationInputs,
    SdJwtVcError, SdJwtVcLimits, issue_draft22, parse_draft22_envelope,
};
use serde_json::{Value, json};

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

fn legacy_envelope(claims: Value) -> (CredentialEnvelope, PublicKeyJwk) {
    let limits = SdJwtLimits::default();
    let private = Ed25519PrivateKey::from_slice(&[7; 32]).unwrap();
    let public = private.to_public_key().encode_jwk();
    let compact = issue_with_options(
        claims,
        &IssuancePlan {
            disclose: vec![],
            decoy_locations: vec![],
            salt_bytes: 16,
        },
        IssuanceOptions {
            protected_type: Some("vc+sd-jwt"),
        },
        &mut Entropy(1),
        &Ed25519Signer::new(&private),
        limits,
    )
    .unwrap();
    let envelope = CredentialEnvelope::new(
        CredentialFormat::parse("vc+sd-jwt").unwrap(),
        CredentialPayload::new(compact.as_str().as_bytes().to_vec()).unwrap(),
    );
    (envelope, public)
}

fn verified_legacy_base(
    envelope: &CredentialEnvelope,
    public: &PublicKeyJwk,
) -> identus_sd_jwt::DisclosureVerifiedSdJwt {
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, public).unwrap();
    LegacyVcSdJwtDraft03Adapter::parse_envelope(envelope, SdJwtLimits::default())
        .unwrap()
        .verify_issuer(&SignatureSuiteRegistry::recommended(), &key)
        .unwrap()
        .process_disclosures()
        .unwrap()
}

#[test]
fn explicit_adapter_accepts_only_verified_common_subset() {
    let (envelope, key) = legacy_envelope(json!({
        "iss":"https://issuer.example",
        "vct":"urn:example:degree",
        "nbf":10,
        "exp":20,
        "sub":"did:example:holder"
    }));

    assert!(matches!(
        parse_draft22_envelope(&envelope, SdJwtLimits::default()),
        Err(SdJwtVcError::Envelope)
    ));
    let legacy = LegacyVcSdJwtDraft03Adapter::validate(
        verified_legacy_base(&envelope, &key),
        LegacyValidationInputs::default(),
        SdJwtVcLimits::default(),
    )
    .unwrap();
    assert!(legacy.matches_type("urn:example:degree"));
    assert!(legacy.receipt().issuer_signature);
    assert!(legacy.receipt().disclosures_processed);
    assert!(legacy.receipt().common_subset_validated);
    assert!(legacy.receipt().canonical_reissuance_required);

    assert!(matches!(
        Draft22SdJwtVc::validate(
            verified_legacy_base(&envelope, &key),
            ProfileValidationInputs::default(),
            SdJwtVcLimits::default(),
        ),
        Err(SdJwtVcError::ProtectedType)
    ));
    let debug = format!("{legacy:?} {:?}", legacy.receipt());
    for sensitive in ["issuer.example", "degree", "did:example:holder"] {
        assert!(!debug.contains(sensitive));
    }
}

#[test]
fn draft_specific_features_fail_closed() {
    for claims in [
        json!({"iss":"https://issuer.example","vct":"urn:example:pid","aka_vcts":["urn:example:identity"]}),
        json!({"iss":"https://issuer.example","vct":"urn:example:pid","vct#integrity":"sha256-YWJj"}),
        json!({"iss":"https://issuer.example","vct":"urn:example:pid","status":{"status_list":{"idx":1,"uri":"https://status.example/list"}}}),
    ] {
        let (envelope, key) = legacy_envelope(claims);
        assert!(matches!(
            LegacyVcSdJwtDraft03Adapter::validate(
                verified_legacy_base(&envelope, &key),
                LegacyValidationInputs::default(),
                SdJwtVcLimits::default(),
            ),
            Err(SdJwtVcError::LegacyDraftFeature)
        ));
    }
}

#[test]
fn canonical_issuance_remains_canonical_only() {
    let private = Ed25519PrivateKey::from_slice(&[9; 32]).unwrap();
    let issued = issue_draft22(
        json!({"iss":"https://issuer.example","vct":"urn:example:pid"}),
        &IssuancePlan {
            disclose: vec![],
            decoy_locations: vec![],
            salt_bytes: 16,
        },
        &mut Entropy(1),
        &Ed25519Signer::new(&private),
        SdJwtLimits::default(),
        SdJwtVcLimits::default(),
    )
    .unwrap();
    let envelope = issued.into_envelope().unwrap();
    assert_eq!(envelope.format().as_str(), "dc+sd-jwt");

    let public = private.to_public_key().encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).unwrap();
    let base = parse_draft22_envelope(&envelope, SdJwtLimits::default())
        .unwrap()
        .verify_issuer(&SignatureSuiteRegistry::recommended(), &key)
        .unwrap()
        .process_disclosures()
        .unwrap();
    assert!(matches!(
        LegacyVcSdJwtDraft03Adapter::validate(
            base,
            LegacyValidationInputs::default(),
            SdJwtVcLimits::default(),
        ),
        Err(SdJwtVcError::ProtectedType)
    ));
}
