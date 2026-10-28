use std::{hint::black_box, time::Instant};

use identus_crypto::{Ed25519PrivateKey, EncodeJwk};
use identus_jose::{
    Ed25519Signer, JwsAlgorithm, JwsLimits, JwsSigningInput, JwsVerificationKey, ProtectedHeader,
    SignatureSuiteRegistry,
};
use identus_oid4vp::{
    AuthorizationRequestInvocation, AuthorizationRequestInvocationLimits, REQUEST_OBJECT_JWT_TYPE,
    REQUEST_OBJECT_MEDIA_TYPE, RequestObjectValidationLimits, RequestUriRetrievalInput,
    RequestUriRetrievalLimits, STATIC_DISCOVERY_AUDIENCE,
};
use serde_json::json;

mod support;

const SAMPLES: usize = 21;
const ITERATIONS: usize = 20;
const PRIVATE_BYTES: [u8; 32] = [0x71; 32];
const CLIENT_ID: &str = "decentralized_identifier:did:example:verifier";
const REFERENCE: &str = "openid4vp:?client_id=decentralized_identifier%3Adid%3Aexample%3Averifier&request_uri=https%3A%2F%2Fverifier.example%2Frequest";

fn main() {
    let ordinary = signed(&json!({
        "client_id": CLIENT_ID,
        "aud": STATIC_DISCOVERY_AUDIENCE,
        "iat": support::NOW_SECONDS - 120,
        "jti": "ordinary-request"
    }));
    let exact_string = "b".repeat(RequestObjectValidationLimits::default().max_string_bytes());
    let boundary = signed(&json!({
        "client_id": CLIENT_ID,
        "aud": exact_string,
        "iat": support::NOW_SECONDS - 120,
        "exp": support::NOW_SECONDS + 180,
        "nbf": support::NOW_SECONDS - 150,
        "jti": "j".repeat(RequestObjectValidationLimits::default().max_string_bytes())
    }));

    measure("security.ordinary.end-to-end", || {
        validate(&ordinary, STATIC_DISCOVERY_AUDIENCE)
    });
    measure("security.boundary.end-to-end", || {
        validate(&boundary, &exact_string)
    });
    println!("oid4vp-request-object-security-benchmark allocation_observation=not_available");
}

fn signed(request: &serde_json::Value) -> String {
    let private = Ed25519PrivateKey::from_slice(&PRIVATE_BYTES).expect("fixed private key");
    let header = ProtectedHeader::new(
        "Ed25519",
        Some(REQUEST_OBJECT_JWT_TYPE),
        Some("did:example:verifier#key-1"),
        JwsLimits::default(),
    )
    .expect("header");
    JwsSigningInput::new(
        header,
        serde_json::to_vec(request).expect("request JSON"),
        JwsLimits::default(),
    )
    .expect("input")
    .sign_with(&Ed25519Signer::new(&private))
    .expect("signature")
    .compact()
    .to_owned()
}

fn validate(compact: &str, expected_audience: &str) {
    let AuthorizationRequestInvocation::Referenced(reference) =
        AuthorizationRequestInvocation::parse(
            REFERENCE,
            AuthorizationRequestInvocationLimits::default(),
        )
        .expect("invocation");
    let object = reference
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
        .expect("binding");
    let private = Ed25519PrivateKey::from_slice(&PRIVATE_BYTES).expect("fixed private key");
    let public = private.to_public_key().encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).expect("verification key");
    let verified = object
        .verify(&SignatureSuiteRegistry::recommended(), &key)
        .expect("verified");
    let evidence = if expected_audience == STATIC_DISCOVERY_AUDIENCE {
        support::secure(verified)
    } else {
        support::secure_for_audience(verified, expected_audience)
    };
    black_box(evidence);
}

fn measure(name: &str, mut operation: impl FnMut()) {
    for _ in 0..ITERATIONS {
        operation();
    }
    let mut observations = Vec::with_capacity(SAMPLES);
    for _ in 0..SAMPLES {
        let started = Instant::now();
        for _ in 0..ITERATIONS {
            operation();
        }
        observations.push(started.elapsed().as_nanos() / ITERATIONS as u128);
    }
    observations.sort_unstable();
    println!(
        "oid4vp-request-object-security-benchmark operation={} median_ns_per_iteration={} samples={} iterations_per_sample={}",
        black_box(name),
        observations[SAMPLES / 2],
        SAMPLES,
        ITERATIONS
    );
}
