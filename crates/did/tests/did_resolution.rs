use std::{collections::BTreeMap, hint::black_box, time::Instant};

use identus_did::{
    DereferencedContent, Did, DidDocument, DidDocumentMetadata, DidResolutionDateTime,
    DidResolutionError, DidResolutionErrorKind, DidResolutionMetadata, DidResolutionResult,
    DidUrlContentMetadata, DidUrlDereferencingMetadata, DidUrlDereferencingResult, DocumentError,
    Error, MAX_DID_RESOLUTION_DATETIME_BYTES, MAX_DID_RESOLUTION_RESULT_BYTES,
    MAX_DID_RESOLUTION_WIRE_DEPTH, MAX_DID_RESOLUTION_WIRE_LIVE_KEY_BYTES,
    MAX_DID_RESOLUTION_WIRE_NODES, MAX_DID_RESOLUTION_WIRE_OBJECT_MEMBERS, MAX_EXTENSION_DEPTH,
    MAX_EXTENSION_NODES, MAX_MEDIA_TYPE_BYTES, MAX_PROBLEM_DETAIL_BYTES, MAX_VERSION_ID_BYTES,
    MediaType, ResolutionError, Service, Uri, VerificationMethod, VersionId,
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

const _: () = {
    assert!(MAX_DID_RESOLUTION_WIRE_DEPTH > MAX_EXTENSION_DEPTH);
    assert!(MAX_DID_RESOLUTION_WIRE_NODES > MAX_EXTENSION_NODES);
    assert!(MAX_DID_RESOLUTION_WIRE_OBJECT_MEMBERS > 64);
    assert!(MAX_DID_RESOLUTION_WIRE_LIVE_KEY_BYTES < MAX_DID_RESOLUTION_RESULT_BYTES);
};

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

    for seed in 0..256 {
        let error = DidResolutionError::new(
            Uri::parse(&format!("https://resolver.example/problems/{seed}")).unwrap(),
            Some(format!("Resolution problem {seed}")),
            Some(format!("Portable public detail for generated case {seed}")),
            Some(Uri::parse(&format!("urn:uuid:00000000-0000-0000-0000-{seed:012}")).unwrap()),
            BTreeMap::from([(
                format!("extension{seed}"),
                json!({"retryable": seed % 2 == 0, "attempt": seed}),
            )]),
        )
        .unwrap();
        assert_eq!(error.kind(), None);
        assert_eq!(
            serde_json::from_value::<DidResolutionError>(serde_json::to_value(&error).unwrap())
                .unwrap(),
            error
        );
    }
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
        "0000-02-29T00:00:00Z",
        "-0004-02-29T24:00:00Z",
        "-0001-12-31T23:59:59Z",
        "2024-02-29T23:59:59Z",
        "2026-09-03T00:00:00Z",
        "2000-02-29T12:34:56Z",
        "12345-01-01T00:00:00Z",
    ] {
        let parsed = DidResolutionDateTime::parse(value).unwrap();
        assert_eq!(parsed.as_str(), value);
        assert_eq!(
            DidResolutionDateTime::try_new(value.to_owned()).unwrap(),
            parsed
        );
        assert_eq!(value.parse::<DidResolutionDateTime>().unwrap(), parsed);
        assert_eq!(
            DidResolutionDateTime::try_from(value.to_owned()).unwrap(),
            parsed
        );
        assert_eq!(
            serde_json::from_str::<DidResolutionDateTime>(&serde_json::to_string(&parsed).unwrap())
                .unwrap(),
            parsed
        );
    }
    for invalid in [
        "2023-02-29T00:00:00Z",
        "2026-13-01T00:00:00Z",
        "2026-01-00T00:00:00Z",
        "2026-01-01T00:60:00Z",
        "2026-01-01T00:00:60Z",
        "2026-01-01T24:00:01Z",
        "2026-01-01T00:00:00.1Z",
        "2026-01-01T00:00:00+00:00",
        "2026-01-01T00:00:00",
        "+2026-01-01T00:00:00Z",
        "02026-01-01T00:00:00Z",
        "-02026-01-01T00:00:00Z",
        "999-01-01T00:00:00Z",
    ] {
        assert!(matches!(
            DidResolutionDateTime::parse(invalid),
            Err(Error::InvalidResolution(ResolutionError::InvalidDateTime))
        ));
    }

    let largest_year = format!("{}-01-01T00:00:00Z", "1".repeat(112));
    assert_eq!(largest_year.len(), MAX_DID_RESOLUTION_DATETIME_BYTES);
    assert!(DidResolutionDateTime::parse(&largest_year).is_ok());
    let excessive_year = format!("{}-01-01T00:00:00Z", "1".repeat(113));
    assert!(DidResolutionDateTime::parse(&excessive_year).is_err());

    for value in ["1", "block-42:7", "opaque version"] {
        assert_eq!(VersionId::parse(value).unwrap().as_str(), value);
    }
    for invalid in ["", " leading", "trailing ", "line\nbreak", "ümlaut"] {
        assert!(VersionId::parse(invalid).is_err());
    }
    assert!(VersionId::parse(&"v".repeat(MAX_VERSION_ID_BYTES + 1)).is_err());
}

#[test]
fn deterministic_scalar_property_matrices_preserve_constructor_and_serde_equivalence() {
    for seed in 0..512 {
        let media = if seed % 2 == 0 {
            format!("application/vnd.identus-{seed}+json;v={}", seed % 17)
        } else {
            format!("x-{seed}/profile; note=\"seed {seed}\"")
        };
        let parsed = MediaType::parse(&media).unwrap();
        assert_eq!(MediaType::try_new(media.clone()).unwrap(), parsed);
        assert_eq!(media.parse::<MediaType>().unwrap(), parsed);
        assert_eq!(MediaType::try_from(media).unwrap(), parsed);
        assert_eq!(
            serde_json::from_value::<MediaType>(serde_json::to_value(&parsed).unwrap()).unwrap(),
            parsed
        );

        let year = seed % 400;
        let month = seed % 12 + 1;
        let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
        let max_day = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 if leap => 29,
            2 => 28,
            _ => unreachable!(),
        };
        let day = seed % max_day + 1;
        let hour = seed % 24;
        let minute = seed % 60;
        let second = seed * 7 % 60;
        let datetime = format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z");
        let parsed = DidResolutionDateTime::parse(&datetime).unwrap();
        assert_eq!(
            DidResolutionDateTime::try_new(datetime.clone()).unwrap(),
            parsed
        );
        assert_eq!(datetime.parse::<DidResolutionDateTime>().unwrap(), parsed);
        assert_eq!(DidResolutionDateTime::try_from(datetime).unwrap(), parsed);
        assert_eq!(
            serde_json::from_value::<DidResolutionDateTime>(serde_json::to_value(&parsed).unwrap())
                .unwrap(),
            parsed
        );

        let version = format!("block-{seed}:{} opaque", seed * 3);
        let parsed = VersionId::parse(&version).unwrap();
        assert_eq!(VersionId::try_new(version.clone()).unwrap(), parsed);
        assert_eq!(version.parse::<VersionId>().unwrap(), parsed);
        assert_eq!(VersionId::try_from(version).unwrap(), parsed);
        assert_eq!(
            serde_json::from_value::<VersionId>(serde_json::to_value(&parsed).unwrap()).unwrap(),
            parsed
        );
    }

    for year in ["0000", "-0004", "0400", "2000", "12345678901234568000"] {
        assert!(DidResolutionDateTime::parse(&format!("{year}-02-29T24:00:00Z")).is_ok());
    }
    for year in ["-0001", "0100", "1900", "12345678901234567899"] {
        assert!(DidResolutionDateTime::parse(&format!("{year}-02-29T00:00:00Z")).is_err());
    }
}

fn assert_wire_reason(input: &str, reason: ResolutionError) {
    assert_eq!(
        DidResolutionResult::from_json_str(input),
        Err(Error::InvalidResolution(reason))
    );
    assert_eq!(
        DidUrlDereferencingResult::from_json_str(input),
        Err(Error::InvalidResolution(reason))
    );
}

#[test]
fn raw_results_reject_duplicates_at_every_nested_object_boundary() {
    for input in [
        r#"{"didResolutionMetadata":{},"didResolutionMetadata":{},"didDocument":null,"didDocumentMetadata":{}}"#,
        r#"{"didResolutionMetadata":{"source":1,"source":2},"didDocument":null,"didDocumentMetadata":{}}"#,
        r#"{"didResolutionMetadata":{"error":{"type":"https://www.w3.org/ns/did#NOT_FOUND","title":"one","title":"two"}},"didDocument":null,"didDocumentMetadata":{}}"#,
        r#"{"didResolutionMetadata":{},"didDocument":{"id":"did:example:one","id":"did:example:two"},"didDocumentMetadata":{}}"#,
        r#"{"didResolutionMetadata":{},"didDocument":null,"didDocumentMetadata":{"ledger":{"height":1,"height":2}}}"#,
        r#"{"didUrlDereferencingMetadata":{},"content":{"claim":1,"claim":2},"contentMetadata":{}}"#,
        r#"{"didUrlDereferencingMetadata":{},"content":{"claim":1,"\u0063laim":2},"contentMetadata":{}}"#,
        r#"{"didUrlDereferencingMetadata":{},"content":"value","contentMetadata":{"proof":{"type":1,"type":2}}}"#,
    ] {
        assert_wire_reason(input, ResolutionError::DuplicateJsonProperty);
    }

    let valid = r#"{
        "didResolutionMetadata":{"left":{"name":1},"right":{"name":2}},
        "didDocument":{"id":"did:example:one"},
        "didDocumentMetadata":{}
    }"#;
    assert!(DidResolutionResult::from_json_str(valid).is_ok());
}

#[test]
fn raw_result_scanner_limits_and_complete_input_fail_before_typed_construction() {
    let mut too_deep = "0".to_owned();
    for _ in 0..=MAX_DID_RESOLUTION_WIRE_DEPTH {
        too_deep = format!("[{too_deep}]");
    }
    assert_wire_reason(&too_deep, ResolutionError::WireTooDeep);

    let too_many_nodes = format!(
        "[{}]",
        std::iter::repeat_n("0", MAX_DID_RESOLUTION_WIRE_NODES)
            .collect::<Vec<_>>()
            .join(",")
    );
    assert!(too_many_nodes.len() < MAX_DID_RESOLUTION_RESULT_BYTES);
    assert_wire_reason(&too_many_nodes, ResolutionError::WireTooLarge);

    let too_many_members = format!(
        "{{{}}}",
        (0..=MAX_DID_RESOLUTION_WIRE_OBJECT_MEMBERS)
            .map(|index| format!("\"p{index}\":null"))
            .collect::<Vec<_>>()
            .join(",")
    );
    assert_wire_reason(&too_many_members, ResolutionError::WireTooManyProperties);

    let key_body = "k".repeat((MAX_DID_RESOLUTION_WIRE_LIVE_KEY_BYTES / 128) + 1);
    let too_many_live_key_bytes = format!(
        "{{{}}}",
        (0..MAX_DID_RESOLUTION_WIRE_OBJECT_MEMBERS)
            .map(|index| format!("\"{index:03}{key_body}\":null"))
            .collect::<Vec<_>>()
            .join(",")
    );
    assert!(too_many_live_key_bytes.len() < MAX_DID_RESOLUTION_RESULT_BYTES);
    assert_wire_reason(&too_many_live_key_bytes, ResolutionError::WireTooLarge);

    for malformed in [
        r#"{"didResolutionMetadata":{}} trailing"#,
        r#"{"didResolutionMetadata":"unterminated}"#,
    ] {
        assert_wire_reason(malformed, ResolutionError::MalformedJson);
    }
}

#[test]
fn duplicate_result_diagnostics_do_not_reflect_caller_data() {
    let marker = "caller-secret-marker";
    let input = format!(
        r#"{{"didResolutionMetadata":{{"{marker}":1,"{marker}":2}},"didDocument":null,"didDocumentMetadata":{{}}}}"#
    );
    let error = DidResolutionResult::from_json_str(&input).unwrap_err();

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
    assert_eq!(
        error.to_identus_error().code().as_str(),
        "did.invalid_resolution"
    );
}

#[test]
fn generated_resolution_and_dereferencing_state_matrices_match_all_construction_paths() {
    for has_document in [false, true] {
        for has_error in [false, true] {
            for deactivated in [None, Some(false), Some(true)] {
                let metadata = if has_error {
                    json!({"error": {"type": DidResolutionErrorKind::NotFound.as_str()}})
                } else {
                    json!({})
                };
                let document = has_document.then(|| json!({"id": "did:example:state"}));
                let document_metadata = deactivated
                    .map(|value| json!({"deactivated": value}))
                    .unwrap_or_else(|| json!({}));
                let wire = json!({
                    "didResolutionMetadata": metadata,
                    "didDocument": document,
                    "didDocumentMetadata": document_metadata,
                });
                let expected = has_document && !has_error && deactivated != Some(true)
                    || !has_document && has_error && deactivated.is_none()
                    || !has_document && !has_error && deactivated == Some(true);
                let semantic = serde_json::from_value::<DidResolutionResult>(wire.clone());
                let raw =
                    DidResolutionResult::from_json_str(&serde_json::to_string(&wire).unwrap());
                assert_eq!(semantic.is_ok(), expected, "{wire}");
                assert_eq!(raw.is_ok(), expected, "{wire}");
                if let (Ok(semantic), Ok(raw)) = (semantic, raw) {
                    assert_eq!(semantic, raw);
                }
            }
        }
    }

    for has_content in [false, true] {
        for has_error in [false, true] {
            for has_content_metadata in [false, true] {
                let metadata = if has_error {
                    json!({"error": {"type": DidResolutionErrorKind::NotFound.as_str()}})
                } else {
                    json!({})
                };
                let content = has_content.then(|| json!({"resource": true}));
                let content_metadata = if has_content_metadata {
                    json!({"versionId": "1"})
                } else {
                    json!({})
                };
                let wire = json!({
                    "didUrlDereferencingMetadata": metadata,
                    "content": content,
                    "contentMetadata": content_metadata,
                });
                let expected =
                    has_content && !has_error || !has_content && has_error && !has_content_metadata;
                let semantic = serde_json::from_value::<DidUrlDereferencingResult>(wire.clone());
                let raw = DidUrlDereferencingResult::from_json_str(
                    &serde_json::to_string(&wire).unwrap(),
                );
                assert_eq!(semantic.is_ok(), expected, "{wire}");
                assert_eq!(raw.is_ok(), expected, "{wire}");
                if let (Ok(semantic), Ok(raw)) = (semantic, raw) {
                    assert_eq!(semantic, raw);
                }
            }
        }
    }
}

#[test]
fn official_w3c_suite_portable_result_assertions_are_pinned() {
    let requested = did("did:example:123");
    let success = DidResolutionResult::from_json_str_for(
        r#"{"didResolutionMetadata":{},"didDocument":{"id":"did:example:123"},"didDocumentMetadata":{}}"#,
        &requested,
    )
    .unwrap();
    assert_eq!(success.document().unwrap().id(), &requested);

    for kind in [
        DidResolutionErrorKind::InvalidDid,
        DidResolutionErrorKind::MethodNotSupported,
    ] {
        let result = DidResolutionResult::from_json_str(&format!(
            r#"{{"didResolutionMetadata":{{"error":{{"type":"{}"}}}},"didDocument":null,"didDocumentMetadata":{{}}}}"#,
            kind.as_str()
        ))
        .unwrap();
        assert_eq!(result.metadata().error().unwrap().kind(), Some(kind));
        assert!(result.document().is_none());
        assert!(result.document_metadata().is_empty());
    }

    let deactivated = DidResolutionResult::from_json_str(
        r#"{"didResolutionMetadata":{},"didDocument":null,"didDocumentMetadata":{"deactivated":true}}"#,
    )
    .unwrap();
    assert_eq!(deactivated.document_metadata().deactivated(), Some(true));
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
    let typed_started = Instant::now();
    for _ in 0..ITERATIONS {
        black_box(serde_json::from_str::<DidResolutionResult>(black_box(SUCCESS_RESULT)).unwrap());
    }
    let typed_elapsed = typed_started.elapsed();

    let hardened_started = Instant::now();
    for _ in 0..ITERATIONS {
        black_box(DidResolutionResult::from_json_str(black_box(SUCCESS_RESULT)).unwrap());
    }
    let hardened_elapsed = hardened_started.elapsed();
    let processed_bytes = ITERATIONS * SUCCESS_RESULT.len();
    let overhead = hardened_elapsed.as_secs_f64() / typed_elapsed.as_secs_f64();
    eprintln!(
        "resolution parse diagnostic: {ITERATIONS} results / {processed_bytes} input bytes; typed {typed_elapsed:?}; hardened {hardened_elapsed:?}; normalized scanner ratio {overhead:.3}x"
    );
}
