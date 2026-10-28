use std::{hint::black_box, time::Instant};

use identus_crypto::Ed25519PrivateKey;
use identus_did_key::DidKey;

const SAMPLES: usize = 101;
const ITERATIONS: usize = 500;

fn main() {
    let key = Ed25519PrivateKey::from_slice(&[42; 32])
        .unwrap()
        .to_public_key();
    let created = DidKey::create(key).unwrap();
    measure("create", || {
        let _ = black_box(DidKey::create(black_box(key)).unwrap());
    });
    measure("parse", || {
        let _ = black_box(DidKey::parse(black_box(created.did().as_str())).unwrap());
    });
    measure("resolve", || {
        let _ = black_box(created.resolve().unwrap());
    });
    measure("reject-oversize", || {
        let input = format!("did:key:z{}", "1".repeat(129));
        let _ = black_box(DidKey::parse(black_box(&input)).unwrap_err());
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
        "did-key-benchmark operation={} median_ns_per_iteration={} samples={} iterations_per_sample={}",
        black_box(name),
        observations[SAMPLES / 2],
        SAMPLES,
        ITERATIONS
    );
}
