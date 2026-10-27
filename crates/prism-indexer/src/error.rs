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
        })
    }
}

impl std::error::Error for Error {}
