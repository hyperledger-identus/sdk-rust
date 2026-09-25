use std::error::Error as _;

use identus_core::{ErrorKind, IdentusError};
use identus_oid4vci::{
    AuthorizationCodeTokenHttpResponseLimits, AuthorizationCodeTokenRequest,
    AuthorizationCodeTokenRequestLimits, AuthorizationCodeTokenResponseOutcome,
    AuthorizationRequestInputLimits, AuthorizationRequestLimits,
    AuthorizationResponseIssuerIdentification, AuthorizationResponseLimits,
    AuthorizationResponseOutcome, AuthorizationServerMetadataCore,
    AuthorizationServerMetadataLimits, CAPABILITY, CredentialIssuerMetadata,
    CredentialIssuerMetadataLimits, CredentialOffer, CredentialOfferError,
    CredentialOfferGrantLimits, CredentialOfferLimits, CredentialOfferSemanticLimits,
    EmbeddedCredentialOffer, TokenEndpointErrorKind, TokenErrorResponseLimits, TokenResponseLimits,
    error_code,
};

const ISSUER: &str = "https://credential-issuer.example";
const CONFIGURATION: &str = "UniversityDegreeCredential";
const TOKEN_ENDPOINT: &str = "https://authorization.example/token";
const STATE: &str = "state-value";
const VERIFIER: &str = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
const ACCESS_TOKEN: &str = "access-token-canary";
const SUCCESS_BODY: &str =
    r#"{"access_token":"access-token-canary","token_type":"Bearer","expires_in":3600}"#;
const ERROR_BODY: &str = r#"{"error":"invalid_grant","error_description":"expired code"}"#;

fn request(evidence: bool) -> AuthorizationCodeTokenRequest {
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
    let issuer_metadata_json = format!(
        r#"{{"credential_issuer":"{ISSUER}","credential_endpoint":"{ISSUER}/credential","credential_configurations_supported":{{"{CONFIGURATION}":{{"format":"dc+sd-jwt"}}}}}}"#
    );
    let issuer_metadata = CredentialIssuerMetadata::parse(
        &issuer_metadata_json,
        ISSUER,
        CredentialIssuerMetadataLimits::default(),
    )
    .expect("issuer metadata");
    let issuer_flag = if evidence {
        r#", "authorization_response_iss_parameter_supported":true"#
    } else {
        ""
    };
    let server_metadata_json = format!(
        r#"{{"issuer":"{ISSUER}","authorization_endpoint":"{ISSUER}/authorize","token_endpoint":"{TOKEN_ENDPOINT}"{issuer_flag}}}"#
    );
    let server_metadata = AuthorizationServerMetadataCore::parse(
        &server_metadata_json,
        ISSUER,
        AuthorizationServerMetadataLimits::default(),
    )
    .expect("server metadata");
    let authorization_request = offer
        .try_with_metadata(issuer_metadata)
        .expect("matched metadata")
        .try_with_authorization_code_server(server_metadata)
        .expect("server bound")
        .try_with_authorization_request_input(
            0,
            "wallet-client",
            "https://wallet.example/callback",
            STATE,
            VERIFIER,
            AuthorizationRequestInputLimits::default(),
        )
        .expect("request input")
        .try_into_authorization_request(AuthorizationRequestLimits::default())
        .expect("authorization request");
    let issuer = if evidence {
        "&iss=https%3A%2F%2Fcredential-issuer.example"
    } else {
        ""
    };
    let AuthorizationResponseOutcome::Authorized(code) = authorization_request
        .try_into_authorization_response(
            &format!("code=authorization-code&state={STATE}{issuer}"),
            AuthorizationResponseLimits::default(),
        )
        .expect("authorization response")
    else {
        panic!("expected authorization code");
    };
    code.try_into_public_client_token_request(AuthorizationCodeTokenRequestLimits::default())
        .expect("token request")
}

fn bind(
    request: AuthorizationCodeTokenRequest,
    status: u16,
    content_type: &str,
    cache_control: &str,
    pragma: &str,
    body: &str,
) -> Result<AuthorizationCodeTokenResponseOutcome, CredentialOfferError> {
    request.try_bind_response(
        status,
        content_type,
        cache_control,
        pragma,
        body,
        AuthorizationCodeTokenHttpResponseLimits::default(),
    )
}

#[test]
fn defaults_and_positive_header_limits_are_explicit() {
    let defaults = AuthorizationCodeTokenHttpResponseLimits::default();
    assert_eq!(
        defaults.success_response_limits(),
        TokenResponseLimits::default()
    );
    assert_eq!(
        defaults.error_response_limits(),
        TokenErrorResponseLimits::default()
    );
    assert_eq!(defaults.max_content_type_bytes(), 1_024);
    assert_eq!(defaults.max_cache_control_bytes(), 1_024);
    assert_eq!(defaults.max_pragma_bytes(), 1_024);

    for limits in [
        AuthorizationCodeTokenHttpResponseLimits::new(
            TokenResponseLimits::default(),
            TokenErrorResponseLimits::default(),
            0,
            1,
            1,
        ),
        AuthorizationCodeTokenHttpResponseLimits::new(
            TokenResponseLimits::default(),
            TokenErrorResponseLimits::default(),
            1,
            0,
            1,
        ),
        AuthorizationCodeTokenHttpResponseLimits::new(
            TokenResponseLimits::default(),
            TokenErrorResponseLimits::default(),
            1,
            1,
            0,
        ),
    ] {
        assert_eq!(
            limits,
            Err(CredentialOfferError::InvalidAuthorizationCodeTokenHttpResponseLimits)
        );
    }
}

#[test]
fn binds_success_and_preserves_exact_lineage_and_evidence() {
    let AuthorizationCodeTokenResponseOutcome::Success(bound) = bind(
        request(true),
        200,
        "Application/JSON; charset=utf-8",
        "private, NO-STORE",
        "NO-CACHE",
        SUCCESS_BODY,
    )
    .expect("bound success") else {
        panic!("expected success branch");
    };

    assert_eq!(
        bound.response().expose_sensitive_access_token(),
        ACCESS_TOKEN
    );
    assert_eq!(bound.response().token_type().as_str(), "Bearer");
    assert_eq!(
        bound
            .lineage()
            .credential_issuer_metadata()
            .credential_issuer()
            .as_str(),
        ISSUER
    );
    assert_eq!(
        bound
            .lineage()
            .authorization_server_metadata()
            .issuer()
            .as_str(),
        ISSUER
    );
    assert_eq!(
        bound.lineage().selected_credential_configuration().as_str(),
        CONFIGURATION
    );
    assert_eq!(
        bound.lineage().issuer_identification(),
        AuthorizationResponseIssuerIdentification::VerifiedRfc9207
    );
}

#[test]
fn binds_400_and_exact_invalid_client_401_errors() {
    let AuthorizationCodeTokenResponseOutcome::Error(error) = bind(
        request(false),
        400,
        "application/json",
        "no-store",
        "no-cache",
        ERROR_BODY,
    )
    .expect("400 error") else {
        panic!("expected error branch");
    };
    assert_eq!(
        error.response().error_kind(),
        TokenEndpointErrorKind::InvalidGrant
    );
    assert_eq!(
        error.lineage().issuer_identification(),
        AuthorizationResponseIssuerIdentification::NotAdvertised
    );

    let AuthorizationCodeTokenResponseOutcome::Error(error) = bind(
        request(false),
        401,
        "application/json",
        "no-store",
        "no-cache",
        r#"{"error":"invalid_client"}"#,
    )
    .expect("401 invalid_client") else {
        panic!("expected error branch");
    };
    assert_eq!(
        error.response().error_kind(),
        TokenEndpointErrorKind::InvalidClient
    );
}

#[test]
fn status_is_checked_before_headers_and_body_and_401_is_specific() {
    for status in [0, 199, 201, 399, 402, 500, 600, u16::MAX] {
        assert_eq!(
            bind(request(false), status, "bad", "bad", "bad", "BODY_CANARY")
                .expect_err("unsupported status"),
            CredentialOfferError::InvalidAuthorizationCodeTokenHttpStatus
        );
    }

    assert_eq!(
        bind(
            request(false),
            401,
            "application/json",
            "no-store",
            "no-cache",
            ERROR_BODY,
        )
        .expect_err("401 non-invalid_client"),
        CredentialOfferError::AuthorizationCodeTokenHttpStatusErrorMismatch
    );
}

#[test]
fn status_selected_parser_rejects_the_opposite_body_shape() {
    assert_eq!(
        bind(
            request(false),
            200,
            "application/json",
            "no-store",
            "no-cache",
            ERROR_BODY,
        )
        .expect_err("error body cannot override success status"),
        CredentialOfferError::InvalidTokenResponse
    );
    assert_eq!(
        bind(
            request(false),
            400,
            "application/json",
            "no-store",
            "no-cache",
            SUCCESS_BODY,
        )
        .expect_err("success body cannot override error status"),
        CredentialOfferError::InvalidTokenErrorResponse
    );
}

#[test]
fn header_grammars_interoperate_and_fail_in_deterministic_order() {
    bind(
        request(false),
        200,
        "\tapplication/json ; charset=\"utf-8\";profile=wallet",
        "max-age=0, private=\"field,a\", No-StOrE",
        "ext=token, No-CaChE",
        SUCCESS_BODY,
    )
    .expect("valid headers");

    let cases = [
        (
            "text/json",
            "broken@cache",
            "broken@pragma",
            CredentialOfferError::InvalidAuthorizationCodeTokenContentType,
        ),
        (
            "application/json",
            "no-store=value",
            "broken@pragma",
            CredentialOfferError::InvalidAuthorizationCodeTokenCacheControl,
        ),
        (
            "application/json",
            "no-store",
            "no-cache=value",
            CredentialOfferError::InvalidAuthorizationCodeTokenPragma,
        ),
    ];
    for (content_type, cache_control, pragma, expected) in cases {
        assert_eq!(
            bind(
                request(false),
                200,
                content_type,
                cache_control,
                pragma,
                "BODY_CANARY"
            )
            .expect_err("invalid header"),
            expected
        );
    }
}

#[test]
fn exact_header_and_body_bounds_are_inclusive_and_specific() {
    let success_defaults = TokenResponseLimits::default();
    let exact_success = TokenResponseLimits::new(
        SUCCESS_BODY.len(),
        success_defaults.max_json_depth(),
        success_defaults.max_json_nodes(),
        success_defaults.max_access_token_bytes(),
        success_defaults.max_token_type_bytes(),
        success_defaults.max_refresh_token_bytes(),
        success_defaults.max_scope_bytes(),
    )
    .expect("success body limit");
    let exact = AuthorizationCodeTokenHttpResponseLimits::new(
        exact_success,
        TokenErrorResponseLimits::default(),
        "application/json".len(),
        "no-store".len(),
        "no-cache".len(),
    )
    .expect("exact limits");
    request(false)
        .try_bind_response(
            200,
            "application/json",
            "no-store",
            "no-cache",
            SUCCESS_BODY,
            exact,
        )
        .expect("exact bounds");

    let error_defaults = TokenErrorResponseLimits::default();
    let exact_error = TokenErrorResponseLimits::new(
        ERROR_BODY.len(),
        error_defaults.max_json_depth(),
        error_defaults.max_json_nodes(),
        error_defaults.max_error_code_bytes(),
        error_defaults.max_error_description_bytes(),
        error_defaults.max_error_uri_bytes(),
    )
    .expect("error body limit");
    let exact = AuthorizationCodeTokenHttpResponseLimits::new(
        TokenResponseLimits::default(),
        exact_error,
        "application/json".len(),
        "no-store".len(),
        "no-cache".len(),
    )
    .expect("exact error limits");
    request(false)
        .try_bind_response(
            400,
            "application/json",
            "no-store",
            "no-cache",
            ERROR_BODY,
            exact,
        )
        .expect("exact error bounds");

    let header_cases = [
        (
            "application/json".len() - 1,
            64,
            64,
            CredentialOfferError::AuthorizationCodeTokenContentTypeTooLarge,
        ),
        (
            64,
            "no-store".len() - 1,
            64,
            CredentialOfferError::AuthorizationCodeTokenCacheControlTooLarge,
        ),
        (
            64,
            64,
            "no-cache".len() - 1,
            CredentialOfferError::AuthorizationCodeTokenPragmaTooLarge,
        ),
    ];
    for (content_type_limit, cache_limit, pragma_limit, expected) in header_cases {
        let limits = AuthorizationCodeTokenHttpResponseLimits::new(
            TokenResponseLimits::default(),
            TokenErrorResponseLimits::default(),
            content_type_limit,
            cache_limit,
            pragma_limit,
        )
        .expect("positive limits");
        assert_eq!(
            request(false)
                .try_bind_response(
                    200,
                    "application/json",
                    "no-store",
                    "no-cache",
                    "BODY_CANARY",
                    limits,
                )
                .expect_err("oversized header"),
            expected
        );
    }

    let short_success = TokenResponseLimits::new(
        SUCCESS_BODY.len() - 1,
        success_defaults.max_json_depth(),
        success_defaults.max_json_nodes(),
        success_defaults.max_access_token_bytes(),
        success_defaults.max_token_type_bytes(),
        success_defaults.max_refresh_token_bytes(),
        success_defaults.max_scope_bytes(),
    )
    .expect("short success limit");
    let limits = AuthorizationCodeTokenHttpResponseLimits::new(
        short_success,
        TokenErrorResponseLimits::default(),
        64,
        64,
        64,
    )
    .expect("limits");
    assert_eq!(
        request(false)
            .try_bind_response(
                200,
                "application/json",
                "no-store",
                "no-cache",
                SUCCESS_BODY,
                limits,
            )
            .expect_err("oversized success body"),
        CredentialOfferError::TokenResponseTooLarge
    );

    let short_error = TokenErrorResponseLimits::new(
        ERROR_BODY.len() - 1,
        error_defaults.max_json_depth(),
        error_defaults.max_json_nodes(),
        error_defaults.max_error_code_bytes(),
        error_defaults.max_error_description_bytes(),
        error_defaults.max_error_uri_bytes(),
    )
    .expect("short error limit");
    let limits = AuthorizationCodeTokenHttpResponseLimits::new(
        TokenResponseLimits::default(),
        short_error,
        64,
        64,
        64,
    )
    .expect("limits");
    assert_eq!(
        request(false)
            .try_bind_response(
                400,
                "application/json",
                "no-store",
                "no-cache",
                ERROR_BODY,
                limits,
            )
            .expect_err("oversized error body"),
        CredentialOfferError::TokenErrorResponseTooLarge
    );
}

#[test]
fn outcome_debug_and_errors_are_redacted() {
    let outcome = bind(
        request(true),
        200,
        "application/json",
        "no-store",
        "no-cache",
        SUCCESS_BODY,
    )
    .expect("success");
    let debug = format!("{outcome:?}");
    for private in [
        ACCESS_TOKEN,
        VERIFIER,
        TOKEN_ENDPOINT,
        ISSUER,
        CONFIGURATION,
    ] {
        assert!(!debug.contains(private));
    }

    let cases = [
        (
            CredentialOfferError::InvalidAuthorizationCodeTokenHttpResponseLimits,
            error_code::INVALID_AUTHORIZATION_CODE_TOKEN_HTTP_RESPONSE_LIMITS,
            "OID4VCI Authorization Code Token HTTP response limits are invalid",
        ),
        (
            CredentialOfferError::InvalidAuthorizationCodeTokenHttpStatus,
            error_code::INVALID_AUTHORIZATION_CODE_TOKEN_HTTP_STATUS,
            "OID4VCI Authorization Code Token HTTP status is invalid",
        ),
        (
            CredentialOfferError::AuthorizationCodeTokenHttpStatusErrorMismatch,
            error_code::AUTHORIZATION_CODE_TOKEN_HTTP_STATUS_ERROR_MISMATCH,
            "OID4VCI Authorization Code Token HTTP status and error do not match",
        ),
        (
            CredentialOfferError::AuthorizationCodeTokenContentTypeTooLarge,
            error_code::AUTHORIZATION_CODE_TOKEN_CONTENT_TYPE_TOO_LARGE,
            "OID4VCI Authorization Code Token Content-Type is too large",
        ),
        (
            CredentialOfferError::InvalidAuthorizationCodeTokenContentType,
            error_code::INVALID_AUTHORIZATION_CODE_TOKEN_CONTENT_TYPE,
            "OID4VCI Authorization Code Token Content-Type is invalid",
        ),
        (
            CredentialOfferError::AuthorizationCodeTokenCacheControlTooLarge,
            error_code::AUTHORIZATION_CODE_TOKEN_CACHE_CONTROL_TOO_LARGE,
            "OID4VCI Authorization Code Token Cache-Control is too large",
        ),
        (
            CredentialOfferError::InvalidAuthorizationCodeTokenCacheControl,
            error_code::INVALID_AUTHORIZATION_CODE_TOKEN_CACHE_CONTROL,
            "OID4VCI Authorization Code Token Cache-Control is invalid",
        ),
        (
            CredentialOfferError::AuthorizationCodeTokenPragmaTooLarge,
            error_code::AUTHORIZATION_CODE_TOKEN_PRAGMA_TOO_LARGE,
            "OID4VCI Authorization Code Token Pragma is too large",
        ),
        (
            CredentialOfferError::InvalidAuthorizationCodeTokenPragma,
            error_code::INVALID_AUTHORIZATION_CODE_TOKEN_PRAGMA,
            "OID4VCI Authorization Code Token Pragma is invalid",
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
        for private in [ACCESS_TOKEN, VERIFIER, TOKEN_ENDPOINT, ISSUER, "no-store"] {
            assert!(!format!("{error:?}").contains(private));
            assert!(!public.to_string().contains(private));
        }
    }
}
