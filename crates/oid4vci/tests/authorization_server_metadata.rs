use identus_oid4vci::{
    AUTHORIZATION_CODE_GRANT_TYPE, AuthorizationServerMetadataCore,
    AuthorizationServerMetadataLimits, CredentialOfferError, IMPLICIT_GRANT_TYPE,
    PRE_AUTHORIZED_CODE_GRANT_TYPE,
};

const ISSUER: &str = "https://issuer.example";

fn metadata(json: &str) -> Result<AuthorizationServerMetadataCore, CredentialOfferError> {
    AuthorizationServerMetadataCore::parse(
        json,
        ISSUER,
        AuthorizationServerMetadataLimits::default(),
    )
}

#[test]
fn accepts_consumer_shape_as_a_partial_core_without_full_conformance_claim() {
    let json = format!(
        r#"{{
        "grant_types_supported":["{PRE_AUTHORIZED_CODE_GRANT_TYPE}"],
        "issuer":"{ISSUER}",
        "pre-authorized_grant_anonymous_access_supported":true,
        "token_endpoint":"https://issuer.example/api/issuer/token",
        "future":{{"large":1e999999,"nested":[true,null]}}
}}"#,
    );
    let parsed = metadata(&json).expect("consumer metadata core");
    assert_eq!(parsed.issuer().as_str(), ISSUER);
    assert!(parsed.authorization_endpoint().is_none());
    assert_eq!(
        parsed.token_endpoint().expect("token endpoint").as_str(),
        "https://issuer.example/api/issuer/token"
    );
    assert_eq!(
        parsed.advertised_grant_types().expect("grants")[0].as_str(),
        PRE_AUTHORIZED_CODE_GRANT_TYPE
    );
    assert_eq!(
        parsed.advertised_anonymous_pre_authorized_access(),
        Some(true)
    );
    assert!(parsed.effective_anonymous_pre_authorized_access());
    assert_eq!(parsed.as_json(), json);
}

#[test]
fn omitted_grants_and_anonymous_flag_preserve_rfc_and_final_defaults() {
    let parsed = metadata(&format!(r#"{{"issuer":"{ISSUER}"}}"#)).expect("metadata core");
    assert!(parsed.advertised_grant_types().is_none());
    assert_eq!(parsed.effective_grant_type_count(), 2);
    assert_eq!(
        parsed.effective_grant_type(0),
        Some(AUTHORIZATION_CODE_GRANT_TYPE)
    );
    assert_eq!(parsed.effective_grant_type(1), Some(IMPLICIT_GRANT_TYPE));
    assert_eq!(parsed.effective_grant_type(2), None);
    assert_eq!(parsed.advertised_anonymous_pre_authorized_access(), None);
    assert!(!parsed.effective_anonymous_pre_authorized_access());
}

#[test]
fn explicit_endpoints_grants_and_false_anonymous_flag_remain_exact() {
    let json = format!(
        r#"{{"issuer":"{ISSUER}","authorization_endpoint":"https://auth.example/authorize?prompt=login","token_endpoint":"https://auth.example/token?tenant=1","grant_types_supported":["authorization_code","future_grant"],"pre-authorized_grant_anonymous_access_supported":false}}"#,
    );
    let parsed = metadata(&json).expect("metadata core");
    assert_eq!(
        parsed
            .authorization_endpoint()
            .expect("authorization endpoint")
            .as_str(),
        "https://auth.example/authorize?prompt=login"
    );
    assert_eq!(
        parsed.token_endpoint().expect("token endpoint").as_str(),
        "https://auth.example/token?tenant=1"
    );
    assert_eq!(parsed.effective_grant_type_count(), 2);
    assert_eq!(parsed.effective_grant_type(1), Some("future_grant"));
    assert_eq!(
        parsed.advertised_anonymous_pre_authorized_access(),
        Some(false)
    );
    assert!(!parsed.effective_anonymous_pre_authorized_access());
}

#[test]
fn issuer_binding_is_exact_and_both_values_require_safe_identifier_syntax() {
    let json = format!(r#"{{"issuer":"{ISSUER}"}}"#);
    for expected in [
        "https://ISSUER.example",
        "https://issuer.example/",
        "http://issuer.example",
        "https://user@issuer.example",
        "https://issuer.example?q=1",
        "https://issuer.example#fragment",
    ] {
        assert!(
            AuthorizationServerMetadataCore::parse(
                &json,
                expected,
                AuthorizationServerMetadataLimits::default()
            )
            .is_err()
        );
    }

    for issuer in [
        "http://issuer.example",
        "https://user@issuer.example",
        "https://issuer.example?q=1",
        "https://issuer.example#fragment",
    ] {
        assert!(
            AuthorizationServerMetadataCore::parse(
                &format!(r#"{{"issuer":"{issuer}"}}"#),
                issuer,
                AuthorizationServerMetadataLimits::default()
            )
            .is_err()
        );
    }
}

#[test]
fn unsafe_or_mistyped_endpoints_fail_closed() {
    for member in [
        r#""authorization_endpoint":7"#,
        r#""authorization_endpoint":"""#,
        r#""authorization_endpoint":"http://auth.example/authorize""#,
        r#""authorization_endpoint":"https://user@auth.example/authorize""#,
        r#""authorization_endpoint":"https://auth.example/authorize#fragment""#,
        r#""token_endpoint":false"#,
        r#""token_endpoint":"http://auth.example/token""#,
        r#""token_endpoint":"https://user@auth.example/token""#,
        r#""token_endpoint":"https://auth.example/token#fragment""#,
    ] {
        assert!(metadata(&format!(r#"{{"issuer":"{ISSUER}",{member}}}"#)).is_err());
    }
}

#[test]
fn grant_and_anonymous_field_ambiguity_fails_closed() {
    for member in [
        r#""grant_types_supported":[]"#,
        r#""grant_types_supported":"authorization_code""#,
        r#""grant_types_supported":[""]"#,
        r#""grant_types_supported":["a","a"]"#,
        r#""grant_types_supported":[7]"#,
        r#""pre-authorized_grant_anonymous_access_supported":"true""#,
        r#""pre-authorized_grant_anonymous_access_supported":1"#,
        r#""pre-authorized_grant_anonymous_access_supported":null"#,
    ] {
        assert!(metadata(&format!(r#"{{"issuer":"{ISSUER}",{member}}}"#)).is_err());
    }
}

#[test]
fn malformed_duplicate_nested_and_trailing_json_fail_closed() {
    let invalid = [
        "[]".to_owned(),
        "{}".to_owned(),
        format!(r#"{{"issuer":"{ISSUER}","issuer":"{ISSUER}"}}"#),
        format!(r#"{{"issuer":"{ISSUER}","future":{{"du\u0070":1,"dup":2}}}}"#,),
        format!(r#"{{"issuer":"{ISSUER}"}}[]"#),
    ];
    for json in invalid {
        assert!(metadata(&json).is_err());
    }
}

#[test]
fn limits_are_positive_inspectable_and_exact() {
    assert!(AuthorizationServerMetadataLimits::new(0, 1, 1, 1, 1, 1, 1).is_err());
    assert!(AuthorizationServerMetadataLimits::new(1, 65, 1, 1, 1, 1, 1).is_err());
    let defaults = AuthorizationServerMetadataLimits::default();
    assert_eq!(defaults.max_json_bytes(), 131_072);
    assert_eq!(defaults.max_json_depth(), 16);
    assert_eq!(defaults.max_json_nodes(), 1_024);
    assert_eq!(defaults.max_issuer_bytes(), 2_048);
    assert_eq!(defaults.max_endpoint_bytes(), 2_048);
    assert_eq!(defaults.max_grant_type_bytes(), 256);
    assert_eq!(defaults.max_grant_types(), 32);

    let endpoint = ISSUER;
    let json = format!(
        r#"{{"issuer":"{ISSUER}","authorization_endpoint":"{endpoint}","token_endpoint":"{endpoint}","grant_types_supported":["g1","g2"],"pre-authorized_grant_anonymous_access_supported":true,"future":{{"x":1e999999}}}}"#,
    );
    let exact = AuthorizationServerMetadataLimits::new(
        json.len(),
        2,
        10,
        ISSUER.len(),
        endpoint.len(),
        2,
        2,
    )
    .expect("exact limits");
    let parsed =
        AuthorizationServerMetadataCore::parse(&json, ISSUER, exact).expect("exact metadata core");
    assert_eq!(parsed.as_json().len(), exact.max_json_bytes());

    let one_less = [
        AuthorizationServerMetadataLimits::new(
            json.len() - 1,
            2,
            10,
            ISSUER.len(),
            endpoint.len(),
            2,
            2,
        ),
        AuthorizationServerMetadataLimits::new(
            json.len(),
            1,
            10,
            ISSUER.len(),
            endpoint.len(),
            2,
            2,
        ),
        AuthorizationServerMetadataLimits::new(
            json.len(),
            2,
            9,
            ISSUER.len(),
            endpoint.len(),
            2,
            2,
        ),
        AuthorizationServerMetadataLimits::new(
            json.len(),
            2,
            10,
            ISSUER.len() - 1,
            endpoint.len(),
            2,
            2,
        ),
        AuthorizationServerMetadataLimits::new(
            json.len(),
            2,
            10,
            ISSUER.len(),
            endpoint.len() - 1,
            2,
            2,
        ),
        AuthorizationServerMetadataLimits::new(
            json.len(),
            2,
            10,
            ISSUER.len(),
            endpoint.len(),
            1,
            2,
        ),
        AuthorizationServerMetadataLimits::new(
            json.len(),
            2,
            10,
            ISSUER.len(),
            endpoint.len(),
            2,
            1,
        ),
    ];
    for limits in one_less {
        assert!(
            AuthorizationServerMetadataCore::parse(&json, ISSUER, limits.expect("limits")).is_err()
        );
    }
}

#[test]
fn metadata_core_and_errors_do_not_disclose_caller_content() {
    let canary = "AUTHORIZATION_METADATA_SECRET_CANARY_9ad2";
    let json = format!(
        r#"{{"issuer":"{ISSUER}","token_endpoint":"https://auth.example/{canary}","grant_types_supported":["{canary}"],"future":"{canary}"}}"#,
    );
    let parsed = metadata(&json).expect("metadata core");
    assert!(!format!("{parsed:?}").contains(canary));
    assert!(!format!("{:?}", parsed.token_endpoint()).contains(canary));
    assert!(!format!("{:?}", parsed.advertised_grant_types()).contains(canary));

    let error = metadata(canary).expect_err("invalid metadata core");
    assert!(!format!("{error:?} {error}").contains(canary));
    assert!(!format!("{:?}", error.to_identus_error()).contains(canary));
}
