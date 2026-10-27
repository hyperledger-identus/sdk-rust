use std::{hint::black_box, time::Instant};

use identus_did_peer::{
    numalgo2_compat::{PeerDid, PublicKey, Purpose},
    numalgo4::{InputDocument, LongForm},
};

const SAMPLES: usize = 101;
const ITERATIONS: usize = 500;

fn main() {
    let input = InputDocument::from_json_str(
        r##"{"verificationMethod":[{"id":"#key-1","type":"Multikey","publicKeyMultibase":"z6MkiToqovww7vYtxm1xNM15u9JzqzUFZ1k7s7MazYJUyAxv"}],"authentication":["#key-1"]}"##,
    )
    .unwrap();
    let long = LongForm::create(input.clone()).unwrap();
    let compatibility = PeerDid::create(
        vec![
            PublicKey::new(
                Purpose::Verification,
                "z6MkiToqovww7vYtxm1xNM15u9JzqzUFZ1k7s7MazYJUyAxv",
            )
            .unwrap(),
            PublicKey::new(
                Purpose::Encryption,
                "z6LSj72tK8brWgZja8NLRwfiAcBXQWZJL6n1AP1zFjvVUtqN",
            )
            .unwrap(),
        ],
        vec![],
    )
    .unwrap();

    measure("numalgo4.create", || {
        black_box(LongForm::create(black_box(input.clone())).unwrap());
    });
    measure("numalgo4.parse-resolve", || {
        let parsed = LongForm::parse(black_box(long.did().as_str())).unwrap();
        black_box(parsed.resolve().unwrap());
    });
    measure("numalgo2.parse-resolve", || {
        let parsed = PeerDid::parse(black_box(compatibility.did().as_str())).unwrap();
        black_box(parsed.resolve().unwrap());
    });
    println!("did-peer-benchmark allocation_observation=not_available");
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
        "did-peer-benchmark operation={} median_ns_per_iteration={} samples={} iterations_per_sample={}",
        black_box(name),
        observations[SAMPLES / 2],
        SAMPLES,
        ITERATIONS
    );
}
