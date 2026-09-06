use identus_core::{ErrorKind, IdentusError};
use identus_oid4vci::{
    AuthorizationServerMetadataCore, AuthorizationServerMetadataLimits, CAPABILITY,
    CredentialIssuerMetadata, CredentialIssuerMetadataLimits, CredentialOffer,
    CredentialOfferError, CredentialOfferGrantLimits, CredentialOfferLimits,
    CredentialOfferSemanticLimits, EmbeddedCredentialOffer, PRE_AUTHORIZED_CODE_GRANT_TYPE,
    PreAuthorizedTokenRequestLimits, TOKEN_REQUEST_HTTP_METHOD, TOKEN_REQUEST_MEDIA_TYPE,
    TransactionCodeInputLimits, error_code,
};

const ISSUER: &str = "https://credential-issuer.example";

fn prepared_input(
    transaction_code_requirement: Option<&str>,
    pre_authorized_code: &str,
    transaction_code: Option<&str>,
    endpoint_suffix: &str,
) -> identus_oid4vci::CredentialOfferWithPreAuthorizedTokenInput {
    let transaction_code_requirement = transaction_code_requirement
        .map_or_else(String::new, |value| format!(r#", "tx_code":{value}"#));
    let offer_json = format!(
        r#"{{"credential_issuer":"{ISSUER}","credential_configuration_ids":["degree"],"grants":{{"{PRE_AUTHORIZED_CODE_GRANT_TYPE}":{{"pre-authorized_code":"{pre_authorized_code}"{transaction_code_requirement}}}}}}}"#,
    );
    let offer = CredentialOffer::try_from_embedded(
        EmbeddedCredentialOffer::try_from_json(&offer_json, CredentialOfferLimits::default())
            .expect("offer transport"),
        CredentialOfferSemanticLimits::default(),
    )
    .expect("offer semantics")
    .try_into_grants(CredentialOfferGrantLimits::default())
    .expect("offer grants");
    let issuer_metadata = CredentialIssuerMetadata::parse(
        &format!(
            r#"{{"credential_issuer":"{ISSUER}","credential_endpoint":"{ISSUER}/credential","credential_configurations_supported":{{"degree":{{"format":"dc+sd-jwt"}}}}}}"#,
        ),
        ISSUER,
        CredentialIssuerMetadataLimits::default(),
    )
    .expect("issuer metadata");
    let authorization_metadata = AuthorizationServerMetadataCore::parse(
        &format!(
            r#"{{"issuer":"{ISSUER}","token_endpoint":"{ISSUER}/{endpoint_suffix}","grant_types_supported":["{PRE_AUTHORIZED_CODE_GRANT_TYPE}"]}}"#,
        ),
        ISSUER,
        AuthorizationServerMetadataLimits::default(),
    )
    .expect("Authorization Server Metadata core");

    offer
        .try_with_metadata(issuer_metadata)
        .expect("matched metadata")
        .try_with_pre_authorized_server(authorization_metadata)
        .expect("bound server")
        .try_with_transaction_code_input(
            transaction_code.map(str::to_owned),
            TransactionCodeInputLimits::default(),
        )
        .expect("prepared input")
}

fn request(
    transaction_code_requirement: Option<&str>,
    pre_authorized_code: &str,
    transaction_code: Option<&str>,
) -> identus_oid4vci::PreAuthorizedTokenRequest {
    prepared_input(
        transaction_code_requirement,
        pre_authorized_code,
        transaction_code,
        "token",
    )
    .try_into_pre_authorized_token_request(PreAuthorizedTokenRequestLimits::default())
    .expect("Token Request")
}

#[test]
fn defaults_and_positive_limits_are_explicit() {
    let defaults = PreAuthorizedTokenRequestLimits::default();
    assert_eq!(defaults.max_form_body_bytes(), 16_384);
    assert_eq!(
        PreAuthorizedTokenRequestLimits::new(0),
        Err(CredentialOfferError::InvalidPreAuthorizedTokenRequestLimits)
    );
    assert_eq!(
        PreAuthorizedTokenRequestLimits::new(17)
            .expect("positive limit")
            .max_form_body_bytes(),
        17
    );
}

#[test]
fn request_without_transaction_code_has_exact_mandatory_form() {
    let request = request(None, "secret-code", None);
    let expected = concat!(
        "grant_type=urn%3Aietf%3Aparams%3Aoauth%3Agrant-type%3Apre-authorized_code",
        "&pre-authorized_code=secret-code"
    );

    assert_eq!(request.expose_sensitive_form_body(), expected);
    assert_eq!(request.form_body_len(), expected.len());
    assert!(!request.transaction_code_present());
    assert_eq!(request.http_method(), TOKEN_REQUEST_HTTP_METHOD);
    assert_eq!(request.http_method(), "POST");
    assert_eq!(request.media_type(), TOKEN_REQUEST_MEDIA_TYPE);
    assert_eq!(request.media_type(), "application/x-www-form-urlencoded");
    assert_eq!(
        request.token_endpoint().as_str(),
        "https://credential-issuer.example/token"
    );
}

#[test]
fn request_with_transaction_code_has_exact_order_and_presence() {
    let request = request(Some("{}"), "secret-code", Some("493536"));
    assert_eq!(
        request.expose_sensitive_form_body(),
        concat!(
            "grant_type=urn%3Aietf%3Aparams%3Aoauth%3Agrant-type%3Apre-authorized_code",
            "&pre-authorized_code=secret-code",
            "&tx_code=493536"
        )
    );
    assert!(request.transaction_code_present());
}

#[test]
fn appendix_b_octets_and_form_literals_are_encoded_canonically() {
    let request = request(Some("{}"), "secret code+%&£€*._-/", Some(" %&+£€"));
    assert_eq!(
        request.expose_sensitive_form_body(),
        concat!(
            "grant_type=urn%3Aietf%3Aparams%3Aoauth%3Agrant-type%3Apre-authorized_code",
            "&pre-authorized_code=secret+code%2B%25%26%C2%A3%E2%82%AC*._-%2F",
            "&tx_code=+%25%26%2B%C2%A3%E2%82%AC"
        )
    );
}

#[test]
fn exact_encoded_byte_limit_succeeds_and_one_less_fails() {
    let exact_len = request(Some("{}"), "secret code", Some("£€")).form_body_len();
    let exact = prepared_input(Some("{}"), "secret code", Some("£€"), "token")
        .try_into_pre_authorized_token_request(
            PreAuthorizedTokenRequestLimits::new(exact_len).expect("exact limit"),
        )
        .expect("exact bound");
    assert_eq!(exact.form_body_len(), exact_len);

    assert!(matches!(
        prepared_input(Some("{}"), "secret code", Some("£€"), "token")
            .try_into_pre_authorized_token_request(
                PreAuthorizedTokenRequestLimits::new(exact_len - 1).expect("smaller limit"),
            ),
        Err(CredentialOfferError::PreAuthorizedTokenRequestTooLarge)
    ));
}

#[test]
fn request_and_new_errors_keep_all_diagnostics_redacted() {
    let pre_authorized_canary = "PRE_AUTHORIZED_REQUEST_CANARY_2a81";
    let transaction_canary = "TRANSACTION_REQUEST_CANARY_7df4";
    let endpoint_canary = "TOKEN_ENDPOINT_REQUEST_CANARY_9be3";
    let request = prepared_input(
        Some("{}"),
        pre_authorized_canary,
        Some(transaction_canary),
        endpoint_canary,
    )
    .try_into_pre_authorized_token_request(PreAuthorizedTokenRequestLimits::default())
    .expect("Token Request");
    let rendered = format!("{request:?}");
    for canary in [pre_authorized_canary, transaction_canary, endpoint_canary] {
        assert!(!rendered.contains(canary));
    }

    let cases = [
        (
            CredentialOfferError::InvalidPreAuthorizedTokenRequestLimits,
            error_code::INVALID_PRE_AUTHORIZED_TOKEN_REQUEST_LIMITS,
        ),
        (
            CredentialOfferError::PreAuthorizedTokenRequestTooLarge,
            error_code::PRE_AUTHORIZED_TOKEN_REQUEST_TOO_LARGE,
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
            for canary in [pre_authorized_canary, transaction_canary, endpoint_canary] {
                assert!(!rendered.contains(canary));
            }
        }
    }
}
