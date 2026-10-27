//! Replay probing and one-call atomic commit orchestration.

use std::fmt;

use crate::{
    Cursor, DidStateLookup, Error, ErrorKind, Mutation, TransactionBatch, TransactionIdentity,
    TransitionDigest, plan_transaction,
};
use identus_prism_vdr::VdrLookup;

/// Redacted store-port failure.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StoreError;

impl fmt::Display for StoreError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("PRISM index store failed")
    }
}

impl std::error::Error for StoreError {}

/// Read-only immutable transition receipt lookup.
pub trait TransitionReceiptLookup {
    /// Return the digest committed under one transaction identity.
    fn transition_receipt(
        &self,
        identity: &TransactionIdentity,
    ) -> Result<Option<TransitionDigest>, StoreError>;
}

/// Complete request supplied to a caller-owned atomic commit boundary.
pub struct CommitRequest<'a> {
    /// Unique source transaction identity.
    pub identity: &'a TransactionIdentity,
    /// Canonical exact-content digest.
    pub digest: TransitionDigest,
    /// Cursor expected before mutation visibility.
    pub expected_cursor: &'a Cursor,
    /// Cursor visible with the mutation set.
    pub next_cursor: &'a Cursor,
    /// Complete ordered mutation set.
    pub mutations: &'a [Mutation],
}

impl fmt::Debug for CommitRequest<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CommitRequest")
            .field("mutations", &self.mutations.len())
            .finish_non_exhaustive()
    }
}

/// Atomic adapter result after its identity-first and cursor checks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum CommitOutcome {
    /// Mutations and next cursor became visible atomically.
    Applied,
    /// The same identity and digest were already committed.
    ExactReplay,
    /// Identity content or expected cursor conflicted.
    Conflict,
}

/// Caller-owned all-or-nothing commit port.
pub trait AtomicCommit {
    /// Apply exactly one complete request atomically.
    fn commit(&self, request: CommitRequest<'_>) -> Result<CommitOutcome, StoreError>;
}

/// Public orchestration outcome.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IndexOutcome {
    /// A new transition was committed.
    Applied,
    /// An exact prior transition was recognized idempotently.
    ExactReplay,
}

/// Probe replay before mutable-state reads, otherwise plan and commit once.
pub fn index_transaction<S>(batch: &TransactionBatch, store: &S) -> Result<IndexOutcome, Error>
where
    S: TransitionReceiptLookup + DidStateLookup + VdrLookup + AtomicCommit,
{
    if let Some(committed) = store
        .transition_receipt(batch.identity())
        .map_err(|_| Error::new(ErrorKind::LookupFailed))?
    {
        return if committed == batch.transition_digest() {
            Ok(IndexOutcome::ExactReplay)
        } else {
            Err(Error::new(ErrorKind::Conflict))
        };
    }
    let plan = plan_transaction(batch, store)?;
    let request = CommitRequest {
        identity: batch.identity(),
        digest: batch.transition_digest(),
        expected_cursor: batch.expected_cursor(),
        next_cursor: batch.next_cursor(),
        mutations: plan.mutations(),
    };
    match store
        .commit(request)
        .map_err(|_| Error::new(ErrorKind::CommitFailed))?
    {
        CommitOutcome::Applied => Ok(IndexOutcome::Applied),
        CommitOutcome::ExactReplay => Ok(IndexOutcome::ExactReplay),
        CommitOutcome::Conflict => Err(Error::new(ErrorKind::Conflict)),
    }
}
