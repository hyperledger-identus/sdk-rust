use std::collections::BTreeSet;

use identus_core::{ErrorKind, IdentusError};
use identus_oid4vci::{
    AuthorizationEndpointErrorKind, AuthorizationRequestInputLimits, AuthorizationRequestLimits,
    AuthorizationResponseIssuerIdentification, AuthorizationResponseLimits,
    AuthorizationResponseOutcome, AuthorizationServerMetadataCore,
    AuthorizationServerMetadataLimits, CAPABILITY, CredentialIssuerMetadata,
    CredentialIssuerMetadataLimits, CredentialOffer, CredentialOfferError,
    CredentialOfferGrantLimits, CredentialOfferLimits, CredentialOfferSemanticLimits,
    EmbeddedCredentialOffer, error_code,
};

const ISSUER: &str = "https://credential-issuer.example";
const AUTHORIZATION_SERVER: &str = "https://authorization.example";
const CONFIGURATION: &str = "UniversityDegreeCredential";
const STATE: &str = "state value";
const VERIFIER: &str = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";

fn request(rfc9207: Option<bool>, delegated: bool) -> identus_oid4vci::AuthorizationRequest {
    let selected_server = if delegated {
        AUTHORIZATION_SERVER
    } else {
        ISSUER
    };
    let offer_json = format!(
        r#"{{"credential_issuer":"{ISSUER}","credential_configuration_ids":["{CONFIGURATION}"],"grants":{{"authorization_code":{{}}}}}}"#
    );
    let offer = CredentialOffer::try_from_embedded(
        EmbeddedCredentialOffer::try_from_json(&offer_json, CredentialOfferLimits::default())
            .expect("offer transport"),
        CredentialOfferSemanticLimits::default(),
    )
    .expect("offer semantics")
    .try_into_grants(CredentialOfferGrantLimits::default())
    .expect("offer grants");

    let authorization_servers = if delegated {
        format!(r#","authorization_servers":["{AUTHORIZATION_SERVER}"]"#)
    } else {
        String::new()
    };
    let metadata_json = format!(
        r#"{{"credential_issuer":"{ISSUER}"{authorization_servers},"credential_endpoint":"{ISSUER}/credential","credential_configurations_supported":{{"{CONFIGURATION}":{{"format":"dc+sd-jwt"}}}}}}"#
    );
    let metadata = CredentialIssuerMetadata::parse(
        &metadata_json,
        ISSUER,
        CredentialIssuerMetadataLimits::default(),
    )
    .expect("issuer metadata");
    let matched = offer.try_with_metadata(metadata).expect("matched metadata");

    let issuer_flag = rfc9207.map_or_else(String::new, |value| {
        format!(r#","authorization_response_iss_parameter_supported":{value}"#)
    });
    let server_metadata_json = format!(
        r#"{{"issuer":"{selected_server}","authorization_endpoint":"{selected_server}/authorize"{issuer_flag}}}"#
    );
    let server_metadata = AuthorizationServerMetadataCore::parse(
        &server_metadata_json,
        selected_server,
        AuthorizationServerMetadataLimits::default(),
    )
    .expect("Authorization Server Metadata");

    matched
        .try_with_authorization_code_server(server_metadata)
        .expect("server-bound offer")
        .try_with_authorization_request_input(
            0,
            "wallet-client",
            "https://wallet.example/callback",
            STATE,
            VERIFIER,
            AuthorizationRequestInputLimits::default(),
        )
        .expect("request input")
        .try_into_authorization_request(AuthorizationRequestLimits::default())
        .expect("request")
}

fn assert_response_error(
    request: identus_oid4vci::AuthorizationRequest,
    query: &str,
    limits: AuthorizationResponseLimits,
    expected: CredentialOfferError,
) {
    assert_eq!(
        request
            .try_into_authorization_response(query, limits)
            .expect_err("response must fail"),
        expected
    );
}

#[test]
fn correlates_success_and_preserves_complete_request_lineage() {
    let outcome = request(None, false)
        .try_into_authorization_response(
            "code=authorization-code&state=state+value&extension=accepted",
            AuthorizationResponseLimits::default(),
        )
        .expect("correlated success");
    let AuthorizationResponseOutcome::Authorized(success) = outcome else {
        panic!("expected authorized outcome");
    };

    assert_eq!(success.expose_sensitive_code(), "authorization-code");
    assert_eq!(
        success.issuer_identification(),
        AuthorizationResponseIssuerIdentification::NotAdvertised
    );
    assert_eq!(
        success
            .authorization_request()
            .authorization_request_input()
            .code_verifier()
            .as_str(),
        VERIFIER
    );
    let debug = format!("{success:?}");
    assert!(!debug.contains("authorization-code"));
    assert!(!debug.contains(VERIFIER));
}

#[test]
fn verifies_the_exact_delegated_server_issuer_when_advertised() {
    let outcome = request(Some(true), true)
        .try_into_authorization_response(
            &format!("state=state+value&iss={AUTHORIZATION_SERVER}&code=code"),
            AuthorizationResponseLimits::default(),
        )
        .expect("RFC 9207 response");
    let AuthorizationResponseOutcome::Authorized(success) = outcome else {
        panic!("expected authorized outcome");
    };
    assert_eq!(
        success.issuer_identification(),
        AuthorizationResponseIssuerIdentification::VerifiedRfc9207
    );
}

#[test]
fn correlates_error_and_classifies_standard_or_extension_codes() {
    let outcome = request(Some(true), false)
        .try_into_authorization_response(
            "error=access_denied&error_description=not+now&error_uri=%2Ferrors%2Fdenied&state=state+value&iss=https%3A%2F%2Fcredential-issuer.example",
            AuthorizationResponseLimits::default(),
        )
        .expect("correlated error");
    let AuthorizationResponseOutcome::Error(error) = outcome else {
        panic!("expected error outcome");
    };
    assert_eq!(error.error().as_str(), "access_denied");
    assert_eq!(
        error.error_kind(),
        AuthorizationEndpointErrorKind::AccessDenied
    );
    assert_eq!(error.expose_untrusted_description(), Some("not now"));
    assert_eq!(error.error_uri().expect("URI").as_str(), "/errors/denied");
    let debug = format!("{error:?}");
    assert!(!debug.contains("not now"));
    assert!(!debug.contains("/errors/denied"));

    let extension = request(None, false)
        .try_into_authorization_response(
            "error=vendor_error&state=state+value",
            AuthorizationResponseLimits::default(),
        )
        .expect("extension error");
    let AuthorizationResponseOutcome::Error(extension) = extension else {
        panic!("expected error outcome");
    };
    assert_eq!(
        extension.error_kind(),
        AuthorizationEndpointErrorKind::Extension
    );
}

#[test]
fn rejects_missing_or_substituted_state_before_exposing_an_outcome() {
    for query in ["code=code", "code=code&state=other", "code=code&state="] {
        assert_response_error(
            request(None, false),
            query,
            AuthorizationResponseLimits::default(),
            CredentialOfferError::AuthorizationResponseStateMismatch,
        );
    }
}

#[test]
fn issuer_policy_fails_closed_for_missing_mismatched_or_unadvertised_values() {
    for query in [
        "code=code&state=state+value",
        "code=code&state=state+value&iss=https%3A%2F%2Fcredential-issuer.example",
    ] {
        assert_response_error(
            request(Some(true), true),
            query,
            AuthorizationResponseLimits::default(),
            CredentialOfferError::AuthorizationResponseIssuerMismatch,
        );
    }
    assert_response_error(
        request(None, false),
        "code=code&state=state+value&iss=https%3A%2F%2Fcredential-issuer.example",
        AuthorizationResponseLimits::default(),
        CredentialOfferError::AuthorizationResponseIssuerMismatch,
    );
}

#[test]
fn rejects_ambiguous_branches_and_developer_fields_without_error() {
    for query in [
        "code=code&error=access_denied&state=state+value",
        "state=state+value",
        "code=code&error_description=no&state=state+value",
        "error_description=no&state=state+value",
    ] {
        assert_response_error(
            request(None, false),
            query,
            AuthorizationResponseLimits::default(),
            CredentialOfferError::InvalidAuthorizationResponse,
        );
    }
}

#[test]
fn rejects_duplicate_decoded_names_malformed_forms_and_full_callback_inputs() {
    let cases = [
        (
            "code=a&%63ode=b&state=state+value",
            CredentialOfferError::DuplicateAuthorizationResponseParameter,
        ),
        (
            "code=%GG&state=state+value",
            CredentialOfferError::InvalidAuthorizationResponseEncoding,
        ),
        (
            "?code=a&state=state+value",
            CredentialOfferError::InvalidAuthorizationResponse,
        ),
        (
            "code=a&state=state+value#fragment",
            CredentialOfferError::InvalidAuthorizationResponse,
        ),
        (
            "=value&state=state+value&code=a",
            CredentialOfferError::InvalidAuthorizationResponse,
        ),
    ];
    for (query, expected) in cases {
        assert_response_error(
            request(None, false),
            query,
            AuthorizationResponseLimits::default(),
            expected,
        );
    }
}

#[test]
fn enforces_independent_query_count_component_and_role_limits() {
    assert_eq!(
        AuthorizationResponseLimits::new(0, 1, 1, 1, 1, 1, 1, 1),
        Err(CredentialOfferError::InvalidAuthorizationResponseLimits)
    );

    let query_limited =
        AuthorizationResponseLimits::new(3, 10, 20, 20, 20, 20, 20, 20).expect("limits");
    assert_response_error(
        request(None, false),
        "code=a&state=state+value",
        query_limited,
        CredentialOfferError::AuthorizationResponseTooLarge,
    );

    let count_limited =
        AuthorizationResponseLimits::new(100, 1, 20, 20, 20, 20, 20, 20).expect("limits");
    assert_response_error(
        request(None, false),
        "code=a&state=state+value",
        count_limited,
        CredentialOfferError::TooManyAuthorizationResponseParameters,
    );

    let code_limited =
        AuthorizationResponseLimits::new(100, 10, 20, 20, 1, 20, 20, 20).expect("limits");
    assert_response_error(
        request(None, false),
        "code=ab&state=state+value",
        code_limited,
        CredentialOfferError::AuthorizationCodeTooLarge,
    );
}

#[test]
fn validates_success_and_error_grammars() {
    for (query, expected) in [
        (
            "code=%0A&state=state+value",
            CredentialOfferError::InvalidAuthorizationCode,
        ),
        (
            "error=bad%22code&state=state+value",
            CredentialOfferError::InvalidAuthorizationEndpointErrorCode,
        ),
        (
            "error=bad&error_description=%0A&state=state+value",
            CredentialOfferError::InvalidAuthorizationErrorDescription,
        ),
        (
            "error=bad&error_uri=http%3A%2F%2F%5B&state=state+value",
            CredentialOfferError::InvalidAuthorizationErrorUri,
        ),
    ] {
        assert_response_error(
            request(None, false),
            query,
            AuthorizationResponseLimits::default(),
            expected,
        );
    }
}

#[test]
fn response_diagnostics_are_static_unique_redacted_contracts() {
    let cases = [
        (
            CredentialOfferError::InvalidAuthorizationResponseLimits,
            error_code::INVALID_AUTHORIZATION_RESPONSE_LIMITS,
            "OID4VCI Authorization Response limits are invalid",
        ),
        (
            CredentialOfferError::AuthorizationResponseTooLarge,
            error_code::AUTHORIZATION_RESPONSE_TOO_LARGE,
            "OID4VCI Authorization Response is too large",
        ),
        (
            CredentialOfferError::TooManyAuthorizationResponseParameters,
            error_code::TOO_MANY_AUTHORIZATION_RESPONSE_PARAMETERS,
            "OID4VCI Authorization Response has too many parameters",
        ),
        (
            CredentialOfferError::AuthorizationResponseComponentTooLarge,
            error_code::AUTHORIZATION_RESPONSE_COMPONENT_TOO_LARGE,
            "OID4VCI Authorization Response component is too large",
        ),
        (
            CredentialOfferError::InvalidAuthorizationResponseEncoding,
            error_code::INVALID_AUTHORIZATION_RESPONSE_ENCODING,
            "OID4VCI Authorization Response form encoding is invalid",
        ),
        (
            CredentialOfferError::DuplicateAuthorizationResponseParameter,
            error_code::DUPLICATE_AUTHORIZATION_RESPONSE_PARAMETER,
            "OID4VCI Authorization Response has a duplicate parameter",
        ),
        (
            CredentialOfferError::AuthorizationResponseStateMismatch,
            error_code::AUTHORIZATION_RESPONSE_STATE_MISMATCH,
            "OID4VCI Authorization Response state does not match the request",
        ),
        (
            CredentialOfferError::AuthorizationResponseIssuerMismatch,
            error_code::AUTHORIZATION_RESPONSE_ISSUER_MISMATCH,
            "OID4VCI Authorization Response issuer does not match selected-server policy",
        ),
        (
            CredentialOfferError::InvalidAuthorizationResponse,
            error_code::INVALID_AUTHORIZATION_RESPONSE,
            "OID4VCI Authorization Response is invalid",
        ),
        (
            CredentialOfferError::AuthorizationCodeTooLarge,
            error_code::AUTHORIZATION_CODE_TOO_LARGE,
            "OID4VCI authorization code is too large",
        ),
        (
            CredentialOfferError::InvalidAuthorizationCode,
            error_code::INVALID_AUTHORIZATION_CODE,
            "OID4VCI authorization code is invalid",
        ),
        (
            CredentialOfferError::AuthorizationEndpointErrorCodeTooLarge,
            error_code::AUTHORIZATION_ENDPOINT_ERROR_CODE_TOO_LARGE,
            "OID4VCI Authorization Endpoint error code is too large",
        ),
        (
            CredentialOfferError::InvalidAuthorizationEndpointErrorCode,
            error_code::INVALID_AUTHORIZATION_ENDPOINT_ERROR_CODE,
            "OID4VCI Authorization Endpoint error code is invalid",
        ),
        (
            CredentialOfferError::AuthorizationErrorDescriptionTooLarge,
            error_code::AUTHORIZATION_ERROR_DESCRIPTION_TOO_LARGE,
            "OID4VCI Authorization Response error description is too large",
        ),
        (
            CredentialOfferError::InvalidAuthorizationErrorDescription,
            error_code::INVALID_AUTHORIZATION_ERROR_DESCRIPTION,
            "OID4VCI Authorization Response error description is invalid",
        ),
        (
            CredentialOfferError::AuthorizationErrorUriTooLarge,
            error_code::AUTHORIZATION_ERROR_URI_TOO_LARGE,
            "OID4VCI Authorization Response error URI is too large",
        ),
        (
            CredentialOfferError::InvalidAuthorizationErrorUri,
            error_code::INVALID_AUTHORIZATION_ERROR_URI,
            "OID4VCI Authorization Response error URI is invalid",
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
        assert!(!format!("{error:?}{public:?}").contains("PRIVATE_356"));
    }
}
