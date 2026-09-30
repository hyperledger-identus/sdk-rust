use identus_did::{Did, DidUrl};
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
#[serde(untagged)]
enum Input {
    Literal { literal: String },
    Generated { generated: Generated },
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Generated {
    prefix: String,
    repeat: String,
    count: usize,
    suffix: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Expected {
    kind: String,
    error_code: Option<String>,
    redacted: Option<bool>,
    fields: Map<String, Value>,
}

impl Input {
    fn materialize(&self) -> String {
        match self {
            Self::Literal { literal } => literal.clone(),
            Self::Generated { generated } => {
                assert_eq!(generated.repeat.chars().count(), 1);
                assert!(generated.repeat.is_ascii());
                format!(
                    "{}{}{}",
                    generated.prefix,
                    generated.repeat.repeat(generated.count),
                    generated.suffix
                )
            }
        }
    }
}

fn expected_text<'a>(fields: &'a Map<String, Value>, name: &str) -> Option<Option<&'a str>> {
    fields.get(name).map(|value| {
        if value.is_null() {
            None
        } else {
            Some(
                value
                    .as_str()
                    .expect("field expectation must be text or null"),
            )
        }
    })
}

fn assert_common_did_fields(
    fields: &Map<String, Value>,
    serialized: &str,
    method: &str,
    method_specific_id: &str,
) {
    if let Some(expected) = expected_text(fields, "serialized") {
        assert_eq!(Some(serialized), expected);
    }
    if let Some(expected) = expected_text(fields, "method") {
        assert_eq!(Some(method), expected);
    }
    if let Some(expected) = expected_text(fields, "methodSpecificId") {
        assert_eq!(Some(method_specific_id), expected);
    }
}

#[test]
fn portable_did_packet_matches_identus_did() {
    let packet: Packet = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/conformance/fixtures/did/v1/vectors.json"
    )))
    .expect("checked-in DID vector packet must deserialize");

    assert_eq!(packet.schema_version, 1);
    assert_eq!(packet.packet_id, "did.syntax.v1");
    assert_eq!(packet.capability, "did.syntax");
    assert_eq!(packet.cases.len(), 22);

    for case in packet.cases {
        let input = case.input.materialize();
        match (case.operation.as_str(), case.expected.kind.as_str()) {
            ("did.parse", "success") => {
                let did = Did::parse(&input)
                    .unwrap_or_else(|error| panic!("{} unexpectedly failed: {error}", case.id));
                assert_common_did_fields(
                    &case.expected.fields,
                    did.as_str(),
                    did.method(),
                    did.method_specific_id(),
                );
                assert!(case.expected.error_code.is_none());
                assert!(case.expected.redacted.is_none());
            }
            ("did-url.parse", "success") => {
                let did_url = DidUrl::parse(&input)
                    .unwrap_or_else(|error| panic!("{} unexpectedly failed: {error}", case.id));
                assert_common_did_fields(
                    &case.expected.fields,
                    did_url.as_str(),
                    did_url.method(),
                    did_url.method_specific_id(),
                );
                if let Some(expected) = expected_text(&case.expected.fields, "did") {
                    assert_eq!(Some(did_url.as_did_str()), expected);
                }
                if let Some(expected) = expected_text(&case.expected.fields, "path") {
                    assert_eq!(Some(did_url.path()), expected);
                }
                if let Some(expected) = expected_text(&case.expected.fields, "query") {
                    assert_eq!(did_url.query(), expected);
                }
                if let Some(expected) = expected_text(&case.expected.fields, "fragment") {
                    assert_eq!(did_url.fragment(), expected);
                }
                assert!(case.expected.error_code.is_none());
                assert!(case.expected.redacted.is_none());
            }
            ("did.parse", "error") => {
                let error = Did::parse(&input).unwrap_err().to_identus_error();
                assert_eq!(
                    error.code().as_str(),
                    case.expected.error_code.as_deref().expect("error code")
                );
                assert_eq!(case.expected.redacted, Some(true));
                assert!(!error.to_string().contains(&input));
            }
            ("did-url.parse", "error") => {
                let error = DidUrl::parse(&input).unwrap_err().to_identus_error();
                assert_eq!(
                    error.code().as_str(),
                    case.expected.error_code.as_deref().expect("error code")
                );
                assert_eq!(case.expected.redacted, Some(true));
                assert!(!error.to_string().contains(&input));
            }
            (operation, kind) => panic!("{} has unsupported {operation}/{kind}", case.id),
        }
    }
}
