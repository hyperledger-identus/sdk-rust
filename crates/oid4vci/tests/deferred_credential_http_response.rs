use identus_core::{ErrorKind, IdentusError};
use identus_oid4vci::{
    CAPABILITY, CredentialIssuerMetadata, CredentialIssuerMetadataLimits, CredentialOfferError,
    DeferredCredentialHttpResponseLimits, DeferredCredentialOutcome, DeferredCredentialRequest,
    DeferredCredentialRequestLimits, DeferredCredentialResponseCore,
    DeferredCredentialResponseLimits, ImmediateCredentialResponseLimits, error_code,
};

const ISSUER: &str = "https://credential-issuer.example.com";
const DEFERRED_ENDPOINT: &str = "https://credential-issuer.example.com/deferred_credential";

fn request(transaction_id: &str) -> DeferredCredentialRequest {
    let transaction_id_json = serde_json::to_string(transaction_id).expect("transaction JSON");
    let response = DeferredCredentialResponseCore::parse(
        &format!(r#"{{"transaction_id":{transaction_id_json},"interval":5}}"#),
        DeferredCredentialResponseLimits::default(),
    )
    .expect("deferred response");
    let metadata = CredentialIssuerMetadata::parse(
        &format!(
            r#"{{"credential_issuer":"{ISSUER}","credential_endpoint":"{ISSUER}/credential","deferred_credential_endpoint":"{DEFERRED_ENDPOINT}","credential_configurations_supported":{{"degree":{{"format":"dc+sd-jwt"}}}}}}"#,
        ),
        ISSUER,
        CredentialIssuerMetadataLimits::default(),
    )
    .expect("issuer metadata");

    response
        .try_deferred_credential_request(&metadata, DeferredCredentialRequestLimits::default())
        .expect("Deferred Credential Request")
}

#[test]
fn defaults_and_positive_content_type_limit_are_explicit() {
    let defaults = DeferredCredentialHttpResponseLimits::default();
    assert_eq!(defaults.max_content_type_bytes(), 1_024);
    assert_eq!(
        defaults.immediate_response_limits(),
        ImmediateCredentialResponseLimits::default()
    );
    assert_eq!(
        defaults.deferred_response_limits(),
        DeferredCredentialResponseLimits::default()
    );
    assert_eq!(
        DeferredCredentialHttpResponseLimits::new(
            ImmediateCredentialResponseLimits::default(),
            DeferredCredentialResponseLimits::default(),
            0,
        ),
        Err(CredentialOfferError::InvalidDeferredCredentialHttpResponseLimits)
    );
}

#[test]
fn exact_200_json_response_returns_issued_outcome_without_cardinality_claim() {
    let request = request("issued-transaction");
    let body = r#"{"credentials":[{"credential":"one"},{"credential":{"claim":2}}]}"#;
    let outcome = request
        .validate_response(
            200,
            "Application/JSON ; Charset=\"utf-8\"",
            body,
            DeferredCredentialHttpResponseLimits::default(),
        )
        .expect("issued response");
    let issued = outcome.issued().expect("issued branch");

    assert!(outcome.pending().is_none());
    let diagnostic = format!("{outcome:?}");
    assert!(!diagnostic.contains("one"));
    assert!(!diagnostic.contains("claim"));
    assert_eq!(issued.credentials().len(), 2);
    assert_eq!(
        issued.credentials()[0].expose_sensitive_string(),
        Some("one")
    );
    assert_eq!(
        issued.credentials()[1].expose_sensitive_json(),
        r#"{"claim":2}"#
    );
}

#[test]
fn exact_202_json_response_returns_correlated_pending_outcome() {
    let request = request("8xLOxBtZp8");
    let body = r#"{"transaction_id":"8xLOxBtZp8","interval":10}"#;
    let outcome = request
        .validate_response(
            202,
            "application/json",
            body,
            DeferredCredentialHttpResponseLimits::default(),
        )
        .expect("pending response");
    let pending = outcome.pending().expect("pending branch");

    assert!(outcome.issued().is_none());
    assert!(!format!("{outcome:?}").contains("8xLOxBtZp8"));
    assert_eq!(pending.interval().as_str(), "10");
    assert_eq!(pending.response_len(), body.len());
}

#[test]
fn substituted_pending_transaction_is_rejected_statically() {
    let request_canary = "REQUEST_TX_CANARY_23a1";
    let response_canary = "RESPONSE_TX_CANARY_b771";
    let request = request(request_canary);
    let body = format!(r#"{{"transaction_id":"{response_canary}","interval":5}}"#);
    let error = request
        .validate_response(
            202,
            "application/json",
            &body,
            DeferredCredentialHttpResponseLimits::default(),
        )
        .expect_err("mismatched transaction");

    assert_eq!(
        error,
        CredentialOfferError::DeferredCredentialTransactionMismatch
    );
    let core = error.to_identus_error();
    for rendered in [
        format!("{request:?}"),
        format!("{error:?}"),
        format!("{error}"),
        format!("{core:?}"),
        format!("{core}"),
    ] {
        assert!(!rendered.contains(request_canary));
        assert!(!rendered.contains(response_canary));
    }
}

#[test]
fn status_is_classified_before_untrusted_fields() {
    let request = request("status-order");
    for status in [0, 199, 201, 204, 400, 500] {
        assert_eq!(
            request
                .validate_response(
                    status,
                    "CONTENT_TYPE_CANARY",
                    "BODY_CANARY",
                    DeferredCredentialHttpResponseLimits::default(),
                )
                .expect_err("unsupported success status"),
            CredentialOfferError::InvalidDeferredCredentialHttpStatus
        );
    }
}

#[test]
fn content_type_is_bounded_and_checked_before_body() {
    let request = request("media-type");
    let issued_body = r#"{"credentials":[{"credential":"one"}]}"#;
    for media_type in [
        "application/json",
        "APPLICATION/JSON",
        " application/json ; charset=utf-8 ",
        "application/json;charset=\"utf-8\";profile=final",
    ] {
        request
            .validate_response(
                200,
                media_type,
                issued_body,
                DeferredCredentialHttpResponseLimits::default(),
            )
            .expect("valid JSON media type");
    }
    for media_type in [
        "",
        "application/jsonp",
        "text/json",
        "application/json, text/plain",
        "application/json; charset=\"unterminated",
    ] {
        assert_eq!(
            request
                .validate_response(
                    200,
                    media_type,
                    "BODY_CANARY",
                    DeferredCredentialHttpResponseLimits::default(),
                )
                .expect_err("invalid media type"),
            CredentialOfferError::InvalidDeferredCredentialContentType
        );
    }

    let exact = DeferredCredentialHttpResponseLimits::new(
        ImmediateCredentialResponseLimits::default(),
        DeferredCredentialResponseLimits::default(),
        "application/json".len(),
    )
    .expect("positive HTTP limits");
    request
        .validate_response(200, "application/json", issued_body, exact)
        .expect("exact Content-Type bound");
    assert_eq!(
        request
            .validate_response(200, "application/json ", "BODY_CANARY", exact)
            .expect_err("oversized Content-Type"),
        CredentialOfferError::DeferredCredentialContentTypeTooLarge
    );
}

#[test]
fn status_selected_body_limits_are_preserved() {
    let request = request("bounded-body");
    let tiny_immediate = ImmediateCredentialResponseLimits::new(1, 1, 1, 1, 1, 1, 1, 1, 1)
        .expect("positive immediate limits");
    let tiny_deferred =
        DeferredCredentialResponseLimits::new(1, 1, 1, 1, 1, 1).expect("positive deferred limits");
    let limits = DeferredCredentialHttpResponseLimits::new(tiny_immediate, tiny_deferred, 64)
        .expect("positive HTTP limits");

    assert_eq!(
        request
            .validate_response(
                200,
                "application/json",
                r#"{"credentials":[{"credential":"one"}]}"#,
                limits,
            )
            .expect_err("immediate body bound"),
        CredentialOfferError::ImmediateCredentialResponseTooLarge
    );
    assert_eq!(
        request
            .validate_response(
                202,
                "application/json",
                r#"{"transaction_id":"bounded-body","interval":5}"#,
                limits,
            )
            .expect_err("deferred body bound"),
        CredentialOfferError::DeferredCredentialResponseTooLarge
    );
}

#[test]
fn repeated_validation_is_structural_and_policy_free() {
    let request = request("repeatable");
    let body = r#"{"transaction_id":"repeatable","interval":5}"#;
    for _ in 0..2 {
        let outcome = request
            .validate_response(
                202,
                "application/json",
                body,
                DeferredCredentialHttpResponseLimits::default(),
            )
            .expect("repeatable validation");
        assert!(matches!(outcome, DeferredCredentialOutcome::Pending(_)));
    }
}

#[test]
fn new_errors_bridge_to_static_codes() {
    let cases = [
        (
            CredentialOfferError::InvalidDeferredCredentialHttpResponseLimits,
            error_code::INVALID_DEFERRED_CREDENTIAL_HTTP_RESPONSE_LIMITS,
            "OID4VCI Deferred Credential HTTP response limits are invalid",
        ),
        (
            CredentialOfferError::InvalidDeferredCredentialHttpStatus,
            error_code::INVALID_DEFERRED_CREDENTIAL_HTTP_STATUS,
            "OID4VCI Deferred Credential HTTP status is invalid",
        ),
        (
            CredentialOfferError::DeferredCredentialContentTypeTooLarge,
            error_code::DEFERRED_CREDENTIAL_CONTENT_TYPE_TOO_LARGE,
            "OID4VCI Deferred Credential Content-Type is too large",
        ),
        (
            CredentialOfferError::InvalidDeferredCredentialContentType,
            error_code::INVALID_DEFERRED_CREDENTIAL_CONTENT_TYPE,
            "OID4VCI Deferred Credential Content-Type is invalid",
        ),
        (
            CredentialOfferError::DeferredCredentialTransactionMismatch,
            error_code::DEFERRED_CREDENTIAL_TRANSACTION_MISMATCH,
            "OID4VCI Deferred Credential transaction does not match its request",
        ),
    ];

    for (error, code, message) in cases {
        let core: IdentusError = error.into();
        assert_eq!(core.code(), code);
        assert_eq!(core.kind(), ErrorKind::InvalidInput);
        assert_eq!(core.capability(), Some(CAPABILITY));
        assert_eq!(error.to_string(), message);
        assert_eq!(core.public_message(), message);
        assert_eq!(core.to_string(), format!("{code}: {message}"));
    }
}
