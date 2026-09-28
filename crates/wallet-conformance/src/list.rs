use core::fmt;

use identus_wallet::{StorageCursor, StorageFuture, StoragePageRequest, StoragePageSize};

use crate::{
    ConformanceFailure, ConformanceFailureKind, ConformanceFixtureError,
    MAX_CONFORMANCE_INDEX_ENTRIES, StorageConformanceReport, exact::ExactDriver, failure,
};

/// A preseeded recovery-index fixture for a list-capable storage adapter.
pub struct ListStoreFixture<Scope, Entry> {
    pub(super) scope: Scope,
    pub(super) expected_entries: Vec<Entry>,
    pub(super) page_size: StoragePageSize,
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

pub(super) trait ListDriver: ExactDriver {
    type IndexEntry: Send + Sync;

    fn list<'a>(
        &'a self,
        scope: &'a Self::Scope,
        request: StoragePageRequest,
    ) -> StorageFuture<'a, identus_wallet::StoragePage<Self::IndexEntry>>;
}

pub(super) async fn run<D>(
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

    Ok(StorageConformanceReport::new(operations, observed.len()))
}
