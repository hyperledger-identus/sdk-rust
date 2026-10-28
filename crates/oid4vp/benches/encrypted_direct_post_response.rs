use std::{hint::black_box, time::Instant};

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use identus_crypto::{Ed25519PrivateKey, EncodeJwk, P256PrivateKey};
use identus_jose::{
    CompactJwe, Ed25519Signer, JweEncrypter, JweEncryptionError, JweEncryptionRequest, JweLimits,
    JwsAlgorithm, JwsLimits, JwsSigningInput, JwsVerificationKey, ProtectedHeader,
    SignatureSuiteRegistry,
};
use identus_oid4vp::{
    AuthorizationRequestInvocation, AuthorizationRequestInvocationLimits,
    AuthorizationRequestValidationLimits, DcqlLimits, DirectPostPresentation,
    DirectPostQueryResult, DirectPostResponseLimits, REQUEST_OBJECT_JWT_TYPE,
    REQUEST_OBJECT_MEDIA_TYPE, RequestObjectValidationLimits, RequestUriRetrievalInput,
    RequestUriRetrievalLimits, ValidatedAuthorizationRequest,
};
use serde_json::json;

mod support;

const SAMPLES: usize = 21;
const ITERATIONS: usize = 10;
const SIGNING_KEY: [u8; 32] = [0x71; 32];
const RESPONSE_KEY: [u8; 32] = [0x42; 32];
const REFERENCE: &str = "openid4vp:?client_id=decentralized_identifier%3Adid%3Aexample%3Averifier&request_uri=https%3A%2F%2Fverifier.example%2Frequest";

fn main() {
    measure("ordinary.end-to-end", || construct("compact"));
    let boundary = "p".repeat(60 * 1_024);
    measure("boundary.end-to-end", || construct(black_box(&boundary)));
    println!(
        "oid4vp-encrypted-direct-post-benchmark allocation_observation=not_available engine=deterministic-test-port"
    );
}

fn construct(presentation: &str) {
    let response_limits = DirectPostResponseLimits::default();
    let jwe_limits = JweLimits::new(128 * 1_024, 262_000, 128).expect("limits");
    let result = DirectPostQueryResult::new(
        "degree",
        vec![DirectPostPresentation::string(presentation, response_limits).expect("presentation")],
        response_limits,
    )
    .expect("result");
    black_box(
        validated_request(jwe_limits)
            .into_encrypted_direct_post_response(
                vec![result],
                response_limits,
                jwe_limits,
                &DeterministicPort,
            )
            .expect("response"),
    );
}

struct DeterministicPort;

impl JweEncrypter for DeterministicPort {
    fn encrypt(&self, request: JweEncryptionRequest) -> Result<CompactJwe, JweEncryptionError> {
        let point = P256PrivateKey::from_slice(&[0x24; 32])
            .expect("ephemeral key")
            .to_public_key()
            .encode_uncompressed();
        let header = json!({
            "alg": request.key_management_algorithm().as_str(),
            "enc": request.content_encryption_algorithm().as_str(),
            "kid": request.recipient().expose_key_id(),
            "epk": {
                "kty": "EC",
                "crv": "P-256",
                "x": URL_SAFE_NO_PAD.encode(&point[1..33]),
                "y": URL_SAFE_NO_PAD.encode(&point[33..65])
            }
        });
        let compact = format!(
            "{}..{}.{}.{}",
            URL_SAFE_NO_PAD.encode(serde_json::to_vec(&header).expect("header")),
            URL_SAFE_NO_PAD.encode([1_u8; 12]),
            URL_SAFE_NO_PAD.encode(request.expose_sensitive_plaintext()),
            URL_SAFE_NO_PAD.encode([2_u8; 16])
        );
        CompactJwe::new(compact, request.max_compact_bytes())
    }
}

fn validated_request(jwe_limits: JweLimits) -> ValidatedAuthorizationRequest {
    let signing = Ed25519PrivateKey::from_slice(&SIGNING_KEY).expect("signing key");
    let recipient = P256PrivateKey::from_slice(&RESPONSE_KEY)
        .expect("recipient key")
        .to_public_key()
        .encode_uncompressed();
    let payload = serde_json::to_vec(&json!({
        "client_id": "decentralized_identifier:did:example:verifier",
        "aud": identus_oid4vp::STATIC_DISCOVERY_AUDIENCE,
        "iat": support::NOW_SECONDS - 120,
        "response_type": "vp_token",
        "response_mode": "direct_post.jwt",
        "nonce": "fresh-nonce_1",
        "response_uri": "https://verifier.example/response",
        "state": "state-123",
        "client_metadata": {
            "jwks": {"keys": [{
                "kty": "EC",
                "crv": "P-256",
                "alg": "ECDH-ES",
                "use": "enc",
                "key_ops": ["deriveKey"],
                "kid": "response-key",
                "x": URL_SAFE_NO_PAD.encode(&recipient[1..33]),
                "y": URL_SAFE_NO_PAD.encode(&recipient[33..65])
            }]},
            "encrypted_response_enc_values_supported": ["A128GCM"]
        },
        "dcql_query": {"credentials": [{
            "id": "degree",
            "format": "dc+sd-jwt",
            "meta": {}
        }]}
    }))
    .expect("payload");
    let header = ProtectedHeader::new(
        "Ed25519",
        Some(REQUEST_OBJECT_JWT_TYPE),
        Some("did:example:verifier#key-1"),
        JwsLimits::default(),
    )
    .expect("header");
    let compact = JwsSigningInput::new(header, payload, JwsLimits::default())
        .expect("input")
        .sign_with(&Ed25519Signer::new(&signing))
        .expect("signature")
        .compact()
        .to_owned();
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
    let public = signing.to_public_key().encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).expect("verification key");
    support::secure(
        object
            .verify(&SignatureSuiteRegistry::recommended(), &key)
            .expect("verified"),
    )
    .into_authorization_request_with_jwe_limits(
        AuthorizationRequestValidationLimits::default(),
        DcqlLimits::default(),
        jwe_limits,
    )
    .expect("validated")
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
        "oid4vp-encrypted-direct-post-benchmark operation={} median_ns_per_iteration={} samples={} iterations_per_sample={}",
        black_box(name),
        observations[SAMPLES / 2],
        SAMPLES,
        ITERATIONS
    );
}
