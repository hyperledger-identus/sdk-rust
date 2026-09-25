use identus_jose::{
    JwsAlgorithm, JwsKeyReference, JwsSigner, Oid4vciProofJwt, Oid4vciProofJwtBuilder,
    Oid4vciProofJwtClaims, Oid4vciProofJwtClient, Oid4vciProofJwtLimits, SignerFailure,
};
use identus_oid4vci::{
    AuthorizationCodeTokenHttpResponseLimits, AuthorizationCodeTokenRequestLimits,
    AuthorizationCodeTokenResponseOutcome, AuthorizationRequestInputLimits,
    AuthorizationRequestLimits, AuthorizationResponseLimits, AuthorizationResponseOutcome,
    AuthorizationServerMetadataCore, AuthorizationServerMetadataLimits, CredentialIssuerMetadata,
    CredentialIssuerMetadataLimits, CredentialOffer, CredentialOfferError,
    CredentialOfferGrantLimits, CredentialOfferLimits, CredentialOfferSemanticLimits,
    EmbeddedCredentialOffer, JwtCredentialRequestLimits, TokenAuthorizationDetailsLimits,
};
use serde_json::json;

const ISSUER: &str = "https://credential-issuer.example";
const ENDPOINT: &str = "https://credential-issuer.example/credential";
const CONFIGURATION: &str = "UniversityDegreeCredential";
const STATE: &str = "state-value";
const VERIFIER: &str = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
const TOKEN: &str = "TOKEN_CANARY_1c20";

struct StubSigner;

impl JwsSigner for StubSigner {
    fn algorithm(&self) -> JwsAlgorithm {
        JwsAlgorithm::Ed25519
    }

    fn sign(&self, _: &[u8]) -> Result<[u8; 64], SignerFailure> {
        Ok([0x5a; 64])
    }
}

fn proof(nonce: &str) -> Oid4vciProofJwt {
    let limits = Oid4vciProofJwtLimits::default();
    let claims = Oid4vciProofJwtClaims::new(
        Oid4vciProofJwtClient::identified("wallet-client", limits).expect("bounded client ID"),
        ISSUER,
        1_700_000_000,
        Some(nonce.to_owned()),
        limits,
    )
    .expect("proof claims");
    Oid4vciProofJwtBuilder::new(limits)
        .prepare(
            JwsAlgorithm::Ed25519,
            JwsKeyReference::key_id("did:example:holder#authentication-1", limits.jws())
                .expect("bounded key ID"),
            claims,
        )
        .expect("proof input")
        .sign_with(&StubSigner)
        .expect("holder-produced proof")
}

fn correlated(
    token_type: &str,
    identifiers: &[&str],
) -> identus_oid4vci::CorrelatedAuthorizationCodeTokenResponse {
    let offer = CredentialOffer::try_from_embedded(
        EmbeddedCredentialOffer::try_from_json(
            &json!({
                "credential_issuer": ISSUER,
                "credential_configuration_ids": [CONFIGURATION],
                "grants": {"authorization_code": {}}
            })
            .to_string(),
            CredentialOfferLimits::default(),
        )
        .expect("offer transport"),
        CredentialOfferSemanticLimits::default(),
    )
    .expect("offer semantics")
    .try_into_grants(CredentialOfferGrantLimits::default())
    .expect("offer grants");
    let issuer_metadata = CredentialIssuerMetadata::parse(
        &json!({
            "credential_issuer": ISSUER,
            "credential_endpoint": ENDPOINT,
            "credential_configurations_supported": {
                CONFIGURATION: {"format": "dc+sd-jwt"}
            }
        })
        .to_string(),
        ISSUER,
        CredentialIssuerMetadataLimits::default(),
    )
    .expect("issuer metadata");
    let server_metadata = AuthorizationServerMetadataCore::parse(
        &json!({
            "issuer": ISSUER,
            "authorization_endpoint": format!("{ISSUER}/authorize"),
            "token_endpoint": format!("{ISSUER}/token"),
            "authorization_response_iss_parameter_supported": true
        })
        .to_string(),
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
    let token_request = code
        .try_into_public_client_token_request(AuthorizationCodeTokenRequestLimits::default())
        .expect("token request");
    let body = json!({
        "access_token": TOKEN,
        "token_type": token_type,
        "authorization_details": [{
            "type": "openid_credential",
            "credential_configuration_id": CONFIGURATION,
            "credential_identifiers": identifiers
        }]
    })
    .to_string();
    let AuthorizationCodeTokenResponseOutcome::Success(bound) = token_request
        .try_bind_response(
            200,
            "application/json",
            "no-store",
            "no-cache",
            &body,
            AuthorizationCodeTokenHttpResponseLimits::default(),
        )
        .expect("bound response")
    else {
        panic!("expected success response");
    };
    bound
        .try_correlate_authorization_details(TokenAuthorizationDetailsLimits::default())
        .expect("correlated response")
}

#[test]
fn consumes_exact_authority_into_owned_source_ordered_request() {
    let first = "DATASET_CANARY_FIRST";
    let selected = "DATASET_\"CANARY\\SECOND";
    let proofs = [proof("nonce-one"), proof("nonce-two")];
    let expected_body = json!({
        "credential_identifier": selected,
        "proofs": {"jwt": [proofs[0].compact(), proofs[1].compact()]}
    })
    .to_string();

    let request = correlated("bEaReR", &[first, selected])
        .try_into_authorized_jwt_credential_request(
            1,
            &proofs,
            JwtCredentialRequestLimits::default(),
        )
        .expect("request-bound Credential Request");

    assert_eq!(request.credential_endpoint().as_str(), ENDPOINT);
    assert_eq!(
        request.expose_sensitive_authorization(),
        format!("Bearer {TOKEN}")
    );
    assert_eq!(request.expose_sensitive_json_body(), expected_body);
    assert!(!request.expose_sensitive_json_body().contains(CONFIGURATION));
    assert_eq!(request.proof_count(), 2);

    drop(proofs);
    assert_eq!(request.credential_endpoint().as_str(), ENDPOINT);
    assert_eq!(request.expose_sensitive_json_body(), expected_body);
    let debug = format!("{request:?}");
    for canary in [TOKEN, selected, CONFIGURATION, ISSUER] {
        assert!(!debug.contains(canary));
    }
}

#[test]
fn missing_identifier_is_static_and_redacted() {
    let error = correlated("Bearer", &["identifier"])
        .try_into_authorized_jwt_credential_request(
            1,
            &[proof("nonce")],
            JwtCredentialRequestLimits::default(),
        )
        .expect_err("out-of-range identifier");
    assert_eq!(
        error,
        CredentialOfferError::CredentialRequestIdentifierMissing
    );
    assert!(!format!("{error:?}").contains(TOKEN));
}

#[test]
fn existing_token_and_resource_limits_remain_authoritative() {
    let dpop_proof = proof("nonce");
    let unsupported = correlated("DPoP", &["identifier"])
        .try_into_authorized_jwt_credential_request(
            0,
            &[dpop_proof],
            JwtCredentialRequestLimits::default(),
        )
        .expect_err("unsupported token type");
    assert_eq!(
        unsupported,
        CredentialOfferError::CredentialRequestTokenTypeUnsupported
    );

    let bounded_proof = proof("nonce");
    let limits = JwtCredentialRequestLimits::new(1, bounded_proof.compact().len(), 8, 64)
        .expect("positive limits");
    let oversized = correlated("Bearer", &["identifier"])
        .try_into_authorized_jwt_credential_request(0, &[bounded_proof], limits)
        .expect_err("bounded body");
    assert_eq!(
        oversized,
        CredentialOfferError::CredentialRequestBodyTooLarge
    );
}

#[test]
fn empty_proofs_precede_body_construction() {
    let error = correlated("Bearer", &["identifier"])
        .try_into_authorized_jwt_credential_request(0, &[], JwtCredentialRequestLimits::default())
        .expect_err("proof required");
    assert_eq!(error, CredentialOfferError::CredentialRequestProofsRequired);
}
