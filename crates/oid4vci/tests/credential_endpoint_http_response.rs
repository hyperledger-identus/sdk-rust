use identus_core::ErrorKind;
use identus_jose::{
    JwsAlgorithm, JwsKeyReference, JwsSigner, Oid4vciProofJwt, Oid4vciProofJwtBuilder,
    Oid4vciProofJwtClaims, Oid4vciProofJwtClient, Oid4vciProofJwtLimits, SignerFailure,
};
use identus_oid4vci::{
    CAPABILITY, CredentialEndpointErrorKind, CredentialEndpointResponseLimits,
    CredentialEndpointResponseOutcome, CredentialIssuerMetadata, CredentialIssuerMetadataLimits,
    CredentialOffer, CredentialOfferError, CredentialOfferGrantLimits, CredentialOfferLimits,
    CredentialOfferSemanticLimits, DeferredCredentialHttpResponseLimits, EmbeddedCredentialOffer,
    ImmediateCredentialHttpResponseLimits, JwtCredentialRequest, JwtCredentialRequestLimits,
    PRE_AUTHORIZED_CODE_GRANT_TYPE, TokenResponseCore, TokenResponseLimits, error_code,
};
use serde_json::json;

const ISSUER: &str = "https://credential-issuer.example.com";
const ENDPOINT: &str = "https://credential-issuer.example.com/credential";

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
            JwsKeyReference::key_id("did:example:holder#key-1", limits.jws())
                .expect("bounded key ID"),
            claims,
        )
        .expect("proof input")
        .sign_with(&StubSigner)
        .expect("holder-produced proof")
}

fn request(proof_count: usize) -> JwtCredentialRequest {
    let offer = CredentialOffer::try_from_embedded(
        EmbeddedCredentialOffer::try_from_json(
            &json!({
                "credential_issuer": ISSUER,
                "credential_configuration_ids": ["degree"],
                "grants": {
                    PRE_AUTHORIZED_CODE_GRANT_TYPE: {
                        "pre-authorized_code": "opaque-code"
                    }
                }
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
    let metadata = CredentialIssuerMetadata::parse(
        &json!({
            "credential_issuer": ISSUER,
            "credential_endpoint": ENDPOINT,
            "credential_configurations_supported": {
                "degree": {"format": "dc+sd-jwt"}
            }
        })
        .to_string(),
        ISSUER,
        CredentialIssuerMetadataLimits::default(),
    )
    .expect("issuer metadata");
    let state = offer.try_with_metadata(metadata).expect("matched metadata");
    let token = TokenResponseCore::parse(
        r#"{"access_token":"opaque-token","token_type":"Bearer"}"#,
        TokenResponseLimits::default(),
    )
    .expect("token response");
    let proofs = (0..proof_count)
        .map(|index| proof(&format!("nonce-{index}")))
        .collect::<Vec<_>>();
    state
        .try_create_jwt_credential_request(
            &token,
            0,
            &proofs,
            JwtCredentialRequestLimits::default(),
        )
        .expect("Credential Request")
}

#[test]
fn default_and_composed_policies_are_explicit() {
    let defaults = CredentialEndpointResponseLimits::default();
    assert_eq!(
        defaults.immediate_response_limits(),
        ImmediateCredentialHttpResponseLimits::default()
    );
    assert_eq!(
        defaults.deferred_response_limits(),
        DeferredCredentialHttpResponseLimits::default()
    );

    let composed = CredentialEndpointResponseLimits::new(
        defaults.immediate_response_limits(),
        defaults.deferred_response_limits(),
        defaults.error_response_limits(),
    );
    assert_eq!(composed, defaults);
}

#[test]
fn exact_200_response_is_classified_and_bound_to_request_count() {
    let outcome = request(2)
        .try_into_credential_endpoint_response(
            200,
            "application/json; charset=utf-8",
            r#"{"credentials":[{"credential":"one"},{"credential":{"claim":2}}]}"#,
            CredentialEndpointResponseLimits::default(),
        )
        .expect("issued response");

    let CredentialEndpointResponseOutcome::Issued(bound) = outcome else {
        panic!("expected issued response")
    };
    assert_eq!(bound.request_proof_count(), 2);
    assert_eq!(bound.response().credentials().len(), 2);
}

#[test]
fn exact_202_response_is_classified_and_bound_to_request_count() {
    let outcome = request(3)
        .try_into_credential_endpoint_response(
            202,
            "application/json",
            r#"{"transaction_id":"opaque-transaction","interval":5}"#,
            CredentialEndpointResponseLimits::default(),
        )
        .expect("deferred response");

    let CredentialEndpointResponseOutcome::Deferred(bound) = outcome else {
        panic!("expected deferred response")
    };
    assert_eq!(bound.request_proof_count(), 3);
    assert_eq!(bound.response().interval().as_str(), "5");
    assert_eq!(
        bound
            .response()
            .transaction_id()
            .expose_sensitive_transaction_id(),
        "opaque-transaction"
    );
    assert_eq!(bound.into_response().interval().as_str(), "5");
}

#[test]
fn exact_400_payload_error_is_classified_and_bound_to_request_count() {
    let outcome = request(1)
        .try_into_credential_endpoint_response(
            400,
            "application/json",
            r#"{"error":"invalid_proof","error_description":"private-description"}"#,
            CredentialEndpointResponseLimits::default(),
        )
        .expect("payload error response");

    let CredentialEndpointResponseOutcome::Error(bound) = outcome else {
        panic!("expected payload error")
    };
    assert_eq!(bound.request_proof_count(), 1);
    assert_eq!(
        bound.response().error_kind(),
        CredentialEndpointErrorKind::InvalidProof
    );
    assert_eq!(
        bound.response().expose_untrusted_description(),
        Some("private-description")
    );
    assert_eq!(
        bound.into_response().error_kind(),
        CredentialEndpointErrorKind::InvalidProof
    );
}

#[test]
fn status_selects_the_branch_before_untrusted_fields_are_parsed() {
    for status in [0, 199, 201, 204, 401, 403, 500] {
        assert_eq!(
            request(1)
                .try_into_credential_endpoint_response(
                    status,
                    "CONTENT_TYPE_CANARY",
                    "BODY_CANARY",
                    CredentialEndpointResponseLimits::default(),
                )
                .expect_err("unsupported Credential Endpoint status"),
            CredentialOfferError::InvalidCredentialEndpointHttpStatus,
            "unexpected result for status {status}"
        );
    }

    assert_eq!(
        request(1)
            .try_into_credential_endpoint_response(
                202,
                "text/plain",
                "BODY_CANARY",
                CredentialEndpointResponseLimits::default(),
            )
            .expect_err("deferred media type"),
        CredentialOfferError::InvalidDeferredCredentialContentType
    );
    assert_eq!(
        request(1)
            .try_into_credential_endpoint_response(
                400,
                "text/plain",
                "BODY_CANARY",
                CredentialEndpointResponseLimits::default(),
            )
            .expect_err("payload-error media type"),
        CredentialOfferError::InvalidCredentialErrorContentType
    );
}

#[test]
fn immediate_credentials_remain_bounded_by_the_consumed_request() {
    assert_eq!(
        request(1)
            .try_into_credential_endpoint_response(
                200,
                "application/json",
                r#"{"credentials":[{"credential":"one"},{"credential":"two"}]}"#,
                CredentialEndpointResponseLimits::default(),
            )
            .expect_err("more credentials than proofs"),
        CredentialOfferError::CredentialResponseExceedsProofCount
    );
}

#[test]
fn debug_and_error_bridges_do_not_disclose_response_values() {
    let deferred = request(1)
        .try_into_credential_endpoint_response(
            202,
            "application/json",
            r#"{"transaction_id":"private-transaction","interval":7}"#,
            CredentialEndpointResponseLimits::default(),
        )
        .expect("deferred response");
    let error = request(1)
        .try_into_credential_endpoint_response(
            400,
            "application/json",
            r#"{"error":"invalid_nonce","error_description":"private-description"}"#,
            CredentialEndpointResponseLimits::default(),
        )
        .expect("payload error response");

    let deferred_debug = format!("{deferred:?}");
    assert!(!deferred_debug.contains("private-transaction"));
    let error_debug = format!("{error:?}");
    assert!(!error_debug.contains("private-description"));

    let public = CredentialOfferError::InvalidCredentialEndpointHttpStatus.to_identus_error();
    assert_eq!(
        public.code(),
        error_code::INVALID_CREDENTIAL_ENDPOINT_HTTP_STATUS
    );
    assert_eq!(public.kind(), ErrorKind::InvalidInput);
    assert_eq!(public.capability(), Some(CAPABILITY));
}
