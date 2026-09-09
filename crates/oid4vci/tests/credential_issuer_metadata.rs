use identus_core::{ErrorKind, IdentusError};
use identus_oid4vci::{
    CAPABILITY, CredentialIssuerMetadata, CredentialIssuerMetadataLimits, CredentialOffer,
    CredentialOfferError, CredentialOfferGrantLimits, CredentialOfferLimits,
    CredentialOfferSemanticLimits, EmbeddedCredentialOffer, error_code,
};

const ISSUER: &str = "https://credential-issuer.example.com/tenant";
const PRE_AUTHORIZED_GRANT: &str = "urn:ietf:params:oauth:grant-type:pre-authorized_code";

fn metadata(json: &str) -> Result<CredentialIssuerMetadata, CredentialOfferError> {
    CredentialIssuerMetadata::parse(json, ISSUER, CredentialIssuerMetadataLimits::default())
}

fn offer(
    issuer: &str,
    configuration_ids: &str,
    grant: &str,
) -> identus_oid4vci::CredentialOfferWithGrants {
    let json = format!(
        r#"{{"credential_issuer":"{issuer}","credential_configuration_ids":{configuration_ids},"grants":{grant}}}"#,
    );
    CredentialOffer::try_from_embedded(
        EmbeddedCredentialOffer::try_from_json(&json, CredentialOfferLimits::default())
            .expect("transport"),
        CredentialOfferSemanticLimits::default(),
    )
    .expect("semantics")
    .try_into_grants(CredentialOfferGrantLimits::default())
    .expect("grants")
}

fn core_metadata(authorization_servers: &str, configurations: &str) -> String {
    format!(
        r#"{{"credential_issuer":"{ISSUER}"{authorization_servers},"credential_endpoint":"https://credential-issuer.example.com:8443/tenant/credential?version=1","credential_configurations_supported":{configurations}}}"#,
    )
}

#[test]
fn accepts_final_and_consumer_shaped_metadata_without_format_policy() {
    let json = core_metadata(
        r#", "authorization_servers":["https://auth.example.com/a"]"#,
        r#"{
            "UniversityDegreeCredential": {
                "format":"jwt_vc_json",
                "scope":"degree",
                "proof_types_supported":{"jwt":{"proof_signing_alg_values_supported":["ES256"]}}
            },
            "digital_passport_v1": {
                "format":"midnight_cbor_phase1",
                "credential_metadata":{"display":[{"name":"Digital Passport"}]},
                "future_number":1e999999
            }
        }"#,
    );
    let parsed = metadata(&json).expect("metadata");
    assert_eq!(parsed.credential_issuer().as_str(), ISSUER);
    assert_eq!(
        parsed.credential_endpoint().as_str(),
        "https://credential-issuer.example.com:8443/tenant/credential?version=1"
    );
    assert!(parsed.nonce_endpoint().is_none());
    assert!(parsed.deferred_credential_endpoint().is_none());
    assert_eq!(parsed.effective_authorization_server_count(), 1);
    assert_eq!(
        parsed.effective_authorization_server(0),
        Some("https://auth.example.com/a")
    );
    assert_eq!(parsed.credential_configurations().len(), 2);
    assert_eq!(
        parsed
            .credential_configuration("digital_passport_v1")
            .expect("configuration")
            .format()
            .as_str(),
        "midnight_cbor_phase1"
    );
    assert_eq!(parsed.as_json(), json);
}

#[test]
fn omitted_authorization_servers_uses_issuer_as_effective_default() {
    let parsed =
        metadata(&core_metadata("", r#"{"degree":{"format":"dc+sd-jwt"}}"#)).expect("metadata");
    assert!(parsed.advertised_authorization_servers().is_none());
    assert_eq!(parsed.effective_authorization_server_count(), 1);
    assert_eq!(parsed.effective_authorization_server(0), Some(ISSUER));
    assert_eq!(parsed.effective_authorization_server(1), None);
}

#[test]
fn exposes_the_exact_optional_final_nonce_endpoint() {
    let nonce_endpoint = "https://nonce.example.com:8443/tenant/nonce?wallet=holder";
    let json = core_metadata(
        &format!(r#", "nonce_endpoint":"{nonce_endpoint}""#),
        r#"{"degree":{"format":"dc+sd-jwt"}}"#,
    );
    let parsed = metadata(&json).expect("metadata");
    assert_eq!(
        parsed.nonce_endpoint().map(|endpoint| endpoint.as_str()),
        Some(nonce_endpoint)
    );
    assert_eq!(parsed.as_json(), json);
}

#[test]
fn exposes_the_exact_optional_final_deferred_credential_endpoint() {
    let endpoint = "https://credential-issuer.example.com:8443/deferred?tenant=wallet";
    let json = core_metadata(
        &format!(r#", "deferred_credential_endpoint":"{endpoint}""#),
        r#"{"degree":{"format":"dc+sd-jwt"}}"#,
    );
    let parsed = metadata(&json).expect("metadata");
    assert_eq!(
        parsed
            .deferred_credential_endpoint()
            .map(|value| value.as_str()),
        Some(endpoint)
    );
    assert_eq!(parsed.as_json(), json);
}

#[test]
fn matching_consumes_offer_and_metadata_without_selecting_a_grant() {
    let grants = format!(
        r#"{{"authorization_code":{{"authorization_server":"https://auth.example.com/a"}},"{PRE_AUTHORIZED_GRANT}":{{"pre-authorized_code":"secret","authorization_server":"https://auth.example.com/b"}}}}"#,
    );
    let offer = offer(ISSUER, r#"["degree","passport"]"#, &grants);
    let metadata = metadata(&core_metadata(
        r#", "authorization_servers":["https://auth.example.com/a","https://auth.example.com/b"]"#,
        r#"{"degree":{"format":"dc+sd-jwt"},"passport":{"format":"mso_mdoc"}}"#,
    ))
    .expect("metadata");
    let matched = offer.try_with_metadata(metadata).expect("matched");
    assert!(matched.credential_offer().authorization_code().is_some());
    assert!(matched.credential_offer().pre_authorized_code().is_some());
    assert_eq!(
        matched
            .credential_issuer_metadata()
            .credential_configurations()
            .len(),
        2
    );
}

#[test]
fn issuer_comparisons_are_exact_and_offered_configurations_must_exist() {
    let valid = core_metadata("", r#"{"degree":{"format":"dc+sd-jwt"}}"#);
    assert!(matches!(
        CredentialIssuerMetadata::parse(
            &valid,
            "https://credential-issuer.example.com/tenant/",
            CredentialIssuerMetadataLimits::default()
        ),
        Err(CredentialOfferError::MetadataIssuerMismatch)
    ));

    let parsed_metadata = metadata(&valid).expect("metadata");
    assert!(matches!(
        offer(
            "https://credential-issuer.example.com/TENANT",
            r#"["degree"]"#,
            "{}"
        )
        .try_with_metadata(parsed_metadata),
        Err(CredentialOfferError::OfferMetadataIssuerMismatch)
    ));

    let parsed_metadata = metadata(&valid).expect("metadata");
    assert!(matches!(
        offer(ISSUER, r#"["missing"]"#, "{}").try_with_metadata(parsed_metadata),
        Err(CredentialOfferError::OfferedConfigurationMissing)
    ));
}

#[test]
fn authorization_server_hints_require_multiple_exact_metadata_entries() {
    let hinted_grant = format!(
        r#"{{"{PRE_AUTHORIZED_GRANT}":{{"pre-authorized_code":"secret","authorization_server":"https://auth.example.com/a"}}}}"#,
    );
    for authorization_servers in [
        "",
        r#", "authorization_servers":["https://auth.example.com/a"]"#,
        r#", "authorization_servers":["https://auth.example.com/b","https://auth.example.com/c"]"#,
    ] {
        let metadata = metadata(&core_metadata(
            authorization_servers,
            r#"{"degree":{"format":"dc+sd-jwt"}}"#,
        ))
        .expect("metadata");
        assert!(matches!(
            offer(ISSUER, r#"["degree"]"#, &hinted_grant).try_with_metadata(metadata),
            Err(CredentialOfferError::InvalidAuthorizationServerHint)
        ));
    }
}

#[test]
fn rejects_invalid_required_fields_endpoints_and_configurations() {
    let invalid = [
        "[]",
        r#"{}"#,
        r#"{"credential_issuer":7,"credential_endpoint":"https://e.example/c","credential_configurations_supported":{"x":{"format":"f"}}}"#,
        r#"{"credential_issuer":"https://credential-issuer.example.com/tenant","credential_endpoint":"http://e.example/c","credential_configurations_supported":{"x":{"format":"f"}}}"#,
        r#"{"credential_issuer":"https://credential-issuer.example.com/tenant","credential_endpoint":"https://u:p@e.example/c","credential_configurations_supported":{"x":{"format":"f"}}}"#,
        r#"{"credential_issuer":"https://credential-issuer.example.com/tenant","credential_endpoint":"https://e.example/c#fragment","credential_configurations_supported":{"x":{"format":"f"}}}"#,
        r#"{"credential_issuer":"https://credential-issuer.example.com/tenant","credential_endpoint":"https://e.example/c","credential_configurations_supported":{}}"#,
        r#"{"credential_issuer":"https://credential-issuer.example.com/tenant","credential_endpoint":"https://e.example/c","credential_configurations_supported":{"x":[]}}"#,
        r#"{"credential_issuer":"https://credential-issuer.example.com/tenant","credential_endpoint":"https://e.example/c","credential_configurations_supported":{"x":{}}}"#,
        r#"{"credential_issuer":"https://credential-issuer.example.com/tenant","credential_endpoint":"https://e.example/c","credential_configurations_supported":{"x":{"format":""}}}"#,
    ];
    for json in invalid {
        assert!(metadata(json).is_err(), "unexpectedly accepted {json}");
    }
}

#[test]
fn rejects_invalid_nonce_endpoint_shapes_with_static_errors() {
    for value in ["\"\"", "7", "null", "[]"] {
        let json = core_metadata(
            &format!(r#", "nonce_endpoint":{value}"#),
            r#"{"degree":{"format":"dc+sd-jwt"}}"#,
        );
        let result = metadata(&json);
        assert!(
            matches!(result, Err(CredentialOfferError::InvalidMetadata)),
            "unexpected result for {value}: {result:?}"
        );
    }

    for endpoint in [
        "http://nonce.example/nonce",
        "https://user:secret@nonce.example/nonce",
        "https://nonce.example/nonce#fragment",
        "https://",
        "not-a-uri",
    ] {
        let json = core_metadata(
            &format!(r#", "nonce_endpoint":"{endpoint}""#),
            r#"{"degree":{"format":"dc+sd-jwt"}}"#,
        );
        assert!(matches!(
            metadata(&json),
            Err(CredentialOfferError::UnsafeNonceEndpoint)
        ));
    }
}

#[test]
fn rejects_invalid_deferred_credential_endpoint_shapes_with_static_errors() {
    for value in ["\"\"", "7", "null", "[]"] {
        let json = core_metadata(
            &format!(r#", "deferred_credential_endpoint":{value}"#),
            r#"{"degree":{"format":"dc+sd-jwt"}}"#,
        );
        let result = metadata(&json);
        assert!(
            matches!(result, Err(CredentialOfferError::InvalidMetadata)),
            "unexpected result for {value}: {result:?}"
        );
    }

    for endpoint in [
        "http://issuer.example/deferred",
        "https://user:secret@issuer.example/deferred",
        "https://issuer.example/deferred#fragment",
        "https://",
        "not-a-uri",
    ] {
        let json = core_metadata(
            &format!(r#", "deferred_credential_endpoint":"{endpoint}""#),
            r#"{"degree":{"format":"dc+sd-jwt"}}"#,
        );
        assert!(matches!(
            metadata(&json),
            Err(CredentialOfferError::UnsafeDeferredCredentialEndpoint)
        ));
    }
}

#[test]
fn rejects_invalid_duplicate_and_excessive_authorization_servers() {
    for authorization_servers in [
        r#", "authorization_servers":[]"#,
        r#", "authorization_servers":"https://auth.example.com""#,
        r#", "authorization_servers":["http://auth.example.com"]"#,
        r#", "authorization_servers":["https://auth.example.com?q=1"]"#,
        r#", "authorization_servers":["https://auth.example.com","https://auth.example.com"]"#,
    ] {
        assert!(
            metadata(&core_metadata(
                authorization_servers,
                r#"{"degree":{"format":"dc+sd-jwt"}}"#
            ))
            .is_err()
        );
    }

    let limits =
        CredentialIssuerMetadataLimits::new(4_096, 8, 64, 2_048, 2_048, 2_048, 1, 256, 128, 8)
            .expect("limits");
    let json = core_metadata(
        r#", "authorization_servers":["https://a.example","https://b.example"]"#,
        r#"{"degree":{"format":"dc+sd-jwt"}}"#,
    );
    assert!(matches!(
        CredentialIssuerMetadata::parse(&json, ISSUER, limits),
        Err(CredentialOfferError::TooManyAuthorizationServers)
    ));
}

#[test]
fn duplicate_members_at_any_depth_and_trailing_json_fail_closed() {
    let duplicate_root = format!(
        r#"{{"credential_issuer":"{ISSUER}","credential_issuer":"{ISSUER}","credential_endpoint":"https://e.example/c","credential_configurations_supported":{{"x":{{"format":"f"}}}}}}"#,
    );
    let duplicate_nested = format!(
        r#"{{"credential_issuer":"{ISSUER}","credential_endpoint":"https://e.example/c","credential_configurations_supported":{{"x":{{"format":"f","fu\u0074ure":1,"future":2}}}}}}"#,
    );
    let duplicate_nonce_endpoint = core_metadata(
        r#", "nonce_endpoint":"https://nonce.example/a", "nonce_endpoint":"https://nonce.example/b""#,
        r#"{"x":{"format":"f"}}"#,
    );
    let duplicate_deferred_endpoint = core_metadata(
        r#", "deferred_credential_endpoint":"https://issuer.example/a", "deferred_credential_endpoint":"https://issuer.example/b""#,
        r#"{"x":{"format":"f"}}"#,
    );
    assert!(matches!(
        metadata(&duplicate_nonce_endpoint),
        Err(CredentialOfferError::DuplicateJsonProperty)
    ));
    assert!(matches!(
        metadata(&duplicate_deferred_endpoint),
        Err(CredentialOfferError::DuplicateJsonProperty)
    ));
    for json in [
        duplicate_root,
        duplicate_nested,
        format!("{}[]", core_metadata("", r#"{"x":{"format":"f"}}"#)),
    ] {
        assert!(metadata(&json).is_err());
    }
}

#[test]
fn metadata_limits_are_positive_inspectable_and_exact() {
    assert!(CredentialIssuerMetadataLimits::new(0, 1, 1, 1, 1, 1, 1, 1, 1, 1).is_err());
    assert!(CredentialIssuerMetadataLimits::new(1, 65, 1, 1, 1, 1, 1, 1, 1, 1).is_err());
    let defaults = CredentialIssuerMetadataLimits::default();
    assert_eq!(defaults.max_json_bytes(), 131_072);
    assert_eq!(defaults.max_json_depth(), 16);
    assert_eq!(defaults.max_json_nodes(), 1_024);
    assert_eq!(defaults.max_credential_issuer_bytes(), 2_048);
    assert_eq!(defaults.max_credential_endpoint_bytes(), 2_048);
    assert_eq!(defaults.max_authorization_server_bytes(), 2_048);
    assert_eq!(defaults.max_authorization_servers(), 16);
    assert_eq!(defaults.max_credential_configuration_id_bytes(), 256);
    assert_eq!(defaults.max_credential_format_bytes(), 128);
    assert_eq!(defaults.max_credential_configurations(), 128);
    let json = core_metadata(
        r#", "authorization_servers":["https://a.example"]"#,
        r#"{"degree":{"format":"fmt"}}"#,
    );
    let exact =
        CredentialIssuerMetadataLimits::new(json.len(), 3, 8, ISSUER.len(), 70, 17, 1, 6, 3, 1)
            .expect("exact limits");
    let parsed = CredentialIssuerMetadata::parse(&json, ISSUER, exact).expect("exact metadata");
    assert_eq!(parsed.as_json().len(), exact.max_json_bytes());
    assert_eq!(exact.max_json_depth(), 3);
    assert_eq!(exact.max_authorization_servers(), 1);
    assert_eq!(exact.max_credential_configurations(), 1);

    let too_small =
        CredentialIssuerMetadataLimits::new(json.len() - 1, 3, 8, ISSUER.len(), 70, 17, 1, 6, 3, 1)
            .expect("limits");
    assert!(matches!(
        CredentialIssuerMetadata::parse(&json, ISSUER, too_small),
        Err(CredentialOfferError::MetadataTooLarge)
    ));

    let one_less_cases = [
        CredentialIssuerMetadataLimits::new(json.len(), 2, 8, ISSUER.len(), 70, 17, 1, 6, 3, 1)
            .expect("limits"),
        CredentialIssuerMetadataLimits::new(json.len(), 3, 7, ISSUER.len(), 70, 17, 1, 6, 3, 1)
            .expect("limits"),
        CredentialIssuerMetadataLimits::new(json.len(), 3, 8, ISSUER.len() - 1, 70, 17, 1, 6, 3, 1)
            .expect("limits"),
        CredentialIssuerMetadataLimits::new(json.len(), 3, 8, ISSUER.len(), 69, 17, 1, 6, 3, 1)
            .expect("limits"),
        CredentialIssuerMetadataLimits::new(json.len(), 3, 8, ISSUER.len(), 70, 16, 1, 6, 3, 1)
            .expect("limits"),
        CredentialIssuerMetadataLimits::new(json.len(), 3, 8, ISSUER.len(), 70, 17, 1, 5, 3, 1)
            .expect("limits"),
        CredentialIssuerMetadataLimits::new(json.len(), 3, 8, ISSUER.len(), 70, 17, 1, 6, 2, 1)
            .expect("limits"),
    ];
    for limits in one_less_cases {
        assert!(CredentialIssuerMetadata::parse(&json, ISSUER, limits).is_err());
    }

    let two_server_json = core_metadata(
        r#", "authorization_servers":["https://a.example","https://b.example"]"#,
        r#"{"degree":{"format":"fmt"}}"#,
    );
    let one_server_limit = CredentialIssuerMetadataLimits::new(
        two_server_json.len(),
        3,
        9,
        ISSUER.len(),
        70,
        17,
        1,
        6,
        3,
        1,
    )
    .expect("limits");
    assert!(matches!(
        CredentialIssuerMetadata::parse(&two_server_json, ISSUER, one_server_limit),
        Err(CredentialOfferError::TooManyAuthorizationServers)
    ));

    let two_configuration_json = core_metadata(
        "",
        r#"{"degree":{"format":"fmt"},"other":{"format":"fmt"}}"#,
    );
    let one_configuration_limit = CredentialIssuerMetadataLimits::new(
        two_configuration_json.len(),
        3,
        9,
        ISSUER.len(),
        70,
        17,
        1,
        6,
        3,
        1,
    )
    .expect("limits");
    assert!(matches!(
        CredentialIssuerMetadata::parse(&two_configuration_json, ISSUER, one_configuration_limit),
        Err(CredentialOfferError::TooManyCredentialConfigurations)
    ));
}

#[test]
fn shared_endpoint_limit_applies_independently_and_exactly() {
    let nonce_endpoint = format!("https://nonce.example/{}", "n".repeat(64));
    let json = core_metadata(
        &format!(r#", "nonce_endpoint":"{nonce_endpoint}""#),
        r#"{"degree":{"format":"fmt"}}"#,
    );
    let exact = CredentialIssuerMetadataLimits::new(
        json.len(),
        3,
        8,
        ISSUER.len(),
        nonce_endpoint.len(),
        17,
        1,
        6,
        3,
        1,
    )
    .expect("limits");
    let parsed = CredentialIssuerMetadata::parse(&json, ISSUER, exact).expect("exact endpoint");
    assert_eq!(
        parsed.nonce_endpoint().map(|endpoint| endpoint.as_str()),
        Some(nonce_endpoint.as_str())
    );

    let one_less = CredentialIssuerMetadataLimits::new(
        json.len(),
        3,
        8,
        ISSUER.len(),
        nonce_endpoint.len() - 1,
        17,
        1,
        6,
        3,
        1,
    )
    .expect("limits");
    assert!(matches!(
        CredentialIssuerMetadata::parse(&json, ISSUER, one_less),
        Err(CredentialOfferError::NonceEndpointTooLarge)
    ));
}

#[test]
fn shared_endpoint_limit_applies_to_deferred_endpoint_independently_and_exactly() {
    let endpoint = format!("https://issuer.example/deferred/{}", "d".repeat(64));
    let json = core_metadata(
        &format!(r#", "deferred_credential_endpoint":"{endpoint}""#),
        r#"{"degree":{"format":"fmt"}}"#,
    );
    let exact = CredentialIssuerMetadataLimits::new(
        json.len(),
        3,
        8,
        ISSUER.len(),
        endpoint.len(),
        17,
        1,
        6,
        3,
        1,
    )
    .expect("limits");
    let parsed = CredentialIssuerMetadata::parse(&json, ISSUER, exact).expect("exact endpoint");
    assert_eq!(
        parsed
            .deferred_credential_endpoint()
            .map(|value| value.as_str()),
        Some(endpoint.as_str())
    );

    let one_less = CredentialIssuerMetadataLimits::new(
        json.len(),
        3,
        8,
        ISSUER.len(),
        endpoint.len() - 1,
        17,
        1,
        6,
        3,
        1,
    )
    .expect("limits");
    assert!(matches!(
        CredentialIssuerMetadata::parse(&json, ISSUER, one_less),
        Err(CredentialOfferError::DeferredCredentialEndpointTooLarge)
    ));
}

#[test]
fn metadata_and_errors_do_not_disclose_caller_content() {
    let canary = "METADATA_SECRET_CANARY_41b4";
    let json = core_metadata(
        r#", "authorization_servers":["https://auth.example.com/a"]"#,
        &format!(r#"{{"{canary}":{{"format":"{canary}","future":"{canary}"}}}}"#),
    );
    let parsed = metadata(&json).expect("metadata");
    assert!(!format!("{parsed:?}").contains(canary));
    assert!(!format!("{:?}", parsed.credential_configurations()[0]).contains(canary));
    assert!(!format!("{:?}", parsed.credential_configurations()[0].format()).contains(canary));
    assert!(!format!("{:?}", parsed.credential_endpoint()).contains(canary));

    let nonce_url = format!("https://nonce.example/nonce?canary={canary}");
    let nonce_json = core_metadata(
        &format!(r#", "nonce_endpoint":"{nonce_url}""#),
        r#"{"degree":{"format":"dc+sd-jwt"}}"#,
    );
    let nonce_metadata = metadata(&nonce_json).expect("metadata with nonce endpoint");
    assert!(!format!("{nonce_metadata:?}").contains(canary));
    assert!(
        !format!(
            "{:?}",
            nonce_metadata.nonce_endpoint().expect("nonce endpoint")
        )
        .contains(canary)
    );

    let error =
        CredentialIssuerMetadata::parse(canary, ISSUER, CredentialIssuerMetadataLimits::default())
            .expect_err("invalid metadata");
    assert!(!format!("{error:?} {error}").contains(canary));
    assert!(!format!("{:?}", error.to_identus_error()).contains(canary));

    let unsafe_json = core_metadata(
        &format!(r#", "nonce_endpoint":"http://nonce.example/{canary}""#),
        r#"{"degree":{"format":"dc+sd-jwt"}}"#,
    );
    let unsafe_error = metadata(&unsafe_json).expect_err("unsafe nonce endpoint");
    assert!(matches!(
        unsafe_error,
        CredentialOfferError::UnsafeNonceEndpoint
    ));
    assert!(!format!("{unsafe_error:?} {unsafe_error}").contains(canary));
    assert!(!format!("{:?}", unsafe_error.to_identus_error()).contains(canary));

    let deferred_url = format!("https://issuer.example/deferred?canary={canary}");
    let deferred_json = core_metadata(
        &format!(r#", "deferred_credential_endpoint":"{deferred_url}""#),
        r#"{"degree":{"format":"dc+sd-jwt"}}"#,
    );
    let deferred_metadata = metadata(&deferred_json).expect("metadata with deferred endpoint");
    assert!(!format!("{deferred_metadata:?}").contains(canary));
    assert!(
        !format!(
            "{:?}",
            deferred_metadata
                .deferred_credential_endpoint()
                .expect("deferred endpoint")
        )
        .contains(canary)
    );

    let unsafe_deferred_json = core_metadata(
        &format!(r#", "deferred_credential_endpoint":"http://issuer.example/{canary}""#),
        r#"{"degree":{"format":"dc+sd-jwt"}}"#,
    );
    let unsafe_deferred = metadata(&unsafe_deferred_json).expect_err("unsafe deferred endpoint");
    assert_eq!(
        unsafe_deferred,
        CredentialOfferError::UnsafeDeferredCredentialEndpoint
    );
    assert!(!format!("{unsafe_deferred:?} {unsafe_deferred}").contains(canary));

    for (error, code) in [
        (
            CredentialOfferError::NonceEndpointTooLarge,
            error_code::NONCE_ENDPOINT_TOO_LARGE,
        ),
        (
            CredentialOfferError::UnsafeNonceEndpoint,
            error_code::UNSAFE_NONCE_ENDPOINT,
        ),
        (
            CredentialOfferError::DeferredCredentialEndpointTooLarge,
            error_code::DEFERRED_CREDENTIAL_ENDPOINT_TOO_LARGE,
        ),
        (
            CredentialOfferError::UnsafeDeferredCredentialEndpoint,
            error_code::UNSAFE_DEFERRED_CREDENTIAL_ENDPOINT,
        ),
    ] {
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
            assert!(!value.contains(canary));
            assert!(!value.contains(&nonce_url));
            assert!(!value.contains(&deferred_url));
        }
    }
}
