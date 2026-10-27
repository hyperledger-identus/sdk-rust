#![no_main]

use identus_prism_did::{RawPrismOperation, SignedPrismOperation};
use identus_prism_indexer::{Cursor, IncomingRecord, TransactionBatch, TransactionIdentity};
use libfuzzer_sys::fuzz_target;

fn push_varint(mut value: u64, target: &mut Vec<u8>) {
    while value >= 0x80 {
        target.push((value as u8 & 0x7f) | 0x80);
        value >>= 7;
    }
    target.push(value as u8);
}

fn push_bytes(target: &mut Vec<u8>, field: u64, value: &[u8]) {
    push_varint(field << 3 | 2, target);
    push_varint(value.len() as u64, target);
    target.extend_from_slice(value);
}

fn signed(marker: u8, content: &[u8]) -> Option<SignedPrismOperation> {
    let mut payload = Vec::new();
    push_bytes(&mut payload, 1, &[marker; 32]);
    push_bytes(&mut payload, 100, content);
    let mut operation = Vec::new();
    push_bytes(&mut operation, 7, &payload);
    let raw = RawPrismOperation::parse(&operation).ok()?;
    SignedPrismOperation::from_signature("vdr-0", &[marker; 64], &raw).ok()
}

fuzz_target!(|data: &[u8]| {
    let marker = data.first().copied().unwrap_or(1).max(1);
    let identity_len = data
        .get(1)
        .copied()
        .map_or(1, |value| usize::from(value).max(1));
    let identity_bytes = vec![marker; identity_len];
    let identity = TransactionIdentity::new(identity_bytes).expect("fuzzed identity is bounded");
    let count = data.get(2).copied().map_or(1, |value| value % 16 + 1);
    let content = data.get(3..).unwrap_or_default();
    let mut records = Vec::with_capacity(usize::from(count));
    for index in 0..count {
        let Some(signed_operation) = signed(marker.wrapping_add(index), content) else {
            return;
        };
        records.push(IncomingRecord {
            transaction_identity: identity.clone(),
            operation_index: u16::from(index),
            signed_operation,
        });
    }
    let batch = TransactionBatch::new(
        identity,
        u16::from(count),
        Cursor::new(b"before".to_vec()).unwrap(),
        Cursor::new(b"after".to_vec()).unwrap(),
        records,
    );
    if let Ok(batch) = batch {
        assert_eq!(batch.records().len(), usize::from(count));
        assert_eq!(batch.transition_digest(), batch.transition_digest());
    }
});
