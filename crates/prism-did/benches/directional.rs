use std::{hint::black_box, time::Instant};

use identus_crypto::Secp256k1PrivateKey;
use identus_prism_did::{
    PrismDidState, RawPrismOperation, SignedPrismOperation, project_did_document,
};

const SAMPLES: usize = 101;
const PARSE_ITERATIONS: usize = 500;
const CRYPTO_ITERATIONS: usize = 50;

fn main() {
    let fixtures = Fixtures::new();

    measure("parse.ordinary", PARSE_ITERATIONS, || {
        black_box(RawPrismOperation::parse(black_box(&fixtures.create)).unwrap());
    });
    measure("parse.maximum-profile", PARSE_ITERATIONS, || {
        black_box(RawPrismOperation::parse(black_box(&fixtures.maximum)).unwrap());
    });
    measure("verify.signed-create", CRYPTO_ITERATIONS, || {
        black_box(PrismDidState::from_signed_create(black_box(&fixtures.signed_create)).unwrap());
    });
    measure("project.maximum-profile", CRYPTO_ITERATIONS, || {
        black_box(project_did_document(black_box(&fixtures.maximum_state)).unwrap());
    });
    measure("state.atomic-update", CRYPTO_ITERATIONS, || {
        let mut state = black_box(fixtures.ordinary_state.clone());
        state
            .apply_signed(black_box(&fixtures.signed_update))
            .unwrap();
        black_box(state);
    });

    println!("did-prism-benchmark allocation_observation=not_available");
}

fn measure(mut name: &str, iterations: usize, mut operation: impl FnMut()) {
    for _ in 0..iterations {
        operation();
    }
    let mut observations = Vec::with_capacity(SAMPLES);
    for _ in 0..SAMPLES {
        let started = Instant::now();
        for _ in 0..iterations {
            operation();
        }
        observations.push(started.elapsed().as_nanos() / iterations as u128);
    }
    observations.sort_unstable();
    let median = observations[SAMPLES / 2];
    name = black_box(name);
    println!(
        "did-prism-benchmark operation={name} median_ns_per_iteration={median} samples={SAMPLES} iterations_per_sample={iterations}"
    );
}

struct Fixtures {
    create: Vec<u8>,
    maximum: Vec<u8>,
    signed_create: SignedPrismOperation,
    signed_update: SignedPrismOperation,
    ordinary_state: PrismDidState,
    maximum_state: PrismDidState,
}

impl Fixtures {
    fn new() -> Self {
        let private = Secp256k1PrivateKey::from_slice(&[1; 32]).unwrap();
        let public = private.to_public_key().encode_compressed();
        let create = create_operation(&public, 2, 1, 1);
        let maximum = create_operation(&public, 50, 50, 127);
        let signed_create = signed(&private, "master-0", &create);
        let ordinary_state = PrismDidState::from_signed_create(&signed_create).unwrap();
        let maximum_state =
            PrismDidState::from_initial_state(&RawPrismOperation::parse(&maximum).unwrap())
                .unwrap();
        let update = update_operation(&ordinary_state);
        let signed_update = signed(&private, "master-0", &update);
        Self {
            create,
            maximum,
            signed_create,
            signed_update,
            ordinary_state,
            maximum_state,
        }
    }
}

fn create_operation(
    public: &[u8],
    key_count: usize,
    service_count: usize,
    context_count: usize,
) -> Vec<u8> {
    let mut data = Vec::new();
    for index in 0..key_count {
        let mut compressed = Vec::new();
        push_bytes(&mut compressed, 1, b"secp256k1");
        push_bytes(&mut compressed, 2, public);

        let mut key = Vec::new();
        let id = (index != 0).then(|| format!("auth-{index}"));
        push_bytes(&mut key, 1, id.as_deref().unwrap_or("master-0").as_bytes());
        push_varint_field(&mut key, 2, if index == 0 { 1 } else { 4 });
        push_bytes(&mut key, 9, &compressed);
        push_bytes(&mut data, 2, &key);
    }
    for index in 0..service_count {
        let mut service = Vec::new();
        push_bytes(&mut service, 1, format!("service-{index}").as_bytes());
        push_bytes(&mut service, 2, b"LinkedDomains");
        push_bytes(
            &mut service,
            3,
            format!("https://example.com/service/{index}").as_bytes(),
        );
        push_bytes(&mut data, 3, &service);
    }
    for index in 0..context_count {
        push_bytes(
            &mut data,
            4,
            format!("https://example.com/context/{index}").as_bytes(),
        );
    }
    let mut create = Vec::new();
    push_bytes(&mut create, 1, &data);
    let mut operation = Vec::new();
    push_bytes(&mut operation, 1, &create);
    operation
}

fn update_operation(state: &PrismDidState) -> Vec<u8> {
    let mut patch = Vec::new();
    push_bytes(&mut patch, 1, b"https://example.com/updated-context");
    let mut action = Vec::new();
    push_bytes(&mut action, 6, &patch);
    let mut update = Vec::new();
    push_bytes(&mut update, 1, state.last_operation_hash().as_bytes());
    push_bytes(&mut update, 2, state.did().suffix().as_bytes());
    push_bytes(&mut update, 3, &action);
    let mut operation = Vec::new();
    push_bytes(&mut operation, 2, &update);
    operation
}

fn signed(
    private: &Secp256k1PrivateKey,
    signed_with: &str,
    operation: &[u8],
) -> SignedPrismOperation {
    let signature = private.sign(operation);
    let mut envelope = Vec::new();
    push_bytes(&mut envelope, 1, signed_with.as_bytes());
    push_bytes(&mut envelope, 2, &signature);
    push_bytes(&mut envelope, 3, operation);
    SignedPrismOperation::parse(&envelope).unwrap()
}

fn push_bytes(output: &mut Vec<u8>, field: u64, bytes: &[u8]) {
    push_varint(output, field << 3 | 2);
    push_varint(output, bytes.len() as u64);
    output.extend_from_slice(bytes);
}

fn push_varint_field(output: &mut Vec<u8>, field: u64, value: u64) {
    push_varint(output, field << 3);
    push_varint(output, value);
}

fn push_varint(output: &mut Vec<u8>, mut value: u64) {
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        output.push(byte);
        if value == 0 {
            return;
        }
    }
}
