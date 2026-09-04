use std::{hint::black_box, time::Instant};

use identus_core::{DurationMillis, ErrorKind, UnixTimestampMillis};
use identus_credentials::{
    CredentialError, CredentialStatusBinding, CredentialStatusBindings, CredentialStatusEvidence,
    CredentialStatusFreshness, CredentialStatusHandle, CredentialStatusMethod,
    CredentialStatusPurpose, CredentialStatusQuery, CredentialStatusReference,
    CredentialStatusRequirements, CredentialStatusRevision, CredentialStatusValue,
    MAX_CREDENTIAL_STATUS_BINDINGS, MAX_CREDENTIAL_STATUS_HANDLE_BYTES,
    MAX_CREDENTIAL_STATUS_METHOD_BYTES, MAX_CREDENTIAL_STATUS_PURPOSE_BYTES,
    MAX_CREDENTIAL_STATUS_REFERENCE_BYTES, MAX_CREDENTIAL_STATUS_REQUIREMENT_VALUES,
    MAX_CREDENTIAL_STATUS_REVISION_BYTES, MAX_CREDENTIAL_STATUS_VALUE_BYTES,
};

fn method(value: &str) -> CredentialStatusMethod {
    CredentialStatusMethod::parse(value).expect("valid status method")
}

fn purpose(value: &str) -> CredentialStatusPurpose {
    CredentialStatusPurpose::parse(value).expect("valid status purpose")
}

fn reference(value: &str) -> CredentialStatusReference {
    CredentialStatusReference::from_text(value).expect("valid status reference")
}

fn handle(value: &str) -> CredentialStatusHandle {
    CredentialStatusHandle::from_text(value).expect("valid status handle")
}

fn binding(
    method_value: &str,
    purpose_value: &str,
    reference_value: &str,
    handle_value: &str,
) -> CredentialStatusBinding {
    CredentialStatusBinding::new(
        method(method_value),
        purpose(purpose_value),
        reference(reference_value),
        handle(handle_value),
    )
}

#[test]
fn open_identifiers_preserve_w3c_and_midnight_names() {
    for value in [
        "BitstringStatusListEntry",
        "midnight:same-contract-live",
        "example.future.status/v1",
    ] {
        assert_eq!(method(value).as_str(), value);
    }
    for value in ["revocation", "suspension", "message", "midnight:live"] {
        assert_eq!(purpose(value).as_str(), value);
    }

    let maximum_method = "m".repeat(MAX_CREDENTIAL_STATUS_METHOD_BYTES);
    let maximum_purpose = "p".repeat(MAX_CREDENTIAL_STATUS_PURPOSE_BYTES);
    assert_eq!(method(&maximum_method).as_str(), maximum_method);
    assert_eq!(purpose(&maximum_purpose).as_str(), maximum_purpose);
}

#[test]
fn identifiers_reject_empty_padded_control_and_oversized_input_without_echoing_it() {
    let oversized = "x".repeat(MAX_CREDENTIAL_STATUS_METHOD_BYTES + 1);
    for value in ["", " padded", "padded ", "line\nbreak", oversized.as_str()] {
        let error = CredentialStatusMethod::parse(value).expect_err("invalid method");
        assert_eq!(error, CredentialError::InvalidStatusMethod);
        if !value.is_empty() {
            assert!(!error.to_string().contains(value));
            assert!(!error.to_identus_error().to_string().contains(value));
        }
    }
    assert_eq!(
        CredentialStatusPurpose::parse(&"x".repeat(MAX_CREDENTIAL_STATUS_PURPOSE_BYTES + 1)),
        Err(CredentialError::InvalidStatusPurpose)
    );
}

#[test]
fn deterministic_boundary_corpus_has_no_hidden_identifier_or_opaque_states() {
    for length in [0, 1, 2, 255, 256, 257] {
        let value = "x".repeat(length);
        assert_eq!(
            CredentialStatusMethod::parse(&value).is_ok(),
            (1..=MAX_CREDENTIAL_STATUS_METHOD_BYTES).contains(&length)
        );
        assert_eq!(
            CredentialStatusValue::from_text(&value).is_ok(),
            (1..=MAX_CREDENTIAL_STATUS_VALUE_BYTES).contains(&length)
        );
    }
    for control in ['\0', '\n', '\r', '\u{7f}', '\u{85}'] {
        let value = format!("left{control}right");
        assert_eq!(
            CredentialStatusPurpose::parse(&value),
            Err(CredentialError::InvalidStatusPurpose)
        );
        assert_eq!(
            CredentialStatusHandle::from_text(&value),
            Err(CredentialError::InvalidStatusHandle)
        );
    }
    for length in [0, 1, 255, 256, 257, 1_023, 1_024, 1_025] {
        let bytes = vec![0xa5; length];
        assert_eq!(
            CredentialStatusHandle::from_bytes(bytes).is_ok(),
            (1..=MAX_CREDENTIAL_STATUS_HANDLE_BYTES).contains(&length)
        );
    }
}

#[test]
fn opaque_roles_preserve_exact_text_and_bytes_at_their_limits() {
    let reference_text = "r".repeat(MAX_CREDENTIAL_STATUS_REFERENCE_BYTES);
    let handle_bytes = vec![0x55; MAX_CREDENTIAL_STATUS_HANDLE_BYTES];
    let revision_text = "v".repeat(MAX_CREDENTIAL_STATUS_REVISION_BYTES);
    let value_bytes = vec![0xa5; MAX_CREDENTIAL_STATUS_VALUE_BYTES];

    let reference = CredentialStatusReference::from_text(&reference_text).unwrap();
    let handle = CredentialStatusHandle::from_bytes(handle_bytes.clone()).unwrap();
    let revision = CredentialStatusRevision::from_text(&revision_text).unwrap();
    let value = CredentialStatusValue::from_bytes(value_bytes.clone()).unwrap();

    assert_eq!(reference.as_text(), Some(reference_text.as_str()));
    assert_eq!(reference.as_bytes(), None);
    assert_eq!(handle.as_text(), None);
    assert_eq!(handle.as_bytes(), Some(handle_bytes.as_slice()));
    assert_eq!(revision.as_text(), Some(revision_text.as_str()));
    assert_eq!(value.as_bytes(), Some(value_bytes.as_slice()));
    assert_eq!(
        reference.encoded_len(),
        MAX_CREDENTIAL_STATUS_REFERENCE_BYTES
    );
    assert_eq!(value.encoded_len(), MAX_CREDENTIAL_STATUS_VALUE_BYTES);
}

#[test]
fn every_opaque_role_rejects_empty_invalid_or_oversized_input() {
    assert_eq!(
        CredentialStatusReference::from_text(""),
        Err(CredentialError::InvalidStatusReference)
    );
    assert_eq!(
        CredentialStatusReference::from_text(" padded"),
        Err(CredentialError::InvalidStatusReference)
    );
    assert_eq!(
        CredentialStatusReference::from_bytes(Vec::new()),
        Err(CredentialError::InvalidStatusReference)
    );
    assert_eq!(
        CredentialStatusReference::from_bytes(vec![0; MAX_CREDENTIAL_STATUS_REFERENCE_BYTES + 1]),
        Err(CredentialError::InvalidStatusReference)
    );
    assert_eq!(
        CredentialStatusHandle::from_bytes(vec![0; MAX_CREDENTIAL_STATUS_HANDLE_BYTES + 1]),
        Err(CredentialError::InvalidStatusHandle)
    );
    assert_eq!(
        CredentialStatusRevision::from_text(&"r".repeat(MAX_CREDENTIAL_STATUS_REVISION_BYTES + 1)),
        Err(CredentialError::InvalidStatusRevision)
    );
    assert_eq!(
        CredentialStatusValue::from_bytes(vec![0; MAX_CREDENTIAL_STATUS_VALUE_BYTES + 1]),
        Err(CredentialError::InvalidStatusValue)
    );
}

#[test]
fn w3c_revocation_and_suspension_bindings_coexist() {
    let revocation = binding(
        "BitstringStatusListEntry",
        "revocation",
        "https://status.example/lists/1",
        "94567",
    );
    let suspension = binding(
        "BitstringStatusListEntry",
        "suspension",
        "https://status.example/lists/2",
        "94567",
    );
    let bindings = CredentialStatusBindings::new(vec![revocation, suspension]).unwrap();

    assert_eq!(bindings.len(), 2);
    assert_eq!(bindings.as_slice()[0].purpose().as_str(), "revocation");
    assert_eq!(bindings.as_slice()[1].purpose().as_str(), "suspension");
}

#[test]
fn midnight_binary_binding_and_evidence_remain_format_neutral() {
    let binding = CredentialStatusBinding::new(
        method("midnight:external-nonmembership"),
        purpose("revocation"),
        CredentialStatusReference::from_bytes(vec![0x11; 32]).unwrap(),
        CredentialStatusHandle::from_bytes(vec![0x22; 32]).unwrap(),
    );
    let evidence = CredentialStatusEvidence::new(
        binding,
        CredentialStatusValue::from_bytes(vec![0]).unwrap(),
        Some(CredentialStatusRevision::from_bytes(vec![0x33; 32]).unwrap()),
        Some(UnixTimestampMillis::new(10_000)),
        Some(UnixTimestampMillis::new(20_000)),
    )
    .unwrap();

    assert_eq!(
        evidence.binding().method().as_str(),
        "midnight:external-nonmembership"
    );
    assert_eq!(evidence.value().as_bytes(), Some([0].as_slice()));
    assert_eq!(evidence.revision().unwrap().as_bytes().unwrap().len(), 32);
    assert_eq!(
        evidence.observed_at(),
        Some(UnixTimestampMillis::new(10_000))
    );
    assert_eq!(
        evidence.expires_at(),
        Some(UnixTimestampMillis::new(20_000))
    );
}

#[test]
fn binding_collections_enforce_nonempty_maximum_and_exact_uniqueness() {
    assert_eq!(
        CredentialStatusBindings::new(Vec::new()),
        Err(CredentialError::InvalidStatusBindingCollection)
    );
    let maximum: Vec<_> = (0..MAX_CREDENTIAL_STATUS_BINDINGS)
        .map(|index| {
            binding(
                "BitstringStatusListEntry",
                "revocation",
                "https://status.example/list",
                &index.to_string(),
            )
        })
        .collect();
    assert_eq!(
        CredentialStatusBindings::new(maximum.clone())
            .unwrap()
            .len(),
        MAX_CREDENTIAL_STATUS_BINDINGS
    );
    let mut oversized = maximum.clone();
    oversized.push(binding(
        "BitstringStatusListEntry",
        "revocation",
        "https://status.example/list",
        "overflow",
    ));
    assert_eq!(
        CredentialStatusBindings::new(oversized),
        Err(CredentialError::InvalidStatusBindingCollection)
    );
    let duplicated = maximum[0].clone();
    assert_eq!(
        CredentialStatusBindings::new(vec![duplicated.clone(), duplicated]),
        Err(CredentialError::DuplicateStatusBinding)
    );
}

#[test]
fn freshness_requires_a_criterion_and_preserves_zero_age() {
    assert_eq!(
        CredentialStatusFreshness::new(None, None),
        Err(CredentialError::InvalidStatusFreshness)
    );
    let age_only = CredentialStatusFreshness::new(None, Some(DurationMillis::new(0))).unwrap();
    assert_eq!(age_only.maximum_age(), Some(DurationMillis::new(0)));
    assert_eq!(age_only.minimum_revision(), None);

    let revision = CredentialStatusRevision::from_text("ledger:42").unwrap();
    let both =
        CredentialStatusFreshness::new(Some(revision.clone()), Some(DurationMillis::new(30_000)))
            .unwrap();
    assert_eq!(both.minimum_revision(), Some(&revision));
    assert_eq!(both.maximum_age(), Some(DurationMillis::new(30_000)));
}

#[test]
fn requirements_distinguish_unrestricted_from_invalid_present_lists() {
    let unrestricted = CredentialStatusRequirements::unrestricted();
    assert_eq!(unrestricted.accepted_methods(), None);
    assert_eq!(unrestricted.accepted_purposes(), None);
    assert_eq!(unrestricted.freshness(), None);

    assert_eq!(
        CredentialStatusRequirements::new(Some(Vec::new()), None, None),
        Err(CredentialError::InvalidStatusRequirements)
    );
    assert_eq!(
        CredentialStatusRequirements::new(None, Some(Vec::new()), None),
        Err(CredentialError::InvalidStatusRequirements)
    );
    let repeated_method = method("BitstringStatusListEntry");
    assert_eq!(
        CredentialStatusRequirements::new(
            Some(vec![repeated_method.clone(), repeated_method]),
            None,
            None
        ),
        Err(CredentialError::DuplicateStatusMethod)
    );
    let repeated_purpose = purpose("revocation");
    assert_eq!(
        CredentialStatusRequirements::new(
            None,
            Some(vec![repeated_purpose.clone(), repeated_purpose]),
            None
        ),
        Err(CredentialError::DuplicateStatusPurpose)
    );
    let oversized = (0..=MAX_CREDENTIAL_STATUS_REQUIREMENT_VALUES)
        .map(|index| method(&format!("example:method:{index}")))
        .collect();
    assert_eq!(
        CredentialStatusRequirements::new(Some(oversized), None, None),
        Err(CredentialError::InvalidStatusRequirements)
    );
}

#[test]
fn complete_query_carries_binding_and_structural_requirements() {
    let binding = binding(
        "BitstringStatusListEntry",
        "revocation",
        "https://status.example/lists/1",
        "42",
    );
    let freshness =
        CredentialStatusFreshness::new(None, Some(DurationMillis::new(5 * 60 * 1_000))).unwrap();
    let requirements = CredentialStatusRequirements::new(
        Some(vec![method("BitstringStatusListEntry")]),
        Some(vec![purpose("revocation")]),
        Some(freshness),
    )
    .unwrap();
    let query = CredentialStatusQuery::new(binding, requirements).unwrap();

    assert_eq!(query.binding().handle().as_text(), Some("42"));
    assert_eq!(
        query.requirements().accepted_methods().unwrap()[0].as_str(),
        "BitstringStatusListEntry"
    );
    assert_eq!(
        query.requirements().freshness().unwrap().maximum_age(),
        Some(DurationMillis::new(300_000))
    );
}

#[test]
fn query_rejects_a_binding_excluded_by_its_own_allow_lists() {
    let binding = binding(
        "BitstringStatusListEntry",
        "revocation",
        "https://status.example/list",
        "42",
    );
    let wrong_method =
        CredentialStatusRequirements::new(Some(vec![method("example:other-method")]), None, None)
            .unwrap();
    assert_eq!(
        CredentialStatusQuery::new(binding.clone(), wrong_method),
        Err(CredentialError::StatusQueryMismatch)
    );

    let wrong_purpose =
        CredentialStatusRequirements::new(None, Some(vec![purpose("suspension")]), None).unwrap();
    assert_eq!(
        CredentialStatusQuery::new(binding, wrong_purpose),
        Err(CredentialError::StatusQueryMismatch)
    );
}

#[test]
fn evidence_rejects_only_a_reversed_known_time_range() {
    let binding = binding(
        "BitstringStatusListEntry",
        "revocation",
        "https://status.example/list",
        "7",
    );
    let value = CredentialStatusValue::from_text("0").unwrap();
    assert_eq!(
        CredentialStatusEvidence::new(
            binding.clone(),
            value.clone(),
            None,
            Some(UnixTimestampMillis::new(20)),
            Some(UnixTimestampMillis::new(10)),
        ),
        Err(CredentialError::InvalidStatusEvidenceRange)
    );
    assert!(
        CredentialStatusEvidence::new(
            binding.clone(),
            value.clone(),
            None,
            Some(UnixTimestampMillis::new(10)),
            Some(UnixTimestampMillis::new(10)),
        )
        .is_ok()
    );
    assert!(CredentialStatusEvidence::new(binding, value, None, None, None).is_ok());
}

#[test]
fn direct_and_aggregate_debug_redact_all_opaque_canaries() {
    let reference_canary = "https://status.example/private/reference-canary";
    let handle_canary = "holder-index-canary";
    let revision_canary = "private-revision-canary";
    let value_canary = "private-status-value-canary";
    let binding = CredentialStatusBinding::new(
        method("example:status"),
        purpose("example:purpose"),
        CredentialStatusReference::from_text(reference_canary).unwrap(),
        CredentialStatusHandle::from_text(handle_canary).unwrap(),
    );
    let freshness = CredentialStatusFreshness::new(
        Some(CredentialStatusRevision::from_text(revision_canary).unwrap()),
        None,
    )
    .unwrap();
    let query = CredentialStatusQuery::new(
        binding.clone(),
        CredentialStatusRequirements::new(None, None, Some(freshness)).unwrap(),
    )
    .unwrap();
    let evidence = CredentialStatusEvidence::new(
        binding.clone(),
        CredentialStatusValue::from_text(value_canary).unwrap(),
        Some(CredentialStatusRevision::from_text(revision_canary).unwrap()),
        None,
        None,
    )
    .unwrap();
    let collection = CredentialStatusBindings::new(vec![binding]).unwrap();

    let rendered = format!("{query:?} {evidence:?} {collection:?}");
    for canary in [
        reference_canary,
        handle_canary,
        revision_canary,
        value_canary,
    ] {
        assert!(!rendered.contains(canary));
    }
    assert!(rendered.contains("representation"));
    assert!(rendered.contains("binding_count"));
}

#[test]
fn every_status_error_bridges_to_static_credential_contract() {
    let cases = [
        (
            CredentialError::InvalidStatusMethod,
            "credential.invalid_status_method",
        ),
        (
            CredentialError::InvalidStatusPurpose,
            "credential.invalid_status_purpose",
        ),
        (
            CredentialError::InvalidStatusReference,
            "credential.invalid_status_reference",
        ),
        (
            CredentialError::InvalidStatusHandle,
            "credential.invalid_status_handle",
        ),
        (
            CredentialError::InvalidStatusRevision,
            "credential.invalid_status_revision",
        ),
        (
            CredentialError::InvalidStatusValue,
            "credential.invalid_status_value",
        ),
        (
            CredentialError::InvalidStatusBindingCollection,
            "credential.invalid_status_binding_collection",
        ),
        (
            CredentialError::DuplicateStatusBinding,
            "credential.duplicate_status_binding",
        ),
        (
            CredentialError::InvalidStatusFreshness,
            "credential.invalid_status_freshness",
        ),
        (
            CredentialError::InvalidStatusRequirements,
            "credential.invalid_status_requirements",
        ),
        (
            CredentialError::DuplicateStatusMethod,
            "credential.duplicate_status_method",
        ),
        (
            CredentialError::DuplicateStatusPurpose,
            "credential.duplicate_status_purpose",
        ),
        (
            CredentialError::InvalidStatusEvidenceRange,
            "credential.invalid_status_evidence_range",
        ),
        (
            CredentialError::StatusQueryMismatch,
            "credential.status_query_mismatch",
        ),
    ];

    for (error, expected_code) in cases {
        let public = error.to_identus_error();
        assert_eq!(public.capability().unwrap().as_str(), "credential");
        assert_eq!(public.kind(), ErrorKind::InvalidInput);
        assert_eq!(public.code().as_str(), expected_code);
        assert!(!error.to_string().contains("caller-controlled-canary"));
        assert!(!public.to_string().contains("caller-controlled-canary"));
    }
}

#[test]
#[ignore = "manual release-mode throughput observation"]
fn credential_status_construction_throughput_diagnostic() {
    const ITERATIONS: u32 = 500_000;
    let started = Instant::now();
    for index in 0..ITERATIONS {
        let binding = CredentialStatusBinding::new(
            method("BitstringStatusListEntry"),
            purpose("revocation"),
            reference("https://status.example/list"),
            handle("42"),
        );
        let requirements = CredentialStatusRequirements::new(
            None,
            None,
            Some(
                CredentialStatusFreshness::new(None, Some(DurationMillis::new(u64::from(index))))
                    .unwrap(),
            ),
        )
        .unwrap();
        let query = CredentialStatusQuery::new(binding.clone(), requirements).unwrap();
        let evidence = CredentialStatusEvidence::new(
            binding,
            CredentialStatusValue::from_text("0").unwrap(),
            None,
            None,
            None,
        )
        .unwrap();
        let _ = black_box((query, evidence));
    }
    let elapsed = started.elapsed();
    let constructions_per_second = f64::from(ITERATIONS) / elapsed.as_secs_f64();
    eprintln!(
        "credential status construction: {ITERATIONS} query/evidence pairs in {elapsed:?} ({constructions_per_second:.0} pairs/s)"
    );
}
