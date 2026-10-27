use std::{collections::HashMap, hint::black_box, time::Instant};

use identus_crypto::{Secp256k1PrivateKey, Sha256Digest, sha256};
use identus_prism_did::{
    CanonicalPrismDid, CreateDidOperation, KeyUsage, PrismDidState, PrismPublicKey,
    PrismPublicKeyMaterial, RawPrismOperation, SignedPrismOperation,
};
use identus_prism_indexer::{
    Cursor, DidStateLookup, IncomingRecord, TransactionBatch, TransactionIdentity, plan_transaction,
};
use identus_prism_vdr::{
    EntrySnapshot, LookupError, TraversalBudget, VdrLookup, VdrNode, VdrOperation, resolve_entry,
};

const SAMPLES: usize = 101;
const ITERATIONS: usize = 100;

#[derive(Default)]
struct Lookup {
    nodes: HashMap<Sha256Digest, VdrNode>,
    entries: HashMap<Sha256Digest, EntrySnapshot>,
}

impl DidStateLookup for Lookup {
    fn did_state(&self, _: &CanonicalPrismDid) -> Result<Option<PrismDidState>, LookupError> {
        Ok(None)
    }
}

impl VdrLookup for Lookup {
    fn node(&self, hash: &Sha256Digest) -> Result<Option<VdrNode>, LookupError> {
        Ok(self.nodes.get(hash).cloned())
    }

    fn entry(&self, root: &Sha256Digest) -> Result<Option<EntrySnapshot>, LookupError> {
        Ok(self.entries.get(root).cloned())
    }
}

fn main() {
    let (signed_did, signed_vdr) = fixtures();
    let parsed_vdr = VdrOperation::parse(&signed_vdr).unwrap();
    let batch = batch(vec![signed_did, signed_vdr.clone()]);
    let (head, traversal) = traversal_fixture(100);

    measure("vdr.parse", || {
        black_box(VdrOperation::parse(black_box(&signed_vdr)).unwrap());
    });
    measure("vdr.traverse-100", || {
        let mut budget = TraversalBudget::default();
        black_box(
            resolve_entry(
                black_box(head),
                &HashMap::new(),
                &HashMap::new(),
                black_box(&traversal),
                &mut budget,
            )
            .unwrap(),
        );
    });
    measure("indexer.batch-128", || {
        black_box(batch_of_repeated(black_box(&signed_vdr), 128));
    });
    measure("indexer.plan-did-vdr", || {
        black_box(plan_transaction(black_box(&batch), &Lookup::default()).unwrap());
    });
    black_box(parsed_vdr);
    println!("prism-vdr-indexer-benchmark allocation_observation=not_available");
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
        "prism-vdr-indexer-benchmark operation={name} median_ns_per_iteration={} samples={SAMPLES} iterations_per_sample={ITERATIONS}",
        observations[SAMPLES / 2]
    );
}

fn fixtures() -> (SignedPrismOperation, SignedPrismOperation) {
    let master = Secp256k1PrivateKey::from_slice(&[1; 32]).unwrap();
    let vdr = Secp256k1PrivateKey::from_slice(&[2; 32]).unwrap();
    let create = CreateDidOperation::new(
        vec![
            PrismPublicKey::new(
                "master-0",
                KeyUsage::Master,
                PrismPublicKeyMaterial::Secp256k1(master.to_public_key()),
            )
            .unwrap(),
            PrismPublicKey::new(
                "vdr-0",
                KeyUsage::Vdr,
                PrismPublicKeyMaterial::Secp256k1(vdr.to_public_key()),
            )
            .unwrap(),
        ],
        Vec::new(),
        Vec::new(),
    )
    .unwrap();
    let raw = create.to_raw_operation().unwrap();
    let signed_did =
        SignedPrismOperation::from_signature("master-0", &master.sign(raw.as_bytes()), &raw)
            .unwrap();
    let mut payload = Vec::new();
    push_bytes(&mut payload, 1, raw.operation_hash().as_bytes());
    push_bytes(&mut payload, 50, &[1]);
    push_bytes(&mut payload, 100, b"benchmark");
    let mut operation = Vec::new();
    push_bytes(&mut operation, 7, &payload);
    let raw_vdr = RawPrismOperation::parse(&operation).unwrap();
    let signed_vdr =
        SignedPrismOperation::from_signature("vdr-0", &vdr.sign(raw_vdr.as_bytes()), &raw_vdr)
            .unwrap();
    (signed_did, signed_vdr)
}

fn batch(operations: Vec<SignedPrismOperation>) -> TransactionBatch {
    let identity = TransactionIdentity::new(b"benchmark-transaction".to_vec()).unwrap();
    let records = operations
        .into_iter()
        .enumerate()
        .map(|(index, signed_operation)| IncomingRecord {
            transaction_identity: identity.clone(),
            operation_index: index as u16,
            signed_operation,
        })
        .collect::<Vec<_>>();
    TransactionBatch::new(
        identity,
        records.len() as u16,
        Cursor::new(b"before".to_vec()).unwrap(),
        Cursor::new(b"after".to_vec()).unwrap(),
        records,
    )
    .unwrap()
}

fn batch_of_repeated(operation: &SignedPrismOperation, count: u16) -> TransactionBatch {
    batch((0..count).map(|_| operation.clone()).collect())
}

fn traversal_fixture(node_count: usize) -> (Sha256Digest, Lookup) {
    let root = sha256(0_u64.to_be_bytes());
    let owner = CanonicalPrismDid::from_digest(&[3; 32]);
    let mut previous = root;
    let mut nodes = HashMap::from([(
        root,
        VdrNode {
            operation_hash: root,
            predecessor: None,
            owner: Some(owner),
        },
    )]);
    for position in 1..node_count {
        let hash = sha256((position as u64).to_be_bytes());
        nodes.insert(
            hash,
            VdrNode {
                operation_hash: hash,
                predecessor: Some(previous),
                owner: None,
            },
        );
        previous = hash;
    }
    let entries = HashMap::from([(
        root,
        EntrySnapshot {
            head: previous,
            terminal: false,
        },
    )]);
    (previous, Lookup { nodes, entries })
}

fn push_bytes(target: &mut Vec<u8>, field: u64, value: &[u8]) {
    push_varint(field << 3 | 2, target);
    push_varint(value.len() as u64, target);
    target.extend_from_slice(value);
}

fn push_varint(mut value: u64, target: &mut Vec<u8>) {
    while value >= 0x80 {
        target.push((value as u8 & 0x7f) | 0x80);
        value >>= 7;
    }
    target.push(value as u8);
}
