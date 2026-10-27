//! Bounded, effect-neutral PRISM verifiable-data-registry primitives.
//!
//! The crate owns PRISM storage payload validation, authorization, entry state,
//! and finite root traversal. Ledger sources, persistence, scheduling, and
//! retry policy remain caller-owned.

use identus_core::Component;

mod error;
mod limits;
mod operation;
mod proto;
mod traversal;
mod wire;

pub use error::{Error, ErrorKind, LookupError};
pub use limits::{
    MAX_ANCESTRY_DEPTH, MAX_IPFS_BYTES, MAX_LOOKUP_NODES_PER_BATCH, MAX_NONCE_BYTES,
    MAX_STATUS_TEXT_BYTES, MAX_VDR_DATA_BYTES,
};
pub use operation::{
    CreateVdrEntry, DeactivateVdrEntry, StatusListEntry, UpdateVdrEntry, VdrData, VdrOperation,
};
pub use traversal::{
    EntrySnapshot, ResolvedEntry, TraversalBudget, VdrLookup, VdrNode, resolve_entry,
};

/// Metadata for the `identus-prism-vdr` crate.
pub const COMPONENT: Component = Component {
    name: "identus-prism-vdr",
    summary: "Bounded PRISM VDR payloads, authorization, state, and root traversal.",
};
