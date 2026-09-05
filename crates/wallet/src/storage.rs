//! Least-authority persistence ports shared by wallet orchestrators.

use core::fmt;
use std::future::Future;
use std::pin::Pin;

use identus_core::{CapabilityId, ErrorCode, ErrorKind, IdentusError};
use identus_derive as identus;

/// Maximum number of bytes in one opaque storage revision.
pub const MAX_STORAGE_REVISION_BYTES: usize = 256;
/// Maximum number of bytes in one opaque pagination cursor.
pub const MAX_STORAGE_CURSOR_BYTES: usize = 1_024;
/// Maximum number of entries in one storage page.
pub const MAX_STORAGE_PAGE_ENTRIES: usize = 256;

const CAPABILITY: CapabilityId = CapabilityId::new("wallet.storage");

/// An adapter-defined revision for one exact stored record.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct StorageRevision(Vec<u8>);

impl StorageRevision {
    /// Validate an opaque adapter revision without interpreting its bytes.
    pub fn new(bytes: Vec<u8>) -> Result<Self, StorageError> {
        if bytes.is_empty() || bytes.len() > MAX_STORAGE_REVISION_BYTES {
            return Err(StorageError::InvalidRevision);
        }
        Ok(Self(bytes))
    }

    /// Borrow the exact adapter-defined bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Recover the exact adapter-defined bytes.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }
}

impl TryFrom<Vec<u8>> for StorageRevision {
    type Error = StorageError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Debug for StorageRevision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("StorageRevision")
            .field("byte_len", &self.0.len())
            .finish()
    }
}

/// An adapter-defined continuation cursor scoped to one list capability.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct StorageCursor(Vec<u8>);

impl StorageCursor {
    /// Validate an opaque cursor without interpreting its bytes.
    pub fn new(bytes: Vec<u8>) -> Result<Self, StorageError> {
        if bytes.is_empty() || bytes.len() > MAX_STORAGE_CURSOR_BYTES {
            return Err(StorageError::InvalidCursor);
        }
        Ok(Self(bytes))
    }

    /// Borrow the exact adapter-defined bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Recover the exact adapter-defined bytes.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }
}

impl TryFrom<Vec<u8>> for StorageCursor {
    type Error = StorageError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Debug for StorageCursor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("StorageCursor")
            .field("byte_len", &self.0.len())
            .finish()
    }
}

/// A validated caller-requested page bound.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StoragePageSize(u16);

impl StoragePageSize {
    /// Construct a size from 1 through [`MAX_STORAGE_PAGE_ENTRIES`].
    pub fn new(size: usize) -> Result<Self, StorageError> {
        if size == 0 || size > MAX_STORAGE_PAGE_ENTRIES {
            return Err(StorageError::InvalidPageSize);
        }
        Ok(Self(size as u16))
    }

    /// Return the page bound as `usize`.
    #[must_use]
    pub const fn get(self) -> usize {
        self.0 as usize
    }
}

impl TryFrom<usize> for StoragePageSize {
    type Error = StorageError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// One bounded list request with an optional opaque continuation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoragePageRequest {
    size: StoragePageSize,
    cursor: Option<StorageCursor>,
}

impl StoragePageRequest {
    /// Construct a request from already validated parts.
    #[must_use]
    pub const fn new(size: StoragePageSize, cursor: Option<StorageCursor>) -> Self {
        Self { size, cursor }
    }

    /// Return the requested maximum number of entries.
    #[must_use]
    pub const fn size(&self) -> StoragePageSize {
        self.size
    }

    /// Borrow the opaque continuation cursor, when present.
    #[must_use]
    pub const fn cursor(&self) -> Option<&StorageCursor> {
        self.cursor.as_ref()
    }

    /// Split the request into its exact parts.
    #[must_use]
    pub fn into_parts(self) -> (StoragePageSize, Option<StorageCursor>) {
        (self.size, self.cursor)
    }
}

/// One bounded page of consumer-owned index entries.
#[derive(Clone, PartialEq, Eq)]
pub struct StoragePage<T> {
    entries: Vec<T>,
    next_cursor: Option<StorageCursor>,
}

impl<T> StoragePage<T> {
    /// Validate a page and its progress invariant.
    pub fn new(entries: Vec<T>, next_cursor: Option<StorageCursor>) -> Result<Self, StorageError> {
        if entries.len() > MAX_STORAGE_PAGE_ENTRIES || (entries.is_empty() && next_cursor.is_some())
        {
            return Err(StorageError::InvalidPage);
        }
        Ok(Self {
            entries,
            next_cursor,
        })
    }

    /// Borrow all entries in this page.
    #[must_use]
    pub fn entries(&self) -> &[T] {
        &self.entries
    }

    /// Return the number of entries in this page.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Return whether this page has no entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Borrow the next opaque cursor, when present.
    #[must_use]
    pub const fn next_cursor(&self) -> Option<&StorageCursor> {
        self.next_cursor.as_ref()
    }

    /// Split the page into its exact entries and continuation.
    #[must_use]
    pub fn into_parts(self) -> (Vec<T>, Option<StorageCursor>) {
        (self.entries, self.next_cursor)
    }
}

impl<T> fmt::Debug for StoragePage<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("StoragePage")
            .field("entry_count", &self.entries.len())
            .field("has_next_cursor", &self.next_cursor.is_some())
            .finish()
    }
}

/// Preconditions for one exact-record write.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum StorageWriteCondition {
    /// Insert or replace regardless of current state.
    Any,
    /// Write only if no record currently exists.
    InsertOnly,
    /// Replace only if the current revision matches exactly.
    IfRevision(StorageRevision),
}

/// Preconditions for one exact-record deletion.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum StorageDeleteCondition {
    /// Delete regardless of current state.
    Any,
    /// Delete only if the current revision matches exactly.
    IfRevision(StorageRevision),
}

/// An owned value and its exact-record write precondition.
pub struct StorageWrite<T> {
    value: T,
    condition: StorageWriteCondition,
}

impl<T> StorageWrite<T> {
    /// Construct a conditional write.
    #[must_use]
    pub const fn new(value: T, condition: StorageWriteCondition) -> Self {
        Self { value, condition }
    }

    /// Borrow the write precondition.
    #[must_use]
    pub const fn condition(&self) -> &StorageWriteCondition {
        &self.condition
    }

    /// Split the write into its owned value and precondition.
    #[must_use]
    pub fn into_parts(self) -> (T, StorageWriteCondition) {
        (self.value, self.condition)
    }
}

impl<T> fmt::Debug for StorageWrite<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("StorageWrite")
            .field("condition", &self.condition)
            .finish_non_exhaustive()
    }
}

/// An owned stored value paired with its current opaque revision.
pub struct Stored<T> {
    value: T,
    revision: StorageRevision,
}

impl<T> Stored<T> {
    /// Construct a stored value from adapter output.
    #[must_use]
    pub const fn new(value: T, revision: StorageRevision) -> Self {
        Self { value, revision }
    }

    /// Borrow the consumer-owned value.
    #[must_use]
    pub const fn value(&self) -> &T {
        &self.value
    }

    /// Borrow the current opaque revision.
    #[must_use]
    pub const fn revision(&self) -> &StorageRevision {
        &self.revision
    }

    /// Split the stored value into its exact parts.
    #[must_use]
    pub fn into_parts(self) -> (T, StorageRevision) {
        (self.value, self.revision)
    }
}

impl<T> fmt::Debug for Stored<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Stored")
            .field("revision_byte_len", &self.revision.as_bytes().len())
            .finish_non_exhaustive()
    }
}

/// Whether a successful write inserted or replaced one record.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum StorageWriteOutcome {
    /// No prior record existed.
    Inserted,
    /// A prior record was replaced.
    Replaced,
}

/// Evidence returned after one successful exact-record write.
#[derive(Clone, PartialEq, Eq)]
pub struct StorageWriteReceipt {
    outcome: StorageWriteOutcome,
    revision: StorageRevision,
}

impl StorageWriteReceipt {
    /// Construct a successful write receipt.
    #[must_use]
    pub const fn new(outcome: StorageWriteOutcome, revision: StorageRevision) -> Self {
        Self { outcome, revision }
    }

    /// Return whether the operation inserted or replaced a record.
    #[must_use]
    pub const fn outcome(&self) -> StorageWriteOutcome {
        self.outcome
    }

    /// Borrow the newly assigned opaque revision.
    #[must_use]
    pub const fn revision(&self) -> &StorageRevision {
        &self.revision
    }

    /// Split the receipt into its exact parts.
    #[must_use]
    pub fn into_parts(self) -> (StorageWriteOutcome, StorageRevision) {
        (self.outcome, self.revision)
    }
}

impl fmt::Debug for StorageWriteReceipt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("StorageWriteReceipt")
            .field("outcome", &self.outcome)
            .field("revision_byte_len", &self.revision.as_bytes().len())
            .finish()
    }
}

/// Result of one unconditional or revision-matched delete.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum StorageDeleteOutcome {
    /// A record was deleted.
    Deleted,
    /// No record existed for an unconditional delete.
    NotFound,
}

/// Redaction-safe failures shared by all wallet storage capabilities.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum StorageError {
    /// A revision was empty or exceeded its bound.
    InvalidRevision,
    /// A cursor was empty or exceeded its bound.
    InvalidCursor,
    /// A requested page size was outside its bound.
    InvalidPageSize,
    /// A returned page violated its size or progress invariant.
    InvalidPage,
    /// A single-record write or delete precondition was false.
    Conflict,
    /// The backing store cannot accept more data.
    CapacityExceeded,
    /// Stored data failed an adapter integrity check.
    Integrity,
    /// The backing store denied access.
    AccessDenied,
    /// The backing store is temporarily unavailable.
    Unavailable,
    /// The backing store failed unexpectedly.
    Internal,
}

impl StorageError {
    /// Bridge to the shared static, redaction-safe SDK error surface.
    pub const fn to_identus_error(self) -> IdentusError {
        let (code, kind, message) = match self {
            Self::InvalidRevision => (
                "wallet.storage_invalid_revision",
                ErrorKind::InvalidInput,
                "storage revision is invalid",
            ),
            Self::InvalidCursor => (
                "wallet.storage_invalid_cursor",
                ErrorKind::InvalidInput,
                "storage cursor is invalid",
            ),
            Self::InvalidPageSize => (
                "wallet.storage_invalid_page_size",
                ErrorKind::InvalidInput,
                "storage page size is invalid",
            ),
            Self::InvalidPage => (
                "wallet.storage_invalid_page",
                ErrorKind::InvalidInput,
                "storage page is invalid",
            ),
            Self::Conflict => (
                "wallet.storage_conflict",
                ErrorKind::Conflict,
                "storage precondition failed",
            ),
            Self::CapacityExceeded => (
                "wallet.storage_capacity_exceeded",
                ErrorKind::Storage,
                "storage capacity was exceeded",
            ),
            Self::Integrity => (
                "wallet.storage_integrity",
                ErrorKind::Storage,
                "stored data failed an integrity check",
            ),
            Self::AccessDenied => (
                "wallet.storage_access_denied",
                ErrorKind::PolicyViolation,
                "storage access was denied",
            ),
            Self::Unavailable => (
                "wallet.storage_unavailable",
                ErrorKind::Storage,
                "storage is unavailable",
            ),
            Self::Internal => (
                "wallet.storage_internal",
                ErrorKind::Internal,
                "storage failed internally",
            ),
        };
        IdentusError::public(ErrorCode::new(code), kind, CAPABILITY, message)
    }
}

impl From<StorageError> for IdentusError {
    fn from(value: StorageError) -> Self {
        value.to_identus_error()
    }
}

impl fmt::Display for StorageError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidRevision => "storage revision is invalid",
            Self::InvalidCursor => "storage cursor is invalid",
            Self::InvalidPageSize => "storage page size is invalid",
            Self::InvalidPage => "storage page is invalid",
            Self::Conflict => "storage precondition failed",
            Self::CapacityExceeded => "storage capacity was exceeded",
            Self::Integrity => "stored data failed an integrity check",
            Self::AccessDenied => "storage access was denied",
            Self::Unavailable => "storage is unavailable",
            Self::Internal => "storage failed internally",
        })
    }
}

impl std::error::Error for StorageError {}

/// Type-erased asynchronous result returned by wallet storage capabilities.
pub type StorageFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T, StorageError>> + Send + 'a>>;

/// Exact-key secret persistence without enumeration or custody policy.
#[identus::port]
pub trait SecretStore: Send + Sync {
    /// Consumer-owned storage namespace type.
    type Scope: Send + Sync;
    /// Consumer-owned exact record key type.
    type Key: Send + Sync;
    /// Consumer-owned stored value type.
    type Value: Send + Sync;

    /// Load one exact record and its current revision.
    fn load<'a>(
        &'a self,
        scope: &'a Self::Scope,
        key: &'a Self::Key,
    ) -> StorageFuture<'a, Option<Stored<Self::Value>>>;

    /// Insert or replace one exact record under an explicit precondition.
    fn write<'a>(
        &'a self,
        scope: &'a Self::Scope,
        key: &'a Self::Key,
        write: StorageWrite<Self::Value>,
    ) -> StorageFuture<'a, StorageWriteReceipt>;

    /// Delete one exact record under an explicit precondition.
    fn delete<'a>(
        &'a self,
        scope: &'a Self::Scope,
        key: &'a Self::Key,
        condition: StorageDeleteCondition,
    ) -> StorageFuture<'a, StorageDeleteOutcome>;
}

/// Credential persistence with a bounded, consumer-defined recovery index.
#[identus::port]
pub trait CredentialStore: Send + Sync {
    /// Consumer-owned storage namespace type.
    type Scope: Send + Sync;
    /// Consumer-owned exact record key type.
    type Key: Send + Sync;
    /// Consumer-owned stored value type.
    type Value: Send + Sync;
    /// Consumer-owned recovery-index entry type.
    type IndexEntry: Send + Sync;

    /// Load one exact record and its current revision.
    fn load<'a>(
        &'a self,
        scope: &'a Self::Scope,
        key: &'a Self::Key,
    ) -> StorageFuture<'a, Option<Stored<Self::Value>>>;

    /// Insert or replace one exact record under an explicit precondition.
    fn write<'a>(
        &'a self,
        scope: &'a Self::Scope,
        key: &'a Self::Key,
        write: StorageWrite<Self::Value>,
    ) -> StorageFuture<'a, StorageWriteReceipt>;

    /// Delete one exact record under an explicit precondition.
    fn delete<'a>(
        &'a self,
        scope: &'a Self::Scope,
        key: &'a Self::Key,
        condition: StorageDeleteCondition,
    ) -> StorageFuture<'a, StorageDeleteOutcome>;

    /// List a bounded page of recovery-index entries within one scope.
    fn list<'a>(
        &'a self,
        scope: &'a Self::Scope,
        request: StoragePageRequest,
    ) -> StorageFuture<'a, StoragePage<Self::IndexEntry>>;
}

/// DID persistence with a bounded, consumer-defined recovery index.
#[identus::port]
pub trait DidStore: Send + Sync {
    /// Consumer-owned storage namespace type.
    type Scope: Send + Sync;
    /// Consumer-owned exact record key type.
    type Key: Send + Sync;
    /// Consumer-owned stored value type.
    type Value: Send + Sync;
    /// Consumer-owned recovery-index entry type.
    type IndexEntry: Send + Sync;

    /// Load one exact record and its current revision.
    fn load<'a>(
        &'a self,
        scope: &'a Self::Scope,
        key: &'a Self::Key,
    ) -> StorageFuture<'a, Option<Stored<Self::Value>>>;

    /// Insert or replace one exact record under an explicit precondition.
    fn write<'a>(
        &'a self,
        scope: &'a Self::Scope,
        key: &'a Self::Key,
        write: StorageWrite<Self::Value>,
    ) -> StorageFuture<'a, StorageWriteReceipt>;

    /// Delete one exact record under an explicit precondition.
    fn delete<'a>(
        &'a self,
        scope: &'a Self::Scope,
        key: &'a Self::Key,
        condition: StorageDeleteCondition,
    ) -> StorageFuture<'a, StorageDeleteOutcome>;

    /// List a bounded page of recovery-index entries within one scope.
    fn list<'a>(
        &'a self,
        scope: &'a Self::Scope,
        request: StoragePageRequest,
    ) -> StorageFuture<'a, StoragePage<Self::IndexEntry>>;
}

/// Protocol-state persistence with a bounded, consumer-defined recovery index.
#[identus::port]
pub trait ProtocolStateStore: Send + Sync {
    /// Consumer-owned storage namespace type.
    type Scope: Send + Sync;
    /// Consumer-owned exact record key type.
    type Key: Send + Sync;
    /// Consumer-owned stored value type.
    type Value: Send + Sync;
    /// Consumer-owned recovery-index entry type.
    type IndexEntry: Send + Sync;

    /// Load one exact record and its current revision.
    fn load<'a>(
        &'a self,
        scope: &'a Self::Scope,
        key: &'a Self::Key,
    ) -> StorageFuture<'a, Option<Stored<Self::Value>>>;

    /// Insert or replace one exact record under an explicit precondition.
    fn write<'a>(
        &'a self,
        scope: &'a Self::Scope,
        key: &'a Self::Key,
        write: StorageWrite<Self::Value>,
    ) -> StorageFuture<'a, StorageWriteReceipt>;

    /// Delete one exact record under an explicit precondition.
    fn delete<'a>(
        &'a self,
        scope: &'a Self::Scope,
        key: &'a Self::Key,
        condition: StorageDeleteCondition,
    ) -> StorageFuture<'a, StorageDeleteOutcome>;

    /// List a bounded page of recovery-index entries within one scope.
    fn list<'a>(
        &'a self,
        scope: &'a Self::Scope,
        request: StoragePageRequest,
    ) -> StorageFuture<'a, StoragePage<Self::IndexEntry>>;
}

/// Exact-key status-cache persistence without expiry or refresh policy.
#[identus::port]
pub trait StatusCacheStore: Send + Sync {
    /// Consumer-owned storage namespace type.
    type Scope: Send + Sync;
    /// Consumer-owned exact record key type.
    type Key: Send + Sync;
    /// Consumer-owned stored value type.
    type Value: Send + Sync;

    /// Load one exact record and its current revision.
    fn load<'a>(
        &'a self,
        scope: &'a Self::Scope,
        key: &'a Self::Key,
    ) -> StorageFuture<'a, Option<Stored<Self::Value>>>;

    /// Insert or replace one exact record under an explicit precondition.
    fn write<'a>(
        &'a self,
        scope: &'a Self::Scope,
        key: &'a Self::Key,
        write: StorageWrite<Self::Value>,
    ) -> StorageFuture<'a, StorageWriteReceipt>;

    /// Delete one exact record under an explicit precondition.
    fn delete<'a>(
        &'a self,
        scope: &'a Self::Scope,
        key: &'a Self::Key,
        condition: StorageDeleteCondition,
    ) -> StorageFuture<'a, StorageDeleteOutcome>;
}
