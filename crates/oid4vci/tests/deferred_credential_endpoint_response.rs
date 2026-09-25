use identus_jose::{
    JwsAlgorithm, JwsKeyReference, JwsSigner, Oid4vciProofJwt, Oid4vciProofJwtBuilder,
    Oid4vciProofJwtClaims, Oid4vciProofJwtClient, Oid4vciProofJwtLimits, SignerFailure,
};
use identus_oid4vci::{
    CredentialEndpointResponseLimits, CredentialEndpointResponseOutcome,
    CredentialErrorHttpResponseLimits, CredentialIssuerMetadata, CredentialIssuerMetadataLimits,
    CredentialOffer, CredentialOfferError, CredentialOfferGrantLimits, CredentialOfferLimits,
    CredentialOfferSemanticLimits, DeferredCredentialEndpointResponseLimits,
    DeferredCredentialErrorKind, DeferredCredentialHttpResponseLimits,
    DeferredCredentialRequestLimits, DeferredCredentialResponseLimits, EmbeddedCredentialOffer,
    ImmediateCredentialResponseLimits, JwtCredentialRequest, JwtCredentialRequestLimits,
    PRE_AUTHORIZED_CODE_GRANT_TYPE, RequestBoundDeferredCredentialRequest, TokenResponseCore,
    TokenResponseLimits,
};
use serde_json::json;

const ISSUER: &str = "https://credential-issuer.example.com";
const ENDPOINT: &str = "https://credential-issuer.example.com/credential";
const DEFERRED_ENDPOINT: &str = "https://credential-issuer.example.com/deferred";

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

fn initial_request(proof_count: usize) -> JwtCredentialRequest {
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
            "deferred_credential_endpoint": DEFERRED_ENDPOINT,
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
        r#"{"access_token":"AUTHORIZATION_CANARY_5f3d","token_type":"Bearer"}"#,
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

fn request(proof_count: usize, transaction_id: &str) -> RequestBoundDeferredCredentialRequest {
    let body = serde_json::to_string(&json!({
        "transaction_id": transaction_id,
        "interval": 5
    }))
    .expect("deferred response JSON");
    let bound = match initial_request(proof_count)
        .try_into_credential_endpoint_response(
            202,
            "application/json",
            &body,
            CredentialEndpointResponseLimits::default(),
        )
        .expect("initial deferred response")
    {
        CredentialEndpointResponseOutcome::Deferred(bound) => bound,
        _ => panic!("expected deferred response"),
    };
    bound
        .try_into_deferred_credential_request(DeferredCredentialRequestLimits::default())
        .expect("authorized Deferred Credential Request")
}

#[test]
fn default_and_composed_limits_are_explicit() {
    let defaults = DeferredCredentialEndpointResponseLimits::default();
    assert_eq!(
        defaults.success_response_limits(),
        DeferredCredentialHttpResponseLimits::default()
    );
    assert_eq!(
        defaults.error_response_limits(),
        CredentialErrorHttpResponseLimits::default()
    );
    assert_eq!(
        DeferredCredentialEndpointResponseLimits::new(
            defaults.success_response_limits(),
            defaults.error_response_limits(),
        ),
        defaults
    );
}

#[test]
fn exact_200_is_terminal_and_bound_to_originating_proof_count() {
    let outcome = request(2, "issued-transaction")
        .try_into_deferred_credential_endpoint_response(
            200,
            "application/json; charset=utf-8",
            r#"{"credentials":[{"credential":"one"},{"credential":{"claim":2}}]}"#,
            DeferredCredentialEndpointResponseLimits::default(),
        )
        .expect("issued response");
    let identus_oid4vci::DeferredCredentialEndpointResponseOutcome::Issued(bound) = outcome else {
        panic!("expected issued response")
    };

    assert_eq!(bound.request_proof_count(), 2);
    assert_eq!(bound.response().credentials().len(), 2);

    assert_eq!(
        request(1, "over-issuance")
            .try_into_deferred_credential_endpoint_response(
                200,
                "application/json",
                r#"{"credentials":[{"credential":"one"},{"credential":"two"}]}"#,
                DeferredCredentialEndpointResponseLimits::default(),
            )
            .expect_err("credential count exceeds originating proof count"),
        CredentialOfferError::CredentialResponseExceedsProofCount
    );
}

#[test]
fn exact_202_alone_preserves_authority_for_the_next_request() {
    let transaction = "TRANSACTION_CANARY_a29f";
    let outcome = request(3, transaction)
        .try_into_deferred_credential_endpoint_response(
            202,
            "application/json",
            &format!(r#"{{"transaction_id":"{transaction}","interval":10}}"#),
            DeferredCredentialEndpointResponseLimits::default(),
        )
        .expect("pending response");
    let identus_oid4vci::DeferredCredentialEndpointResponseOutcome::Pending(bound) = outcome else {
        panic!("expected pending response")
    };

    assert_eq!(bound.request_proof_count(), 3);
    assert_eq!(bound.response().interval().as_str(), "10");
    let next = bound
        .try_into_deferred_credential_request(DeferredCredentialRequestLimits::default())
        .expect("next authorized request");
    assert_eq!(next.credential_issuer().as_str(), ISSUER);
    assert_eq!(
        next.deferred_credential_endpoint().as_str(),
        DEFERRED_ENDPOINT
    );
    assert_eq!(next.request_proof_count(), 3);
    assert_eq!(
        next.expose_sensitive_authorization(),
        "Bearer AUTHORIZATION_CANARY_5f3d"
    );
    assert_eq!(
        next.expose_sensitive_json_body(),
        format!(r#"{{"transaction_id":"{transaction}"}}"#).as_bytes()
    );
}

#[test]
fn substituted_or_malformed_pending_response_retains_no_result() {
    assert_eq!(
        request(1, "REQUEST_TRANSACTION_CANARY_8e2a")
            .try_into_deferred_credential_endpoint_response(
                202,
                "application/json",
                r#"{"transaction_id":"RESPONSE_TRANSACTION_CANARY_331f","interval":5}"#,
                DeferredCredentialEndpointResponseLimits::default(),
            )
            .expect_err("transaction substitution"),
        CredentialOfferError::DeferredCredentialTransactionMismatch
    );
    assert!(
        request(1, "malformed")
            .try_into_deferred_credential_endpoint_response(
                202,
                "application/json",
                "BODY_CANARY",
                DeferredCredentialEndpointResponseLimits::default(),
            )
            .is_err()
    );
}

#[test]
fn exact_400_is_terminal_deferred_error_evidence() {
    let outcome = request(4, "denied-transaction")
        .try_into_deferred_credential_endpoint_response(
            400,
            "application/json",
            r#"{"error":"credential_request_denied","error_description":"DESCRIPTION_CANARY"}"#,
            DeferredCredentialEndpointResponseLimits::default(),
        )
        .expect("deferred error response");
    let identus_oid4vci::DeferredCredentialEndpointResponseOutcome::Error(bound) = outcome else {
        panic!("expected deferred error response")
    };

    assert_eq!(bound.request_proof_count(), 4);
    assert_eq!(
        bound.response().kind(),
        DeferredCredentialErrorKind::CredentialRequestDenied
    );
    assert!(bound.response().should_stop_polling());
    assert_eq!(
        bound.response().core().expose_untrusted_description(),
        Some("DESCRIPTION_CANARY")
    );
}

#[test]
fn status_selects_before_media_or_body_and_selected_limits_remain_exact() {
    for status in [0, 199, 201, 204, 401, 403, 500] {
        assert_eq!(
            request(1, "status-order")
                .try_into_deferred_credential_endpoint_response(
                    status,
                    "CONTENT_TYPE_CANARY",
                    "BODY_CANARY",
                    DeferredCredentialEndpointResponseLimits::default(),
                )
                .expect_err("unsupported status"),
            CredentialOfferError::InvalidDeferredCredentialHttpStatus
        );
    }

    assert_eq!(
        request(1, "success-media")
            .try_into_deferred_credential_endpoint_response(
                200,
                "text/plain",
                "BODY_CANARY",
                DeferredCredentialEndpointResponseLimits::default(),
            )
            .expect_err("success media type"),
        CredentialOfferError::InvalidDeferredCredentialContentType
    );
    assert_eq!(
        request(1, "error-media")
            .try_into_deferred_credential_endpoint_response(
                400,
                "text/plain",
                "BODY_CANARY",
                DeferredCredentialEndpointResponseLimits::default(),
            )
            .expect_err("error media type"),
        CredentialOfferError::InvalidCredentialErrorContentType
    );

    let tiny_success = DeferredCredentialHttpResponseLimits::new(
        ImmediateCredentialResponseLimits::new(1, 1, 1, 1, 1, 1, 1, 1, 1)
            .expect("positive immediate limits"),
        DeferredCredentialResponseLimits::new(1, 1, 1, 1, 1, 1).expect("positive deferred limits"),
        64,
    )
    .expect("positive success HTTP limits");
    let limits = DeferredCredentialEndpointResponseLimits::new(
        tiny_success,
        CredentialErrorHttpResponseLimits::default(),
    );
    assert_eq!(
        request(1, "bounded-success")
            .try_into_deferred_credential_endpoint_response(
                200,
                "application/json",
                r#"{"credentials":[{"credential":"one"}]}"#,
                limits,
            )
            .expect_err("selected immediate body limit"),
        CredentialOfferError::ImmediateCredentialResponseTooLarge
    );
}

#[test]
fn diagnostics_redact_authority_transactions_credentials_and_descriptions() {
    let transaction = "TRANSACTION_CANARY_e180";
    let pending = request(1, transaction)
        .try_into_deferred_credential_endpoint_response(
            202,
            "application/json",
            &format!(r#"{{"transaction_id":"{transaction}","interval":5}}"#),
            DeferredCredentialEndpointResponseLimits::default(),
        )
        .expect("pending response");
    let issued = request(1, "issued-redaction")
        .try_into_deferred_credential_endpoint_response(
            200,
            "application/json",
            r#"{"credentials":[{"credential":"CREDENTIAL_CANARY_91c4"}]}"#,
            DeferredCredentialEndpointResponseLimits::default(),
        )
        .expect("issued response");
    let error = request(1, "error-redaction")
        .try_into_deferred_credential_endpoint_response(
            400,
            "application/json",
            r#"{"error":"credential_request_denied","error_description":"DESCRIPTION_CANARY_6de0"}"#,
            DeferredCredentialEndpointResponseLimits::default(),
        )
        .expect("error response");
    let mismatch = request(1, "REQUEST_CANARY_a4d2")
        .try_into_deferred_credential_endpoint_response(
            202,
            "application/json",
            r#"{"transaction_id":"RESPONSE_CANARY_209b","interval":5}"#,
            DeferredCredentialEndpointResponseLimits::default(),
        )
        .expect_err("mismatch");
    let core = mismatch.to_identus_error();

    for rendered in [
        format!("{pending:?}"),
        format!("{issued:?}"),
        format!("{error:?}"),
        format!("{mismatch:?}"),
        format!("{mismatch}"),
        format!("{core:?}"),
        format!("{core}"),
    ] {
        for canary in [
            ISSUER,
            ENDPOINT,
            DEFERRED_ENDPOINT,
            "AUTHORIZATION_CANARY_5f3d",
            transaction,
            "CREDENTIAL_CANARY_91c4",
            "DESCRIPTION_CANARY_6de0",
            "REQUEST_CANARY_a4d2",
            "RESPONSE_CANARY_209b",
        ] {
            assert!(!rendered.contains(canary), "diagnostic leaked {canary}");
        }
    }
}
