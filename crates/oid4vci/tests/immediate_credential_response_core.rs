use identus_core::{ErrorKind, IdentusError};
use identus_oid4vci::{
    CAPABILITY, CredentialOfferError, CredentialValueKind, ImmediateCredentialResponseCore,
    ImmediateCredentialResponseLimits, MAX_CONFIGURABLE_JSON_DEPTH, error_code,
};

fn parse(json: &str) -> Result<ImmediateCredentialResponseCore, CredentialOfferError> {
    ImmediateCredentialResponseCore::parse(json, ImmediateCredentialResponseLimits::default())
}

#[allow(clippy::too_many_arguments)]
fn limits(
    json_bytes: usize,
    depth: usize,
    nodes: usize,
    response_members: usize,
    credentials: usize,
    credential_members: usize,
    credential_bytes: usize,
    total_credential_bytes: usize,
    notification_id_bytes: usize,
) -> ImmediateCredentialResponseLimits {
    ImmediateCredentialResponseLimits::new(
        json_bytes,
        depth,
        nodes,
        response_members,
        credentials,
        credential_members,
        credential_bytes,
        total_credential_bytes,
        notification_id_bytes,
    )
    .expect("positive limits")
}

#[test]
fn defaults_and_positive_limits_are_explicit() {
    let defaults = ImmediateCredentialResponseLimits::default();
    assert_eq!(defaults.max_json_bytes(), 1_048_576);
    assert_eq!(defaults.max_json_depth(), 32);
    assert_eq!(defaults.max_json_nodes(), 16_384);
    assert_eq!(defaults.max_response_members(), 32);
    assert_eq!(defaults.max_credentials(), 64);
    assert_eq!(defaults.max_credential_members(), 32);
    assert_eq!(defaults.max_credential_bytes(), 262_144);
    assert_eq!(defaults.max_total_credential_bytes(), 786_432);
    assert_eq!(defaults.max_notification_id_bytes(), 4_096);

    let invalid = [
        ImmediateCredentialResponseLimits::new(0, 1, 1, 1, 1, 1, 1, 1, 1),
        ImmediateCredentialResponseLimits::new(1, 0, 1, 1, 1, 1, 1, 1, 1),
        ImmediateCredentialResponseLimits::new(
            1,
            MAX_CONFIGURABLE_JSON_DEPTH + 1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
        ),
        ImmediateCredentialResponseLimits::new(1, 1, 0, 1, 1, 1, 1, 1, 1),
        ImmediateCredentialResponseLimits::new(1, 1, 1, 0, 1, 1, 1, 1, 1),
        ImmediateCredentialResponseLimits::new(1, 1, 1, 1, 0, 1, 1, 1, 1),
        ImmediateCredentialResponseLimits::new(1, 1, 1, 1, 1, 0, 1, 1, 1),
        ImmediateCredentialResponseLimits::new(1, 1, 1, 1, 1, 1, 0, 1, 1),
        ImmediateCredentialResponseLimits::new(1, 1, 1, 1, 1, 1, 1, 0, 1),
        ImmediateCredentialResponseLimits::new(1, 1, 1, 1, 1, 1, 1, 1, 0),
    ];
    for result in invalid {
        assert_eq!(
            result,
            Err(CredentialOfferError::InvalidImmediateCredentialResponseLimits)
        );
    }
}

#[test]
fn final_string_and_object_credentials_remain_ordered_and_exact() {
    let json = r#" {
        "credentials":[
            {"credential":"\u004aWT.payload.signature","format_hint":"jwt_vc_json"},
            {"credential":{ "claim":"välue", "number":1e3 }}
        ],
        "notification_id":"notify-opaque"
    } "#;
    let response = parse(json).expect("Final immediate response");

    assert_eq!(response.response_len(), json.len());
    assert_eq!(response.credentials().len(), 2);
    assert!(response.notification_id_present());
    assert_eq!(
        response.expose_sensitive_notification_id(),
        Some("notify-opaque")
    );

    let string = &response.credentials()[0];
    assert_eq!(string.kind(), CredentialValueKind::String);
    assert_eq!(
        string.expose_sensitive_json(),
        r#""\u004aWT.payload.signature""#
    );
    assert_eq!(
        string.expose_sensitive_string(),
        Some("JWT.payload.signature")
    );
    assert_eq!(string.json_len(), r#""\u004aWT.payload.signature""#.len());

    let object = &response.credentials()[1];
    assert_eq!(object.kind(), CredentialValueKind::Object);
    assert_eq!(
        object.expose_sensitive_json(),
        r#"{ "claim":"välue", "number":1e3 }"#
    );
    assert_eq!(object.expose_sensitive_string(), None);
}

#[test]
fn unique_bounded_extensions_are_validated_then_discarded() {
    let json = r#"{
        "future":{"nested":[true,null,{"number":1e999999}]},
        "credentials":[{
            "future_entry":[1,2,{"ok":true}],
            "credential":"opaque"
        }]
    }"#;
    let response = parse(json).expect("bounded extensions");

    assert_eq!(response.credentials().len(), 1);
    assert_eq!(
        response.credentials()[0].expose_sensitive_string(),
        Some("opaque")
    );
    assert!(!response.notification_id_present());
}

#[test]
fn required_immediate_shape_and_credential_kinds_fail_closed() {
    let invalid = [
        r#"{}"#,
        r#"[]"#,
        r#"{"credentials":null}"#,
        r#"{"credentials":[]}"#,
        r#"{"credentials":[null]}"#,
        r#"{"credentials":[{}]}"#,
        r#"{"credentials":[{"credential":null}]}"#,
        r#"{"credentials":[{"credential":true}]}"#,
        r#"{"credentials":[{"credential":7}]}"#,
        r#"{"credentials":[{"credential":[]}]}"#,
        r#"{"credentials":[{"credential":"valid"}]}[]"#,
        r#"{"credentials":[{"credential":"\uD800"}]}"#,
    ];
    for json in invalid {
        assert!(parse(json).is_err(), "unexpectedly accepted {json}");
    }
}

#[test]
fn deferred_and_misplaced_interval_members_are_explicitly_rejected() {
    for json in [
        r#"{"transaction_id":"deferred","interval":5}"#,
        r#"{"credentials":[{"credential":"value"}],"transaction_id":"deferred"}"#,
        r#"{"transaction_id":"deferred","credentials":[{"credential":"value"}]}"#,
    ] {
        assert_eq!(
            parse(json).expect_err("deferred branch"),
            CredentialOfferError::DeferredCredentialResponseUnsupported
        );
    }

    assert_eq!(
        parse(r#"{"credentials":[{"credential":"value"}],"interval":5}"#)
            .expect_err("interval on immediate branch"),
        CredentialOfferError::InvalidImmediateCredentialResponse
    );
}

#[test]
fn duplicate_members_fail_at_every_object_layer() {
    for json in [
        r#"{"credentials":[{"credential":"one"}],"credent\u0069als":[{"credential":"two"}]}"#,
        r#"{"credentials":[{"credential":"one","credent\u0069al":"two"}]}"#,
        r#"{"credentials":[{"credential":{"du\u0070":1,"dup":2}}]}"#,
        r#"{"credentials":[{"credential":"one","future":{"du\u0070":1,"dup":2}}]}"#,
    ] {
        assert_eq!(
            parse(json).expect_err("duplicate member"),
            CredentialOfferError::DuplicateJsonProperty
        );
    }
}

#[test]
fn response_count_member_and_retention_limits_are_independent() {
    let minimal = r#"{"credentials":[{"credential":"one"}]}"#;
    let exact = limits(minimal.len(), 16, 128, 1, 1, 1, 5, 5, 16);
    assert_eq!(
        ImmediateCredentialResponseCore::parse(minimal, exact)
            .expect("exact bounds")
            .credentials()
            .len(),
        1
    );

    let cases = [
        (
            minimal,
            limits(minimal.len() - 1, 16, 128, 4, 4, 4, 64, 64, 64),
            CredentialOfferError::ImmediateCredentialResponseTooLarge,
        ),
        (
            r#"{"credentials":[{"credential":"one"}],"extension":true}"#,
            limits(1_024, 16, 128, 1, 4, 4, 64, 64, 64),
            CredentialOfferError::TooManyCredentialResponseMembers,
        ),
        (
            r#"{"credentials":[{"credential":"one"},{"credential":"two"}]}"#,
            limits(1_024, 16, 128, 4, 1, 4, 64, 64, 64),
            CredentialOfferError::TooManyIssuedCredentials,
        ),
        (
            r#"{"credentials":[{"credential":"one","extension":true}]}"#,
            limits(1_024, 16, 128, 4, 4, 1, 64, 64, 64),
            CredentialOfferError::TooManyIssuedCredentialMembers,
        ),
        (
            minimal,
            limits(1_024, 16, 128, 4, 4, 4, 4, 64, 64),
            CredentialOfferError::IssuedCredentialTooLarge,
        ),
        (
            r#"{"credentials":[{"credential":"one"},{"credential":"two"}]}"#,
            limits(1_024, 16, 128, 4, 4, 4, 64, 9, 64),
            CredentialOfferError::IssuedCredentialsTooLarge,
        ),
        (
            r#"{"credentials":[{"credential":"one"}],"notification_id":"notify"}"#,
            limits(1_024, 16, 128, 4, 4, 4, 64, 64, 5),
            CredentialOfferError::CredentialNotificationIdTooLarge,
        ),
    ];
    for (json, limits, expected) in cases {
        assert_eq!(
            ImmediateCredentialResponseCore::parse(json, limits).expect_err("bounded failure"),
            expected
        );
    }

    assert_eq!(
        ImmediateCredentialResponseCore::parse(
            r#"{"credentials":[{"credential":{"nested":true}}]}"#,
            limits(1_024, 2, 128, 4, 4, 4, 64, 64, 64),
        )
        .expect_err("depth bound"),
        CredentialOfferError::JsonTooDeep
    );
    assert_eq!(
        ImmediateCredentialResponseCore::parse(minimal, limits(1_024, 16, 2, 4, 4, 4, 64, 64, 64),)
            .expect_err("node bound"),
        CredentialOfferError::JsonTooManyNodes
    );
}

#[test]
fn notification_identifier_must_be_nonempty_string() {
    for json in [
        r#"{"credentials":[{"credential":"one"}],"notification_id":""}"#,
        r#"{"credentials":[{"credential":"one"}],"notification_id":null}"#,
        r#"{"credentials":[{"credential":"one"}],"notification_id":7}"#,
    ] {
        assert_eq!(
            parse(json).expect_err("invalid notification ID"),
            CredentialOfferError::InvalidCredentialNotificationId
        );
    }
}

#[test]
fn response_credentials_and_errors_keep_remote_content_redacted() {
    let credential_canary = "CREDENTIAL_CANARY_4ed1";
    let notification_canary = "NOTIFICATION_CANARY_ba73";
    let extension_canary = "EXTENSION_CANARY_671c";
    let json = format!(
        r#"{{"credentials":[{{"credential":"{credential_canary}","future":"{extension_canary}"}}],"notification_id":"{notification_canary}"}}"#
    );
    let response = parse(&json).expect("canary response");

    for rendered in [
        format!("{response:?}"),
        format!("{:?}", response.credentials()[0]),
    ] {
        for canary in [credential_canary, notification_canary, extension_canary] {
            assert!(!rendered.contains(canary));
        }
    }

    let cases = [
        (
            CredentialOfferError::InvalidImmediateCredentialResponseLimits,
            error_code::INVALID_IMMEDIATE_CREDENTIAL_RESPONSE_LIMITS,
            ErrorKind::InvalidInput,
        ),
        (
            CredentialOfferError::ImmediateCredentialResponseTooLarge,
            error_code::IMMEDIATE_CREDENTIAL_RESPONSE_TOO_LARGE,
            ErrorKind::InvalidInput,
        ),
        (
            CredentialOfferError::InvalidImmediateCredentialResponse,
            error_code::INVALID_IMMEDIATE_CREDENTIAL_RESPONSE,
            ErrorKind::InvalidInput,
        ),
        (
            CredentialOfferError::DeferredCredentialResponseUnsupported,
            error_code::DEFERRED_CREDENTIAL_RESPONSE_UNSUPPORTED,
            ErrorKind::Unsupported,
        ),
        (
            CredentialOfferError::TooManyCredentialResponseMembers,
            error_code::TOO_MANY_CREDENTIAL_RESPONSE_MEMBERS,
            ErrorKind::InvalidInput,
        ),
        (
            CredentialOfferError::TooManyIssuedCredentials,
            error_code::TOO_MANY_ISSUED_CREDENTIALS,
            ErrorKind::InvalidInput,
        ),
        (
            CredentialOfferError::TooManyIssuedCredentialMembers,
            error_code::TOO_MANY_ISSUED_CREDENTIAL_MEMBERS,
            ErrorKind::InvalidInput,
        ),
        (
            CredentialOfferError::InvalidIssuedCredential,
            error_code::INVALID_ISSUED_CREDENTIAL,
            ErrorKind::InvalidInput,
        ),
        (
            CredentialOfferError::IssuedCredentialTooLarge,
            error_code::ISSUED_CREDENTIAL_TOO_LARGE,
            ErrorKind::InvalidInput,
        ),
        (
            CredentialOfferError::IssuedCredentialsTooLarge,
            error_code::ISSUED_CREDENTIALS_TOO_LARGE,
            ErrorKind::InvalidInput,
        ),
        (
            CredentialOfferError::InvalidCredentialNotificationId,
            error_code::INVALID_CREDENTIAL_NOTIFICATION_ID,
            ErrorKind::InvalidInput,
        ),
        (
            CredentialOfferError::CredentialNotificationIdTooLarge,
            error_code::CREDENTIAL_NOTIFICATION_ID_TOO_LARGE,
            ErrorKind::InvalidInput,
        ),
    ];
    for (error, code, kind) in cases {
        let core: IdentusError = error.into();
        assert_eq!(core.code(), code);
        assert_eq!(core.kind(), kind);
        assert_eq!(core.capability(), Some(CAPABILITY));
        for rendered in [
            format!("{error}"),
            format!("{error:?}"),
            format!("{core}"),
            format!("{core:?}"),
        ] {
            for canary in [credential_canary, notification_canary, extension_canary] {
                assert!(!rendered.contains(canary));
            }
        }
    }
}
