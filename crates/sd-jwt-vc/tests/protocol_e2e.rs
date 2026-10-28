use std::{
    sync::Arc,
    task::{Context, Poll, Waker},
};

use identus_core::UnixTimestampMillis;
use identus_credentials::{
    CredentialEnvelope, CredentialFormat as GenericFormat, CredentialPayload,
    CredentialStatusEvidence, CredentialStatusRevision, CredentialStatusValue,
    CredentialVerificationRequest, CredentialVerifier, VerificationOutcome, VerificationStageName,
    VerificationStageStatus,
};
use identus_crypto::{Ed25519PrivateKey, EncodeJwk, SecureRandom};
use identus_jose::{Ed25519Signer, JwsAlgorithm, JwsVerificationKey, SignatureSuiteRegistry};
use identus_oid4vci::{ImmediateCredentialResponseCore, ImmediateCredentialResponseLimits};
use identus_oid4vp::{
    CredentialFormat, CredentialFormatCapability, CredentialFormatCapabilityError,
};
use identus_sd_jwt::{
    ClaimPath, IssuanceOptions, IssuancePlan, KeyBindingExpectations, PathSegment, SdJwtLimits,
    SelectionPlan, UnverifiedSdJwt, issue_with_options,
};
use identus_sd_jwt_vc::{
    Draft22CredentialVerifier, Draft22SdJwtVc, Draft22VerifierEvidence, IssuerEvidence,
    IssuerMetadataEvidence, ProfileValidationInputs, SdJwtVcLimits, StatusListBinding,
    StatusListEvidence, issue_draft22, parse_draft22_envelope,
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

struct Draft22Capability;
impl CredentialFormatCapability for Draft22Capability {
    fn format(&self) -> CredentialFormat {
        CredentialFormat::DcSdJwt
    }

    fn validate_metadata(&self, metadata: &Value) -> Result<(), CredentialFormatCapabilityError> {
        let object = metadata
            .as_object()
            .ok_or(CredentialFormatCapabilityError::InvalidMetadata)?;
        if object.len() != 1 {
            return Err(CredentialFormatCapabilityError::InvalidMetadata);
        }
        let values = object
            .get("vct_values")
            .and_then(Value::as_array)
            .ok_or(CredentialFormatCapabilityError::InvalidMetadata)?;
        if values.is_empty() || values.iter().any(|value| value.as_str().is_none()) {
            return Err(CredentialFormatCapabilityError::InvalidMetadata);
        }
        Ok(())
    }

    fn supports_transaction_type(&self, _transaction_type: &str) -> bool {
        false
    }
}

#[test]
fn oid4vci_receipt_oid4vp_match_selection_and_holder_binding_work_offline() {
    let base_limits = SdJwtLimits::default();
    let issuer_private = Ed25519PrivateKey::from_slice(&[7; 32]).unwrap();
    let holder_private = Ed25519PrivateKey::from_slice(&[8; 32]).unwrap();
    let issuer_public = issuer_private.to_public_key().encode_jwk();
    let holder_public = holder_private.to_public_key().encode_jwk();
    let issuer_key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &issuer_public).unwrap();
    let holder_key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &holder_public).unwrap();
    let degree = ClaimPath::new(vec![PathSegment::Member("degree".into())], base_limits).unwrap();
    let issued = issue_draft22(
        json!({
            "iss":"https://issuer.example",
            "vct":"urn:example:degree",
            "cnf":{"jwk":holder_public},
            "degree":"Computer Science",
            "public":"kept"
        }),
        &IssuancePlan {
            disclose: vec![degree.clone()],
            decoy_locations: vec![],
            salt_bytes: 16,
        },
        &mut Entropy(1),
        &Ed25519Signer::new(&issuer_private),
        base_limits,
        SdJwtVcLimits::default(),
    )
    .unwrap();

    let body = json!({"credentials":[{"credential":issued.as_str()}]}).to_string();
    let response =
        ImmediateCredentialResponseCore::parse(&body, ImmediateCredentialResponseLimits::default())
            .unwrap();
    let compact = response.credentials()[0].expose_sensitive_string().unwrap();
    let envelope = CredentialEnvelope::new(
        GenericFormat::parse("dc+sd-jwt").unwrap(),
        CredentialPayload::new(compact.as_bytes().to_vec()).unwrap(),
    );
    let generic_verifier = Draft22CredentialVerifier::new(
        Arc::new(SignatureSuiteRegistry::recommended()),
        JwsAlgorithm::Ed25519,
        issuer_public.clone(),
        Draft22VerifierEvidence::default(),
        base_limits,
        SdJwtVcLimits::default(),
    )
    .unwrap();
    let mut future =
        generic_verifier.verify(CredentialVerificationRequest::from_envelope(&envelope));
    let mut context = Context::from_waker(Waker::noop());
    let Poll::Ready(report) = future.as_mut().poll(&mut context) else {
        panic!("effect-free verifier must complete immediately")
    };
    assert_eq!(
        report.unwrap().outcome(),
        VerificationOutcome::Indeterminate
    );
    let base = parse_draft22_envelope(&envelope, base_limits)
        .unwrap()
        .verify_issuer(&SignatureSuiteRegistry::recommended(), &issuer_key)
        .unwrap()
        .process_disclosures()
        .unwrap();
    let profile = Draft22SdJwtVc::validate(
        base,
        ProfileValidationInputs {
            require_key_binding: true,
            ..ProfileValidationInputs::default()
        },
        SdJwtVcLimits::default(),
    )
    .unwrap();
    assert!(profile.matches_type("urn:example:degree"));

    let capability = Draft22Capability;
    capability
        .validate_metadata(&json!({"vct_values":["urn:example:degree"]}))
        .unwrap();
    assert_eq!(
        capability.format().expose_sensitive_identifier(),
        "dc+sd-jwt"
    );

    let presentation = profile
        .into_base()
        .select(&SelectionPlan {
            paths: vec![degree],
        })
        .unwrap();
    let time = UnixTimestampMillis::new(1_700_000_000_000);
    let bound = presentation
        .bind(
            "https://verifier.example",
            "nonce",
            time,
            &Ed25519Signer::new(&holder_private),
        )
        .unwrap();
    let verified = UnverifiedSdJwt::parse(&bound, base_limits)
        .unwrap()
        .verify_issuer(&SignatureSuiteRegistry::recommended(), &issuer_key)
        .unwrap()
        .process_disclosures()
        .unwrap()
        .verify_key_binding(
            &SignatureSuiteRegistry::recommended(),
            &holder_key,
            KeyBindingExpectations {
                audience: "https://verifier.example",
                nonce: "nonce",
                issued_at: time,
            },
        )
        .unwrap();
    let receipt = verified.receipt();
    assert!(receipt.issuer_signature && receipt.disclosures_processed && receipt.holder_binding);
    let debug = format!("{response:?} {verified:?} {receipt:?}");
    for secret in ["Computer Science", "nonce", "https://issuer.example"] {
        assert!(!debug.contains(secret));
    }
}

#[test]
fn generic_report_preserves_proof_and_does_not_promote_status_correlation() {
    let base_limits = SdJwtLimits::default();
    let profile_limits = SdJwtVcLimits::default();
    let private = Ed25519PrivateKey::from_slice(&[7; 32]).unwrap();
    let public = private.to_public_key().encode_jwk();
    let binding = StatusListBinding::new(7, "https://status.example/list", profile_limits).unwrap();
    let status = StatusListEvidence::jwt(
        CredentialStatusEvidence::new(
            binding.generic().clone(),
            CredentialStatusValue::from_text("revoked").unwrap(),
            Some(CredentialStatusRevision::from_text("1").unwrap()),
            None,
            None,
        )
        .unwrap(),
    );
    let issued = issue_draft22(
        json!({
            "iss":"https://issuer.example",
            "vct":"urn:example:pid",
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
    let verifier = Draft22CredentialVerifier::new(
        Arc::new(SignatureSuiteRegistry::recommended()),
        JwsAlgorithm::Ed25519,
        public.clone(),
        Draft22VerifierEvidence {
            status: Some(status),
            ..Draft22VerifierEvidence::default()
        },
        base_limits,
        profile_limits,
    )
    .unwrap();
    let mut future = verifier.verify(CredentialVerificationRequest::from_envelope(&envelope));
    let mut context = Context::from_waker(Waker::noop());
    let Poll::Ready(report) = future.as_mut().poll(&mut context) else {
        panic!("effect-free verifier must complete immediately")
    };
    let report = report.unwrap();
    assert_eq!(
        report.stage(VerificationStageName::IssuerKey).status(),
        VerificationStageStatus::NotChecked
    );
    assert_eq!(
        report.stage(VerificationStageName::Status).status(),
        VerificationStageStatus::NotChecked
    );
    assert_eq!(
        report
            .stage(VerificationStageName::Status)
            .reason()
            .unwrap()
            .as_str(),
        "sd_jwt_vc.status_correlated_policy_not_checked"
    );

    let wrong_binding =
        StatusListBinding::new(8, "https://status.example/list", profile_limits).unwrap();
    let wrong_status = StatusListEvidence::jwt(
        CredentialStatusEvidence::new(
            wrong_binding.generic().clone(),
            CredentialStatusValue::from_text("active").unwrap(),
            None,
            None,
            None,
        )
        .unwrap(),
    );
    let verifier = Draft22CredentialVerifier::new(
        Arc::new(SignatureSuiteRegistry::recommended()),
        JwsAlgorithm::Ed25519,
        public.clone(),
        Draft22VerifierEvidence {
            status: Some(wrong_status),
            ..Draft22VerifierEvidence::default()
        },
        base_limits,
        profile_limits,
    )
    .unwrap();
    let mut future = verifier.verify(CredentialVerificationRequest::from_envelope(&envelope));
    let Poll::Ready(report) = future.as_mut().poll(&mut context) else {
        panic!("effect-free verifier must complete immediately")
    };
    let report = report.unwrap();
    assert_eq!(report.outcome(), VerificationOutcome::Invalid);
    assert_eq!(
        report.stage(VerificationStageName::Structural).status(),
        VerificationStageStatus::Passed
    );
    assert_eq!(
        report.stage(VerificationStageName::Proof).status(),
        VerificationStageStatus::Passed
    );
    assert_eq!(
        report.stage(VerificationStageName::Status).status(),
        VerificationStageStatus::Failed
    );

    let malformed = issue_with_options(
        json!({
            "iss":"https://issuer.example",
            "vct":"urn:example:pid",
            "status":{"status_list":{"idx":"invalid","uri":"https://status.example/list"}}
        }),
        &IssuancePlan {
            disclose: vec![],
            decoy_locations: vec![],
            salt_bytes: 16,
        },
        IssuanceOptions {
            protected_type: Some("dc+sd-jwt"),
        },
        &mut Entropy(1),
        &Ed25519Signer::new(&private),
        base_limits,
    )
    .unwrap();
    let malformed_envelope = CredentialEnvelope::new(
        GenericFormat::parse("dc+sd-jwt").unwrap(),
        CredentialPayload::new(malformed.as_str().as_bytes().to_vec()).unwrap(),
    );
    let wrong_issuer = IssuerEvidence::Metadata(
        IssuerMetadataEvidence::inline_jwks(
            "https://other.example",
            vec![public.clone()],
            128,
            profile_limits,
        )
        .unwrap(),
    );
    let verifier = Draft22CredentialVerifier::new(
        Arc::new(SignatureSuiteRegistry::recommended()),
        JwsAlgorithm::Ed25519,
        public.clone(),
        Draft22VerifierEvidence {
            issuer: Some(wrong_issuer),
            ..Draft22VerifierEvidence::default()
        },
        base_limits,
        profile_limits,
    )
    .unwrap();
    let mut future = verifier.verify(CredentialVerificationRequest::from_envelope(
        &malformed_envelope,
    ));
    let Poll::Ready(report) = future.as_mut().poll(&mut context) else {
        panic!("effect-free verifier must complete immediately")
    };
    assert_eq!(
        report
            .unwrap()
            .stage(VerificationStageName::Structural)
            .status(),
        VerificationStageStatus::Failed
    );

    let legacy = issue_with_options(
        json!({"iss":"https://issuer.example", "vct":"urn:example:pid"}),
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
        base_limits,
    )
    .unwrap();
    let legacy_envelope = CredentialEnvelope::new(
        GenericFormat::parse("dc+sd-jwt").unwrap(),
        CredentialPayload::new(legacy.as_str().as_bytes().to_vec()).unwrap(),
    );
    let verifier = Draft22CredentialVerifier::new(
        Arc::new(SignatureSuiteRegistry::recommended()),
        JwsAlgorithm::Ed25519,
        public,
        Draft22VerifierEvidence::default(),
        base_limits,
        profile_limits,
    )
    .unwrap();
    let mut future = verifier.verify(CredentialVerificationRequest::from_envelope(
        &legacy_envelope,
    ));
    let Poll::Ready(report) = future.as_mut().poll(&mut context) else {
        panic!("effect-free verifier must complete immediately")
    };
    let report = report.unwrap();
    assert_eq!(report.outcome(), VerificationOutcome::Invalid);
    assert_eq!(
        report.stage(VerificationStageName::Structural).status(),
        VerificationStageStatus::Failed
    );
    assert_eq!(
        report.stage(VerificationStageName::IssuerKey).status(),
        VerificationStageStatus::NotChecked
    );
    assert_eq!(
        report.stage(VerificationStageName::Proof).status(),
        VerificationStageStatus::Passed
    );
}
