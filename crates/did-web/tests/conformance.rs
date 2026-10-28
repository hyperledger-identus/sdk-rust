use identus_did_web::{DidWeb, DidWebLimits, Error};

#[test]
fn pinned_url_mapping_examples_match_the_selected_profile() {
    for (did, expected) in [
        (
            "did:web:w3c-ccg.github.io",
            "https://w3c-ccg.github.io/.well-known/did.json",
        ),
        (
            "did:web:w3c-ccg.github.io:user:alice",
            "https://w3c-ccg.github.io/user/alice/did.json",
        ),
        (
            "did:web:example.com%3A3000:user:alice",
            "https://example.com:3000/user/alice/did.json",
        ),
    ] {
        let parsed = DidWeb::parse(did).unwrap();
        assert_eq!(parsed.url(), expected);
        assert_eq!(DidWeb::parse(parsed.did().as_str()).unwrap(), parsed);
    }
}

#[test]
fn parse_format_and_url_property_covers_sixty_four_domain_classes() {
    for index in 0..64 {
        let did = format!("did:web:n{index}.example:users:u{index}");
        let expected = format!("https://n{index}.example/users/u{index}/did.json");
        let parsed = DidWeb::parse(&did).unwrap();
        assert_eq!(parsed.did().as_str(), did);
        assert_eq!(parsed.url(), expected);
        assert_eq!(DidWeb::parse(parsed.did().as_str()).unwrap(), parsed);
    }
}

#[test]
fn strict_profile_rejects_unsafe_or_ambiguous_forms() {
    for input in [
        "did:web:127.0.0.1",
        "did:web:0x7f.0.0.1",
        "did:web:2130706433.1",
        "did:web:0177.0.0.1",
        "did:web:[::1]",
        "did:web:user@example.com",
        "did:web:example.com?query",
        "did:web:example.com#fragment",
        "did:web:example.com%3a8443",
        "did:web:example.com%253A8443",
        "did:web:example.com%3A443",
        "did:web:example.com:.",
        "did:web:example.com:a::b",
    ] {
        assert!(DidWeb::parse(input).is_err(), "accepted {input}");
    }
}

#[test]
fn public_limits_cover_identifier_url_header_and_body() {
    assert_eq!(
        DidWebLimits::new(0, 1, 1, 1, 1, 1),
        Err(Error::InvalidLimits)
    );
    let limits = DidWebLimits::new(32, 2, 8, 64, 24, 64).unwrap();
    assert_eq!(
        DidWeb::parse_with_limits("did:web:example.com:segmentlong", limits),
        Err(Error::ResourceLimit)
    );
    let value = DidWeb::parse_with_limits("did:web:a.co", limits).unwrap();
    assert_eq!(
        value
            .request()
            .response(200, Some("application/did+json"), vec![0; 65]),
        Err(Error::ResourceLimit)
    );
}

#[test]
fn response_binding_checks_envelope_before_document_and_exact_id() {
    let value = DidWeb::parse("did:web:example.com").unwrap();
    let request = value.request();
    let response = request
        .response(404, Some("application/did+json"), b"not-json".to_vec())
        .unwrap();
    assert_eq!(value.bind_response(response), Err(Error::HttpStatus));
    let response = request
        .response(200, Some("text/html"), b"not-json".to_vec())
        .unwrap();
    assert_eq!(value.bind_response(response), Err(Error::InvalidMediaType));
    let response = request
        .response(
            200,
            Some("application/did+json; charset=utf-8"),
            br#"{"id":"did:web:other.example"}"#.to_vec(),
        )
        .unwrap();
    assert_eq!(
        value.bind_response(response),
        Err(Error::DocumentIdMismatch)
    );

    for malformed in [
        "application/did+json; charset=bad value",
        "application/did+json; charset=\"unterminated",
        "application/did+json; char set=utf-8",
        "application/did+json; charset=",
        "application/did+json;",
        "application/did+json; charset=utf-8;",
    ] {
        let response = request
            .response(
                200,
                Some(malformed),
                br#"{"id":"did:web:example.com"}"#.to_vec(),
            )
            .unwrap();
        assert_eq!(value.bind_response(response), Err(Error::InvalidMediaType));
    }

    let response = request
        .response(
            200,
            Some("application/did+json; charset=\"utf-8\""),
            br#"{"id":"did:web:example.com"}"#.to_vec(),
        )
        .unwrap();
    assert_eq!(value.bind_response(response).unwrap().id(), value.did());

    let response = request
        .response(
            200,
            Some("application/did+json;\tcharset=utf-8"),
            br#"{"id":"did:web:example.com"}"#.to_vec(),
        )
        .unwrap();
    assert_eq!(value.bind_response(response).unwrap().id(), value.did());

    assert_eq!(request.accept(), "application/did+json, application/json");
    assert!(!request.no_cache());
}
