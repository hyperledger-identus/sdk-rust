use std::error::Error as _;

use identus_core::{ErrorKind, IdentusError};
use identus_oid4vci::{
    AuthorizationCodeTokenRequestLimits, AuthorizationRequestInputLimits,
    AuthorizationRequestLimits, AuthorizationResponseIssuerIdentification,
    AuthorizationResponseLimits, AuthorizationResponseOutcome, AuthorizationServerMetadataCore,
    AuthorizationServerMetadataLimits, CAPABILITY, CredentialIssuerMetadata,
    CredentialIssuerMetadataLimits, CredentialOffer, CredentialOfferError,
    CredentialOfferGrantLimits, CredentialOfferLimits, CredentialOfferSemanticLimits,
    EmbeddedCredentialOffer, TOKEN_REQUEST_HTTP_METHOD, TOKEN_REQUEST_MEDIA_TYPE, error_code,
};

const ISSUER: &str = "https://credential-issuer.example";
const CONFIGURATION: &str = "UniversityDegreeCredential";
const STATE: &str = "state-value";
const VERIFIER: &str = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
const CLIENT_ID: &str = "wallet client+id";
const REDIRECT_URI: &str = "https://wallet.example/callback?x=1&y=two";
const TOKEN_ENDPOINT: &str = "https://credential-issuer.example/token?tenant=one";

fn correlated_code(
    token_endpoint: Option<&str>,
    rfc9207: Option<bool>,
) -> identus_oid4vci::CorrelatedAuthorizationCode {
    let offer_json = format!(
        r#"{{"credential_issuer":"{ISSUER}","credential_configuration_ids":["{CONFIGURATION}"],"grants":{{"authorization_code":{{}}}}}}"#
    );
    let offer = CredentialOffer::try_from_embedded(
        EmbeddedCredentialOffer::try_from_json(&offer_json, CredentialOfferLimits::default())
            .expect("offer transport"),
        CredentialOfferSemanticLimits::default(),
    )
    .expect("offer semantics")
    .try_into_grants(CredentialOfferGrantLimits::default())
    .expect("offer grants");
    let metadata_json = format!(
        r#"{{"credential_issuer":"{ISSUER}","credential_endpoint":"{ISSUER}/credential","credential_configurations_supported":{{"{CONFIGURATION}":{{"format":"dc+sd-jwt"}}}}}}"#
    );
    let metadata = CredentialIssuerMetadata::parse(
        &metadata_json,
        ISSUER,
        CredentialIssuerMetadataLimits::default(),
    )
    .expect("issuer metadata");
    let matched = offer.try_with_metadata(metadata).expect("matched metadata");

    let token_member = token_endpoint.map_or_else(String::new, |endpoint| {
        format!(r#","token_endpoint":"{endpoint}""#)
    });
    let issuer_flag = rfc9207.map_or_else(String::new, |value| {
        format!(r#","authorization_response_iss_parameter_supported":{value}"#)
    });
    let server_json = format!(
        r#"{{"issuer":"{ISSUER}","authorization_endpoint":"{ISSUER}/authorize"{token_member}{issuer_flag}}}"#
    );
    let server_metadata = AuthorizationServerMetadataCore::parse(
        &server_json,
        ISSUER,
        AuthorizationServerMetadataLimits::default(),
    )
    .expect("Authorization Server Metadata");
    let request = matched
        .try_with_authorization_code_server(server_metadata)
        .expect("server-bound offer")
        .try_with_authorization_request_input(
            0,
            CLIENT_ID,
            REDIRECT_URI,
            STATE,
            VERIFIER,
            AuthorizationRequestInputLimits::default(),
        )
        .expect("request input")
        .try_into_authorization_request(AuthorizationRequestLimits::default())
        .expect("request");
    let issuer_parameter = if rfc9207 == Some(true) {
        "&iss=https%3A%2F%2Fcredential-issuer.example"
    } else {
        ""
    };
    let outcome = request
        .try_into_authorization_response(
            &format!("code=a%2Fb%3Fc&state={STATE}{issuer_parameter}"),
            AuthorizationResponseLimits::default(),
        )
        .expect("correlated response");
    let AuthorizationResponseOutcome::Authorized(code) = outcome else {
        panic!("expected successful response");
    };
    code
}

#[test]
fn constructs_exact_public_client_request_and_preserves_nonsensitive_lineage() {
    let request = correlated_code(Some(TOKEN_ENDPOINT), Some(true))
        .try_into_public_client_token_request(AuthorizationCodeTokenRequestLimits::default())
        .expect("Token Request");
    let expected = concat!(
        "grant_type=authorization_code",
        "&code=a%2Fb%3Fc",
        "&redirect_uri=https%3A%2F%2Fwallet.example%2Fcallback%3Fx%3D1%26y%3Dtwo",
        "&client_id=wallet+client%2Bid",
        "&code_verifier=dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk"
    );

    assert_eq!(request.token_endpoint().as_str(), TOKEN_ENDPOINT);
    assert_eq!(request.http_method(), TOKEN_REQUEST_HTTP_METHOD);
    assert_eq!(request.media_type(), TOKEN_REQUEST_MEDIA_TYPE);
    assert_eq!(request.form_body_len(), expected.len());
    assert_eq!(request.expose_sensitive_form_body(), expected);
    assert_eq!(
        request.issuer_identification(),
        AuthorizationResponseIssuerIdentification::VerifiedRfc9207
    );
    assert_eq!(
        request.selected_credential_configuration().as_str(),
        CONFIGURATION
    );
    assert_eq!(
        request.authorization_server_metadata().issuer().as_str(),
        ISSUER
    );
    assert_eq!(
        request
            .credential_issuer_metadata()
            .credential_issuer()
            .as_str(),
        ISSUER
    );

    let debug = format!("{request:?}");
    assert!(debug.contains("form_body_bytes"));
    for secret in [
        "a/b?c",
        VERIFIER,
        CLIENT_ID,
        REDIRECT_URI,
        TOKEN_ENDPOINT,
        ISSUER,
        expected,
    ] {
        assert!(!debug.contains(secret));
    }
}

#[test]
fn preserves_not_advertised_issuer_evidence_without_upgrade() {
    let request = correlated_code(Some(TOKEN_ENDPOINT), None)
        .try_into_public_client_token_request(AuthorizationCodeTokenRequestLimits::default())
        .expect("Token Request");

    assert_eq!(
        request.issuer_identification(),
        AuthorizationResponseIssuerIdentification::NotAdvertised
    );
}

#[test]
fn requires_the_selected_server_token_endpoint() {
    assert_eq!(
        correlated_code(None, None)
            .try_into_public_client_token_request(AuthorizationCodeTokenRequestLimits::default())
            .expect_err("missing endpoint must fail"),
        CredentialOfferError::TokenEndpointRequired
    );
}

#[test]
fn endpoint_and_body_exact_boundaries_are_enforced_independently() {
    let default_request = correlated_code(Some(TOKEN_ENDPOINT), None)
        .try_into_public_client_token_request(AuthorizationCodeTokenRequestLimits::default())
        .expect("default request");
    let body_len = default_request.form_body_len();

    correlated_code(Some(TOKEN_ENDPOINT), None)
        .try_into_public_client_token_request(
            AuthorizationCodeTokenRequestLimits::new(TOKEN_ENDPOINT.len(), body_len)
                .expect("exact limits"),
        )
        .expect("exact maxima are inclusive");

    assert_eq!(
        correlated_code(Some(TOKEN_ENDPOINT), None)
            .try_into_public_client_token_request(
                AuthorizationCodeTokenRequestLimits::new(TOKEN_ENDPOINT.len() - 1, body_len)
                    .expect("endpoint limit"),
            )
            .expect_err("endpoint maximum plus one must fail"),
        CredentialOfferError::AuthorizationCodeTokenEndpointTooLarge
    );
    assert_eq!(
        correlated_code(Some(TOKEN_ENDPOINT), None)
            .try_into_public_client_token_request(
                AuthorizationCodeTokenRequestLimits::new(TOKEN_ENDPOINT.len(), body_len - 1)
                    .expect("body limit"),
            )
            .expect_err("body maximum plus one must fail"),
        CredentialOfferError::AuthorizationCodeTokenRequestTooLarge
    );
}

#[test]
fn limits_are_strictly_positive() {
    for limits in [
        AuthorizationCodeTokenRequestLimits::new(0, 1),
        AuthorizationCodeTokenRequestLimits::new(1, 0),
    ] {
        assert_eq!(
            limits.expect_err("zero limit must fail"),
            CredentialOfferError::InvalidAuthorizationCodeTokenRequestLimits
        );
    }
}

#[test]
fn appended_errors_are_exact_static_and_redacted() {
    let cases = [
        (
            CredentialOfferError::InvalidAuthorizationCodeTokenRequestLimits,
            error_code::INVALID_AUTHORIZATION_CODE_TOKEN_REQUEST_LIMITS,
            "OID4VCI Authorization Code Token Request limits are invalid",
        ),
        (
            CredentialOfferError::AuthorizationCodeTokenEndpointTooLarge,
            error_code::AUTHORIZATION_CODE_TOKEN_ENDPOINT_TOO_LARGE,
            "OID4VCI Authorization Code Token Endpoint is too large",
        ),
        (
            CredentialOfferError::AuthorizationCodeTokenRequestTooLarge,
            error_code::AUTHORIZATION_CODE_TOKEN_REQUEST_TOO_LARGE,
            "OID4VCI Authorization Code Token Request is too large",
        ),
    ];

    for (error, code, message) in cases {
        let public: IdentusError = error.into();
        assert_eq!(public.code(), code);
        assert_eq!(public.kind(), ErrorKind::InvalidInput);
        assert_eq!(public.capability(), Some(CAPABILITY));
        assert_eq!(error.to_string(), message);
        assert_eq!(public.public_message(), message);
        assert!(error.source().is_none());
        for private in [TOKEN_ENDPOINT, VERIFIER, CLIENT_ID, REDIRECT_URI, "a/b?c"] {
            assert!(!format!("{error:?}").contains(private));
            assert!(!public.to_string().contains(private));
        }
    }
}
