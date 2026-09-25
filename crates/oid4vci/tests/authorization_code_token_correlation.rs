use std::error::Error as _;

use identus_core::{ErrorKind, IdentusError};
use identus_oid4vci::{
    AuthorizationCodeTokenHttpResponseLimits, AuthorizationCodeTokenRequestLimits,
    AuthorizationCodeTokenResponseOutcome, AuthorizationRequestInputLimits,
    AuthorizationRequestLimits, AuthorizationResponseIssuerIdentification,
    AuthorizationResponseLimits, AuthorizationResponseOutcome, AuthorizationServerMetadataCore,
    AuthorizationServerMetadataLimits, CAPABILITY, CorrelatedAuthorizationCodeTokenResponse,
    CredentialIssuerMetadata, CredentialIssuerMetadataLimits, CredentialOffer,
    CredentialOfferError, CredentialOfferGrantLimits, CredentialOfferLimits,
    CredentialOfferSemanticLimits, EmbeddedCredentialOffer, TokenAuthorizationDetailsLimits,
    error_code,
};

const ISSUER: &str = "https://credential-issuer.example";
const CONFIGURATION: &str = "UniversityDegreeCredential";
const TOKEN_ENDPOINT: &str = "https://authorization.example/token";
const STATE: &str = "state-value";
const VERIFIER: &str = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
const TOKEN: &str = "TOKEN_CANARY_1c20";

fn bound_success(body: &str) -> identus_oid4vci::RequestBoundAuthorizationCodeTokenResponse {
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
    let issuer_metadata_json = format!(
        r#"{{"credential_issuer":"{ISSUER}","credential_endpoint":"{ISSUER}/credential","credential_configurations_supported":{{"{CONFIGURATION}":{{"format":"dc+sd-jwt"}}}}}}"#
    );
    let issuer_metadata = CredentialIssuerMetadata::parse(
        &issuer_metadata_json,
        ISSUER,
        CredentialIssuerMetadataLimits::default(),
    )
    .expect("issuer metadata");
    let server_metadata_json = format!(
        r#"{{"issuer":"{ISSUER}","authorization_endpoint":"{ISSUER}/authorize","token_endpoint":"{TOKEN_ENDPOINT}","authorization_response_iss_parameter_supported":true}}"#
    );
    let server_metadata = AuthorizationServerMetadataCore::parse(
        &server_metadata_json,
        ISSUER,
        AuthorizationServerMetadataLimits::default(),
    )
    .expect("server metadata");
    let request = offer
        .try_with_metadata(issuer_metadata)
        .expect("matched metadata")
        .try_with_authorization_code_server(server_metadata)
        .expect("server bound")
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
        .expect("authorization request");
    let AuthorizationResponseOutcome::Authorized(code) = request
        .try_into_authorization_response(
            &format!(
                "code=authorization-code&state={STATE}&iss=https%3A%2F%2Fcredential-issuer.example"
            ),
            AuthorizationResponseLimits::default(),
        )
        .expect("authorization response")
    else {
        panic!("expected authorization code");
    };
    let request = code
        .try_into_public_client_token_request(AuthorizationCodeTokenRequestLimits::default())
        .expect("token request");
    let AuthorizationCodeTokenResponseOutcome::Success(bound) = request
        .try_bind_response(
            200,
            "application/json",
            "no-store",
            "no-cache",
            body,
            AuthorizationCodeTokenHttpResponseLimits::default(),
        )
        .expect("bound response")
    else {
        panic!("expected success response");
    };
    bound
}

fn response(details: &str) -> String {
    format!(
        r#"{{"access_token":"{TOKEN}","token_type":"Bearer","authorization_details":[{details}]}}"#
    )
}

fn detail(configuration: &str, identifiers: &str) -> String {
    format!(
        r#"{{"type":"openid_credential","credential_configuration_id":"{configuration}","credential_identifiers":[{identifiers}]}}"#
    )
}

fn correlate(body: &str) -> Result<CorrelatedAuthorizationCodeTokenResponse, CredentialOfferError> {
    bound_success(body)
        .try_correlate_authorization_details(TokenAuthorizationDetailsLimits::default())
}

#[test]
fn exact_configuration_advances_with_source_order_and_lineage() {
    let first = "DATASET_CANARY_FIRST_21c9";
    let second = "DATASET_CANARY_SECOND_7ea4";
    let body = response(&format!(
        "{},{}",
        detail(CONFIGURATION, &format!(r#""{first}","{second}""#)),
        r#"{"type":"example_extension","nested":{"allowed":true}}"#
    ));
    let correlated = correlate(&body).expect("exact correlation");

    assert_eq!(correlated.authorized_credential_identifier_count(), 2);
    assert_eq!(
        correlated
            .authorized_credential_identifiers()
            .collect::<Vec<_>>(),
        vec![first, second]
    );
    assert_eq!(correlated.unknown_authorization_detail_count(), 1);
    assert_eq!(
        correlated
            .token_response_core()
            .expose_sensitive_access_token(),
        TOKEN
    );
    assert_eq!(
        correlated
            .lineage()
            .selected_credential_configuration()
            .as_str(),
        CONFIGURATION
    );
    assert_eq!(
        correlated.lineage().issuer_identification(),
        AuthorizationResponseIssuerIdentification::VerifiedRfc9207
    );

    let debug = format!("{correlated:?}");
    for canary in [first, second, TOKEN, CONFIGURATION, ISSUER, TOKEN_ENDPOINT] {
        assert!(!debug.contains(canary));
    }
}

#[test]
fn missing_and_unknown_only_details_fail_in_the_existing_parser() {
    let missing = format!(r#"{{"access_token":"{TOKEN}","token_type":"Bearer"}}"#);
    assert_eq!(
        correlate(&missing).expect_err("missing details"),
        CredentialOfferError::InvalidTokenAuthorizationDetails
    );
    assert_eq!(
        correlate(&response(r#"{"type":"example_extension"}"#)).expect_err("unknown-only details"),
        CredentialOfferError::InvalidTokenAuthorizationDetails
    );
}

#[test]
fn any_configuration_mismatch_precedes_ambiguity_in_both_source_orders() {
    let matching = detail(CONFIGURATION, r#""matching-id""#);
    let mismatching = detail("UNREQUESTED_CONFIGURATION_CANARY", r#""other-id""#);
    for details in [
        format!("{matching},{mismatching}"),
        format!("{mismatching},{matching}"),
    ] {
        assert_eq!(
            correlate(&response(&details)).expect_err("configuration expansion"),
            CredentialOfferError::AuthorizationCodeTokenConfigurationMismatch
        );
    }
}

#[test]
fn repeated_matching_entries_are_ambiguous() {
    let first = detail(CONFIGURATION, r#""first-id""#);
    let second = detail(CONFIGURATION, r#""second-id""#);
    assert_eq!(
        correlate(&response(&format!("{first},{second}")))
            .expect_err("repeated recognized details"),
        CredentialOfferError::AmbiguousAuthorizationCodeTokenAuthorizationDetails
    );
}

#[test]
fn response_local_limits_remain_authoritative() {
    let body = response(&detail(CONFIGURATION, r#""identifier""#));
    let limits = TokenAuthorizationDetailsLimits::new(1, 17, CONFIGURATION.len() - 1, 1, 10)
        .expect("positive narrow limits");
    assert_eq!(
        bound_success(&body)
            .try_correlate_authorization_details(limits)
            .expect_err("configuration is one byte over"),
        CredentialOfferError::TokenAuthorizationDetailValueTooLarge
    );
}

#[test]
fn correlation_error_contracts_are_static_and_redacted() {
    let cases = [
        (
            CredentialOfferError::AuthorizationCodeTokenConfigurationMismatch,
            error_code::AUTHORIZATION_CODE_TOKEN_CONFIGURATION_MISMATCH,
            "OID4VCI Authorization Code Token Response configuration does not match the request",
        ),
        (
            CredentialOfferError::AmbiguousAuthorizationCodeTokenAuthorizationDetails,
            error_code::AMBIGUOUS_AUTHORIZATION_CODE_TOKEN_AUTHORIZATION_DETAILS,
            "OID4VCI Authorization Code Token Response Authorization Details are ambiguous",
        ),
    ];
    for (error, code, message) in cases {
        let public: IdentusError = error.into();
        assert_eq!(public.code(), code);
        assert_eq!(public.kind(), ErrorKind::InvalidInput);
        assert_eq!(public.capability(), Some(CAPABILITY));
        assert_eq!(error.to_string(), message);
        assert_eq!(public.public_message(), message);
        assert!(error.source().is_none());
        for canary in [TOKEN, CONFIGURATION, ISSUER, TOKEN_ENDPOINT] {
            assert!(!format!("{error:?}").contains(canary));
            assert!(!public.to_string().contains(canary));
        }
    }
}
