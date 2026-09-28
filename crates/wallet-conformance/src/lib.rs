//! Reusable behavioral conformance checks for Identus wallet storage adapters.
//!
//! This verification-only crate invokes the production `identus-wallet`
//! storage traits directly. It provides no executor or storage implementation;
//! consumers await the checks in their own test runtime.

#![forbid(unsafe_code)]

mod exact;
mod list;

use core::fmt;

use identus_wallet::{
    CredentialStore, DidStore, ProtocolStateStore, SecretStore, StatusCacheStore,
    StorageDeleteCondition, StorageDeleteOutcome, StorageFuture, StoragePageRequest, StorageWrite,
};

pub use exact::ExactStoreFixture;
pub use list::ListStoreFixture;

/// Maximum number of index entries accepted by one conformance fixture.
pub const MAX_CONFORMANCE_INDEX_ENTRIES: usize = 4_096;

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

    const fn new(completed_operations: usize, observed_entries: usize) -> Self {
        Self {
            completed_operations,
            observed_entries,
        }
    }

    const fn combine(self, other: Self) -> Self {
        Self::new(
            self.completed_operations + other.completed_operations,
            self.observed_entries + other.observed_entries,
        )
    }
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
        impl<S: $port + ?Sized> exact::ExactDriver for $wrapper<'_, S> {
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
        impl<S: $port + ?Sized> list::ListDriver for $wrapper<'_, S> {
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
    exact::run(&driver, fixture).await
}

/// Check one credential store including its preseeded recovery index.
///
/// The exact fixture namespace should be disposable. The list fixture must be
/// preseeded through the consumer adapter's native setup API.
pub async fn check_credential_store<S>(
    store: &S,
    exact_fixture: &ExactStoreFixture<S::Scope, S::Key, S::Value>,
    list_fixture: &ListStoreFixture<S::Scope, S::IndexEntry>,
) -> Result<StorageConformanceReport, ConformanceFailure>
where
    S: CredentialStore + ?Sized,
    S::Value: Clone + PartialEq,
    S::IndexEntry: PartialEq,
{
    let driver = CredentialDriver(store);
    Ok(exact::run(&driver, exact_fixture)
        .await?
        .combine(list::run(&driver, list_fixture).await?))
}

/// Check one DID store including its preseeded recovery index.
///
/// The exact fixture namespace should be disposable. The list fixture must be
/// preseeded through the consumer adapter's native setup API.
pub async fn check_did_store<S>(
    store: &S,
    exact_fixture: &ExactStoreFixture<S::Scope, S::Key, S::Value>,
    list_fixture: &ListStoreFixture<S::Scope, S::IndexEntry>,
) -> Result<StorageConformanceReport, ConformanceFailure>
where
    S: DidStore + ?Sized,
    S::Value: Clone + PartialEq,
    S::IndexEntry: PartialEq,
{
    let driver = DidDriver(store);
    Ok(exact::run(&driver, exact_fixture)
        .await?
        .combine(list::run(&driver, list_fixture).await?))
}

/// Check one protocol-state store including its preseeded recovery index.
///
/// The exact fixture namespace should be disposable. The list fixture must be
/// preseeded through the consumer adapter's native setup API.
pub async fn check_protocol_state_store<S>(
    store: &S,
    exact_fixture: &ExactStoreFixture<S::Scope, S::Key, S::Value>,
    list_fixture: &ListStoreFixture<S::Scope, S::IndexEntry>,
) -> Result<StorageConformanceReport, ConformanceFailure>
where
    S: ProtocolStateStore + ?Sized,
    S::Value: Clone + PartialEq,
    S::IndexEntry: PartialEq,
{
    let driver = ProtocolStateDriver(store);
    Ok(exact::run(&driver, exact_fixture)
        .await?
        .combine(list::run(&driver, list_fixture).await?))
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
    exact::run(&driver, fixture).await
}

#[cfg(test)]
mod tests;
