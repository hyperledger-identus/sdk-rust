use std::collections::BTreeSet;

use identus_core::{ErrorKind, IdentusError};
use identus_oid4vci::{
    AuthorizationRequestInputLimits, AuthorizationServerMetadataCore,
    AuthorizationServerMetadataLimits, CAPABILITY, CredentialIssuerMetadata,
    CredentialIssuerMetadataLimits, CredentialOffer, CredentialOfferError,
    CredentialOfferGrantLimits, CredentialOfferLimits, CredentialOfferSemanticLimits,
    CredentialOfferWithAuthorizationCodeServer, EmbeddedCredentialOffer, PKCE_S256_METHOD,
    error_code,
};

const ISSUER: &str = "https://credential-issuer.example/tenant";
const ISSUER_STATE: &str = "ISSUER_STATE_PRIVATE_352";
const CLIENT_ID: &str = "wallet-client";
const REDIRECT_URI: &str = "com.example.wallet:/callback";
const STATE: &str = "STATE_PRIVATE_352";
const VERIFIER: &str = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
const CHALLENGE: &str = "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM";

fn server_bound_offer() -> CredentialOfferWithAuthorizationCodeServer {
    let offer_json = format!(
        r#"{{"credential_issuer":"{ISSUER}","credential_configuration_ids":["degree","employee"],"grants":{{"authorization_code":{{"issuer_state":"{ISSUER_STATE}"}}}}}}"#,
    );
    let offer = CredentialOffer::try_from_embedded(
        EmbeddedCredentialOffer::try_from_json(&offer_json, CredentialOfferLimits::default())
            .expect("offer transport"),
        CredentialOfferSemanticLimits::default(),
    )
    .expect("offer semantics")
    .try_into_grants(CredentialOfferGrantLimits::default())
    .expect("offer grants");
    let metadata = CredentialIssuerMetadata::parse(
        &format!(
            r#"{{"credential_issuer":"{ISSUER}","credential_endpoint":"{ISSUER}/credential","credential_configurations_supported":{{"degree":{{"format":"dc+sd-jwt"}},"employee":{{"format":"jwt_vc_json"}}}}}}"#,
        ),
        ISSUER,
        CredentialIssuerMetadataLimits::default(),
    )
    .expect("issuer metadata");
    let matched = offer.try_with_metadata(metadata).expect("matched metadata");
    let server = AuthorizationServerMetadataCore::parse(
        &format!(r#"{{"issuer":"{ISSUER}","authorization_endpoint":"{ISSUER}/authorize"}}"#),
        ISSUER,
        AuthorizationServerMetadataLimits::default(),
    )
    .expect("Authorization Server Metadata");
    matched
        .try_with_authorization_code_server(server)
        .expect("server-bound offer")
}

fn prepare(
    configuration_index: usize,
    client_id: &str,
    redirect_uri: &str,
    state: &str,
    verifier: &str,
    limits: AuthorizationRequestInputLimits,
) -> Result<identus_oid4vci::CredentialOfferWithAuthorizationRequestInput, CredentialOfferError> {
    server_bound_offer().try_with_authorization_request_input(
        configuration_index,
        client_id,
        redirect_uri,
        state,
        verifier,
        limits,
    )
}

#[test]
fn prepares_owned_inputs_and_preserves_selected_offer_state() {
    let prepared = prepare(
        1,
        CLIENT_ID,
        REDIRECT_URI,
        STATE,
        VERIFIER,
        AuthorizationRequestInputLimits::default(),
    )
    .expect("prepared Authorization Request inputs");

    assert_eq!(
        prepared.selected_credential_configuration().as_str(),
        "employee"
    );
    assert_eq!(prepared.client_id().as_str(), CLIENT_ID);
    assert_eq!(prepared.redirect_uri().as_str(), REDIRECT_URI);
    assert_eq!(prepared.state().as_str(), STATE);
    assert_eq!(prepared.code_verifier().as_str(), VERIFIER);
    assert_eq!(prepared.code_challenge().as_str(), CHALLENGE);
    assert_eq!(prepared.code_challenge_method(), PKCE_S256_METHOD);
    assert_eq!(
        prepared
            .credential_offer_with_authorization_code_server()
            .credential_offer_with_metadata()
            .credential_offer()
            .authorization_code()
            .expect("Authorization Code grant")
            .issuer_state()
            .expect("issuer state")
            .as_str(),
        ISSUER_STATE
    );
}

#[test]
fn rfc_7636_appendix_b_vector_matches_exactly() {
    let prepared = prepare(
        0,
        CLIENT_ID,
        REDIRECT_URI,
        STATE,
        VERIFIER,
        AuthorizationRequestInputLimits::default(),
    )
    .expect("RFC vector");

    assert_eq!(prepared.code_challenge().as_str(), CHALLENGE);
    assert_eq!(prepared.code_challenge_method(), "S256");
}

#[test]
fn configuration_selection_precedes_caller_input_validation() {
    assert!(matches!(
        prepare(
            2,
            &"x".repeat(4_096),
            "relative",
            "",
            "short",
            AuthorizationRequestInputLimits::default(),
        ),
        Err(CredentialOfferError::AuthorizationRequestConfigurationMissing)
    ));
}

#[test]
fn limits_are_positive_and_exact_boundaries_are_accepted() {
    assert!(matches!(
        AuthorizationRequestInputLimits::new(0, 1, 1),
        Err(CredentialOfferError::InvalidAuthorizationRequestInputLimits)
    ));
    assert!(matches!(
        AuthorizationRequestInputLimits::new(1, 0, 1),
        Err(CredentialOfferError::InvalidAuthorizationRequestInputLimits)
    ));
    assert!(matches!(
        AuthorizationRequestInputLimits::new(1, 1, 0),
        Err(CredentialOfferError::InvalidAuthorizationRequestInputLimits)
    ));

    let limits =
        AuthorizationRequestInputLimits::new(CLIENT_ID.len(), REDIRECT_URI.len(), STATE.len())
            .expect("exact limits");
    let prepared =
        prepare(0, CLIENT_ID, REDIRECT_URI, STATE, VERIFIER, limits).expect("exact boundaries");
    assert_eq!(prepared.client_id().as_str(), CLIENT_ID);
    assert_eq!(limits.max_client_id_bytes(), CLIENT_ID.len());
    assert_eq!(limits.max_redirect_uri_bytes(), REDIRECT_URI.len());
    assert_eq!(limits.max_state_bytes(), STATE.len());
}

#[test]
fn size_errors_precede_syntax_errors() {
    let limits = AuthorizationRequestInputLimits::new(3, 3, 3).expect("limits");
    assert!(matches!(
        prepare(0, "\nBAD", REDIRECT_URI, STATE, VERIFIER, limits),
        Err(CredentialOfferError::AuthorizationRequestClientIdTooLarge)
    ));
    assert!(matches!(
        prepare(0, "abc", "bad#fragment", STATE, VERIFIER, limits),
        Err(CredentialOfferError::AuthorizationRequestRedirectUriTooLarge)
    ));
    assert!(matches!(
        prepare(0, "abc", "x:/", "\nBAD", VERIFIER, limits),
        Err(CredentialOfferError::AuthorizationRequestStateTooLarge)
    ));
}

#[test]
fn client_identifier_and_state_require_nonempty_visible_ascii() {
    for client_id in ["", "line\nbreak", "é"] {
        assert!(matches!(
            prepare(
                0,
                client_id,
                REDIRECT_URI,
                STATE,
                VERIFIER,
                AuthorizationRequestInputLimits::default(),
            ),
            Err(CredentialOfferError::InvalidAuthorizationRequestClientId)
        ));
    }
    for state in ["", "line\nbreak", "é"] {
        assert!(matches!(
            prepare(
                0,
                CLIENT_ID,
                REDIRECT_URI,
                state,
                VERIFIER,
                AuthorizationRequestInputLimits::default(),
            ),
            Err(CredentialOfferError::InvalidAuthorizationRequestState)
        ));
    }
}

#[test]
fn redirect_uri_accepts_native_shapes_and_rejects_unsafe_structure() {
    for redirect_uri in [
        "https://wallet.example/callback?flow=1",
        "http://127.0.0.1:49152/callback",
        "com.example.wallet:/callback",
    ] {
        let prepared = prepare(
            0,
            CLIENT_ID,
            redirect_uri,
            STATE,
            VERIFIER,
            AuthorizationRequestInputLimits::default(),
        )
        .expect("supported redirect shape");
        assert_eq!(prepared.redirect_uri().as_str(), redirect_uri);
    }

    for redirect_uri in [
        "relative/callback",
        "https://wallet.example/callback#fragment",
        "https://user@wallet.example/callback",
    ] {
        assert!(matches!(
            prepare(
                0,
                CLIENT_ID,
                redirect_uri,
                STATE,
                VERIFIER,
                AuthorizationRequestInputLimits::default(),
            ),
            Err(CredentialOfferError::InvalidAuthorizationRequestRedirectUri)
        ));
    }
}

#[test]
fn verifier_enforces_rfc_length_and_character_boundaries_without_panic() {
    for verifier in ["a".repeat(43), "Z".repeat(128)] {
        assert!(
            prepare(
                0,
                CLIENT_ID,
                REDIRECT_URI,
                STATE,
                &verifier,
                AuthorizationRequestInputLimits::default(),
            )
            .is_ok()
        );
    }
    for verifier in ["a".repeat(42), "!".repeat(43), "é".repeat(43)] {
        assert!(matches!(
            prepare(
                0,
                CLIENT_ID,
                REDIRECT_URI,
                STATE,
                &verifier,
                AuthorizationRequestInputLimits::default(),
            ),
            Err(CredentialOfferError::InvalidPkceCodeVerifier)
        ));
    }
    assert!(matches!(
        prepare(
            0,
            CLIENT_ID,
            REDIRECT_URI,
            STATE,
            &"a".repeat(129),
            AuthorizationRequestInputLimits::default(),
        ),
        Err(CredentialOfferError::PkceCodeVerifierTooLarge)
    ));
}

#[test]
fn retained_inputs_and_aggregate_debug_are_redacted() {
    let prepared = prepare(
        0,
        CLIENT_ID,
        REDIRECT_URI,
        STATE,
        VERIFIER,
        AuthorizationRequestInputLimits::default(),
    )
    .expect("prepared inputs");

    for debug in [
        format!("{prepared:?}"),
        format!("{:?}", prepared.client_id()),
        format!("{:?}", prepared.redirect_uri()),
        format!("{:?}", prepared.state()),
        format!("{:?}", prepared.code_verifier()),
        format!("{:?}", prepared.code_challenge()),
    ] {
        for canary in [
            CLIENT_ID,
            REDIRECT_URI,
            STATE,
            VERIFIER,
            CHALLENGE,
            ISSUER_STATE,
        ] {
            assert!(!debug.contains(canary));
        }
    }
}

#[test]
fn new_diagnostics_are_static_unique_redacted_contracts() {
    let cases = [
        (
            CredentialOfferError::InvalidAuthorizationRequestInputLimits,
            error_code::INVALID_AUTHORIZATION_REQUEST_INPUT_LIMITS,
            "OID4VCI Authorization Request input limits are invalid",
        ),
        (
            CredentialOfferError::AuthorizationRequestConfigurationMissing,
            error_code::AUTHORIZATION_REQUEST_CONFIGURATION_MISSING,
            "OID4VCI Authorization Request Credential Configuration is missing",
        ),
        (
            CredentialOfferError::InvalidAuthorizationRequestClientId,
            error_code::INVALID_AUTHORIZATION_REQUEST_CLIENT_ID,
            "OID4VCI Authorization Request client identifier is invalid",
        ),
        (
            CredentialOfferError::AuthorizationRequestClientIdTooLarge,
            error_code::AUTHORIZATION_REQUEST_CLIENT_ID_TOO_LARGE,
            "OID4VCI Authorization Request client identifier is too large",
        ),
        (
            CredentialOfferError::InvalidAuthorizationRequestRedirectUri,
            error_code::INVALID_AUTHORIZATION_REQUEST_REDIRECT_URI,
            "OID4VCI Authorization Request redirect URI is invalid",
        ),
        (
            CredentialOfferError::AuthorizationRequestRedirectUriTooLarge,
            error_code::AUTHORIZATION_REQUEST_REDIRECT_URI_TOO_LARGE,
            "OID4VCI Authorization Request redirect URI is too large",
        ),
        (
            CredentialOfferError::InvalidAuthorizationRequestState,
            error_code::INVALID_AUTHORIZATION_REQUEST_STATE,
            "OID4VCI Authorization Request state is invalid",
        ),
        (
            CredentialOfferError::AuthorizationRequestStateTooLarge,
            error_code::AUTHORIZATION_REQUEST_STATE_TOO_LARGE,
            "OID4VCI Authorization Request state is too large",
        ),
        (
            CredentialOfferError::InvalidPkceCodeVerifier,
            error_code::INVALID_PKCE_CODE_VERIFIER,
            "OID4VCI PKCE code verifier is invalid",
        ),
        (
            CredentialOfferError::PkceCodeVerifierTooLarge,
            error_code::PKCE_CODE_VERIFIER_TOO_LARGE,
            "OID4VCI PKCE code verifier is too large",
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
        for canary in [CLIENT_ID, REDIRECT_URI, STATE, VERIFIER] {
            assert!(!public.to_string().contains(canary));
            assert!(!error.to_string().contains(canary));
        }
    }
}
