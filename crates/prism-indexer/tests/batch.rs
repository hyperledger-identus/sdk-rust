use std::collections::HashSet;

use identus_prism_did::{RawPrismOperation, SignedPrismOperation};
use identus_prism_indexer::{
    Cursor, ErrorKind, IncomingRecord, TransactionBatch, TransactionIdentity,
};

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

fn signed(marker: u8) -> SignedPrismOperation {
    let mut payload = Vec::new();
    push_bytes(&mut payload, 1, &[marker; 32]);
    let mut operation = Vec::new();
    push_bytes(&mut operation, 7, &payload);
    let raw = RawPrismOperation::parse(&operation).unwrap();
    SignedPrismOperation::from_signature("vdr-0", &[marker; 64], &raw).unwrap()
}

fn record(identity: &TransactionIdentity, index: u16, marker: u8) -> IncomingRecord {
    IncomingRecord {
        transaction_identity: identity.clone(),
        operation_index: index,
        signed_operation: signed(marker),
    }
}

#[test]
fn complete_batches_are_stable_and_redacted() {
    let identity = TransactionIdentity::new(b"transaction-a".to_vec()).unwrap();
    let batch = TransactionBatch::new(
        identity.clone(),
        2,
        Cursor::new(b"before".to_vec()).unwrap(),
        Cursor::new(b"after".to_vec()).unwrap(),
        vec![record(&identity, 0, 1), record(&identity, 1, 2)],
    )
    .unwrap();
    let replay = TransactionBatch::new(
        identity.clone(),
        2,
        Cursor::new(b"before".to_vec()).unwrap(),
        Cursor::new(b"after".to_vec()).unwrap(),
        vec![record(&identity, 0, 1), record(&identity, 1, 2)],
    )
    .unwrap();
    assert_eq!(batch.transition_digest(), replay.transition_digest());
    assert!(!format!("{batch:?}").contains("transaction-a"));
}

#[test]
fn framing_and_order_change_the_digest() {
    let one = TransactionIdentity::new(b"a".to_vec()).unwrap();
    let two = TransactionIdentity::new(b"ab".to_vec()).unwrap();
    let first = TransactionBatch::new(
        one.clone(),
        1,
        Cursor::new(b"bc".to_vec()).unwrap(),
        Cursor::new(b"d".to_vec()).unwrap(),
        vec![record(&one, 0, 1)],
    )
    .unwrap();
    let second = TransactionBatch::new(
        two.clone(),
        1,
        Cursor::new(b"c".to_vec()).unwrap(),
        Cursor::new(b"d".to_vec()).unwrap(),
        vec![record(&two, 0, 1)],
    )
    .unwrap();
    assert_ne!(first.transition_digest(), second.transition_digest());
}

#[test]
fn canonical_transition_digest_matches_the_portable_vector() {
    let identity = TransactionIdentity::new(b"tx-vector".to_vec()).unwrap();
    let signed = SignedPrismOperation::parse(
        &hex::decode(concat!(
            "0a057664722d301240",
            "0101010101010101010101010101010101010101010101010101010101010101",
            "0101010101010101010101010101010101010101010101010101010101010101",
            "1a2c3a2a0a201313131313131313131313131313131313131313131313131313",
            "131313131313a2060576616c7565"
        ))
        .unwrap(),
    )
    .unwrap();
    let batch = TransactionBatch::new(
        identity.clone(),
        1,
        Cursor::new(b"before".to_vec()).unwrap(),
        Cursor::new(b"after".to_vec()).unwrap(),
        vec![IncomingRecord {
            transaction_identity: identity,
            operation_index: 0,
            signed_operation: signed,
        }],
    )
    .unwrap();
    assert_eq!(
        hex::encode(batch.transition_digest().as_bytes()),
        "f771dfd3438a3fa39e2481d346d53d27df10ec962d74aad5fd77dd01813d6718"
    );
}

#[test]
fn rejects_incomplete_and_cross_transaction_records() {
    let identity = TransactionIdentity::new(b"transaction-a".to_vec()).unwrap();
    let other = TransactionIdentity::new(b"transaction-b".to_vec()).unwrap();
    let result = TransactionBatch::new(
        identity,
        2,
        Cursor::new(b"before".to_vec()).unwrap(),
        Cursor::new(b"after".to_vec()).unwrap(),
        vec![record(&other, 0, 1)],
    );
    assert_eq!(result.unwrap_err().kind(), ErrorKind::IncompleteTransaction);
}

#[test]
fn property_exhaustive_four_record_orders_are_bounded_and_domain_separated() {
    let identity = TransactionIdentity::new(b"transaction-property".to_vec()).unwrap();
    let before = Cursor::new(b"before".to_vec()).unwrap();
    let after = Cursor::new(b"after".to_vec()).unwrap();
    let mut digests = HashSet::new();
    let mut cases = 0;
    for a in 0..4_u8 {
        for b in 0..4_u8 {
            for c in 0..4_u8 {
                for d in 0..4_u8 {
                    let order = [a, b, c, d];
                    if order.iter().copied().collect::<HashSet<_>>().len() != order.len() {
                        continue;
                    }
                    let records = order
                        .into_iter()
                        .enumerate()
                        .map(|(index, marker)| record(&identity, index as u16, marker + 1))
                        .collect();
                    let batch = TransactionBatch::new(
                        identity.clone(),
                        4,
                        before.clone(),
                        after.clone(),
                        records,
                    )
                    .unwrap();
                    assert!(digests.insert(batch.transition_digest()));
                    cases += 1;
                }
            }
        }
    }
    assert_eq!(cases, 24);
}
