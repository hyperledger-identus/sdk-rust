use identus_core::{ErrorKind, IdentusError};
use identus_oid4vci::{
    CAPABILITY, CredentialEndpointErrorKind, CredentialErrorHttpResponseLimits,
    CredentialErrorResponseCore, CredentialErrorResponseLimits, CredentialOfferError, error_code,
};

#[test]
fn defaults_and_positive_content_type_limit_are_explicit() {
    let defaults = CredentialErrorHttpResponseLimits::default();
    assert_eq!(defaults.max_content_type_bytes(), 1_024);
    assert_eq!(
        defaults.response_limits(),
        CredentialErrorResponseLimits::default()
    );
    assert_eq!(
        CredentialErrorHttpResponseLimits::new(CredentialErrorResponseLimits::default(), 0),
        Err(CredentialOfferError::InvalidCredentialErrorHttpResponseLimits)
    );
}

#[test]
fn exact_400_json_envelope_returns_the_bounded_core() {
    let body = r#"{"error":"invalid_proof","error_description":"proof rejected"}"#;
    let response = CredentialErrorResponseCore::parse_http_response(
        400,
        "Application/JSON ; Charset=\"utf-8\"",
        body,
        CredentialErrorHttpResponseLimits::default(),
    )
    .expect("Credential payload-error response");

    assert_eq!(response.response_len(), body.len());
    assert_eq!(response.error().as_str(), "invalid_proof");
    assert_eq!(
        response.error_kind(),
        CredentialEndpointErrorKind::InvalidProof
    );
    assert_eq!(
        response.expose_untrusted_description(),
        Some("proof rejected")
    );
}

#[test]
fn all_final_codes_keep_their_existing_classification() {
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
    ];

    for (code, kind) in cases {
        let body = format!(r#"{{"error":"{code}"}}"#);
        let response = CredentialErrorResponseCore::parse_http_response(
            400,
            "application/json",
            &body,
            CredentialErrorHttpResponseLimits::default(),
        )
        .expect("known Final code");
        assert_eq!(response.error().as_str(), code);
        assert_eq!(response.error_kind(), kind);
    }
}

#[test]
fn generic_invalid_request_is_forbidden_but_other_extensions_survive() {
    assert_eq!(
        CredentialErrorResponseCore::parse_http_response(
            400,
            "application/json",
            r#"{"error":"invalid_request"}"#,
            CredentialErrorHttpResponseLimits::default(),
        )
        .expect_err("generic RFC 6750 code"),
        CredentialOfferError::GenericCredentialErrorCodeForbidden
    );

    let response = CredentialErrorResponseCore::parse_http_response(
        400,
        "application/json",
        r#"{"error":"vendor_specific_error"}"#,
        CredentialErrorHttpResponseLimits::default(),
    )
    .expect("extension code");
    assert_eq!(response.error().as_str(), "vendor_specific_error");
    assert_eq!(
        response.error_kind(),
        CredentialEndpointErrorKind::Extension
    );
}

#[test]
fn status_is_classified_before_untrusted_fields() {
    for status in [0, 199, 200, 201, 202, 204, 299, 401, 500] {
        assert_eq!(
            CredentialErrorResponseCore::parse_http_response(
                status,
                "CONTENT_TYPE_CANARY_3c70",
                "BODY_CANARY_7a93",
                CredentialErrorHttpResponseLimits::default(),
            )
            .expect_err("non-400 status"),
            CredentialOfferError::InvalidCredentialErrorHttpStatus
        );
    }
}

#[test]
fn content_type_is_strict_bounded_and_checked_before_body() {
    let body = r#"{"error":"invalid_nonce"}"#;
    for media_type in [
        "application/json",
        "APPLICATION/JSON",
        " application/json ; charset=utf-8 ",
        "application/json;charset=\"utf-8\";profile=final",
    ] {
        CredentialErrorResponseCore::parse_http_response(
            400,
            media_type,
            body,
            CredentialErrorHttpResponseLimits::default(),
        )
        .expect("valid JSON media type");
    }
    for media_type in [
        "",
        "application/jsonp",
        "text/json",
        "application/json, text/plain",
        "application/json; charset=\"unterminated",
        "application/json; charset",
        "application/json;charset=utf-8;CHARSET=iso-8859-1",
        "application/json\r\nX-Injected: true",
    ] {
        assert_eq!(
            CredentialErrorResponseCore::parse_http_response(
                400,
                media_type,
                "BODY_CANARY_f105",
                CredentialErrorHttpResponseLimits::default(),
            )
            .expect_err("invalid media type"),
            CredentialOfferError::InvalidCredentialErrorContentType,
            "unexpected result for {media_type:?}"
        );
    }

    let exact = CredentialErrorHttpResponseLimits::new(
        CredentialErrorResponseLimits::default(),
        "application/json".len(),
    )
    .expect("positive HTTP limits");
    CredentialErrorResponseCore::parse_http_response(400, "application/json", body, exact)
        .expect("exact Content-Type bound");
    assert_eq!(
        CredentialErrorResponseCore::parse_http_response(
            400,
            "application/json ",
            "BODY_CANARY_41ee",
            exact,
        )
        .expect_err("oversized Content-Type"),
        CredentialOfferError::CredentialErrorContentTypeTooLarge
    );
}

#[test]
fn existing_body_limits_and_errors_are_preserved() {
    let tiny_body =
        CredentialErrorResponseLimits::new(1, 1, 1, 1, 1).expect("positive response limits");
    assert_eq!(
        CredentialErrorResponseCore::parse_http_response(
            400,
            "application/json",
            r#"{"error":"invalid_nonce"}"#,
            CredentialErrorHttpResponseLimits::new(tiny_body, 64).expect("positive HTTP limits"),
        )
        .expect_err("body bound"),
        CredentialOfferError::CredentialErrorResponseTooLarge
    );
    assert_eq!(
        CredentialErrorResponseCore::parse_http_response(
            400,
            "application/json",
            r#"{"error":"invalid_nonce","error":"invalid_proof"}"#,
            CredentialErrorHttpResponseLimits::default(),
        )
        .expect_err("duplicate body member"),
        CredentialOfferError::DuplicateJsonProperty
    );
}

#[test]
fn diagnostics_and_bridge_codes_are_static_and_redacted() {
    let content_type_canary = "CONTENT_TYPE_CANARY_bc32";
    let body_canary = "BODY_CANARY_9fd2";
    let cases = [
        (
            CredentialOfferError::InvalidCredentialErrorHttpResponseLimits,
            error_code::INVALID_CREDENTIAL_ERROR_HTTP_RESPONSE_LIMITS,
        ),
        (
            CredentialOfferError::InvalidCredentialErrorHttpStatus,
            error_code::INVALID_CREDENTIAL_ERROR_HTTP_STATUS,
        ),
        (
            CredentialOfferError::CredentialErrorContentTypeTooLarge,
            error_code::CREDENTIAL_ERROR_CONTENT_TYPE_TOO_LARGE,
        ),
        (
            CredentialOfferError::InvalidCredentialErrorContentType,
            error_code::INVALID_CREDENTIAL_ERROR_CONTENT_TYPE,
        ),
        (
            CredentialOfferError::GenericCredentialErrorCodeForbidden,
            error_code::GENERIC_CREDENTIAL_ERROR_CODE_FORBIDDEN,
        ),
    ];

    for (error, code) in cases {
        let core: IdentusError = error.into();
        assert_eq!(core.code(), code);
        assert_eq!(core.kind(), ErrorKind::InvalidInput);
        assert_eq!(core.capability(), Some(CAPABILITY));
        for rendered in [
            format!("{error}"),
            format!("{error:?}"),
            format!("{core}"),
            format!("{core:?}"),
        ] {
            assert!(!rendered.contains(content_type_canary));
            assert!(!rendered.contains(body_canary));
        }
    }

    let error = CredentialErrorResponseCore::parse_http_response(
        500,
        content_type_canary,
        body_canary,
        CredentialErrorHttpResponseLimits::default(),
    )
    .expect_err("redacted status failure");
    let rendered = format!("{error:?} {}", error.to_identus_error());
    assert!(!rendered.contains(content_type_canary));
    assert!(!rendered.contains(body_canary));
}
