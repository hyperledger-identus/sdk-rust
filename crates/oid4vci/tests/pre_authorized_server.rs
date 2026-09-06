use identus_core::{ErrorKind, IdentusError};
use identus_oid4vci::{
    AuthorizationServerMetadataCore, AuthorizationServerMetadataLimits, CAPABILITY,
    CredentialIssuerMetadata, CredentialIssuerMetadataLimits, CredentialOffer,
    CredentialOfferError, CredentialOfferGrantLimits, CredentialOfferLimits,
    CredentialOfferSemanticLimits, CredentialOfferWithMetadata, EmbeddedCredentialOffer,
    PRE_AUTHORIZED_CODE_GRANT_TYPE, error_code,
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

fn pre_authorized_grant(hint: Option<&str>, code: &str) -> String {
    let hint = hint.map_or_else(String::new, |server| {
        format!(r#", "authorization_server":"{server}""#)
    });
    format!(r#"{{"{PRE_AUTHORIZED_CODE_GRANT_TYPE}":{{"pre-authorized_code":"{code}"{hint}}}}}"#,)
}

fn server_metadata(
    issuer: &str,
    grants: Option<&[&str]>,
    token_endpoint: Option<&str>,
    anonymous_access: Option<bool>,
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
    let token_endpoint = token_endpoint.map_or_else(String::new, |endpoint| {
        format!(r#","token_endpoint":"{endpoint}""#)
    });
    let anonymous_access = anonymous_access.map_or_else(String::new, |value| {
        format!(r#","pre-authorized_grant_anonymous_access_supported":{value}"#)
    });
    AuthorizationServerMetadataCore::parse(
        &format!(r#"{{"issuer":"{issuer}"{grants}{token_endpoint}{anonymous_access}}}"#),
        issuer,
        AuthorizationServerMetadataLimits::default(),
    )
    .expect("Authorization Server Metadata core")
}

#[test]
fn binds_the_effective_single_server_and_preserves_owned_states() {
    let code = "PRE_AUTHORIZED_SECRET_13a0";
    let matched = matched_offer(&pre_authorized_grant(None, code), None);
    let selected = server_metadata(
        ISSUER,
        Some(&[PRE_AUTHORIZED_CODE_GRANT_TYPE]),
        Some("https://credential-issuer.example/token"),
        Some(false),
    );

    let bound = matched
        .try_with_pre_authorized_server(selected)
        .expect("bound server");
    assert_eq!(
        bound
            .credential_offer_with_metadata()
            .credential_offer()
            .pre_authorized_code()
            .expect("grant")
            .pre_authorized_code()
            .as_str(),
        code
    );
    assert_eq!(
        bound.authorization_server_metadata().issuer().as_str(),
        ISSUER
    );
    assert_eq!(
        bound
            .authorization_server_metadata()
            .token_endpoint()
            .expect("Token Endpoint")
            .as_str(),
        "https://credential-issuer.example/token"
    );
    assert_eq!(
        bound
            .authorization_server_metadata()
            .advertised_anonymous_pre_authorized_access(),
        Some(false)
    );
}

#[test]
fn caller_can_select_any_exact_listed_server_when_the_offer_has_no_hint() {
    let matched = matched_offer(
        &pre_authorized_grant(None, "secret"),
        Some(&[SERVER_A, SERVER_B]),
    );
    let selected = server_metadata(
        SERVER_B,
        Some(&["future", PRE_AUTHORIZED_CODE_GRANT_TYPE]),
        Some("https://authorization.example/token"),
        Some(true),
    );

    let bound = matched
        .try_with_pre_authorized_server(selected)
        .expect("caller-selected listed server");
    assert_eq!(
        bound.authorization_server_metadata().issuer().as_str(),
        SERVER_B
    );
    assert!(
        bound
            .authorization_server_metadata()
            .effective_anonymous_pre_authorized_access()
    );
}

#[test]
fn exact_pre_authorized_server_hint_selects_the_matching_listed_server() {
    let matched = matched_offer(
        &pre_authorized_grant(Some(SERVER_B), "secret"),
        Some(&[SERVER_A, SERVER_B]),
    );
    let selected = server_metadata(
        SERVER_B,
        Some(&[PRE_AUTHORIZED_CODE_GRANT_TYPE]),
        Some("https://authorization.example/token"),
        None,
    );

    assert!(matched.try_with_pre_authorized_server(selected).is_ok());
}

#[test]
fn missing_pre_authorized_grant_fails_before_server_capabilities() {
    let matched = matched_offer(r#"{"authorization_code":{}}"#, None);
    let selected = server_metadata(ISSUER, None, None, None);

    assert!(matches!(
        matched.try_with_pre_authorized_server(selected),
        Err(CredentialOfferError::PreAuthorizedCodeGrantMissing)
    ));
}

#[test]
fn selected_server_must_be_effectively_advertised() {
    let matched = matched_offer(
        &pre_authorized_grant(None, "secret"),
        Some(&[SERVER_A, SERVER_B]),
    );
    let selected = server_metadata(
        "https://authorization.example/unlisted",
        Some(&[PRE_AUTHORIZED_CODE_GRANT_TYPE]),
        Some("https://authorization.example/token"),
        None,
    );

    assert!(matches!(
        matched.try_with_pre_authorized_server(selected),
        Err(CredentialOfferError::AuthorizationServerNotAdvertised)
    ));

    let matched = matched_offer(&pre_authorized_grant(None, "secret"), None);
    let selected = server_metadata(
        SERVER_A,
        Some(&[PRE_AUTHORIZED_CODE_GRANT_TYPE]),
        Some("https://authorization.example/token"),
        None,
    );
    assert!(matches!(
        matched.try_with_pre_authorized_server(selected),
        Err(CredentialOfferError::AuthorizationServerNotAdvertised)
    ));
}

#[test]
fn selected_server_must_equal_a_present_pre_authorized_hint() {
    let matched = matched_offer(
        &pre_authorized_grant(Some(SERVER_A), "secret"),
        Some(&[SERVER_A, SERVER_B]),
    );
    let selected = server_metadata(
        SERVER_B,
        Some(&[PRE_AUTHORIZED_CODE_GRANT_TYPE]),
        Some("https://authorization.example/token"),
        None,
    );

    assert!(matches!(
        matched.try_with_pre_authorized_server(selected),
        Err(CredentialOfferError::PreAuthorizedServerHintMismatch)
    ));
}

#[test]
fn pre_authorized_grant_must_be_explicitly_supported() {
    for grants in [None, Some(&["authorization_code"][..])] {
        let matched = matched_offer(&pre_authorized_grant(None, "secret"), None);
        let selected = server_metadata(
            ISSUER,
            grants,
            Some("https://credential-issuer.example/token"),
            None,
        );
        assert!(matches!(
            matched.try_with_pre_authorized_server(selected),
            Err(CredentialOfferError::PreAuthorizedGrantNotSupported)
        ));
    }
}

#[test]
fn selected_server_must_expose_a_token_endpoint() {
    let matched = matched_offer(&pre_authorized_grant(None, "secret"), None);
    let selected = server_metadata(ISSUER, Some(&[PRE_AUTHORIZED_CODE_GRANT_TYPE]), None, None);

    assert!(matches!(
        matched.try_with_pre_authorized_server(selected),
        Err(CredentialOfferError::TokenEndpointRequired)
    ));
}

#[test]
fn new_states_and_errors_have_static_redacted_diagnostics() {
    let canary = "PRE_AUTHORIZED_BINDING_SECRET_CANARY_4b91";
    let matched = matched_offer(&pre_authorized_grant(None, canary), None);
    let selected = server_metadata(
        ISSUER,
        Some(&[PRE_AUTHORIZED_CODE_GRANT_TYPE]),
        Some(&format!("https://credential-issuer.example/{canary}")),
        None,
    );
    let bound = matched
        .try_with_pre_authorized_server(selected)
        .expect("bound server");
    assert!(!format!("{bound:?}").contains(canary));

    let cases = [
        (
            CredentialOfferError::PreAuthorizedCodeGrantMissing,
            error_code::PRE_AUTHORIZED_CODE_GRANT_MISSING,
        ),
        (
            CredentialOfferError::AuthorizationServerNotAdvertised,
            error_code::AUTHORIZATION_SERVER_NOT_ADVERTISED,
        ),
        (
            CredentialOfferError::PreAuthorizedServerHintMismatch,
            error_code::PRE_AUTHORIZED_SERVER_HINT_MISMATCH,
        ),
        (
            CredentialOfferError::PreAuthorizedGrantNotSupported,
            error_code::PRE_AUTHORIZED_GRANT_NOT_SUPPORTED,
        ),
        (
            CredentialOfferError::TokenEndpointRequired,
            error_code::TOKEN_ENDPOINT_REQUIRED,
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
            assert!(!rendered.contains(canary));
        }
    }
}
