use identus_core::{ErrorKind, IdentusError};
use identus_oid4vci::{
    CAPABILITY, CredentialOfferError, TokenAuthorizationDetailsLimits, TokenResponseCore,
    TokenResponseLimits, error_code,
};

fn parse(json: &str) -> TokenResponseCore {
    TokenResponseCore::parse(json, TokenResponseLimits::default()).expect("Token Response core")
}

fn validate(
    json: &str,
    limits: TokenAuthorizationDetailsLimits,
) -> Result<identus_oid4vci::TokenResponseWithAuthorizationDetails, CredentialOfferError> {
    parse(json).try_validate_authorization_details(limits)
}

fn detail(configuration: &str, identifiers: &str) -> String {
    format!(
        r#"{{"type":"openid_credential","credential_configuration_id":"{configuration}","credential_identifiers":[{identifiers}]}}"#
    )
}

#[test]
fn defaults_and_positive_limits_are_explicit() {
    let defaults = TokenAuthorizationDetailsLimits::default();
    assert_eq!(defaults.max_authorization_details(), 32);
    assert_eq!(defaults.max_type_bytes(), 128);
    assert_eq!(defaults.max_credential_configuration_id_bytes(), 256);
    assert_eq!(defaults.max_credential_identifiers_per_detail(), 64);
    assert_eq!(defaults.max_credential_identifier_bytes(), 2_048);

    for result in [
        TokenAuthorizationDetailsLimits::new(0, 1, 1, 1, 1),
        TokenAuthorizationDetailsLimits::new(1, 0, 1, 1, 1),
        TokenAuthorizationDetailsLimits::new(1, 1, 0, 1, 1),
        TokenAuthorizationDetailsLimits::new(1, 1, 1, 0, 1),
        TokenAuthorizationDetailsLimits::new(1, 1, 1, 1, 0),
    ] {
        assert_eq!(
            result,
            Err(CredentialOfferError::InvalidTokenAuthorizationDetailsLimits)
        );
    }
}

#[test]
fn final_example_and_mixed_extensions_produce_a_typed_redacted_state() {
    let json = r#"{
        "access_token":"SYNTHETIC_ACCESS_TOKEN",
        "token_type":"Bearer",
        "authorization_details":[
            {
                "credential_identifiers":["CivilEngineeringDegree-2023","ElectricalEngineeringDegree-2023"],
                "future":{"nested":[true,null]},
                "credential_configuration_id":"UniversityDegreeCredential",
                "type":"openid_credential"
            },
            {
                "type":"example_payment",
                "credential_configuration_id":{"not":"a credential field"},
                "credential_identifiers":false
            }
        ]
    }"#;
    let response = validate(json, TokenAuthorizationDetailsLimits::default())
        .expect("Final Authorization Details");

    assert_eq!(response.unknown_authorization_detail_count(), 1);
    assert_eq!(
        response
            .token_response_core()
            .expose_sensitive_access_token(),
        "SYNTHETIC_ACCESS_TOKEN"
    );
    let details = response.credential_authorization_details();
    assert_eq!(details.len(), 1);
    assert_eq!(
        details[0].credential_configuration_id(),
        "UniversityDegreeCredential"
    );
    assert_eq!(details[0].credential_identifier_count(), 2);
    assert_eq!(
        details[0].credential_identifiers().collect::<Vec<_>>(),
        vec![
            "CivilEngineeringDegree-2023",
            "ElectricalEngineeringDegree-2023"
        ]
    );
}

#[test]
fn presence_only_core_remains_compatible_and_transition_fails_closed() {
    for value in [r#"{}"#, r#"[]"#, r#"42"#] {
        let json = format!(
            r#"{{"access_token":"token","token_type":"Bearer","authorization_details":{value}}}"#
        );
        let core = parse(&json);
        assert!(core.authorization_details_present());
        assert_eq!(
            core.try_validate_authorization_details(TokenAuthorizationDetailsLimits::default())
                .expect_err("invalid semantic details"),
            CredentialOfferError::InvalidTokenAuthorizationDetails
        );
    }

    let core = parse(r#"{"access_token":"token","token_type":"Bearer"}"#);
    assert!(!core.authorization_details_present());
    assert_eq!(
        core.try_validate_authorization_details(TokenAuthorizationDetailsLimits::default())
            .expect_err("missing details"),
        CredentialOfferError::InvalidTokenAuthorizationDetails
    );
}

#[test]
fn recognized_entries_require_all_final_fields_and_a_supported_entry() {
    let invalid_details = [
        r#"{"type":"openid_credential","credential_identifiers":["id"]}"#,
        r#"{"type":"openid_credential","credential_configuration_id":"cfg"}"#,
        r#"{"type":"openid_credential","credential_configuration_id":"","credential_identifiers":["id"]}"#,
        r#"{"type":"openid_credential","credential_configuration_id":"cfg","credential_identifiers":[]}"#,
        r#"{"type":"openid_credential","credential_configuration_id":"cfg","credential_identifiers":[""]}"#,
        r#"{"type":"openid_credential","credential_configuration_id":7,"credential_identifiers":["id"]}"#,
        r#"{"type":"openid_credential","credential_configuration_id":"cfg","credential_identifiers":false}"#,
        r#"{"type":"example_only"}"#,
        r#"{"credential_configuration_id":"cfg","credential_identifiers":["id"]}"#,
    ];
    for details in invalid_details {
        let json = format!(
            r#"{{"access_token":"token","token_type":"Bearer","authorization_details":[{details}]}}"#
        );
        assert!(
            validate(&json, TokenAuthorizationDetailsLimits::default()).is_err(),
            "unexpectedly accepted {details}"
        );
    }
}

#[test]
fn exact_and_one_over_collection_and_string_limits_are_enforced() {
    let limits =
        TokenAuthorizationDetailsLimits::new(2, 17, 3, 2, 3).expect("positive narrow limits");
    let first = detail("cfg", r#""one","two""#);
    let second = detail("abc", r#""red""#);
    let exact = format!(
        r#"{{"access_token":"token","token_type":"Bearer","authorization_details":[{first},{second}]}}"#
    );
    assert_eq!(
        validate(&exact, limits)
            .expect("exact limits")
            .credential_authorization_details()
            .len(),
        2
    );

    let cases = [
        (
            format!(
                r#"{{"access_token":"token","token_type":"Bearer","authorization_details":[{first},{second},{second}]}}"#
            ),
            CredentialOfferError::TooManyTokenAuthorizationDetails,
        ),
        (
            format!(
                r#"{{"access_token":"token","token_type":"Bearer","authorization_details":[{{"type":"openid_credentialx","credential_configuration_id":"cfg","credential_identifiers":["one"]}},{first}]}}"#
            ),
            CredentialOfferError::TokenAuthorizationDetailValueTooLarge,
        ),
        (
            format!(
                r#"{{"access_token":"token","token_type":"Bearer","authorization_details":[{}]}}"#,
                detail("four", r#""one""#)
            ),
            CredentialOfferError::TokenAuthorizationDetailValueTooLarge,
        ),
        (
            format!(
                r#"{{"access_token":"token","token_type":"Bearer","authorization_details":[{}]}}"#,
                detail("cfg", r#""one","two","red""#)
            ),
            CredentialOfferError::TooManyCredentialIdentifiers,
        ),
        (
            format!(
                r#"{{"access_token":"token","token_type":"Bearer","authorization_details":[{}]}}"#,
                detail("cfg", r#""four""#)
            ),
            CredentialOfferError::TokenAuthorizationDetailValueTooLarge,
        ),
    ];
    for (json, expected) in cases {
        assert_eq!(validate(&json, limits).expect_err("one over"), expected);
    }
}

#[test]
fn duplicate_members_and_dataset_identifiers_are_rejected() {
    let duplicate_member = r#"{"access_token":"token","token_type":"Bearer","authorization_details":[{"type":"openid_credential","ty\u0070e":"openid_credential","credential_configuration_id":"a","credential_identifiers":["one"]}]}"#;
    assert_eq!(
        TokenResponseCore::parse(duplicate_member, TokenResponseLimits::default())
            .expect_err("duplicate nested member"),
        CredentialOfferError::DuplicateJsonProperty
    );

    let cases = [
        r#"{"access_token":"token","token_type":"Bearer","authorization_details":[{"type":"openid_credential","credential_configuration_id":"a","credential_identifiers":["one","\u006fne"]}]}"#.to_owned(),
        format!(
            r#"{{"access_token":"token","token_type":"Bearer","authorization_details":[{},{}]}}"#,
            detail("a", r#""one""#),
            detail("b", r#""one""#)
        ),
    ];
    for json in cases {
        assert!(
            validate(&json, TokenAuthorizationDetailsLimits::default()).is_err(),
            "unexpectedly accepted {json}"
        );
    }
}

#[test]
fn decoded_utf8_bytes_define_string_limits() {
    let json = r#"{"access_token":"token","token_type":"Bearer","authorization_details":[{"type":"openid_credential","credential_configuration_id":"\u00e9","credential_identifiers":["\u00e9"]}]}"#;
    validate(
        json,
        TokenAuthorizationDetailsLimits::new(1, 17, 2, 1, 2).expect("two-byte limits"),
    )
    .expect("decoded two-byte values");
    assert_eq!(
        validate(
            json,
            TokenAuthorizationDetailsLimits::new(1, 17, 1, 1, 2).expect("one-byte config limit"),
        )
        .expect_err("decoded configuration is two bytes"),
        CredentialOfferError::TokenAuthorizationDetailValueTooLarge
    );
    assert_eq!(
        validate(
            json,
            TokenAuthorizationDetailsLimits::new(1, 17, 2, 1, 1).expect("one-byte ID limit"),
        )
        .expect_err("decoded identifier is two bytes"),
        CredentialOfferError::TokenAuthorizationDetailValueTooLarge
    );
}

#[test]
fn successful_values_and_all_new_errors_are_redacted() {
    let configuration_canary = "CONFIGURATION_CANARY_837e";
    let identifier_canary = "IDENTIFIER_CANARY_991a";
    let token_canary = "TOKEN_CANARY_1c20";
    let json = format!(
        r#"{{"access_token":"{token_canary}","token_type":"Bearer","authorization_details":[{{"type":"openid_credential","credential_configuration_id":"{configuration_canary}","credential_identifiers":["{identifier_canary}"]}}]}}"#
    );
    let response =
        validate(&json, TokenAuthorizationDetailsLimits::default()).expect("canary details");
    for rendered in [
        format!("{response:?}"),
        format!("{:?}", response.credential_authorization_details()[0]),
    ] {
        for canary in [configuration_canary, identifier_canary, token_canary] {
            assert!(!rendered.contains(canary));
        }
    }

    let cases = [
        (
            CredentialOfferError::InvalidTokenAuthorizationDetailsLimits,
            error_code::INVALID_TOKEN_AUTHORIZATION_DETAILS_LIMITS,
        ),
        (
            CredentialOfferError::InvalidTokenAuthorizationDetails,
            error_code::INVALID_TOKEN_AUTHORIZATION_DETAILS,
        ),
        (
            CredentialOfferError::TooManyTokenAuthorizationDetails,
            error_code::TOO_MANY_TOKEN_AUTHORIZATION_DETAILS,
        ),
        (
            CredentialOfferError::TokenAuthorizationDetailValueTooLarge,
            error_code::TOKEN_AUTHORIZATION_DETAIL_VALUE_TOO_LARGE,
        ),
        (
            CredentialOfferError::TooManyCredentialIdentifiers,
            error_code::TOO_MANY_CREDENTIAL_IDENTIFIERS,
        ),
        (
            CredentialOfferError::DuplicateCredentialIdentifier,
            error_code::DUPLICATE_CREDENTIAL_IDENTIFIER,
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
            for canary in [configuration_canary, identifier_canary, token_canary] {
                assert!(!rendered.contains(canary));
            }
        }
    }
}
