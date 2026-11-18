use identus_did_peer::{
    numalgo2_compat::{PeerDid, PublicKey, Purpose, Service, ServiceEndpoint, ServiceEndpointMap},
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
    input: Value,
    expected: Expected,
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
    assert_eq!(packet.cases.len(), 13);

    for case in packet.cases {
        if case.expected.kind == "error" {
            assert!(case.expected.error_code.is_some(), "{}", case.id);
            assert_eq!(case.expected.redacted, Some(true), "{}", case.id);
            continue;
        }
        assert_eq!(case.expected.kind, "success", "{}", case.id);
        assert!(case.expected.error_code.is_none(), "{}", case.id);
        assert!(case.expected.redacted.is_none(), "{}", case.id);
        match case.operation.as_str() {
            "did-peer.numalgo4.create" => {
                let source = if let Some(literal) = case.input["literal"].as_str() {
                    literal.to_owned()
                } else {
                    let generated = &case.input["generated"];
                    format!(
                        "{}{}{}",
                        generated["prefix"].as_str().expect("generated prefix"),
                        generated["repeat"]
                            .as_str()
                            .expect("generated repeat")
                            .repeat(
                                generated["count"]
                                    .as_u64()
                                    .expect("generated count")
                                    .try_into()
                                    .expect("bounded generated count"),
                            ),
                        generated["suffix"].as_str().expect("generated suffix"),
                    )
                };
                let input = InputDocument::from_json_str(&source).expect("input");
                let actual = LongForm::create(input).expect("create");
                if let Some(expected) = case.expected.fields["did"].as_str() {
                    assert_eq!(actual.did().as_str(), expected);
                } else {
                    assert_eq!(case.expected.fields["resultKind"], "success");
                }
                LongForm::parse(actual.did().as_str())
                    .expect("parse")
                    .resolve()
                    .expect("resolve");
            }
            "did-peer.numalgo2.parse-resolve" => {
                let actual = PeerDid::parse(case.input["literal"].as_str().expect("literal input"))
                    .expect("parse");
                actual.resolve().expect("resolve");
                assert_eq!(
                    actual.did().as_str(),
                    case.expected.fields["did"].as_str().expect("expected DID")
                );
            }
            "did-peer.numalgo2.create-pairwise" => {
                let input: Value =
                    serde_json::from_str(case.input["literal"].as_str().expect("literal input"))
                        .expect("pairwise input JSON");
                let keys = input
                    .get("keys")
                    .and_then(Value::as_array)
                    .expect("keys")
                    .iter()
                    .map(|key| {
                        let purpose = match key["purpose"].as_str().expect("purpose") {
                            "verification" => Purpose::Verification,
                            "encryption" => Purpose::Encryption,
                            value => panic!("unsupported fixture purpose {value}"),
                        };
                        PublicKey::new(purpose, key["multibase"].as_str().expect("multibase"))
                            .expect("public key")
                    })
                    .collect();
                let services = input
                    .get("services")
                    .and_then(Value::as_array)
                    .expect("services")
                    .iter()
                    .map(|service| Service {
                        type_: service["type"].as_str().expect("service type").to_owned(),
                        endpoint: ServiceEndpoint::Map(ServiceEndpointMap {
                            uri: service["endpoint"]["uri"]
                                .as_str()
                                .expect("service URI")
                                .to_owned(),
                            accept: Some(
                                service["endpoint"]["accept"]
                                    .as_array()
                                    .expect("accept")
                                    .iter()
                                    .map(|value| value.as_str().expect("accept value").to_owned())
                                    .collect(),
                            ),
                            routing_keys: Some(
                                service["endpoint"]["routingKeys"]
                                    .as_array()
                                    .expect("routing keys")
                                    .iter()
                                    .map(|value| value.as_str().expect("routing key").to_owned())
                                    .collect(),
                            ),
                        }),
                        id: None,
                    })
                    .collect();
                let actual = PeerDid::create(keys, services).expect("create pairwise DID");
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
