#![no_main]

use identus_crypto::{Ed25519PrivateKey, EncodeJwk};
use identus_jose::{
    Ed25519Signer, JwsAlgorithm, JwsLimits, JwsSigningInput, JwsVerificationKey, ProtectedHeader,
    SignatureSuiteRegistry,
};
use identus_oid4vp::{
    AuthorizationRequestInvocation, AuthorizationRequestInvocationLimits, REQUEST_OBJECT_JWT_TYPE,
    REQUEST_OBJECT_MEDIA_TYPE, RequestObjectValidationLimits, RequestUriRetrievalInput,
    RequestUriRetrievalLimits,
};
use libfuzzer_sys::fuzz_target;
use serde_json::json;

mod oid4vp_support;

const PRIVATE: [u8; 32] = [0x71; 32];
const REFERENCE: &str = "openid4vp:?client_id=decentralized_identifier%3Adid%3Aexample%3Averifier&request_uri=https%3A%2F%2Fverifier.example%2Frequest";

fuzz_target!(|data: &[u8]| {
    let Some((time_bytes, audience_bytes)) = data.split_at_checked(8) else {
        return;
    };
    let issued_at = u64::from_le_bytes(time_bytes.try_into().expect("eight bytes"));
    let Ok(audience) = std::str::from_utf8(audience_bytes) else {
        return;
    };
    let payload = serde_json::to_vec(&json!({
        "client_id": "decentralized_identifier:did:example:verifier",
        "aud": audience,
        "iat": issued_at,
        "jti": audience,
    }))
    .expect("JSON");
    let private = Ed25519PrivateKey::from_slice(&PRIVATE).expect("fixed private key");
    let header = ProtectedHeader::new(
        "Ed25519",
        Some(REQUEST_OBJECT_JWT_TYPE),
        Some("did:example:verifier#key-1"),
        JwsLimits::default(),
    )
    .expect("header");
    let Ok(input) = JwsSigningInput::new(header, payload, JwsLimits::default()) else {
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
        .expect("invocation");
    let Ok(object) = reference
        .prepare_retrieval(
            RequestUriRetrievalInput::get(),
            RequestUriRetrievalLimits::default(),
        )
        .expect("retrieval")
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
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).expect("key");
    if let Ok(verified) = object.verify(&SignatureSuiteRegistry::recommended(), &key) {
        let _ = oid4vp_support::secure(verified);
    }
});
