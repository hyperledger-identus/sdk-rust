//! Reusable behavioral conformance checks for Identus wallet storage adapters.
//!
//! This verification-only crate invokes the production `identus-wallet`
//! storage traits directly. It provides no executor or storage implementation;
//! consumers await the checks in their own test runtime.

#![forbid(unsafe_code)]

use core::fmt;

use identus_wallet::{
    CredentialStore, DidStore, ProtocolStateStore, SecretStore, StatusCacheStore, StorageCursor,
    StorageDeleteCondition, StorageDeleteOutcome, StorageError, StorageFuture, StoragePageRequest,
    StoragePageSize, StorageWrite, StorageWriteCondition, StorageWriteOutcome,
};

/// Maximum number of index entries accepted by one conformance fixture.
pub const MAX_CONFORMANCE_INDEX_ENTRIES: usize = 4_096;

/// Exact-key values used to exercise one consumer storage capability.
pub struct ExactStoreFixture<Scope, Key, Value> {
    scope: Scope,
    isolated_scope: Scope,
    key: Key,
    missing_key: Key,
    initial_value: Value,
    replacement_value: Value,
}

impl<Scope, Key, Value> ExactStoreFixture<Scope, Key, Value> {
    /// Construct a fixture whose scopes and keys are known to be distinct.
    ///
    /// Use a disposable namespace: a failing adapter may leave the primary
    /// record behind because generic cleanup cannot safely override a failed
    /// compare-and-swap contract.
    #[must_use]
    pub const fn new(
        scope: Scope,
        isolated_scope: Scope,
        key: Key,
        missing_key: Key,
        initial_value: Value,
        replacement_value: Value,
    ) -> Self {
        Self {
            scope,
            isolated_scope,
            key,
            missing_key,
            initial_value,
            replacement_value,
        }
    }
}

impl<Scope, Key, Value> fmt::Debug for ExactStoreFixture<Scope, Key, Value> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ExactStoreFixture")
            .finish_non_exhaustive()
    }
}

/// A preseeded recovery-index fixture for a list-capable storage adapter.
pub struct ListStoreFixture<Scope, Entry> {
    scope: Scope,
    expected_entries: Vec<Entry>,
    page_size: StoragePageSize,
}

impl<Scope, Entry: PartialEq> ListStoreFixture<Scope, Entry> {
    /// Validate expected entries and a page size that forces pagination.
    pub fn new(
        scope: Scope,
        expected_entries: Vec<Entry>,
        page_size: StoragePageSize,
    ) -> Result<Self, ConformanceFixtureError> {
        if !(2..=MAX_CONFORMANCE_INDEX_ENTRIES).contains(&expected_entries.len()) {
            return Err(ConformanceFixtureError::InvalidExpectedEntryCount);
        }
        if page_size.get() >= expected_entries.len() {
            return Err(ConformanceFixtureError::PageSizeDoesNotPaginate);
        }
        for (offset, entry) in expected_entries.iter().enumerate() {
            if expected_entries[offset + 1..]
                .iter()
                .any(|candidate| candidate == entry)
            {
                return Err(ConformanceFixtureError::DuplicateExpectedEntry);
            }
        }
        Ok(Self {
            scope,
            expected_entries,
            page_size,
        })
    }

    /// Return the number of entries the adapter was seeded with.
    #[must_use]
    pub fn expected_entry_count(&self) -> usize {
        self.expected_entries.len()
    }
}

impl<Scope, Entry> fmt::Debug for ListStoreFixture<Scope, Entry> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ListStoreFixture")
            .field("expected_entry_count", &self.expected_entries.len())
            .field("page_size", &self.page_size.get())
            .finish()
    }
}

/// Static reasons why a caller-provided conformance fixture is invalid.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ConformanceFixtureError {
    /// Expected entry count is outside the bounded range 2 through 4,096.
    InvalidExpectedEntryCount,
    /// The requested page size would not require more than one page.
    PageSizeDoesNotPaginate,
    /// The expected entry set contains a duplicate.
    DuplicateExpectedEntry,
}

impl fmt::Display for ConformanceFixtureError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidExpectedEntryCount => "expected entry count is invalid",
            Self::PageSizeDoesNotPaginate => "page size does not force pagination",
            Self::DuplicateExpectedEntry => "expected entries contain a duplicate",
        })
    }
}

impl std::error::Error for ConformanceFixtureError {}

/// Closed, value-free classes of storage conformance failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ConformanceFailureKind {
    /// A port returned an operational error where success was required.
    OperationFailed,
    /// A port returned an operational error other than the expected conflict.
    UnexpectedStorageError,
    /// A record was present or absent contrary to the contract.
    UnexpectedRecordPresence,
    /// A loaded consumer value differed from the expected fixture value.
    ValueMismatch,
    /// A loaded revision differed from the successful write receipt.
    RevisionMismatch,
    /// A write receipt reported the wrong mutation outcome.
    WriteOutcomeMismatch,
    /// A mutation succeeded when a conflict was required.
    ExpectedConflict,
    /// A delete reported the wrong outcome.
    DeleteOutcomeMismatch,
    /// A replacement reused the revision it invalidated.
    RevisionNotInvalidated,
    /// A record crossed the consumer-provided scope boundary.
    ScopeIsolationViolation,
    /// A list response exceeded the caller-provided page bound.
    PageBoundExceeded,
    /// Pagination repeated a previously observed continuation cursor.
    CursorDidNotProgress,
    /// Pagination did not terminate within its fixture-derived bound.
    PaginationDidNotTerminate,
    /// Pagination returned the same index entry more than once.
    DuplicateObservedEntry,
    /// The observed index entries did not equal the expected fixture set.
    IndexMembershipMismatch,
}

/// A redaction-safe failure at one static conformance step.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConformanceFailure {
    step: &'static str,
    kind: ConformanceFailureKind,
}

impl ConformanceFailure {
    const fn new(step: &'static str, kind: ConformanceFailureKind) -> Self {
        Self { step, kind }
    }

    /// Return the static suite step that failed.
    #[must_use]
    pub const fn step(self) -> &'static str {
        self.step
    }

    /// Return the value-free failure class.
    #[must_use]
    pub const fn kind(self) -> ConformanceFailureKind {
        self.kind
    }
}

impl fmt::Display for ConformanceFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "storage conformance failed at {}: {:?}",
            self.step, self.kind
        )
    }
}

impl std::error::Error for ConformanceFailure {}

/// Aggregate, value-free evidence from one successful conformance run.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StorageConformanceReport {
    completed_operations: usize,
    observed_entries: usize,
}

impl StorageConformanceReport {
    /// Number of completed port calls checked by the suite.
    #[must_use]
    pub const fn completed_operations(self) -> usize {
        self.completed_operations
    }

    /// Number of recovery-index entries observed by the list suite.
    #[must_use]
    pub const fn observed_entries(self) -> usize {
        self.observed_entries
    }

    const fn combine(self, other: Self) -> Self {
        Self {
            completed_operations: self.completed_operations + other.completed_operations,
            observed_entries: self.observed_entries + other.observed_entries,
        }
    }
}

trait ExactDriver: Sync {
    type Scope: Send + Sync;
    type Key: Send + Sync;
    type Value: Send + Sync;

    fn load<'a>(
        &'a self,
        scope: &'a Self::Scope,
        key: &'a Self::Key,
    ) -> StorageFuture<'a, Option<identus_wallet::Stored<Self::Value>>>;

    fn write<'a>(
        &'a self,
        scope: &'a Self::Scope,
        key: &'a Self::Key,
        write: StorageWrite<Self::Value>,
    ) -> StorageFuture<'a, identus_wallet::StorageWriteReceipt>;

    fn delete<'a>(
        &'a self,
        scope: &'a Self::Scope,
        key: &'a Self::Key,
        condition: StorageDeleteCondition,
    ) -> StorageFuture<'a, StorageDeleteOutcome>;
}

trait ListDriver: ExactDriver {
    type IndexEntry: Send + Sync;

    fn list<'a>(
        &'a self,
        scope: &'a Self::Scope,
        request: StoragePageRequest,
    ) -> StorageFuture<'a, identus_wallet::StoragePage<Self::IndexEntry>>;
}

async fn run_exact<D>(
    driver: &D,
    fixture: &ExactStoreFixture<D::Scope, D::Key, D::Value>,
) -> Result<StorageConformanceReport, ConformanceFailure>
where
    D: ExactDriver + ?Sized,
    D::Value: Clone + PartialEq,
{
    let mut operations = 0usize;

    let missing = driver
        .load(&fixture.scope, &fixture.missing_key)
        .await
        .map_err(|_| failure("missing-load", ConformanceFailureKind::OperationFailed))?;
    operations += 1;
    if missing.is_some() {
        return Err(failure(
            "missing-load",
            ConformanceFailureKind::UnexpectedRecordPresence,
        ));
    }

    let initial = driver
        .load(&fixture.scope, &fixture.key)
        .await
        .map_err(|_| failure("clean-load", ConformanceFailureKind::OperationFailed))?;
    operations += 1;
    if initial.is_some() {
        return Err(failure(
            "clean-load",
            ConformanceFailureKind::UnexpectedRecordPresence,
        ));
    }

    let isolated = driver
        .load(&fixture.isolated_scope, &fixture.key)
        .await
        .map_err(|_| {
            failure(
                "isolated-clean-load",
                ConformanceFailureKind::OperationFailed,
            )
        })?;
    operations += 1;
    if isolated.is_some() {
        return Err(failure(
            "isolated-clean-load",
            ConformanceFailureKind::ScopeIsolationViolation,
        ));
    }

    let inserted = driver
        .write(
            &fixture.scope,
            &fixture.key,
            StorageWrite::new(
                fixture.initial_value.clone(),
                StorageWriteCondition::InsertOnly,
            ),
        )
        .await
        .map_err(|_| failure("insert-only", ConformanceFailureKind::OperationFailed))?;
    operations += 1;
    if inserted.outcome() != StorageWriteOutcome::Inserted {
        return Err(failure(
            "insert-only",
            ConformanceFailureKind::WriteOutcomeMismatch,
        ));
    }
    let first_revision = inserted.revision().clone();

    let loaded = required_load(driver, &fixture.scope, &fixture.key, "read-after-insert").await?;
    operations += 1;
    if loaded.value() != &fixture.initial_value {
        return Err(failure(
            "read-after-insert",
            ConformanceFailureKind::ValueMismatch,
        ));
    }
    if loaded.revision() != &first_revision {
        return Err(failure(
            "read-after-insert",
            ConformanceFailureKind::RevisionMismatch,
        ));
    }

    let isolated = driver
        .load(&fixture.isolated_scope, &fixture.key)
        .await
        .map_err(|_| failure("scope-isolation", ConformanceFailureKind::OperationFailed))?;
    operations += 1;
    if isolated.is_some() {
        return Err(failure(
            "scope-isolation",
            ConformanceFailureKind::ScopeIsolationViolation,
        ));
    }

    expect_write_conflict(
        driver,
        &fixture.scope,
        &fixture.key,
        StorageWrite::new(
            fixture.replacement_value.clone(),
            StorageWriteCondition::InsertOnly,
        ),
        "duplicate-insert",
    )
    .await?;
    operations += 1;

    let preserved = required_load(
        driver,
        &fixture.scope,
        &fixture.key,
        "insert-conflict-preserves",
    )
    .await?;
    operations += 1;
    if preserved.value() != &fixture.initial_value || preserved.revision() != &first_revision {
        return Err(failure(
            "insert-conflict-preserves",
            ConformanceFailureKind::ValueMismatch,
        ));
    }

    let replaced = driver
        .write(
            &fixture.scope,
            &fixture.key,
            StorageWrite::new(
                fixture.replacement_value.clone(),
                StorageWriteCondition::Any,
            ),
        )
        .await
        .map_err(|_| failure("replace", ConformanceFailureKind::OperationFailed))?;
    operations += 1;
    if replaced.outcome() != StorageWriteOutcome::Replaced {
        return Err(failure(
            "replace",
            ConformanceFailureKind::WriteOutcomeMismatch,
        ));
    }
    let current_revision = replaced.revision().clone();
    if current_revision == first_revision {
        return Err(failure(
            "replace",
            ConformanceFailureKind::RevisionNotInvalidated,
        ));
    }

    expect_write_conflict(
        driver,
        &fixture.scope,
        &fixture.key,
        StorageWrite::new(
            fixture.initial_value.clone(),
            StorageWriteCondition::IfRevision(first_revision.clone()),
        ),
        "stale-write",
    )
    .await?;
    operations += 1;

    let current = required_load(
        driver,
        &fixture.scope,
        &fixture.key,
        "stale-write-preserves",
    )
    .await?;
    operations += 1;
    if current.value() != &fixture.replacement_value || current.revision() != &current_revision {
        return Err(failure(
            "stale-write-preserves",
            ConformanceFailureKind::ValueMismatch,
        ));
    }

    expect_delete_conflict(
        driver,
        &fixture.scope,
        &fixture.key,
        StorageDeleteCondition::IfRevision(first_revision),
        "stale-delete",
    )
    .await?;
    operations += 1;

    let current = required_load(
        driver,
        &fixture.scope,
        &fixture.key,
        "stale-delete-preserves",
    )
    .await?;
    operations += 1;
    if current.value() != &fixture.replacement_value || current.revision() != &current_revision {
        return Err(failure(
            "stale-delete-preserves",
            ConformanceFailureKind::ValueMismatch,
        ));
    }

    let deleted = driver
        .delete(
            &fixture.scope,
            &fixture.key,
            StorageDeleteCondition::IfRevision(current_revision),
        )
        .await
        .map_err(|_| failure("current-delete", ConformanceFailureKind::OperationFailed))?;
    operations += 1;
    if deleted != StorageDeleteOutcome::Deleted {
        return Err(failure(
            "current-delete",
            ConformanceFailureKind::DeleteOutcomeMismatch,
        ));
    }

    let absent = driver
        .load(&fixture.scope, &fixture.key)
        .await
        .map_err(|_| failure("read-after-delete", ConformanceFailureKind::OperationFailed))?;
    operations += 1;
    if absent.is_some() {
        return Err(failure(
            "read-after-delete",
            ConformanceFailureKind::UnexpectedRecordPresence,
        ));
    }

    let missing_delete = driver
        .delete(&fixture.scope, &fixture.key, StorageDeleteCondition::Any)
        .await
        .map_err(|_| failure("missing-delete", ConformanceFailureKind::OperationFailed))?;
    operations += 1;
    if missing_delete != StorageDeleteOutcome::NotFound {
        return Err(failure(
            "missing-delete",
            ConformanceFailureKind::DeleteOutcomeMismatch,
        ));
    }

    Ok(StorageConformanceReport {
        completed_operations: operations,
        observed_entries: 0,
    })
}

async fn required_load<'a, D: ExactDriver + ?Sized>(
    driver: &'a D,
    scope: &'a D::Scope,
    key: &'a D::Key,
    step: &'static str,
) -> Result<identus_wallet::Stored<D::Value>, ConformanceFailure> {
    driver
        .load(scope, key)
        .await
        .map_err(|_| failure(step, ConformanceFailureKind::OperationFailed))?
        .ok_or_else(|| failure(step, ConformanceFailureKind::UnexpectedRecordPresence))
}

async fn expect_write_conflict<D: ExactDriver + ?Sized>(
    driver: &D,
    scope: &D::Scope,
    key: &D::Key,
    write: StorageWrite<D::Value>,
    step: &'static str,
) -> Result<(), ConformanceFailure> {
    match driver.write(scope, key, write).await {
        Err(StorageError::Conflict) => Ok(()),
        Err(_) => Err(failure(
            step,
            ConformanceFailureKind::UnexpectedStorageError,
        )),
        Ok(_) => Err(failure(step, ConformanceFailureKind::ExpectedConflict)),
    }
}

async fn expect_delete_conflict<D: ExactDriver + ?Sized>(
    driver: &D,
    scope: &D::Scope,
    key: &D::Key,
    condition: StorageDeleteCondition,
    step: &'static str,
) -> Result<(), ConformanceFailure> {
    match driver.delete(scope, key, condition).await {
        Err(StorageError::Conflict) => Ok(()),
        Err(_) => Err(failure(
            step,
            ConformanceFailureKind::UnexpectedStorageError,
        )),
        Ok(_) => Err(failure(step, ConformanceFailureKind::ExpectedConflict)),
    }
}

async fn run_list<D>(
    driver: &D,
    fixture: &ListStoreFixture<D::Scope, D::IndexEntry>,
) -> Result<StorageConformanceReport, ConformanceFailure>
where
    D: ListDriver + ?Sized,
    D::IndexEntry: PartialEq,
{
    let mut operations = 0usize;
    let mut observed = Vec::with_capacity(fixture.expected_entries.len());
    let mut seen_cursors: Vec<StorageCursor> = Vec::new();
    let mut cursor = None;
    let max_pages = fixture.expected_entries.len() + 1;

    loop {
        if operations >= max_pages {
            return Err(failure(
                "list-termination",
                ConformanceFailureKind::PaginationDidNotTerminate,
            ));
        }
        let page = driver
            .list(
                &fixture.scope,
                StoragePageRequest::new(fixture.page_size, cursor),
            )
            .await
            .map_err(|_| failure("list-page", ConformanceFailureKind::OperationFailed))?;
        operations += 1;
        if page.len() > fixture.page_size.get() {
            return Err(failure(
                "list-page-bound",
                ConformanceFailureKind::PageBoundExceeded,
            ));
        }
        let (entries, next_cursor) = page.into_parts();
        for entry in entries {
            if observed.iter().any(|candidate| candidate == &entry) {
                return Err(failure(
                    "list-duplicate-entry",
                    ConformanceFailureKind::DuplicateObservedEntry,
                ));
            }
            observed.push(entry);
            if observed.len() > fixture.expected_entries.len() {
                return Err(failure(
                    "list-membership",
                    ConformanceFailureKind::IndexMembershipMismatch,
                ));
            }
        }

        let Some(next_cursor) = next_cursor else {
            break;
        };
        if seen_cursors
            .iter()
            .any(|candidate| candidate == &next_cursor)
        {
            return Err(failure(
                "list-cursor-progress",
                ConformanceFailureKind::CursorDidNotProgress,
            ));
        }
        seen_cursors.push(next_cursor.clone());
        cursor = Some(next_cursor);
    }

    if observed.len() != fixture.expected_entries.len()
        || fixture
            .expected_entries
            .iter()
            .any(|expected| !observed.iter().any(|actual| actual == expected))
    {
        return Err(failure(
            "list-membership",
            ConformanceFailureKind::IndexMembershipMismatch,
        ));
    }

    Ok(StorageConformanceReport {
        completed_operations: operations,
        observed_entries: observed.len(),
    })
}

const fn failure(step: &'static str, kind: ConformanceFailureKind) -> ConformanceFailure {
    ConformanceFailure::new(step, kind)
}

struct SecretDriver<'a, S: ?Sized>(&'a S);
struct CredentialDriver<'a, S: ?Sized>(&'a S);
struct DidDriver<'a, S: ?Sized>(&'a S);
struct ProtocolStateDriver<'a, S: ?Sized>(&'a S);
struct StatusCacheDriver<'a, S: ?Sized>(&'a S);

macro_rules! impl_exact_driver {
    ($wrapper:ident, $port:ident) => {
        impl<S: $port + ?Sized> ExactDriver for $wrapper<'_, S> {
            type Scope = S::Scope;
            type Key = S::Key;
            type Value = S::Value;

            fn load<'a>(
                &'a self,
                scope: &'a Self::Scope,
                key: &'a Self::Key,
            ) -> StorageFuture<'a, Option<identus_wallet::Stored<Self::Value>>> {
                $port::load(self.0, scope, key)
            }

            fn write<'a>(
                &'a self,
                scope: &'a Self::Scope,
                key: &'a Self::Key,
                write: StorageWrite<Self::Value>,
            ) -> StorageFuture<'a, identus_wallet::StorageWriteReceipt> {
                $port::write(self.0, scope, key, write)
            }

            fn delete<'a>(
                &'a self,
                scope: &'a Self::Scope,
                key: &'a Self::Key,
                condition: StorageDeleteCondition,
            ) -> StorageFuture<'a, StorageDeleteOutcome> {
                $port::delete(self.0, scope, key, condition)
            }
        }
    };
}

impl_exact_driver!(SecretDriver, SecretStore);
impl_exact_driver!(CredentialDriver, CredentialStore);
impl_exact_driver!(DidDriver, DidStore);
impl_exact_driver!(ProtocolStateDriver, ProtocolStateStore);
impl_exact_driver!(StatusCacheDriver, StatusCacheStore);

macro_rules! impl_list_driver {
    ($wrapper:ident, $port:ident) => {
        impl<S: $port + ?Sized> ListDriver for $wrapper<'_, S> {
            type IndexEntry = S::IndexEntry;

            fn list<'a>(
                &'a self,
                scope: &'a Self::Scope,
                request: StoragePageRequest,
            ) -> StorageFuture<'a, identus_wallet::StoragePage<Self::IndexEntry>> {
                $port::list(self.0, scope, request)
            }
        }
    };
}

impl_list_driver!(CredentialDriver, CredentialStore);
impl_list_driver!(DidDriver, DidStore);
impl_list_driver!(ProtocolStateDriver, ProtocolStateStore);

/// Check one secret store without requesting enumeration authority.
///
/// The fixture namespace should be disposable because failure can interrupt
/// the lifecycle before its final conditional deletion.
pub async fn check_secret_store<S>(
    store: &S,
    fixture: &ExactStoreFixture<S::Scope, S::Key, S::Value>,
) -> Result<StorageConformanceReport, ConformanceFailure>
where
    S: SecretStore + ?Sized,
    S::Value: Clone + PartialEq,
{
    let driver = SecretDriver(store);
    run_exact(&driver, fixture).await
}

/// Check one credential store including its preseeded recovery index.
///
/// The exact fixture namespace should be disposable. The list fixture must be
/// preseeded through the consumer adapter's native setup API.
pub async fn check_credential_store<S>(
    store: &S,
    exact: &ExactStoreFixture<S::Scope, S::Key, S::Value>,
    list: &ListStoreFixture<S::Scope, S::IndexEntry>,
) -> Result<StorageConformanceReport, ConformanceFailure>
where
    S: CredentialStore + ?Sized,
    S::Value: Clone + PartialEq,
    S::IndexEntry: PartialEq,
{
    let driver = CredentialDriver(store);
    Ok(run_exact(&driver, exact)
        .await?
        .combine(run_list(&driver, list).await?))
}

/// Check one DID store including its preseeded recovery index.
///
/// The exact fixture namespace should be disposable. The list fixture must be
/// preseeded through the consumer adapter's native setup API.
pub async fn check_did_store<S>(
    store: &S,
    exact: &ExactStoreFixture<S::Scope, S::Key, S::Value>,
    list: &ListStoreFixture<S::Scope, S::IndexEntry>,
) -> Result<StorageConformanceReport, ConformanceFailure>
where
    S: DidStore + ?Sized,
    S::Value: Clone + PartialEq,
    S::IndexEntry: PartialEq,
{
    let driver = DidDriver(store);
    Ok(run_exact(&driver, exact)
        .await?
        .combine(run_list(&driver, list).await?))
}

/// Check one protocol-state store including its preseeded recovery index.
///
/// The exact fixture namespace should be disposable. The list fixture must be
/// preseeded through the consumer adapter's native setup API.
pub async fn check_protocol_state_store<S>(
    store: &S,
    exact: &ExactStoreFixture<S::Scope, S::Key, S::Value>,
    list: &ListStoreFixture<S::Scope, S::IndexEntry>,
) -> Result<StorageConformanceReport, ConformanceFailure>
where
    S: ProtocolStateStore + ?Sized,
    S::Value: Clone + PartialEq,
    S::IndexEntry: PartialEq,
{
    let driver = ProtocolStateDriver(store);
    Ok(run_exact(&driver, exact)
        .await?
        .combine(run_list(&driver, list).await?))
}

/// Check one exact-key status cache without requesting sweep authority.
///
/// The fixture namespace should be disposable because failure can interrupt
/// the lifecycle before its final conditional deletion.
pub async fn check_status_cache_store<S>(
    store: &S,
    fixture: &ExactStoreFixture<S::Scope, S::Key, S::Value>,
) -> Result<StorageConformanceReport, ConformanceFailure>
where
    S: StatusCacheStore + ?Sized,
    S::Value: Clone + PartialEq,
{
    let driver = StatusCacheDriver(store);
    run_exact(&driver, fixture).await
}

#[cfg(test)]
mod tests;
