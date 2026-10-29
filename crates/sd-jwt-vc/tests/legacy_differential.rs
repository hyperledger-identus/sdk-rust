use identus_credentials::{CredentialEnvelope, CredentialFormat, CredentialPayload};
use identus_crypto::{Ed25519PrivateKey, EncodeJwk, SecureRandom};
use identus_jose::{Ed25519Signer, JwsAlgorithm, JwsVerificationKey, SignatureSuiteRegistry};
use identus_sd_jwt::{IssuanceOptions, IssuancePlan, SdJwtLimits, issue_with_options};
use identus_sd_jwt_vc::{LegacyValidationInputs, LegacyVcSdJwtDraft03Adapter, SdJwtVcLimits};
use serde::Deserialize;
use serde_json::Value;

const PACKET: &str =
    include_str!("../../../docs/conformance/fixtures/sd-jwt-vc-draft03-adapter/v1/vectors.json");
const RECEIPT: &str = include_str!("fixtures/legacy-draft03-v1/oracle-receipt.json");

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Packet {
    schema_version: u8,
    packet_id: String,
    capability: String,
    cases: Vec<Case>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Case {
    id: String,
    operation: String,
    input: Input,
    expected: Expected,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Input {
    literal: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Expected {
    kind: String,
    error_code: Option<String>,
    redacted: Option<bool>,
    fields: serde_json::Map<String, Value>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct OracleReceipt {
    schema_version: u8,
    recorded_at: String,
    consumer: Consumer,
    oracle: Oracle,
    normative_sources: Vec<NormativeSource>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Consumer {
    id: String,
    version: String,
    revision: String,
    license: String,
    selectors: Vec<String>,
    observations: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Oracle {
    id: String,
    version: String,
    revision: String,
    license: String,
    npm_integrity: String,
    tarball_sha512: String,
    observations: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct NormativeSource {
    id: String,
    sha256: String,
}

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
fn legacy_packet_executes_through_the_public_adapter() {
    let packet: Packet = serde_json::from_str(PACKET).expect("closed legacy packet");
    assert_eq!(packet.schema_version, 1);
    assert_eq!(packet.packet_id, "credentials.sd-jwt-vc.draft03-adapter.v1");
    assert_eq!(packet.capability, "credentials.sd-jwt");
    assert_eq!(packet.cases.len(), 4);

    let private = Ed25519PrivateKey::from_slice(&[7; 32]).unwrap();
    let public = private.to_public_key().encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).unwrap();
    for case in packet.cases {
        assert_eq!(case.operation, "sd-jwt-vc.validate-profile");
        let claims: Value = serde_json::from_str(&case.input.literal).unwrap();
        let issued = issue_with_options(
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
            SdJwtLimits::default(),
        )
        .unwrap();
        let envelope = CredentialEnvelope::new(
            CredentialFormat::parse("vc+sd-jwt").unwrap(),
            CredentialPayload::new(issued.as_str().as_bytes().to_vec()).unwrap(),
        );
        let base = LegacyVcSdJwtDraft03Adapter::parse_envelope(&envelope, SdJwtLimits::default())
            .unwrap()
            .verify_issuer(&SignatureSuiteRegistry::recommended(), &key)
            .unwrap()
            .process_disclosures()
            .unwrap();
        let result = LegacyVcSdJwtDraft03Adapter::validate(
            base,
            LegacyValidationInputs::default(),
            SdJwtVcLimits::default(),
        );
        match case.expected.kind.as_str() {
            "success" => {
                let result = result.unwrap();
                assert!(case.expected.error_code.is_none());
                assert!(case.expected.redacted.is_none());
                assert_eq!(
                    case.expected.fields.get("operationKind"),
                    Some(&Value::String("legacy-common-subset".into()))
                );
                assert!(result.receipt().canonical_reissuance_required);
            }
            "error" => {
                let error = result.unwrap_err();
                assert_eq!(
                    case.expected.error_code.as_deref(),
                    Some(error.code().as_str())
                );
                assert_eq!(case.expected.redacted, Some(true));
                assert!(case.expected.fields.is_empty());
            }
            other => panic!("unknown packet outcome: {other}"),
        }
        assert!(case.id.starts_with("credentials.sd-jwt-vc.draft03-"));
    }
}

#[test]
fn oracle_receipt_is_exact_and_source_bound() {
    let receipt: OracleReceipt = serde_json::from_str(RECEIPT).expect("closed oracle receipt");
    assert_eq!(receipt.schema_version, 1);
    assert_eq!(receipt.recorded_at, "2026-10-29");
    assert_eq!(receipt.consumer.id, "hyperledger-identus/sdk-ts");
    assert_eq!(receipt.consumer.version, "8.1.4");
    assert_eq!(
        receipt.consumer.revision,
        "4bf86ebf69d5e96616a148e4c973f831f95fa38e"
    );
    assert_eq!(receipt.consumer.license, "Apache-2.0");
    assert_eq!(receipt.consumer.selectors.len(), 4);
    assert_eq!(receipt.consumer.observations.len(), 3);
    assert_eq!(receipt.oracle.id, "@sd-jwt/sd-jwt-vc");
    assert_eq!(receipt.oracle.version, "0.7.2");
    assert_eq!(
        receipt.oracle.revision,
        "d3cc53b66be9ef40ede9f86a931b7a43f55dbd3a"
    );
    assert_eq!(receipt.oracle.license, "Apache-2.0");
    assert!(receipt.oracle.npm_integrity.starts_with("sha512-"));
    assert_eq!(receipt.oracle.tarball_sha512.len(), 128);
    assert_eq!(receipt.oracle.observations.len(), 3);
    assert_eq!(receipt.normative_sources.len(), 2);
    for source in receipt.normative_sources {
        assert!(source.id.starts_with("draft-ietf-oauth-sd-jwt-vc-"));
        assert_eq!(source.sha256.len(), 64);
    }
}
