//! Deterministic, effect-neutral PRISM transaction indexing primitives.

use identus_core::Component;

mod batch;
mod error;
mod planner;
mod store;

pub use batch::{
    Cursor, IncomingRecord, MAX_BATCH_RECORDS, MAX_CURSOR_BYTES, MAX_SIGNED_BYTES_PER_BATCH,
    MAX_TRANSACTION_ID_BYTES, TransactionBatch, TransactionIdentity, TransitionDigest,
};
pub use error::{Error, ErrorKind};
pub use planner::{DidStateLookup, Mutation, PlannedTransaction, plan_transaction};
pub use store::{
    AtomicCommit, CommitOutcome, CommitRequest, CursorLookup, IndexOutcome, StoreError,
    TransitionReceiptLookup, index_transaction,
};

/// Metadata for the `identus-prism-indexer` crate.
pub const COMPONENT: Component = Component {
    name: "identus-prism-indexer",
    summary: "Bounded deterministic PRISM transaction planning and atomic commit contracts.",
};
