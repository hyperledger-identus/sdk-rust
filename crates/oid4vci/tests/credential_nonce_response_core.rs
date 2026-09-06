use identus_core::{ErrorKind, IdentusError};
use identus_oid4vci::{
    CAPABILITY, CredentialNonceResponseCore, CredentialNonceResponseLimits, CredentialOfferError,
    MAX_CONFIGURABLE_JSON_DEPTH, error_code,
};

fn parse(json: &str) -> Result<CredentialNonceResponseCore, CredentialOfferError> {
    CredentialNonceResponseCore::parse(json, CredentialNonceResponseLimits::default())
}

fn limits(nonce_bytes: usize) -> CredentialNonceResponseLimits {
    CredentialNonceResponseLimits::new(1_024, 16, 128, nonce_bytes).expect("positive limits")
}

#[test]
fn defaults_and_positive_limits_are_explicit() {
    let defaults = CredentialNonceResponseLimits::default();
    assert_eq!(defaults.max_json_bytes(), 16_384);
    assert_eq!(defaults.max_json_depth(), 16);
    assert_eq!(defaults.max_json_nodes(), 256);
    assert_eq!(defaults.max_nonce_bytes(), 4_096);

    let invalid = [
        CredentialNonceResponseLimits::new(0, 1, 1, 1),
        CredentialNonceResponseLimits::new(1, 0, 1, 1),
        CredentialNonceResponseLimits::new(1, MAX_CONFIGURABLE_JSON_DEPTH + 1, 1, 1),
        CredentialNonceResponseLimits::new(1, 1, 0, 1),
        CredentialNonceResponseLimits::new(1, 1, 1, 0),
    ];
    for result in invalid {
        assert_eq!(
            result,
            Err(CredentialOfferError::InvalidCredentialNonceResponseLimits)
        );
    }
}

#[test]
fn minimal_final_response_exposes_only_deliberate_values() {
    let json = r#"{"c_nonce":"wKI4LT17ac15ES9bw8ac4"}"#;
    let response = parse(json).expect("minimal Credential Nonce Response");

    assert_eq!(response.response_len(), json.len());
    assert_eq!(
        response.nonce().expose_sensitive_nonce(),
        "wKI4LT17ac15ES9bw8ac4"
    );
}

#[test]
fn nonce_is_an_exact_opaque_unicode_string() {
    let nonce = "挑戦-🔐-opaque";
    let encoded = serde_json::to_string(nonce).expect("nonce JSON");
    let json = format!(r#"{{"c_nonce":{encoded}}}"#);
    let response = parse(&json).expect("Unicode nonce");

    assert_eq!(response.nonce().expose_sensitive_nonce(), nonce);
}

#[test]
fn historical_and_future_extensions_are_checked_then_discarded() {
    let json = r#" {
        "c_nonce":"opaque",
        "c_nonce_expires_in":300,
        "future":{"nested":[true,null,{"number":1e999999}]}
    } "#;
    let response = parse(json).expect("bounded extensions");

    assert_eq!(response.response_len(), json.len());
    assert_eq!(response.nonce().expose_sensitive_nonce(), "opaque");
}

#[test]
fn required_nonce_shape_fails_closed() {
    let invalid = [
        r#"{}"#,
        r#"{"c_nonce":""}"#,
        r#"{"c_nonce":null}"#,
        r#"{"c_nonce":7}"#,
        r#"{"c_nonce":true}"#,
        r#"{"c_nonce":[]}"#,
        r#"{"c_nonce":{}}"#,
    ];
    for json in invalid {
        assert!(parse(json).is_err(), "unexpectedly accepted {json}");
    }
}

#[test]
fn aggregate_structure_and_nonce_limits_are_enforced() {
    let minimal = r#"{"c_nonce":"opaque"}"#;
    assert_eq!(
        CredentialNonceResponseCore::parse(
            minimal,
            CredentialNonceResponseLimits::new(minimal.len(), 16, 128, 6).expect("exact limits"),
        )
        .expect("exact aggregate and nonce bounds")
        .response_len(),
        minimal.len()
    );
    assert_eq!(
        CredentialNonceResponseCore::parse(
            minimal,
            CredentialNonceResponseLimits::new(minimal.len() - 1, 16, 128, 6)
                .expect("small aggregate limit"),
        )
        .expect_err("aggregate oversize"),
        CredentialOfferError::CredentialNonceResponseTooLarge
    );
    assert_eq!(
        CredentialNonceResponseCore::parse(minimal, limits(5)).expect_err("nonce oversize"),
        CredentialOfferError::CredentialNonceTooLarge
    );

    let nested = r#"{"c_nonce":"opaque","future":{"value":1}}"#;
    assert_eq!(
        CredentialNonceResponseCore::parse(
            nested,
            CredentialNonceResponseLimits::new(1_024, 1, 128, 64).expect("shallow limit"),
        )
        .expect_err("deep response"),
        CredentialOfferError::JsonTooDeep
    );
    assert_eq!(
        CredentialNonceResponseCore::parse(
            minimal,
            CredentialNonceResponseLimits::new(1_024, 16, 1, 64).expect("narrow node limit"),
        )
        .expect_err("wide response"),
        CredentialOfferError::JsonTooManyNodes
    );
}

#[test]
fn malformed_duplicate_nested_and_trailing_json_fail_closed() {
    let invalid = [
        r#"[]"#,
        r#"{"c_nonce":"one","c_n\u006fnce":"two"}"#,
        r#"{"c_nonce":"opaque","future":{"du\u0070":1,"dup":2}}"#,
        r#"{"c_nonce":"opaque"}[]"#,
        r#"{"c_nonce":"opaque","future":[}"#,
        r#"{"c_nonce":"\uD800"}"#,
    ];
    for json in invalid {
        assert!(parse(json).is_err(), "unexpectedly accepted {json}");
    }
}

#[test]
fn response_nonce_and_errors_keep_remote_content_redacted() {
    let nonce_canary = "NONCE_CANARY_35d8";
    let extension_canary = "EXTENSION_CANARY_21ae";
    let json = format!(r#"{{"c_nonce":"{nonce_canary}","future":"{extension_canary}"}}"#);
    let response = parse(&json).expect("canary Credential Nonce Response");

    for value in [format!("{response:?}"), format!("{:?}", response.nonce())] {
        assert!(!value.contains(nonce_canary));
        assert!(!value.contains(extension_canary));
    }

    let cases = [
        (
            CredentialOfferError::InvalidCredentialNonceResponseLimits,
            error_code::INVALID_CREDENTIAL_NONCE_RESPONSE_LIMITS,
        ),
        (
            CredentialOfferError::CredentialNonceResponseTooLarge,
            error_code::CREDENTIAL_NONCE_RESPONSE_TOO_LARGE,
        ),
        (
            CredentialOfferError::InvalidCredentialNonceResponse,
            error_code::INVALID_CREDENTIAL_NONCE_RESPONSE,
        ),
        (
            CredentialOfferError::InvalidCredentialNonce,
            error_code::INVALID_CREDENTIAL_NONCE,
        ),
        (
            CredentialOfferError::CredentialNonceTooLarge,
            error_code::CREDENTIAL_NONCE_TOO_LARGE,
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
            assert!(!value.contains(nonce_canary));
            assert!(!value.contains(extension_canary));
        }
    }
}
