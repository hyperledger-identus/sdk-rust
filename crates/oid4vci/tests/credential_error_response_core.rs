use identus_core::{ErrorKind, IdentusError};
use identus_oid4vci::{
    CAPABILITY, CredentialEndpointErrorKind, CredentialErrorResponseCore,
    CredentialErrorResponseLimits, CredentialOfferError, MAX_CONFIGURABLE_JSON_DEPTH, error_code,
};

fn parse(json: &str) -> Result<CredentialErrorResponseCore, CredentialOfferError> {
    CredentialErrorResponseCore::parse(json, CredentialErrorResponseLimits::default())
}

fn limits(code: usize, description: usize) -> CredentialErrorResponseLimits {
    CredentialErrorResponseLimits::new(1_024, 16, 128, code, description).expect("positive limits")
}

#[test]
fn defaults_and_positive_limits_are_explicit() {
    let defaults = CredentialErrorResponseLimits::default();
    assert_eq!(defaults.max_json_bytes(), 32_768);
    assert_eq!(defaults.max_json_depth(), 16);
    assert_eq!(defaults.max_json_nodes(), 512);
    assert_eq!(defaults.max_error_code_bytes(), 256);
    assert_eq!(defaults.max_error_description_bytes(), 4_096);

    let invalid = [
        CredentialErrorResponseLimits::new(0, 1, 1, 1, 1),
        CredentialErrorResponseLimits::new(1, 0, 1, 1, 1),
        CredentialErrorResponseLimits::new(1, MAX_CONFIGURABLE_JSON_DEPTH + 1, 1, 1, 1),
        CredentialErrorResponseLimits::new(1, 1, 0, 1, 1),
        CredentialErrorResponseLimits::new(1, 1, 1, 0, 1),
        CredentialErrorResponseLimits::new(1, 1, 1, 1, 0),
    ];
    for result in invalid {
        assert_eq!(
            result,
            Err(CredentialOfferError::InvalidCredentialErrorResponseLimits)
        );
    }
}

#[test]
fn minimal_final_error_exposes_only_deliberate_core_values() {
    let json = r#"{"error":"invalid_proof"}"#;
    let response = parse(json).expect("minimal Credential Error Response");

    assert_eq!(response.response_len(), json.len());
    assert_eq!(response.error().as_str(), "invalid_proof");
    assert_eq!(
        response.error().kind(),
        CredentialEndpointErrorKind::InvalidProof
    );
    assert_eq!(
        response.error_kind(),
        CredentialEndpointErrorKind::InvalidProof
    );
    assert!(!response.error_description_present());
    assert_eq!(response.expose_untrusted_description(), None);
}

#[test]
fn exact_final_codes_are_classified_and_extensions_remain_open() {
    let cases = [
        (
            "invalid_credential_request",
            CredentialEndpointErrorKind::InvalidCredentialRequest,
        ),
        (
            "unknown_credential_configuration",
            CredentialEndpointErrorKind::UnknownCredentialConfiguration,
        ),
        (
            "unknown_credential_identifier",
            CredentialEndpointErrorKind::UnknownCredentialIdentifier,
        ),
        ("invalid_proof", CredentialEndpointErrorKind::InvalidProof),
        ("invalid_nonce", CredentialEndpointErrorKind::InvalidNonce),
        (
            "invalid_encryption_parameters",
            CredentialEndpointErrorKind::InvalidEncryptionParameters,
        ),
        (
            "credential_request_denied",
            CredentialEndpointErrorKind::CredentialRequestDenied,
        ),
        ("invalid_Proof", CredentialEndpointErrorKind::Extension),
        (
            "vendor future code!",
            CredentialEndpointErrorKind::Extension,
        ),
    ];

    for (code, expected) in cases {
        let json = format!(r#"{{"error":"{code}"}}"#);
        let response = parse(&json).expect("valid error code");
        assert_eq!(response.error().as_str(), code);
        assert_eq!(response.error_kind(), expected);
    }
}

#[test]
fn description_is_untrusted_and_foreign_or_legacy_fields_are_discarded() {
    let json = r#"{
        "error":"invalid_nonce",
        "error_description":"The proof nonce is no longer valid.",
        "c_nonce":"legacy-secret",
        "error_uri":"https://example.invalid/error",
        "future":{"nested":[true,null,{"number":1e999999}]}
    }"#;
    let response = parse(json).expect("complete Credential Error Response");

    assert!(response.error_description_present());
    assert_eq!(
        response.expose_untrusted_description(),
        Some("The proof nonce is no longer valid.")
    );
    assert_eq!(
        response.error_kind(),
        CredentialEndpointErrorKind::InvalidNonce
    );
}

#[test]
fn required_code_and_optional_description_fail_closed() {
    let invalid_codes = [
        r#"{}"#,
        r#"{"error":""}"#,
        r#"{"error":null}"#,
        r#"{"error":7}"#,
        r#"{"error":"bad\"code"}"#,
        r#"{"error":"bad\\code"}"#,
        r#"{"error":"bad\u0009code"}"#,
        r#"{"error":"ümlaut"}"#,
    ];
    for json in invalid_codes {
        assert!(parse(json).is_err(), "unexpectedly accepted {json}");
    }

    for description in ["", "quote\"text", "slash\\text", "line\ntext", "ümlaut"] {
        let encoded = serde_json::to_string(description).expect("description JSON");
        let json = format!(r#"{{"error":"invalid_proof","error_description":{encoded}}}"#);
        assert_eq!(
            parse(&json).expect_err("invalid description"),
            CredentialOfferError::InvalidCredentialErrorDescription
        );
    }
}

#[test]
fn aggregate_structure_and_independent_value_limits_are_enforced() {
    let minimal = r#"{"error":"invalid_proof"}"#;
    assert_eq!(
        CredentialErrorResponseCore::parse(
            minimal,
            CredentialErrorResponseLimits::new(minimal.len(), 16, 128, 32, 32)
                .expect("exact byte limit"),
        )
        .expect("exact aggregate bound")
        .response_len(),
        minimal.len()
    );
    assert_eq!(
        CredentialErrorResponseCore::parse(
            minimal,
            CredentialErrorResponseLimits::new(minimal.len() - 1, 16, 128, 32, 32)
                .expect("small byte limit"),
        )
        .expect_err("aggregate oversize"),
        CredentialOfferError::CredentialErrorResponseTooLarge
    );

    let cases = [
        (
            r#"{"error":"invalid_proof"}"#,
            limits(12, 64),
            CredentialOfferError::CredentialEndpointErrorCodeTooLarge,
        ),
        (
            r#"{"error":"invalid_proof","error_description":"detail"}"#,
            limits(64, 5),
            CredentialOfferError::CredentialErrorDescriptionTooLarge,
        ),
    ];
    for (json, limits, expected) in cases {
        assert_eq!(
            CredentialErrorResponseCore::parse(json, limits).expect_err("known value oversize"),
            expected
        );
    }

    let nested = r#"{"error":"invalid_proof","future":{"value":1}}"#;
    assert_eq!(
        CredentialErrorResponseCore::parse(
            nested,
            CredentialErrorResponseLimits::new(1_024, 1, 128, 64, 64).expect("shallow limit"),
        )
        .expect_err("deep response"),
        CredentialOfferError::JsonTooDeep
    );
    assert_eq!(
        CredentialErrorResponseCore::parse(
            minimal,
            CredentialErrorResponseLimits::new(1_024, 16, 1, 64, 64).expect("narrow node limit"),
        )
        .expect_err("wide response"),
        CredentialOfferError::JsonTooManyNodes
    );
}

#[test]
fn malformed_duplicate_nested_and_trailing_json_fail_closed() {
    let invalid = [
        r#"[]"#,
        r#"{"error":"invalid_proof","err\u006fr":"invalid_nonce"}"#,
        r#"{"error":"invalid_proof","future":{"du\u0070":1,"dup":2}}"#,
        r#"{"error":"invalid_proof"}[]"#,
        r#"{"error":"invalid_proof","future":[}"#,
    ];
    for json in invalid {
        assert!(parse(json).is_err(), "unexpectedly accepted {json}");
    }
}

#[test]
fn response_types_and_errors_keep_all_remote_content_redacted() {
    let code_canary = "VENDOR_ERROR_CANARY_7fc1";
    let description_canary = "DESCRIPTION_CANARY_8ba2";
    let legacy_canary = "LEGACY_NONCE_CANARY_9de3";
    let extension_canary = "EXTENSION_CANARY_1af4";
    let json = format!(
        r#"{{"error":"{code_canary}","error_description":"{description_canary}","c_nonce":"{legacy_canary}","future":"{extension_canary}"}}"#
    );
    let response = parse(&json).expect("canary Credential Error Response");

    for value in [format!("{response:?}"), format!("{:?}", response.error())] {
        for canary in [
            code_canary,
            description_canary,
            legacy_canary,
            extension_canary,
        ] {
            assert!(!value.contains(canary));
        }
    }

    let cases = [
        (
            CredentialOfferError::InvalidCredentialErrorResponseLimits,
            error_code::INVALID_CREDENTIAL_ERROR_RESPONSE_LIMITS,
        ),
        (
            CredentialOfferError::CredentialErrorResponseTooLarge,
            error_code::CREDENTIAL_ERROR_RESPONSE_TOO_LARGE,
        ),
        (
            CredentialOfferError::InvalidCredentialErrorResponse,
            error_code::INVALID_CREDENTIAL_ERROR_RESPONSE,
        ),
        (
            CredentialOfferError::InvalidCredentialEndpointErrorCode,
            error_code::INVALID_CREDENTIAL_ENDPOINT_ERROR_CODE,
        ),
        (
            CredentialOfferError::CredentialEndpointErrorCodeTooLarge,
            error_code::CREDENTIAL_ENDPOINT_ERROR_CODE_TOO_LARGE,
        ),
        (
            CredentialOfferError::InvalidCredentialErrorDescription,
            error_code::INVALID_CREDENTIAL_ERROR_DESCRIPTION,
        ),
        (
            CredentialOfferError::CredentialErrorDescriptionTooLarge,
            error_code::CREDENTIAL_ERROR_DESCRIPTION_TOO_LARGE,
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
                legacy_canary,
                extension_canary,
            ] {
                assert!(!value.contains(canary));
            }
        }
    }
}
