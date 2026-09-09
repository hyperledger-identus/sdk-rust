use identus_core::{ErrorKind, IdentusError};
use identus_jose::{
    JwsAlgorithm, JwsKeyReference, JwsSigner, Oid4vciProofJwt, Oid4vciProofJwtBuilder,
    Oid4vciProofJwtClaims, Oid4vciProofJwtClient, Oid4vciProofJwtLimits, SignerFailure,
};
use identus_oid4vci::{
    CAPABILITY, CREDENTIAL_REQUEST_HTTP_METHOD, CREDENTIAL_REQUEST_MEDIA_TYPE,
    CredentialIssuerMetadata, CredentialIssuerMetadataLimits, CredentialOffer,
    CredentialOfferError, CredentialOfferGrantLimits, CredentialOfferLimits,
    CredentialOfferSemanticLimits, EmbeddedCredentialOffer, JwtCredentialRequest,
    JwtCredentialRequestLimits, PRE_AUTHORIZED_CODE_GRANT_TYPE, TokenAuthorizationDetailsLimits,
    TokenResponseCore, TokenResponseLimits, TokenResponseWithAuthorizationDetails, error_code,
};
use serde_json::{Map, Value, json};

const ISSUER: &str = "https://credential-issuer.example.com/tenant";
const ENDPOINT: &str = "https://credential-issuer.example.com:8443/credential?tenant=wallet";

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
        Oid4vciProofJwtClient::AnonymousPreAuthorized,
        ISSUER,
        1_700_000_000,
        Some(nonce.to_owned()),
        limits,
    )
    .expect("proof claims");
    Oid4vciProofJwtBuilder::new(limits)
        .prepare(
            JwsAlgorithm::Ed25519,
            JwsKeyReference::KeyId("did:example:holder#authentication-1".to_owned()),
            claims,
        )
        .expect("proof input")
        .sign_with(&StubSigner)
        .expect("holder-produced proof")
}

fn matched(
    configuration_ids: &[&str],
    endpoint: &str,
) -> identus_oid4vci::CredentialOfferWithMetadata {
    let offer_json = json!({
        "credential_issuer": ISSUER,
        "credential_configuration_ids": configuration_ids,
        "grants": {
            PRE_AUTHORIZED_CODE_GRANT_TYPE: {
                "pre-authorized_code": "opaque-pre-authorized-code"
            }
        }
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

    let configurations = configuration_ids
        .iter()
        .map(|id| ((*id).to_owned(), json!({"format": "dc+sd-jwt"})))
        .collect::<Map<String, Value>>();
    let metadata_json = json!({
        "credential_issuer": ISSUER,
        "credential_endpoint": endpoint,
        "credential_configurations_supported": configurations
    })
    .to_string();
    let metadata = CredentialIssuerMetadata::parse(
        &metadata_json,
        ISSUER,
        CredentialIssuerMetadataLimits::default(),
    )
    .expect("issuer metadata");
    offer.try_with_metadata(metadata).expect("matched metadata")
}

fn token(access_token: &str, token_type: &str, authorization_details: bool) -> TokenResponseCore {
    let mut value = json!({
        "access_token": access_token,
        "token_type": token_type
    });
    if authorization_details {
        value["authorization_details"] = json!([{
            "type": "openid_credential",
            "credential_identifiers": ["dataset-1"]
        }]);
    }
    TokenResponseCore::parse(&value.to_string(), TokenResponseLimits::default())
        .expect("Token Response core")
}

fn authorized_token(
    access_token: &str,
    token_type: &str,
    details: Value,
) -> TokenResponseWithAuthorizationDetails {
    TokenResponseCore::parse(
        &json!({
            "access_token": access_token,
            "token_type": token_type,
            "authorization_details": details
        })
        .to_string(),
        TokenResponseLimits::default(),
    )
    .expect("Token Response core")
    .try_validate_authorization_details(TokenAuthorizationDetailsLimits::default())
    .expect("validated Authorization Details")
}

fn request(
    state: &identus_oid4vci::CredentialOfferWithMetadata,
    token: &TokenResponseCore,
    index: usize,
    proofs: &[Oid4vciProofJwt],
    limits: JwtCredentialRequestLimits,
) -> Result<JwtCredentialRequest, CredentialOfferError> {
    state.try_create_jwt_credential_request(token, index, proofs, limits)
}

#[test]
fn defaults_and_positive_limits_are_explicit() {
    let defaults = JwtCredentialRequestLimits::default();
    assert_eq!(defaults.max_proofs(), 16);
    assert_eq!(defaults.max_proof_bytes(), 16_384);
    assert_eq!(defaults.max_json_body_bytes(), 262_144);
    assert_eq!(defaults.max_authorization_bytes(), 16_384);

    for result in [
        JwtCredentialRequestLimits::new(0, 1, 1, 1),
        JwtCredentialRequestLimits::new(1, 0, 1, 1),
        JwtCredentialRequestLimits::new(1, 1, 0, 1),
        JwtCredentialRequestLimits::new(1, 1, 1, 0),
    ] {
        assert_eq!(
            result,
            Err(CredentialOfferError::InvalidJwtCredentialRequestLimits)
        );
    }

    let limits = JwtCredentialRequestLimits::new(2, 3, 4, 5).expect("positive limits");
    assert_eq!(limits.max_proofs(), 2);
    assert_eq!(limits.max_proof_bytes(), 3);
    assert_eq!(limits.max_json_body_bytes(), 4);
    assert_eq!(limits.max_authorization_bytes(), 5);
}

#[test]
fn constructs_owned_consumer_shaped_request_with_exact_wire_values() {
    let state = matched(&["degree", "passport"], ENDPOINT);
    let response = token("mF_9.B5f-4.1JqM==", "bEaReR", false);
    let proofs = [proof("nonce-one"), proof("nonce-two")];
    let expected = format!(
        r#"{{"credential_configuration_id":"passport","proofs":{{"jwt":["{}","{}"]}}}}"#,
        proofs[0].compact(),
        proofs[1].compact()
    );
    let request = request(
        &state,
        &response,
        1,
        &proofs,
        JwtCredentialRequestLimits::default(),
    )
    .expect("Credential Request");

    assert_eq!(request.credential_endpoint().as_str(), ENDPOINT);
    assert_eq!(request.http_method(), "POST");
    assert_eq!(request.http_method(), CREDENTIAL_REQUEST_HTTP_METHOD);
    assert_eq!(request.media_type(), "application/json");
    assert_eq!(request.media_type(), CREDENTIAL_REQUEST_MEDIA_TYPE);
    assert_eq!(request.proof_count(), 2);
    assert_eq!(
        request.expose_sensitive_authorization(),
        "Bearer mF_9.B5f-4.1JqM=="
    );
    assert_eq!(
        request.authorization_len(),
        "Bearer mF_9.B5f-4.1JqM==".len()
    );
    assert_eq!(request.expose_sensitive_json_body(), expected);
    assert_eq!(request.json_body_len(), expected.len());

    drop(state);
    drop(response);
    drop(proofs);
    assert_eq!(request.credential_endpoint().as_str(), ENDPOINT);
    assert_eq!(request.expose_sensitive_json_body(), expected);
}

#[test]
fn constructs_authorized_dataset_request_with_exact_exclusive_selector() {
    let state = matched(&["degree", "passport"], ENDPOINT);
    let response = authorized_token(
        "authorized-token",
        "Bearer",
        json!([
            {
                "type": "example_extension"
            },
            {
                "type": "openid_credential",
                "credential_configuration_id": "passport",
                "credential_identifiers": ["dataset-one", "dataset\"\\two"]
            }
        ]),
    );
    let proofs = [proof("nonce-one"), proof("nonce-two")];
    let request = state
        .try_create_authorized_jwt_credential_request(
            &response,
            0,
            1,
            &proofs,
            JwtCredentialRequestLimits::default(),
        )
        .expect("authorized Credential Request");
    let expected = format!(
        r#"{{"credential_identifier":"dataset\"\\two","proofs":{{"jwt":["{}","{}"]}}}}"#,
        proofs[0].compact(),
        proofs[1].compact()
    );

    assert_eq!(
        request.expose_sensitive_authorization(),
        "Bearer authorized-token"
    );
    assert_eq!(request.expose_sensitive_json_body(), expected);
    let body: Value =
        serde_json::from_str(request.expose_sensitive_json_body()).expect("JSON body");
    assert_eq!(body["credential_identifier"], "dataset\"\\two");
    assert!(body.get("credential_configuration_id").is_none());
    assert_eq!(body["proofs"]["jwt"][0], proofs[0].compact());
    assert_eq!(body["proofs"]["jwt"][1], proofs[1].compact());
    assert_eq!(body.as_object().map(Map::len), Some(2));
}

#[test]
fn authorized_dataset_selection_and_offer_binding_fail_closed() {
    let state = matched(&["degree"], ENDPOINT);
    let proof = [proof("nonce")];
    let response = authorized_token(
        "authorized-token",
        "Bearer",
        json!([{
            "type": "openid_credential",
            "credential_configuration_id": "degree",
            "credential_identifiers": ["dataset-one"]
        }]),
    );

    assert_eq!(
        state
            .try_create_authorized_jwt_credential_request(
                &response,
                1,
                0,
                &proof,
                JwtCredentialRequestLimits::default(),
            )
            .expect_err("Authorization Detail index"),
        CredentialOfferError::CredentialRequestAuthorizationDetailMissing
    );
    assert_eq!(
        state
            .try_create_authorized_jwt_credential_request(
                &response,
                0,
                1,
                &proof,
                JwtCredentialRequestLimits::default(),
            )
            .expect_err("Credential Dataset index"),
        CredentialOfferError::CredentialRequestIdentifierMissing
    );

    let mismatched = authorized_token(
        "authorized-token",
        "Bearer",
        json!([{
            "type": "openid_credential",
            "credential_configuration_id": "passport",
            "credential_identifiers": ["dataset-one"]
        }]),
    );
    assert_eq!(
        state
            .try_create_authorized_jwt_credential_request(
                &mismatched,
                0,
                0,
                &proof,
                JwtCredentialRequestLimits::default(),
            )
            .expect_err("configuration not offered"),
        CredentialOfferError::CredentialRequestAuthorizationConfigurationMismatch
    );
}

#[test]
fn authorized_dataset_route_reuses_token_proof_and_complete_body_bounds() {
    let state = matched(&["degree"], ENDPOINT);
    let proof = [proof("nonce")];
    let response = authorized_token(
        "authorized-token",
        "Bearer",
        json!([{
            "type": "openid_credential",
            "credential_configuration_id": "degree",
            "credential_identifiers": ["dataset-one"]
        }]),
    );
    let complete = state
        .try_create_authorized_jwt_credential_request(
            &response,
            0,
            0,
            &proof,
            JwtCredentialRequestLimits::default(),
        )
        .expect("complete request");

    assert_eq!(
        state
            .try_create_authorized_jwt_credential_request(
                &response,
                0,
                0,
                &proof,
                JwtCredentialRequestLimits::new(
                    1,
                    proof[0].compact().len(),
                    complete.json_body_len() - 1,
                    complete.authorization_len(),
                )
                .expect("limits"),
            )
            .expect_err("body one byte over"),
        CredentialOfferError::CredentialRequestBodyTooLarge
    );

    let unsupported = authorized_token(
        "authorized-token",
        "DPoP",
        json!([{
            "type": "openid_credential",
            "credential_configuration_id": "degree",
            "credential_identifiers": ["dataset-one"]
        }]),
    );
    assert_eq!(
        state
            .try_create_authorized_jwt_credential_request(
                &unsupported,
                0,
                0,
                &proof,
                JwtCredentialRequestLimits::default(),
            )
            .expect_err("unsupported token type"),
        CredentialOfferError::CredentialRequestTokenTypeUnsupported
    );
}

#[test]
fn selection_and_token_state_fail_closed_before_request_construction() {
    let state = matched(&["degree"], ENDPOINT);
    let proof = [proof("nonce")];

    assert_eq!(
        request(
            &state,
            &token("access-token", "Bearer", false),
            1,
            &proof,
            JwtCredentialRequestLimits::default(),
        )
        .expect_err("invalid selection"),
        CredentialOfferError::CredentialRequestConfigurationMissing
    );
    assert_eq!(
        request(
            &state,
            &token("access-token", "Bearer", true),
            0,
            &proof,
            JwtCredentialRequestLimits::default(),
        )
        .expect_err("Authorization Details route"),
        CredentialOfferError::CredentialRequestAuthorizationDetailsUnsupported
    );
    assert_eq!(
        request(
            &state,
            &token("access-token", "DPoP", false),
            0,
            &proof,
            JwtCredentialRequestLimits::default(),
        )
        .expect_err("unsupported token type"),
        CredentialOfferError::CredentialRequestTokenTypeUnsupported
    );
}

#[test]
fn bearer_token_must_match_rfc6750_header_grammar() {
    let state = matched(&["degree"], ENDPOINT);
    let proof = [proof("nonce")];
    for malformed in [
        " token",
        "token value",
        "token=middle",
        "=padding",
        "token%",
    ] {
        assert_eq!(
            request(
                &state,
                &token(malformed, "Bearer", false),
                0,
                &proof,
                JwtCredentialRequestLimits::default(),
            )
            .expect_err("invalid Bearer token"),
            CredentialOfferError::InvalidCredentialRequestBearerToken
        );
    }

    for valid in ["a", "AZaz09-._~+/", "token=", "token==="] {
        request(
            &state,
            &token(valid, "BEARER", false),
            0,
            &proof,
            JwtCredentialRequestLimits::default(),
        )
        .expect("valid Bearer token");
    }
}

#[test]
fn proof_count_and_per_proof_bounds_are_independent() {
    let state = matched(&["degree"], ENDPOINT);
    let response = token("access-token", "Bearer", false);
    let one = proof("nonce-one");
    let two = proof("nonce-two");

    assert_eq!(
        request(
            &state,
            &response,
            0,
            &[],
            JwtCredentialRequestLimits::default(),
        )
        .expect_err("proof required"),
        CredentialOfferError::CredentialRequestProofsRequired
    );
    assert_eq!(
        request(
            &state,
            &response,
            0,
            &[one.clone(), two],
            JwtCredentialRequestLimits::new(1, 16_384, 262_144, 16_384).expect("limits"),
        )
        .expect_err("too many proofs"),
        CredentialOfferError::TooManyCredentialRequestProofs
    );
    assert_eq!(
        request(
            &state,
            &response,
            0,
            std::slice::from_ref(&one),
            JwtCredentialRequestLimits::new(1, one.compact().len() - 1, 262_144, 16_384)
                .expect("limits"),
        )
        .expect_err("oversized proof"),
        CredentialOfferError::CredentialRequestProofTooLarge
    );
}

#[test]
fn final_body_and_authorization_limits_include_complete_encoding() {
    let state = matched(&["degree"], ENDPOINT);
    let response = token("access-token", "Bearer", false);
    let proof = [proof("nonce")];
    let complete = request(
        &state,
        &response,
        0,
        &proof,
        JwtCredentialRequestLimits::default(),
    )
    .expect("complete request");

    request(
        &state,
        &response,
        0,
        &proof,
        JwtCredentialRequestLimits::new(
            1,
            proof[0].compact().len(),
            complete.json_body_len(),
            complete.authorization_len(),
        )
        .expect("exact limits"),
    )
    .expect("exact limits pass");
    assert_eq!(
        request(
            &state,
            &response,
            0,
            &proof,
            JwtCredentialRequestLimits::new(
                1,
                proof[0].compact().len(),
                complete.json_body_len() - 1,
                complete.authorization_len(),
            )
            .expect("limits"),
        )
        .expect_err("body one byte over"),
        CredentialOfferError::CredentialRequestBodyTooLarge
    );
    assert_eq!(
        request(
            &state,
            &response,
            0,
            &proof,
            JwtCredentialRequestLimits::new(
                1,
                proof[0].compact().len(),
                complete.json_body_len(),
                complete.authorization_len() - 1,
            )
            .expect("limits"),
        )
        .expect_err("authorization one byte over"),
        CredentialOfferError::CredentialRequestAuthorizationTooLarge
    );
}

#[test]
fn json_escaping_and_proof_order_are_deterministic() {
    let configuration = "degree\"\\passport";
    let state = matched(&[configuration], ENDPOINT);
    let response = token("access-token", "Bearer", false);
    let proofs = [proof("nonce-one"), proof("nonce-two")];
    let first = request(
        &state,
        &response,
        0,
        &proofs,
        JwtCredentialRequestLimits::default(),
    )
    .expect("first request");
    let second = request(
        &state,
        &response,
        0,
        &proofs,
        JwtCredentialRequestLimits::default(),
    )
    .expect("second request");
    assert_eq!(
        first.expose_sensitive_json_body(),
        second.expose_sensitive_json_body()
    );

    let body: Value = serde_json::from_str(first.expose_sensitive_json_body()).expect("JSON body");
    assert_eq!(body["credential_configuration_id"], configuration);
    assert_eq!(body["proofs"]["jwt"][0], proofs[0].compact());
    assert_eq!(body["proofs"]["jwt"][1], proofs[1].compact());
    assert_eq!(body.as_object().map(Map::len), Some(2));
    assert_eq!(body["proofs"].as_object().map(Map::len), Some(1));
}

#[test]
fn request_and_all_new_errors_have_static_redacted_diagnostics() {
    let endpoint_canary = "ENDPOINT_CANARY_d7f2";
    let endpoint = format!("https://credential-issuer.example.com/{endpoint_canary}");
    let state = matched(&["CONFIG_CANARY_50fa"], &endpoint);
    let response = token("TOKEN_CANARY_3d77", "Bearer", false);
    let proofs = [proof("PROOF_NONCE_CANARY_49cd")];
    let proof_compact = proofs[0].compact().to_owned();
    let request = request(
        &state,
        &response,
        0,
        &proofs,
        JwtCredentialRequestLimits::default(),
    )
    .expect("request");
    let authorized = authorized_token(
        "AUTHORIZED_TOKEN_CANARY_397c",
        "Bearer",
        json!([{
            "type": "openid_credential",
            "credential_configuration_id": "CONFIG_CANARY_50fa",
            "credential_identifiers": ["DATASET_CANARY_d7cb"]
        }]),
    );
    let authorized_request = state
        .try_create_authorized_jwt_credential_request(
            &authorized,
            0,
            0,
            &proofs,
            JwtCredentialRequestLimits::default(),
        )
        .expect("authorized request");

    let debug = format!("{request:?} {authorized_request:?}");
    for canary in [
        endpoint_canary,
        "CONFIG_CANARY_50fa",
        "TOKEN_CANARY_3d77",
        "AUTHORIZED_TOKEN_CANARY_397c",
        "DATASET_CANARY_d7cb",
        proof_compact.as_str(),
    ] {
        assert!(!debug.contains(canary));
    }

    let errors = [
        CredentialOfferError::InvalidJwtCredentialRequestLimits,
        CredentialOfferError::CredentialRequestConfigurationMissing,
        CredentialOfferError::CredentialRequestAuthorizationDetailMissing,
        CredentialOfferError::CredentialRequestIdentifierMissing,
        CredentialOfferError::CredentialRequestAuthorizationConfigurationMismatch,
        CredentialOfferError::CredentialRequestAuthorizationDetailsUnsupported,
        CredentialOfferError::CredentialRequestTokenTypeUnsupported,
        CredentialOfferError::InvalidCredentialRequestBearerToken,
        CredentialOfferError::CredentialRequestProofsRequired,
        CredentialOfferError::TooManyCredentialRequestProofs,
        CredentialOfferError::CredentialRequestProofTooLarge,
        CredentialOfferError::CredentialRequestAuthorizationTooLarge,
        CredentialOfferError::CredentialRequestBodyTooLarge,
    ];
    for error in errors {
        let core: IdentusError = error.into();
        assert_eq!(
            core.kind(),
            if matches!(
                error,
                CredentialOfferError::CredentialRequestAuthorizationDetailsUnsupported
                    | CredentialOfferError::CredentialRequestTokenTypeUnsupported
            ) {
                ErrorKind::Unsupported
            } else {
                ErrorKind::InvalidInput
            }
        );
        assert_eq!(core.capability(), Some(CAPABILITY));
        for diagnostic in [
            format!("{error}"),
            format!("{error:?}"),
            format!("{core}"),
            format!("{core:?}"),
        ] {
            for canary in [
                endpoint_canary,
                "CONFIG_CANARY_50fa",
                "TOKEN_CANARY_3d77",
                proof_compact.as_str(),
            ] {
                assert!(!diagnostic.contains(canary));
            }
        }
    }

    assert_eq!(
        CredentialOfferError::InvalidJwtCredentialRequestLimits
            .to_identus_error()
            .code(),
        error_code::INVALID_JWT_CREDENTIAL_REQUEST_LIMITS
    );
    assert_eq!(
        CredentialOfferError::CredentialRequestBodyTooLarge
            .to_identus_error()
            .code(),
        error_code::CREDENTIAL_REQUEST_BODY_TOO_LARGE
    );
    assert_eq!(
        CredentialOfferError::CredentialRequestAuthorizationDetailMissing
            .to_identus_error()
            .code(),
        error_code::CREDENTIAL_REQUEST_AUTHORIZATION_DETAIL_MISSING
    );
    assert_eq!(
        CredentialOfferError::CredentialRequestIdentifierMissing
            .to_identus_error()
            .code(),
        error_code::CREDENTIAL_REQUEST_IDENTIFIER_MISSING
    );
    assert_eq!(
        CredentialOfferError::CredentialRequestAuthorizationConfigurationMismatch
            .to_identus_error()
            .code(),
        error_code::CREDENTIAL_REQUEST_AUTHORIZATION_CONFIGURATION_MISMATCH
    );
}
