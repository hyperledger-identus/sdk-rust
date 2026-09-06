use identus_core::{ErrorKind, IdentusError};
use identus_oid4vci::{
    CAPABILITY, CredentialOffer, CredentialOfferError, CredentialOfferGrantLimits,
    CredentialOfferLimits, CredentialOfferRequest, CredentialOfferSemanticLimits,
    CredentialOfferWithGrants, EmbeddedCredentialOffer, TransactionCodeInputMode, error_code,
};

fn embedded(json: &str) -> EmbeddedCredentialOffer {
    EmbeddedCredentialOffer::try_from_json(json, CredentialOfferLimits::default())
        .expect("test JSON should pass transport validation")
}

fn grants_with_limits(
    json: &str,
    limits: CredentialOfferGrantLimits,
) -> Result<CredentialOfferWithGrants, CredentialOfferError> {
    CredentialOffer::try_from_embedded(embedded(json), CredentialOfferSemanticLimits::default())?
        .try_into_grants(limits)
}

fn grants(json: &str) -> Result<CredentialOfferWithGrants, CredentialOfferError> {
    grants_with_limits(json, CredentialOfferGrantLimits::default())
}

fn offer_with(grants: &str) -> String {
    format!(
        r#"{{"credential_issuer":"https://issuer.example","credential_configuration_ids":["Example"],"grants":{grants}}}"#
    )
}

fn encode_form(value: &str) -> String {
    let mut encoded = String::new();
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
            encoded.push(char::from(byte));
        } else {
            encoded.push_str(&format!("%{byte:02X}"));
        }
    }
    encoded
}

#[test]
fn accepts_final_known_grants_without_selecting_between_them() {
    let json = offer_with(
        r#"{"authorization_code":{"issuer_state":"eyJhbGciOiJSU0Et...FYUaBy","authorization_server":"https://authorization.example/tenant"},"urn:ietf:params:oauth:grant-type:pre-authorized_code":{"pre-authorized_code":"oaKazRN8I0IbtZ0C7JuMn5","tx_code":{"length":4,"input_mode":"numeric","description":"Please provide the one-time code that was sent via e-mail"},"authorization_server":"https://authorization.example/tenant"}}"#,
    );
    let offer = grants(&json).expect("both Final known grant shapes should validate");

    let authorization_code = offer
        .authorization_code()
        .expect("Authorization Code alternative");
    assert_eq!(
        authorization_code
            .issuer_state()
            .expect("issuer state")
            .as_str(),
        "eyJhbGciOiJSU0Et...FYUaBy"
    );
    assert_eq!(
        authorization_code
            .authorization_server()
            .expect("Authorization Server")
            .as_str(),
        "https://authorization.example/tenant"
    );

    let pre_authorized = offer
        .pre_authorized_code()
        .expect("Pre-Authorized Code alternative");
    assert_eq!(
        pre_authorized.pre_authorized_code().as_str(),
        "oaKazRN8I0IbtZ0C7JuMn5"
    );
    let transaction_code = pre_authorized
        .transaction_code()
        .expect("Transaction Code requirements");
    assert_eq!(
        transaction_code.input_mode(),
        Some(TransactionCodeInputMode::Numeric)
    );
    assert_eq!(
        transaction_code.effective_input_mode(),
        TransactionCodeInputMode::Numeric
    );
    assert_eq!(transaction_code.length(), Some(4));
    assert_eq!(
        transaction_code
            .description()
            .expect("description")
            .as_str(),
        "Please provide the one-time code that was sent via e-mail"
    );
    assert_eq!(offer.as_json(), json);
}

#[test]
fn preserves_absent_empty_and_unknown_grants() {
    let absent_json = r#"{"credential_issuer":"https://issuer.example","credential_configuration_ids":["Example"],"future":1e400}"#;
    let absent = grants(absent_json).expect("absent grants should validate");
    assert!(absent.authorization_code().is_none());
    assert!(absent.pre_authorized_code().is_none());
    assert_eq!(absent.as_json(), absent_json);

    for grant_object in [
        "{}",
        r#"{"future_grant":{"secret":"extension-canary","number":1e400}}"#,
    ] {
        let json = offer_with(grant_object);
        let offer = grants(&json).expect("unknown grant extensions remain opaque");
        assert!(offer.authorization_code().is_none());
        assert!(offer.pre_authorized_code().is_none());
        assert_eq!(offer.as_json(), json);
    }

    assert_eq!(
        grants(&offer_with(r#"{"future_grant":true}"#))
            .expect_err("every grant value must retain object shape"),
        CredentialOfferError::InvalidGrants
    );
}

#[test]
fn empty_transaction_code_uses_final_default() {
    let json = offer_with(
        r#"{"urn:ietf:params:oauth:grant-type:pre-authorized_code":{"pre-authorized_code":"secret","tx_code":{}}}"#,
    );
    let offer = grants(&json).expect("empty Transaction Code object is defined by Final");
    let transaction_code = offer
        .pre_authorized_code()
        .expect("grant")
        .transaction_code()
        .expect("present empty object requires a Transaction Code");
    assert_eq!(transaction_code.input_mode(), None);
    assert_eq!(
        transaction_code.effective_input_mode(),
        TransactionCodeInputMode::Numeric
    );
    assert_eq!(transaction_code.length(), None);
    assert!(transaction_code.description().is_none());
}

#[test]
fn known_grants_and_transaction_code_reject_type_confusion() {
    let cases = [
        (
            r#"{"authorization_code":null}"#,
            CredentialOfferError::InvalidAuthorizationCodeGrant,
        ),
        (
            r#"{"urn:ietf:params:oauth:grant-type:pre-authorized_code":[]}"#,
            CredentialOfferError::InvalidPreAuthorizedCodeGrant,
        ),
        (
            r#"{"urn:ietf:params:oauth:grant-type:pre-authorized_code":{}}"#,
            CredentialOfferError::InvalidPreAuthorizedCodeGrant,
        ),
        (
            r#"{"urn:ietf:params:oauth:grant-type:pre-authorized_code":{"pre-authorized_code":true}}"#,
            CredentialOfferError::InvalidPreAuthorizedCodeGrant,
        ),
        (
            r#"{"urn:ietf:params:oauth:grant-type:pre-authorized_code":{"pre-authorized_code":"secret","tx_code":null}}"#,
            CredentialOfferError::InvalidTransactionCode,
        ),
        (
            r#"{"urn:ietf:params:oauth:grant-type:pre-authorized_code":{"pre-authorized_code":"secret","tx_code":{"input_mode":"binary"}}}"#,
            CredentialOfferError::InvalidTransactionCodeMode,
        ),
        (
            r#"{"urn:ietf:params:oauth:grant-type:pre-authorized_code":{"pre-authorized_code":"secret","tx_code":{"length":0}}}"#,
            CredentialOfferError::InvalidTransactionCodeLength,
        ),
        (
            r#"{"urn:ietf:params:oauth:grant-type:pre-authorized_code":{"pre-authorized_code":"secret","tx_code":{"length":4.0}}}"#,
            CredentialOfferError::InvalidTransactionCodeLength,
        ),
        (
            r#"{"urn:ietf:params:oauth:grant-type:pre-authorized_code":{"pre-authorized_code":"secret","tx_code":{"length":4e0}}}"#,
            CredentialOfferError::InvalidTransactionCodeLength,
        ),
        (
            r#"{"urn:ietf:params:oauth:grant-type:pre-authorized_code":{"pre-authorized_code":"secret","tx_code":{"length":-4}}}"#,
            CredentialOfferError::InvalidTransactionCodeLength,
        ),
        (
            r#"{"urn:ietf:params:oauth:grant-type:pre-authorized_code":{"pre-authorized_code":"secret","tx_code":{"length":"4"}}}"#,
            CredentialOfferError::InvalidTransactionCodeLength,
        ),
        (
            r#"{"urn:ietf:params:oauth:grant-type:pre-authorized_code":{"pre-authorized_code":"secret","tx_code":{"description":""}}}"#,
            CredentialOfferError::InvalidTransactionCode,
        ),
    ];

    for (grant_object, expected) in cases {
        assert_eq!(
            grants(&offer_with(grant_object)).expect_err("invalid known grant must fail"),
            expected,
            "unexpected result for {grant_object}"
        );
    }
}

#[test]
fn rejects_empty_oversized_and_unsafe_grant_strings() {
    for member in [r#""issuer_state":"""#, r#""authorization_server":"""#] {
        let json = offer_with(&format!(r#"{{"authorization_code":{{{member}}}}}"#));
        assert_eq!(
            grants(&json).expect_err("empty Authorization Code member must fail"),
            CredentialOfferError::InvalidAuthorizationCodeGrant
        );
    }

    let empty_pre_authorized = offer_with(
        r#"{"urn:ietf:params:oauth:grant-type:pre-authorized_code":{"pre-authorized_code":""}}"#,
    );
    assert_eq!(
        grants(&empty_pre_authorized).expect_err("empty Pre-Authorized Code must fail"),
        CredentialOfferError::InvalidPreAuthorizedCodeGrant
    );

    for identifier in [
        "relative",
        "http://authorization.example",
        "https://",
        "https://user@authorization.example",
        "https://authorization.example?query=1",
        "https://authorization.example#fragment",
    ] {
        let json = offer_with(&format!(
            r#"{{"authorization_code":{{"authorization_server":"{identifier}"}}}}"#
        ));
        assert_eq!(
            grants(&json).expect_err("unsafe Authorization Server must fail"),
            CredentialOfferError::UnsafeAuthorizationServer
        );
    }
}

#[test]
fn grant_limits_and_final_character_ceiling_are_exact() {
    let authorization_server = "https://authorization.example";
    let description = "é".repeat(300);
    let json = offer_with(&format!(
        r#"{{"authorization_code":{{"issuer_state":"é","authorization_server":"{authorization_server}"}},"urn:ietf:params:oauth:grant-type:pre-authorized_code":{{"pre-authorized_code":"秘密","tx_code":{{"length":8,"input_mode":"text","description":"{description}"}}}}}}"#
    ));
    let exact = CredentialOfferGrantLimits::new(
        "é".len(),
        "秘密".len(),
        authorization_server.len(),
        description.len(),
        8,
    )
    .expect("exact limits");
    assert!(grants_with_limits(&json, exact).is_ok());

    let cases = [
        (
            CredentialOfferGrantLimits::new(1, "秘密".len(), authorization_server.len(), 600, 8)
                .expect("issuer-state limit"),
            CredentialOfferError::IssuerStateTooLarge,
        ),
        (
            CredentialOfferGrantLimits::new(2, 5, authorization_server.len(), 600, 8)
                .expect("Pre-Authorized Code limit"),
            CredentialOfferError::PreAuthorizedCodeTooLarge,
        ),
        (
            CredentialOfferGrantLimits::new(2, 6, authorization_server.len() - 1, 600, 8)
                .expect("Authorization Server limit"),
            CredentialOfferError::AuthorizationServerTooLarge,
        ),
        (
            CredentialOfferGrantLimits::new(2, 6, authorization_server.len(), 599, 8)
                .expect("description limit"),
            CredentialOfferError::TransactionCodeDescriptionTooLarge,
        ),
        (
            CredentialOfferGrantLimits::new(2, 6, authorization_server.len(), 600, 7)
                .expect("length limit"),
            CredentialOfferError::TransactionCodeLengthTooLarge,
        ),
    ];
    for (limits, expected) in cases {
        assert_eq!(
            grants_with_limits(&json, limits).expect_err("one-unit excess must fail"),
            expected
        );
    }

    let over_characters = offer_with(&format!(
        r#"{{"urn:ietf:params:oauth:grant-type:pre-authorized_code":{{"pre-authorized_code":"secret","tx_code":{{"description":"{}"}}}}}}"#,
        "a".repeat(301)
    ));
    assert_eq!(
        grants(&over_characters).expect_err("Final 300-character ceiling must hold"),
        CredentialOfferError::TransactionCodeDescriptionTooLarge
    );
}

#[test]
fn grant_limit_policy_is_positive_and_inspectable() {
    let defaults = CredentialOfferGrantLimits::default();
    assert_eq!(defaults.max_issuer_state_bytes(), 2_048);
    assert_eq!(defaults.max_pre_authorized_code_bytes(), 4_096);
    assert_eq!(defaults.max_authorization_server_bytes(), 2_048);
    assert_eq!(defaults.max_transaction_code_description_bytes(), 1_200);
    assert_eq!(defaults.max_transaction_code_length(), 64);

    for values in [
        (0, 1, 1, 1, 1),
        (1, 0, 1, 1, 1),
        (1, 1, 0, 1, 1),
        (1, 1, 1, 0, 1),
        (1, 1, 1, 1, 0),
    ] {
        assert_eq!(
            CredentialOfferGrantLimits::new(values.0, values.1, values.2, values.3, values.4),
            Err(CredentialOfferError::InvalidGrantLimits)
        );
    }
}

#[test]
fn invocation_and_direct_transport_reach_equivalent_grants() {
    let json = offer_with(
        r#"{"urn:ietf:params:oauth:grant-type:pre-authorized_code":{"pre-authorized_code":"secret","tx_code":{"input_mode":"text"}}}"#,
    );
    let direct = grants(&json).expect("direct transition");
    let invocation = format!(
        "openid-credential-offer://?credential_offer={}",
        encode_form(&json)
    );
    let CredentialOfferRequest::Embedded(transport) =
        CredentialOfferRequest::parse(&invocation, CredentialOfferLimits::default())
            .expect("invocation transport")
    else {
        panic!("expected embedded transport");
    };
    let through_invocation =
        CredentialOffer::try_from_embedded(transport, CredentialOfferSemanticLimits::default())
            .expect("core semantics")
            .try_into_grants(CredentialOfferGrantLimits::default())
            .expect("grant semantics");

    assert_eq!(direct.as_json(), through_invocation.as_json());
    assert_eq!(
        direct
            .pre_authorized_code()
            .expect("direct grant")
            .transaction_code()
            .expect("direct Transaction Code")
            .effective_input_mode(),
        through_invocation
            .pre_authorized_code()
            .expect("invocation grant")
            .transaction_code()
            .expect("invocation Transaction Code")
            .effective_input_mode()
    );
}

#[test]
fn grant_values_and_errors_are_redaction_safe() {
    let canaries = [
        "issuer-state-canary",
        "pre-authorized-canary",
        "authorization-server-canary",
        "description-canary",
        "extension-canary",
    ];
    let json = offer_with(
        r#"{"authorization_code":{"issuer_state":"issuer-state-canary"},"urn:ietf:params:oauth:grant-type:pre-authorized_code":{"pre-authorized_code":"pre-authorized-canary","authorization_server":"https://authorization-server-canary.example","tx_code":{"description":"description-canary"}},"future":{"value":"extension-canary"}}"#,
    );
    let offer = grants(&json).expect("canary offer should validate");
    let authorization_code = offer
        .authorization_code()
        .expect("Authorization Code grant");
    let pre_authorized = offer
        .pre_authorized_code()
        .expect("Pre-Authorized Code grant");
    let transaction_code = pre_authorized.transaction_code().expect("Transaction Code");
    for rendered in [
        format!("{offer:?}"),
        format!("{authorization_code:?}"),
        format!("{:?}", authorization_code.issuer_state().expect("state")),
        format!("{pre_authorized:?}"),
        format!("{:?}", pre_authorized.pre_authorized_code()),
        format!(
            "{:?}",
            pre_authorized
                .authorization_server()
                .expect("Authorization Server")
        ),
        format!("{transaction_code:?}"),
        format!("{:?}", transaction_code.description().expect("description")),
    ] {
        for canary in canaries {
            assert!(!rendered.contains(canary));
        }
    }

    let cases = [
        (
            CredentialOfferError::InvalidGrantLimits,
            error_code::INVALID_GRANT_LIMITS,
        ),
        (
            CredentialOfferError::InvalidAuthorizationCodeGrant,
            error_code::INVALID_AUTHORIZATION_CODE_GRANT,
        ),
        (
            CredentialOfferError::InvalidPreAuthorizedCodeGrant,
            error_code::INVALID_PRE_AUTHORIZED_CODE_GRANT,
        ),
        (
            CredentialOfferError::IssuerStateTooLarge,
            error_code::ISSUER_STATE_TOO_LARGE,
        ),
        (
            CredentialOfferError::PreAuthorizedCodeTooLarge,
            error_code::PRE_AUTHORIZED_CODE_TOO_LARGE,
        ),
        (
            CredentialOfferError::AuthorizationServerTooLarge,
            error_code::AUTHORIZATION_SERVER_TOO_LARGE,
        ),
        (
            CredentialOfferError::UnsafeAuthorizationServer,
            error_code::UNSAFE_AUTHORIZATION_SERVER,
        ),
        (
            CredentialOfferError::InvalidTransactionCode,
            error_code::INVALID_TRANSACTION_CODE,
        ),
        (
            CredentialOfferError::InvalidTransactionCodeMode,
            error_code::INVALID_TRANSACTION_CODE_MODE,
        ),
        (
            CredentialOfferError::InvalidTransactionCodeLength,
            error_code::INVALID_TRANSACTION_CODE_LENGTH,
        ),
        (
            CredentialOfferError::TransactionCodeLengthTooLarge,
            error_code::TRANSACTION_CODE_LENGTH_TOO_LARGE,
        ),
        (
            CredentialOfferError::TransactionCodeDescriptionTooLarge,
            error_code::TRANSACTION_CODE_DESCRIPTION_TOO_LARGE,
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
            for canary in canaries {
                assert!(!rendered.contains(canary));
            }
        }
    }
}
