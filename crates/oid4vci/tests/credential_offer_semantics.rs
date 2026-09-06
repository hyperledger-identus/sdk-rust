use identus_core::{ErrorKind, IdentusError};
use identus_oid4vci::{
    CAPABILITY, CredentialOffer, CredentialOfferError, CredentialOfferLimits,
    CredentialOfferRequest, CredentialOfferSemanticLimits, EmbeddedCredentialOffer, error_code,
};

fn embedded(json: &str) -> EmbeddedCredentialOffer {
    EmbeddedCredentialOffer::try_from_json(json, CredentialOfferLimits::default())
        .expect("test JSON should pass transport validation")
}

fn semantic(json: &str) -> Result<CredentialOffer, CredentialOfferError> {
    CredentialOffer::try_from_embedded(embedded(json), CredentialOfferSemanticLimits::default())
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
fn accepts_final_example_and_preserves_exact_order_and_json() {
    // OpenID4VCI 1.0 Final section 4.1.1, reconstructed without display
    // whitespace and with the published example values unchanged.
    let json = r#"{"credential_issuer":"https://credential-issuer.example.com","credential_configuration_ids":["UniversityDegreeCredential","org.iso.18013.5.1.mDL"],"grants":{"urn:ietf:params:oauth:grant-type:pre-authorized_code":{"pre-authorized_code":"oaKazRN8I0IbtZ0C7JuMn5","tx_code":{"length":4,"input_mode":"numeric","description":"Please provide the one-time code that was sent via e-mail"}}}}"#;
    let offer = semantic(json).expect("Final Credential Offer should validate");

    assert_eq!(
        offer.credential_issuer().as_str(),
        "https://credential-issuer.example.com"
    );
    assert_eq!(
        offer
            .credential_configuration_ids()
            .iter()
            .map(|id| id.as_str())
            .collect::<Vec<_>>(),
        ["UniversityDegreeCredential", "org.iso.18013.5.1.mDL"]
    );
    assert!(offer.grants_present());
    assert_eq!(offer.as_json(), json);
}

#[test]
fn accepts_consumer_shapes_empty_id_and_escaped_strings() {
    let oxid = semantic(
        r#"{"credential_issuer":"https://issuer.example/tenant","credential_configuration_ids":["ExampleCredential"],"oxid_extension":{"phase":1}}"#,
    )
    .expect("Oxid-shaped offer should validate");
    assert!(!oxid.grants_present());

    let lace = semantic(
        r#"{"credential_configuration_ids":["University\u0044egree",""],"credential_issuer":"HTTPS://issuer.example:8443/path","grants":{}}"#,
    )
    .expect("Lace-shaped object should validate independently of URI transport");
    assert_eq!(
        lace.credential_issuer().as_str(),
        "HTTPS://issuer.example:8443/path"
    );
    assert_eq!(
        lace.credential_configuration_ids()[0].as_str(),
        "UniversityDegree"
    );
    assert_eq!(lace.credential_configuration_ids()[1].as_str(), "");
    assert!(lace.grants_present());
}

#[test]
fn invocation_and_direct_transport_reach_equivalent_semantics() {
    let json = r#"{"credential_issuer":"https://issuer.example","credential_configuration_ids":["Example"]}"#;
    let direct = semantic(json).expect("direct semantic transition");
    let invocation = format!(
        "openid-credential-offer://?credential_offer={}",
        encode_form(json)
    );
    let parsed = CredentialOfferRequest::parse(&invocation, CredentialOfferLimits::default())
        .expect("invocation transport");
    let CredentialOfferRequest::Embedded(transport) = parsed else {
        panic!("expected embedded transport");
    };
    let through_invocation =
        CredentialOffer::try_from_embedded(transport, CredentialOfferSemanticLimits::default())
            .expect("invocation semantic transition");

    assert_eq!(
        direct.credential_issuer().as_str(),
        through_invocation.credential_issuer().as_str()
    );
    assert_eq!(direct.as_json(), through_invocation.as_json());
}

#[test]
fn rejects_missing_or_wrong_required_fields() {
    let cases = [
        (
            r#"{"credential_configuration_ids":["A"]}"#,
            CredentialOfferError::InvalidOfferFields,
        ),
        (
            r#"{"credential_issuer":"https://issuer.example"}"#,
            CredentialOfferError::InvalidConfigurationIds,
        ),
        (
            r#"{"credential_issuer":1,"credential_configuration_ids":["A"]}"#,
            CredentialOfferError::InvalidOfferFields,
        ),
        (
            r#"{"credential_issuer":"https://issuer.example","credential_configuration_ids":"A"}"#,
            CredentialOfferError::InvalidConfigurationIds,
        ),
        (
            r#"{"credential_issuer":"https://issuer.example","credential_configuration_ids":[]}"#,
            CredentialOfferError::InvalidConfigurationIds,
        ),
        (
            r#"{"credential_issuer":"https://issuer.example","credential_configuration_ids":[1]}"#,
            CredentialOfferError::InvalidConfigurationIds,
        ),
    ];

    for (json, expected) in cases {
        assert_eq!(
            semantic(json).expect_err("invalid fields must fail"),
            expected
        );
    }
}

#[test]
fn validates_issuer_identifier_syntax_without_claiming_trust() {
    for issuer in [
        "relative",
        "http://issuer.example",
        "https:/issuer.example",
        "https://",
        "https://:443/path",
        "https://user@issuer.example",
        "https://user:password@issuer.example",
        "https://issuer.example?query=1",
        "https://issuer.example#fragment",
    ] {
        let json =
            format!(r#"{{"credential_issuer":"{issuer}","credential_configuration_ids":["A"]}}"#);
        assert_eq!(
            semantic(&json).expect_err("unsafe issuer must fail"),
            CredentialOfferError::UnsafeCredentialIssuer,
            "unexpected result for {issuer}"
        );
    }
}

#[test]
fn accepts_rfc_3986_ipvfuture_issuer_hosts() {
    let issuer = "https://[v1.fe80]:8443/tenant";
    let json =
        format!(r#"{{"credential_issuer":"{issuer}","credential_configuration_ids":["A"]}}"#);
    let offer = semantic(&json).expect("RFC 3986 IPvFuture host should validate");
    assert_eq!(offer.credential_issuer().as_str(), issuer);

    for issuer in [
        "https://[v1.]",
        "https://[v.fe80]",
        "https://[vG.fe80]",
        "https://[v1.fe80]suffix",
        "https://[v1.fe80]:invalid",
    ] {
        let json =
            format!(r#"{{"credential_issuer":"{issuer}","credential_configuration_ids":["A"]}}"#);
        assert_eq!(
            semantic(&json).expect_err("malformed IPvFuture issuer must fail"),
            CredentialOfferError::UnsafeCredentialIssuer,
            "unexpected result for {issuer}"
        );
    }
}

#[test]
fn configuration_ids_are_unique_after_json_decoding() {
    assert_eq!(
        semantic(
            r#"{"credential_issuer":"https://issuer.example","credential_configuration_ids":["same","\u0073ame"]}"#
        )
        .expect_err("decoded duplicate IDs must fail"),
        CredentialOfferError::DuplicateConfigurationId
    );
}

#[test]
fn grants_are_optional_opaque_objects_and_extensions_are_lossless() {
    for grants in [None, Some("{}"), Some(r#"{"future":{"value":true}}"#)] {
        let grants_member =
            grants.map_or_else(String::new, |value| format!(r#","grants":{value}"#));
        let json = format!(
            r#"{{"credential_issuer":"https://issuer.example","credential_configuration_ids":["A"],"future_number":1e400,"future_object":{{"secret":"extension-canary"}}{grants_member}}}"#
        );
        let offer = semantic(&json).expect("opaque extensions and object grants should survive");
        assert_eq!(offer.grants_present(), grants.is_some());
        assert_eq!(offer.as_json(), json);
    }

    for grants in ["null", "true", "1", r#""value""#, "[]"] {
        let json = format!(
            r#"{{"credential_issuer":"https://issuer.example","credential_configuration_ids":["A"],"grants":{grants}}}"#
        );
        assert_eq!(
            semantic(&json).expect_err("non-object grants must fail"),
            CredentialOfferError::InvalidGrants
        );
    }
}

#[test]
fn semantic_limits_are_positive_inspectable_and_exact() {
    let defaults = CredentialOfferSemanticLimits::default();
    assert_eq!(defaults.max_credential_issuer_bytes(), 2_048);
    assert_eq!(defaults.max_credential_configuration_id_bytes(), 256);
    assert_eq!(defaults.max_credential_configuration_ids(), 32);

    for fields in [(0, 1, 1), (1, 0, 1), (1, 1, 0)] {
        assert_eq!(
            CredentialOfferSemanticLimits::new(fields.0, fields.1, fields.2),
            Err(CredentialOfferError::InvalidSemanticLimits)
        );
    }

    let issuer = "https://issuer.example";
    let json =
        format!(r#"{{"credential_issuer":"{issuer}","credential_configuration_ids":["é"]}}"#);
    let exact =
        CredentialOfferSemanticLimits::new(issuer.len(), "é".len(), 1).expect("exact limits");
    assert!(CredentialOffer::try_from_embedded(embedded(&json), exact).is_ok());

    let short_issuer =
        CredentialOfferSemanticLimits::new(issuer.len() - 1, 2, 1).expect("short issuer limit");
    assert_eq!(
        CredentialOffer::try_from_embedded(embedded(&json), short_issuer)
            .expect_err("one excess issuer byte must fail"),
        CredentialOfferError::IssuerTooLarge
    );

    let short_id = CredentialOfferSemanticLimits::new(issuer.len(), 1, 1).expect("short ID limit");
    assert_eq!(
        CredentialOffer::try_from_embedded(embedded(&json), short_id)
            .expect_err("one excess decoded ID byte must fail"),
        CredentialOfferError::ConfigurationIdTooLarge
    );

    let two_ids =
        format!(r#"{{"credential_issuer":"{issuer}","credential_configuration_ids":["A","B"]}}"#);
    assert_eq!(
        CredentialOffer::try_from_embedded(embedded(&two_ids), exact)
            .expect_err("one excess ID must fail"),
        CredentialOfferError::TooManyConfigurationIds
    );
}

#[test]
fn semantic_values_and_errors_are_redaction_safe() {
    let canaries = [
        "issuer-canary",
        "configuration-canary",
        "grant-canary",
        "extension-canary",
    ];
    let json = r#"{"credential_issuer":"https://issuer-canary.example","credential_configuration_ids":["configuration-canary"],"grants":{"grant-canary":{}},"extension":"extension-canary"}"#.to_owned();
    let offer = semantic(&json).expect("canary offer should validate");
    for rendered in [
        format!("{offer:?}"),
        format!("{:?}", offer.credential_issuer()),
        format!("{:?}", offer.credential_configuration_ids()[0]),
    ] {
        for canary in canaries {
            assert!(!rendered.contains(canary));
        }
    }

    let cases = [
        (
            CredentialOfferError::InvalidSemanticLimits,
            error_code::INVALID_SEMANTIC_LIMITS,
        ),
        (
            CredentialOfferError::InvalidOfferFields,
            error_code::INVALID_OFFER_FIELDS,
        ),
        (
            CredentialOfferError::IssuerTooLarge,
            error_code::ISSUER_TOO_LARGE,
        ),
        (
            CredentialOfferError::UnsafeCredentialIssuer,
            error_code::UNSAFE_CREDENTIAL_ISSUER,
        ),
        (
            CredentialOfferError::InvalidConfigurationIds,
            error_code::INVALID_CONFIGURATION_IDS,
        ),
        (
            CredentialOfferError::ConfigurationIdTooLarge,
            error_code::CONFIGURATION_ID_TOO_LARGE,
        ),
        (
            CredentialOfferError::TooManyConfigurationIds,
            error_code::TOO_MANY_CONFIGURATION_IDS,
        ),
        (
            CredentialOfferError::DuplicateConfigurationId,
            error_code::DUPLICATE_CONFIGURATION_ID,
        ),
        (
            CredentialOfferError::InvalidGrants,
            error_code::INVALID_GRANTS,
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
