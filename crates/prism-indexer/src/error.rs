//! Redaction-safe PRISM indexer failures.

use std::fmt;

/// Stable non-sensitive indexer failure category.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    /// A bounded identifier or cursor is empty or oversized.
    InvalidBoundaryValue,
    /// Declared count, identity, or record indices are inconsistent.
    IncompleteTransaction,
    /// Record or aggregate byte capacity was exceeded.
    ResourceLimitExceeded,
    /// A signed PRISM operation is unsupported, malformed, or invalid.
    InvalidOperation,
    /// Required committed DID or VDR state is absent.
    MissingState,
    /// A caller-provided read port failed.
    LookupFailed,
    /// The same transaction identity or cursor conflicts with committed state.
    Conflict,
    /// The caller-provided atomic commit port failed.
    CommitFailed,
}

/// A redacted indexer construction failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Error {
    kind: ErrorKind,
    position: Option<u16>,
}

impl Error {
    pub(crate) const fn new(kind: ErrorKind) -> Self {
        Self {
            kind,
            position: None,
        }
    }

    pub(crate) const fn at(kind: ErrorKind, position: u16) -> Self {
        Self {
            kind,
            position: Some(position),
        }
    }

    /// Return the stable failure category.
    #[must_use]
    pub const fn kind(self) -> ErrorKind {
        self.kind
    }

    /// Return the bounded record position when applicable.
    #[must_use]
    pub const fn position(self) -> Option<u16> {
        self.position
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self.kind {
            ErrorKind::InvalidBoundaryValue => "invalid PRISM indexer boundary value",
            ErrorKind::IncompleteTransaction => "incomplete PRISM transaction batch",
            ErrorKind::ResourceLimitExceeded => "PRISM indexer resource limit exceeded",
            ErrorKind::InvalidOperation => "invalid PRISM operation in transaction batch",
            ErrorKind::MissingState => "required PRISM state is missing",
            ErrorKind::LookupFailed => "PRISM indexer lookup failed",
            ErrorKind::Conflict => "PRISM indexer transition conflicts with committed state",
            ErrorKind::CommitFailed => "PRISM indexer commit failed",
        })
    }
}

impl std::error::Error for Error {}
