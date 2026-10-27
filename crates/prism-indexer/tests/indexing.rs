use std::{collections::HashMap, sync::Mutex};

use identus_crypto::{Secp256k1PrivateKey, Sha256Digest};
use identus_prism_did::{
    CanonicalPrismDid, CreateDidOperation, KeyUsage, PrismDidState, PrismPublicKey,
    PrismPublicKeyMaterial, RawPrismOperation, SignedPrismOperation,
};
use identus_prism_indexer::{
    AtomicCommit, CommitOutcome, CommitRequest, Cursor, DidStateLookup,
    ErrorKind as IndexerErrorKind, IncomingRecord, IndexOutcome, Mutation, StoreError,
    TransactionBatch, TransactionIdentity, TransitionDigest, TransitionReceiptLookup,
    index_transaction, plan_transaction,
};
use identus_prism_vdr::{EntrySnapshot, LookupError, VdrLookup, VdrNode, VdrOperation};

#[derive(Clone, Default)]
struct State {
    cursor: Option<Cursor>,
    receipts: HashMap<TransactionIdentity, TransitionDigest>,
    dids: HashMap<CanonicalPrismDid, PrismDidState>,
    nodes: HashMap<Sha256Digest, VdrNode>,
    entries: HashMap<Sha256Digest, EntrySnapshot>,
    commits: usize,
}

#[derive(Default)]
struct InMemoryStore(Mutex<State>);

struct FailingLookup;

#[derive(Clone)]
struct NeoPrismMetadata {
    tx_id: [u8; 32],
    prism_block_index: u32,
    operation_index: u32,
}

#[derive(Clone)]
struct NeoPrismRecord {
    metadata: NeoPrismMetadata,
    signed_operation: SignedPrismOperation,
}

impl DidStateLookup for FailingLookup {
    fn did_state(&self, _: &CanonicalPrismDid) -> Result<Option<PrismDidState>, LookupError> {
        Err(LookupError)
    }
}

impl VdrLookup for FailingLookup {
    fn node(&self, _: &Sha256Digest) -> Result<Option<VdrNode>, LookupError> {
        Err(LookupError)
    }

    fn entry(&self, _: &Sha256Digest) -> Result<Option<EntrySnapshot>, LookupError> {
        Err(LookupError)
    }
}

impl DidStateLookup for InMemoryStore {
    fn did_state(&self, did: &CanonicalPrismDid) -> Result<Option<PrismDidState>, LookupError> {
        Ok(self.0.lock().unwrap().dids.get(did).cloned())
    }
}

impl VdrLookup for InMemoryStore {
    fn node(&self, hash: &Sha256Digest) -> Result<Option<VdrNode>, LookupError> {
        Ok(self.0.lock().unwrap().nodes.get(hash).cloned())
    }

    fn entry(&self, root: &Sha256Digest) -> Result<Option<EntrySnapshot>, LookupError> {
        Ok(self.0.lock().unwrap().entries.get(root).cloned())
    }
}

impl TransitionReceiptLookup for InMemoryStore {
    fn transition_receipt(
        &self,
        identity: &TransactionIdentity,
    ) -> Result<Option<TransitionDigest>, StoreError> {
        Ok(self.0.lock().unwrap().receipts.get(identity).copied())
    }
}

impl AtomicCommit for InMemoryStore {
    fn commit(&self, request: CommitRequest<'_>) -> Result<CommitOutcome, StoreError> {
        let mut guard = self.0.lock().unwrap();
        guard.commits += 1;
        if let Some(digest) = guard.receipts.get(request.identity) {
            return Ok(if *digest == request.digest {
                CommitOutcome::ExactReplay
            } else {
                CommitOutcome::Conflict
            });
        }
        if guard.cursor.as_ref() != Some(request.expected_cursor) && guard.cursor.is_some() {
            return Ok(CommitOutcome::Conflict);
        }
        let mut candidate = guard.clone();
        for mutation in request.mutations {
            match mutation {
                Mutation::Did(state) => {
                    candidate.dids.insert(state.did().clone(), state.clone());
                }
                Mutation::Vdr {
                    operation,
                    root,
                    entry,
                    owner,
                } => {
                    let hash = operation.operation_hash();
                    candidate.nodes.insert(
                        hash,
                        VdrNode {
                            operation_hash: hash,
                            predecessor: operation.predecessor(),
                            owner: matches!(operation, VdrOperation::Create(_))
                                .then(|| owner.clone()),
                        },
                    );
                    candidate.entries.insert(*root, entry.clone());
                }
                _ => panic!("unexpected future mutation in conformance adapter"),
            }
        }
        candidate.cursor = Some(request.next_cursor.clone());
        candidate
            .receipts
            .insert(request.identity.clone(), request.digest);
        candidate.commits = guard.commits;
        *guard = candidate;
        Ok(CommitOutcome::Applied)
    }
}

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

fn signed_vdr(field: u64, payload: &[u8], key: &Secp256k1PrivateKey) -> SignedPrismOperation {
    let mut operation = Vec::new();
    push_bytes(&mut operation, field, payload);
    let raw = RawPrismOperation::parse(&operation).unwrap();
    SignedPrismOperation::from_signature("vdr-0", &key.sign(raw.as_bytes()), &raw).unwrap()
}

fn fixture() -> (
    SignedPrismOperation,
    SignedPrismOperation,
    Secp256k1PrivateKey,
) {
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
    let signed_create =
        SignedPrismOperation::from_signature("master-0", &master.sign(raw.as_bytes()), &raw)
            .unwrap();
    let mut payload = Vec::new();
    push_bytes(&mut payload, 1, raw.operation_hash().as_bytes());
    push_bytes(&mut payload, 50, &[1]);
    push_bytes(&mut payload, 100, b"initial");
    let signed_storage = signed_vdr(7, &payload, &vdr);
    (signed_create, signed_storage, vdr)
}

fn batch(
    id: &[u8],
    expected: &[u8],
    next: &[u8],
    operations: Vec<SignedPrismOperation>,
) -> TransactionBatch {
    let identity = TransactionIdentity::new(id.to_vec()).unwrap();
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
        Cursor::new(expected.to_vec()).unwrap(),
        Cursor::new(next.to_vec()).unwrap(),
        records,
    )
    .unwrap()
}

fn neoprism_shaped_batch(
    expected_cursor: &[u8],
    next_cursor: &[u8],
    mut records: Vec<NeoPrismRecord>,
) -> Result<TransactionBatch, &'static str> {
    let first = records.first().ok_or("empty NeoPRISM transaction")?;
    let tx_id = first.metadata.tx_id;
    if records.iter().any(|record| record.metadata.tx_id != tx_id) {
        return Err("mixed NeoPRISM transaction identities");
    }
    records.sort_by_key(|record| {
        (
            record.metadata.prism_block_index,
            record.metadata.operation_index,
        )
    });
    let identity = TransactionIdentity::new(tx_id.to_vec()).map_err(|_| "invalid identity")?;
    let incoming = records
        .into_iter()
        .enumerate()
        .map(|(index, record)| {
            let operation_index = u16::try_from(index).map_err(|_| "too many operations")?;
            Ok(IncomingRecord {
                transaction_identity: identity.clone(),
                operation_index,
                signed_operation: record.signed_operation,
            })
        })
        .collect::<Result<Vec<_>, &'static str>>()?;
    let count = u16::try_from(incoming.len()).map_err(|_| "too many operations")?;
    TransactionBatch::new(
        identity,
        count,
        Cursor::new(expected_cursor.to_vec()).map_err(|_| "invalid expected cursor")?,
        Cursor::new(next_cursor.to_vec()).map_err(|_| "invalid next cursor")?,
        incoming,
    )
    .map_err(|_| "invalid complete transaction")
}

#[test]
fn plans_same_transaction_did_and_vdr_then_replays_without_commit() {
    let (create_did, create_storage, _) = fixture();
    let batch = batch(
        b"tx-1",
        b"genesis",
        b"cursor-1",
        vec![create_did, create_storage],
    );
    let store = InMemoryStore::default();
    assert_eq!(
        index_transaction(&batch, &store).unwrap(),
        IndexOutcome::Applied
    );
    {
        let state = store.0.lock().unwrap();
        assert_eq!(state.dids.len(), 1);
        assert_eq!(state.nodes.len(), 1);
        assert_eq!(state.entries.len(), 1);
        assert_eq!(state.commits, 1);
    }
    assert_eq!(
        index_transaction(&batch, &store).unwrap(),
        IndexOutcome::ExactReplay
    );
    assert_eq!(store.0.lock().unwrap().commits, 1);
}

#[test]
fn advances_an_existing_vdr_head_and_rejects_identity_reuse() {
    let (create_did, create_storage, vdr) = fixture();
    let first = batch(
        b"tx-1",
        b"genesis",
        b"cursor-1",
        vec![create_did, create_storage.clone()],
    );
    let store = InMemoryStore::default();
    index_transaction(&first, &store).unwrap();

    let mut update_payload = Vec::new();
    push_bytes(
        &mut update_payload,
        2,
        create_storage.operation_hash().as_bytes(),
    );
    push_bytes(&mut update_payload, 100, b"updated");
    let update = signed_vdr(8, &update_payload, &vdr);
    let second = batch(b"tx-2", b"cursor-1", b"cursor-2", vec![update]);
    assert_eq!(
        index_transaction(&second, &store).unwrap(),
        IndexOutcome::Applied
    );

    let conflicting = batch(b"tx-2", b"cursor-2", b"cursor-3", vec![create_storage]);
    assert!(index_transaction(&conflicting, &store).is_err());
    assert_eq!(store.0.lock().unwrap().commits, 2);
}

#[test]
fn preserves_vdr_traversal_failure_categories() {
    let key = Secp256k1PrivateKey::from_slice(&[3; 32]).unwrap();
    let predecessor = Sha256Digest::from_bytes(&[9; 32]).unwrap();
    let mut update_payload = Vec::new();
    push_bytes(&mut update_payload, 2, predecessor.as_bytes());
    push_bytes(&mut update_payload, 100, b"updated");
    let update = signed_vdr(8, &update_payload, &key);
    let batch = batch(b"tx-errors", b"genesis", b"next", vec![update]);

    let missing = plan_transaction(&batch, &InMemoryStore::default()).unwrap_err();
    assert_eq!(missing.kind(), IndexerErrorKind::MissingState);

    let failed = plan_transaction(&batch, &FailingLookup).unwrap_err();
    assert_eq!(failed.kind(), IndexerErrorKind::LookupFailed);

    let cyclic = InMemoryStore::default();
    cyclic.0.lock().unwrap().nodes.insert(
        predecessor,
        VdrNode {
            operation_hash: predecessor,
            predecessor: Some(predecessor),
            owner: None,
        },
    );
    let bounded = plan_transaction(&batch, &cyclic).unwrap_err();
    assert_eq!(bounded.kind(), IndexerErrorKind::Conflict);
}

#[test]
fn neoprism_shaped_metadata_maps_to_one_complete_ordered_batch() {
    let (create_did, create_storage, _) = fixture();
    let tx_id = [7; 32];
    let records = vec![
        NeoPrismRecord {
            metadata: NeoPrismMetadata {
                tx_id,
                prism_block_index: 1,
                operation_index: 0,
            },
            signed_operation: create_storage,
        },
        NeoPrismRecord {
            metadata: NeoPrismMetadata {
                tx_id,
                prism_block_index: 0,
                operation_index: 0,
            },
            signed_operation: create_did,
        },
    ];
    let batch = neoprism_shaped_batch(b"slot-41", b"slot-42", records).unwrap();
    assert_eq!(batch.identity().as_bytes(), tx_id);
    assert_eq!(batch.records().len(), 2);
    assert_eq!(batch.records()[0].operation_index, 0);
    assert_eq!(batch.records()[1].operation_index, 1);

    let store = InMemoryStore::default();
    assert_eq!(
        index_transaction(&batch, &store).unwrap(),
        IndexOutcome::Applied
    );
    let state = store.0.lock().unwrap();
    assert_eq!(state.dids.len(), 1);
    assert_eq!(state.entries.len(), 1);
}
