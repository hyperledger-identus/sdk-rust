use identus_core::{ErrorKind, IdentusError};
use identus_oid4vci::{
    CAPABILITY, CredentialOfferError, CredentialOfferLimits, CredentialOfferReference,
    CredentialOfferRequest, EmbeddedCredentialOffer, MAX_CONFIGURABLE_JSON_DEPTH, error_code,
};

fn encode_form(value: &str) -> String {
    let mut encoded = String::new();
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
            encoded.push(char::from(byte));
        } else if byte == b' ' {
            encoded.push('+');
        } else {
            encoded.push_str(&format!("%{byte:02X}"));
        }
    }
    encoded
}

fn embedded_invocation(json: &str) -> String {
    format!(
        "openid-credential-offer://?credential_offer={}",
        encode_form(json)
    )
}

fn reference_invocation(uri: &str) -> String {
    format!(
        "openid-credential-offer://?credential_offer_uri={}",
        encode_form(uri)
    )
}

#[test]
fn accepts_final_by_value_and_by_reference_examples() {
    let limits = CredentialOfferLimits::default();
    // OpenID4VCI 1.0 Final sections 4.1.2 and 4.1.3, with only the
    // presentation line breaks removed from each URI.
    let invocation = "openid-credential-offer://?credential_offer=%7B%22credential_issuer%22:%22https://credential-issuer.example.com%22,%22credential_configuration_ids%22:%5B%22org.iso.18013.5.1.mDL%22%5D,%22grants%22:%7B%22urn:ietf:params:oauth:grant-type:pre-authorized_code%22:%7B%22pre-authorized_code%22:%22oaKazRN8I0IbtZ0C7JuMn5%22,%22tx_code%22:%7B%22input_mode%22:%22text%22,%22description%22:%22Please%20enter%20the%20serial%20number%20of%20your%20physical%20drivers%20license%22%7D%7D%7D%7D";
    let json = r#"{"credential_issuer":"https://credential-issuer.example.com","credential_configuration_ids":["org.iso.18013.5.1.mDL"],"grants":{"urn:ietf:params:oauth:grant-type:pre-authorized_code":{"pre-authorized_code":"oaKazRN8I0IbtZ0C7JuMn5","tx_code":{"input_mode":"text","description":"Please enter the serial number of your physical drivers license"}}}}"#;
    let embedded = CredentialOfferRequest::parse(invocation, limits)
        .expect("final by-value example should parse");
    match embedded {
        CredentialOfferRequest::Embedded(offer) => assert_eq!(offer.as_json(), json),
        CredentialOfferRequest::Referenced(_) => panic!("expected embedded transport"),
    }

    let invocation = "openid-credential-offer://?credential_offer_uri=https%3A%2F%2Fserver%2Eexample%2Ecom%2Fcredential-offer%2FGkurKxf5T0Y-mnPFCHqWOMiZi4VS138cQO_V7PZHAdM";
    let uri =
        "https://server.example.com/credential-offer/GkurKxf5T0Y-mnPFCHqWOMiZi4VS138cQO_V7PZHAdM";
    let referenced = CredentialOfferRequest::parse(invocation, limits)
        .expect("final by-reference example should parse");
    match referenced {
        CredentialOfferRequest::Referenced(reference) => assert_eq!(reference.as_uri(), uri),
        CredentialOfferRequest::Embedded(_) => panic!("expected referenced transport"),
    }
}

#[test]
fn accepts_oxid_shape_and_rejects_lace_legacy_extra_parameter() {
    let oxid = embedded_invocation(
        r#"{"credential_issuer":"https://issuer.example","credential_configuration_ids":["ExampleCredential"]}"#,
    );
    assert!(CredentialOfferRequest::parse(&oxid, CredentialOfferLimits::default()).is_ok());

    let lace = format!("{oxid}&issuer_origin=https%3A%2F%2Fissuer.example");
    assert_eq!(
        CredentialOfferRequest::parse(&lace, CredentialOfferLimits::default())
            .expect_err("extra legacy query pair must fail"),
        CredentialOfferError::UnsupportedTransport
    );
}

#[test]
fn rejects_query_smuggling_and_non_exact_invocations() {
    let limits = CredentialOfferLimits::default();
    let rejected = [
        "openid-credential-offer://?credential_offer=",
        "openid-credential-offer://?credential_offer",
        "openid-credential-offer://?Credential_offer=%7B%7D",
        "openid-credential-offer://?credential%5Foffer=%7B%7D",
        "openid-credential-offer://?unknown=%7B%7D",
        "openid-credential-offer://issuer?credential_offer=%7B%7D",
        "openid-credential-offer:///path?credential_offer=%7B%7D",
        "openid-credential-offer://?credential_offer=%7B%7D#fragment",
        "openid-credential-offer://?credential_offer=%7B%7D&credential_offer=%7B%7D",
        "openid-credential-offer://?credential_offer=%7B%7D&credential_offer_uri=https%3A%2F%2Fexample.com",
    ];
    for invocation in rejected {
        assert!(
            CredentialOfferRequest::parse(invocation, limits).is_err(),
            "unexpectedly accepted {invocation}"
        );
    }

    assert!(
        CredentialOfferRequest::parse("OPENID-CREDENTIAL-OFFER://?credential_offer=%7B%7D", limits)
            .is_ok()
    );
}

#[test]
fn strict_form_decoding_handles_spaces_and_rejects_invalid_bytes() {
    let limits = CredentialOfferLimits::default();
    let parsed = CredentialOfferRequest::parse(
        "openid-credential-offer://?credential_offer=%7b%22note%22%3A%22two+words%22%7D",
        limits,
    )
    .expect("mixed-case escapes and plus should decode");
    match parsed {
        CredentialOfferRequest::Embedded(offer) => {
            assert_eq!(offer.as_json(), r#"{"note":"two words"}"#)
        }
        CredentialOfferRequest::Referenced(_) => panic!("expected embedded transport"),
    }

    for value in ["%", "%2", "%GG", "%FF", "%00", "é"] {
        let invocation = format!("openid-credential-offer://?credential_offer={value}");
        assert_eq!(
            CredentialOfferRequest::parse(&invocation, limits)
                .expect_err("invalid form value must fail"),
            CredentialOfferError::InvalidFormEncoding
        );
    }
}

#[test]
fn embedded_json_requires_one_bounded_unambiguous_object() {
    let limits = CredentialOfferLimits::default();
    for json in ["null", "[]", "1", r#"{"id":1} trailing"#, "{"] {
        assert_eq!(
            EmbeddedCredentialOffer::try_from_json(json, limits)
                .expect_err("invalid JSON root or syntax must fail"),
            CredentialOfferError::InvalidEmbeddedJson
        );
    }
    for json in [
        r#"{"same":1,"same":2}"#,
        r#"{"name":1,"\u006eame":2}"#,
        r#"{"outer":{"same":1,"same":2}}"#,
    ] {
        assert_eq!(
            EmbeddedCredentialOffer::try_from_json(json, limits)
                .expect_err("duplicate decoded member name must fail"),
            CredentialOfferError::DuplicateJsonProperty
        );
    }

    let extension = r#"{"credential_issuer":"https://issuer.example","credential_configuration_ids":["Example"],"extension":{"opaque":true}}"#;
    assert_eq!(
        EmbeddedCredentialOffer::try_from_json(extension, limits)
            .expect("unknown extension should remain opaque")
            .as_json(),
        extension
    );

    for number in [
        "1e400",
        "-1e400",
        "18446744073709551616",
        "-9223372036854775809",
        "0.123456789012345678901234567890",
    ] {
        let json = format!(r#"{{"extension":{number}}}"#);
        assert_eq!(
            EmbeddedCredentialOffer::try_from_json(&json, limits)
                .expect("valid number magnitude must remain opaque")
                .as_json(),
            json
        );
    }

    for number in ["01", "-", "1.", "1e", "1e+", "+1", ".1", "NaN", "Infinity"] {
        let json = format!(r#"{{"extension":{number}}}"#);
        assert_eq!(
            EmbeddedCredentialOffer::try_from_json(&json, limits)
                .expect_err("malformed JSON number must fail"),
            CredentialOfferError::InvalidEmbeddedJson
        );
    }
}

#[test]
fn structural_scanner_enforces_complete_json_grammar() {
    let limits = CredentialOfferLimits::default();
    for json in [
        r#"{}"#,
        r#" { "values" : [null, true, false, -0, 0, 1, -1, 1.0, 1E+2, {"nested":[]}] } "#,
        r#"{"escaped":"quote=\" slash=\\ solidus=\/ unicode=\u0061 pair=\uD834\uDD1E"}"#,
    ] {
        assert!(
            EmbeddedCredentialOffer::try_from_json(json, limits).is_ok(),
            "valid JSON object was rejected: {json}"
        );
    }

    for json in [
        r#"{"a":true false}"#,
        r#"{"a":nul}"#,
        r#"{"a" 1}"#,
        r#"{"a":1,}"#,
        r#"{"a":[1,]}"#,
        r#"{"a":"\q"}"#,
        r#"{"a":"\u12"}"#,
        r#"{"a":"\uD800"}"#,
        "{\"a\":\"raw\nnewline\"}",
        "{\"a\":1}\u{000b}",
    ] {
        assert_eq!(
            EmbeddedCredentialOffer::try_from_json(json, limits)
                .expect_err("malformed JSON structure must fail"),
            CredentialOfferError::InvalidEmbeddedJson,
            "unexpected error for {json:?}"
        );
    }
}

#[test]
fn exact_json_and_byte_limits_are_deterministic() {
    let exact = CredentialOfferLimits::new(100, 7, 32, 2, 2).expect("valid limits");
    assert!(EmbeddedCredentialOffer::try_from_json(r#"{"a":1}"#, exact).is_ok());
    assert_eq!(
        EmbeddedCredentialOffer::try_from_json(r#"{"a":10}"#, exact)
            .expect_err("one excess byte must fail"),
        CredentialOfferError::EmbeddedTooLarge
    );

    let depth = CredentialOfferLimits::new(100, 100, 32, 2, 20).expect("valid limits");
    assert!(EmbeddedCredentialOffer::try_from_json(r#"{"a":{}}"#, depth).is_ok());
    assert_eq!(
        EmbeddedCredentialOffer::try_from_json(r#"{"a":{"b":{}}}"#, depth)
            .expect_err("one excess container depth must fail"),
        CredentialOfferError::JsonTooDeep
    );

    let nodes = CredentialOfferLimits::new(100, 100, 32, 5, 2).expect("valid limits");
    assert!(EmbeddedCredentialOffer::try_from_json(r#"{"a":1}"#, nodes).is_ok());
    assert_eq!(
        EmbeddedCredentialOffer::try_from_json(r#"{"a":1,"b":2}"#, nodes)
            .expect_err("one excess JSON node must fail"),
        CredentialOfferError::JsonTooManyNodes
    );
}

#[test]
fn exact_invocation_and_decoded_transport_limits_are_deterministic() {
    let invocation = embedded_invocation("{}");
    let exact_invocation =
        CredentialOfferLimits::new(invocation.len(), 2, 32, 1, 1).expect("valid limits");
    assert!(CredentialOfferRequest::parse(&invocation, exact_invocation).is_ok());

    let embedded_exact = CredentialOfferLimits::new(512, 2, 32, 1, 1).expect("valid limits");
    assert!(CredentialOfferRequest::parse(&invocation, embedded_exact).is_ok());
    let embedded_short = CredentialOfferLimits::new(512, 1, 32, 1, 1).expect("valid limits");
    assert_eq!(
        CredentialOfferRequest::parse(&invocation, embedded_short)
            .expect_err("decoded embedded excess must fail"),
        CredentialOfferError::EmbeddedTooLarge
    );

    let uri = "https://example.com/offer";
    let reference = reference_invocation(uri);
    let reference_exact =
        CredentialOfferLimits::new(512, 2, uri.len(), 1, 1).expect("valid limits");
    assert!(CredentialOfferRequest::parse(&reference, reference_exact).is_ok());
    let reference_short =
        CredentialOfferLimits::new(512, 2, uri.len() - 1, 1, 1).expect("valid limits");
    assert_eq!(
        CredentialOfferRequest::parse(&reference, reference_short)
            .expect_err("decoded reference excess must fail"),
        CredentialOfferError::ReferenceTooLarge
    );
}

#[test]
fn constructor_and_transport_parser_apply_equivalent_json_validation() {
    let limits = CredentialOfferLimits::new(512, 128, 64, 3, 8).expect("valid limits");
    for json in [r#"{"ok":{"nested":true}}"#, r#"{"same":1,"same":2}"#, "[]"] {
        let direct = EmbeddedCredentialOffer::try_from_json(json, limits).map(|_| ());
        let transport =
            CredentialOfferRequest::parse(&embedded_invocation(json), limits).map(|_| ());
        assert_eq!(direct, transport);
    }
}

#[test]
fn reference_validation_is_https_only_and_least_authority() {
    let limits = CredentialOfferLimits::default();
    assert!(CredentialOfferReference::try_from_uri("HTTPS://example.com/offer", limits).is_ok());
    assert!(CredentialOfferReference::try_from_uri("https://[v1.fe80]/offer", limits).is_ok());
    for uri in [
        "/relative",
        "http://example.com/offer",
        "https:/hostless",
        "https:///hostless",
        "https://",
        "https://:443/offer",
        "https://user@example.com/offer",
        "https://user:password@example.com/offer",
        "https://example.com/offer#fragment",
        "https://example.com/bad%escape",
        "not a uri",
    ] {
        let result = CredentialOfferReference::try_from_uri(uri, limits);
        assert!(
            matches!(result, Err(CredentialOfferError::UnsafeReferenceUri)),
            "unexpectedly accepted unsafe reference: {uri}"
        );
    }
}

#[test]
fn limits_are_positive_inspectable_and_enforced() {
    let defaults = CredentialOfferLimits::default();
    assert_eq!(defaults.max_invocation_bytes(), 32_768);
    assert_eq!(defaults.max_embedded_json_bytes(), 16_384);
    assert_eq!(defaults.max_reference_uri_bytes(), 2_048);
    assert_eq!(defaults.max_json_depth(), 16);
    assert_eq!(defaults.max_json_nodes(), 128);

    for fields in [
        (0, 1, 1, 1, 1),
        (1, 0, 1, 1, 1),
        (1, 1, 0, 1, 1),
        (1, 1, 1, 0, 1),
        (1, 1, 1, 1, 0),
    ] {
        assert_eq!(
            CredentialOfferLimits::new(fields.0, fields.1, fields.2, fields.3, fields.4),
            Err(CredentialOfferError::InvalidLimits)
        );
    }

    let at_depth_ceiling = format!(
        "{}0{}",
        r#"{"value":"#.repeat(MAX_CONFIGURABLE_JSON_DEPTH),
        "}".repeat(MAX_CONFIGURABLE_JSON_DEPTH)
    );
    let supported_depth = CredentialOfferLimits::new(
        1,
        at_depth_ceiling.len(),
        1,
        MAX_CONFIGURABLE_JSON_DEPTH,
        MAX_CONFIGURABLE_JSON_DEPTH + 1,
    )
    .expect("hard JSON depth ceiling must be configurable");
    assert!(EmbeddedCredentialOffer::try_from_json(&at_depth_ceiling, supported_depth).is_ok());
    assert_eq!(
        CredentialOfferLimits::new(
            1,
            at_depth_ceiling.len(),
            1,
            MAX_CONFIGURABLE_JSON_DEPTH + 1,
            MAX_CONFIGURABLE_JSON_DEPTH + 2,
        ),
        Err(CredentialOfferError::InvalidLimits)
    );

    let invocation = embedded_invocation("{}");
    let short = CredentialOfferLimits::new(invocation.len() - 1, 2, 1, 1, 1)
        .expect("valid short invocation limit");
    assert_eq!(
        CredentialOfferRequest::parse(&invocation, short)
            .expect_err("invocation ceiling must be enforced first"),
        CredentialOfferError::InvocationTooLarge
    );
}

#[test]
fn sensitive_values_never_enter_diagnostics_or_core_errors() {
    let offer_canary = "offer-canary-pre-authorized-code";
    let uri_canary = "reference-uri-canary";
    let embedded = EmbeddedCredentialOffer::try_from_json(
        &format!(r#"{{"secret":"{offer_canary}"}}"#),
        CredentialOfferLimits::default(),
    )
    .expect("canary object should validate");
    let reference = CredentialOfferReference::try_from_uri(
        &format!("https://example.com/{uri_canary}"),
        CredentialOfferLimits::default(),
    )
    .expect("canary reference should validate");
    assert!(!format!("{embedded:?}").contains(offer_canary));
    assert!(!format!("{reference:?}").contains(uri_canary));

    let cases = [
        (
            CredentialOfferError::InvalidLimits,
            error_code::INVALID_LIMITS,
            ErrorKind::InvalidInput,
        ),
        (
            CredentialOfferError::InvocationTooLarge,
            error_code::INVOCATION_TOO_LARGE,
            ErrorKind::InvalidInput,
        ),
        (
            CredentialOfferError::InvalidInvocation,
            error_code::INVALID_INVOCATION,
            ErrorKind::InvalidInput,
        ),
        (
            CredentialOfferError::UnsupportedTransport,
            error_code::UNSUPPORTED_TRANSPORT,
            ErrorKind::Unsupported,
        ),
        (
            CredentialOfferError::InvalidFormEncoding,
            error_code::INVALID_FORM_ENCODING,
            ErrorKind::InvalidInput,
        ),
        (
            CredentialOfferError::EmbeddedTooLarge,
            error_code::EMBEDDED_TOO_LARGE,
            ErrorKind::InvalidInput,
        ),
        (
            CredentialOfferError::InvalidEmbeddedJson,
            error_code::INVALID_EMBEDDED_JSON,
            ErrorKind::InvalidInput,
        ),
        (
            CredentialOfferError::DuplicateJsonProperty,
            error_code::DUPLICATE_JSON_PROPERTY,
            ErrorKind::InvalidInput,
        ),
        (
            CredentialOfferError::JsonTooDeep,
            error_code::JSON_TOO_DEEP,
            ErrorKind::InvalidInput,
        ),
        (
            CredentialOfferError::JsonTooManyNodes,
            error_code::JSON_TOO_MANY_NODES,
            ErrorKind::InvalidInput,
        ),
        (
            CredentialOfferError::ReferenceTooLarge,
            error_code::REFERENCE_TOO_LARGE,
            ErrorKind::InvalidInput,
        ),
        (
            CredentialOfferError::UnsafeReferenceUri,
            error_code::UNSAFE_REFERENCE_URI,
            ErrorKind::InvalidInput,
        ),
    ];
    for (error, expected_code, expected_kind) in cases {
        let core: IdentusError = error.into();
        for rendered in [
            format!("{error}"),
            format!("{error:?}"),
            format!("{core}"),
            format!("{core:?}"),
        ] {
            assert!(!rendered.contains(offer_canary));
            assert!(!rendered.contains(uri_canary));
        }
        assert_eq!(core.code(), expected_code);
        assert_eq!(core.kind(), expected_kind);
        assert_eq!(core.capability(), Some(CAPABILITY));
    }
}

#[test]
#[ignore = "release-mode throughput diagnostic"]
fn credential_offer_parse_throughput_diagnostic() {
    use std::time::Instant;

    let embedded = embedded_invocation(
        r#"{"credential_issuer":"https://issuer.example","credential_configuration_ids":["Example"]}"#,
    );
    let referenced = reference_invocation("https://issuer.example/offer?id=example");
    let limits = CredentialOfferLimits::default();
    let iterations = 20_000_u32;
    let started = Instant::now();
    let mut consumed = 0_usize;
    for _ in 0..iterations {
        match CredentialOfferRequest::parse(&embedded, limits).expect("embedded parse") {
            CredentialOfferRequest::Embedded(value) => consumed += value.as_json().len(),
            CredentialOfferRequest::Referenced(_) => panic!("wrong variant"),
        }
        match CredentialOfferRequest::parse(&referenced, limits).expect("reference parse") {
            CredentialOfferRequest::Referenced(value) => consumed += value.as_uri().len(),
            CredentialOfferRequest::Embedded(_) => panic!("wrong variant"),
        }
    }
    let elapsed = started.elapsed();
    let operations = f64::from(iterations) * 2.0;
    println!(
        "Credential Offer transport: {operations} operations in {elapsed:?} ({:.0} ops/s, consumed={consumed})",
        operations / elapsed.as_secs_f64()
    );
}
