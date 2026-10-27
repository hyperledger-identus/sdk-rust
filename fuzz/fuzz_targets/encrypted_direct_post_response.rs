#![no_main]

use identus_crypto::{Ed25519PrivateKey, EncodeJwk};
use identus_jose::{
    CompactJwe, Ed25519Signer, JweLimits, JwsAlgorithm, JwsLimits, JwsSigningInput,
    JwsVerificationKey, ProtectedHeader, SignatureSuiteRegistry,
};
use identus_oid4vp::{
    AuthorizationRequestInvocation, AuthorizationRequestInvocationLimits,
    AuthorizationRequestValidationLimits, DcqlLimits, REQUEST_OBJECT_JWT_TYPE,
    REQUEST_OBJECT_MEDIA_TYPE, RequestObjectValidationLimits, RequestUriRetrievalInput,
    RequestUriRetrievalLimits,
};
use libfuzzer_sys::fuzz_target;

const REFERENCE: &str = "openid4vp:?client_id=decentralized_identifier%3Adid%3Aexample%3Averifier&request_uri=https%3A%2F%2Fverifier.example%2Frequest";
const PRIVATE: [u8; 32] = [0x71; 32];

fuzz_target!(|data: &[u8]| {
    if let Ok(compact) = std::str::from_utf8(data) {
        let _ = CompactJwe::new(compact.to_owned(), 65_536);
    }

    let Some(metadata) = std::str::from_utf8(data)
        .ok()
        .filter(|value| value.len() <= 48_000)
    else {
        return;
    };
    let payload = format!(
        r#"{{"client_id":"decentralized_identifier:did:example:verifier","response_type":"vp_token","response_mode":"direct_post.jwt","nonce":"n","response_uri":"https://verifier.example/response","client_metadata":{},"dcql_query":{{"credentials":[{{"id":"degree","format":"dc+sd-jwt","meta":{{}}}}]}}}}"#,
        metadata
    );
    let private = Ed25519PrivateKey::from_slice(&PRIVATE).expect("fixed private key");
    let header = ProtectedHeader::new(
        "Ed25519",
        Some(REQUEST_OBJECT_JWT_TYPE),
        Some("did:example:verifier#key-1"),
        JwsLimits::default(),
    )
    .expect("fixed header");
    let Ok(input) = JwsSigningInput::new(header, payload.into_bytes(), JwsLimits::default()) else {
        return;
    };
    let compact = input
        .sign_with(&Ed25519Signer::new(&private))
        .expect("fixed signer")
        .compact()
        .to_owned();
    let AuthorizationRequestInvocation::Referenced(reference) =
        AuthorizationRequestInvocation::parse(
            REFERENCE,
            AuthorizationRequestInvocationLimits::default(),
        )
        .expect("fixed invocation");
    let Ok(object) = reference
        .prepare_retrieval(
            RequestUriRetrievalInput::get(),
            RequestUriRetrievalLimits::default(),
        )
        .expect("fixed retrieval")
        .bind_response(
            200,
            Some(REQUEST_OBJECT_MEDIA_TYPE),
            compact.as_bytes(),
            RequestObjectValidationLimits::default(),
        )
    else {
        return;
    };
    let public = private.to_public_key().encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).expect("fixed key");
    let Ok(verified) = object.verify(&SignatureSuiteRegistry::recommended(), &key) else {
        return;
    };
    let _ = verified.into_authorization_request_with_jwe_limits(
        AuthorizationRequestValidationLimits::default(),
        DcqlLimits::default(),
        JweLimits::default(),
    );
});
