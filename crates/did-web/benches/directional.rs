use std::{hint::black_box, time::Instant};

use identus_did_web::DidWeb;

const SAMPLES: usize = 101;
const ITERATIONS: usize = 500;

fn main() {
    let ordinary = "did:web:example.com:users:alice";
    let parsed = DidWeb::parse(ordinary).unwrap();
    let request = parsed.request();
    let document = br#"{"id":"did:web:example.com:users:alice"}"#.to_vec();
    let oversized = format!("did:web:{}", "a".repeat(2_049));
    measure("parse-derive", || {
        let _ = black_box(DidWeb::parse(black_box(ordinary)).unwrap());
    });
    measure("bind-response", || {
        let response = request
            .response(200, Some("application/did+json"), document.clone())
            .unwrap();
        let _ = black_box(parsed.bind_response(response).unwrap());
    });
    measure("reject-oversize", || {
        let _ = black_box(DidWeb::parse(black_box(&oversized)).unwrap_err());
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
        "did-web-benchmark operation={} median_ns_per_iteration={} samples={} iterations_per_sample={}",
        black_box(name),
        observations[SAMPLES / 2],
        SAMPLES,
        ITERATIONS
    );
}
