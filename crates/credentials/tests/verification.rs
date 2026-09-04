use std::{hint::black_box, str::FromStr, time::Instant};

use identus_core::ErrorKind;
use identus_credentials::{
    CredentialEnvelope, CredentialError, CredentialFormat, CredentialPayload,
    MAX_VERIFICATION_REASON_CODE_BYTES, VERIFICATION_STAGE_COUNT, VerificationOutcome,
    VerificationReasonCode, VerificationReport, VerificationStage, VerificationStageName,
    VerificationStageStatus,
};

fn reason(value: &str) -> VerificationReasonCode {
    VerificationReasonCode::parse(value).expect("valid reason")
}

fn all_passed() -> [VerificationStage; VERIFICATION_STAGE_COUNT] {
    VerificationStageName::ALL.map(|name| {
        VerificationStage::new(name, VerificationStageStatus::Passed, None).expect("passed stage")
    })
}

#[test]
fn stage_taxonomy_is_complete_canonical_and_policy_neutral() {
    assert_eq!(
        VerificationStageName::ALL.map(VerificationStageName::as_str),
        [
            "structural",
            "issuer_key",
            "proof",
            "temporal",
            "status",
            "schema",
        ]
    );
    assert_eq!(VerificationStageName::ALL.len(), VERIFICATION_STAGE_COUNT);
    assert!(
        VerificationStageName::ALL
            .iter()
            .all(|name| name.as_str() != "trust")
    );

    for name in VerificationStageName::ALL {
        assert_eq!(VerificationStageName::from_str(name.as_str()), Ok(name));
        assert_eq!(name.to_string(), name.as_str());
    }
}

#[test]
fn stage_names_reject_noncanonical_input_without_echoing_it() {
    for value in [
        "",
        "issuer/key",
        "issuer-key",
        "Trust",
        "status ",
        "unknown",
    ] {
        let error = VerificationStageName::parse(value).expect_err("invalid stage name");
        assert_eq!(error, CredentialError::InvalidVerificationStageName);
        if !value.is_empty() {
            assert!(!error.to_string().contains(value));
            assert!(!error.to_identus_error().to_string().contains(value));
        }
    }
}

#[test]
fn reason_codes_accept_namespaces_and_exact_boundaries() {
    for value in [
        "proof.invalid_signature",
        "status:not_supported",
        "schema_unavailable",
        "adapter-neutral",
        "7",
    ] {
        let code = VerificationReasonCode::from_str(value).expect("valid reason");
        assert_eq!(code.as_str(), value);
        assert_eq!(code.to_string(), value);
    }

    let maximum = "a".repeat(MAX_VERIFICATION_REASON_CODE_BYTES);
    assert_eq!(
        VerificationReasonCode::parse(&maximum)
            .expect("maximum reason")
            .as_str(),
        maximum
    );
}

#[test]
fn reason_codes_reject_unsafe_input_without_echoing_it() {
    let oversized = "a".repeat(MAX_VERIFICATION_REASON_CODE_BYTES + 1);
    for value in [
        "",
        "Proof.invalid",
        ".leading",
        "has space",
        "line\nbreak",
        "slash/value",
        "emoji-🔐",
        oversized.as_str(),
    ] {
        let error = VerificationReasonCode::parse(value).expect_err("invalid reason");
        assert_eq!(error, CredentialError::InvalidVerificationReasonCode);
        if !value.is_empty() {
            assert!(!error.to_string().contains(value));
            assert!(!error.to_identus_error().to_string().contains(value));
        }
    }
}

#[test]
fn stage_construction_enforces_reason_presence() {
    let name = VerificationStageName::Proof;
    let code = reason("proof.invalid_signature");

    assert!(VerificationStage::new(name, VerificationStageStatus::Passed, None).is_ok());
    assert!(
        VerificationStage::new(name, VerificationStageStatus::Failed, Some(code.clone())).is_ok()
    );
    assert!(
        VerificationStage::new(
            name,
            VerificationStageStatus::NotChecked,
            Some(code.clone())
        )
        .is_ok()
    );
    assert_eq!(
        VerificationStage::new(name, VerificationStageStatus::Passed, Some(code.clone())),
        Err(CredentialError::UnexpectedVerificationReason)
    );
    assert_eq!(
        VerificationStage::new(name, VerificationStageStatus::Failed, None),
        Err(CredentialError::MissingVerificationReason)
    );
    assert_eq!(
        VerificationStage::new(name, VerificationStageStatus::NotChecked, None),
        Err(CredentialError::MissingVerificationReason)
    );
}

#[test]
fn report_derives_valid_and_indeterminate_outcomes() {
    let valid = VerificationReport::new(all_passed()).expect("valid report");
    assert_eq!(valid.outcome(), VerificationOutcome::Valid);
    assert_eq!(valid.outcome().as_str(), "valid");

    let mut incomplete = all_passed();
    incomplete[4] = VerificationStage::new(
        VerificationStageName::Status,
        VerificationStageStatus::NotChecked,
        Some(reason("status:not_supported")),
    )
    .expect("not checked stage");
    let report = VerificationReport::new(incomplete).expect("indeterminate report");
    assert_eq!(report.outcome(), VerificationOutcome::Indeterminate);
    assert_eq!(
        report.stage(VerificationStageName::Status).reason(),
        Some(&reason("status:not_supported"))
    );
}

#[test]
fn invalid_outcome_precedes_indeterminate() {
    let mut stages = all_passed();
    stages[2] = VerificationStage::new(
        VerificationStageName::Proof,
        VerificationStageStatus::Failed,
        Some(reason("proof.invalid_signature")),
    )
    .expect("failed stage");
    stages[4] = VerificationStage::new(
        VerificationStageName::Status,
        VerificationStageStatus::NotChecked,
        Some(reason("status:registry_unavailable")),
    )
    .expect("not checked stage");

    let report = VerificationReport::new(stages).expect("invalid report");
    assert_eq!(report.outcome(), VerificationOutcome::Invalid);
    assert_eq!(report.outcome().as_str(), "invalid");
    assert_eq!(
        report.stage(VerificationStageName::Proof).status(),
        VerificationStageStatus::Failed
    );
}

#[test]
fn report_rejects_reordered_or_duplicate_stages() {
    let mut reordered = all_passed();
    reordered.swap(0, 1);
    assert_eq!(
        VerificationReport::new(reordered),
        Err(CredentialError::NonCanonicalVerificationReport)
    );

    let mut duplicate = all_passed();
    duplicate[5] = VerificationStage::new(
        VerificationStageName::Status,
        VerificationStageStatus::Passed,
        None,
    )
    .expect("duplicate stage value");
    assert_eq!(
        VerificationReport::new(duplicate),
        Err(CredentialError::NonCanonicalVerificationReport)
    );
}

#[test]
fn validity_and_product_trust_remain_independent() {
    struct ProductAssessment {
        issuer_trusted: bool,
        report: VerificationReport,
    }

    let valid_untrusted = ProductAssessment {
        issuer_trusted: false,
        report: VerificationReport::new(all_passed()).expect("valid report"),
    };

    let mut stages = all_passed();
    stages[2] = VerificationStage::new(
        VerificationStageName::Proof,
        VerificationStageStatus::Failed,
        Some(reason("proof.invalid_signature")),
    )
    .expect("failed stage");
    let invalid_trusted = ProductAssessment {
        issuer_trusted: true,
        report: VerificationReport::new(stages).expect("invalid report"),
    };

    assert!(!valid_untrusted.issuer_trusted);
    assert_eq!(valid_untrusted.report.outcome(), VerificationOutcome::Valid);
    assert!(invalid_trusted.issuer_trusted);
    assert_eq!(
        invalid_trusted.report.outcome(),
        VerificationOutcome::Invalid
    );
}

#[test]
fn independent_formats_and_donor_shaped_status_evidence_share_the_report() {
    let midnight = CredentialEnvelope::new(
        CredentialFormat::parse("midnight_cbor_phase1").expect("format"),
        CredentialPayload::new(vec![0xa1, 0x01]).expect("payload"),
    );
    let unrelated = CredentialEnvelope::new(
        CredentialFormat::parse("example+opaque:v1").expect("format"),
        CredentialPayload::new(vec![0xff]).expect("payload"),
    );

    let mut midnight_stages = all_passed();
    midnight_stages[4] = VerificationStage::new(
        VerificationStageName::Status,
        VerificationStageStatus::NotChecked,
        Some(reason("status:registry_unavailable")),
    )
    .expect("Midnight status projection");
    let midnight_report = VerificationReport::new(midnight_stages).expect("Midnight-shaped report");
    let cardano_report = VerificationReport::new(all_passed()).expect("Cardano-shaped report");

    assert_eq!(midnight.format().as_str(), "midnight_cbor_phase1");
    assert_eq!(unrelated.format().as_str(), "example+opaque:v1");
    assert_eq!(
        midnight_report.outcome(),
        VerificationOutcome::Indeterminate
    );
    assert_eq!(cardano_report.outcome(), VerificationOutcome::Valid);
}

#[test]
fn verification_errors_bridge_to_static_credential_codes() {
    let cases = [
        (
            CredentialError::InvalidVerificationStageName,
            "credential.invalid_verification_stage_name",
        ),
        (
            CredentialError::InvalidVerificationReasonCode,
            "credential.invalid_verification_reason_code",
        ),
        (
            CredentialError::MissingVerificationReason,
            "credential.missing_verification_reason",
        ),
        (
            CredentialError::UnexpectedVerificationReason,
            "credential.unexpected_verification_reason",
        ),
        (
            CredentialError::NonCanonicalVerificationReport,
            "credential.non_canonical_verification_report",
        ),
    ];

    for (error, expected_code) in cases {
        let public = error.to_identus_error();
        assert_eq!(
            public.capability().expect("capability").as_str(),
            "credential"
        );
        assert_eq!(public.kind(), ErrorKind::InvalidInput);
        assert_eq!(public.code().as_str(), expected_code);
        assert!(!public.to_string().contains("caller-controlled-value"));
    }
}

#[test]
#[ignore = "manual release-mode throughput observation"]
fn verification_report_construction_throughput_diagnostic() {
    const ITERATIONS: u32 = 1_000_000;
    let started = Instant::now();
    for _ in 0..ITERATIONS {
        black_box(VerificationReport::new(all_passed()).expect("valid report"));
    }
    let elapsed = started.elapsed();
    let reports_per_second = f64::from(ITERATIONS) / elapsed.as_secs_f64();
    eprintln!(
        "verification report construction: {ITERATIONS} reports in {elapsed:?} ({reports_per_second:.0} reports/s)"
    );
}
