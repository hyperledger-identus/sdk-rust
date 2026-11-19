use identus_did_peer::{
    Error,
    numalgo2_compat::{PeerDid, PublicKey, Purpose, Service, ServiceEndpoint, ServiceEndpointMap},
    numalgo4::{InputDocument, LongForm, ShortForm},
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

const TYPESCRIPT_ONLY_PRECONDITIONS: &[&str] = &[
    "accessor-property rejection",
    "cycle rejection",
    "exotic-prototype rejection",
    "unsafe-number rejection",
    "closed-object field rejection",
];

fn expected_error(code: &str) -> Error {
    match code {
        "did.peer.wrong_profile" => Error::WrongProfile,
        "did.peer.unsupported_algorithm" => Error::UnsupportedAlgorithm,
        "did.peer.malformed_encoding" => Error::MalformedEncoding,
        "did.peer.unsupported_codec" => Error::UnsupportedCodec,
        "did.peer.invalid_input_document" => Error::InvalidInputDocument,
        "did.peer.hash_mismatch" => Error::HashMismatch,
        "did.peer.long_form_required" => Error::LongFormRequired,
        "did.peer.resource_limit" => Error::ResourceLimit,
        value => panic!("unsupported fixture error {value}"),
    }
}

fn numalgo2_input(input: &Value) -> Result<(Vec<PublicKey>, Vec<Service>), Error> {
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
        })
        .collect::<Result<Vec<_>, _>>()?;
    let services = input
        .get("services")
        .and_then(Value::as_array)
        .map(|services| {
            services
                .iter()
                .map(|service| Service {
                    type_: service["type"].as_str().expect("service type").to_owned(),
                    endpoint: if let Some(uri) = service["endpoint"].as_str() {
                        ServiceEndpoint::Uri(uri.to_owned())
                    } else {
                        ServiceEndpoint::Map(ServiceEndpointMap {
                            uri: service["endpoint"]["uri"]
                                .as_str()
                                .expect("service URI")
                                .to_owned(),
                            accept: service["endpoint"].get("accept").map(|values| {
                                values
                                    .as_array()
                                    .expect("accept")
                                    .iter()
                                    .map(|value| value.as_str().expect("accept value").to_owned())
                                    .collect()
                            }),
                            routing_keys: service["endpoint"].get("routingKeys").map(|values| {
                                values
                                    .as_array()
                                    .expect("routing keys")
                                    .iter()
                                    .map(|value| value.as_str().expect("routing key").to_owned())
                                    .collect()
                            }),
                        })
                    },
                    id: service.get("id").and_then(Value::as_str).map(str::to_owned),
                })
                .collect()
        })
        .unwrap_or_default();
    Ok((keys, services))
}

fn generated_input(input: &Value) -> String {
    if let Some(literal) = input["literal"].as_str() {
        return literal.to_owned();
    }
    let generated = &input["generated"];
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
}

fn run_error_case(case: &Case) -> Error {
    let result = match case.operation.as_str() {
        "did-peer.numalgo4.parse-long" => {
            LongForm::parse(case.input["literal"].as_str().expect("literal input")).map(|_| ())
        }
        "did-peer.numalgo2.parse-resolve" => {
            PeerDid::parse(case.input["literal"].as_str().expect("literal input"))
                .and_then(|peer| peer.resolve().map(|_| ()))
        }
        "did-peer.numalgo4.resolve-short" => {
            ShortForm::parse(case.input["literal"].as_str().expect("literal input"))
                .and_then(|short| short.resolve().map(|_| ()))
        }
        "did-peer.numalgo4.create" => InputDocument::from_json_str(&generated_input(&case.input))
            .and_then(LongForm::create)
            .map(|_| ()),
        "did-peer.numalgo2.create" => {
            let input: Value =
                serde_json::from_str(case.input["literal"].as_str().expect("literal input"))
                    .expect("numalgo-2 input JSON");
            numalgo2_input(&input)
                .and_then(|(keys, services)| PeerDid::create_sdk_ts_compatibility(keys, services))
                .map(|_| ())
        }
        operation => panic!("{} has unsupported error operation {operation}", case.id),
    };
    result.expect_err("error fixture must reject")
}

fn assert_pairwise_document(case: &Case, peer: &PeerDid, input: &Value) {
    let document = serde_json::to_value(peer.resolve().expect("resolve pairwise DID"))
        .expect("serialize resolved document");
    let did = case.expected.fields["did"].as_str().expect("expected DID");
    assert_eq!(document["id"], did, "{} document identifier", case.id);
    assert_eq!(
        document["authentication"][0],
        format!(
            "{did}{}",
            case.expected.fields["authentication"]
                .as_str()
                .expect("authentication")
        ),
        "{} authentication reference",
        case.id
    );
    assert_eq!(
        document["keyAgreement"][0],
        format!(
            "{did}{}",
            case.expected.fields["keyAgreement"]
                .as_str()
                .expect("key agreement")
        ),
        "{} key-agreement reference",
        case.id
    );
    let expected_service = &input["services"][0];
    let actual_service = &document["service"][0];
    assert_eq!(
        actual_service["id"],
        format!("{did}#service"),
        "{} service ID",
        case.id
    );
    assert_eq!(
        actual_service["type"], case.expected.fields["serviceType"],
        "{} service type",
        case.id
    );
    assert_eq!(
        actual_service["serviceEndpoint"]["uri"], case.expected.fields["serviceUri"],
        "{} service URI",
        case.id
    );
    assert_eq!(
        actual_service["serviceEndpoint"]["routingKeys"],
        expected_service["endpoint"]["routingKeys"],
        "{} routing key values",
        case.id
    );
    assert_eq!(
        actual_service["serviceEndpoint"]["accept"], expected_service["endpoint"]["accept"],
        "{} accept values",
        case.id
    );
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
    assert_eq!(TYPESCRIPT_ONLY_PRECONDITIONS.len(), 5);

    for case in packet.cases {
        if case.expected.kind == "error" {
            let expected = case.expected.error_code.as_deref().expect("error code");
            assert_eq!(case.expected.redacted, Some(true), "{}", case.id);
            assert_eq!(
                run_error_case(&case),
                expected_error(expected),
                "{}",
                case.id
            );
            continue;
        }
        assert_eq!(case.expected.kind, "success", "{}", case.id);
        assert!(case.expected.error_code.is_none(), "{}", case.id);
        assert!(case.expected.redacted.is_none(), "{}", case.id);
        match case.operation.as_str() {
            "did-peer.numalgo4.create" => {
                let source = generated_input(&case.input);
                let input = InputDocument::from_json_str(&source).expect("input");
                let actual = LongForm::create(input).expect("create");
                if let Some(expected) = case.expected.fields.get("did").and_then(Value::as_str) {
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
                let (keys, services) = numalgo2_input(&input).expect("pairwise input");
                let actual = PeerDid::create_sdk_ts_compatibility(keys, services)
                    .expect("create pairwise DID");
                assert_eq!(
                    actual.did().as_str(),
                    case.expected.fields["did"].as_str().expect("expected DID")
                );
                assert_pairwise_document(&case, &actual, &input);
            }
            operation => panic!("{} has unsupported operation {operation}", case.id),
        }
    }
}
