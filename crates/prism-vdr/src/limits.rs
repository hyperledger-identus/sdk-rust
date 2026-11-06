//! Finite public resource profile for PRISM VDR operations.

/// Maximum opaque byte payload retained by one VDR operation.
pub const MAX_VDR_DATA_BYTES: usize = 64 * 1_024;
/// Maximum nonce bytes in a create operation.
pub const MAX_NONCE_BYTES: usize = 64;
/// Maximum UTF-8 bytes in an IPFS reference.
pub const MAX_IPFS_BYTES: usize = 2_048;
/// Maximum UTF-8 bytes in each status-list text field.
pub const MAX_STATUS_TEXT_BYTES: usize = 4_096;
/// Maximum predecessor hops in one root traversal.
pub const MAX_ANCESTRY_DEPTH: usize = 200;
/// Maximum lookup nodes consumed across one transaction batch.
pub const MAX_LOOKUP_NODES_PER_BATCH: usize = 4_096;
