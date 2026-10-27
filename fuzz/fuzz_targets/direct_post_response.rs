#![no_main]

use identus_crypto::PublicKeyJwk;
use identus_jose::{JwsAlgorithm, JwsVerificationKey, SignatureSuiteRegistry};
use identus_oid4vp::{
    AuthorizationRequestInvocation, AuthorizationRequestInvocationLimits,
    AuthorizationRequestValidationLimits, DcqlLimits, DirectPostErrorCode,
    DirectPostErrorDetails, DirectPostErrorResponseLimits, DirectPostPresentation,
    DirectPostQueryResult, DirectPostResponseLimits, REQUEST_OBJECT_MEDIA_TYPE,
    RequestObjectValidationLimits, RequestUriRetrievalInput, RequestUriRetrievalLimits,
};
use libfuzzer_sys::fuzz_target;

const REFERENCE: &str = "openid4vp:?client_id=decentralized_identifier%3Adid%3Aexample%3Averifier&request_uri=https%3A%2F%2Fverifier.example%2Frequest";
const REQUEST_OBJECT: &str = "eyJhbGciOiJFZDI1NTE5IiwidHlwIjoib2F1dGgtYXV0aHotcmVxK2p3dCIsImtpZCI6ImRpZDpleGFtcGxlOnZlcmlmaWVyI2tleS0xIn0.eyJjbGllbnRfaWQiOiJkZWNlbnRyYWxpemVkX2lkZW50aWZpZXI6ZGlkOmV4YW1wbGU6dmVyaWZpZXIiLCJyZXNwb25zZV90eXBlIjoidnBfdG9rZW4iLCJyZXNwb25zZV9tb2RlIjoiZGlyZWN0X3Bvc3QiLCJub25jZSI6ImZyZXNoLW5vbmNlXzEiLCJyZXNwb25zZV91cmkiOiJodHRwczovL3ZlcmlmaWVyLmV4YW1wbGUvcmVzcG9uc2UiLCJzdGF0ZSI6InN0YXRlLTEyMyIsImRjcWxfcXVlcnkiOnsiY3JlZGVudGlhbHMiOlt7ImlkIjoiZGVncmVlIiwiZm9ybWF0IjoiZGMrc2Qtand0IiwibWV0YSI6e319XX19.daneYuPWoerhkVN0-BMpQvCmb6u-R3XPEnBwjmnh4RI_hSuSvWEz7Vf8hhD_nkuwYQL8qpr2pniNBoxp9xgKCg";
const PUBLIC_JWK: &str = r#"{"kty":"OKP","crv":"Ed25519","x":"4v4qObcyZkKCfW2C1JdiLNbCl_54Jw6qQ2sB8Ia288s"}"#;

fuzz_target!(|data: &[u8]| {
    let Some((&kind, value)) = data.split_first() else {
        return;
    };
    let AuthorizationRequestInvocation::Referenced(reference) =
        AuthorizationRequestInvocation::parse(
            REFERENCE,
            AuthorizationRequestInvocationLimits::default(),
        )
        .expect("fixed invocation");
    let unverified = reference
        .prepare_retrieval(
            RequestUriRetrievalInput::get(),
            RequestUriRetrievalLimits::default(),
        )
        .expect("fixed retrieval")
        .bind_response(
            200,
            Some(REQUEST_OBJECT_MEDIA_TYPE),
            REQUEST_OBJECT.as_bytes(),
            RequestObjectValidationLimits::default(),
        )
        .expect("fixed response");
    let public: PublicKeyJwk = serde_json::from_str(PUBLIC_JWK).expect("fixed public JWK");
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).expect("fixed key");
    let request = unverified
        .verify(&SignatureSuiteRegistry::recommended(), &key)
        .expect("fixed signature")
        .into_authorization_request(
            AuthorizationRequestValidationLimits::default(),
            DcqlLimits::default(),
        )
        .expect("fixed authorization request");
    if kind & 2 == 0 {
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
