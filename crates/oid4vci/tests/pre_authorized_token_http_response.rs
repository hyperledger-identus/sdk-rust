use identus_core::{ErrorKind, IdentusError};
use identus_oid4vci::{
    AuthorizationServerMetadataCore, AuthorizationServerMetadataLimits, CAPABILITY,
    CredentialIssuerMetadata, CredentialIssuerMetadataLimits, CredentialOffer,
    CredentialOfferError, CredentialOfferGrantLimits, CredentialOfferLimits,
    CredentialOfferSemanticLimits, EmbeddedCredentialOffer, PRE_AUTHORIZED_CODE_GRANT_TYPE,
    PreAuthorizedTokenHttpResponseLimits, PreAuthorizedTokenRequest,
    PreAuthorizedTokenRequestLimits, PreAuthorizedTokenResponseOutcome, TokenEndpointErrorKind,
    TokenErrorResponseLimits, TokenResponseLimits, TransactionCodeInputLimits, error_code,
};

const ISSUER: &str = "https://credential-issuer.example";
const SERVER: &str = "https://authorization.example";
const TOKEN_ENDPOINT: &str = "https://authorization.example/token";
const CONFIGURATION_A: &str = "UniversityDegreeCredential";
const CONFIGURATION_B: &str = "EmployeeCredential";
const PRE_AUTHORIZED_CODE: &str = "PRE_AUTHORIZED_CODE_CANARY";
const TRANSACTION_CODE: &str = "TRANSACTION_CODE_CANARY";
const ACCESS_TOKEN: &str = "ACCESS_TOKEN_CANARY";
const SUCCESS_BODY: &str =
    r#"{"access_token":"ACCESS_TOKEN_CANARY","token_type":"Bearer","expires_in":3600}"#;
const ERROR_BODY: &str = r#"{"error":"invalid_grant","error_description":"expired code"}"#;

fn request(with_transaction_code: bool) -> PreAuthorizedTokenRequest {
    let tx_requirement = if with_transaction_code {
        r#", "tx_code":{"input_mode":"numeric","length":6}"#
    } else {
        ""
    };
    let offer_json = format!(
        r#"{{"credential_issuer":"{ISSUER}","credential_configuration_ids":["{CONFIGURATION_A}","{CONFIGURATION_B}"],"grants":{{"{PRE_AUTHORIZED_CODE_GRANT_TYPE}":{{"pre-authorized_code":"{PRE_AUTHORIZED_CODE}","authorization_server":"{SERVER}"{tx_requirement}}}}}}}"#
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
        r#"{{"credential_issuer":"{ISSUER}","authorization_servers":["{SERVER}","https://authorization-backup.example"],"credential_endpoint":"{ISSUER}/credential","credential_configurations_supported":{{"{CONFIGURATION_A}":{{"format":"dc+sd-jwt"}},"{CONFIGURATION_B}":{{"format":"jwt_vc_json"}}}}}}"#
    );
    let issuer_metadata = CredentialIssuerMetadata::parse(
        &issuer_metadata_json,
        ISSUER,
        CredentialIssuerMetadataLimits::default(),
    )
    .expect("issuer metadata");
    let server_metadata_json = format!(
        r#"{{"issuer":"{SERVER}","token_endpoint":"{TOKEN_ENDPOINT}","grant_types_supported":["{PRE_AUTHORIZED_CODE_GRANT_TYPE}"]}}"#
    );
    let server_metadata = AuthorizationServerMetadataCore::parse(
        &server_metadata_json,
        SERVER,
        AuthorizationServerMetadataLimits::default(),
    )
    .expect("server metadata");

    offer
        .try_with_metadata(issuer_metadata)
        .expect("matched metadata")
        .try_with_pre_authorized_server(server_metadata)
        .expect("server bound")
        .try_with_transaction_code_input(
            with_transaction_code.then(|| TRANSACTION_CODE.to_owned()),
            TransactionCodeInputLimits::default(),
        )
        .expect("transaction input")
        .try_into_pre_authorized_token_request(PreAuthorizedTokenRequestLimits::default())
        .expect("token request")
}

fn bind(
    request: PreAuthorizedTokenRequest,
    status: u16,
    content_type: &str,
    cache_control: &str,
    pragma: &str,
    body: &str,
) -> Result<PreAuthorizedTokenResponseOutcome, CredentialOfferError> {
    request.try_bind_response(
        status,
        content_type,
        cache_control,
        pragma,
        body,
        PreAuthorizedTokenHttpResponseLimits::default(),
    )
}

#[test]
fn defaults_and_positive_header_limits_are_explicit() {
    let defaults = PreAuthorizedTokenHttpResponseLimits::default();
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
        PreAuthorizedTokenHttpResponseLimits::new(
            TokenResponseLimits::default(),
            TokenErrorResponseLimits::default(),
            0,
            1,
            1,
        ),
        PreAuthorizedTokenHttpResponseLimits::new(
            TokenResponseLimits::default(),
            TokenErrorResponseLimits::default(),
            1,
            0,
            1,
        ),
        PreAuthorizedTokenHttpResponseLimits::new(
            TokenResponseLimits::default(),
            TokenErrorResponseLimits::default(),
            1,
            1,
            0,
        ),
    ] {
        assert_eq!(
            limits,
            Err(CredentialOfferError::InvalidPreAuthorizedTokenHttpResponseLimits)
        );
    }
}

#[test]
fn request_and_success_preserve_exact_non_secret_lineage() {
    let request = request(true);
    assert_eq!(
        request
            .credential_issuer_metadata()
            .credential_issuer()
            .as_str(),
        ISSUER
    );
    assert_eq!(
        request.authorization_server_metadata().issuer().as_str(),
        SERVER
    );
    assert_eq!(request.offered_credential_configurations().len(), 2);
    assert_eq!(
        request.offered_credential_configurations()[0].as_str(),
        CONFIGURATION_A
    );

    let PreAuthorizedTokenResponseOutcome::Success(bound) = bind(
        request,
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
        SERVER
    );
    let configurations = bound.lineage().offered_credential_configurations();
    assert_eq!(configurations.len(), 2);
    assert_eq!(configurations[0].as_str(), CONFIGURATION_A);
    assert_eq!(configurations[1].as_str(), CONFIGURATION_B);
    assert!(bound.lineage().transaction_code_present());
}

#[test]
fn binds_pre_authorized_error_semantics_to_the_same_lineage() {
    for (body, expected) in [
        (
            r#"{"error":"invalid_request"}"#,
            TokenEndpointErrorKind::InvalidRequest,
        ),
        (ERROR_BODY, TokenEndpointErrorKind::InvalidGrant),
    ] {
        let PreAuthorizedTokenResponseOutcome::Error(bound) = bind(
            request(false),
            400,
            "application/json",
            "no-store",
            "no-cache",
            body,
        )
        .expect("bound error") else {
            panic!("expected error branch");
        };
        assert_eq!(bound.response().error_kind(), expected);
        assert_eq!(
            bound.lineage().offered_credential_configurations()[1].as_str(),
            CONFIGURATION_B
        );
        assert!(!bound.lineage().transaction_code_present());
    }
}

#[test]
fn status_is_checked_first_and_selects_the_only_body_parser() {
    for status in [0, 199, 201, 399, 401, 500, 600, u16::MAX] {
        assert_eq!(
            bind(request(false), status, "bad", "bad", "bad", "BODY_CANARY")
                .expect_err("unsupported status"),
            CredentialOfferError::InvalidPreAuthorizedTokenHttpStatus
        );
    }
    assert_eq!(
        bind(
            request(false),
            200,
            "application/json",
            "no-store",
            "no-cache",
            ERROR_BODY,
        )
        .expect_err("error body cannot override success"),
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
        .expect_err("success body cannot override error"),
        CredentialOfferError::InvalidTokenErrorResponse
    );
}

#[test]
fn headers_interoperate_and_fail_in_deterministic_order() {
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
            CredentialOfferError::InvalidPreAuthorizedTokenContentType,
        ),
        (
            "application/json",
            "no-store=value",
            "broken@pragma",
            CredentialOfferError::InvalidPreAuthorizedTokenCacheControl,
        ),
        (
            "application/json",
            "no-store",
            "no-cache=value",
            CredentialOfferError::InvalidPreAuthorizedTokenPragma,
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
                "BODY_CANARY",
            )
            .expect_err("invalid header"),
            expected
        );
    }
}

#[test]
fn exact_header_and_body_bounds_are_inclusive_and_specific() {
    let success_defaults = TokenResponseLimits::default();
    let success_limits = TokenResponseLimits::new(
        SUCCESS_BODY.len(),
        success_defaults.max_json_depth(),
        success_defaults.max_json_nodes(),
        success_defaults.max_access_token_bytes(),
        success_defaults.max_token_type_bytes(),
        success_defaults.max_refresh_token_bytes(),
        success_defaults.max_scope_bytes(),
    )
    .expect("success limits");
    let exact = PreAuthorizedTokenHttpResponseLimits::new(
        success_limits,
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
    let error_limits = TokenErrorResponseLimits::new(
        ERROR_BODY.len(),
        error_defaults.max_json_depth(),
        error_defaults.max_json_nodes(),
        error_defaults.max_error_code_bytes(),
        error_defaults.max_error_description_bytes(),
        error_defaults.max_error_uri_bytes(),
    )
    .expect("error limits");
    let exact = PreAuthorizedTokenHttpResponseLimits::new(
        TokenResponseLimits::default(),
        error_limits,
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
            CredentialOfferError::PreAuthorizedTokenContentTypeTooLarge,
        ),
        (
            64,
            "no-store".len() - 1,
            64,
            CredentialOfferError::PreAuthorizedTokenCacheControlTooLarge,
        ),
        (
            64,
            64,
            "no-cache".len() - 1,
            CredentialOfferError::PreAuthorizedTokenPragmaTooLarge,
        ),
    ];
    for (content_type_limit, cache_limit, pragma_limit, expected) in header_cases {
        let limits = PreAuthorizedTokenHttpResponseLimits::new(
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

    let short = TokenResponseLimits::new(
        SUCCESS_BODY.len() - 1,
        success_defaults.max_json_depth(),
        success_defaults.max_json_nodes(),
        success_defaults.max_access_token_bytes(),
        success_defaults.max_token_type_bytes(),
        success_defaults.max_refresh_token_bytes(),
        success_defaults.max_scope_bytes(),
    )
    .expect("short body limit");
    let limits = PreAuthorizedTokenHttpResponseLimits::new(
        short,
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
            .expect_err("oversized body"),
        CredentialOfferError::TokenResponseTooLarge
    );
}

#[test]
fn outcomes_and_static_errors_are_redacted() {
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
        PRE_AUTHORIZED_CODE,
        TRANSACTION_CODE,
        TOKEN_ENDPOINT,
        ISSUER,
        SERVER,
        CONFIGURATION_A,
    ] {
        assert!(!debug.contains(private));
    }

    let cases = [
        (
            CredentialOfferError::InvalidPreAuthorizedTokenHttpResponseLimits,
            error_code::INVALID_PRE_AUTHORIZED_TOKEN_HTTP_RESPONSE_LIMITS,
            "OID4VCI Pre-Authorized Token HTTP response limits are invalid",
        ),
        (
            CredentialOfferError::InvalidPreAuthorizedTokenHttpStatus,
            error_code::INVALID_PRE_AUTHORIZED_TOKEN_HTTP_STATUS,
            "OID4VCI Pre-Authorized Token HTTP status is invalid",
        ),
        (
            CredentialOfferError::PreAuthorizedTokenContentTypeTooLarge,
            error_code::PRE_AUTHORIZED_TOKEN_CONTENT_TYPE_TOO_LARGE,
            "OID4VCI Pre-Authorized Token Content-Type is too large",
        ),
        (
            CredentialOfferError::InvalidPreAuthorizedTokenContentType,
            error_code::INVALID_PRE_AUTHORIZED_TOKEN_CONTENT_TYPE,
            "OID4VCI Pre-Authorized Token Content-Type is invalid",
        ),
        (
            CredentialOfferError::PreAuthorizedTokenCacheControlTooLarge,
            error_code::PRE_AUTHORIZED_TOKEN_CACHE_CONTROL_TOO_LARGE,
            "OID4VCI Pre-Authorized Token Cache-Control is too large",
        ),
        (
            CredentialOfferError::InvalidPreAuthorizedTokenCacheControl,
            error_code::INVALID_PRE_AUTHORIZED_TOKEN_CACHE_CONTROL,
            "OID4VCI Pre-Authorized Token Cache-Control is invalid",
        ),
        (
            CredentialOfferError::PreAuthorizedTokenPragmaTooLarge,
            error_code::PRE_AUTHORIZED_TOKEN_PRAGMA_TOO_LARGE,
            "OID4VCI Pre-Authorized Token Pragma is too large",
        ),
        (
            CredentialOfferError::InvalidPreAuthorizedTokenPragma,
            error_code::INVALID_PRE_AUTHORIZED_TOKEN_PRAGMA,
            "OID4VCI Pre-Authorized Token Pragma is invalid",
        ),
    ];
    for (error, code, message) in cases {
        let core: IdentusError = error.into();
        assert_eq!(core.code(), code);
        assert_eq!(core.kind(), ErrorKind::InvalidInput);
        assert_eq!(core.capability(), Some(CAPABILITY));
        assert_eq!(error.to_string(), message);
        for rendered in [
            format!("{error}"),
            format!("{error:?}"),
            format!("{core}"),
            format!("{core:?}"),
        ] {
            for private in [PRE_AUTHORIZED_CODE, TRANSACTION_CODE, ACCESS_TOKEN, ISSUER] {
                assert!(!rendered.contains(private));
            }
        }
    }
}
