use identus_core::{ErrorKind, IdentusError};
use identus_oid4vci::{
    CAPABILITY, CredentialIssuerMetadata, CredentialIssuerMetadataLimits, CredentialOfferError,
    DEFERRED_CREDENTIAL_REQUEST_HTTP_METHOD, DEFERRED_CREDENTIAL_REQUEST_MEDIA_TYPE,
    DeferredCredentialRequestLimits, DeferredCredentialResponseCore,
    DeferredCredentialResponseLimits, error_code,
};

const ISSUER: &str = "https://credential-issuer.example.com";
const DEFERRED_ENDPOINT: &str = "https://server.example.com/deferred_credential";

fn metadata(endpoint: Option<&str>) -> CredentialIssuerMetadata {
    let endpoint = endpoint.map_or_else(String::new, |value| {
        format!(r#", "deferred_credential_endpoint":"{value}""#)
    });
    let json = format!(
        r#"{{"credential_issuer":"{ISSUER}","credential_endpoint":"{ISSUER}/credential"{endpoint},"credential_configurations_supported":{{"degree":{{"format":"dc+sd-jwt"}}}}}}"#,
    );
    CredentialIssuerMetadata::parse(&json, ISSUER, CredentialIssuerMetadataLimits::default())
        .expect("valid issuer metadata")
}

fn response(transaction_id: &str) -> DeferredCredentialResponseCore {
    let transaction_id = serde_json::to_string(transaction_id).expect("string JSON");
    DeferredCredentialResponseCore::parse(
        &format!(r#"{{"transaction_id":{transaction_id},"interval":5}}"#),
        DeferredCredentialResponseLimits::default(),
    )
    .expect("valid deferred response")
}

#[test]
fn defaults_and_positive_limits_are_explicit() {
    let defaults = DeferredCredentialRequestLimits::default();
    assert_eq!(defaults.max_json_body_bytes(), 16_384);
    assert_eq!(
        DeferredCredentialRequestLimits::new(0),
        Err(CredentialOfferError::InvalidDeferredCredentialRequestLimits)
    );
    assert_eq!(
        DeferredCredentialRequestLimits::new(17)
            .expect("positive limit")
            .max_json_body_bytes(),
        17
    );
}

#[test]
fn final_example_constructs_the_exact_unencrypted_request() {
    let response = response("8xLOxBtZp8");
    let metadata = metadata(Some(DEFERRED_ENDPOINT));
    let request = response
        .try_deferred_credential_request(&metadata, DeferredCredentialRequestLimits::default())
        .expect("Deferred Credential Request");

    assert_eq!(
        request.deferred_credential_endpoint().as_str(),
        DEFERRED_ENDPOINT
    );
    assert_eq!(request.http_method(), "POST");
    assert_eq!(
        request.http_method(),
        DEFERRED_CREDENTIAL_REQUEST_HTTP_METHOD
    );
    assert_eq!(request.media_type(), "application/json");
    assert_eq!(request.media_type(), DEFERRED_CREDENTIAL_REQUEST_MEDIA_TYPE);
    assert_eq!(
        request.expose_sensitive_json_body(),
        br#"{"transaction_id":"8xLOxBtZp8"}"#
    );
    assert_eq!(
        request.json_body_len(),
        request.expose_sensitive_json_body().len()
    );
    assert!(request.access_token_required());
}

#[test]
fn json_serializer_preserves_escaped_and_non_ascii_transaction_text() {
    let transaction = "quote\" slash\\ newline\n snowman ☃";
    let response = response(transaction);
    let request = response
        .try_deferred_credential_request(
            &metadata(Some(DEFERRED_ENDPOINT)),
            DeferredCredentialRequestLimits::default(),
        )
        .expect("escaped request");
    let value: serde_json::Value =
        serde_json::from_slice(request.expose_sensitive_json_body()).expect("request JSON");

    assert_eq!(value.as_object().map(serde_json::Map::len), Some(1));
    assert_eq!(value["transaction_id"].as_str(), Some(transaction));
}

#[test]
fn complete_body_limit_accepts_exact_and_rejects_one_under() {
    let response = response("quote\"and\\slash");
    let metadata = metadata(Some(DEFERRED_ENDPOINT));
    let expected = serde_json::to_vec(&serde_json::json!({
        "transaction_id": "quote\"and\\slash"
    }))
    .expect("expected JSON");
    let exact = DeferredCredentialRequestLimits::new(expected.len()).expect("exact limit");
    let request = response
        .try_deferred_credential_request(&metadata, exact)
        .expect("exact request limit");
    assert_eq!(request.expose_sensitive_json_body(), expected);

    let one_under = DeferredCredentialRequestLimits::new(expected.len() - 1).expect("one under");
    assert_eq!(
        response
            .try_deferred_credential_request(&metadata, one_under)
            .expect_err("one byte too small"),
        CredentialOfferError::DeferredCredentialRequestTooLarge
    );
}

#[test]
fn absent_endpoint_fails_statically_without_exposing_the_transaction() {
    let transaction_canary = "MISSING_ENDPOINT_TX_CANARY_52a9";
    let response = response(transaction_canary);
    let metadata = metadata(None);
    let error = response
        .try_deferred_credential_request(&metadata, DeferredCredentialRequestLimits::default())
        .expect_err("missing Deferred Credential Endpoint");

    assert_eq!(
        error,
        CredentialOfferError::DeferredCredentialEndpointRequired
    );
    assert!(metadata.deferred_credential_endpoint().is_none());
    let rendered = format!("{error:?} {error} {:?}", error.to_identus_error());
    assert!(!rendered.contains(transaction_canary));
}

#[test]
fn repeated_structural_construction_is_identical_and_policy_free() {
    let response = response("pollable-transaction");
    let metadata = metadata(Some(DEFERRED_ENDPOINT));
    let first = response
        .try_deferred_credential_request(&metadata, DeferredCredentialRequestLimits::default())
        .expect("first request");
    let second = response
        .try_deferred_credential_request(&metadata, DeferredCredentialRequestLimits::default())
        .expect("second request");

    assert_eq!(
        first.expose_sensitive_json_body(),
        second.expose_sensitive_json_body()
    );
    assert_eq!(first.http_method(), second.http_method());
    assert_eq!(first.media_type(), second.media_type());
}

#[test]
fn request_debug_and_errors_redact_endpoint_and_transaction_content() {
    let endpoint_canary = "ENDPOINT_CANARY_f1c4";
    let transaction_canary = "TRANSACTION_CANARY_b7e2";
    let endpoint = format!("https://issuer.example/{endpoint_canary}");
    let response = response(transaction_canary);
    let request = response
        .try_deferred_credential_request(
            &metadata(Some(&endpoint)),
            DeferredCredentialRequestLimits::default(),
        )
        .expect("canary request");

    for diagnostic in [
        format!("{request:?}"),
        format!("{:?}", request.deferred_credential_endpoint()),
        format!("{response:?}"),
    ] {
        assert!(!diagnostic.contains(endpoint_canary));
        assert!(!diagnostic.contains(transaction_canary));
    }

    let error = response
        .try_deferred_credential_request(
            &metadata(Some(&endpoint)),
            DeferredCredentialRequestLimits::new(1).expect("tiny limit"),
        )
        .expect_err("oversized request");
    let core: IdentusError = error.into();
    for diagnostic in [
        format!("{error:?}"),
        format!("{error}"),
        format!("{core:?}"),
        format!("{core}"),
    ] {
        assert!(!diagnostic.contains(endpoint_canary));
        assert!(!diagnostic.contains(transaction_canary));
    }
}

#[test]
fn request_errors_bridge_to_static_codes() {
    let cases = [
        (
            CredentialOfferError::InvalidDeferredCredentialRequestLimits,
            error_code::INVALID_DEFERRED_CREDENTIAL_REQUEST_LIMITS,
        ),
        (
            CredentialOfferError::DeferredCredentialEndpointRequired,
            error_code::DEFERRED_CREDENTIAL_ENDPOINT_REQUIRED,
        ),
        (
            CredentialOfferError::DeferredCredentialRequestTooLarge,
            error_code::DEFERRED_CREDENTIAL_REQUEST_TOO_LARGE,
        ),
    ];

    for (error, code) in cases {
        let core: IdentusError = error.into();
        assert_eq!(core.code(), code);
        assert_eq!(core.kind(), ErrorKind::InvalidInput);
        assert_eq!(core.capability(), Some(CAPABILITY));
    }
}
