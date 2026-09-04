use std::{hint::black_box, str::FromStr, time::Instant};

use identus_core::{ErrorKind, UnixTimestampMillis};
use identus_credentials::{
    CredentialClaimDescriptor, CredentialClaimDisclosure, CredentialClaimId, CredentialClaimPath,
    CredentialClaimPathSegment, CredentialClaimValueType, CredentialEntityId, CredentialError,
    CredentialMetadata, CredentialSchemaDescriptor, CredentialSchemaId, CredentialSchemaVersion,
    CredentialType, MAX_CREDENTIAL_CLAIM_ID_BYTES, MAX_CREDENTIAL_CLAIM_PATH_SEGMENT_BYTES,
    MAX_CREDENTIAL_CLAIM_PATH_SEGMENTS, MAX_CREDENTIAL_CLAIM_VALUE_TYPE_BYTES,
    MAX_CREDENTIAL_ENTITY_ID_BYTES, MAX_CREDENTIAL_SCHEMA_CLAIMS, MAX_CREDENTIAL_SCHEMA_ID_BYTES,
    MAX_CREDENTIAL_SCHEMA_VERSION_BYTES, MAX_CREDENTIAL_SCHEMAS, MAX_CREDENTIAL_SUBJECTS,
    MAX_CREDENTIAL_TYPE_BYTES, MAX_CREDENTIAL_TYPES,
};

fn entity(value: &str) -> CredentialEntityId {
    CredentialEntityId::parse(value).expect("valid entity identifier")
}

fn credential_type(value: &str) -> CredentialType {
    CredentialType::parse(value).expect("valid credential type")
}

fn schema_id(value: &str) -> CredentialSchemaId {
    CredentialSchemaId::parse(value).expect("valid schema identifier")
}

fn segment(value: &str) -> CredentialClaimPathSegment {
    CredentialClaimPathSegment::parse(value).expect("valid claim path segment")
}

fn claim(
    id: &str,
    path: &[&str],
    disclosure: CredentialClaimDisclosure,
) -> CredentialClaimDescriptor {
    CredentialClaimDescriptor::new(
        CredentialClaimId::parse(id).expect("valid claim identifier"),
        CredentialClaimPath::new(path.iter().map(|value| segment(value)).collect())
            .expect("valid claim path"),
        disclosure,
        true,
        None,
    )
}

fn schema(id: &str) -> CredentialSchemaDescriptor {
    CredentialSchemaDescriptor::new(
        schema_id(id),
        Some(CredentialSchemaVersion::parse("1.0").expect("version")),
        vec![credential_type("ExampleCredential")],
        vec![claim(
            "example",
            &["credentialSubject", "example"],
            CredentialClaimDisclosure::Public,
        )],
    )
    .expect("valid schema")
}

#[test]
fn descriptor_roles_preserve_unrelated_identifier_dialects() {
    assert_eq!(
        entity("did:midnight:undeployed:issuer").as_str(),
        "did:midnight:undeployed:issuer"
    );
    assert_eq!(
        entity("https://issuer.example/issuers/14").as_str(),
        "https://issuer.example/issuers/14"
    );
    assert_eq!(
        schema_id("digital-passport:v1").as_str(),
        "digital-passport:v1"
    );
    assert_eq!(
        credential_type("https://credentials.example/types/EmployeeCredential").as_str(),
        "https://credentials.example/types/EmployeeCredential"
    );
    assert_eq!(
        credential_type("urn:ietf:params:oauth:token-type:jwt").as_str(),
        "urn:ietf:params:oauth:token-type:jwt"
    );
}

#[test]
fn every_descriptor_accepts_exact_byte_limit() {
    let entity_value = "e".repeat(MAX_CREDENTIAL_ENTITY_ID_BYTES);
    let type_value = "t".repeat(MAX_CREDENTIAL_TYPE_BYTES);
    let schema_value = "s".repeat(MAX_CREDENTIAL_SCHEMA_ID_BYTES);
    let version_value = "v".repeat(MAX_CREDENTIAL_SCHEMA_VERSION_BYTES);
    let claim_value = "c".repeat(MAX_CREDENTIAL_CLAIM_ID_BYTES);
    let value_type = "y".repeat(MAX_CREDENTIAL_CLAIM_VALUE_TYPE_BYTES);
    let path_segment = "p".repeat(MAX_CREDENTIAL_CLAIM_PATH_SEGMENT_BYTES);

    assert_eq!(
        CredentialEntityId::parse(&entity_value).unwrap().as_str(),
        entity_value
    );
    assert_eq!(
        CredentialType::parse(&type_value).unwrap().as_str(),
        type_value
    );
    assert_eq!(
        CredentialSchemaId::parse(&schema_value).unwrap().as_str(),
        schema_value
    );
    assert_eq!(
        CredentialSchemaVersion::parse(&version_value)
            .unwrap()
            .as_str(),
        version_value
    );
    assert_eq!(
        CredentialClaimId::parse(&claim_value).unwrap().as_str(),
        claim_value
    );
    assert_eq!(
        CredentialClaimValueType::parse(&value_type)
            .unwrap()
            .as_str(),
        value_type
    );
    assert_eq!(
        CredentialClaimPathSegment::parse(&path_segment)
            .unwrap()
            .as_str(),
        path_segment
    );
}

#[test]
fn descriptor_text_rejects_empty_padded_control_and_oversized_values() {
    let oversized = "x".repeat(MAX_CREDENTIAL_ENTITY_ID_BYTES + 1);
    for (value, error) in [
        ("", CredentialError::InvalidEntityIdentifier),
        (" padded", CredentialError::InvalidEntityIdentifier),
        ("padded ", CredentialError::InvalidEntityIdentifier),
        ("line\nbreak", CredentialError::InvalidEntityIdentifier),
        (oversized.as_str(), CredentialError::InvalidEntityIdentifier),
    ] {
        let actual = CredentialEntityId::parse(value).expect_err("invalid entity identifier");
        assert_eq!(actual, error);
        if !value.is_empty() {
            assert!(!actual.to_string().contains(value));
            assert!(!actual.to_identus_error().to_string().contains(value));
        }
    }

    assert_eq!(
        CredentialType::parse(&"x".repeat(MAX_CREDENTIAL_TYPE_BYTES + 1)),
        Err(CredentialError::InvalidCredentialType)
    );
    assert_eq!(
        CredentialSchemaId::parse(&"x".repeat(MAX_CREDENTIAL_SCHEMA_ID_BYTES + 1)),
        Err(CredentialError::InvalidSchemaIdentifier)
    );
    assert_eq!(
        CredentialSchemaVersion::parse(&"x".repeat(MAX_CREDENTIAL_SCHEMA_VERSION_BYTES + 1)),
        Err(CredentialError::InvalidSchemaVersion)
    );
    assert_eq!(
        CredentialClaimId::parse(&"x".repeat(MAX_CREDENTIAL_CLAIM_ID_BYTES + 1)),
        Err(CredentialError::InvalidClaimIdentifier)
    );
    assert_eq!(
        CredentialClaimValueType::parse(&"x".repeat(MAX_CREDENTIAL_CLAIM_VALUE_TYPE_BYTES + 1)),
        Err(CredentialError::InvalidClaimValueType)
    );
    assert_eq!(
        CredentialClaimPathSegment::parse(&"x".repeat(MAX_CREDENTIAL_CLAIM_PATH_SEGMENT_BYTES + 1)),
        Err(CredentialError::InvalidClaimPathSegment)
    );
}

#[test]
fn disclosure_modes_round_trip_and_unknown_names_fail() {
    for mode in [
        CredentialClaimDisclosure::Public,
        CredentialClaimDisclosure::Selective,
        CredentialClaimDisclosure::Committed,
        CredentialClaimDisclosure::PredicateOnly,
    ] {
        assert_eq!(CredentialClaimDisclosure::from_str(mode.as_str()), Ok(mode));
        assert_eq!(mode.to_string(), mode.as_str());
    }
    assert_eq!(
        CredentialClaimDisclosure::parse("predicate-only"),
        Err(CredentialError::InvalidClaimDisclosure)
    );
}

#[test]
fn claim_paths_enforce_segment_count_bounds() {
    assert_eq!(
        CredentialClaimPath::new(Vec::new()),
        Err(CredentialError::InvalidClaimPath)
    );
    let maximum = CredentialClaimPath::new(
        (0..MAX_CREDENTIAL_CLAIM_PATH_SEGMENTS)
            .map(|index| segment(&format!("segment{index}")))
            .collect(),
    )
    .expect("maximum path");
    assert_eq!(maximum.segments().len(), MAX_CREDENTIAL_CLAIM_PATH_SEGMENTS);
    assert_eq!(
        CredentialClaimPath::new(
            (0..=MAX_CREDENTIAL_CLAIM_PATH_SEGMENTS)
                .map(|index| segment(&format!("segment{index}")))
                .collect()
        ),
        Err(CredentialError::InvalidClaimPath)
    );
}

#[test]
fn midnight_schema_preserves_committed_and_predicate_claim_shape() {
    let date_of_birth = CredentialClaimDescriptor::new(
        CredentialClaimId::parse("dateOfBirth").unwrap(),
        CredentialClaimPath::new(vec![segment("credentialSubject"), segment("dateOfBirth")])
            .unwrap(),
        CredentialClaimDisclosure::Committed,
        true,
        Some(CredentialClaimValueType::parse("date").unwrap()),
    );
    let age_over = claim(
        "ageOver18",
        &["credentialSubject", "ageOver18"],
        CredentialClaimDisclosure::PredicateOnly,
    );
    let descriptor = CredentialSchemaDescriptor::new(
        schema_id("birth-schema"),
        Some(CredentialSchemaVersion::parse("1.0.0").unwrap()),
        vec![credential_type("BirthCredential")],
        vec![date_of_birth, age_over],
    )
    .expect("Midnight-shaped schema");

    assert_eq!(descriptor.id().as_str(), "birth-schema");
    assert_eq!(descriptor.version().unwrap().as_str(), "1.0.0");
    assert_eq!(descriptor.claims().len(), 2);
    assert_eq!(
        descriptor.claims()[0].disclosure(),
        CredentialClaimDisclosure::Committed
    );
    assert_eq!(
        descriptor.claims()[0].value_type().unwrap().as_str(),
        "date"
    );
}

#[test]
fn w3c_schema_and_metadata_preserve_multiple_subjects_and_validity() {
    let descriptor = CredentialSchemaDescriptor::new(
        schema_id("https://example.org/examples/degree.json"),
        None,
        vec![
            credential_type("VerifiableCredential"),
            credential_type("ExampleDegreeCredential"),
        ],
        vec![claim(
            "degree",
            &["credentialSubject", "degree"],
            CredentialClaimDisclosure::Public,
        )],
    )
    .unwrap();
    let metadata = CredentialMetadata::new(
        entity("https://university.example/issuers/14"),
        vec![entity("did:example:alice"), entity("did:example:bob")],
        descriptor.credential_types().to_vec(),
        vec![descriptor],
        Some(UnixTimestampMillis::new(1_262_370_204_000)),
        Some(UnixTimestampMillis::new(1_577_843_404_000)),
    )
    .expect("W3C-shaped metadata");

    assert_eq!(metadata.subjects().len(), 2);
    assert_eq!(metadata.credential_types().len(), 2);
    assert_eq!(metadata.schemas().len(), 1);
    assert_eq!(
        metadata.valid_from(),
        Some(UnixTimestampMillis::new(1_262_370_204_000))
    );
    assert_eq!(
        metadata.valid_until(),
        Some(UnixTimestampMillis::new(1_577_843_404_000))
    );
}

#[test]
fn lace_shape_accepts_local_schema_version_without_semver_policy() {
    let descriptor = CredentialSchemaDescriptor::new(
        schema_id("digital-passport:v1"),
        Some(CredentialSchemaVersion::parse("1.0").unwrap()),
        vec![credential_type("digital-passport")],
        Vec::new(),
    )
    .unwrap();
    let metadata = CredentialMetadata::new(
        entity("did:midnight:undeployed:issuer"),
        Vec::new(),
        vec![credential_type("digital-passport")],
        vec![descriptor],
        None,
        Some(UnixTimestampMillis::new(4_102_444_800_000)),
    )
    .expect("Lace-shaped metadata");

    assert!(metadata.subjects().is_empty());
    assert_eq!(metadata.schemas()[0].version().unwrap().as_str(), "1.0");
}

#[test]
fn schemas_reject_oversized_and_ambiguous_collections() {
    let too_many_types = (0..=MAX_CREDENTIAL_TYPES)
        .map(|index| credential_type(&format!("Type{index}")))
        .collect();
    assert_eq!(
        CredentialSchemaDescriptor::new(schema_id("schema"), None, too_many_types, Vec::new()),
        Err(CredentialError::InvalidDescriptorCollection)
    );
    let too_many_claims = (0..=MAX_CREDENTIAL_SCHEMA_CLAIMS)
        .map(|index| {
            claim(
                &format!("claim{index}"),
                &[&format!("path{index}")],
                CredentialClaimDisclosure::Public,
            )
        })
        .collect();
    assert_eq!(
        CredentialSchemaDescriptor::new(
            schema_id("schema"),
            None,
            vec![credential_type("Type")],
            too_many_claims
        ),
        Err(CredentialError::InvalidDescriptorCollection)
    );
    assert_eq!(
        CredentialSchemaDescriptor::new(
            schema_id("schema"),
            None,
            vec![credential_type("Type"), credential_type("Type")],
            Vec::new()
        ),
        Err(CredentialError::DuplicateCredentialType)
    );

    let duplicate_id = vec![
        claim("same", &["one"], CredentialClaimDisclosure::Public),
        claim("same", &["two"], CredentialClaimDisclosure::Selective),
    ];
    assert_eq!(
        CredentialSchemaDescriptor::new(
            schema_id("schema"),
            None,
            vec![credential_type("Type")],
            duplicate_id
        ),
        Err(CredentialError::DuplicateClaimIdentifier)
    );
    let duplicate_path = vec![
        claim("one", &["same"], CredentialClaimDisclosure::Public),
        claim("two", &["same"], CredentialClaimDisclosure::Selective),
    ];
    assert_eq!(
        CredentialSchemaDescriptor::new(
            schema_id("schema"),
            None,
            vec![credential_type("Type")],
            duplicate_path
        ),
        Err(CredentialError::DuplicateClaimPath)
    );
}

#[test]
fn exact_collection_boundaries_are_accepted() {
    let types = (0..MAX_CREDENTIAL_TYPES)
        .map(|index| credential_type(&format!("Type{index}")))
        .collect::<Vec<_>>();
    let claims = (0..MAX_CREDENTIAL_SCHEMA_CLAIMS)
        .map(|index| {
            claim(
                &format!("claim{index}"),
                &[&format!("path{index}")],
                CredentialClaimDisclosure::Public,
            )
        })
        .collect::<Vec<_>>();
    let boundary_schema =
        CredentialSchemaDescriptor::new(schema_id("boundary-schema"), None, types.clone(), claims)
            .expect("maximum schema collections");
    assert_eq!(
        boundary_schema.credential_types().len(),
        MAX_CREDENTIAL_TYPES
    );
    assert_eq!(boundary_schema.claims().len(), MAX_CREDENTIAL_SCHEMA_CLAIMS);

    let subjects = (0..MAX_CREDENTIAL_SUBJECTS)
        .map(|index| entity(&format!("subject{index}")))
        .collect();
    let schemas = (0..MAX_CREDENTIAL_SCHEMAS)
        .map(|index| {
            CredentialSchemaDescriptor::new(
                schema_id(&format!("schema{index}")),
                None,
                vec![credential_type("Type")],
                Vec::new(),
            )
            .unwrap()
        })
        .collect();
    let metadata = CredentialMetadata::new(
        entity("issuer"),
        subjects,
        types,
        schemas,
        Some(UnixTimestampMillis::new(1)),
        Some(UnixTimestampMillis::new(1)),
    )
    .expect("maximum metadata collections and inclusive validity");
    assert_eq!(metadata.subjects().len(), MAX_CREDENTIAL_SUBJECTS);
    assert_eq!(metadata.schemas().len(), MAX_CREDENTIAL_SCHEMAS);
}

#[test]
fn metadata_rejects_bounds_duplicates_and_reversed_validity() {
    assert_eq!(
        CredentialMetadata::new(
            entity("issuer"),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            None,
            None
        ),
        Err(CredentialError::InvalidDescriptorCollection)
    );
    assert_eq!(
        CredentialMetadata::new(
            entity("issuer"),
            vec![entity("subject"), entity("subject")],
            vec![credential_type("Type")],
            Vec::new(),
            None,
            None
        ),
        Err(CredentialError::DuplicateCredentialSubject)
    );
    assert_eq!(
        CredentialMetadata::new(
            entity("issuer"),
            Vec::new(),
            vec![credential_type("Type"), credential_type("Type")],
            Vec::new(),
            None,
            None
        ),
        Err(CredentialError::DuplicateCredentialType)
    );
    assert_eq!(
        CredentialMetadata::new(
            entity("issuer"),
            Vec::new(),
            vec![credential_type("Type")],
            vec![schema("same"), schema("same")],
            None,
            None
        ),
        Err(CredentialError::DuplicateSchemaIdentifier)
    );
    assert_eq!(
        CredentialMetadata::new(
            entity("issuer"),
            Vec::new(),
            vec![credential_type("Type")],
            Vec::new(),
            Some(UnixTimestampMillis::new(2)),
            Some(UnixTimestampMillis::new(1))
        ),
        Err(CredentialError::InvalidValidityRange)
    );

    let subjects = (0..=MAX_CREDENTIAL_SUBJECTS)
        .map(|index| entity(&format!("subject{index}")))
        .collect();
    assert_eq!(
        CredentialMetadata::new(
            entity("issuer"),
            subjects,
            vec![credential_type("Type")],
            Vec::new(),
            None,
            None
        ),
        Err(CredentialError::InvalidDescriptorCollection)
    );
    let schemas = (0..=MAX_CREDENTIAL_SCHEMAS)
        .map(|index| schema(&format!("schema{index}")))
        .collect();
    assert_eq!(
        CredentialMetadata::new(
            entity("issuer"),
            Vec::new(),
            vec![credential_type("Type")],
            schemas,
            None,
            None
        ),
        Err(CredentialError::InvalidDescriptorCollection)
    );
}

#[test]
fn entity_and_metadata_debug_redact_correlating_identifiers() {
    let metadata = CredentialMetadata::new(
        entity("did:example:known-issuer"),
        vec![entity("did:example:known-subject")],
        vec![credential_type("ExampleCredential")],
        Vec::new(),
        None,
        None,
    )
    .unwrap();

    let entity_debug = format!("{:?}", metadata.issuer());
    let metadata_debug = format!("{metadata:?}");
    for rendered in [&entity_debug, &metadata_debug] {
        assert!(!rendered.contains("known-issuer"));
        assert!(!rendered.contains("known-subject"));
    }
    assert!(metadata_debug.contains("subject_count: 1"));
}

#[test]
fn metadata_errors_bridge_to_static_credential_codes() {
    let cases = [
        (
            CredentialError::InvalidEntityIdentifier,
            "credential.invalid_entity_identifier",
        ),
        (
            CredentialError::InvalidCredentialType,
            "credential.invalid_credential_type",
        ),
        (
            CredentialError::InvalidSchemaIdentifier,
            "credential.invalid_schema_identifier",
        ),
        (
            CredentialError::InvalidSchemaVersion,
            "credential.invalid_schema_version",
        ),
        (
            CredentialError::InvalidClaimIdentifier,
            "credential.invalid_claim_identifier",
        ),
        (
            CredentialError::InvalidClaimValueType,
            "credential.invalid_claim_value_type",
        ),
        (
            CredentialError::InvalidClaimPathSegment,
            "credential.invalid_claim_path_segment",
        ),
        (
            CredentialError::InvalidClaimPath,
            "credential.invalid_claim_path",
        ),
        (
            CredentialError::InvalidClaimDisclosure,
            "credential.invalid_claim_disclosure",
        ),
        (
            CredentialError::InvalidDescriptorCollection,
            "credential.invalid_descriptor_collection",
        ),
        (
            CredentialError::DuplicateCredentialSubject,
            "credential.duplicate_credential_subject",
        ),
        (
            CredentialError::DuplicateCredentialType,
            "credential.duplicate_credential_type",
        ),
        (
            CredentialError::DuplicateSchemaIdentifier,
            "credential.duplicate_schema_identifier",
        ),
        (
            CredentialError::DuplicateClaimIdentifier,
            "credential.duplicate_claim_identifier",
        ),
        (
            CredentialError::DuplicateClaimPath,
            "credential.duplicate_claim_path",
        ),
        (
            CredentialError::InvalidValidityRange,
            "credential.invalid_validity_range",
        ),
    ];

    for (error, expected_code) in cases {
        let public = error.to_identus_error();
        assert_eq!(public.capability().unwrap().as_str(), "credential");
        assert_eq!(public.kind(), ErrorKind::InvalidInput);
        assert_eq!(public.code().as_str(), expected_code);
        assert!(!public.to_string().contains("caller-controlled-value"));
    }
}

#[test]
#[ignore = "manual release-mode throughput observation"]
fn metadata_construction_throughput_diagnostic() {
    const ITERATIONS: u32 = 100_000;
    let started = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = black_box(
            CredentialMetadata::new(
                entity("did:example:issuer"),
                vec![entity("did:example:subject")],
                vec![credential_type("ExampleCredential")],
                vec![schema("example-schema")],
                Some(UnixTimestampMillis::new(1)),
                Some(UnixTimestampMillis::new(2)),
            )
            .expect("valid metadata"),
        );
    }
    let elapsed = started.elapsed();
    let operations_per_second = f64::from(ITERATIONS) / elapsed.as_secs_f64();
    eprintln!(
        "credential metadata construction: {ITERATIONS} values in {elapsed:?} ({operations_per_second:.0} operations/s)"
    );
}
