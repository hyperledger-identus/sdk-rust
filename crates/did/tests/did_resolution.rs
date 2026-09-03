use std::{collections::BTreeMap, hint::black_box, time::Instant};

use identus_did::{
    DereferencedContent, Did, DidDocument, DidDocumentMetadata, DidResolutionDateTime,
    DidResolutionError, DidResolutionErrorKind, DidResolutionMetadata, DidResolutionResult,
    DidUrlContentMetadata, DidUrlDereferencingMetadata, DidUrlDereferencingResult, DocumentError,
    Error, MAX_DID_RESOLUTION_RESULT_BYTES, MAX_EXTENSION_DEPTH, MAX_EXTENSION_NODES,
    MAX_MEDIA_TYPE_BYTES, MAX_PROBLEM_DETAIL_BYTES, MAX_VERSION_ID_BYTES, MediaType,
    ResolutionError, Service, Uri, VerificationMethod, VersionId,
};
use serde_json::{Value, json};

const SUCCESS_RESULT: &str = r#"{
    "didResolutionMetadata": {
        "contentType": "application/did+ld+json;profile=\"https://www.w3.org/ns/did/v1\"",
        "retrieved": "2026-09-03T01:02:03Z"
    },
    "didDocument": {
        "@context": "https://www.w3.org/ns/did/v1",
        "id": "did:midnight:testnet:alice",
        "verificationMethod": [{
            "id": "did:midnight:testnet:alice#key-1",
            "type": "JsonWebKey2020",
            "controller": "did:midnight:testnet:alice",
            "publicKeyJwk": {"kty": "OKP", "crv": "Ed25519", "x": "abc"}
        }],
        "service": [{
            "id": "did:midnight:testnet:alice#messages",
            "type": "DIDCommMessaging",
            "serviceEndpoint": "https://wallet.example/messages"
        }]
    },
    "didDocumentMetadata": {
        "created": "2026-09-01T00:00:00Z",
        "updated": "2026-09-02T23:59:59Z",
        "nextUpdate": "2026-09-04T00:00:00Z",
        "versionId": "block-42:7",
        "nextVersionId": "block-43:0",
        "equivalentId": ["did:midnight:alice", "did:midnight:testnet:alice-v2"],
        "canonicalId": "did:midnight:alice",
        "proof": [{"type": "DataIntegrityProof", "proofValue": "zExample"}],
        "ledger": {"block": 42, "transaction": "0x123"}
    }
}"#;

fn did(value: &str) -> Did {
    Did::parse(value).unwrap()
}

fn media_type() -> MediaType {
    MediaType::parse("application/did").unwrap()
}

fn success_metadata() -> DidResolutionMetadata {
    DidResolutionMetadata::new(Some(media_type()), None, BTreeMap::new()).unwrap()
}

fn failure_metadata(kind: DidResolutionErrorKind) -> DidResolutionMetadata {
    DidResolutionMetadata::new(
        None,
        Some(DidResolutionError::standard(kind)),
        BTreeMap::new(),
    )
    .unwrap()
}

fn minimal_document(value: &str) -> DidDocument {
    DidDocument::from_json_str(&format!(r#"{{"id":"{value}"}}"#)).unwrap()
}

#[test]
fn current_w3c_success_result_roundtrips_semantically() {
    let expected: Value = serde_json::from_str(SUCCESS_RESULT).unwrap();
    let requested = did("did:midnight:testnet:alice");
    let result =
        DidResolutionResult::from_json_slice_for(SUCCESS_RESULT.as_bytes(), &requested).unwrap();

    assert_eq!(
        result.metadata().content_type().unwrap().as_str(),
        "application/did+ld+json;profile=\"https://www.w3.org/ns/did/v1\""
    );
    assert_eq!(result.document().unwrap().id(), &requested);
    assert_eq!(
        result.document_metadata().version_id().unwrap().as_str(),
        "block-42:7"
    );
    assert_eq!(
        result.document_metadata().equivalent_ids().unwrap().len(),
        2
    );
    assert!(
        result
            .document_metadata()
            .extensions()
            .contains_key("proof")
    );
    assert_eq!(serde_json::to_value(&result).unwrap(), expected);
    assert_eq!(
        serde_json::from_value::<DidResolutionResult>(expected).unwrap(),
        result
    );
}

#[test]
fn ordinary_failure_and_deactivation_are_distinct_states() {
    let failure = DidResolutionResult::from_json_str(
        r#"{
            "didResolutionMetadata": {
                "error": {
                    "type": "https://www.w3.org/ns/did#NOT_FOUND",
                    "title": "DID not found",
                    "detail": "No active version is available",
                    "instance": "urn:uuid:0f50e2b9-31e8-4fc8-88bb-0894c0a1b50a",
                    "retryable": false
                }
            },
            "didDocument": null,
            "didDocumentMetadata": {}
        }"#,
    )
    .unwrap();
    let error = failure.metadata().error().unwrap();
    assert_eq!(error.kind(), Some(DidResolutionErrorKind::NotFound));
    assert_eq!(error.title(), Some("DID not found"));
    assert_eq!(error.extensions()["retryable"], false);
    assert!(failure.document_metadata().is_empty());

    let deactivated = DidResolutionResult::from_json_str(
        r#"{
            "didResolutionMetadata": {"source": "ledger"},
            "didDocument": null,
            "didDocumentMetadata": {"deactivated": true, "versionId": "42"}
        }"#,
    )
    .unwrap();
    assert!(deactivated.metadata().error().is_none());
    assert_eq!(deactivated.document_metadata().deactivated(), Some(true));
    deactivated
        .validate_for(&did("did:prism:deactivated"))
        .unwrap();
}

#[test]
fn all_standard_error_urls_and_legacy_helpers_are_explicit() {
    let cases = [
        (DidResolutionErrorKind::InvalidDid, "invalidDid"),
        (
            DidResolutionErrorKind::InvalidDidDocument,
            "invalidDidDocument",
        ),
        (DidResolutionErrorKind::NotFound, "notFound"),
        (
            DidResolutionErrorKind::RepresentationNotSupported,
            "representationNotSupported",
        ),
        (DidResolutionErrorKind::InvalidDidUrl, "invalidDidUrl"),
        (
            DidResolutionErrorKind::MethodNotSupported,
            "methodNotSupported",
        ),
        (DidResolutionErrorKind::InvalidOptions, "invalidOptions"),
        (DidResolutionErrorKind::InternalError, "internalError"),
        (
            DidResolutionErrorKind::FeatureNotSupported,
            "featureNotSupported",
        ),
    ];

    for (kind, legacy) in cases {
        let standard = DidResolutionError::standard(kind);
        assert_eq!(standard.type_uri().as_str(), kind.as_str());
        assert_eq!(standard.kind(), Some(kind));
        let migrated = DidResolutionError::from_legacy_keyword(legacy).unwrap();
        assert_eq!(migrated, standard);
        assert_eq!(
            serde_json::from_str::<DidResolutionError>(&serde_json::to_string(&standard).unwrap())
                .unwrap(),
            standard
        );
    }

    assert!(DidResolutionError::from_legacy_keyword("unknownError").is_err());
    assert!(serde_json::from_str::<DidResolutionError>(r#""notFound""#).is_err());
    assert!(
        DidResolutionResult::from_json_str(
            r#"{
            "didResolutionMetadata":{"error":"notFound"},
            "didDocument":null,
            "didDocumentMetadata":{}
        }"#
        )
        .is_err()
    );

    let extension = DidResolutionError::new(
        Uri::parse("https://resolver.example/errors/temporarily-unavailable").unwrap(),
        Some("Temporarily unavailable".to_owned()),
        None,
        None,
        BTreeMap::from([("retryAfter".to_owned(), json!(30))]),
    )
    .unwrap();
    assert_eq!(extension.kind(), None);
    assert_eq!(
        serde_json::from_value::<DidResolutionError>(serde_json::to_value(&extension).unwrap())
            .unwrap(),
        extension
    );
}

#[test]
fn media_datetime_and_version_values_are_bounded_and_roundtrip() {
    for value in [
        "application/did",
        "application/did+ld+json;profile=\"https://www.w3.org/ns/did/v1\"",
        "text/plain; charset=utf-8; format=flowed",
        "application/vnd.example-wallet+json;v=2",
    ] {
        let parsed = MediaType::parse(value).unwrap();
        assert_eq!(parsed.as_str(), value);
        assert_eq!(
            serde_json::from_str::<MediaType>(&serde_json::to_string(&parsed).unwrap()).unwrap(),
            parsed
        );
    }
    for invalid in [
        "",
        "application",
        "/json",
        "application/",
        "application/json;",
        "application/json;charset",
        "application/json;charset=\"unterminated",
        "application/json\n",
    ] {
        assert!(matches!(
            MediaType::parse(invalid),
            Err(Error::InvalidResolution(ResolutionError::InvalidMediaType))
        ));
    }
    assert!(MediaType::parse(&format!("text/{}", "x".repeat(MAX_MEDIA_TYPE_BYTES))).is_err());

    for value in [
        "2024-02-29T23:59:59Z",
        "2026-09-03T00:00:00Z",
        "2000-02-29T12:34:56Z",
    ] {
        assert_eq!(DidResolutionDateTime::parse(value).unwrap().as_str(), value);
    }
    for invalid in [
        "2023-02-29T00:00:00Z",
        "2026-13-01T00:00:00Z",
        "2026-01-00T00:00:00Z",
        "2026-01-01T24:00:00Z",
        "2026-01-01T00:60:00Z",
        "2026-01-01T00:00:60Z",
        "2026-01-01T00:00:00.1Z",
        "2026-01-01T00:00:00+00:00",
    ] {
        assert!(matches!(
            DidResolutionDateTime::parse(invalid),
            Err(Error::InvalidResolution(ResolutionError::InvalidDateTime))
        ));
    }

    for value in ["1", "block-42:7", "opaque version"] {
        assert_eq!(VersionId::parse(value).unwrap().as_str(), value);
    }
    for invalid in ["", " leading", "trailing ", "line\nbreak", "ümlaut"] {
        assert!(VersionId::parse(invalid).is_err());
    }
    assert!(VersionId::parse(&"v".repeat(MAX_VERSION_ID_BYTES + 1)).is_err());
}

#[test]
fn native_metadata_and_result_construction_enforces_identity() {
    let document = minimal_document("did:prism:alice");
    let metadata = DidDocumentMetadata::builder()
        .created(DidResolutionDateTime::parse("2026-09-01T00:00:00Z").unwrap())
        .updated(DidResolutionDateTime::parse("2026-09-02T00:00:00Z").unwrap())
        .deactivated(false)
        .version_id(VersionId::parse("42").unwrap())
        .next_version_id(VersionId::parse("43").unwrap())
        .equivalent_ids(vec![did("did:prism:alice:long")])
        .canonical_id(did("did:prism:alice"))
        .extensions(BTreeMap::from([("blockHeight".to_owned(), json!(42))]))
        .build()
        .unwrap();
    let result = DidResolutionResult::success(success_metadata(), document, metadata).unwrap();
    result.validate_for(&did("did:prism:alice")).unwrap();
    assert_eq!(
        result
            .document_metadata()
            .next_version_id()
            .unwrap()
            .as_str(),
        "43"
    );

    assert!(matches!(
        result.validate_for(&did("did:prism:bob")),
        Err(Error::InvalidResolution(
            ResolutionError::DocumentIdMismatch
        ))
    ));

    let cross_method = DidDocumentMetadata::builder()
        .canonical_id(did("did:midnight:alice"))
        .build()
        .unwrap();
    assert!(matches!(
        DidResolutionResult::success(
            success_metadata(),
            minimal_document("did:prism:alice"),
            cross_method
        ),
        Err(Error::InvalidResolution(
            ResolutionError::DifferentDidMethod
        ))
    ));

    assert!(
        DidDocumentMetadata::builder()
            .equivalent_ids(vec![did("did:prism:alice"), did("did:prism:alice")])
            .build()
            .is_err()
    );
    assert!(
        DidDocumentMetadata::builder()
            .equivalent_ids(Vec::new())
            .build()
            .is_err()
    );
}

#[test]
fn contradictory_or_incomplete_resolution_states_fail_closed() {
    let success_with_error = DidResolutionResult::success(
        failure_metadata(DidResolutionErrorKind::NotFound),
        minimal_document("did:example:123"),
        DidDocumentMetadata::empty(),
    );
    assert!(matches!(
        success_with_error,
        Err(Error::InvalidResolution(ResolutionError::InvalidState))
    ));
    assert!(DidResolutionResult::failure(success_metadata()).is_err());
    assert!(
        DidResolutionResult::deactivated(
            success_metadata(),
            DidDocumentMetadata::builder()
                .deactivated(false)
                .build()
                .unwrap()
        )
        .is_err()
    );

    for (index, invalid) in [
        r#"{"didResolutionMetadata":{},"didDocument":{"id":"did:example:123"},"didDocumentMetadata":{"deactivated":true}}"#,
        r#"{"didResolutionMetadata":{"error":{"type":"https://www.w3.org/ns/did#NOT_FOUND"}},"didDocument":null,"didDocumentMetadata":{"versionId":"1"}}"#,
        r#"{"didResolutionMetadata":{},"didDocument":null,"didDocumentMetadata":{}}"#,
        r#"{"didResolutionMetadata":{},"didDocumentMetadata":{"deactivated":true}}"#,
    ]
    .into_iter()
    .enumerate()
    {
        assert!(
            DidResolutionResult::from_json_str(invalid).is_err(),
            "invalid resolution case {index} was accepted"
        );
    }
}

#[test]
fn dereferencing_preserves_open_content_and_known_projections() {
    let document = minimal_document("did:example:123");
    let content = DereferencedContent::from_did_document(&document).unwrap();
    assert_eq!(content.to_did_document().unwrap(), document);

    let result = DidUrlDereferencingResult::success(
        DidUrlDereferencingMetadata::new(Some(media_type()), None, BTreeMap::new()).unwrap(),
        content,
        DidUrlContentMetadata::new(BTreeMap::from([("versionId".to_owned(), json!("7"))])).unwrap(),
    )
    .unwrap();
    let wire = serde_json::to_value(&result).unwrap();
    assert!(wire.get("didUrlDereferencingMetadata").is_some());
    assert!(wire.get("dereferencingMetadata").is_none());
    assert_eq!(
        serde_json::from_value::<DidUrlDereferencingResult>(wire).unwrap(),
        result
    );

    let method: VerificationMethod = serde_json::from_value(json!({
        "id": "did:example:123#key-1",
        "type": "Multikey",
        "controller": "did:example:123",
        "publicKeyMultibase": "z6Mexample"
    }))
    .unwrap();
    assert_eq!(
        DereferencedContent::from_verification_method(&method)
            .unwrap()
            .to_verification_method()
            .unwrap(),
        method
    );

    let service: Service = serde_json::from_value(json!({
        "id": "did:example:123#service",
        "type": "LinkedDomains",
        "serviceEndpoint": "https://example.com"
    }))
    .unwrap();
    assert_eq!(
        DereferencedContent::from_service(&service)
            .unwrap()
            .to_service()
            .unwrap(),
        service
    );

    let uri = Uri::parse("https://example.com/resource").unwrap();
    assert_eq!(
        DereferencedContent::from_uri(&uri)
            .unwrap()
            .to_uri()
            .unwrap(),
        uri
    );

    let open = DereferencedContent::new(json!({
        "type": "FutureResource",
        "payload": [1, 2, 3]
    }))
    .unwrap();
    assert_eq!(open.as_value()["payload"], json!([1, 2, 3]));
    assert!(open.to_did_document().is_err());
    assert!(DereferencedContent::new(Value::Null).is_err());
}

#[test]
fn dereferencing_failure_and_metadata_conversion_are_validated() {
    let document_metadata = DidDocumentMetadata::builder()
        .created(DidResolutionDateTime::parse("2026-09-03T01:02:03Z").unwrap())
        .version_id(VersionId::parse("midnight:42").unwrap())
        .extensions(BTreeMap::from([("network".to_owned(), json!("testnet"))]))
        .build()
        .unwrap();
    let content_metadata =
        DidUrlContentMetadata::from_document_metadata(&document_metadata).unwrap();
    assert_eq!(
        content_metadata.to_document_metadata().unwrap(),
        document_metadata
    );

    let failure = DidUrlDereferencingResult::failure(
        DidUrlDereferencingMetadata::new(
            None,
            Some(DidResolutionError::standard(
                DidResolutionErrorKind::InvalidDidUrl,
            )),
            BTreeMap::from([("resolver".to_owned(), json!("local"))]),
        )
        .unwrap(),
    )
    .unwrap();
    assert!(failure.content().is_none());
    assert!(failure.content_metadata().is_empty());

    assert!(DidUrlDereferencingResult::failure(DidUrlDereferencingMetadata::empty()).is_err());
    assert!(
        DidUrlDereferencingResult::success(
            DidUrlDereferencingMetadata::new(
                None,
                Some(DidResolutionError::standard(
                    DidResolutionErrorKind::NotFound
                )),
                BTreeMap::new()
            )
            .unwrap(),
            DereferencedContent::new(json!("https://example.com")).unwrap(),
            DidUrlContentMetadata::empty()
        )
        .is_err()
    );

    assert!(
        DidUrlDereferencingResult::from_json_str(
            r#"{"dereferencingMetadata":{},"content":"https://example.com","contentMetadata":{}}"#
        )
        .is_err()
    );
}

#[test]
fn open_metadata_and_raw_envelopes_share_resource_bounds() {
    let mut deep = json!(true);
    for _ in 0..=MAX_EXTENSION_DEPTH {
        deep = json!([deep]);
    }
    assert!(matches!(
        DereferencedContent::new(deep),
        Err(Error::InvalidDocument(DocumentError::ExtensionTooDeep))
    ));

    let too_many = (0..65)
        .map(|index| (format!("field{index}"), json!(index)))
        .collect();
    assert!(matches!(
        DidResolutionMetadata::new(None, None, too_many),
        Err(Error::InvalidDocument(DocumentError::TooManyProperties))
    ));
    assert!(
        DidResolutionError::new(
            Uri::parse("urn:example:error").unwrap(),
            None,
            Some("x".repeat(MAX_PROBLEM_DETAIL_BYTES + 1)),
            None,
            BTreeMap::new()
        )
        .is_err()
    );

    let oversized = format!(
        "{{\"didResolutionMetadata\":{{}},\"didDocument\":null,\"didDocumentMetadata\":{{\"x\":\"{}\"}}}}",
        "x".repeat(MAX_DID_RESOLUTION_RESULT_BYTES)
    );
    assert!(matches!(
        DidResolutionResult::from_json_str(&oversized),
        Err(Error::InvalidResolution(ResolutionError::TooLarge))
    ));

    assert!(matches!(
        DidResolutionMetadata::new(
            None,
            None,
            BTreeMap::from([("contentType".to_owned(), json!("application/did"))])
        ),
        Err(Error::InvalidDocument(DocumentError::ReservedProperty))
    ));

    let values = (0..MAX_EXTENSION_NODES / 2)
        .map(|index| json!(index))
        .collect::<Vec<_>>();
    let tree = Value::Array(
        values
            .chunks(64)
            .map(|chunk| Value::Array(chunk.to_vec()))
            .collect(),
    );
    let operation = DidResolutionMetadata::new(
        Some(media_type()),
        None,
        BTreeMap::from([("operation".to_owned(), tree.clone())]),
    )
    .unwrap();
    let document_metadata = DidDocumentMetadata::builder()
        .extensions(BTreeMap::from([("method".to_owned(), tree)]))
        .build()
        .unwrap();
    assert!(matches!(
        DidResolutionResult::success(
            operation,
            minimal_document("did:example:123"),
            document_metadata
        ),
        Err(Error::InvalidDocument(DocumentError::ExtensionTooLarge))
    ));
}

#[test]
fn resolution_validation_errors_are_stable_and_redacted() {
    let marker = "caller-secret-marker";
    let malformed = format!("{{\"didResolutionMetadata\":{{}},\"didDocument\":\"{marker}");
    let error = DidResolutionResult::from_json_str(&malformed).unwrap_err();
    let public = error.to_identus_error();
    assert_eq!(public.code().as_str(), "did.invalid_resolution");
    assert_eq!(public.capability().unwrap().as_str(), "did");
    assert!(!public.to_string().contains(marker));
    assert!(!error.to_string().contains(marker));
}

#[test]
#[ignore = "manual release-mode performance diagnostic"]
fn resolution_parse_throughput_diagnostic() {
    const ITERATIONS: usize = 50_000;
    let started = Instant::now();
    for _ in 0..ITERATIONS {
        black_box(DidResolutionResult::from_json_str(black_box(SUCCESS_RESULT)).unwrap());
    }
    let elapsed = started.elapsed();
    let per_second = ITERATIONS as f64 / elapsed.as_secs_f64();
    eprintln!(
        "parsed {ITERATIONS} representative DID resolution results in {elapsed:?} ({per_second:.0} results/s)"
    );
}
