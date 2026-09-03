use std::{collections::BTreeMap, hint::black_box, time::Instant};

use identus_did::{
    ContextEntry, Did, DidDocument, DocumentError, Error, MAX_DID_DOCUMENT_BYTES,
    MAX_DOCUMENT_ITEMS, MAX_EXTENSION_DEPTH, MAX_EXTENSION_NODES, MAX_URI_BYTES, OneOrMany,
    Service, ServiceEndpoint, Uri, UriSyntaxError, VerificationMethod, VerificationRelationship,
};
use serde_json::{Value, json};

const REPRESENTATIVE_DOCUMENT: &str = r#"{
    "@context": [
        "https://www.w3.org/ns/did/v1",
        {"oxid": "https://oxid.example/ns#"}
    ],
    "id": "did:midnight:testnet:alice",
    "controller": ["did:midnight:testnet:alice", "did:prism:delegate"],
    "alsoKnownAs": ["https://alice.example/profile", "urn:uuid:1234"],
    "verificationMethod": [
        {
            "id": "did:midnight:testnet:alice#auth-1",
            "type": "JsonWebKey2020",
            "controller": "did:midnight:testnet:alice",
            "publicKeyJwk": {
                "kty": "OKP",
                "crv": "Ed25519",
                "x": "11qYAYdk9JTuG6urO8YUNELcD9q4G2O_TW9LtUo9G2M"
            },
            "expires": "2030-01-01T00:00:00Z"
        },
        {
            "id": "https://keys.example/alice/agreement-1",
            "type": "X25519KeyAgreementKey2020",
            "controller": "did:prism:delegate",
            "publicKeyMultibase": "z6LSbysY2xFMR4tQ"
        }
    ],
    "authentication": [
        "did:midnight:testnet:alice#auth-1",
        {
            "id": "did:midnight:testnet:alice#auth-local",
            "type": "Ed25519VerificationKey2020",
            "controller": "did:midnight:testnet:alice",
            "publicKeyMultibase": "z6MkiTBz1y"
        }
    ],
    "assertionMethod": ["did:midnight:testnet:alice#auth-1"],
    "keyAgreement": ["https://keys.example/alice/agreement-1"],
    "capabilityInvocation": ["did:midnight:testnet:alice#auth-1"],
    "capabilityDelegation": ["did:prism:delegate#capability-1"],
    "service": [
        {
            "id": "did:midnight:testnet:alice#linked-domain",
            "type": "LinkedDomains",
            "serviceEndpoint": "https://alice.example"
        },
        {
            "id": "did:midnight:testnet:alice#messages",
            "type": ["DIDCommMessaging", "OxidInbox"],
            "serviceEndpoint": [
                "https://inbox.example/alice",
                {"uri": "https://backup.example/alice", "accept": ["didcomm/v2"]}
            ],
            "routingKeys": ["did:example:mediator#key-1"]
        },
        {
            "id": "urn:example:service:profile",
            "type": "Profile",
            "serviceEndpoint": {"origins": ["https://alice.example"]}
        }
    ],
    "oxid:profile": {"locale": "uk-UA", "features": ["wallet", "identity"]}
}"#;

fn public_jwk() -> BTreeMap<String, Value> {
    BTreeMap::from([(
        "publicKeyJwk".to_owned(),
        json!({
                "kty": "OKP",
                "crv": "Ed25519",
                "x": "11qYAYdk9JTuG6urO8YUNELcD9q4G2O_TW9LtUo9G2M"
        }),
    )])
}

fn method(id: &str) -> VerificationMethod {
    VerificationMethod::new(
        Uri::parse(id).unwrap(),
        "JsonWebKey2020".to_owned(),
        Did::parse("did:example:123").unwrap(),
        public_jwk(),
    )
    .unwrap()
}

#[test]
fn uri_accepts_generic_absolute_schemes_and_preserves_allocation() {
    for value in [
        "https://user@example.com:443/a/b?x=1#fragment",
        "did:example:123#key-1",
        "urn:uuid:6ba7b810-9dad-11d1-80b4-00c04fd430c8",
        "custom:/path%20with%20space",
        "file:///tmp/example",
        "https://[2001:db8::1]:8443/path",
        "custom://[v1.fe80]/resource",
    ] {
        let uri = Uri::parse(value).unwrap();
        assert_eq!(uri.as_str(), value);
        assert_eq!(
            serde_json::from_str::<Uri>(&serde_json::to_string(&uri).unwrap()).unwrap(),
            uri
        );
    }

    let owned = "did:example:allocation#key".to_owned();
    let pointer = owned.as_ptr();
    let uri = Uri::try_from(owned).unwrap();
    assert_eq!(uri.as_str().as_ptr(), pointer);
    assert_eq!(uri.scheme(), "did");
    assert_eq!(uri.to_did_url().unwrap().fragment(), Some("key"));

    let did = Did::parse("did:example:moved").unwrap();
    let pointer = did.as_str().as_ptr();
    let uri = Uri::from(did);
    assert_eq!(uri.as_str().as_ptr(), pointer);
}

#[test]
fn uri_rejects_relative_malformed_and_oversized_values() {
    let cases = [
        ("relative/path", UriSyntaxError::MissingScheme),
        (":empty", UriSyntaxError::MissingScheme),
        ("1bad:value", UriSyntaxError::InvalidScheme),
        ("https://exa mple", UriSyntaxError::InvalidAuthority),
        ("https://user@@example", UriSyntaxError::InvalidAuthority),
        ("https://[not-ip]/", UriSyntaxError::InvalidAuthority),
        ("custom://[v1.]/", UriSyntaxError::InvalidAuthority),
        ("urn:value%2", UriSyntaxError::InvalidPercentEncoding),
        ("urn:value#one#two", UriSyntaxError::MultipleFragments),
        ("urn:válue", UriSyntaxError::InvalidCharacter),
    ];
    for (value, reason) in cases {
        assert_eq!(Uri::parse(value), Err(Error::InvalidUri(reason)));
    }

    let oversized = format!("urn:{}", "a".repeat(MAX_URI_BYTES));
    assert_eq!(
        Uri::parse(&oversized),
        Err(Error::InvalidUri(UriSyntaxError::TooLong))
    );
}

#[test]
fn representative_document_roundtrips_semantically() {
    let expected: Value = serde_json::from_str(REPRESENTATIVE_DOCUMENT).unwrap();
    let document = DidDocument::from_json_str(REPRESENTATIVE_DOCUMENT).unwrap();

    assert_eq!(document.id().as_str(), "did:midnight:testnet:alice");
    assert!(document.context().unwrap().is_many());
    assert_eq!(document.controller().unwrap().as_slice().len(), 2);
    assert_eq!(document.also_known_as().unwrap().len(), 2);
    assert_eq!(document.verification_methods().unwrap().len(), 2);
    assert_eq!(document.authentication().unwrap().len(), 2);
    assert_eq!(document.assertion_method().unwrap().len(), 1);
    assert_eq!(document.key_agreement().unwrap().len(), 1);
    assert_eq!(document.capability_invocation().unwrap().len(), 1);
    assert_eq!(document.capability_delegation().unwrap().len(), 1);
    assert_eq!(document.services().unwrap().len(), 3);
    assert!(document.extensions().contains_key("oxid:profile"));

    let methods = document.verification_methods().unwrap();
    assert!(methods[0].public_key_jwk().is_some());
    assert_eq!(methods[1].public_key_multibase(), Some("z6LSbysY2xFMR4tQ"));
    assert_eq!(methods[1].id().scheme(), "https");

    assert_eq!(serde_json::to_value(&document).unwrap(), expected);
    let serde_document: DidDocument = serde_json::from_value(expected).unwrap();
    assert_eq!(serde_document, document);
}

#[test]
fn native_builder_uses_the_same_structural_boundary() {
    let verification = method("did:example:123#key-1");
    let service = Service::new(
        Uri::parse("did:example:123#service").unwrap(),
        OneOrMany::one("LinkedDomains".to_owned()),
        ServiceEndpoint::Uri(Uri::parse("https://example.com").unwrap()),
        BTreeMap::new(),
    )
    .unwrap();
    let document = DidDocument::builder(Did::parse("did:example:123").unwrap())
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
        .controller(OneOrMany::one(
            Did::parse("did:example:controller").unwrap(),
        ))
        .also_known_as(vec![Uri::parse("urn:example:alias").unwrap()])
        .verification_methods(vec![verification])
        .authentication(vec![VerificationRelationship::Reference(
            Uri::parse("did:example:123#key-1").unwrap(),
        )])
        .services(vec![service])
        .extensions(BTreeMap::from([("example:flag".to_owned(), json!(true))]))
        .build()
        .unwrap();

    let json = serde_json::to_value(&document).unwrap();
    assert_eq!(
        DidDocument::from_json_str(&json.to_string()).unwrap(),
        document
    );
    assert_eq!(json["controller"], "did:example:controller");
    assert!(json["@context"].is_array());
}

#[test]
fn unknown_verification_suite_properties_remain_available() {
    let properties = BTreeMap::from([
        ("blockchainAccountId".to_owned(), json!("eip155:1:0x1234")),
        ("suiteMetadata".to_owned(), json!({"version": 2})),
    ]);
    let method = VerificationMethod::new(
        Uri::parse("https://keys.example/account/1").unwrap(),
        "BlockchainVerificationMethod2021".to_owned(),
        Did::parse("did:example:123").unwrap(),
        properties,
    )
    .unwrap();
    assert_eq!(method.properties()["suiteMetadata"]["version"], 2);
    assert_eq!(
        serde_json::to_value(&method).unwrap()["blockchainAccountId"],
        "eip155:1:0x1234"
    );
}

#[test]
fn verification_methods_reject_private_or_multiple_known_material() {
    for private_member in ["d", "p", "q", "dp", "dq", "qi", "oth", "k"] {
        let mut jwk = serde_json::Map::from_iter([("kty".to_owned(), json!("OKP"))]);
        jwk.insert(private_member.to_owned(), json!("secret-marker"));
        let result = VerificationMethod::new(
            Uri::parse("did:example:123#key").unwrap(),
            "JsonWebKey2020".to_owned(),
            Did::parse("did:example:123").unwrap(),
            BTreeMap::from([("publicKeyJwk".to_owned(), Value::Object(jwk))]),
        );
        assert!(matches!(
            result,
            Err(Error::InvalidDocument(DocumentError::PrivateKeyMaterial))
        ));
    }

    let mut properties = public_jwk();
    properties.insert("publicKeyMultibase".to_owned(), json!("z6Mk"));
    assert!(matches!(
        VerificationMethod::new(
            Uri::parse("did:example:123#key").unwrap(),
            "Example".to_owned(),
            Did::parse("did:example:123").unwrap(),
            properties,
        ),
        Err(Error::InvalidDocument(
            DocumentError::MultipleVerificationMaterial
        ))
    ));

    for properties in [
        BTreeMap::new(),
        BTreeMap::from([("publicKeyJwk".to_owned(), json!("not-a-map"))]),
        BTreeMap::from([("publicKeyJwk".to_owned(), json!({"crv": "Ed25519"}))]),
        BTreeMap::from([("publicKeyMultibase".to_owned(), json!("contains space"))]),
    ] {
        assert!(
            VerificationMethod::new(
                Uri::parse("did:example:123#invalid").unwrap(),
                "Example".to_owned(),
                Did::parse("did:example:123").unwrap(),
                properties,
            )
            .is_err()
        );
    }
}

#[test]
fn malformed_wire_cardinalities_and_service_shapes_are_rejected() {
    for document in [
        json!({"id": "did:example:123", "controller": []}),
        json!({"id": "did:example:123", "alsoKnownAs": []}),
        json!({"id": "did:example:123", "verificationMethod": []}),
        json!({"id": "did:example:123", "authentication": []}),
        json!({"id": "did:example:123", "service": []}),
        json!({
                "id": "did:example:123",
                "service": [{
                        "id": "did:example:123#service",
                        "type": [],
                        "serviceEndpoint": "https://example.com"
                }]
        }),
        json!({
                "id": "did:example:123",
                "service": [{
                        "id": "did:example:123#service",
                        "type": "Example",
                        "serviceEndpoint": []
                }]
        }),
        json!({
                "id": "did:example:123",
                "service": [{
                        "id": "did:example:123#service",
                        "type": "Example",
                        "serviceEndpoint": 42
                }]
        }),
    ] {
        let wire = document.to_string();
        assert!(
            DidDocument::from_json_str(&wire).is_err(),
            "accepted {wire}"
        );
        assert!(
            serde_json::from_str::<DidDocument>(&wire).is_err(),
            "serde accepted {wire}"
        );
    }

    let duplicate_type = Service::new(
        Uri::parse("did:example:123#service").unwrap(),
        OneOrMany::try_many(vec!["Example".to_owned(), "Example".to_owned()]).unwrap(),
        ServiceEndpoint::Uri(Uri::parse("https://example.com").unwrap()),
        BTreeMap::new(),
    );
    assert!(matches!(
        duplicate_type,
        Err(Error::InvalidDocument(DocumentError::DuplicateSetMember))
    ));
}

#[test]
fn document_rejects_duplicate_resource_and_set_identifiers() {
    let duplicate_method = DidDocument::builder(Did::parse("did:example:123").unwrap())
        .verification_methods(vec![
            method("did:example:123#key"),
            method("did:example:123#key"),
        ])
        .build();
    assert!(matches!(
        duplicate_method,
        Err(Error::InvalidDocument(
            DocumentError::DuplicateVerificationMethod
        ))
    ));

    let service = || {
        Service::new(
            Uri::parse("did:example:123#service").unwrap(),
            OneOrMany::one("Example".to_owned()),
            ServiceEndpoint::Uri(Uri::parse("https://example.com").unwrap()),
            BTreeMap::new(),
        )
        .unwrap()
    };
    let duplicate_service = DidDocument::builder(Did::parse("did:example:123").unwrap())
        .services(vec![service(), service()])
        .build();
    assert!(matches!(
        duplicate_service,
        Err(Error::InvalidDocument(DocumentError::DuplicateService))
    ));

    let duplicate_reference = DidDocument::builder(Did::parse("did:example:123").unwrap())
        .authentication(vec![
            VerificationRelationship::Reference(Uri::parse("did:example:123#key").unwrap()),
            VerificationRelationship::Reference(Uri::parse("did:example:123#key").unwrap()),
        ])
        .build();
    assert!(matches!(
        duplicate_reference,
        Err(Error::InvalidDocument(DocumentError::DuplicateSetMember))
    ));
}

#[test]
fn resource_and_collision_limits_apply_to_native_and_wire_inputs() {
    let oversized = format!(
        "{{\"id\":\"did:example:123\"}}{}",
        " ".repeat(MAX_DID_DOCUMENT_BYTES)
    );
    assert_eq!(
        DidDocument::from_json_str(&oversized),
        Err(Error::InvalidDocument(DocumentError::TooLarge))
    );

    let aliases = (0..=MAX_DOCUMENT_ITEMS)
        .map(|index| Uri::parse(&format!("urn:example:{index}")).unwrap())
        .collect();
    assert!(matches!(
        DidDocument::builder(Did::parse("did:example:123").unwrap())
            .also_known_as(aliases)
            .build(),
        Err(Error::InvalidDocument(DocumentError::TooManyItems))
    ));

    assert!(matches!(
        DidDocument::builder(Did::parse("did:example:123").unwrap())
            .authentication(Vec::new())
            .build(),
        Err(Error::InvalidDocument(DocumentError::EmptyValue))
    ));

    assert!(matches!(
        DidDocument::builder(Did::parse("did:example:123").unwrap())
            .extensions(BTreeMap::from([("id".to_owned(), json!("shadow"))]))
            .build(),
        Err(Error::InvalidDocument(DocumentError::ReservedProperty))
    ));

    let too_many_properties = (0..65)
        .map(|index| (format!("property-{index}"), json!(index)))
        .collect();
    assert!(matches!(
        DidDocument::builder(Did::parse("did:example:123").unwrap())
            .extensions(too_many_properties)
            .build(),
        Err(Error::InvalidDocument(DocumentError::TooManyProperties))
    ));

    assert!(matches!(
        DidDocument::builder(Did::parse("did:example:123").unwrap())
            .extensions(BTreeMap::from([("x".repeat(257), json!(true))]))
            .build(),
        Err(Error::InvalidDocument(DocumentError::InvalidPropertyName))
    ));

    assert!(matches!(
        DidDocument::builder(Did::parse("did:example:123").unwrap())
            .extensions(BTreeMap::from([(
                "large".to_owned(),
                json!("x".repeat(64 * 1_024 + 1)),
            )]))
            .build(),
        Err(Error::InvalidDocument(DocumentError::InvalidString))
    ));
}

#[test]
fn extension_depth_and_node_budgets_are_enforced() {
    let mut deep = json!(true);
    for _ in 0..=MAX_EXTENSION_DEPTH {
        deep = json!([deep]);
    }
    assert!(matches!(
        DidDocument::builder(Did::parse("did:example:123").unwrap())
            .extensions(BTreeMap::from([("deep".to_owned(), deep)]))
            .build(),
        Err(Error::InvalidDocument(DocumentError::ExtensionTooDeep))
    ));

    let wide = (0..MAX_EXTENSION_NODES)
        .map(|index| json!(index))
        .collect::<Vec<_>>();
    let nested = wide
        .chunks(MAX_DOCUMENT_ITEMS)
        .map(|chunk| Value::Array(chunk.to_vec()))
        .collect::<Vec<_>>();
    assert!(matches!(
        DidDocument::builder(Did::parse("did:example:123").unwrap())
            .extensions(BTreeMap::from([("wide".to_owned(), Value::Array(nested))]))
            .build(),
        Err(Error::InvalidDocument(DocumentError::ExtensionTooLarge))
    ));
}

#[test]
fn stable_document_errors_are_redacted() {
    let marker = "caller-secret-marker";
    let malformed = format!("{{\"id\":\"did:example:{marker}\"");
    let error = DidDocument::from_json_str(&malformed).unwrap_err();
    let public = error.to_identus_error();
    assert_eq!(public.code().as_str(), "did.invalid_document");
    assert_eq!(public.capability().unwrap().as_str(), "did");
    assert!(!public.to_string().contains(marker));
    assert!(!error.to_string().contains(marker));
}

#[test]
#[ignore = "manual release-mode performance diagnostic"]
fn document_parse_throughput_diagnostic() {
    const ITERATIONS: usize = 100_000;
    let started = Instant::now();
    for _ in 0..ITERATIONS {
        black_box(DidDocument::from_json_str(black_box(REPRESENTATIVE_DOCUMENT)).unwrap());
    }
    let elapsed = started.elapsed();
    let per_second = ITERATIONS as f64 / elapsed.as_secs_f64();
    eprintln!(
        "parsed {ITERATIONS} representative DID documents in {elapsed:?} ({per_second:.0} documents/s)"
    );
}
