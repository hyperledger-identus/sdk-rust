//! Deterministic, effect-neutral PRISM transaction indexing primitives.

use identus_core::Component;

mod batch;
mod error;

pub use batch::{
    Cursor, IncomingRecord, MAX_BATCH_RECORDS, MAX_CURSOR_BYTES, MAX_SIGNED_BYTES_PER_BATCH,
    MAX_TRANSACTION_ID_BYTES, TransactionBatch, TransactionIdentity, TransitionDigest,
};
pub use error::{Error, ErrorKind};

/// Metadata for the `identus-prism-indexer` crate.
pub const COMPONENT: Component = Component {
    name: "identus-prism-indexer",
    summary: "Bounded deterministic PRISM transaction planning and atomic commit contracts.",
};
