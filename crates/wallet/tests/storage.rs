use std::collections::HashMap;
use std::future::Future;
use std::hint::black_box;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};
use std::time::Instant;

use identus_core::ErrorKind;
use identus_wallet::{
    CredentialStore, DidStore, MAX_STORAGE_CURSOR_BYTES, MAX_STORAGE_PAGE_ENTRIES,
    MAX_STORAGE_REVISION_BYTES, ProtocolStateStore, SecretStore, StatusCacheStore, StorageCursor,
    StorageDeleteCondition, StorageDeleteOutcome, StorageError, StorageFuture, StoragePage,
    StoragePageRequest, StoragePageSize, StorageRevision, StorageWrite, StorageWriteCondition,
    StorageWriteOutcome, StorageWriteReceipt, Stored,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Tenant(String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct CredentialId(String);

#[derive(Clone, Debug, PartialEq, Eq)]
struct CredentialRecord {
    issuer: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CredentialIndexEntry {
    id: CredentialId,
}

#[derive(Default)]
struct MemoryCredentialAdapter {
    records: Mutex<HashMap<(Tenant, CredentialId), (u64, CredentialRecord)>>,
    next_revision: Mutex<u64>,
}

impl MemoryCredentialAdapter {
    fn allocate_revision(&self) -> u64 {
        let mut next = self.next_revision.lock().unwrap();
        *next += 1;
        *next
    }

    fn revision(version: u64) -> StorageRevision {
        StorageRevision::new(version.to_be_bytes().to_vec()).unwrap()
    }

    fn cursor(offset: usize) -> StorageCursor {
        StorageCursor::new((offset as u64).to_be_bytes().to_vec()).unwrap()
    }

    fn offset(cursor: Option<&StorageCursor>) -> Result<usize, StorageError> {
        let Some(cursor) = cursor else {
            return Ok(0);
        };
        let bytes: [u8; 8] = cursor
            .as_bytes()
            .try_into()
            .map_err(|_| StorageError::InvalidCursor)?;
        usize::try_from(u64::from_be_bytes(bytes)).map_err(|_| StorageError::InvalidCursor)
    }
}

impl CredentialStore for MemoryCredentialAdapter {
    type Scope = Tenant;
    type Key = CredentialId;
    type Value = CredentialRecord;
    type IndexEntry = CredentialIndexEntry;

    fn load<'a>(
        &'a self,
        scope: &'a Self::Scope,
        key: &'a Self::Key,
    ) -> StorageFuture<'a, Option<Stored<Self::Value>>> {
        Box::pin(async move {
            Ok(self
                .records
                .lock()
                .unwrap()
                .get(&(scope.clone(), key.clone()))
                .map(|(revision, value)| Stored::new(value.clone(), Self::revision(*revision))))
        })
    }

    fn write<'a>(
        &'a self,
        scope: &'a Self::Scope,
        key: &'a Self::Key,
        write: StorageWrite<Self::Value>,
    ) -> StorageFuture<'a, StorageWriteReceipt> {
        Box::pin(async move {
            let (value, condition) = write.into_parts();
            let storage_key = (scope.clone(), key.clone());
            let mut records = self.records.lock().unwrap();
            let current = records.get(&storage_key).map(|(revision, _)| *revision);
            match condition {
                StorageWriteCondition::Any => {}
                StorageWriteCondition::InsertOnly if current.is_some() => {
                    return Err(StorageError::Conflict);
                }
                StorageWriteCondition::IfRevision(expected)
                    if current.map(Self::revision).as_ref() != Some(&expected) =>
                {
                    return Err(StorageError::Conflict);
                }
                StorageWriteCondition::InsertOnly | StorageWriteCondition::IfRevision(_) => {}
                _ => return Err(StorageError::Internal),
            }
            let outcome = if current.is_some() {
                StorageWriteOutcome::Replaced
            } else {
                StorageWriteOutcome::Inserted
            };
            let revision = self.allocate_revision();
            records.insert(storage_key, (revision, value));
            Ok(StorageWriteReceipt::new(outcome, Self::revision(revision)))
        })
    }

    fn delete<'a>(
        &'a self,
        scope: &'a Self::Scope,
        key: &'a Self::Key,
        condition: StorageDeleteCondition,
    ) -> StorageFuture<'a, StorageDeleteOutcome> {
        Box::pin(async move {
            let storage_key = (scope.clone(), key.clone());
            let mut records = self.records.lock().unwrap();
            let current = records.get(&storage_key).map(|(revision, _)| *revision);
            match condition {
                StorageDeleteCondition::Any => {}
                StorageDeleteCondition::IfRevision(expected)
                    if current.map(Self::revision).as_ref() != Some(&expected) =>
                {
                    return Err(StorageError::Conflict);
                }
                StorageDeleteCondition::IfRevision(_) => {}
                _ => return Err(StorageError::Internal),
            }
            Ok(if records.remove(&storage_key).is_some() {
                StorageDeleteOutcome::Deleted
            } else {
                StorageDeleteOutcome::NotFound
            })
        })
    }

    fn list<'a>(
        &'a self,
        scope: &'a Self::Scope,
        request: StoragePageRequest,
    ) -> StorageFuture<'a, StoragePage<Self::IndexEntry>> {
        Box::pin(async move {
            let offset = Self::offset(request.cursor())?;
            let mut ids = self
                .records
                .lock()
                .unwrap()
                .keys()
                .filter(|(candidate, _)| candidate == scope)
                .map(|(_, id)| id.clone())
                .collect::<Vec<_>>();
            ids.sort();
            let end = offset.saturating_add(request.size().get()).min(ids.len());
            let entries = ids
                .get(offset..end)
                .unwrap_or_default()
                .iter()
                .cloned()
                .map(|id| CredentialIndexEntry { id })
                .collect::<Vec<_>>();
            let next_cursor = (end < ids.len()).then(|| Self::cursor(end));
            StoragePage::new(entries, next_cursor)
        })
    }
}

fn block_on<F: Future>(future: F) -> F::Output {
    struct ThreadWake(std::thread::Thread);
    impl Wake for ThreadWake {
        fn wake(self: Arc<Self>) {
            self.0.unpark();
        }
    }

    let waker = Waker::from(Arc::new(ThreadWake(std::thread::current())));
    let mut context = Context::from_waker(&waker);
    let mut future = std::pin::pin!(future);
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => return output,
            Poll::Pending => std::thread::park(),
        }
    }
}

#[test]
fn opaque_values_and_pages_enforce_exact_bounds() {
    let revision = StorageRevision::new(vec![7; MAX_STORAGE_REVISION_BYTES]).unwrap();
    let cursor = StorageCursor::new(vec![8; MAX_STORAGE_CURSOR_BYTES]).unwrap();
    assert_eq!(revision.as_bytes(), &[7; MAX_STORAGE_REVISION_BYTES]);
    assert_eq!(cursor.as_bytes(), &[8; MAX_STORAGE_CURSOR_BYTES]);
    assert!(StorageRevision::new(Vec::new()).is_err());
    assert!(StorageRevision::new(vec![0; MAX_STORAGE_REVISION_BYTES + 1]).is_err());
    assert!(StorageCursor::new(Vec::new()).is_err());
    assert!(StorageCursor::new(vec![0; MAX_STORAGE_CURSOR_BYTES + 1]).is_err());

    assert_eq!(StoragePageSize::new(1).unwrap().get(), 1);
    assert_eq!(
        StoragePageSize::new(MAX_STORAGE_PAGE_ENTRIES)
            .unwrap()
            .get(),
        MAX_STORAGE_PAGE_ENTRIES
    );
    assert_eq!(StoragePageSize::new(0), Err(StorageError::InvalidPageSize));
    assert_eq!(
        StoragePageSize::new(MAX_STORAGE_PAGE_ENTRIES + 1),
        Err(StorageError::InvalidPageSize)
    );
    assert!(StoragePage::new(vec![0_u8; MAX_STORAGE_PAGE_ENTRIES], None).is_ok());
    assert_eq!(
        StoragePage::new(vec![0_u8; MAX_STORAGE_PAGE_ENTRIES + 1], None),
        Err(StorageError::InvalidPage)
    );
    assert_eq!(
        StoragePage::<u8>::new(Vec::new(), Some(StorageCursor::new(vec![1]).unwrap())),
        Err(StorageError::InvalidPage)
    );
}

#[test]
fn generic_debug_surfaces_redact_values_and_tokens() {
    struct Canary(&'static str);
    let canary = "TOP-SECRET-CANARY";
    let revision_bytes = b"REVISION-CANARY".to_vec();
    let cursor_bytes = b"CURSOR-CANARY".to_vec();

    let stored = Stored::new(
        Canary(canary),
        StorageRevision::new(revision_bytes.clone()).unwrap(),
    );
    let write = StorageWrite::new(
        Canary(canary),
        StorageWriteCondition::IfRevision(StorageRevision::new(revision_bytes).unwrap()),
    );
    let page = StoragePage::new(
        vec![Canary(canary)],
        Some(StorageCursor::new(cursor_bytes).unwrap()),
    )
    .unwrap();
    let receipt = StorageWriteReceipt::new(
        StorageWriteOutcome::Inserted,
        StorageRevision::new(b"RECEIPT-CANARY".to_vec()).unwrap(),
    );
    let rendered = format!("{stored:?} {write:?} {page:?} {receipt:?}");
    assert!(!rendered.contains(canary));
    assert!(!rendered.contains("REVISION-CANARY"));
    assert!(!rendered.contains("CURSOR-CANARY"));
    assert!(!rendered.contains("RECEIPT-CANARY"));
    assert_eq!(stored.value().0, canary);
}

#[test]
fn missing_load_and_unconditional_delete_are_ordinary_outcomes() {
    let store = MemoryCredentialAdapter::default();
    let scope = Tenant("wallet-a".into());
    let key = CredentialId("credential-a".into());
    let object: &dyn CredentialStore<
        Scope = Tenant,
        Key = CredentialId,
        Value = CredentialRecord,
        IndexEntry = CredentialIndexEntry,
    > = &store;

    assert!(block_on(object.load(&scope, &key)).unwrap().is_none());
    assert_eq!(
        block_on(object.delete(&scope, &key, StorageDeleteCondition::Any)).unwrap(),
        StorageDeleteOutcome::NotFound
    );
}

#[test]
fn conditional_writes_rotate_revisions_and_stale_writes_fail_closed() {
    let store = MemoryCredentialAdapter::default();
    let scope = Tenant("wallet-a".into());
    let key = CredentialId("credential-a".into());
    let first = block_on(store.write(
        &scope,
        &key,
        StorageWrite::new(
            CredentialRecord {
                issuer: "issuer-a".into(),
            },
            StorageWriteCondition::InsertOnly,
        ),
    ))
    .unwrap();
    assert_eq!(first.outcome(), StorageWriteOutcome::Inserted);
    assert_eq!(
        block_on(store.write(
            &scope,
            &key,
            StorageWrite::new(
                CredentialRecord {
                    issuer: "issuer-b".into(),
                },
                StorageWriteCondition::InsertOnly,
            ),
        )),
        Err(StorageError::Conflict)
    );

    let stale = first.revision().clone();
    let second = block_on(store.write(
        &scope,
        &key,
        StorageWrite::new(
            CredentialRecord {
                issuer: "issuer-b".into(),
            },
            StorageWriteCondition::IfRevision(stale.clone()),
        ),
    ))
    .unwrap();
    assert_eq!(second.outcome(), StorageWriteOutcome::Replaced);
    assert_ne!(second.revision(), &stale);
    assert_eq!(
        block_on(store.write(
            &scope,
            &key,
            StorageWrite::new(
                CredentialRecord {
                    issuer: "lost-update".into(),
                },
                StorageWriteCondition::IfRevision(stale),
            ),
        )),
        Err(StorageError::Conflict)
    );
    assert_eq!(
        block_on(store.load(&scope, &key))
            .unwrap()
            .unwrap()
            .value()
            .issuer,
        "issuer-b"
    );
}

#[test]
fn revision_matched_delete_is_atomic_and_stale_delete_preserves_record() {
    let store = MemoryCredentialAdapter::default();
    let scope = Tenant("wallet-a".into());
    let key = CredentialId("credential-a".into());
    let receipt = block_on(store.write(
        &scope,
        &key,
        StorageWrite::new(
            CredentialRecord {
                issuer: "issuer-a".into(),
            },
            StorageWriteCondition::Any,
        ),
    ))
    .unwrap();
    let stale = StorageRevision::new(vec![99]).unwrap();
    assert_eq!(
        block_on(store.delete(&scope, &key, StorageDeleteCondition::IfRevision(stale),)),
        Err(StorageError::Conflict)
    );
    assert!(block_on(store.load(&scope, &key)).unwrap().is_some());
    assert_eq!(
        block_on(store.delete(
            &scope,
            &key,
            StorageDeleteCondition::IfRevision(receipt.revision().clone()),
        ))
        .unwrap(),
        StorageDeleteOutcome::Deleted
    );
}

#[test]
fn credential_recovery_index_is_scope_bound_bounded_and_resumable() {
    let store = MemoryCredentialAdapter::default();
    let selected = Tenant("selected".into());
    let other = Tenant("other".into());
    for (scope, id) in [
        (&selected, "c"),
        (&selected, "a"),
        (&selected, "b"),
        (&other, "not-visible"),
    ] {
        block_on(store.write(
            scope,
            &CredentialId(id.into()),
            StorageWrite::new(
                CredentialRecord { issuer: id.into() },
                StorageWriteCondition::Any,
            ),
        ))
        .unwrap();
    }

    let first = block_on(store.list(
        &selected,
        StoragePageRequest::new(StoragePageSize::new(2).unwrap(), None),
    ))
    .unwrap();
    assert_eq!(
        first
            .entries()
            .iter()
            .map(|entry| entry.id.0.as_str())
            .collect::<Vec<_>>(),
        ["a", "b"]
    );
    let second = block_on(store.list(
        &selected,
        StoragePageRequest::new(
            StoragePageSize::new(2).unwrap(),
            first.next_cursor().cloned(),
        ),
    ))
    .unwrap();
    assert_eq!(second.len(), 1);
    assert_eq!(second.entries()[0].id.0, "c");
    assert!(second.next_cursor().is_none());
}

#[test]
fn every_storage_error_maps_to_static_redaction_safe_sdk_metadata() {
    let errors = [
        StorageError::InvalidRevision,
        StorageError::InvalidCursor,
        StorageError::InvalidPageSize,
        StorageError::InvalidPage,
        StorageError::Conflict,
        StorageError::CapacityExceeded,
        StorageError::Integrity,
        StorageError::AccessDenied,
        StorageError::Unavailable,
        StorageError::Internal,
    ];
    for error in errors {
        let sdk = error.to_identus_error();
        assert!(sdk.code().as_str().starts_with("wallet.storage_"));
        assert_eq!(sdk.capability().unwrap().as_str(), "wallet.storage");
        assert!(!sdk.to_string().contains("caller-controlled"));
    }
    assert_eq!(
        StorageError::Conflict.to_identus_error().kind(),
        ErrorKind::Conflict
    );
    assert_eq!(
        StorageError::InvalidPage.to_identus_error().kind(),
        ErrorKind::InvalidInput
    );
}

struct SecretBytes;
struct DidScope;
struct DidKey;
struct DidRecord;
struct DidIndex;
struct ProtocolScope;
struct ProtocolKey;
struct ProtocolRecord;
struct ProtocolIndex;
struct StatusScope;
struct StatusKey;
struct StatusRecord;

#[derive(Default)]
struct ReadyAdapter;

macro_rules! implement_exact_ready_store {
    ($trait_name:ident, $scope:ty, $key:ty, $value:ty) => {
        impl $trait_name for ReadyAdapter {
            type Scope = $scope;
            type Key = $key;
            type Value = $value;

            fn load<'a>(
                &'a self,
                _scope: &'a Self::Scope,
                _key: &'a Self::Key,
            ) -> StorageFuture<'a, Option<Stored<Self::Value>>> {
                Box::pin(async { Ok(None) })
            }

            fn write<'a>(
                &'a self,
                _scope: &'a Self::Scope,
                _key: &'a Self::Key,
                _write: StorageWrite<Self::Value>,
            ) -> StorageFuture<'a, StorageWriteReceipt> {
                Box::pin(async { Err(StorageError::Unavailable) })
            }

            fn delete<'a>(
                &'a self,
                _scope: &'a Self::Scope,
                _key: &'a Self::Key,
                _condition: StorageDeleteCondition,
            ) -> StorageFuture<'a, StorageDeleteOutcome> {
                Box::pin(async { Err(StorageError::Unavailable) })
            }
        }
    };
}

macro_rules! implement_indexed_ready_store {
    ($trait_name:ident, $scope:ty, $key:ty, $value:ty, $index:ty) => {
        impl $trait_name for ReadyAdapter {
            type Scope = $scope;
            type Key = $key;
            type Value = $value;
            type IndexEntry = $index;

            fn load<'a>(
                &'a self,
                _scope: &'a Self::Scope,
                _key: &'a Self::Key,
            ) -> StorageFuture<'a, Option<Stored<Self::Value>>> {
                Box::pin(async { Ok(None) })
            }

            fn write<'a>(
                &'a self,
                _scope: &'a Self::Scope,
                _key: &'a Self::Key,
                _write: StorageWrite<Self::Value>,
            ) -> StorageFuture<'a, StorageWriteReceipt> {
                Box::pin(async { Err(StorageError::Unavailable) })
            }

            fn delete<'a>(
                &'a self,
                _scope: &'a Self::Scope,
                _key: &'a Self::Key,
                _condition: StorageDeleteCondition,
            ) -> StorageFuture<'a, StorageDeleteOutcome> {
                Box::pin(async { Err(StorageError::Unavailable) })
            }

            fn list<'a>(
                &'a self,
                _scope: &'a Self::Scope,
                _request: StoragePageRequest,
            ) -> StorageFuture<'a, StoragePage<Self::IndexEntry>> {
                Box::pin(async { StoragePage::new(Vec::new(), None) })
            }
        }
    };
}

implement_exact_ready_store!(SecretStore, String, String, SecretBytes);
implement_indexed_ready_store!(DidStore, DidScope, DidKey, DidRecord, DidIndex);
implement_indexed_ready_store!(
    ProtocolStateStore,
    ProtocolScope,
    ProtocolKey,
    ProtocolRecord,
    ProtocolIndex
);
implement_exact_ready_store!(StatusCacheStore, StatusScope, StatusKey, StatusRecord);

#[test]
fn all_five_ports_are_independent_object_safe_and_runtime_neutral() {
    let ready = ReadyAdapter;
    let secret: &dyn SecretStore<Scope = String, Key = String, Value = SecretBytes> = &ready;
    let credential: &dyn CredentialStore<
        Scope = Tenant,
        Key = CredentialId,
        Value = CredentialRecord,
        IndexEntry = CredentialIndexEntry,
    > = &MemoryCredentialAdapter::default();
    let did: &dyn DidStore<Scope = DidScope, Key = DidKey, Value = DidRecord, IndexEntry = DidIndex> =
        &ready;
    let protocol: &dyn ProtocolStateStore<
        Scope = ProtocolScope,
        Key = ProtocolKey,
        Value = ProtocolRecord,
        IndexEntry = ProtocolIndex,
    > = &ready;
    let status: &dyn StatusCacheStore<Scope = StatusScope, Key = StatusKey, Value = StatusRecord> =
        &ready;

    assert!(
        block_on(secret.load(&"vault".into(), &"secret".into()))
            .unwrap()
            .is_none()
    );
    assert!(
        block_on(credential.load(&Tenant("scope".into()), &CredentialId("key".into())))
            .unwrap()
            .is_none()
    );
    assert!(block_on(did.load(&DidScope, &DidKey)).unwrap().is_none());
    assert!(
        block_on(protocol.load(&ProtocolScope, &ProtocolKey))
            .unwrap()
            .is_none()
    );
    assert!(
        block_on(status.load(&StatusScope, &StatusKey))
            .unwrap()
            .is_none()
    );
}

#[test]
fn shared_trait_objects_support_concurrent_dispatch() {
    let adapter: Arc<dyn SecretStore<Scope = String, Key = String, Value = SecretBytes>> =
        Arc::new(ReadyAdapter);
    let threads = (0..8)
        .map(|_| {
            let adapter = Arc::clone(&adapter);
            std::thread::spawn(move || {
                block_on(adapter.load(&"vault".into(), &"secret".into()))
                    .unwrap()
                    .is_none()
            })
        })
        .collect::<Vec<_>>();
    assert!(threads.into_iter().all(|thread| thread.join().unwrap()));
}

#[test]
#[ignore = "release-only diagnostic; reports throughput without a timing threshold"]
fn release_trait_object_dispatch_diagnostic() {
    let adapter = ReadyAdapter;
    let secret: &dyn SecretStore<Scope = String, Key = String, Value = SecretBytes> = &adapter;
    let credential_adapter = MemoryCredentialAdapter::default();
    let credential: &dyn CredentialStore<
        Scope = Tenant,
        Key = CredentialId,
        Value = CredentialRecord,
        IndexEntry = CredentialIndexEntry,
    > = &credential_adapter;
    let did: &dyn DidStore<Scope = DidScope, Key = DidKey, Value = DidRecord, IndexEntry = DidIndex> =
        &adapter;
    let protocol: &dyn ProtocolStateStore<
        Scope = ProtocolScope,
        Key = ProtocolKey,
        Value = ProtocolRecord,
        IndexEntry = ProtocolIndex,
    > = &adapter;
    let status: &dyn StatusCacheStore<Scope = StatusScope, Key = StatusKey, Value = StatusRecord> =
        &adapter;
    let tenant = Tenant("diagnostic".into());
    let credential_id = CredentialId("absent".into());
    let vault = "vault".to_owned();
    let secret_name = "secret".to_owned();
    let rounds = 200_000_u64;
    let started = Instant::now();
    for _ in 0..rounds {
        assert!(
            block_on(secret.load(black_box(&vault), black_box(&secret_name)))
                .unwrap()
                .is_none()
        );
        assert!(
            block_on(credential.load(black_box(&tenant), black_box(&credential_id)))
                .unwrap()
                .is_none()
        );
        assert!(
            block_on(did.load(black_box(&DidScope), black_box(&DidKey)))
                .unwrap()
                .is_none()
        );
        assert!(
            block_on(protocol.load(black_box(&ProtocolScope), black_box(&ProtocolKey)))
                .unwrap()
                .is_none()
        );
        assert!(
            block_on(status.load(black_box(&StatusScope), black_box(&StatusKey)))
                .unwrap()
                .is_none()
        );
    }
    let elapsed = started.elapsed();
    let calls = rounds * 5;
    let calls_per_second = calls as f64 / elapsed.as_secs_f64();
    eprintln!(
        "wallet storage dynamic dispatch: {calls} calls in {elapsed:?} ({calls_per_second:.0} calls/s)"
    );
}
