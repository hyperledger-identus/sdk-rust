use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use identus_crypto::{Ed25519PrivateKey, EncodeJwk};
use identus_jose::{
    Ed25519Signer, JwsAlgorithm, JwsLimits, JwsSigningInput, JwsVerificationKey, ProtectedHeader,
    SignatureSuiteRegistry,
};
use identus_sd_jwt::{SdJwtError, SdJwtLimits, UnverifiedSdJwt};
use serde::Deserialize;
use serde_json::{Map, Value, json};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Packet {
    schema_version: u32,
    packet_id: String,
    capability: String,
    cases: Vec<Case>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
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
    fields: Map<String, Value>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct OraclePacket {
    schema_version: u32,
    oracle: String,
    version: String,
    revision: String,
    crate_sha256: String,
    source: String,
    cases: Vec<OracleCase>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct OracleCase {
    disclosure: String,
    digest: String,
}

fn signed(payload: Value) -> (String, Ed25519PrivateKey) {
    let private = Ed25519PrivateKey::from_slice(&[37; 32]).unwrap();
    let limits = JwsLimits::new(524_288, 4_096, 524_288, 1_024, 2_048).unwrap();
    let header = ProtectedHeader::new("Ed25519", None, None, limits).unwrap();
    let compact = JwsSigningInput::new(header, serde_json::to_vec(&payload).unwrap(), limits)
        .unwrap()
        .sign_with(&Ed25519Signer::new(&private))
        .unwrap()
        .compact()
        .to_owned();
    (compact, private)
}

fn process(issuer: &str, disclosure: &str, private: &Ed25519PrivateKey) -> Value {
    let public = private.to_public_key().encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).unwrap();
    UnverifiedSdJwt::parse(&format!("{issuer}~{disclosure}~"), SdJwtLimits::default())
        .unwrap()
        .verify_issuer(&SignatureSuiteRegistry::recommended(), &key)
        .unwrap()
        .process_disclosures()
        .unwrap()
        .claims()
        .clone()
}

#[test]
fn malformed_compact_is_rejected() {
    let packet: Packet = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/conformance/fixtures/sd-jwt-rfc9901/v1/vectors.json"
    )))
    .unwrap();
    assert_eq!(packet.schema_version, 1);
    assert_eq!(packet.packet_id, "credentials.sd-jwt.rfc9901.v1");
    assert_eq!(packet.capability, "credentials.sd-jwt");
    let case = packet.cases.first().unwrap();
    assert_eq!(case.id, "credentials.sd-jwt.malformed-compact");
    assert_eq!(case.operation, "sd-jwt.parse");
    assert_eq!(case.expected.kind, "error");
    assert_eq!(
        case.expected.error_code.as_deref(),
        Some("sd_jwt.invalid_compact")
    );
    assert_eq!(case.expected.redacted, Some(true));
    assert!(case.expected.fields.is_empty());
    assert_eq!(
        UnverifiedSdJwt::parse(&case.input.literal, SdJwtLimits::default()).unwrap_err(),
        SdJwtError::CompactStructure
    );
}

#[test]
fn rfc_9901_section_4_disclosures_match_exact_bytes_and_digests() {
    // RFC 9901 sections 4.2.1 and 4.2.2. Whitespace is intentionally
    // retained because the digest covers the encoded Disclosure itself.
    let object = "WyJfMjZiYzRMVC1hYzZxMktJNmNCVzVlcyIsICJmYW1pbHlfbmFtZSIsICJNw7ZiaXVzIl0";
    let object_digest =
        URL_SAFE_NO_PAD.encode(identus_crypto::sha256(object.as_bytes()).as_bytes());
    assert_eq!(object_digest, "X9yH0Ajrdm1Oij4tWso9UzzKJvPoDxwmuEcO3XAdRC0");
    let (issuer, private) = signed(json!({"_sd_alg":"sha-256", "_sd":[object_digest]}));
    assert_eq!(process(&issuer, object, &private)["family_name"], "Möbius");

    let array = "WyJsa2x4RjVqTVlsR1RQVW92TU5JdkNBIiwgIkZSIl0";
    let array_digest = URL_SAFE_NO_PAD.encode(identus_crypto::sha256(array.as_bytes()).as_bytes());
    assert_eq!(array_digest, "w0I8EKcdCtUPkGCNUrfwVp2xEgNjtoIDlOxc9-PlOhs");
    let (issuer, private) = signed(json!({
        "_sd_alg":"sha-256", "nationalities":[{"...":array_digest}]
    }));
    assert_eq!(
        process(&issuer, array, &private)["nationalities"],
        json!(["FR"])
    );
}

#[test]
fn differential_oracle_disclosure_packet_is_stable() {
    let packets = [
        serde_json::from_str::<OraclePacket>(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../docs/conformance/fixtures/sd-jwt-rfc9901/v1/oracles/affinidi-0.1.8.json"
        )))
        .unwrap(),
        serde_json::from_str::<OraclePacket>(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../docs/conformance/fixtures/sd-jwt-rfc9901/v1/oracles/iota-0.5.1.json"
        )))
        .unwrap(),
    ];
    assert_eq!(packets[0].oracle, "affinidi-sd-jwt");
    assert_eq!(packets[1].oracle, "sd-jwt-payload");
    for packet in packets {
        assert_eq!(packet.schema_version, 1);
        assert!(!packet.version.is_empty());
        assert_eq!(packet.revision.len(), 40);
        assert_eq!(packet.crate_sha256.len(), 64);
        assert!(!packet.source.is_empty());
        for case in packet.cases {
            assert_eq!(
                URL_SAFE_NO_PAD
                    .encode(identus_crypto::sha256(case.disclosure.as_bytes()).as_bytes()),
                case.digest
            );
        }
    }
}
