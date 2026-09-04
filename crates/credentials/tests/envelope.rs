use std::str::FromStr;

use identus_core::ErrorKind;
use identus_credentials::{
    CredentialDetachedProof, CredentialEnvelope, CredentialError, CredentialFormat,
    CredentialPayload, CredentialPrivateMaterial, MAX_CREDENTIAL_DETACHED_PROOF_BYTES,
    MAX_CREDENTIAL_FORMAT_BYTES, MAX_CREDENTIAL_PAYLOAD_BYTES,
    MAX_CREDENTIAL_PRIVATE_MATERIAL_BYTES,
};
use zeroize::{Zeroize, ZeroizeOnDrop};

fn assert_zeroize_contract<T: Zeroize + ZeroizeOnDrop>() {}

#[test]
fn open_format_identifiers_preserve_exact_spelling() {
    for value in [
        "vc+sd-jwt",
        "mso_mdoc",
        "midnight_cbor_phase1",
        "Example.Format:v1",
    ] {
        let format = CredentialFormat::from_str(value).expect("valid format");
        assert_eq!(format.as_str(), value);
        assert_eq!(format.to_string(), value);
    }
}

#[test]
fn invalid_format_identifiers_are_rejected_without_echoing_input() {
    let oversized = "a".repeat(MAX_CREDENTIAL_FORMAT_BYTES + 1);
    for value in [
        "",
        "-leading",
        "has space",
        "line\nbreak",
        "slash/value",
        "emoji-🔐",
        oversized.as_str(),
    ] {
        let error = CredentialFormat::parse(value).expect_err("invalid format");
        assert_eq!(error, CredentialError::InvalidFormat);
        if !value.is_empty() {
            assert!(!error.to_string().contains(value));
            assert!(!error.to_identus_error().to_string().contains(value));
        }
    }
}

#[test]
fn credential_payload_accepts_exact_boundaries_and_preserves_bytes() {
    let one = CredentialPayload::new(vec![0xa1]).expect("one byte");
    assert_eq!(one.as_bytes(), &[0xa1]);

    let maximum = vec![0x5a; MAX_CREDENTIAL_PAYLOAD_BYTES];
    let payload = CredentialPayload::new(maximum.clone()).expect("maximum payload");
    assert_eq!(payload.as_bytes(), maximum);
}

#[test]
fn detached_proof_accepts_exact_boundaries_and_preserves_bytes() {
    let one = CredentialDetachedProof::new(vec![0x01]).expect("one byte");
    assert_eq!(one.as_bytes(), &[0x01]);

    let maximum = vec![0x5b; MAX_CREDENTIAL_DETACHED_PROOF_BYTES];
    let proof = CredentialDetachedProof::new(maximum.clone()).expect("maximum proof");
    assert_eq!(proof.as_bytes(), maximum);
}

#[test]
fn private_material_accepts_exact_boundaries_and_preserves_bytes() {
    let one = CredentialPrivateMaterial::new(vec![0x02]).expect("one byte");
    assert_eq!(one.as_bytes(), &[0x02]);

    let maximum = vec![0x5c; MAX_CREDENTIAL_PRIVATE_MATERIAL_BYTES];
    let material = CredentialPrivateMaterial::new(maximum.clone()).expect("maximum material");
    assert_eq!(material.as_bytes(), maximum);
}

#[test]
fn artifacts_reject_empty_and_oversized_values() {
    assert_eq!(
        CredentialPayload::new(Vec::new()),
        Err(CredentialError::EmptyPayload)
    );
    assert_eq!(
        CredentialPayload::new(vec![0; MAX_CREDENTIAL_PAYLOAD_BYTES + 1]),
        Err(CredentialError::PayloadTooLarge)
    );
    assert_eq!(
        CredentialDetachedProof::new(Vec::new()),
        Err(CredentialError::EmptyDetachedProof)
    );
    assert_eq!(
        CredentialDetachedProof::new(vec![0; MAX_CREDENTIAL_DETACHED_PROOF_BYTES + 1]),
        Err(CredentialError::DetachedProofTooLarge)
    );
    assert!(matches!(
        CredentialPrivateMaterial::new(Vec::new()),
        Err(CredentialError::EmptyPrivateMaterial)
    ));
    assert!(matches!(
        CredentialPrivateMaterial::new(vec![0; MAX_CREDENTIAL_PRIVATE_MATERIAL_BYTES + 1]),
        Err(CredentialError::PrivateMaterialTooLarge)
    ));
}

#[test]
fn private_material_is_zeroizing_and_artifact_debug_is_redacted() {
    assert_zeroize_contract::<CredentialPrivateMaterial>();

    let mut material =
        CredentialPrivateMaterial::new(b"opening-secret".to_vec()).expect("private material");
    let payload = CredentialPayload::new(b"credential-pii".to_vec()).expect("payload");
    let proof = CredentialDetachedProof::new(b"proof-value".to_vec()).expect("proof");

    assert_eq!(
        format!("{material:?}"),
        "CredentialPrivateMaterial { length: 14, .. }"
    );
    assert!(!format!("{payload:?}").contains("credential-pii"));
    assert!(!format!("{proof:?}").contains("proof-value"));

    material.zeroize();
    assert!(material.as_bytes().is_empty());
}

#[test]
fn unrelated_formats_use_the_same_non_validating_envelope() {
    let midnight = CredentialEnvelope::new(
        CredentialFormat::parse("midnight_cbor_phase1").expect("format"),
        CredentialPayload::new(vec![0xa1, 0x61, b'a', 0x01]).expect("payload"),
    )
    .with_detached_proof(CredentialDetachedProof::new(vec![0x11, 0x22]).expect("proof"))
    .with_private_material(
        CredentialPrivateMaterial::new(b"claim-opening".to_vec()).expect("private material"),
    );

    let arbitrary = CredentialEnvelope::new(
        CredentialFormat::parse("example+opaque:v1").expect("format"),
        CredentialPayload::new(vec![0xff, 0x00, 0xfe]).expect("opaque payload"),
    );

    assert_eq!(midnight.format().as_str(), "midnight_cbor_phase1");
    assert_eq!(midnight.payload().as_bytes(), &[0xa1, 0x61, b'a', 0x01]);
    assert_eq!(
        midnight
            .detached_proof()
            .expect("detached proof")
            .as_bytes(),
        &[0x11, 0x22]
    );
    assert_eq!(
        midnight
            .private_material()
            .expect("private material")
            .as_bytes(),
        b"claim-opening"
    );
    assert_eq!(arbitrary.payload().as_bytes(), &[0xff, 0x00, 0xfe]);
    assert!(arbitrary.detached_proof().is_none());
    assert!(arbitrary.private_material().is_none());

    let debug = format!("{midnight:?}");
    assert!(debug.contains("midnight_cbor_phase1"));
    assert!(debug.contains("payload_length: 4"));
    assert!(!debug.contains("claim-opening"));
}

#[test]
fn envelope_errors_bridge_to_static_credential_codes() {
    let cases = [
        (CredentialError::InvalidFormat, "credential.invalid_format"),
        (CredentialError::EmptyPayload, "credential.empty_payload"),
        (
            CredentialError::PayloadTooLarge,
            "credential.payload_too_large",
        ),
        (
            CredentialError::EmptyDetachedProof,
            "credential.empty_detached_proof",
        ),
        (
            CredentialError::DetachedProofTooLarge,
            "credential.detached_proof_too_large",
        ),
        (
            CredentialError::EmptyPrivateMaterial,
            "credential.empty_private_material",
        ),
        (
            CredentialError::PrivateMaterialTooLarge,
            "credential.private_material_too_large",
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
fn component_marks_the_crate_as_implemented_semantics() {
    assert_eq!(identus_credentials::COMPONENT.name, "identus-credentials");
    assert!(identus_credentials::COMPONENT.summary.contains("Bounded"));
    assert!(
        !identus_credentials::COMPONENT
            .summary
            .contains("Quarantined")
    );
}
