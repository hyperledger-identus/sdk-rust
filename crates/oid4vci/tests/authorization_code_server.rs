use std::collections::BTreeSet;

use identus_core::{ErrorKind, IdentusError};
use identus_oid4vci::{
    AUTHORIZATION_CODE_GRANT_TYPE, AuthorizationServerMetadataCore,
    AuthorizationServerMetadataLimits, CAPABILITY, CredentialIssuerMetadata,
    CredentialIssuerMetadataLimits, CredentialOffer, CredentialOfferError,
    CredentialOfferGrantLimits, CredentialOfferLimits, CredentialOfferSemanticLimits,
    CredentialOfferWithMetadata, EmbeddedCredentialOffer, error_code,
};

const ISSUER: &str = "https://credential-issuer.example/tenant";
const SERVER_A: &str = "https://authorization.example/a";
const SERVER_B: &str = "https://authorization.example/b";

fn matched_offer(
    grants: &str,
    authorization_servers: Option<&[&str]>,
) -> CredentialOfferWithMetadata {
    let offer_json = format!(
        r#"{{"credential_issuer":"{ISSUER}","credential_configuration_ids":["degree"],"grants":{grants}}}"#,
    );
    let offer = CredentialOffer::try_from_embedded(
        EmbeddedCredentialOffer::try_from_json(&offer_json, CredentialOfferLimits::default())
            .expect("offer transport"),
        CredentialOfferSemanticLimits::default(),
    )
    .expect("offer semantics")
    .try_into_grants(CredentialOfferGrantLimits::default())
    .expect("offer grants");

    let servers = authorization_servers.map_or_else(String::new, |servers| {
        format!(
            r#","authorization_servers":[{}]"#,
            servers
                .iter()
                .map(|server| format!(r#""{server}""#))
                .collect::<Vec<_>>()
                .join(",")
        )
    });
    let metadata_json = format!(
        r#"{{"credential_issuer":"{ISSUER}"{servers},"credential_endpoint":"https://credential-issuer.example/credential","credential_configurations_supported":{{"degree":{{"format":"dc+sd-jwt"}}}}}}"#,
    );
    let metadata = CredentialIssuerMetadata::parse(
        &metadata_json,
        ISSUER,
        CredentialIssuerMetadataLimits::default(),
    )
    .expect("issuer metadata");
    offer.try_with_metadata(metadata).expect("matched metadata")
}

fn authorization_code_grant(hint: Option<&str>, issuer_state: Option<&str>) -> String {
    let issuer_state =
        issuer_state.map_or_else(String::new, |state| format!(r#""issuer_state":"{state}""#));
    let hint = hint.map_or_else(String::new, |server| {
        let separator = if issuer_state.is_empty() { "" } else { "," };
        format!(r#"{separator}"authorization_server":"{server}""#)
    });
    format!(r#"{{"authorization_code":{{{issuer_state}{hint}}}}}"#)
}

fn server_metadata(
    issuer: &str,
    grants: Option<&[&str]>,
    authorization_endpoint: Option<&str>,
    token_endpoint: Option<&str>,
) -> AuthorizationServerMetadataCore {
    let grants = grants.map_or_else(String::new, |grants| {
        format!(
            r#","grant_types_supported":[{}]"#,
            grants
                .iter()
                .map(|grant| format!(r#""{grant}""#))
                .collect::<Vec<_>>()
                .join(",")
        )
    });
    let authorization_endpoint = authorization_endpoint.map_or_else(String::new, |endpoint| {
        format!(r#","authorization_endpoint":"{endpoint}""#)
    });
    let token_endpoint = token_endpoint.map_or_else(String::new, |endpoint| {
        format!(r#","token_endpoint":"{endpoint}""#)
    });
    AuthorizationServerMetadataCore::parse(
        &format!(r#"{{"issuer":"{issuer}"{grants}{authorization_endpoint}{token_endpoint}}}"#),
        issuer,
        AuthorizationServerMetadataLimits::default(),
    )
    .expect("Authorization Server Metadata core")
}

#[test]
fn binds_defaulted_single_server_preserves_issuer_state_and_needs_no_token_endpoint() {
    let issuer_state = "ISSUER_STATE_PRIVATE_350";
    let matched = matched_offer(&authorization_code_grant(None, Some(issuer_state)), None);
    let selected = server_metadata(
        ISSUER,
        None,
        Some("https://credential-issuer.example/authorize"),
        None,
    );

    let bound = matched
        .try_with_authorization_code_server(selected)
        .expect("defaulted authorization-code server");
    assert_eq!(
        bound
            .credential_offer_with_metadata()
            .credential_offer()
            .authorization_code()
            .expect("grant")
            .issuer_state()
            .expect("issuer state")
            .as_str(),
        issuer_state
    );
    assert_eq!(
        bound
            .authorization_server_metadata()
            .authorization_endpoint()
            .expect("Authorization Endpoint")
            .as_str(),
        "https://credential-issuer.example/authorize"
    );
    assert!(
        bound
            .authorization_server_metadata()
            .token_endpoint()
            .is_none()
    );

    let debug = format!("{bound:?}");
    assert!(!debug.contains(issuer_state));
    assert!(!debug.contains(ISSUER));
    assert!(!debug.contains("authorize"));
}

#[test]
fn caller_can_select_an_exact_listed_server_with_explicit_support() {
    let matched = matched_offer(
        &authorization_code_grant(None, None),
        Some(&[SERVER_A, SERVER_B]),
    );
    let selected = server_metadata(
        SERVER_B,
        Some(&["future", AUTHORIZATION_CODE_GRANT_TYPE]),
        Some("https://authorization.example/authorize"),
        Some("https://authorization.example/token"),
    );

    let bound = matched
        .try_with_authorization_code_server(selected)
        .expect("caller-selected listed server");
    assert_eq!(
        bound.authorization_server_metadata().issuer().as_str(),
        SERVER_B
    );
}

#[test]
fn exact_authorization_code_hint_selects_the_matching_listed_server() {
    let matched = matched_offer(
        &authorization_code_grant(Some(SERVER_B), None),
        Some(&[SERVER_A, SERVER_B]),
    );
    let selected = server_metadata(
        SERVER_B,
        Some(&[AUTHORIZATION_CODE_GRANT_TYPE]),
        Some("https://authorization.example/authorize"),
        None,
    );

    assert!(matched.try_with_authorization_code_server(selected).is_ok());
}

#[test]
fn missing_grant_fails_before_server_capabilities() {
    let matched = matched_offer(
        r#"{"urn:ietf:params:oauth:grant-type:pre-authorized_code":{"pre-authorized_code":"secret"}}"#,
        None,
    );
    let selected = server_metadata(ISSUER, Some(&["future"]), None, None);

    assert!(matches!(
        matched.try_with_authorization_code_server(selected),
        Err(CredentialOfferError::AuthorizationCodeGrantMissing)
    ));
}

#[test]
fn selected_server_must_be_effectively_advertised() {
    let matched = matched_offer(
        &authorization_code_grant(None, None),
        Some(&[SERVER_A, SERVER_B]),
    );
    let selected = server_metadata(
        "https://authorization.example/unlisted",
        Some(&[AUTHORIZATION_CODE_GRANT_TYPE]),
        Some("https://authorization.example/authorize"),
        None,
    );

    assert!(matches!(
        matched.try_with_authorization_code_server(selected),
        Err(CredentialOfferError::AuthorizationServerNotAdvertised)
    ));
}

#[test]
fn selected_server_must_equal_a_present_authorization_code_hint() {
    let matched = matched_offer(
        &authorization_code_grant(Some(SERVER_A), None),
        Some(&[SERVER_A, SERVER_B]),
    );
    let selected = server_metadata(
        SERVER_B,
        Some(&[AUTHORIZATION_CODE_GRANT_TYPE]),
        Some("https://authorization.example/authorize"),
        None,
    );

    assert!(matches!(
        matched.try_with_authorization_code_server(selected),
        Err(CredentialOfferError::AuthorizationCodeServerHintMismatch)
    ));
}

#[test]
fn explicit_unsupported_grant_fails_before_missing_endpoint() {
    let matched = matched_offer(&authorization_code_grant(None, None), None);
    let selected = server_metadata(ISSUER, Some(&["future"]), None, None);

    assert!(matches!(
        matched.try_with_authorization_code_server(selected),
        Err(CredentialOfferError::AuthorizationCodeGrantNotSupported)
    ));
}

#[test]
fn supported_grant_requires_an_authorization_endpoint() {
    let matched = matched_offer(&authorization_code_grant(None, None), None);
    let selected = server_metadata(
        ISSUER,
        Some(&[AUTHORIZATION_CODE_GRANT_TYPE]),
        None,
        Some("https://credential-issuer.example/token"),
    );

    assert!(matches!(
        matched.try_with_authorization_code_server(selected),
        Err(CredentialOfferError::AuthorizationEndpointRequired)
    ));
}

#[test]
fn new_diagnostics_are_static_unique_redacted_contracts() {
    let cases = [
        (
            CredentialOfferError::AuthorizationCodeGrantMissing,
            error_code::AUTHORIZATION_CODE_GRANT_MISSING,
            "OID4VCI Credential Offer has no Authorization Code grant",
        ),
        (
            CredentialOfferError::AuthorizationCodeServerHintMismatch,
            error_code::AUTHORIZATION_CODE_SERVER_HINT_MISMATCH,
            "OID4VCI selected Authorization Server does not match the Authorization Code hint",
        ),
        (
            CredentialOfferError::AuthorizationCodeGrantNotSupported,
            error_code::AUTHORIZATION_CODE_GRANT_NOT_SUPPORTED,
            "OID4VCI selected Authorization Server does not support the Authorization Code grant",
        ),
        (
            CredentialOfferError::AuthorizationEndpointRequired,
            error_code::AUTHORIZATION_ENDPOINT_REQUIRED,
            "OID4VCI selected Authorization Server has no Authorization Endpoint",
        ),
    ];

    let mut codes = BTreeSet::new();
    for (error, code, message) in cases {
        assert!(codes.insert(code.as_str()));
        let public: IdentusError = error.into();
        assert_eq!(public.code(), code);
        assert_eq!(public.kind(), ErrorKind::InvalidInput);
        assert_eq!(public.capability(), Some(CAPABILITY));
        assert_eq!(error.to_string(), message);
        assert_eq!(public.public_message(), message);
        assert_eq!(public, error.to_identus_error());
        assert!(std::error::Error::source(&error).is_none());
        assert!(std::error::Error::source(&public).is_none());
        assert!(!public.to_string().contains("PRIVATE_350"));
        assert!(!error.to_string().contains("PRIVATE_350"));
    }
}
