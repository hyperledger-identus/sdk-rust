use std::collections::BTreeMap;

use identus_did::{
    ContextEntry, Did, DidDocument, DocumentError, Error, MAX_DID_DOCUMENT_BYTES,
    MAX_DID_DOCUMENT_WIRE_DEPTH, MAX_DID_DOCUMENT_WIRE_LIVE_KEY_BYTES, MAX_DID_DOCUMENT_WIRE_NODES,
    MAX_DID_DOCUMENT_WIRE_OBJECT_MEMBERS, MAX_EXTENSION_DEPTH, MAX_EXTENSION_NODES, OneOrMany,
    Service, ServiceEndpoint, ServiceEndpointValue, Uri, VerificationMethod,
    VerificationRelationship,
};
use serde_json::{Value, json};
use uriparse::URI;

const _: () = {
    assert!(MAX_DID_DOCUMENT_WIRE_DEPTH > MAX_EXTENSION_DEPTH);
    assert!(MAX_DID_DOCUMENT_WIRE_NODES > MAX_EXTENSION_NODES);
    assert!(MAX_DID_DOCUMENT_WIRE_OBJECT_MEMBERS > 64);
    assert!(MAX_DID_DOCUMENT_WIRE_LIVE_KEY_BYTES < MAX_DID_DOCUMENT_BYTES);
};

fn oracle_decision(value: &str) -> Option<bool> {
    std::panic::catch_unwind(|| URI::try_from(value).is_ok()).ok()
}

fn oracle_accepts(value: &str) -> bool {
    oracle_decision(value).unwrap_or_else(|| panic!("uriparse 0.6.4 panicked for URI: {value}"))
}

fn sdk_accepts(value: &str) -> bool {
    Uri::parse(value).is_ok()
}

#[test]
fn uri_recognizer_agrees_with_neoprism_oracle_across_component_classes() {
    let schemes = ["a", "did", "HTTP", "x+v1", "git.transport"];
    let hier_parts = [
        "",
        "rootless",
        "rootless/child",
        "/absolute/path",
        "//example.com",
        "//user:pass@example.com:443/path",
        "//[2001:db8::1]:8443/path",
        "///empty-authority",
    ];
    let queries = ["", "?", "?name=value", "?encoded=%20", "?a/b?c"];
    let fragments = ["", "#", "#key-1", "#encoded%20value", "#a/b?c"];

    let mut compared = 0_usize;
    for scheme in schemes {
        for hier_part in hier_parts {
            for query in queries {
                for fragment in fragments {
                    let value = format!("{scheme}:{hier_part}{query}{fragment}");
                    assert_eq!(sdk_accepts(&value), oracle_accepts(&value), "{value}");
                    compared += 1;
                }
            }
        }
    }
    assert_eq!(compared, 1_000);

    for value in [
        "x:value%",
        "x:value%2",
        "x://user@@example.com",
        "x://[v1.]/",
        "x://[not-ip]/",
        "x:path with space",
        "x:path#first#second",
    ] {
        assert_eq!(sdk_accepts(value), oracle_accepts(value), "{value}");
        assert!(!sdk_accepts(value));
    }
}

#[test]
fn uri_oracle_differences_are_explicitly_classified() {
    let oversized = format!("x:{}", "a".repeat(4_095));
    assert!(oracle_accepts(&oversized));
    assert!(!sdk_accepts(&oversized));

    let exact = "HTTP://Example.COM/%7euser?x=%2f#Fragment";
    let uri = Uri::parse(exact).unwrap();
    assert!(oracle_accepts(exact));
    assert_eq!(uri.as_str(), exact);

    // RFC 3986 section 3.2.2 defines IPvFuture, which uriparse 0.6.4 does not
    // recognize. The standards-derived SDK parser intentionally accepts it.
    let ipv_future = "custom://[v1.fe80]:9/resource";
    assert!(sdk_accepts(ipv_future));
    assert!(!oracle_accepts(ipv_future));

    // The oracle also panics on a leading-digit scheme instead of returning an
    // error. The SDK rejects the same malformed input without unwinding.
    assert_eq!(oracle_decision("1bad:value"), None);
    assert!(!sdk_accepts("1bad:value"));
}

fn assert_duplicate(input: &str) {
    let error = DidDocument::from_json_str(input).unwrap_err();
    assert_eq!(
        error,
        Error::InvalidDocument(DocumentError::DuplicateJsonProperty)
    );
    assert_eq!(
        error.to_string(),
        "invalid DID document: DID document JSON contains a duplicate property"
    );
    assert_eq!(
        error.to_identus_error().code().as_str(),
        "did.invalid_document"
    );
}

#[test]
fn raw_document_rejects_duplicates_at_every_security_relevant_map_boundary() {
    for input in [
        r#"{"id":"did:example:one","id":"did:example:two"}"#,
        r#"{"id":"did:example:one","@context":{"term":"urn:one","term":"urn:two"}}"#,
        r#"{"id":"did:example:one","verificationMethod":[{"id":"did:example:one#one","id":"did:example:one#two","type":"Example","controller":"did:example:one","publicKeyMultibase":"z6Mk"}]}"#,
        r#"{"id":"did:example:one","verificationMethod":[{"id":"did:example:one#one","type":"JsonWebKey2020","controller":"did:example:one","publicKeyJwk":{"kty":"OKP","x":"one","x":"two"}}]}"#,
        r#"{"id":"did:example:one","service":[{"id":"did:example:one#service","type":"One","type":"Two","serviceEndpoint":"https://example.com"}]}"#,
        r#"{"id":"did:example:one","service":[{"id":"did:example:one#service","type":"Example","serviceEndpoint":{"uri":"https://one.example","uri":"https://two.example"}}]}"#,
        r#"{"id":"did:example:one","profile":{"nested":{"flag":true,"flag":false}}}"#,
    ] {
        assert_duplicate(input);
    }
}

#[test]
fn duplicate_equality_uses_decoded_names_and_stays_object_local() {
    assert_duplicate(r#"{"id":"did:example:one","profile":{"name":1,"\u006eame":2}}"#);

    let valid = r#"{
        "id":"did:example:one",
        "profile":{"left":{"name":1},"right":{"name":2}}
    }"#;
    let document = DidDocument::from_json_str(valid).unwrap();
    assert_eq!(document.extensions()["profile"]["left"]["name"], 1);
    assert_eq!(document.extensions()["profile"]["right"]["name"], 2);
}

#[test]
fn duplicate_diagnostics_do_not_expose_caller_controlled_names() {
    let marker = "caller-secret-marker";
    let input = format!(r#"{{"id":"did:example:one","profile":{{"{marker}":1,"{marker}":2}}}}"#);
    let error = DidDocument::from_json_str(&input).unwrap_err();

    for rendered in [
        error.to_string(),
        format!("{error:?}"),
        error.to_identus_error().to_string(),
    ] {
        assert!(
            !rendered.contains(marker),
            "diagnostic leaked input: {rendered}"
        );
    }
}

#[test]
fn raw_scanner_limits_and_complete_input_fail_before_typed_construction() {
    let mut too_deep = "0".to_owned();
    for _ in 0..=MAX_DID_DOCUMENT_WIRE_DEPTH {
        too_deep = format!("[{too_deep}]");
    }
    assert_eq!(
        DidDocument::from_json_str(&too_deep),
        Err(Error::InvalidDocument(DocumentError::ExtensionTooDeep))
    );

    let too_many_nodes = format!(
        "[{}]",
        std::iter::repeat_n("0", MAX_DID_DOCUMENT_WIRE_NODES)
            .collect::<Vec<_>>()
            .join(",")
    );
    assert!(too_many_nodes.len() < MAX_DID_DOCUMENT_BYTES);
    assert_eq!(
        DidDocument::from_json_str(&too_many_nodes),
        Err(Error::InvalidDocument(DocumentError::ExtensionTooLarge))
    );

    let too_many_members = format!(
        "{{{}}}",
        (0..=MAX_DID_DOCUMENT_WIRE_OBJECT_MEMBERS)
            .map(|index| format!("\"p{index}\":null"))
            .collect::<Vec<_>>()
            .join(",")
    );
    assert_eq!(
        DidDocument::from_json_str(&too_many_members),
        Err(Error::InvalidDocument(DocumentError::TooManyProperties))
    );

    let key_body = "k".repeat((MAX_DID_DOCUMENT_WIRE_LIVE_KEY_BYTES / 128) + 1);
    let too_many_live_key_bytes = format!(
        "{{{}}}",
        (0..MAX_DID_DOCUMENT_WIRE_OBJECT_MEMBERS)
            .map(|index| format!("\"{index:03}{key_body}\":null"))
            .collect::<Vec<_>>()
            .join(",")
    );
    assert!(too_many_live_key_bytes.len() < MAX_DID_DOCUMENT_BYTES);
    assert_eq!(
        DidDocument::from_json_str(&too_many_live_key_bytes),
        Err(Error::InvalidDocument(DocumentError::ExtensionTooLarge))
    );

    for malformed in [
        r#"{"id":"did:example:one"} trailing"#,
        r#"{"id":"did:example:unterminated}"#,
    ] {
        assert_eq!(
            DidDocument::from_json_str(malformed),
            Err(Error::InvalidDocument(DocumentError::MalformedJson))
        );
    }
}

fn generated_extension(seed: usize) -> Value {
    let mut value = json!({"seed": seed, "enabled": seed % 2 == 0});
    for depth in 0..seed % 6 {
        value = json!({format!("level-{depth}"): [value, depth]});
    }
    value
}

fn generated_method(did: &Did, seed: usize) -> VerificationMethod {
    VerificationMethod::new(
        Uri::parse(&format!("{}#key-{seed}", did.as_str())).unwrap(),
        "ExampleVerificationMethod2026".to_owned(),
        did.clone(),
        BTreeMap::from([(
            "publicKeyMultibase".to_owned(),
            json!(format!("z6Mk{seed:04}")),
        )]),
    )
    .unwrap()
}

#[test]
fn deterministic_generated_documents_preserve_native_semantic_wire_equivalence() {
    for seed in 0..256 {
        let did = Did::parse(&format!("did:example:generated-{seed}")).unwrap();
        let method = generated_method(&did, seed);
        let endpoint = match seed % 3 {
            0 => ServiceEndpoint::Uri(Uri::parse("https://example.com/endpoint").unwrap()),
            1 => ServiceEndpoint::Map(BTreeMap::from([(
                "uri".to_owned(),
                json!("https://example.com/endpoint"),
            )])),
            _ => ServiceEndpoint::Set(vec![
                ServiceEndpointValue::Uri(Uri::parse("https://example.com/one").unwrap()),
                ServiceEndpointValue::Map(BTreeMap::from([(
                    "uri".to_owned(),
                    json!("https://example.com/two"),
                )])),
            ]),
        };
        let service = Service::new(
            Uri::parse(&format!("{}#service-{seed}", did.as_str())).unwrap(),
            if seed % 2 == 0 {
                OneOrMany::one("ExampleService".to_owned())
            } else {
                OneOrMany::try_many(vec!["ExampleService".to_owned(), format!("Profile{seed}")])
                    .unwrap()
            },
            endpoint,
            BTreeMap::from([("profile".to_owned(), generated_extension(seed))]),
        )
        .unwrap();
        let document = DidDocument::builder(did.clone())
            .context(
                OneOrMany::try_many(vec![
                    ContextEntry::Uri(Uri::parse("https://www.w3.org/ns/did/v1").unwrap()),
                    ContextEntry::Object(BTreeMap::from([(
                        "example".to_owned(),
                        json!("https://example.com/ns#"),
                    )])),
                ])
                .unwrap(),
            )
            .also_known_as(vec![
                Uri::parse(&format!("urn:example:alias-{seed}")).unwrap(),
            ])
            .verification_methods(vec![method])
            .authentication(vec![VerificationRelationship::Reference(
                Uri::parse(&format!("{}#key-{seed}", did.as_str())).unwrap(),
            )])
            .services(vec![service])
            .extensions(BTreeMap::from([(
                "generated".to_owned(),
                generated_extension(seed),
            )]))
            .build()
            .unwrap();

        let wire = serde_json::to_vec(&document).unwrap();
        assert_eq!(DidDocument::from_json_slice(&wire).unwrap(), document);
        assert_eq!(
            serde_json::from_value::<DidDocument>(serde_json::to_value(&document).unwrap())
                .unwrap(),
            document
        );
    }
}
