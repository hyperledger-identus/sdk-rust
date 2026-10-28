use std::collections::BTreeSet;

use identus_crypto::{Ed25519PrivateKey, EncodeJwk, SecureRandom};
use identus_jose::{Ed25519Signer, JwsAlgorithm, JwsVerificationKey, SignatureSuiteRegistry};
use identus_sd_jwt::{
    ClaimPath, IssuanceOptions, IssuancePlan, PathSegment, SdJwtLimits, issue_with_options,
};
use identus_sd_jwt_vc::{
    ClaimDisclosureRule, ClaimMetadata, Draft22SdJwtVc, ProfileValidationInputs, SdJwtVcError,
    SdJwtVcLimits, TypeMetadata, TypeMetadataChain, issue_draft22, parse_draft22_envelope,
};
use serde::Deserialize;
use serde_json::{Value, json};

const PACKET: &str = include_str!("fixtures/differential-v1/packet.json");
const RECEIPTS: &str = include_str!("fixtures/differential-v1/oracle-receipts.json");
const DRAFT_SHA256: &str = "da2ade6128e9088e28e482866aa744f8287ac7eb6f11a5713bb2166827299124";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Packet {
    schema_version: u8,
    packet_id: String,
    normative_source_sha256: String,
    oracle_ids: Vec<String>,
    cases: Vec<Case>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Case {
    id: String,
    group: String,
    scenario: Scenario,
    normative: Outcome,
    draft_section: String,
    observations: Vec<Observation>,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum Scenario {
    CanonicalProfile,
    LegacyProtectedType,
    MissingVct,
    AdditionalTypeMatch,
    MalformedAdditionalTypes,
    ProtectedStatusDisclosure,
    HolderConfirmationPresent,
    HolderConfirmationMissing,
    MetadataExtensionChain,
    MetadataCycle,
    MandatoryNeverMissing,
    OptionalDisplayIgnored,
    StatusReferenceValid,
    StatusReferenceMalformed,
}

#[derive(Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
enum Outcome {
    Accept,
    Reject,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Observation {
    oracle_id: String,
    result: OracleResult,
    selector: String,
    note_code: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
enum OracleResult {
    Accept,
    Reject,
    NotExposed,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Receipts {
    schema_version: u8,
    recorded_at: String,
    normative_source: NormativeSource,
    oracles: Vec<OracleReceipt>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct NormativeSource {
    id: String,
    revision: String,
    text_sha256: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct OracleReceipt {
    id: String,
    language: String,
    version: String,
    revision: String,
    archive_sha256: String,
    registry_integrity: String,
    license: String,
    license_sha256: String,
    toolchain: String,
    lock_sha256: String,
    resolved_production_dependencies: usize,
    upstream_tests: String,
    audit: String,
    audit_sha256: String,
}

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
fn differential_packet_is_closed_source_bound_and_executable() {
    let packet: Packet = serde_json::from_str(PACKET).expect("closed differential packet");
    let receipts: Receipts = serde_json::from_str(RECEIPTS).expect("closed oracle receipts");
    assert_eq!(packet.schema_version, 1);
    assert_eq!(receipts.schema_version, 1);
    assert_eq!(
        packet.packet_id,
        "credentials.sd-jwt-vc.draft22.differential.v1"
    );
    assert_eq!(packet.normative_source_sha256, DRAFT_SHA256);
    assert_eq!(receipts.normative_source.text_sha256, DRAFT_SHA256);
    assert_eq!(
        receipts.normative_source.id,
        "draft-ietf-oauth-sd-jwt-vc-22"
    );
    assert_eq!(receipts.normative_source.revision.len(), 40);
    assert_eq!(receipts.recorded_at, "2026-10-29");

    let oracle_ids = packet.oracle_ids.iter().cloned().collect::<BTreeSet<_>>();
    assert_eq!(oracle_ids.len(), 2);
    assert_eq!(receipts.oracles.len(), 2);
    for receipt in &receipts.oracles {
        assert!(oracle_ids.contains(&receipt.id));
        assert!(matches!(receipt.language.as_str(), "typescript" | "php"));
        assert!(!receipt.version.is_empty());
        assert_eq!(receipt.revision.len(), 40);
        assert_eq!(receipt.archive_sha256.len(), 64);
        assert!(!receipt.registry_integrity.is_empty());
        assert!(matches!(receipt.license.as_str(), "Apache-2.0" | "MIT"));
        assert_eq!(receipt.license_sha256.len(), 64);
        assert!(!receipt.toolchain.is_empty());
        assert_eq!(receipt.lock_sha256.len(), 64);
        assert!(receipt.resolved_production_dependencies > 0);
        assert!(!receipt.upstream_tests.is_empty());
        assert!(!receipt.audit.is_empty());
        assert_eq!(receipt.audit_sha256.len(), 64);
    }

    let mut case_ids = BTreeSet::new();
    let mut groups = BTreeSet::new();
    for case in &packet.cases {
        assert!(
            case_ids.insert(case.id.as_str()),
            "duplicate case {}",
            case.id
        );
        assert!(!case.draft_section.is_empty());
        assert!(groups.insert(case.group.as_str()) || groups.contains(case.group.as_str()));
        assert_eq!(case.observations.len(), oracle_ids.len());
        let observed = case
            .observations
            .iter()
            .map(|observation| {
                assert!(oracle_ids.contains(&observation.oracle_id));
                assert!(!observation.selector.is_empty());
                assert!(!observation.note_code.is_empty());
                let _ = observation.result;
                observation.oracle_id.as_str()
            })
            .collect::<BTreeSet<_>>();
        assert_eq!(observed.len(), oracle_ids.len());

        let accepted = execute(case.scenario);
        assert_eq!(
            accepted,
            case.normative == Outcome::Accept,
            "normative mismatch for {}",
            case.id
        );
    }
    assert_eq!(case_ids.len(), 14);
    assert_eq!(groups.len(), 6);
}

fn execute(scenario: Scenario) -> bool {
    match scenario {
        Scenario::CanonicalProfile => validate(
            json!({"iss":"https://issuer.example", "vct":"urn:example:pid"}),
            vec![],
            ProfileValidationInputs::default(),
        )
        .is_ok(),
        Scenario::LegacyProtectedType => legacy_profile().is_ok(),
        Scenario::MissingVct => issue(json!({"iss":"https://issuer.example"}), vec![]).is_ok(),
        Scenario::AdditionalTypeMatch => validate(
            json!({
                "iss":"https://issuer.example",
                "vct":"urn:example:pid",
                "aka_vcts":["urn:example:pid:v2"]
            }),
            vec![],
            ProfileValidationInputs::default(),
        )
        .is_ok_and(|credential| credential.matches_type("urn:example:pid:v2")),
        Scenario::MalformedAdditionalTypes => issue(
            json!({
                "iss":"https://issuer.example",
                "vct":"urn:example:pid",
                "aka_vcts":"urn:example:pid:v2"
            }),
            vec![],
        )
        .is_ok(),
        Scenario::ProtectedStatusDisclosure => validate(
            json!({
                "iss":"https://issuer.example",
                "vct":"urn:example:pid",
                "status":{"status_list":{"idx":7,"uri":"https://status.example/list"}}
            }),
            vec![vec!["status", "status_list", "idx"]],
            ProfileValidationInputs::default(),
        )
        .is_ok(),
        Scenario::HolderConfirmationPresent => validate(
            json!({
                "iss":"https://issuer.example",
                "vct":"urn:example:pid",
                "cnf":{"kid":"did:example:holder#key-1"}
            }),
            vec![],
            ProfileValidationInputs {
                require_key_binding: true,
                ..ProfileValidationInputs::default()
            },
        )
        .is_ok(),
        Scenario::HolderConfirmationMissing => validate(
            json!({"iss":"https://issuer.example", "vct":"urn:example:pid"}),
            vec![],
            ProfileValidationInputs {
                require_key_binding: true,
                ..ProfileValidationInputs::default()
            },
        )
        .is_ok(),
        Scenario::MetadataExtensionChain => metadata_extension_chain(),
        Scenario::MetadataCycle => metadata_cycle(),
        Scenario::MandatoryNeverMissing => metadata_mandatory_missing(),
        Scenario::OptionalDisplayIgnored => optional_display_ignored(),
        Scenario::StatusReferenceValid => validate(
            json!({
                "iss":"https://issuer.example",
                "vct":"urn:example:pid",
                "status":{"status_list":{"idx":7,"uri":"https://status.example/list"}}
            }),
            vec![],
            ProfileValidationInputs::default(),
        )
        .is_ok(),
        Scenario::StatusReferenceMalformed => issue(
            json!({
                "iss":"https://issuer.example",
                "vct":"urn:example:pid",
                "status":{"status_list":{"idx":"7","uri":"https://status.example/list"}}
            }),
            vec![],
        )
        .is_ok(),
    }
}

fn issue(
    claims: Value,
    disclosures: Vec<Vec<&str>>,
) -> Result<identus_sd_jwt_vc::IssuedDraft22SdJwtVc, SdJwtVcError> {
    let limits = SdJwtLimits::default();
    let paths = disclosures
        .into_iter()
        .map(|segments| {
            ClaimPath::new(
                segments
                    .into_iter()
                    .map(|segment| PathSegment::Member(segment.to_owned()))
                    .collect(),
                limits,
            )
            .expect("bounded clean-room path")
        })
        .collect();
    issue_draft22(
        claims,
        &IssuancePlan {
            disclose: paths,
            decoy_locations: vec![],
            salt_bytes: 16,
        },
        &mut DeterministicEntropy(1),
        &Ed25519Signer::new(&Ed25519PrivateKey::from_slice(&[7; 32]).unwrap()),
        limits,
        SdJwtVcLimits::default(),
    )
}

fn validate(
    claims: Value,
    disclosures: Vec<Vec<&str>>,
    inputs: ProfileValidationInputs<'_>,
) -> Result<Draft22SdJwtVc, SdJwtVcError> {
    let limits = SdJwtLimits::default();
    let private = Ed25519PrivateKey::from_slice(&[7; 32]).unwrap();
    let public = private.to_public_key().encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).unwrap();
    let issued = issue(claims, disclosures)?;
    let base = parse_draft22_envelope(&issued.into_envelope()?, limits)?
        .verify_issuer(&SignatureSuiteRegistry::recommended(), &key)
        .map_err(|_| SdJwtVcError::Base)?
        .process_disclosures()
        .map_err(|_| SdJwtVcError::Base)?;
    Draft22SdJwtVc::validate(base, inputs, SdJwtVcLimits::default())
}

fn legacy_profile() -> Result<Draft22SdJwtVc, SdJwtVcError> {
    let limits = SdJwtLimits::default();
    let private = Ed25519PrivateKey::from_slice(&[7; 32]).unwrap();
    let public = private.to_public_key().encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).unwrap();
    let issued = issue_with_options(
        json!({"iss":"https://issuer.example", "vct":"urn:example:pid"}),
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
        limits,
    )
    .map_err(|_| SdJwtVcError::Base)?;
    let base = identus_sd_jwt::UnverifiedSdJwt::parse(issued.as_str(), limits)
        .map_err(|_| SdJwtVcError::Base)?
        .verify_issuer(&SignatureSuiteRegistry::recommended(), &key)
        .map_err(|_| SdJwtVcError::Base)?
        .process_disclosures()
        .map_err(|_| SdJwtVcError::Base)?;
    Draft22SdJwtVc::validate(
        base,
        ProfileValidationInputs::default(),
        SdJwtVcLimits::default(),
    )
}

fn metadata_extension_chain() -> bool {
    let limits = SdJwtVcLimits::default();
    let leaf = TypeMetadata::new(
        "urn:example:pid",
        Some("urn:example:base"),
        vec![],
        64,
        limits,
    )
    .unwrap();
    let base = TypeMetadata::new("urn:example:base", None, vec![], 64, limits).unwrap();
    let chain = TypeMetadataChain::new(vec![leaf, base], limits).unwrap();
    validate(
        json!({"iss":"https://issuer.example", "vct":"urn:example:pid"}),
        vec![],
        ProfileValidationInputs {
            type_metadata: Some(&chain),
            ..ProfileValidationInputs::default()
        },
    )
    .is_ok()
}

fn metadata_cycle() -> bool {
    let limits = SdJwtVcLimits::default();
    let first = TypeMetadata::new(
        "urn:example:first",
        Some("urn:example:second"),
        vec![],
        64,
        limits,
    )
    .unwrap();
    let second = TypeMetadata::new(
        "urn:example:second",
        Some("urn:example:first"),
        vec![],
        64,
        limits,
    )
    .unwrap();
    TypeMetadataChain::new(vec![first, second], limits).is_ok()
}

fn metadata_mandatory_missing() -> bool {
    let limits = SdJwtVcLimits::default();
    let claim = ClaimMetadata::new(
        vec!["family_name".to_owned()],
        true,
        ClaimDisclosureRule::Never,
        limits,
    )
    .unwrap();
    let metadata = TypeMetadata::new("urn:example:pid", None, vec![claim], 128, limits).unwrap();
    let chain = TypeMetadataChain::new(vec![metadata], limits).unwrap();
    validate(
        json!({"iss":"https://issuer.example", "vct":"urn:example:pid"}),
        vec![],
        ProfileValidationInputs {
            type_metadata: Some(&chain),
            ..ProfileValidationInputs::default()
        },
    )
    .is_ok()
}

fn optional_display_ignored() -> bool {
    // Draft-21 made claim display/svg_id non-critical for credential processing.
    // The policy-neutral Rust evidence boundary imports only path/mandatory/sd.
    let limits = SdJwtVcLimits::default();
    let claim = ClaimMetadata::new(
        vec!["given_name".to_owned()],
        false,
        ClaimDisclosureRule::Allowed,
        limits,
    )
    .unwrap();
    let metadata = TypeMetadata::new("urn:example:pid", None, vec![claim], 128, limits).unwrap();
    let chain = TypeMetadataChain::new(vec![metadata], limits).unwrap();
    validate(
        json!({
            "iss":"https://issuer.example",
            "vct":"urn:example:pid",
            "given_name":"Ada"
        }),
        vec![],
        ProfileValidationInputs {
            type_metadata: Some(&chain),
            ..ProfileValidationInputs::default()
        },
    )
    .is_ok()
}
