use std::collections::BTreeMap;
use std::future::Future;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};
use std::time::Instant;

use super::*;
use identus_wallet::{
    StorageCursor, StorageError, StoragePage, StoragePageSize, StorageRevision,
    StorageWriteCondition, StorageWriteOutcome, StorageWriteReceipt, Stored,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ExactOperation {
    LoadMissing,
    LoadPrimary,
    LoadIsolated,
    WriteInsertOnly,
    WriteAny,
    WriteIfRevision,
    DeleteIfRevision,
    DeleteAny,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ListFault {
    OperationFailed,
    OversizedPage,
    DuplicateEntry,
    WrongMembership,
    ExcessEntry,
    RepeatCursor,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ListRequestEvidence {
    continuation: bool,
    page_size: usize,
}

#[derive(Default)]
struct MemoryState {
    records: BTreeMap<(String, String), (String, u64)>,
    indexes: BTreeMap<String, Vec<String>>,
    next_revision: u64,
    exact_operations: Vec<ExactOperation>,
    list_requests: Vec<ListRequestEvidence>,
}

#[derive(Default)]
struct MemoryStore {
    state: Mutex<MemoryState>,
    reuse_replaced_revision: bool,
    list_fault: Option<ListFault>,
    corrupt_revision_on_operation: Option<usize>,
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

    fn with_list_fault(scope: &str, entries: &[&str], list_fault: ListFault) -> Self {
        Self {
            list_fault: Some(list_fault),
            ..Self::with_index(scope, entries)
        }
    }

    fn with_corrupt_revision_on_operation(operation: usize) -> Self {
        Self {
            corrupt_revision_on_operation: Some(operation),
            ..Self::default()
        }
    }

    fn exact_operations(&self) -> Vec<ExactOperation> {
        self.state
            .lock()
            .expect("test memory state is available")
            .exact_operations
            .clone()
    }

    fn list_requests(&self) -> Vec<ListRequestEvidence> {
        self.state
            .lock()
            .expect("test memory state is available")
            .list_requests
            .clone()
    }

    fn load_value(&self, scope: &str, key: &str) -> Result<Option<Stored<String>>, StorageError> {
        let mut state = self.state.lock().map_err(|_| StorageError::Internal)?;
        state.exact_operations.push(match (scope, key) {
            ("primary", "missing") => ExactOperation::LoadMissing,
            ("isolated", "record") => ExactOperation::LoadIsolated,
            _ => ExactOperation::LoadPrimary,
        });
        let operation = state.exact_operations.len();
        state
            .records
            .get(&(scope.to_owned(), key.to_owned()))
            .map(|(value, revision)| {
                let reported_revision = if self.corrupt_revision_on_operation == Some(operation) {
                    revision.checked_add(1).ok_or(StorageError::Internal)?
                } else {
                    *revision
                };
                Ok(Stored::new(
                    value.clone(),
                    encode_revision(reported_revision)?,
                ))
            })
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
        state.exact_operations.push(match &condition {
            StorageWriteCondition::InsertOnly => ExactOperation::WriteInsertOnly,
            StorageWriteCondition::Any => ExactOperation::WriteAny,
            StorageWriteCondition::IfRevision(_) => ExactOperation::WriteIfRevision,
            _ => return Err(StorageError::Internal),
        });
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
        state.exact_operations.push(match &condition {
            StorageDeleteCondition::IfRevision(_) => ExactOperation::DeleteIfRevision,
            StorageDeleteCondition::Any => ExactOperation::DeleteAny,
            _ => return Err(StorageError::Internal),
        });
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
        let mut state = self.state.lock().map_err(|_| StorageError::Internal)?;
        state.list_requests.push(ListRequestEvidence {
            continuation: request.cursor().is_some(),
            page_size: request.size().get(),
        });
        let call = state.list_requests.len();
        if self.list_fault == Some(ListFault::OperationFailed) {
            return Err(StorageError::Internal);
        }
        if let Some(entries) = synthetic_fault_page(self.list_fault, call) {
            let (entries, cursor) = entries;
            return StoragePage::new(entries, cursor.map(encode_cursor).transpose()?);
        }
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
            if self.list_fault == Some(ListFault::RepeatCursor) {
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

fn synthetic_fault_page(
    fault: Option<ListFault>,
    call: usize,
) -> Option<(Vec<String>, Option<usize>)> {
    let values = match fault? {
        ListFault::OversizedPage => (vec!["alpha", "beta"], None),
        ListFault::DuplicateEntry => match call {
            1 => (vec!["alpha"], Some(1)),
            _ => (vec!["alpha"], None),
        },
        ListFault::WrongMembership => match call {
            1 => (vec!["alpha"], Some(1)),
            2 => (vec!["beta"], Some(2)),
            _ => (vec!["delta"], None),
        },
        ListFault::ExcessEntry => match call {
            1 => (vec!["alpha"], Some(1)),
            2 => (vec!["beta"], Some(2)),
            3 => (vec!["gamma"], Some(3)),
            _ => (vec!["delta"], None),
        },
        ListFault::OperationFailed | ListFault::RepeatCursor => {
            return None;
        }
    };
    Some((values.0.into_iter().map(str::to_owned).collect(), values.1))
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
    assert_eq!(
        credential.list_requests(),
        [
            ListRequestEvidence {
                continuation: false,
                page_size: 1,
            },
            ListRequestEvidence {
                continuation: true,
                page_size: 1,
            },
            ListRequestEvidence {
                continuation: true,
                page_size: 1,
            },
        ]
    );

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
fn exact_suite_preserves_the_complete_operation_transcript() {
    let store = MemoryStore::default();
    let report = block_on(check_secret_store(&store, &exact_fixture())).expect("exact suite");
    assert_eq!(report.completed_operations(), 16);
    assert_eq!(
        store.exact_operations(),
        [
            ExactOperation::LoadMissing,
            ExactOperation::LoadPrimary,
            ExactOperation::LoadIsolated,
            ExactOperation::WriteInsertOnly,
            ExactOperation::LoadPrimary,
            ExactOperation::LoadIsolated,
            ExactOperation::WriteInsertOnly,
            ExactOperation::LoadPrimary,
            ExactOperation::WriteAny,
            ExactOperation::WriteIfRevision,
            ExactOperation::LoadPrimary,
            ExactOperation::DeleteIfRevision,
            ExactOperation::LoadPrimary,
            ExactOperation::DeleteIfRevision,
            ExactOperation::LoadPrimary,
            ExactOperation::DeleteAny,
        ]
    );
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
fn revision_mismatch_failure_projection_remains_step_specific() {
    for (operation, step, kind) in [
        (
            5,
            "read-after-insert",
            ConformanceFailureKind::RevisionMismatch,
        ),
        (
            8,
            "insert-conflict-preserves",
            ConformanceFailureKind::ValueMismatch,
        ),
        (
            11,
            "stale-write-preserves",
            ConformanceFailureKind::ValueMismatch,
        ),
        (
            13,
            "stale-delete-preserves",
            ConformanceFailureKind::ValueMismatch,
        ),
    ] {
        let error = block_on(check_secret_store(
            &MemoryStore::with_corrupt_revision_on_operation(operation),
            &exact_fixture(),
        ))
        .expect_err("corrupt revision must fail");
        assert_eq!(error.step(), step);
        assert_eq!(error.kind(), kind);
    }
}

#[test]
fn list_failures_preserve_exact_projection_and_request_bound() {
    for (fault, step, kind, completed_requests) in [
        (
            ListFault::OperationFailed,
            "list-page",
            ConformanceFailureKind::OperationFailed,
            1,
        ),
        (
            ListFault::OversizedPage,
            "list-page-bound",
            ConformanceFailureKind::PageBoundExceeded,
            1,
        ),
        (
            ListFault::DuplicateEntry,
            "list-duplicate-entry",
            ConformanceFailureKind::DuplicateObservedEntry,
            2,
        ),
        (
            ListFault::WrongMembership,
            "list-membership",
            ConformanceFailureKind::IndexMembershipMismatch,
            3,
        ),
        (
            ListFault::ExcessEntry,
            "list-membership",
            ConformanceFailureKind::IndexMembershipMismatch,
            4,
        ),
        (
            ListFault::RepeatCursor,
            "list-cursor-progress",
            ConformanceFailureKind::CursorDidNotProgress,
            2,
        ),
    ] {
        let store = MemoryStore::with_list_fault("index", &["alpha", "beta", "gamma"], fault);
        let error = block_on(check_did_store(&store, &exact_fixture(), &list_fixture()))
            .expect_err("fault must fail closed");
        assert_eq!(error.step(), step, "fault: {fault:?}");
        assert_eq!(error.kind(), kind, "fault: {fault:?}");
        assert_eq!(
            store.list_requests().len(),
            completed_requests,
            "fault: {fault:?}"
        );
    }
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
