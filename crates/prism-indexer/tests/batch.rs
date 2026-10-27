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
