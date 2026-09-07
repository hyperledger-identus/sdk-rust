use identus_core::{ErrorKind, IdentusError};
use identus_jose::{
    JwsAlgorithm, JwsKeyReference, JwsSigner, Oid4vciProofJwt, Oid4vciProofJwtBuilder,
    Oid4vciProofJwtClaims, Oid4vciProofJwtClient, Oid4vciProofJwtLimits, SignerFailure,
};
use identus_oid4vci::{
    CAPABILITY, CredentialIssuerMetadata, CredentialIssuerMetadataLimits, CredentialOffer,
    CredentialOfferError, CredentialOfferGrantLimits, CredentialOfferLimits,
    CredentialOfferSemanticLimits, EmbeddedCredentialOffer, ImmediateCredentialHttpResponseLimits,
    ImmediateCredentialResponseLimits, JwtCredentialRequest, JwtCredentialRequestLimits,
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
            JwsKeyReference::KeyId("did:example:holder#key-1".to_owned()),
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
fn defaults_and_positive_content_type_limit_are_explicit() {
    let defaults = ImmediateCredentialHttpResponseLimits::default();
    assert_eq!(defaults.max_content_type_bytes(), 1_024);
    assert_eq!(
        defaults.response_limits(),
        ImmediateCredentialResponseLimits::default()
    );
    assert_eq!(
        ImmediateCredentialHttpResponseLimits::new(ImmediateCredentialResponseLimits::default(), 0,),
        Err(CredentialOfferError::InvalidImmediateCredentialHttpResponseLimits)
    );
}

#[test]
fn exact_200_json_response_is_bound_to_request_count() {
    let request = request(2);
    let body = r#"{"credentials":[{"credential":"one"},{"credential":{"claim":2}}],"notification_id":"opaque-notify"}"#;
    let bound = request
        .validate_immediate_response(
            200,
            "Application/JSON ; Charset=\"utf-8\"",
            body,
            ImmediateCredentialHttpResponseLimits::default(),
        )
        .expect("request-bound immediate response");

    assert_eq!(bound.request_proof_count(), 2);
    assert_eq!(bound.response().credentials().len(), 2);
    assert_eq!(
        bound.response().credentials()[0].expose_sensitive_string(),
        Some("one")
    );
    assert_eq!(
        bound.response().credentials()[1].expose_sensitive_json(),
        r#"{"claim":2}"#
    );
    assert_eq!(
        bound.response().expose_sensitive_notification_id(),
        Some("opaque-notify")
    );
    assert_eq!(bound.into_response().response_len(), body.len());
}

#[test]
fn fewer_credentials_than_proofs_are_allowed() {
    let request = request(2);
    let bound = request
        .validate_immediate_response(
            200,
            "application/json",
            r#"{"credentials":[{"credential":"one"}]}"#,
            ImmediateCredentialHttpResponseLimits::default(),
        )
        .expect("fewer credentials");
    assert_eq!(bound.response().credentials().len(), 1);
    assert_eq!(bound.request_proof_count(), 2);
}

#[test]
fn credentials_cannot_exceed_request_proof_count() {
    let request = request(1);
    assert_eq!(
        request
            .validate_immediate_response(
                200,
                "application/json",
                r#"{"credentials":[{"credential":"one"},{"credential":"two"}]}"#,
                ImmediateCredentialHttpResponseLimits::default(),
            )
            .expect_err("too many credentials"),
        CredentialOfferError::CredentialResponseExceedsProofCount
    );
}

#[test]
fn status_is_classified_before_untrusted_fields() {
    let request = request(1);
    assert_eq!(
        request
            .validate_immediate_response(
                202,
                "CONTENT_TYPE_CANARY",
                "BODY_CANARY",
                ImmediateCredentialHttpResponseLimits::default(),
            )
            .expect_err("deferred response"),
        CredentialOfferError::DeferredCredentialResponseUnsupported
    );
    for status in [0, 199, 201, 204, 400, 500] {
        assert_eq!(
            request
                .validate_immediate_response(
                    status,
                    "CONTENT_TYPE_CANARY",
                    "BODY_CANARY",
                    ImmediateCredentialHttpResponseLimits::default(),
                )
                .expect_err("invalid immediate status"),
            CredentialOfferError::InvalidImmediateCredentialHttpStatus
        );
    }
}

#[test]
fn content_type_is_strict_bounded_and_checked_before_body() {
    let request = request(1);
    let body = r#"{"credentials":[{"credential":"one"}]}"#;
    for media_type in [
        "application/json",
        "APPLICATION/JSON",
        " application/json ; charset=utf-8 ",
        "application/json;charset=\"utf-8\";profile=final",
    ] {
        request
            .validate_immediate_response(
                200,
                media_type,
                body,
                ImmediateCredentialHttpResponseLimits::default(),
            )
            .expect("valid JSON media type");
    }
    for media_type in [
        "",
        "application/jsonp",
        "text/json",
        "application/json, text/plain",
        "application/json; charset=\"unterminated",
        "application/json; charset",
        "application/json;charset=utf-8;CHARSET=iso-8859-1",
    ] {
        assert_eq!(
            request
                .validate_immediate_response(
                    200,
                    media_type,
                    "BODY_CANARY",
                    ImmediateCredentialHttpResponseLimits::default(),
                )
                .expect_err("invalid media type"),
            CredentialOfferError::InvalidImmediateCredentialContentType,
            "unexpected result for {media_type:?}"
        );
    }

    let exact = ImmediateCredentialHttpResponseLimits::new(
        ImmediateCredentialResponseLimits::default(),
        "application/json".len(),
    )
    .expect("positive HTTP limits");
    request
        .validate_immediate_response(200, "application/json", body, exact)
        .expect("exact Content-Type bound");
    assert_eq!(
        request
            .validate_immediate_response(200, "application/json ", "BODY_CANARY", exact)
            .expect_err("oversized Content-Type"),
        CredentialOfferError::ImmediateCredentialContentTypeTooLarge
    );
}

#[test]
fn body_parser_errors_are_preserved_and_request_is_reusable() {
    let request = request(1);
    let body = r#"{"credentials":[{"credential":"one"}]}"#;
    let tiny_body = ImmediateCredentialResponseLimits::new(1, 1, 1, 1, 1, 1, 1, 1, 1)
        .expect("positive response limits");
    assert_eq!(
        request
            .validate_immediate_response(
                200,
                "application/json",
                body,
                ImmediateCredentialHttpResponseLimits::new(tiny_body, 64)
                    .expect("positive HTTP limits"),
            )
            .expect_err("body bound"),
        CredentialOfferError::ImmediateCredentialResponseTooLarge
    );
    assert_eq!(
        request
            .validate_immediate_response(
                200,
                "application/json",
                r#"{"transaction_id":"deferred"}"#,
                ImmediateCredentialHttpResponseLimits::default(),
            )
            .expect_err("deferred body"),
        CredentialOfferError::DeferredCredentialResponseUnsupported
    );
    request
        .validate_immediate_response(
            200,
            "application/json",
            body,
            ImmediateCredentialHttpResponseLimits::default(),
        )
        .expect("request remains reusable");
}

#[test]
fn bound_state_and_diagnostics_redact_remote_values() {
    let credential_canary = "CREDENTIAL_CANARY_c813";
    let notification_canary = "NOTIFICATION_CANARY_d112";
    let request = request(1);
    let body = format!(
        r#"{{"credentials":[{{"credential":"{credential_canary}"}}],"notification_id":"{notification_canary}"}}"#
    );
    let bound = request
        .validate_immediate_response(
            200,
            "application/json",
            &body,
            ImmediateCredentialHttpResponseLimits::default(),
        )
        .expect("bound response");
    let rendered = format!("{bound:?}");
    assert!(!rendered.contains(credential_canary));
    assert!(!rendered.contains(notification_canary));

    let cases = [
        (
            CredentialOfferError::InvalidImmediateCredentialHttpResponseLimits,
            error_code::INVALID_IMMEDIATE_CREDENTIAL_HTTP_RESPONSE_LIMITS,
        ),
        (
            CredentialOfferError::InvalidImmediateCredentialHttpStatus,
            error_code::INVALID_IMMEDIATE_CREDENTIAL_HTTP_STATUS,
        ),
        (
            CredentialOfferError::ImmediateCredentialContentTypeTooLarge,
            error_code::IMMEDIATE_CREDENTIAL_CONTENT_TYPE_TOO_LARGE,
        ),
        (
            CredentialOfferError::InvalidImmediateCredentialContentType,
            error_code::INVALID_IMMEDIATE_CREDENTIAL_CONTENT_TYPE,
        ),
        (
            CredentialOfferError::CredentialResponseExceedsProofCount,
            error_code::CREDENTIAL_RESPONSE_EXCEEDS_PROOF_COUNT,
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
            assert!(!rendered.contains(credential_canary));
            assert!(!rendered.contains(notification_canary));
        }
    }
}
