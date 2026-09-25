use std::collections::BTreeSet;

use identus_core::{ErrorKind, IdentusError};
use identus_oid4vci::{
    AUTHORIZATION_REQUEST_HTTP_METHOD, AuthorizationRequestInputLimits, AuthorizationRequestLimits,
    AuthorizationServerMetadataCore, AuthorizationServerMetadataLimits, CAPABILITY,
    CredentialIssuerMetadata, CredentialIssuerMetadataLimits, CredentialOffer,
    CredentialOfferError, CredentialOfferGrantLimits, CredentialOfferLimits,
    CredentialOfferSemanticLimits, EmbeddedCredentialOffer, error_code,
};

const ISSUER: &str = "https://credential-issuer.example/tenant";
const AUTHORIZATION_SERVER: &str = "https://authorization.example";
const CONFIGURATION: &str = "UniversityDegreeCredential";
const CLIENT_ID: &str = "s6BhdRkqt3";
const REDIRECT_URI: &str = "https://wallet.example.org/cb";
const STATE: &str = "xyz";
const VERIFIER: &str = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
const CHALLENGE: &str = "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM";
const ISSUER_STATE: &str = "issuer state+?";

fn prepare(
    endpoint: &str,
    explicit_authorization_server: bool,
    issuer_state: Option<&str>,
) -> identus_oid4vci::CredentialOfferWithAuthorizationRequestInput {
    let server = if explicit_authorization_server {
        AUTHORIZATION_SERVER
    } else {
        ISSUER
    };
    let mut authorization_code = serde_json::Map::new();
    if let Some(value) = issuer_state {
        authorization_code.insert("issuer_state".into(), value.into());
    }
    let offer_json = serde_json::json!({
        "credential_issuer": ISSUER,
        "credential_configuration_ids": [CONFIGURATION],
        "grants": {"authorization_code": authorization_code},
    })
    .to_string();
    let offer = CredentialOffer::try_from_embedded(
        EmbeddedCredentialOffer::try_from_json(&offer_json, CredentialOfferLimits::default())
            .expect("offer transport"),
        CredentialOfferSemanticLimits::default(),
    )
    .expect("offer semantics")
    .try_into_grants(CredentialOfferGrantLimits::default())
    .expect("offer grants");
    let mut metadata_value = serde_json::json!({
        "credential_issuer": ISSUER,
        "credential_endpoint": format!("{ISSUER}/credential"),
        "credential_configurations_supported": {
            CONFIGURATION: {"format": "dc+sd-jwt"},
        },
    });
    if explicit_authorization_server {
        metadata_value
            .as_object_mut()
            .expect("metadata object")
            .insert(
                "authorization_servers".into(),
                serde_json::json!([AUTHORIZATION_SERVER]),
            );
    }
    let metadata_json = metadata_value.to_string();
    let metadata = CredentialIssuerMetadata::parse(
        &metadata_json,
        ISSUER,
        CredentialIssuerMetadataLimits::default(),
    )
    .expect("issuer metadata");
    let matched = offer.try_with_metadata(metadata).expect("matched metadata");
    let server_metadata = AuthorizationServerMetadataCore::parse(
        &format!(r#"{{"issuer":"{server}","authorization_endpoint":"{endpoint}"}}"#),
        server,
        AuthorizationServerMetadataLimits::default(),
    )
    .expect("Authorization Server Metadata");
    matched
        .try_with_authorization_code_server(server_metadata)
        .expect("server-bound offer")
        .try_with_authorization_request_input(
            0,
            CLIENT_ID,
            REDIRECT_URI,
            STATE,
            VERIFIER,
            AuthorizationRequestInputLimits::default(),
        )
        .expect("request input")
}

fn construct(
    endpoint: &str,
    explicit_authorization_server: bool,
    issuer_state: Option<&str>,
    limits: AuthorizationRequestLimits,
) -> Result<identus_oid4vci::AuthorizationRequest, CredentialOfferError> {
    prepare(endpoint, explicit_authorization_server, issuer_state)
        .try_into_authorization_request(limits)
}

#[test]
fn constructs_exact_authorization_details_request_in_fixed_order() {
    let request = construct(
        &format!("{ISSUER}/authorize"),
        false,
        Some(ISSUER_STATE),
        AuthorizationRequestLimits::default(),
    )
    .expect("Authorization Request");
    let expected = format!(
        "{ISSUER}/authorize?response_type=code&client_id={CLIENT_ID}&redirect_uri=https%3A%2F%2Fwallet.example.org%2Fcb&state={STATE}&code_challenge={CHALLENGE}&code_challenge_method=S256&authorization_details=%5B%7B%22type%22%3A%22openid_credential%22%2C%22credential_configuration_id%22%3A%22UniversityDegreeCredential%22%7D%5D&issuer_state=issuer+state%2B%3F"
    );

    assert_eq!(request.http_method(), AUTHORIZATION_REQUEST_HTTP_METHOD);
    assert_eq!(request.expose_sensitive_request_uri(), expected);
    assert_eq!(request.request_uri_len(), expected.len());
    assert!(request.issuer_state_present());
    assert_eq!(
        request
            .authorization_request_input()
            .code_verifier()
            .as_str(),
        VERIFIER
    );
}

#[test]
fn explicit_authorization_servers_add_exact_issuer_location() {
    let request = construct(
        &format!("{AUTHORIZATION_SERVER}/authorize"),
        true,
        None,
        AuthorizationRequestLimits::default(),
    )
    .expect("delegated Authorization Request");

    assert!(request.expose_sensitive_request_uri().contains(
        "authorization_details=%5B%7B%22type%22%3A%22openid_credential%22%2C%22locations%22%3A%5B%22https%3A%2F%2Fcredential-issuer.example%2Ftenant%22%5D%2C%22credential_configuration_id%22%3A%22UniversityDegreeCredential%22%7D%5D"
    ));
    assert!(!request.expose_sensitive_request_uri().contains("scope="));
    assert!(!request.expose_sensitive_request_uri().contains("resource="));
    assert!(!request.issuer_state_present());
}

#[test]
fn preserves_safe_existing_endpoint_query_bytes_before_managed_parameters() {
    let endpoint = format!("{ISSUER}/authorize?tenant=a%20b&display=touch+screen&flag");
    let request = construct(
        &endpoint,
        false,
        None,
        AuthorizationRequestLimits::default(),
    )
    .expect("query-preserving request");

    assert!(
        request
            .expose_sensitive_request_uri()
            .starts_with(&format!("{endpoint}&response_type=code&client_id="))
    );
}

#[test]
fn rejects_literal_encoded_duplicate_and_alternate_intent_collisions() {
    for query in [
        "state=existing",
        "%73tate=existing",
        "x=1&x=2",
        "x=1&%78=2",
        "scope=credential",
        "resource=https%3A%2F%2Fresource.example",
        "request=jwt",
        "request_uri=urn%3Arequest",
    ] {
        assert!(matches!(
            construct(
                &format!("{ISSUER}/authorize?{query}"),
                false,
                None,
                AuthorizationRequestLimits::default(),
            ),
            Err(CredentialOfferError::AuthorizationEndpointQueryParameterCollision)
        ));
    }
}

#[test]
fn endpoint_query_count_and_component_limits_are_exact() {
    let exact = AuthorizationRequestLimits::new(4_096, 2, 4, 5, 16_384).expect("limits");
    construct(
        &format!("{ISSUER}/authorize?name=value&x=1"),
        false,
        None,
        exact,
    )
    .expect("exact endpoint query limits");

    let one_parameter = AuthorizationRequestLimits::new(4_096, 1, 4, 5, 16_384).expect("limits");
    assert!(matches!(
        construct(
            &format!("{ISSUER}/authorize?a=1&b=2"),
            false,
            None,
            one_parameter,
        ),
        Err(CredentialOfferError::TooManyAuthorizationEndpointQueryParameters)
    ));
    assert!(matches!(
        construct(
            &format!("{ISSUER}/authorize?names=value"),
            false,
            None,
            exact,
        ),
        Err(CredentialOfferError::AuthorizationEndpointQueryComponentTooLarge)
    ));
    assert!(matches!(
        construct(
            &format!("{ISSUER}/authorize?name=values"),
            false,
            None,
            exact,
        ),
        Err(CredentialOfferError::AuthorizationEndpointQueryComponentTooLarge)
    ));
}

#[test]
fn positive_limits_and_exact_output_boundaries_are_enforced() {
    for limits in [
        AuthorizationRequestLimits::new(0, 1, 1, 1, 1),
        AuthorizationRequestLimits::new(1, 0, 1, 1, 1),
        AuthorizationRequestLimits::new(1, 1, 0, 1, 1),
        AuthorizationRequestLimits::new(1, 1, 1, 0, 1),
        AuthorizationRequestLimits::new(1, 1, 1, 1, 0),
    ] {
        assert!(matches!(
            limits,
            Err(CredentialOfferError::InvalidAuthorizationRequestLimits)
        ));
    }

    let defaults = AuthorizationRequestLimits::default();
    let request =
        construct(&format!("{ISSUER}/authorize"), false, None, defaults).expect("default request");
    let request_len = request.request_uri_len();
    let details = r#"[{"type":"openid_credential","credential_configuration_id":"UniversityDegreeCredential"}]"#;

    let exact = AuthorizationRequestLimits::new(details.len(), 1, 1, 1, request_len)
        .expect("exact output limits");
    construct(&format!("{ISSUER}/authorize"), false, None, exact).expect("exact request boundary");
    assert_eq!(exact.max_authorization_details_bytes(), details.len());
    assert_eq!(exact.max_endpoint_query_parameters(), 1);
    assert_eq!(exact.max_endpoint_query_name_bytes(), 1);
    assert_eq!(exact.max_endpoint_query_value_bytes(), 1);
    assert_eq!(exact.max_request_uri_bytes(), request_len);

    let details_short =
        AuthorizationRequestLimits::new(details.len() - 1, 1, 1, 1, 16_384).expect("details bound");
    assert!(matches!(
        construct(&format!("{ISSUER}/authorize"), false, None, details_short,),
        Err(CredentialOfferError::AuthorizationDetailsTooLarge)
    ));
    let uri_short =
        AuthorizationRequestLimits::new(4_096, 1, 1, 1, request_len - 1).expect("URI bound");
    assert!(matches!(
        construct(&format!("{ISSUER}/authorize"), false, None, uri_short,),
        Err(CredentialOfferError::AuthorizationRequestUriTooLarge)
    ));
}

#[test]
fn debug_and_errors_are_static_and_redacted() {
    let request = construct(
        &format!("{ISSUER}/authorize"),
        false,
        Some(ISSUER_STATE),
        AuthorizationRequestLimits::default(),
    )
    .expect("request");
    let debug = format!("{request:?}");
    for canary in [STATE, VERIFIER, ISSUER_STATE, CLIENT_ID, ISSUER] {
        assert!(!debug.contains(canary));
    }

    let contracts = [
        (
            CredentialOfferError::InvalidAuthorizationRequestLimits,
            error_code::INVALID_AUTHORIZATION_REQUEST_LIMITS,
            "OID4VCI Authorization Request limits are invalid",
        ),
        (
            CredentialOfferError::AuthorizationDetailsTooLarge,
            error_code::AUTHORIZATION_DETAILS_TOO_LARGE,
            "OID4VCI Authorization Details are too large",
        ),
        (
            CredentialOfferError::TooManyAuthorizationEndpointQueryParameters,
            error_code::TOO_MANY_AUTHORIZATION_ENDPOINT_QUERY_PARAMETERS,
            "OID4VCI Authorization Endpoint has too many query parameters",
        ),
        (
            CredentialOfferError::AuthorizationEndpointQueryComponentTooLarge,
            error_code::AUTHORIZATION_ENDPOINT_QUERY_COMPONENT_TOO_LARGE,
            "OID4VCI Authorization Endpoint query component is too large",
        ),
        (
            CredentialOfferError::InvalidAuthorizationEndpointQuery,
            error_code::INVALID_AUTHORIZATION_ENDPOINT_QUERY,
            "OID4VCI Authorization Endpoint query is invalid",
        ),
        (
            CredentialOfferError::AuthorizationEndpointQueryParameterCollision,
            error_code::AUTHORIZATION_ENDPOINT_QUERY_PARAMETER_COLLISION,
            "OID4VCI Authorization Endpoint query parameter collides with request construction",
        ),
        (
            CredentialOfferError::AuthorizationRequestUriTooLarge,
            error_code::AUTHORIZATION_REQUEST_URI_TOO_LARGE,
            "OID4VCI Authorization Request URI is too large",
        ),
    ];
    let mut unique_codes = BTreeSet::new();
    for (error, code, message) in contracts {
        let public: IdentusError = error.into();
        assert_eq!(public.code(), code);
        assert_eq!(public.kind(), ErrorKind::InvalidInput);
        assert_eq!(public.capability(), Some(CAPABILITY));
        assert_eq!(error.to_string(), message);
        assert!(unique_codes.insert(code.as_str()));
        for canary in [STATE, VERIFIER, ISSUER_STATE, CLIENT_ID, ISSUER] {
            assert!(!format!("{error:?} {error} {public}").contains(canary));
        }
    }
}
