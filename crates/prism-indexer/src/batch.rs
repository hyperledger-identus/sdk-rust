//! Complete bounded transaction batches and canonical replay digests.

use std::{fmt, sync::Arc};

use identus_crypto::Sha256Digest;
use identus_prism_did::SignedPrismOperation;
use sha2::{Digest, Sha256};

use crate::{Error, ErrorKind};

/// Maximum opaque transaction-identity bytes.
pub const MAX_TRANSACTION_ID_BYTES: usize = 256;
/// Maximum encoded bytes in each cursor.
pub const MAX_CURSOR_BYTES: usize = 1_024;
/// Maximum records in one complete transaction.
pub const MAX_BATCH_RECORDS: usize = 1_024;
/// Maximum aggregate exact signed-operation bytes in one batch.
pub const MAX_SIGNED_BYTES_PER_BATCH: usize = 8 * 1_024 * 1_024;

const TRANSITION_DOMAIN: &[u8] = b"identus.prism-indexer.transition.v1\0";

/// A bounded opaque source transaction identity.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TransactionIdentity(Arc<[u8]>);

impl TransactionIdentity {
    /// Validate a non-empty opaque transaction identity before retaining it.
    pub fn new(bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
        let bytes = bytes.as_ref();
        if bytes.is_empty() || bytes.len() > MAX_TRANSACTION_ID_BYTES {
            return Err(Error::new(ErrorKind::InvalidBoundaryValue));
        }
        Ok(Self(Arc::from(bytes)))
    }

    /// Borrow exact identity bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Debug for TransactionIdentity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TransactionIdentity")
            .field("byte_len", &self.0.len())
            .finish()
    }
}

/// A bounded opaque expected or next cursor.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Cursor(Arc<[u8]>);

impl Cursor {
    /// Validate a non-empty encoded cursor before retaining it.
    pub fn new(bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
        let bytes = bytes.as_ref();
        if bytes.is_empty() || bytes.len() > MAX_CURSOR_BYTES {
            return Err(Error::new(ErrorKind::InvalidBoundaryValue));
        }
        Ok(Self(Arc::from(bytes)))
    }

    /// Borrow exact encoded cursor bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Debug for Cursor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Cursor")
            .field("byte_len", &self.0.len())
            .finish()
    }
}

/// One source record before complete-transaction validation.
#[derive(Clone)]
pub struct IncomingRecord {
    /// Source transaction identity repeated for boundary verification.
    pub transaction_identity: TransactionIdentity,
    /// Zero-based operation index inside the source transaction.
    pub operation_index: u16,
    /// Exact bounded signed PRISM operation.
    pub signed_operation: SignedPrismOperation,
}

impl fmt::Debug for IncomingRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("IncomingRecord")
            .field("operation_index", &self.operation_index)
            .field("signed_operation", &self.signed_operation)
            .finish_non_exhaustive()
    }
}

/// A complete structurally validated source transaction.
#[derive(Clone)]
pub struct TransactionBatch {
    identity: TransactionIdentity,
    expected_cursor: Cursor,
    next_cursor: Cursor,
    records: Vec<IncomingRecord>,
    digest: TransitionDigest,
}

impl TransactionBatch {
    /// Validate a complete transaction and compute its canonical replay digest.
    pub fn new(
        identity: TransactionIdentity,
        declared_operation_count: u16,
        expected_cursor: Cursor,
        next_cursor: Cursor,
        records: Vec<IncomingRecord>,
    ) -> Result<Self, Error> {
        let declared = usize::from(declared_operation_count);
        if declared == 0 || declared > MAX_BATCH_RECORDS || records.len() != declared {
            return Err(Error::new(if declared > MAX_BATCH_RECORDS {
                ErrorKind::ResourceLimitExceeded
            } else {
                ErrorKind::IncompleteTransaction
            }));
        }
        let mut signed_bytes = 0_usize;
        for (position, record) in records.iter().enumerate() {
            let position = u16::try_from(position).expect("batch maximum fits u16");
            if record.transaction_identity != identity || record.operation_index != position {
                return Err(Error::at(ErrorKind::IncompleteTransaction, position));
            }
            signed_bytes = signed_bytes
                .checked_add(record.signed_operation.as_bytes().len())
                .ok_or_else(|| Error::at(ErrorKind::ResourceLimitExceeded, position))?;
            if signed_bytes > MAX_SIGNED_BYTES_PER_BATCH {
                return Err(Error::at(ErrorKind::ResourceLimitExceeded, position));
            }
        }
        let digest = transition_digest(
            &identity,
            &expected_cursor,
            &next_cursor,
            declared_operation_count,
            &records,
        );
        Ok(Self {
            identity,
            expected_cursor,
            next_cursor,
            records,
            digest,
        })
    }

    /// Borrow the transaction identity.
    #[must_use]
    pub const fn identity(&self) -> &TransactionIdentity {
        &self.identity
    }

    /// Borrow the expected cursor.
    #[must_use]
    pub const fn expected_cursor(&self) -> &Cursor {
        &self.expected_cursor
    }

    /// Borrow the next cursor.
    #[must_use]
    pub const fn next_cursor(&self) -> &Cursor {
        &self.next_cursor
    }

    /// Borrow records in authoritative source order.
    #[must_use]
    pub fn records(&self) -> &[IncomingRecord] {
        &self.records
    }

    /// Return the canonical transition digest.
    pub const fn transition_digest(&self) -> TransitionDigest {
        self.digest
    }
}

impl fmt::Debug for TransactionBatch {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TransactionBatch")
            .field("records", &self.records.len())
            .finish_non_exhaustive()
    }
}

/// SHA-256 of the versioned tagged transition transcript.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TransitionDigest(Sha256Digest);

impl TransitionDigest {
    /// Borrow the fixed digest bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8; 32] {
        self.0.as_array()
    }
}

impl fmt::Debug for TransitionDigest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("TransitionDigest([REDACTED])")
    }
}

fn transition_digest(
    identity: &TransactionIdentity,
    expected: &Cursor,
    next: &Cursor,
    declared: u16,
    records: &[IncomingRecord],
) -> TransitionDigest {
    let mut hasher = Sha256::new();
    hasher.update(TRANSITION_DOMAIN);
    frame(&mut hasher, 1, identity.as_bytes());
    frame(&mut hasher, 2, expected.as_bytes());
    frame(&mut hasher, 3, next.as_bytes());
    frame(&mut hasher, 4, &u32::from(declared).to_be_bytes());
    for record in records {
        let index = record.operation_index.to_be_bytes();
        let signed = record.signed_operation.as_bytes();
        let nested_len = framed_len(index.len()) + framed_len(signed.len());
        hasher.update([5]);
        hasher.update(
            u64::try_from(nested_len)
                .expect("bounded transcript length")
                .to_be_bytes(),
        );
        frame(&mut hasher, 6, &index);
        frame(&mut hasher, 7, signed);
    }
    let bytes: [u8; 32] = hasher.finalize().into();
    TransitionDigest(Sha256Digest::from_bytes(&bytes).expect("SHA-256 output has fixed length"))
}

fn frame(hasher: &mut Sha256, tag: u8, bytes: &[u8]) {
    hasher.update([tag]);
    hasher.update(
        u64::try_from(bytes.len())
            .expect("bounded transcript length")
            .to_be_bytes(),
    );
    hasher.update(bytes);
}

const fn framed_len(value_len: usize) -> usize {
    1 + 8 + value_len
}
