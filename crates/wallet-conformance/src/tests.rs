use std::collections::BTreeMap;
use std::future::Future;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};
use std::time::Instant;

use super::*;
use identus_wallet::{StoragePage, StorageRevision, StorageWriteReceipt, Stored};

#[derive(Default)]
struct MemoryState {
    records: BTreeMap<(String, String), (String, u64)>,
    indexes: BTreeMap<String, Vec<String>>,
    next_revision: u64,
}

#[derive(Default)]
struct MemoryStore {
    state: Mutex<MemoryState>,
    reuse_replaced_revision: bool,
    repeat_cursor: bool,
}

impl MemoryStore {
    fn with_index(scope: &str, entries: &[&str]) -> Self {
        let mut state = MemoryState::default();
        state.indexes.insert(
            scope.to_owned(),
            entries.iter().map(|entry| (*entry).to_owned()).collect(),
        );
        Self {
            state: Mutex::new(state),
            ..Self::default()
        }
    }

    fn with_reused_revision() -> Self {
        Self {
            reuse_replaced_revision: true,
            ..Self::default()
        }
    }

    fn with_repeated_cursor(scope: &str, entries: &[&str]) -> Self {
        Self {
            repeat_cursor: true,
            ..Self::with_index(scope, entries)
        }
    }

    fn load_value(&self, scope: &str, key: &str) -> Result<Option<Stored<String>>, StorageError> {
        let state = self.state.lock().map_err(|_| StorageError::Internal)?;
        state
            .records
            .get(&(scope.to_owned(), key.to_owned()))
            .map(|(value, revision)| Ok(Stored::new(value.clone(), encode_revision(*revision)?)))
            .transpose()
    }

    fn write_value(
        &self,
        scope: &str,
        key: &str,
        write: StorageWrite<String>,
    ) -> Result<StorageWriteReceipt, StorageError> {
        let (value, condition) = write.into_parts();
        let mut state = self.state.lock().map_err(|_| StorageError::Internal)?;
        let record_key = (scope.to_owned(), key.to_owned());
        let current = state.records.get(&record_key).cloned();

        let allowed = match condition {
            StorageWriteCondition::Any => true,
            StorageWriteCondition::InsertOnly => current.is_none(),
            StorageWriteCondition::IfRevision(expected) => {
                current.as_ref().is_some_and(|(_, revision)| {
                    encode_revision(*revision).ok().as_ref() == Some(&expected)
                })
            }
            _ => false,
        };
        if !allowed {
            return Err(StorageError::Conflict);
        }

        let outcome = if current.is_some() {
            StorageWriteOutcome::Replaced
        } else {
            StorageWriteOutcome::Inserted
        };
        let revision = if self.reuse_replaced_revision {
            current.as_ref().map(|(_, revision)| *revision)
        } else {
            None
        }
        .unwrap_or_else(|| {
            state.next_revision += 1;
            state.next_revision
        });
        state.records.insert(record_key, (value, revision));
        Ok(StorageWriteReceipt::new(
            outcome,
            encode_revision(revision)?,
        ))
    }

    fn delete_value(
        &self,
        scope: &str,
        key: &str,
        condition: StorageDeleteCondition,
    ) -> Result<StorageDeleteOutcome, StorageError> {
        let mut state = self.state.lock().map_err(|_| StorageError::Internal)?;
        let record_key = (scope.to_owned(), key.to_owned());
        let current = state.records.get(&record_key).cloned();

        match condition {
            StorageDeleteCondition::Any => {}
            StorageDeleteCondition::IfRevision(expected) => {
                let matches = current.as_ref().is_some_and(|(_, revision)| {
                    encode_revision(*revision).ok().as_ref() == Some(&expected)
                });
                if !matches {
                    return Err(StorageError::Conflict);
                }
            }
            _ => return Err(StorageError::Internal),
        }

        Ok(if state.records.remove(&record_key).is_some() {
            StorageDeleteOutcome::Deleted
        } else {
            StorageDeleteOutcome::NotFound
        })
    }

    fn list_values(
        &self,
        scope: &str,
        request: StoragePageRequest,
    ) -> Result<StoragePage<String>, StorageError> {
        let state = self.state.lock().map_err(|_| StorageError::Internal)?;
        let entries = state.indexes.get(scope).map(Vec::as_slice).unwrap_or(&[]);
        let start = request.cursor().map_or(Ok(0), decode_cursor)?;
        if start > entries.len() {
            return Err(StorageError::InvalidCursor);
        }
        let end = start
            .saturating_add(request.size().get())
            .min(entries.len());
        let page_entries = entries[start..end].to_vec();
        let next_cursor = if end < entries.len() {
            if self.repeat_cursor {
                request
                    .cursor()
                    .cloned()
                    .map_or_else(|| encode_cursor(end), Ok)?
            } else {
                encode_cursor(end)?
            }
            .into()
        } else {
            None
        };
        StoragePage::new(page_entries, next_cursor)
    }
}

fn encode_revision(revision: u64) -> Result<StorageRevision, StorageError> {
    StorageRevision::new(revision.to_be_bytes().to_vec())
}

fn encode_cursor(offset: usize) -> Result<StorageCursor, StorageError> {
    StorageCursor::new((offset as u64).to_be_bytes().to_vec())
}

fn decode_cursor(cursor: &StorageCursor) -> Result<usize, StorageError> {
    let bytes: [u8; 8] = cursor
        .as_bytes()
        .try_into()
        .map_err(|_| StorageError::InvalidCursor)?;
    usize::try_from(u64::from_be_bytes(bytes)).map_err(|_| StorageError::InvalidCursor)
}

macro_rules! impl_exact_store {
    ($port:ident) => {
        impl $port for MemoryStore {
            type Scope = String;
            type Key = String;
            type Value = String;

            fn load<'a>(
                &'a self,
                scope: &'a Self::Scope,
                key: &'a Self::Key,
            ) -> StorageFuture<'a, Option<Stored<Self::Value>>> {
                Box::pin(async move { self.load_value(scope, key) })
            }

            fn write<'a>(
                &'a self,
                scope: &'a Self::Scope,
                key: &'a Self::Key,
                write: StorageWrite<Self::Value>,
            ) -> StorageFuture<'a, StorageWriteReceipt> {
                Box::pin(async move { self.write_value(scope, key, write) })
            }

            fn delete<'a>(
                &'a self,
                scope: &'a Self::Scope,
                key: &'a Self::Key,
                condition: StorageDeleteCondition,
            ) -> StorageFuture<'a, StorageDeleteOutcome> {
                Box::pin(async move { self.delete_value(scope, key, condition) })
            }
        }
    };
}

impl_exact_store!(SecretStore);
impl_exact_store!(StatusCacheStore);

macro_rules! impl_list_store {
    ($port:ident) => {
        impl $port for MemoryStore {
            type Scope = String;
            type Key = String;
            type Value = String;
            type IndexEntry = String;

            fn load<'a>(
                &'a self,
                scope: &'a Self::Scope,
                key: &'a Self::Key,
            ) -> StorageFuture<'a, Option<Stored<Self::Value>>> {
                Box::pin(async move { self.load_value(scope, key) })
            }

            fn write<'a>(
                &'a self,
                scope: &'a Self::Scope,
                key: &'a Self::Key,
                write: StorageWrite<Self::Value>,
            ) -> StorageFuture<'a, StorageWriteReceipt> {
                Box::pin(async move { self.write_value(scope, key, write) })
            }

            fn delete<'a>(
                &'a self,
                scope: &'a Self::Scope,
                key: &'a Self::Key,
                condition: StorageDeleteCondition,
            ) -> StorageFuture<'a, StorageDeleteOutcome> {
                Box::pin(async move { self.delete_value(scope, key, condition) })
            }

            fn list<'a>(
                &'a self,
                scope: &'a Self::Scope,
                request: StoragePageRequest,
            ) -> StorageFuture<'a, StoragePage<Self::IndexEntry>> {
                Box::pin(async move { self.list_values(scope, request) })
            }
        }
    };
}

impl_list_store!(CredentialStore);
impl_list_store!(DidStore);
impl_list_store!(ProtocolStateStore);

fn exact_fixture() -> ExactStoreFixture<String, String, String> {
    ExactStoreFixture::new(
        "primary".to_owned(),
        "isolated".to_owned(),
        "record".to_owned(),
        "missing".to_owned(),
        "initial".to_owned(),
        "replacement".to_owned(),
    )
}

fn list_fixture() -> ListStoreFixture<String, String> {
    ListStoreFixture::new(
        "index".to_owned(),
        vec!["alpha".to_owned(), "beta".to_owned(), "gamma".to_owned()],
        StoragePageSize::new(1).expect("one is a valid page size"),
    )
    .expect("fixture is valid")
}

#[test]
fn all_five_production_ports_pass_the_same_contract() {
    let secret = MemoryStore::default();
    let secret_object: &dyn SecretStore<Scope = String, Key = String, Value = String> = &secret;
    let report = block_on(check_secret_store(secret_object, &exact_fixture())).expect("secret");
    assert_eq!(report.completed_operations(), 16);
    assert_eq!(report.observed_entries(), 0);

    let status = MemoryStore::default();
    let report = block_on(check_status_cache_store(&status, &exact_fixture())).expect("status");
    assert_eq!(report.completed_operations(), 16);

    let credential = MemoryStore::with_index("index", &["alpha", "beta", "gamma"]);
    let report = block_on(check_credential_store(
        &credential,
        &exact_fixture(),
        &list_fixture(),
    ))
    .expect("credential");
    assert_eq!(report.completed_operations(), 19);
    assert_eq!(report.observed_entries(), 3);

    let did = MemoryStore::with_index("index", &["alpha", "beta", "gamma"]);
    let report = block_on(check_did_store(&did, &exact_fixture(), &list_fixture())).expect("did");
    assert_eq!(report.completed_operations(), 19);

    let protocol = MemoryStore::with_index("index", &["alpha", "beta", "gamma"]);
    let report = block_on(check_protocol_state_store(
        &protocol,
        &exact_fixture(),
        &list_fixture(),
    ))
    .expect("protocol state");
    assert_eq!(report.completed_operations(), 19);
}

#[test]
fn revision_reuse_fails_closed() {
    let error = block_on(check_secret_store(
        &MemoryStore::with_reused_revision(),
        &exact_fixture(),
    ))
    .expect_err("reused replacement revision must fail");
    assert_eq!(error.step(), "replace");
    assert_eq!(error.kind(), ConformanceFailureKind::RevisionNotInvalidated);
}

#[test]
fn repeated_cursor_fails_before_unbounded_work() {
    let store = MemoryStore::with_repeated_cursor("index", &["alpha", "beta", "gamma"]);
    let error = block_on(check_did_store(&store, &exact_fixture(), &list_fixture()))
        .expect_err("repeated cursor must fail");
    assert_eq!(error.step(), "list-cursor-progress");
    assert_eq!(error.kind(), ConformanceFailureKind::CursorDidNotProgress);
}

#[test]
fn list_fixture_rejects_unbounded_and_ambiguous_inputs() {
    let size_one = StoragePageSize::new(1).expect("valid");
    assert_eq!(
        ListStoreFixture::<(), String>::new((), Vec::new(), size_one).unwrap_err(),
        ConformanceFixtureError::InvalidExpectedEntryCount
    );
    assert_eq!(
        ListStoreFixture::new((), vec!["same", "same"], size_one).unwrap_err(),
        ConformanceFixtureError::DuplicateExpectedEntry
    );
    assert_eq!(
        ListStoreFixture::new(
            (),
            vec!["one", "two"],
            StoragePageSize::new(2).expect("valid")
        )
        .unwrap_err(),
        ConformanceFixtureError::PageSizeDoesNotPaginate
    );
}

#[derive(Clone, PartialEq, Eq)]
struct Canary {
    _secret: &'static str,
}

struct FailingCanaryStore;

impl SecretStore for FailingCanaryStore {
    type Scope = Canary;
    type Key = Canary;
    type Value = Canary;

    fn load<'a>(
        &'a self,
        _scope: &'a Self::Scope,
        _key: &'a Self::Key,
    ) -> StorageFuture<'a, Option<Stored<Self::Value>>> {
        Box::pin(async { Err(StorageError::Internal) })
    }

    fn write<'a>(
        &'a self,
        _scope: &'a Self::Scope,
        _key: &'a Self::Key,
        _write: StorageWrite<Self::Value>,
    ) -> StorageFuture<'a, StorageWriteReceipt> {
        Box::pin(async { Err(StorageError::Internal) })
    }

    fn delete<'a>(
        &'a self,
        _scope: &'a Self::Scope,
        _key: &'a Self::Key,
        _condition: StorageDeleteCondition,
    ) -> StorageFuture<'a, StorageDeleteOutcome> {
        Box::pin(async { Err(StorageError::Internal) })
    }
}

#[test]
fn non_debug_fixture_values_never_enter_diagnostics() {
    let canary = || Canary {
        _secret: "TOP-SECRET-CANARY",
    };
    let fixture =
        ExactStoreFixture::new(canary(), canary(), canary(), canary(), canary(), canary());
    let error = block_on(check_secret_store(&FailingCanaryStore, &fixture)).expect_err("fails");
    let rendered = format!("{fixture:?} {error:?} {error}");
    assert!(!rendered.contains("TOP-SECRET-CANARY"));
    assert_eq!(error.step(), "missing-load");
    assert_eq!(error.kind(), ConformanceFailureKind::OperationFailed);
}

#[test]
#[ignore = "release diagnostic; run explicitly"]
fn release_exact_suite_throughput() {
    const RUNS: usize = 10_000;
    let store = MemoryStore::default();
    let fixture = exact_fixture();
    let started = Instant::now();
    for _ in 0..RUNS {
        block_on(check_secret_store(&store, &fixture)).expect("conformance run");
    }
    let elapsed = started.elapsed();
    let port_calls = RUNS * 16;
    let calls_per_second = port_calls as f64 / elapsed.as_secs_f64();
    eprintln!(
        "wallet storage conformance: {port_calls} port calls in {elapsed:?} ({calls_per_second:.0} calls/s)"
    );
}

struct ThreadWake(std::thread::Thread);

impl Wake for ThreadWake {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}

fn block_on<F: Future>(future: F) -> F::Output {
    let waker = Waker::from(Arc::new(ThreadWake(std::thread::current())));
    let mut context = Context::from_waker(&waker);
    let mut future = Box::pin(future);
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => return output,
            Poll::Pending => std::thread::park(),
        }
    }
}
