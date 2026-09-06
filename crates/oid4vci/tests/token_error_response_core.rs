use identus_core::{ErrorKind, IdentusError};
use identus_oid4vci::{
    CAPABILITY, CredentialOfferError, MAX_CONFIGURABLE_JSON_DEPTH, TokenEndpointErrorKind,
    TokenErrorResponseCore, TokenErrorResponseLimits, error_code,
};

fn parse(json: &str) -> Result<TokenErrorResponseCore, CredentialOfferError> {
    TokenErrorResponseCore::parse(json, TokenErrorResponseLimits::default())
}

fn limits(code: usize, description: usize, uri: usize) -> TokenErrorResponseLimits {
    TokenErrorResponseLimits::new(1_024, 16, 128, code, description, uri).expect("positive limits")
}

#[test]
fn defaults_and_positive_limits_are_explicit() {
    let defaults = TokenErrorResponseLimits::default();
    assert_eq!(defaults.max_json_bytes(), 32_768);
    assert_eq!(defaults.max_json_depth(), 16);
    assert_eq!(defaults.max_json_nodes(), 512);
    assert_eq!(defaults.max_error_code_bytes(), 256);
    assert_eq!(defaults.max_error_description_bytes(), 4_096);
    assert_eq!(defaults.max_error_uri_bytes(), 2_048);

    let invalid = [
        TokenErrorResponseLimits::new(0, 1, 1, 1, 1, 1),
        TokenErrorResponseLimits::new(1, 0, 1, 1, 1, 1),
        TokenErrorResponseLimits::new(1, MAX_CONFIGURABLE_JSON_DEPTH + 1, 1, 1, 1, 1),
        TokenErrorResponseLimits::new(1, 1, 0, 1, 1, 1),
        TokenErrorResponseLimits::new(1, 1, 1, 0, 1, 1),
        TokenErrorResponseLimits::new(1, 1, 1, 1, 0, 1),
        TokenErrorResponseLimits::new(1, 1, 1, 1, 1, 0),
    ];
    for result in invalid {
        assert_eq!(
            result,
            Err(CredentialOfferError::InvalidTokenErrorResponseLimits)
        );
    }
}

#[test]
fn minimal_final_error_exposes_only_deliberate_core_values() {
    let json = r#"{"error":"invalid_request"}"#;
    let response = parse(json).expect("minimal Token Error Response");

    assert_eq!(response.response_len(), json.len());
    assert_eq!(response.error().as_str(), "invalid_request");
    assert_eq!(
        response.error().kind(),
        TokenEndpointErrorKind::InvalidRequest
    );
    assert_eq!(
        response.error_kind(),
        TokenEndpointErrorKind::InvalidRequest
    );
    assert!(!response.error_description_present());
    assert_eq!(response.expose_untrusted_description(), None);
    assert!(!response.error_uri_present());
    assert!(response.error_uri().is_none());
}

#[test]
fn exact_standard_codes_are_classified_and_extensions_remain_open() {
    let cases = [
        ("invalid_request", TokenEndpointErrorKind::InvalidRequest),
        ("invalid_client", TokenEndpointErrorKind::InvalidClient),
        ("invalid_grant", TokenEndpointErrorKind::InvalidGrant),
        (
            "unauthorized_client",
            TokenEndpointErrorKind::UnauthorizedClient,
        ),
        (
            "unsupported_grant_type",
            TokenEndpointErrorKind::UnsupportedGrantType,
        ),
        ("invalid_scope", TokenEndpointErrorKind::InvalidScope),
        ("invalid_Request", TokenEndpointErrorKind::Extension),
        ("vendor future code!", TokenEndpointErrorKind::Extension),
    ];

    for (code, expected) in cases {
        let json = format!(r#"{{"error":"{code}"}}"#);
        let response = parse(&json).expect("valid error code");
        assert_eq!(response.error().as_str(), code);
        assert_eq!(response.error_kind(), expected);
    }
}

#[test]
fn optional_metadata_and_bounded_extensions_are_preserved_or_discarded() {
    let json = r#"{
        "error":"invalid_grant",
        "error_description":"The credential offer is no longer valid.",
        "error_uri":"../help/token-errors#invalid_grant",
        "future":{"nested":[true,null,{"number":1e999999}]}
    }"#;
    let response = parse(json).expect("complete Token Error Response");

    assert!(response.error_description_present());
    assert_eq!(
        response.expose_untrusted_description(),
        Some("The credential offer is no longer valid.")
    );
    assert!(response.error_uri_present());
    assert_eq!(
        response.error_uri().map(|uri| uri.as_str()),
        Some("../help/token-errors#invalid_grant")
    );
}

#[test]
fn required_code_type_and_nqschar_grammar_fail_closed() {
    let invalid = [
        r#"{}"#,
        r#"{"error":""}"#,
        r#"{"error":null}"#,
        r#"{"error":7}"#,
        r#"{"error":"bad\"code"}"#,
        r#"{"error":"bad\\code"}"#,
        r#"{"error":"bad\u0009code"}"#,
        r#"{"error":"ümlaut"}"#,
    ];
    for json in invalid {
        assert!(parse(json).is_err(), "unexpectedly accepted {json}");
    }
}

#[test]
fn optional_metadata_grammar_and_uri_syntax_fail_closed() {
    let descriptions = ["", "quote\"text", "slash\\text", "line\ntext", "ümlaut"];
    for description in descriptions {
        let encoded = serde_json::to_string(description).expect("description JSON");
        let json = format!(r#"{{"error":"invalid_grant","error_description":{encoded}}}"#);
        assert_eq!(
            parse(&json).expect_err("invalid description"),
            CredentialOfferError::InvalidTokenErrorDescription
        );
    }

    let uris = [
        "",
        "relative path",
        "bad\\path",
        "https://[",
        "bad%escape",
        "ümlaut",
    ];
    for uri in uris {
        let encoded = serde_json::to_string(uri).expect("URI JSON");
        let json = format!(r#"{{"error":"invalid_grant","error_uri":{encoded}}}"#);
        assert_eq!(
            parse(&json).expect_err("invalid URI-reference"),
            CredentialOfferError::InvalidTokenErrorUri
        );
    }
}

#[test]
fn aggregate_structure_and_independent_value_limits_are_enforced() {
    let minimal = r#"{"error":"invalid_grant"}"#;
    assert_eq!(
        TokenErrorResponseCore::parse(
            minimal,
            TokenErrorResponseLimits::new(minimal.len(), 16, 128, 32, 32, 32)
                .expect("exact byte limit"),
        )
        .expect("exact aggregate bound")
        .response_len(),
        minimal.len()
    );
    assert_eq!(
        TokenErrorResponseCore::parse(
            minimal,
            TokenErrorResponseLimits::new(minimal.len() - 1, 16, 128, 32, 32, 32)
                .expect("small byte limit"),
        )
        .expect_err("aggregate oversize"),
        CredentialOfferError::TokenErrorResponseTooLarge
    );

    let cases = [
        (
            r#"{"error":"invalid_grant"}"#,
            limits(12, 64, 64),
            CredentialOfferError::TokenEndpointErrorCodeTooLarge,
        ),
        (
            r#"{"error":"invalid_grant","error_description":"detail"}"#,
            limits(64, 5, 64),
            CredentialOfferError::TokenErrorDescriptionTooLarge,
        ),
        (
            r#"{"error":"invalid_grant","error_uri":"help/errors"}"#,
            limits(64, 64, 10),
            CredentialOfferError::TokenErrorUriTooLarge,
        ),
    ];
    for (json, limits, expected) in cases {
        assert_eq!(
            TokenErrorResponseCore::parse(json, limits).expect_err("known value oversize"),
            expected
        );
    }

    let nested = r#"{"error":"invalid_grant","future":{"value":1}}"#;
    assert_eq!(
        TokenErrorResponseCore::parse(
            nested,
            TokenErrorResponseLimits::new(1_024, 1, 128, 64, 64, 64).expect("shallow limit"),
        )
        .expect_err("deep response"),
        CredentialOfferError::JsonTooDeep
    );
    assert_eq!(
        TokenErrorResponseCore::parse(
            minimal,
            TokenErrorResponseLimits::new(1_024, 16, 1, 64, 64, 64).expect("narrow node limit"),
        )
        .expect_err("wide response"),
        CredentialOfferError::JsonTooManyNodes
    );
}

#[test]
fn malformed_duplicate_nested_and_trailing_json_fail_closed() {
    let invalid = [
        r#"[]"#,
        r#"{"error":"invalid_grant","err\u006fr":"invalid_scope"}"#,
        r#"{"error":"invalid_grant","future":{"du\u0070":1,"dup":2}}"#,
        r#"{"error":"invalid_grant"}[]"#,
        r#"{"error":"invalid_grant","future":[}"#,
    ];
    for json in invalid {
        assert!(parse(json).is_err(), "unexpectedly accepted {json}");
    }
}

#[test]
fn response_types_and_errors_keep_all_remote_content_redacted() {
    let code_canary = "VENDOR_ERROR_CANARY_61c0";
    let description_canary = "DESCRIPTION_CANARY_0aa2";
    let uri_canary = "https://example.invalid/URI_CANARY_8fa4";
    let extension_canary = "EXTENSION_CANARY_42d3";
    let json = format!(
        r#"{{"error":"{code_canary}","error_description":"{description_canary}","error_uri":"{uri_canary}","future":"{extension_canary}"}}"#
    );
    let response = parse(&json).expect("canary Token Error Response");

    let rendered = [
        format!("{response:?}"),
        format!("{:?}", response.error()),
        format!("{:?}", response.error_uri().expect("URI")),
    ];
    for value in rendered {
        for canary in [
            code_canary,
            description_canary,
            uri_canary,
            extension_canary,
        ] {
            assert!(!value.contains(canary));
        }
    }

    let cases = [
        (
            CredentialOfferError::InvalidTokenErrorResponseLimits,
            error_code::INVALID_TOKEN_ERROR_RESPONSE_LIMITS,
        ),
        (
            CredentialOfferError::TokenErrorResponseTooLarge,
            error_code::TOKEN_ERROR_RESPONSE_TOO_LARGE,
        ),
        (
            CredentialOfferError::InvalidTokenErrorResponse,
            error_code::INVALID_TOKEN_ERROR_RESPONSE,
        ),
        (
            CredentialOfferError::InvalidTokenEndpointErrorCode,
            error_code::INVALID_TOKEN_ENDPOINT_ERROR_CODE,
        ),
        (
            CredentialOfferError::TokenEndpointErrorCodeTooLarge,
            error_code::TOKEN_ENDPOINT_ERROR_CODE_TOO_LARGE,
        ),
        (
            CredentialOfferError::InvalidTokenErrorDescription,
            error_code::INVALID_TOKEN_ERROR_DESCRIPTION,
        ),
        (
            CredentialOfferError::TokenErrorDescriptionTooLarge,
            error_code::TOKEN_ERROR_DESCRIPTION_TOO_LARGE,
        ),
        (
            CredentialOfferError::InvalidTokenErrorUri,
            error_code::INVALID_TOKEN_ERROR_URI,
        ),
        (
            CredentialOfferError::TokenErrorUriTooLarge,
            error_code::TOKEN_ERROR_URI_TOO_LARGE,
        ),
    ];
    for (error, code) in cases {
        let core: IdentusError = error.into();
        assert_eq!(core.code(), code);
        assert_eq!(core.kind(), ErrorKind::InvalidInput);
        assert_eq!(core.capability(), Some(CAPABILITY));
        for value in [
            format!("{error}"),
            format!("{error:?}"),
            format!("{core}"),
            format!("{core:?}"),
        ] {
            for canary in [
                code_canary,
                description_canary,
                uri_canary,
                extension_canary,
            ] {
                assert!(!value.contains(canary));
            }
        }
    }
}
