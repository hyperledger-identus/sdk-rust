use identus_core::{ErrorKind, IdentusError};
use identus_oid4vci::{
    CAPABILITY, CredentialOfferError, MAX_CONFIGURABLE_JSON_DEPTH, TokenResponseCore,
    TokenResponseLimits, error_code,
};

fn parse(json: &str) -> Result<TokenResponseCore, CredentialOfferError> {
    TokenResponseCore::parse(json, TokenResponseLimits::default())
}

fn limits(
    access_token: usize,
    token_type: usize,
    refresh_token: usize,
    scope: usize,
) -> TokenResponseLimits {
    TokenResponseLimits::new(
        1_024,
        16,
        128,
        access_token,
        token_type,
        refresh_token,
        scope,
    )
    .expect("positive limits")
}

#[test]
fn defaults_and_positive_limits_are_explicit() {
    let defaults = TokenResponseLimits::default();
    assert_eq!(defaults.max_json_bytes(), 65_536);
    assert_eq!(defaults.max_json_depth(), 16);
    assert_eq!(defaults.max_json_nodes(), 1_024);
    assert_eq!(defaults.max_access_token_bytes(), 16_384);
    assert_eq!(defaults.max_token_type_bytes(), 256);
    assert_eq!(defaults.max_refresh_token_bytes(), 16_384);
    assert_eq!(defaults.max_scope_bytes(), 4_096);

    let invalid = [
        TokenResponseLimits::new(0, 1, 1, 1, 1, 1, 1),
        TokenResponseLimits::new(1, 0, 1, 1, 1, 1, 1),
        TokenResponseLimits::new(1, MAX_CONFIGURABLE_JSON_DEPTH + 1, 1, 1, 1, 1, 1),
        TokenResponseLimits::new(1, 1, 0, 1, 1, 1, 1),
        TokenResponseLimits::new(1, 1, 1, 0, 1, 1, 1),
        TokenResponseLimits::new(1, 1, 1, 1, 0, 1, 1),
        TokenResponseLimits::new(1, 1, 1, 1, 1, 0, 1),
        TokenResponseLimits::new(1, 1, 1, 1, 1, 1, 0),
    ];
    for result in invalid {
        assert_eq!(
            result,
            Err(CredentialOfferError::InvalidTokenResponseLimits)
        );
    }
}

#[test]
fn minimal_final_response_exposes_only_deliberate_core_values() {
    let json = r#"{"access_token":"SYNTHETIC_ACCESS_TOKEN","token_type":"Bearer"}"#;
    let response = parse(json).expect("minimal Token Response");

    assert_eq!(response.response_len(), json.len());
    assert_eq!(
        response.expose_sensitive_access_token(),
        "SYNTHETIC_ACCESS_TOKEN"
    );
    assert_eq!(response.token_type().as_str(), "Bearer");
    assert!(response.token_type().eq_ignore_ascii_case("bEaReR"));
    assert_eq!(response.expires_in_seconds(), None);
    assert!(!response.refresh_token_present());
    assert_eq!(response.expose_sensitive_refresh_token(), None);
    assert!(!response.scope_present());
    assert_eq!(response.expose_sensitive_scope(), None);
    assert!(!response.authorization_details_present());
}

#[test]
fn complete_core_preserves_optional_fields_and_ignores_extensions() {
    let json = r#"{
        "access_token":"token with visible ASCII!",
        "token_type":"bEaReR",
        "expires_in":18446744073709551615,
        "refresh_token":"refresh-token_1",
        "scope":"openid_credential profile.read",
        "authorization_details":[{"type":"openid_credential","credential_identifiers":["dataset-1"]}],
        "c_nonce":"HISTORICAL_NONCE",
        "c_nonce_expires_in":300,
        "future":{"arbitrary":1e999999,"nested":[true,null]}
    }"#;
    let response = parse(json).expect("complete Token Response core");

    assert_eq!(
        response.expose_sensitive_access_token(),
        "token with visible ASCII!"
    );
    assert_eq!(response.token_type().as_str(), "bEaReR");
    assert!(response.token_type().eq_ignore_ascii_case("Bearer"));
    assert_eq!(response.expires_in_seconds(), Some(u64::MAX));
    assert!(response.refresh_token_present());
    assert_eq!(
        response.expose_sensitive_refresh_token(),
        Some("refresh-token_1")
    );
    assert!(response.scope_present());
    assert_eq!(
        response.expose_sensitive_scope(),
        Some("openid_credential profile.read")
    );
    assert!(response.authorization_details_present());
}

#[test]
fn uri_reference_token_type_is_supported_without_selecting_policy() {
    let response = parse(r#"{"access_token":"token","token_type":"urn:example:token"}"#)
        .expect("URI-reference token type");
    assert_eq!(response.token_type().as_str(), "urn:example:token");
    assert!(!response.token_type().eq_ignore_ascii_case("Bearer"));
}

#[test]
fn required_fields_and_visible_token_grammar_fail_closed() {
    let invalid = [
        r#"{}"#,
        r#"{"access_token":"token"}"#,
        r#"{"token_type":"Bearer"}"#,
        r#"{"access_token":"","token_type":"Bearer"}"#,
        r#"{"access_token":"bad\nvalue","token_type":"Bearer"}"#,
        r#"{"access_token":"tökén","token_type":"Bearer"}"#,
        r#"{"access_token":7,"token_type":"Bearer"}"#,
        r#"{"access_token":"token","token_type":""}"#,
        r#"{"access_token":"token","token_type":"bad type"}"#,
        r#"{"access_token":"token","token_type":"urn:bad%escape"}"#,
        r#"{"access_token":"token","token_type":false}"#,
        r#"{"access_token":"token","token_type":"Bearer","refresh_token":""}"#,
        r#"{"access_token":"token","token_type":"Bearer","refresh_token":"bad\u0000value"}"#,
    ];
    for json in invalid {
        assert!(parse(json).is_err(), "unexpectedly accepted {json}");
    }
}

#[test]
fn expiry_requires_plain_non_negative_u64_json_integer() {
    let invalid = [
        "-1",
        "+1",
        "1.0",
        "1e3",
        "\"1\"",
        "null",
        "true",
        "18446744073709551616",
    ];
    for expires_in in invalid {
        let json = format!(
            r#"{{"access_token":"token","token_type":"Bearer","expires_in":{expires_in}}}"#
        );
        assert_eq!(
            parse(&json).expect_err("invalid expiry"),
            CredentialOfferError::InvalidTokenExpiresIn
        );
    }

    assert_eq!(
        parse(r#"{"access_token":"token","token_type":"Bearer","expires_in":0}"#)
            .expect("zero lifetime is syntactically valid")
            .expires_in_seconds(),
        Some(0)
    );
}

#[test]
fn scope_requires_nonempty_single_space_delimited_ascii_tokens() {
    for scope in [
        "",
        " leading",
        "trailing ",
        "two  spaces",
        "quoted\"scope",
        "slash\\scope",
        "ümlaut",
    ] {
        let encoded = serde_json::to_string(scope).expect("scope JSON");
        let json = format!(r#"{{"access_token":"token","token_type":"Bearer","scope":{encoded}}}"#);
        assert_eq!(
            parse(&json).expect_err("invalid scope"),
            CredentialOfferError::InvalidTokenScope
        );
    }
}

#[test]
fn aggregate_structure_and_independent_value_limits_are_enforced() {
    let minimal = r#"{"access_token":"token","token_type":"Bearer"}"#;
    assert_eq!(
        TokenResponseCore::parse(
            minimal,
            TokenResponseLimits::new(minimal.len(), 16, 128, 16, 16, 16, 16)
                .expect("exact byte limit"),
        )
        .expect("exact aggregate bound")
        .response_len(),
        minimal.len()
    );
    assert_eq!(
        TokenResponseCore::parse(
            minimal,
            TokenResponseLimits::new(minimal.len() - 1, 16, 128, 16, 16, 16, 16)
                .expect("small byte limit"),
        )
        .expect_err("aggregate oversize"),
        CredentialOfferError::TokenResponseTooLarge
    );

    let cases = [
        (
            r#"{"access_token":"token","token_type":"Bearer"}"#,
            limits(4, 16, 16, 16),
            CredentialOfferError::AccessTokenTooLarge,
        ),
        (
            r#"{"access_token":"token","token_type":"Bearer"}"#,
            limits(16, 5, 16, 16),
            CredentialOfferError::TokenTypeTooLarge,
        ),
        (
            r#"{"access_token":"token","token_type":"Bearer","refresh_token":"refresh"}"#,
            limits(16, 16, 6, 16),
            CredentialOfferError::RefreshTokenTooLarge,
        ),
        (
            r#"{"access_token":"token","token_type":"Bearer","scope":"scope-a scope-b"}"#,
            limits(16, 16, 16, 14),
            CredentialOfferError::TokenScopeTooLarge,
        ),
    ];
    for (json, limits, expected) in cases {
        assert_eq!(
            TokenResponseCore::parse(json, limits).expect_err("known value oversize"),
            expected
        );
    }

    let nested = r#"{"access_token":"token","token_type":"Bearer","future":{"value":1}}"#;
    assert_eq!(
        TokenResponseCore::parse(
            nested,
            TokenResponseLimits::new(1_024, 1, 128, 16, 16, 16, 16).expect("shallow limit"),
        )
        .expect_err("deep response"),
        CredentialOfferError::JsonTooDeep
    );
    assert_eq!(
        TokenResponseCore::parse(
            minimal,
            TokenResponseLimits::new(1_024, 16, 2, 16, 16, 16, 16).expect("narrow node limit"),
        )
        .expect_err("wide response"),
        CredentialOfferError::JsonTooManyNodes
    );
}

#[test]
fn malformed_duplicate_nested_and_trailing_json_fail_closed() {
    let invalid = [
        r#"[]"#,
        r#"{"access_token":"token","access_\u0074oken":"other","token_type":"Bearer"}"#,
        r#"{"access_token":"token","token_type":"Bearer","future":{"du\u0070":1,"dup":2}}"#,
        r#"{"access_token":"token","token_type":"Bearer"}[]"#,
        r#"{"access_token":"token","token_type":"Bearer","future":[}"#,
    ];
    for json in invalid {
        assert!(parse(json).is_err(), "unexpectedly accepted {json}");
    }
}

#[test]
fn response_and_new_errors_keep_all_diagnostics_redacted() {
    let access_canary = "ACCESS_TOKEN_CANARY_2c63";
    let refresh_canary = "REFRESH_TOKEN_CANARY_1a90";
    let scope_canary = "SCOPE_CANARY_6bf4";
    let details_canary = "DETAILS_CANARY_d749";
    let json = format!(
        r#"{{"access_token":"{access_canary}","token_type":"Bearer","refresh_token":"{refresh_canary}","scope":"{scope_canary}","authorization_details":{{"canary":"{details_canary}"}}}}"#
    );
    let response = parse(&json).expect("canary Token Response");

    for rendered in [
        format!("{response:?}"),
        format!("{:?}", response.token_type()),
    ] {
        for canary in [access_canary, refresh_canary, scope_canary, details_canary] {
            assert!(!rendered.contains(canary));
        }
    }

    let cases = [
        (
            CredentialOfferError::InvalidTokenResponseLimits,
            error_code::INVALID_TOKEN_RESPONSE_LIMITS,
        ),
        (
            CredentialOfferError::TokenResponseTooLarge,
            error_code::TOKEN_RESPONSE_TOO_LARGE,
        ),
        (
            CredentialOfferError::InvalidTokenResponse,
            error_code::INVALID_TOKEN_RESPONSE,
        ),
        (
            CredentialOfferError::InvalidAccessToken,
            error_code::INVALID_ACCESS_TOKEN,
        ),
        (
            CredentialOfferError::AccessTokenTooLarge,
            error_code::ACCESS_TOKEN_TOO_LARGE,
        ),
        (
            CredentialOfferError::InvalidTokenType,
            error_code::INVALID_TOKEN_TYPE,
        ),
        (
            CredentialOfferError::TokenTypeTooLarge,
            error_code::TOKEN_TYPE_TOO_LARGE,
        ),
        (
            CredentialOfferError::InvalidTokenExpiresIn,
            error_code::INVALID_TOKEN_EXPIRES_IN,
        ),
        (
            CredentialOfferError::InvalidRefreshToken,
            error_code::INVALID_REFRESH_TOKEN,
        ),
        (
            CredentialOfferError::RefreshTokenTooLarge,
            error_code::REFRESH_TOKEN_TOO_LARGE,
        ),
        (
            CredentialOfferError::InvalidTokenScope,
            error_code::INVALID_TOKEN_SCOPE,
        ),
        (
            CredentialOfferError::TokenScopeTooLarge,
            error_code::TOKEN_SCOPE_TOO_LARGE,
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
            for canary in [access_canary, refresh_canary, scope_canary, details_canary] {
                assert!(!rendered.contains(canary));
            }
        }
    }
}
