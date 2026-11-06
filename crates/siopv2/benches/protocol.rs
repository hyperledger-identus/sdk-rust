use std::{hint::black_box, time::Instant};

use identus_crypto::{Ed25519PrivateKey, EncodeJwk};
use identus_jose::{
    Ed25519Signer, JwsAlgorithm, JwsLimits, JwsSigningInput, JwsVerificationKey, ProtectedHeader,
    SignatureSuiteRegistry,
};
use identus_siopv2::{REQUEST_OBJECT_JWT_TYPE, Siopv2Limits, UnverifiedSiopRequest};
use serde_json::json;

const SAMPLES: usize = 21;
const ITERATIONS: usize = 20;
const PRIVATE_BYTES: [u8; 32] = [0x31; 32];

fn main() {
    let compact = fixed_request();
    measure("request.verify", || verify(black_box(&compact)));
    println!("siopv2-benchmark allocation_observation=not_available");
}

fn fixed_request() -> String {
    let private = Ed25519PrivateKey::from_slice(&PRIVATE_BYTES).unwrap();
    let header = ProtectedHeader::new(
        "Ed25519",
        Some(REQUEST_OBJECT_JWT_TYPE),
        Some("did:example:rp#key-1"),
        JwsLimits::default(),
    )
    .unwrap();
    let payload = serde_json::to_vec(&json!({
        "iss": "https://rp.example", "aud": "https://self-issued.me/v2",
        "client_id": "https://rp.example", "redirect_uri": "https://rp.example/direct-post",
        "response_type": "id_token", "response_mode": "direct_post", "scope": "openid",
        "id_token_type": "subject_signed_id_token", "nonce": "n", "iat": 1800000000_i64,
        "exp": 1800000300_i64
    }))
    .unwrap();
    JwsSigningInput::new(header, payload, JwsLimits::default())
        .unwrap()
        .sign_with(&Ed25519Signer::new(&private))
        .unwrap()
        .compact()
        .to_owned()
}

fn verify(compact: &str) {
    let private = Ed25519PrivateKey::from_slice(&PRIVATE_BYTES).unwrap();
    let public = private.to_public_key().encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).unwrap();
    black_box(
        UnverifiedSiopRequest::parse(compact, Siopv2Limits::default())
            .unwrap()
            .verify(&SignatureSuiteRegistry::recommended(), &key, 1_800_000_001)
            .unwrap(),
    );
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
        "siopv2-benchmark operation={} median_ns_per_iteration={} samples={} iterations_per_sample={}",
        name,
        observations[SAMPLES / 2],
        SAMPLES,
        ITERATIONS
    );
}
