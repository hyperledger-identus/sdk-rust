use identus_oid4vci::{
    CredentialEndpointErrorKind, CredentialErrorHttpResponseLimits, CredentialErrorResponseLimits,
    CredentialIssuerMetadata, CredentialIssuerMetadataLimits, CredentialOfferError,
    DeferredCredentialErrorKind, DeferredCredentialRequest, DeferredCredentialRequestLimits,
    DeferredCredentialResponseCore, DeferredCredentialResponseLimits,
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
fn invalid_transaction_id_is_classified_without_inventing_stop_guidance() {
    let request = request("transaction-canary");
    let description = "DESCRIPTION_CANARY_5f42";
    let body =
        format!(r#"{{"error":"invalid_transaction_id","error_description":"{description}"}}"#);
    let response = request
        .validate_error_response(
            400,
            "Application/JSON ; Charset=\"utf-8\"",
            &body,
            CredentialErrorHttpResponseLimits::default(),
        )
        .expect("invalid transaction response");

    assert_eq!(
        response.kind(),
        DeferredCredentialErrorKind::InvalidTransactionId
    );
    assert!(!response.should_stop_polling());
    assert_eq!(response.core().error().as_str(), "invalid_transaction_id");
    assert_eq!(
        response.core().expose_untrusted_description(),
        Some(description)
    );
    let diagnostic = format!("{response:?}");
    assert!(!diagnostic.contains("invalid_transaction_id"));
    assert!(!diagnostic.contains(description));
    assert!(!diagnostic.contains("transaction-canary"));
}

#[test]
fn credential_request_denied_exposes_pure_stop_polling_guidance() {
    let request = request("denied-transaction");
    let body = r#"{"error":"credential_request_denied"}"#;

    for _ in 0..2 {
        let response = request
            .validate_error_response(
                400,
                "application/json",
                body,
                CredentialErrorHttpResponseLimits::default(),
            )
            .expect("denied response");
        assert_eq!(
            response.kind(),
            DeferredCredentialErrorKind::CredentialRequestDenied
        );
        assert!(response.should_stop_polling());
        assert_eq!(response.core().response_len(), body.len());
    }
}

#[test]
fn inherited_known_and_extension_codes_keep_generic_classification() {
    let request = request("inherited-transaction");
    let cases = [
        ("invalid_nonce", CredentialEndpointErrorKind::InvalidNonce),
        (
            "VENDOR_ERROR_CANARY_914d",
            CredentialEndpointErrorKind::Extension,
        ),
    ];

    for (code, inherited) in cases {
        let body = format!(r#"{{"error":"{code}"}}"#);
        let response = request
            .validate_error_response(
                400,
                "application/json",
                &body,
                CredentialErrorHttpResponseLimits::default(),
            )
            .expect("inherited response");
        assert_eq!(
            response.kind(),
            DeferredCredentialErrorKind::Inherited(inherited)
        );
        assert_eq!(response.core().error().as_str(), code);
        assert!(!response.should_stop_polling());
        assert!(!format!("{response:?}").contains(code));
    }
}

#[test]
fn existing_status_and_media_precedence_is_preserved() {
    let request = request("envelope-transaction");
    assert_eq!(
        request
            .validate_error_response(
                202,
                "CONTENT_TYPE_CANARY",
                "BODY_CANARY",
                CredentialErrorHttpResponseLimits::default(),
            )
            .expect_err("wrong payload-error status"),
        CredentialOfferError::InvalidCredentialErrorHttpStatus
    );
    assert_eq!(
        request
            .validate_error_response(
                400,
                "text/plain",
                "BODY_CANARY",
                CredentialErrorHttpResponseLimits::default(),
            )
            .expect_err("wrong payload-error media type"),
        CredentialOfferError::InvalidCredentialErrorContentType
    );
}

#[test]
fn inherited_body_limits_and_forbidden_generic_code_are_preserved() {
    let request = request("body-transaction");
    let tiny_body =
        CredentialErrorResponseLimits::new(1, 1, 1, 1, 1).expect("positive response limits");
    let tiny_limits =
        CredentialErrorHttpResponseLimits::new(tiny_body, 64).expect("positive HTTP limits");
    assert_eq!(
        request
            .validate_error_response(
                400,
                "application/json",
                r#"{"error":"invalid_transaction_id"}"#,
                tiny_limits,
            )
            .expect_err("bounded body"),
        CredentialOfferError::CredentialErrorResponseTooLarge
    );
    assert_eq!(
        request
            .validate_error_response(
                400,
                "application/json",
                r#"{"error":"invalid_request"}"#,
                CredentialErrorHttpResponseLimits::default(),
            )
            .expect_err("generic authorization code"),
        CredentialOfferError::GenericCredentialErrorCodeForbidden
    );
}
