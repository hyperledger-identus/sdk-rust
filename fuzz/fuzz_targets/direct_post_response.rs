#![no_main]

use identus_crypto::{Ed25519PrivateKey, EncodeJwk, PublicKeyJwk};
use identus_jose::{
    Ed25519Signer, JwsAlgorithm, JwsLimits, JwsSigningInput, JwsVerificationKey,
    ProtectedHeader, SignatureSuiteRegistry,
};
use identus_oid4vp::{
    AuthorizationRequestInvocation, AuthorizationRequestInvocationLimits,
    AuthorizationRequestValidationLimits, DcqlLimits, DirectPostAcknowledgementLimits,
    DirectPostErrorCode, DirectPostErrorDetails, DirectPostErrorResponseLimits,
    DirectPostPresentation, DirectPostQueryResult, DirectPostResponseLimits,
    REQUEST_OBJECT_JWT_TYPE, REQUEST_OBJECT_MEDIA_TYPE, RedirectResponseLimits,
    RequestObjectValidationLimits, RequestUriRetrievalInput, RequestUriRetrievalLimits,
    ValidatedAuthorizationRequest,
};
use libfuzzer_sys::fuzz_target;
use serde_json::json;

mod oid4vp_support;

const REFERENCE: &str = "openid4vp:?client_id=decentralized_identifier%3Adid%3Aexample%3Averifier&request_uri=https%3A%2F%2Fverifier.example%2Frequest";
const REQUEST_OBJECT: &str = "eyJhbGciOiJFZDI1NTE5IiwidHlwIjoib2F1dGgtYXV0aHotcmVxK2p3dCIsImtpZCI6ImRpZDpleGFtcGxlOnZlcmlmaWVyI2tleS0xIn0.eyJhdWQiOiJodHRwczovL3NlbGYtaXNzdWVkLm1lL3YyIiwiY2xpZW50X2lkIjoiZGVjZW50cmFsaXplZF9pZGVudGlmaWVyOmRpZDpleGFtcGxlOnZlcmlmaWVyIiwiZGNxbF9xdWVyeSI6eyJjcmVkZW50aWFscyI6W3siZm9ybWF0IjoiZGMrc2Qtand0IiwiaWQiOiJkZWdyZWUiLCJtZXRhIjp7fX1dfSwiaWF0IjoxNzM1Njg5NDgwLCJub25jZSI6ImZyZXNoLW5vbmNlXzEiLCJyZXNwb25zZV9tb2RlIjoiZGlyZWN0X3Bvc3QiLCJyZXNwb25zZV90eXBlIjoidnBfdG9rZW4iLCJyZXNwb25zZV91cmkiOiJodHRwczovL3ZlcmlmaWVyLmV4YW1wbGUvcmVzcG9uc2UiLCJzdGF0ZSI6InN0YXRlLTEyMyJ9.LXDik4Kt5ZC16J7GFXgu83kS7slDNTXAjNi8-UavuHin0BWuXQ1OxzjcjIxHRF6bY6hkuAAhsWqWD_S9bjfUDg";
const PUBLIC_JWK: &str =
    r#"{"kty":"OKP","crv":"Ed25519","x":"4v4qObcyZkKCfW2C1JdiLNbCl_54Jw6qQ2sB8Ia288s"}"#;
const PRIVATE_BYTES: [u8; 32] = [0x71; 32];

fuzz_target!(|data: &[u8]| {
    let Some((&kind, value)) = data.split_first() else {
        return;
    };
    if kind & 8 != 0 {
        let mode = match kind % 3 {
            0 => "fragment",
            1 => "query",
            _ => "form_post",
        };
        let split = value.len() / 2;
        let uri_component = String::from_utf8_lossy(&value[..split]);
        let presentation = String::from_utf8_lossy(&value[split..]);
        let destination = match mode {
            "fragment" => format!("openid://wallet/cb#input={uri_component}"),
            _ => format!("https://verifier.example/cb?input={uri_component}"),
        };
        let Some(request) = dynamic_request(mode, "redirect_uri", &destination) else {
            return;
        };
        let limits = DirectPostResponseLimits::default();
        let Some(presentation) = DirectPostPresentation::string(&presentation, limits).ok() else {
            return;
        };
        let Some(result) = DirectPostQueryResult::new("degree", vec![presentation], limits).ok()
        else {
            return;
        };
        if let Ok(response) = request.into_redirect_response(
            vec![result],
            limits,
            RedirectResponseLimits::default(),
        ) {
            assert!(response.expose_sensitive_target().len() <= 1_048_576);
        }
        return;
    }

    let Some(request) = fixed_request() else {
        return;
    };
    if kind & 4 != 0 {
        let (content_type, cache_control, body) = if kind & 1 == 0 {
            (Some("application/json"), Some("no-store"), value)
        } else {
            let first = value.len() / 3;
            let second = first.saturating_mul(2);
            (
                std::str::from_utf8(&value[..first]).ok(),
                std::str::from_utf8(&value[first..second]).ok(),
                &value[second..],
            )
        };
        let limits = DirectPostResponseLimits::default();
        let presentation = DirectPostPresentation::string("fixed.presentation", limits)
            .expect("fixed presentation");
        let result = DirectPostQueryResult::new("degree", vec![presentation], limits)
            .expect("fixed result");
        let response = request
            .into_direct_post_response(vec![result], limits)
            .expect("fixed response");
        let _ = response.bind_endpoint_acknowledgement(
            200,
            content_type,
            cache_control,
            body,
            DirectPostAcknowledgementLimits::default(),
        );
    } else if kind & 2 == 0 {
        let limits = DirectPostResponseLimits::default();
        let presentation = if kind & 1 == 0 {
            std::str::from_utf8(value)
                .ok()
                .and_then(|value| DirectPostPresentation::string(value, limits).ok())
        } else {
            DirectPostPresentation::object_json(value, limits).ok()
        };
        let Some(presentation) = presentation else {
            return;
        };
        let Ok(result) = DirectPostQueryResult::new("degree", vec![presentation], limits) else {
            return;
        };
        if let Ok(response) = request.into_direct_post_response(vec![result], limits) {
            assert!(response.form_body_len() <= limits.max_form_body_bytes());
            assert_eq!(response.query_result_count(), 1);
        }
    } else {
        let limits = DirectPostErrorResponseLimits::default();
        let text = std::str::from_utf8(value).ok();
        let details = DirectPostErrorDetails::new(text, None, limits);
        if let Ok(details) = details {
            let response = request.into_direct_post_error_response(
                DirectPostErrorCode::InvalidRequest,
                details,
                limits,
            );
            if let Ok(response) = response {
                assert!(response.form_body_len() <= limits.max_form_body_bytes());
                assert_eq!(response.error_code(), DirectPostErrorCode::InvalidRequest);
            }
        }
    }
});

fn fixed_request() -> Option<ValidatedAuthorizationRequest> {
    bind_request(REQUEST_OBJECT, &serde_json::from_str(PUBLIC_JWK).ok()?)
}

fn dynamic_request(
    mode: &str,
    destination_name: &str,
    destination: &str,
) -> Option<ValidatedAuthorizationRequest> {
    let private = Ed25519PrivateKey::from_slice(&PRIVATE_BYTES).ok()?;
    let mut request = json!({
        "client_id": "decentralized_identifier:did:example:verifier",
        "aud": identus_oid4vp::STATIC_DISCOVERY_AUDIENCE,
        "iat": 1_735_689_480_i64,
        "response_type": "vp_token",
        "response_mode": mode,
        "nonce": "fresh-nonce_1",
        "state": "state-123",
        "dcql_query": {"credentials": [{"id": "degree", "format": "dc+sd-jwt", "meta": {}}]}
    });
    request
        .as_object_mut()?
        .insert(destination_name.to_owned(), json!(destination));
    let header = ProtectedHeader::new(
        "Ed25519",
        Some(REQUEST_OBJECT_JWT_TYPE),
        Some("did:example:verifier#key-1"),
        JwsLimits::default(),
    )
    .ok()?;
    let compact = JwsSigningInput::new(
        header,
        serde_json::to_vec(&request).ok()?,
        JwsLimits::default(),
    )
    .ok()?
    .sign_with(&Ed25519Signer::new(&private))
    .ok()?
    .compact()
    .to_owned();
    bind_request(&compact, &private.to_public_key().encode_jwk())
}

fn bind_request(compact: &str, public: &PublicKeyJwk) -> Option<ValidatedAuthorizationRequest> {
    let AuthorizationRequestInvocation::Referenced(reference) =
        AuthorizationRequestInvocation::parse(
            REFERENCE,
            AuthorizationRequestInvocationLimits::default(),
        )
        .ok()?;
    let unverified = reference
        .prepare_retrieval(
            RequestUriRetrievalInput::get(),
            RequestUriRetrievalLimits::default(),
        )
        .ok()?
        .bind_response(
            200,
            Some(REQUEST_OBJECT_MEDIA_TYPE),
            compact.as_bytes(),
            RequestObjectValidationLimits::default(),
        )
        .ok()?;
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, public).ok()?;
    let request = oid4vp_support::secure(
        unverified
            .verify(&SignatureSuiteRegistry::recommended(), &key)
            .ok()?,
    )?;
    oid4vp_support::authorize(request)?
        .into_authorization_request(
            AuthorizationRequestValidationLimits::default(),
            DcqlLimits::default(),
        )
        .ok()
}
