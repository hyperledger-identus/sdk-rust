use identus_prism_did::{PrismDidState, SignedPrismOperation};
use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Packet {
    schema_version: u32,
    packet_id: String,
    capability: String,
    cases: Vec<Case>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Case {
    id: String,
    operation: String,
    input: Input,
    expected: Expected,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Input {
    literal: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Expected {
    kind: String,
    error_code: Option<String>,
    redacted: Option<bool>,
    fields: Map<String, Value>,
}

fn expected_text<'a>(fields: &'a Map<String, Value>, name: &str) -> Option<&'a str> {
    fields.get(name).map(|value| {
        value
            .as_str()
            .unwrap_or_else(|| panic!("{name} expectation must be text"))
    })
}

#[test]
fn portable_prism_packet_matches_identus_prism_did() {
    let packet: Packet = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/conformance/fixtures/did-prism/v1/vectors.json"
    )))
    .expect("checked-in PRISM DID vector packet must deserialize");

    assert_eq!(packet.schema_version, 1);
    assert_eq!(packet.packet_id, "did.prism.portable-method.v1");
    assert_eq!(packet.capability, "did.prism");
    assert_eq!(packet.cases.len(), 3);

    for case in packet.cases {
        let bytes = hex::decode(&case.input.literal)
            .unwrap_or_else(|error| panic!("{} has invalid hex: {error}", case.id));
        match (case.operation.as_str(), case.expected.kind.as_str()) {
            ("did-prism.signed-create.verify", "success") => {
                let signed = SignedPrismOperation::parse(&bytes)
                    .unwrap_or_else(|error| panic!("{} parse failed: {error}", case.id));
                let state = PrismDidState::from_signed_create(&signed)
                    .unwrap_or_else(|error| panic!("{} verify failed: {error}", case.id));
                if let Some(expected) = expected_text(&case.expected.fields, "operationKind") {
                    assert_eq!(expected, "createDid");
                }
                if let Some(expected) = expected_text(&case.expected.fields, "operationId") {
                    assert_eq!(hex::encode(signed.operation_id().as_bytes()), expected);
                }
                if let Some(expected) = expected_text(&case.expected.fields, "operationHash") {
                    assert_eq!(hex::encode(signed.operation_hash().as_bytes()), expected);
                }
                if let Some(expected) = expected_text(&case.expected.fields, "canonicalDid") {
                    assert_eq!(state.did().as_did().as_str(), expected);
                }
                assert!(case.expected.error_code.is_none());
                assert!(case.expected.redacted.is_none());
            }
            ("did-prism.envelope.parse", "error") => {
                let local_error = SignedPrismOperation::parse(&bytes).unwrap_err();
                assert!(!local_error.to_string().contains(&case.input.literal));
                assert!(!format!("{local_error:?}").contains(&case.input.literal));
                let error = local_error.to_identus_error();
                assert_eq!(
                    error.code().as_str(),
                    case.expected.error_code.as_deref().expect("error code")
                );
                assert_eq!(case.expected.redacted, Some(true));
                assert!(case.expected.fields.is_empty());
            }
            (operation, kind) => panic!("{} has unsupported {operation}/{kind}", case.id),
        }
    }
}
