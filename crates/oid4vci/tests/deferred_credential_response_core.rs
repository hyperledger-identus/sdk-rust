use identus_core::{ErrorKind, IdentusError};
use identus_oid4vci::{
    CAPABILITY, CredentialOfferError, DeferredCredentialResponseCore,
    DeferredCredentialResponseLimits, error_code,
};

fn limits(
    max_json_bytes: usize,
    max_json_depth: usize,
    max_json_nodes: usize,
    max_response_members: usize,
    max_transaction_id_bytes: usize,
    max_interval_bytes: usize,
) -> DeferredCredentialResponseLimits {
    DeferredCredentialResponseLimits::new(
        max_json_bytes,
        max_json_depth,
        max_json_nodes,
        max_response_members,
        max_transaction_id_bytes,
        max_interval_bytes,
    )
    .expect("positive limits")
}

fn parse(json: &str) -> Result<DeferredCredentialResponseCore, CredentialOfferError> {
    DeferredCredentialResponseCore::parse(json, DeferredCredentialResponseLimits::default())
}

#[test]
fn defaults_and_positive_limits_are_explicit() {
    let defaults = DeferredCredentialResponseLimits::default();
    assert_eq!(defaults.max_json_bytes(), 32_768);
    assert_eq!(defaults.max_json_depth(), 16);
    assert_eq!(defaults.max_json_nodes(), 512);
    assert_eq!(defaults.max_response_members(), 16);
    assert_eq!(defaults.max_transaction_id_bytes(), 2_048);
    assert_eq!(defaults.max_interval_bytes(), 128);

    for invalid in [
        (0, 1, 1, 1, 1, 1),
        (1, 0, 1, 1, 1, 1),
        (1, 65, 1, 1, 1, 1),
        (1, 1, 0, 1, 1, 1),
        (1, 1, 1, 0, 1, 1),
        (1, 1, 1, 1, 0, 1),
        (1, 1, 1, 1, 1, 0),
    ] {
        assert_eq!(
            DeferredCredentialResponseLimits::new(
                invalid.0, invalid.1, invalid.2, invalid.3, invalid.4, invalid.5,
            ),
            Err(CredentialOfferError::InvalidDeferredCredentialResponseLimits)
        );
    }
}

#[test]
fn final_and_consumer_shaped_response_preserves_exact_values() {
    let json = r#"{"transaction_id":"8xLOxBtZp8","interval":3600}"#;
    let response = parse(json).expect("Final deferred response");
    assert_eq!(response.response_len(), json.len());
    assert_eq!(
        response.transaction_id().expose_sensitive_transaction_id(),
        "8xLOxBtZp8"
    );
    assert_eq!(response.interval().as_str(), "3600");

    let consumer =
        parse(r#"{"transaction_id":"opaque-handle","interval":5,"vendor":{"nested":true}}"#)
            .expect("consumer-shaped deferred response");
    assert_eq!(consumer.interval().as_str(), "5");
}

#[test]
fn positive_integer_fraction_and_exponent_forms_survive_exactly() {
    for interval in ["1", "3600", "0.0001", "1.2300", "1e3", "1E+03", "1e-999"] {
        let json = format!(r#"{{"transaction_id":"tx","interval":{interval}}}"#);
        let response = parse(&json).expect("positive JSON number");
        assert_eq!(response.interval().as_str(), interval);
    }
}

#[test]
fn zero_negative_and_non_number_intervals_fail_closed() {
    let invalid = [
        "0",
        "0.0",
        "0e10",
        "0.000e-10",
        "-0",
        "-0.1",
        "-1",
        "\"1\"",
        "true",
        "null",
        "[]",
        "{}",
        "+1",
        "01",
        "1.",
        "1e",
        "1e+",
    ];
    for interval in invalid {
        let json = format!(r#"{{"transaction_id":"tx","interval":{interval}}}"#);
        assert_eq!(
            parse(&json).expect_err("invalid interval"),
            CredentialOfferError::InvalidDeferredCredentialInterval,
            "wrong error for {interval}"
        );
    }
}

#[test]
fn required_types_and_branch_exclusivity_are_enforced() {
    for json in [
        r#"{}"#,
        r#"[]"#,
        r#"{"transaction_id":"tx"}"#,
        r#"{"interval":1}"#,
        r#"{"transaction_id":"","interval":1}"#,
        r#"{"transaction_id":1,"interval":1}"#,
    ] {
        assert!(parse(json).is_err(), "unexpectedly accepted {json}");
    }

    for json in [
        r#"{"transaction_id":"tx","interval":1,"credentials":[]}"#,
        r#"{"credentials":[],"transaction_id":"tx","interval":1}"#,
        r#"{"transaction_id":"tx","notification_id":"note","interval":1}"#,
    ] {
        assert_eq!(
            parse(json).expect_err("mixed response branches"),
            CredentialOfferError::DeferredCredentialResponseBranchConflict
        );
    }
}

#[test]
fn duplicate_malformed_and_trailing_json_fail_closed() {
    for json in [
        r#"{"transaction_id":"one","transaction\u005fid":"two","interval":1}"#,
        r#"{"transaction_id":"tx","interval":1,"future":{"du\u0070":1,"dup":2}}"#,
        r#"{"transaction_id":"tx","interval":1}[]"#,
        r#"{"transaction_id":"tx","interval":1,}"#,
        r#"{"transaction_id":"\uD800","interval":1}"#,
    ] {
        assert!(parse(json).is_err(), "unexpectedly accepted {json}");
    }
}

#[test]
fn independent_limits_are_enforced_at_exact_boundaries() {
    let json = r#"{"transaction_id":"é","interval":1e-9}"#;
    let exact = limits(json.len(), 1, 3, 2, "é".len(), "1e-9".len());
    let response = DeferredCredentialResponseCore::parse(json, exact).expect("exact limits");
    assert_eq!(response.interval().as_str(), "1e-9");

    let cases = [
        (
            json,
            limits(json.len() - 1, 1, 3, 2, 2, 4),
            CredentialOfferError::DeferredCredentialResponseTooLarge,
        ),
        (
            r#"{"transaction_id":"é","interval":1,"future":true}"#,
            limits(1_024, 1, 8, 2, 2, 4),
            CredentialOfferError::TooManyDeferredCredentialResponseMembers,
        ),
        (
            json,
            limits(1_024, 1, 3, 2, 1, 4),
            CredentialOfferError::DeferredTransactionIdTooLarge,
        ),
        (
            json,
            limits(1_024, 1, 3, 2, 2, 3),
            CredentialOfferError::DeferredCredentialIntervalTooLarge,
        ),
    ];
    for (input, policy, expected) in cases {
        assert_eq!(
            DeferredCredentialResponseCore::parse(input, policy).expect_err("independent limit"),
            expected
        );
    }

    let nested = r#"{"transaction_id":"tx","interval":1,"future":{"child":true}}"#;
    assert_eq!(
        DeferredCredentialResponseCore::parse(nested, limits(1_024, 1, 16, 4, 8, 8))
            .expect_err("depth limit"),
        CredentialOfferError::JsonTooDeep
    );
    assert_eq!(
        DeferredCredentialResponseCore::parse(nested, limits(1_024, 4, 3, 4, 8, 8))
            .expect_err("node limit"),
        CredentialOfferError::JsonTooManyNodes
    );
}

#[test]
fn debug_and_errors_redact_remote_values() {
    let transaction_canary = "TRANSACTION_CANARY_49ce";
    let interval_canary = "7.654321e99";
    let json =
        format!(r#"{{"transaction_id":"{transaction_canary}","interval":{interval_canary}}}"#);
    let response = parse(&json).expect("canary response");
    for rendered in [
        format!("{response:?}"),
        format!("{:?}", response.transaction_id()),
        format!("{:?}", response.interval()),
    ] {
        assert!(!rendered.contains(transaction_canary));
        assert!(!rendered.contains(interval_canary));
    }

    let error = parse(&format!(
        r#"{{"transaction_id":"{transaction_canary}","interval":-{interval_canary}}}"#
    ))
    .expect_err("negative canary");
    let rendered = format!("{error:?} {} {:?}", error, error.to_identus_error());
    assert!(!rendered.contains(transaction_canary));
    assert!(!rendered.contains(interval_canary));
}

#[test]
fn new_errors_bridge_to_static_codes() {
    let cases = [
        (
            CredentialOfferError::InvalidDeferredCredentialResponseLimits,
            error_code::INVALID_DEFERRED_CREDENTIAL_RESPONSE_LIMITS,
        ),
        (
            CredentialOfferError::DeferredCredentialResponseTooLarge,
            error_code::DEFERRED_CREDENTIAL_RESPONSE_TOO_LARGE,
        ),
        (
            CredentialOfferError::InvalidDeferredCredentialResponse,
            error_code::INVALID_DEFERRED_CREDENTIAL_RESPONSE,
        ),
        (
            CredentialOfferError::TooManyDeferredCredentialResponseMembers,
            error_code::TOO_MANY_DEFERRED_CREDENTIAL_RESPONSE_MEMBERS,
        ),
        (
            CredentialOfferError::InvalidDeferredTransactionId,
            error_code::INVALID_DEFERRED_TRANSACTION_ID,
        ),
        (
            CredentialOfferError::DeferredTransactionIdTooLarge,
            error_code::DEFERRED_TRANSACTION_ID_TOO_LARGE,
        ),
        (
            CredentialOfferError::InvalidDeferredCredentialInterval,
            error_code::INVALID_DEFERRED_CREDENTIAL_INTERVAL,
        ),
        (
            CredentialOfferError::DeferredCredentialIntervalTooLarge,
            error_code::DEFERRED_CREDENTIAL_INTERVAL_TOO_LARGE,
        ),
        (
            CredentialOfferError::DeferredCredentialResponseBranchConflict,
            error_code::DEFERRED_CREDENTIAL_RESPONSE_BRANCH_CONFLICT,
        ),
    ];

    for (error, code) in cases {
        let core: IdentusError = error.into();
        assert_eq!(core.code(), code);
        assert_eq!(core.kind(), ErrorKind::InvalidInput);
        assert_eq!(core.capability(), Some(CAPABILITY));
    }
}
