use std::{hint::black_box, time::Instant};

use identus_crypto::{Ed25519PrivateKey, EncodeJwk};
use identus_did_jwk::DidJwk;

const SAMPLES: usize = 101;
const ITERATIONS: usize = 500;

fn main() {
    let jwk = Ed25519PrivateKey::from_slice(&[42; 32])
        .unwrap()
        .to_public_key()
        .encode_jwk();
    let created = DidJwk::create(jwk.clone()).unwrap();
    let oversized = format!("did:jwk:{}", "A".repeat(21_847));
    measure("create", || {
        let _ = black_box(DidJwk::create(black_box(jwk.clone())).unwrap());
    });
    measure("parse", || {
        let _ = black_box(DidJwk::parse(black_box(created.did().as_str())).unwrap());
    });
    measure("resolve", || {
        let _ = black_box(created.resolve().unwrap());
    });
    measure("reject-oversize", || {
        let _ = black_box(DidJwk::parse(black_box(&oversized)).unwrap_err());
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
        "did-jwk-benchmark operation={} median_ns_per_iteration={} samples={} iterations_per_sample={}",
        black_box(name),
        observations[SAMPLES / 2],
        SAMPLES,
        ITERATIONS
    );
}
