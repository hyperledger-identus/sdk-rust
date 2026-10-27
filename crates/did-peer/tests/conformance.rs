use identus_did_peer::{
    numalgo2_compat::PeerDid,
    numalgo4::{InputDocument, LongForm},
};
use serde::Deserialize;
use serde_json::{Map, Value};

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

#[test]
fn portable_peer_did_packet_matches_both_profiles() {
    let packet: Packet = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/conformance/fixtures/did-peer/v1/vectors.json"
    )))
    .expect("checked-in peer DID packet must deserialize");

    assert_eq!(packet.schema_version, 1);
    assert_eq!(packet.packet_id, "did.peer.profiles.v1");
    assert_eq!(packet.capability, "did.peer.profiles");
    assert_eq!(packet.cases.len(), 2);

    for case in packet.cases {
        assert_eq!(case.expected.kind, "success", "{}", case.id);
        assert!(case.expected.error_code.is_none(), "{}", case.id);
        assert!(case.expected.redacted.is_none(), "{}", case.id);
        match case.operation.as_str() {
            "did-peer.numalgo4.create" => {
                let input = InputDocument::from_json_str(&case.input.literal).expect("input");
                let actual = LongForm::create(input).expect("create");
                assert_eq!(
                    actual.did().as_str(),
                    case.expected.fields["did"].as_str().expect("expected DID")
                );
                LongForm::parse(actual.did().as_str())
                    .expect("parse")
                    .resolve()
                    .expect("resolve");
            }
            "did-peer.numalgo2.parse-resolve" => {
                let actual = PeerDid::parse(&case.input.literal).expect("parse");
                actual.resolve().expect("resolve");
                assert_eq!(
                    actual.did().as_str(),
                    case.expected.fields["did"].as_str().expect("expected DID")
                );
            }
            operation => panic!("{} has unsupported operation {operation}", case.id),
        }
    }
}
