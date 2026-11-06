use identus_crypto::{Ed25519PrivateKey, EncodeJwk};
use identus_did::{Did, DidUrl};
use identus_jose::{
    Ed25519Signer, JwsAlgorithm, JwsLimits, JwsSigningInput, JwsVerificationKey, ProtectedHeader,
    SignatureSuiteRegistry, UnverifiedCompactJws,
};
use identus_siopv2::{REQUEST_OBJECT_JWT_TYPE, Siopv2Limits, UnverifiedSiopRequest};
use serde::Deserialize;

const REQUEST_PRIVATE: [u8; 32] = [0x31; 32];
const SUBJECT_PRIVATE: [u8; 32] = [0x52; 32];
const NOW: i64 = 1_800_000_000;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Packet {
    schema_version: u32,
    packet_id: String,
    capability: String,
    cases: Vec<Case>,
}

#[derive(Deserialize)]
struct Case {
    id: String,
    operation: String,
    input: Input,
    expected: Expected,
}

#[derive(Deserialize)]
struct Input {
    literal: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Expected {
    kind: String,
    error_code: Option<String>,
    redacted: Option<bool>,
    fields: Fields,
}

#[derive(Deserialize)]
struct Fields {
    serialized: String,
}

#[test]
fn siopv2_draft13_vector_executes_through_the_sdk_boundary() {
    let packet: Packet = serde_json::from_str(include_str!(
        "../../../docs/conformance/fixtures/siopv2/v1/vectors.json"
    ))
    .expect("bounded checked-in vector packet");
    assert_eq!(packet.schema_version, 1);
    assert_eq!(packet.packet_id, "siopv2.did-authentication.draft13.v1");
    assert_eq!(packet.capability, "siopv2.did-authentication");
    assert_eq!(packet.cases.len(), 1);
    let case = &packet.cases[0];
    assert_eq!(case.id, "siopv2.did-authentication.draft13-ed25519");
    assert_eq!(case.operation, "siopv2.did-authentication.roundtrip");
    assert_eq!(case.expected.kind, "success");
    assert_eq!(case.expected.error_code, None);
    assert_eq!(case.expected.redacted, None);

    let request_private = Ed25519PrivateKey::from_slice(&REQUEST_PRIVATE).unwrap();
    let header = ProtectedHeader::new(
        "Ed25519",
        Some(REQUEST_OBJECT_JWT_TYPE),
        Some("did:example:rp#key-1"),
        JwsLimits::default(),
    )
    .unwrap();
    let compact = JwsSigningInput::new(
        header,
        case.input.literal.as_bytes().to_vec(),
        JwsLimits::default(),
    )
    .unwrap()
    .sign_with(&Ed25519Signer::new(&request_private))
    .unwrap()
    .compact()
    .to_owned();
    let public = request_private.to_public_key().encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).unwrap();
    let request = UnverifiedSiopRequest::parse(&compact, Siopv2Limits::default())
        .unwrap()
        .verify(&SignatureSuiteRegistry::recommended(), &key, NOW)
        .unwrap();
    let subject_private = Ed25519PrivateKey::from_slice(&SUBJECT_PRIVATE).unwrap();
    let response = request
        .plan_response(
            Did::parse("did:example:alice").unwrap(),
            DidUrl::parse("did:example:alice#auth-1").unwrap(),
            JwsAlgorithm::Ed25519,
            NOW,
            NOW + 120,
        )
        .unwrap()
        .sign(&Ed25519Signer::new(&subject_private))
        .unwrap();
    let body = std::str::from_utf8(response.expose_sensitive_body()).unwrap();
    let (token_form, state_form) = body.split_once('&').unwrap();
    let token = percent_decode(token_form.strip_prefix("id_token=").unwrap());
    assert_eq!(state_form, "state=state-456");
    let compact = UnverifiedCompactJws::parse(&token, JwsLimits::default()).unwrap();
    let subject_public = subject_private.to_public_key().encode_jwk();
    let subject_key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &subject_public).unwrap();
    let verified = SignatureSuiteRegistry::recommended()
        .verify(&compact, &subject_key)
        .unwrap();
    let claims: serde_json::Value =
        serde_json::from_slice(verified.as_compact().payload()).unwrap();
    let actual = format!(
        "iss=sub={};aud={};nonce={};mode=direct_post;state=state-456",
        claims["iss"].as_str().unwrap(),
        claims["aud"].as_str().unwrap(),
        claims["nonce"].as_str().unwrap(),
    );
    assert_eq!(case.expected.fields.serialized, actual);
}

fn percent_decode(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut output = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            output.push((hex(bytes[index + 1]) << 4) | hex(bytes[index + 2]));
            index += 3;
        } else {
            output.push(bytes[index]);
            index += 1;
        }
    }
    String::from_utf8(output).unwrap()
}

fn hex(byte: u8) -> u8 {
    match byte {
        b'0'..=b'9' => byte - b'0',
        b'A'..=b'F' => byte - b'A' + 10,
        b'a'..=b'f' => byte - b'a' + 10,
        _ => panic!("invalid form encoding"),
    }
}
