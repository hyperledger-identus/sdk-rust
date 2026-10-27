//! Redaction-safe PRISM VDR failures.

use std::fmt;

use identus_core::{CapabilityId, ErrorCode, ErrorKind as CoreErrorKind, IdentusError};

const CAPABILITY: CapabilityId = CapabilityId::new("did-prism-vdr");
const CODE: ErrorCode = ErrorCode::new("did-prism-vdr.invalid_operation");
const LIMIT_CODE: ErrorCode = ErrorCode::new("did-prism-vdr.resource_limit");
const AUTH_CODE: ErrorCode = ErrorCode::new("did-prism-vdr.unauthorized");
const NOT_FOUND_CODE: ErrorCode = ErrorCode::new("did-prism-vdr.missing_parent");
const CONFLICT_CODE: ErrorCode = ErrorCode::new("did-prism-vdr.state_conflict");
const LOOKUP_CODE: ErrorCode = ErrorCode::new("did-prism-vdr.lookup_failed");

/// Stable non-sensitive VDR failure category.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    /// The operation kind or protobuf payload is malformed.
    InvalidWire,
    /// Required payload data is absent or invalid.
    InvalidOperation,
    /// A finite byte, depth, or work limit was exceeded.
    ResourceLimitExceeded,
    /// The owner DID state or VDR signature is invalid.
    Unauthorized,
    /// A predecessor cannot be resolved to a root.
    MissingParent,
    /// An ancestry cycle was detected.
    Cycle,
    /// The predecessor is not the current entry head.
    StalePredecessor,
    /// The entry is terminal after deactivation.
    Deactivated,
    /// The caller-provided lookup failed.
    LookupFailed,
}

/// A redacted lookup-port failure.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LookupError;

impl fmt::Display for LookupError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("VDR lookup failed")
    }
}

impl std::error::Error for LookupError {}

/// Rich local VDR failure retaining only a stable category and bounded index.
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

    /// Return the stable failure category.
    #[must_use]
    pub const fn kind(self) -> ErrorKind {
        self.kind
    }

    /// Return the bounded ancestry position when applicable.
    #[must_use]
    pub const fn position(self) -> Option<u16> {
        self.position
    }

    pub(crate) const fn at(kind: ErrorKind, position: u16) -> Self {
        Self {
            kind,
            position: Some(position),
        }
    }

    /// Convert to the SDK-wide redacted error contract.
    pub const fn to_identus_error(self) -> IdentusError {
        let (code, kind, message) = match self.kind {
            ErrorKind::InvalidWire | ErrorKind::InvalidOperation => (
                CODE,
                CoreErrorKind::InvalidInput,
                "invalid PRISM VDR operation",
            ),
            ErrorKind::ResourceLimitExceeded => (
                LIMIT_CODE,
                CoreErrorKind::InvalidInput,
                "PRISM VDR resource limit exceeded",
            ),
            ErrorKind::Unauthorized => (
                AUTH_CODE,
                CoreErrorKind::VerificationFailed,
                "PRISM VDR authorization failed",
            ),
            ErrorKind::MissingParent => (
                NOT_FOUND_CODE,
                CoreErrorKind::NotFound,
                "PRISM VDR predecessor was not found",
            ),
            ErrorKind::Cycle | ErrorKind::StalePredecessor | ErrorKind::Deactivated => (
                CONFLICT_CODE,
                CoreErrorKind::Conflict,
                "PRISM VDR state conflicts with its history",
            ),
            ErrorKind::LookupFailed => (
                LOOKUP_CODE,
                CoreErrorKind::Storage,
                "PRISM VDR lookup failed",
            ),
        };
        IdentusError::public(code, kind, CAPABILITY, message)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self.kind {
            ErrorKind::InvalidWire => "invalid PRISM VDR encoding",
            ErrorKind::InvalidOperation => "invalid PRISM VDR operation",
            ErrorKind::ResourceLimitExceeded => "PRISM VDR resource limit exceeded",
            ErrorKind::Unauthorized => "PRISM VDR operation is unauthorized",
            ErrorKind::MissingParent => "PRISM VDR predecessor is missing",
            ErrorKind::Cycle => "PRISM VDR ancestry contains a cycle",
            ErrorKind::StalePredecessor => "PRISM VDR predecessor is stale",
            ErrorKind::Deactivated => "PRISM VDR entry is deactivated",
            ErrorKind::LookupFailed => "PRISM VDR lookup failed",
        })
    }
}

impl std::error::Error for Error {}
