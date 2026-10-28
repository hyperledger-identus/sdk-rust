use std::{hint::black_box, time::Instant};

use identus_crypto::{Ed25519PrivateKey, EncodeJwk, SecureRandom};
use identus_jose::{Ed25519Signer, JwsAlgorithm, JwsVerificationKey, SignatureSuiteRegistry};
use identus_sd_jwt::{
    ClaimPath, IssuancePlan, PathSegment, SdJwtLimits, SelectionPlan, UnverifiedSdJwt, issue,
};
use serde_json::json;

const SAMPLES: usize = 101;
const ITERATIONS: usize = 500;

struct Entropy(u8);
impl SecureRandom for Entropy {
    fn fill_bytes(&mut self, output: &mut [u8]) -> Result<(), identus_crypto::Error> {
        for byte in output {
            *byte = self.0;
            self.0 = self.0.wrapping_add(1);
        }
        Ok(())
    }
}

fn main() {
    let limits = SdJwtLimits::default();
    let private = Ed25519PrivateKey::from_slice(&[41; 32]).unwrap();
    let public = private.to_public_key().encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).unwrap();
    let suites = SignatureSuiteRegistry::recommended();
    let city = ClaimPath::new(
        vec![
            PathSegment::Member("address".to_owned()),
            PathSegment::Member("city".to_owned()),
        ],
        limits,
    )
    .unwrap();
    let issued = issue(
        json!({"iss":"issuer", "address":{"city":"Bratislava", "street":"Main"}}),
        &IssuancePlan {
            disclose: vec![city.clone()],
            decoy_locations: Vec::new(),
            salt_bytes: 16,
        },
        &mut Entropy(1),
        &Ed25519Signer::new(&private),
        limits,
    )
    .unwrap();
    let compact = issued.as_str().to_owned();

    measure("parse", || {
        black_box(UnverifiedSdJwt::parse(black_box(&compact), limits).unwrap());
    });
    measure("verify-process-select", || {
        let verified = UnverifiedSdJwt::parse(black_box(&compact), limits)
            .unwrap()
            .verify_issuer(&suites, &key)
            .unwrap()
            .process_disclosures()
            .unwrap();
        black_box(
            verified
                .select(&SelectionPlan {
                    paths: vec![city.clone()],
                })
                .unwrap(),
        );
    });
    let oversized = "x".repeat(limits.compact_bytes + 1);
    measure("reject-oversize", || {
        black_box(UnverifiedSdJwt::parse(black_box(&oversized), limits).unwrap_err());
    });
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
        "sd-jwt-benchmark operation={} median_ns_per_iteration={} samples={} iterations_per_sample={}",
        black_box(name),
        observations[SAMPLES / 2],
        SAMPLES,
        ITERATIONS
    );
}
