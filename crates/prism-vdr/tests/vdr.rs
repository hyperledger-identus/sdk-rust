use std::collections::HashMap;

use identus_core::ErrorKind as CoreErrorKind;
use identus_crypto::{Secp256k1PrivateKey, Sha256Digest, sha256};
use identus_prism_did::{
    CanonicalPrismDid, CreateDidOperation, KeyUsage, PrismDidState, PrismPublicKey,
    PrismPublicKeyMaterial, RawPrismOperation, SignedPrismOperation,
};
use identus_prism_vdr::{
    EntrySnapshot, ErrorKind, LookupError, MAX_ANCESTRY_DEPTH, MAX_VDR_DATA_BYTES, TraversalBudget,
    VdrData, VdrLookup, VdrNode, VdrOperation, resolve_entry,
};

#[derive(Default)]
struct Lookup {
    nodes: HashMap<Sha256Digest, VdrNode>,
    entries: HashMap<Sha256Digest, EntrySnapshot>,
}

impl VdrLookup for Lookup {
    fn node(&self, operation_hash: &Sha256Digest) -> Result<Option<VdrNode>, LookupError> {
        Ok(self.nodes.get(operation_hash).cloned())
    }

    fn entry(&self, root: &Sha256Digest) -> Result<Option<EntrySnapshot>, LookupError> {
        Ok(self.entries.get(root).cloned())
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

fn signed(kind_field: u64, payload: &[u8]) -> SignedPrismOperation {
    let mut operation = Vec::new();
    push_bytes(&mut operation, kind_field, payload);
    let mut envelope = Vec::new();
    push_bytes(&mut envelope, 1, b"vdr-0");
    push_bytes(&mut envelope, 2, &[1; 64]);
    push_bytes(&mut envelope, 3, &operation);
    SignedPrismOperation::parse(&envelope).unwrap()
}

fn signed_by(
    kind_field: u64,
    payload: &[u8],
    key_id: &str,
    key: &Secp256k1PrivateKey,
) -> SignedPrismOperation {
    let mut operation = Vec::new();
    push_bytes(&mut operation, kind_field, payload);
    let raw = RawPrismOperation::parse(&operation).unwrap();
    SignedPrismOperation::from_signature(key_id, &key.sign(raw.as_bytes()), &raw).unwrap()
}

fn create_payload(owner: &[u8; 32], content: &[u8]) -> Vec<u8> {
    let mut payload = Vec::new();
    push_bytes(&mut payload, 1, owner);
    push_bytes(&mut payload, 50, &[7]);
    push_bytes(&mut payload, 100, content);
    payload
}

fn update_payload(predecessor: Sha256Digest, content: &[u8]) -> Vec<u8> {
    let mut payload = Vec::new();
    push_bytes(&mut payload, 2, predecessor.as_bytes());
    push_bytes(&mut payload, 100, content);
    payload
}

fn deactivate_payload(predecessor: Sha256Digest) -> Vec<u8> {
    let mut payload = Vec::new();
    push_bytes(&mut payload, 2, predecessor.as_bytes());
    payload
}

#[test]
fn parses_create_and_update_without_exposing_payload_debug() {
    let owner = [0xabu8; 32];
    let create = VdrOperation::parse(&signed(7, &create_payload(&owner, b"secret-value"))).unwrap();
    let VdrOperation::Create(create) = create else {
        panic!("expected create");
    };
    assert_eq!(create.owner(), &CanonicalPrismDid::from_digest(&owner));
    assert_eq!(create.nonce(), &[7]);
    assert!(matches!(create.data(), VdrData::Bytes(value) if value == b"secret-value"));
    assert!(!format!("{:?}", create.data()).contains("secret-value"));

    let predecessor = sha256(b"predecessor");
    let update =
        VdrOperation::parse(&signed(8, &update_payload(predecessor, b"replacement"))).unwrap();
    assert_eq!(update.predecessor(), Some(predecessor));
}

#[test]
fn rejects_duplicate_owner_and_oversized_content() {
    let owner = [0x11u8; 32];
    let mut duplicate = create_payload(&owner, b"value");
    push_bytes(&mut duplicate, 1, &owner);
    assert_eq!(
        VdrOperation::parse(&signed(7, &duplicate))
            .unwrap_err()
            .kind(),
        ErrorKind::InvalidWire
    );

    let oversized = vec![0; MAX_VDR_DATA_BYTES + 1];
    assert_eq!(
        VdrOperation::parse(&signed(7, &create_payload(&owner, &oversized)))
            .unwrap_err()
            .kind(),
        ErrorKind::ResourceLimitExceeded
    );
}

#[test]
fn accepts_the_schema_default_empty_nonce() {
    let owner = [0x13u8; 32];
    let mut payload = Vec::new();
    push_bytes(&mut payload, 1, &owner);
    push_bytes(&mut payload, 100, b"value");
    let operation = VdrOperation::parse(&signed(7, &payload)).unwrap();
    let VdrOperation::Create(create) = operation else {
        panic!("expected create");
    };
    assert!(create.nonce().is_empty());
}

#[test]
fn sdk_error_bridge_preserves_storage_and_conflict_categories() {
    let missing = sha256(b"missing");
    struct FailingLookup;
    impl VdrLookup for FailingLookup {
        fn node(&self, _: &Sha256Digest) -> Result<Option<VdrNode>, LookupError> {
            Err(LookupError)
        }

        fn entry(&self, _: &Sha256Digest) -> Result<Option<EntrySnapshot>, LookupError> {
            Err(LookupError)
        }
    }
    let mut budget = TraversalBudget::default();
    let error = resolve_entry(
        missing,
        &HashMap::new(),
        &HashMap::new(),
        &FailingLookup,
        &mut budget,
    )
    .unwrap_err();
    assert_eq!(error.to_identus_error().kind(), CoreErrorKind::Storage);

    let root = sha256(b"root-conflict");
    let owner = CanonicalPrismDid::from_digest(&[0x44; 32]);
    let nodes = HashMap::from([(
        root,
        VdrNode {
            operation_hash: root,
            predecessor: None,
            owner: Some(owner),
        },
    )]);
    let entries = HashMap::from([(
        root,
        EntrySnapshot {
            head: sha256(b"new-head"),
            terminal: false,
        },
    )]);
    let error = resolve_entry(root, &nodes, &entries, &Lookup::default(), &mut budget).unwrap_err();
    assert_eq!(error.to_identus_error().kind(), CoreErrorKind::Conflict);
}

#[test]
fn rejects_duplicate_nested_status_fields_and_deactivate_content() {
    let owner = [0x12u8; 32];
    let mut status = Vec::new();
    push_varint(1 << 3, &mut status);
    push_varint(1, &mut status);
    push_bytes(&mut status, 2, b"first");
    push_bytes(&mut status, 2, b"second");
    let mut create = Vec::new();
    push_bytes(&mut create, 1, &owner);
    push_bytes(&mut create, 50, &[1]);
    push_bytes(&mut create, 102, &status);
    assert_eq!(
        VdrOperation::parse(&signed(7, &create)).unwrap_err().kind(),
        ErrorKind::InvalidWire
    );

    let predecessor = sha256(b"predecessor");
    let mut deactivate = deactivate_payload(predecessor);
    push_bytes(&mut deactivate, 100, b"not-allowed");
    assert_eq!(
        VdrOperation::parse(&signed(9, &deactivate))
            .unwrap_err()
            .kind(),
        ErrorKind::InvalidWire
    );
}

#[test]
fn authorizes_exact_bytes_with_the_active_vdr_key() {
    let master = Secp256k1PrivateKey::from_slice(&[1; 32]).unwrap();
    let vdr = Secp256k1PrivateKey::from_slice(&[2; 32]).unwrap();
    let create_did = CreateDidOperation::new(
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
    let raw_create_did = create_did.to_raw_operation().unwrap();
    let signed_create_did = SignedPrismOperation::from_signature(
        "master-0",
        &master.sign(raw_create_did.as_bytes()),
        &raw_create_did,
    )
    .unwrap();
    let state = PrismDidState::from_signed_create(&signed_create_did).unwrap();

    let payload = create_payload(raw_create_did.operation_hash().as_array(), b"value");
    let signed_vdr = signed_by(7, &payload, "vdr-0", &vdr);
    let operation = VdrOperation::parse(&signed_vdr).unwrap();
    operation
        .authorize(&signed_vdr, state.did(), &state)
        .unwrap();

    let forged = signed_by(7, &payload, "vdr-0", &master);
    let forged_operation = VdrOperation::parse(&forged).unwrap();
    assert_eq!(
        forged_operation
            .authorize(&forged, state.did(), &state)
            .unwrap_err()
            .kind(),
        ErrorKind::Unauthorized
    );
}

#[test]
fn resolves_local_chain_and_checks_current_head() {
    let root = sha256(b"root");
    let child = sha256(b"child");
    let owner = CanonicalPrismDid::from_digest(&[0x22; 32]);
    let local_nodes = HashMap::from([
        (
            root,
            VdrNode {
                operation_hash: root,
                predecessor: None,
                owner: Some(owner.clone()),
            },
        ),
        (
            child,
            VdrNode {
                operation_hash: child,
                predecessor: Some(root),
                owner: None,
            },
        ),
    ]);
    let local_entries = HashMap::from([(
        root,
        EntrySnapshot {
            head: child,
            terminal: false,
        },
    )]);
    let mut budget = TraversalBudget::default();
    let resolved = resolve_entry(
        child,
        &local_nodes,
        &local_entries,
        &Lookup::default(),
        &mut budget,
    )
    .unwrap();
    assert_eq!(resolved.root, root);
    assert_eq!(resolved.owner, owner);
    assert_eq!(resolved.head, child);
    let debug = format!("{resolved:?}");
    assert!(!debug.contains(&format!("{root:?}")));
    assert!(!debug.contains(owner.suffix()));

    let stale = resolve_entry(
        root,
        &local_nodes,
        &local_entries,
        &Lookup::default(),
        &mut budget,
    )
    .unwrap_err();
    assert_eq!(stale.kind(), ErrorKind::StalePredecessor);
}

#[test]
fn rejects_cycles_terminal_entries_and_exhausted_work() {
    let first = sha256(b"first");
    let second = sha256(b"second");
    let cyclic = HashMap::from([
        (
            first,
            VdrNode {
                operation_hash: first,
                predecessor: Some(second),
                owner: None,
            },
        ),
        (
            second,
            VdrNode {
                operation_hash: second,
                predecessor: Some(first),
                owner: None,
            },
        ),
    ]);
    let mut budget = TraversalBudget::default();
    let error = resolve_entry(
        first,
        &cyclic,
        &HashMap::new(),
        &Lookup::default(),
        &mut budget,
    )
    .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Cycle);

    let root = sha256(b"terminal-root");
    let owner = CanonicalPrismDid::from_digest(&[0x33; 32]);
    let nodes = HashMap::from([(
        root,
        VdrNode {
            operation_hash: root,
            predecessor: None,
            owner: Some(owner),
        },
    )]);
    let entries = HashMap::from([(
        root,
        EntrySnapshot {
            head: root,
            terminal: true,
        },
    )]);
    let error = resolve_entry(root, &nodes, &entries, &Lookup::default(), &mut budget).unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Deactivated);

    let mut empty_budget = TraversalBudget::new(0);
    let error = resolve_entry(
        root,
        &nodes,
        &entries,
        &Lookup::default(),
        &mut empty_budget,
    )
    .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::ResourceLimitExceeded);
}

#[test]
fn property_generated_ancestry_lengths_preserve_root_and_head() {
    for node_count in 1..=MAX_ANCESTRY_DEPTH {
        let owner = CanonicalPrismDid::from_digest(&[0x55; 32]);
        let root = sha256(0_u64.to_be_bytes());
        let mut previous = root;
        let mut nodes = HashMap::from([(
            root,
            VdrNode {
                operation_hash: root,
                predecessor: None,
                owner: Some(owner.clone()),
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
        let mut budget = TraversalBudget::default();
        let resolved =
            resolve_entry(previous, &nodes, &entries, &Lookup::default(), &mut budget).unwrap();
        assert_eq!(resolved.root, root);
        assert_eq!(resolved.head, previous);
        assert_eq!(resolved.owner, owner);
    }
}
