//! Foundation crate for the Identus Rust SDK workspace.
//!
//! Owns the shared, redaction-safe error and result contract that every later
//! crate and future language binding depends on. `IdentusError` carries only
//! `&'static str` fields and its `Display` renders only a stable code and a
//! public message, so secret or internal context never enters the value.

pub mod time;
pub mod url;

pub use time::{
    ClockError, DurationMillis, MonotonicClock, MonotonicTimestampMillis, UnixTimestampMillis,
    WallClock,
};
pub use url::{MAX_URL_BYTES, Url, UrlError};

use std::fmt;

/// Stable metadata describing a workspace crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Component {
    /// Stable crate identifier (e.g. `"identus-core"`).
    pub name: &'static str,
    /// One-line human-readable summary.
    pub summary: &'static str,
}

/// Metadata for the `identus-core` crate itself.
pub const COMPONENT: Component = Component {
    name: "identus-core",
    summary: "Shared, redaction-safe error and result contract for the Identus Rust SDK.",
};

/// Identifies the capability that owns an error (e.g. `"did"`, `"crypto"`).
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CapabilityId(&'static str);

impl CapabilityId {
    /// Construct a capability identifier from a stable string.
    pub const fn new(id: &'static str) -> Self {
        Self(id)
    }

    /// The stable identifier string.
    pub const fn as_str(&self) -> &'static str {
        self.0
    }
}

impl fmt::Display for CapabilityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

/// Stable error code used as a compatibility contract for conformance fixtures
/// and language bindings.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ErrorCode(&'static str);

impl ErrorCode {
    /// Construct an error code from a stable string.
    pub const fn new(code: &'static str) -> Self {
        Self(code)
    }

    /// The stable code string.
    pub const fn as_str(&self) -> &'static str {
        self.0
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

/// The families an `IdentusError` may belong to. These are stable contracts:
/// conformance fixtures and bindings match on them, so new families require an
/// explicit compatibility decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    /// Input violated a format or value constraint.
    InvalidInput,
    /// The requested operation or input form is not supported.
    Unsupported,
    /// A referenced resource (DID, document, key) was not found.
    NotFound,
    /// The request conflicts with existing state.
    Conflict,
    /// The request violates an applicable policy or rule.
    PolicyViolation,
    /// A signature, proof, or other verification check failed.
    VerificationFailed,
    /// A transport-level failure (network, remote endpoint).
    Transport,
    /// A storage-layer failure.
    Storage,
    /// A cryptographic operation or key-management failure.
    Crypto,
    /// A trust-establishment or trust-resolution failure.
    Trust,
    /// An internal invariant or unexpected condition.
    Internal,
}

/// The shared, redaction-safe error type for the workspace.
///
/// Carries only `&'static str` fields. Its `Display` renders
/// `"{code}: {public_message}"` and nothing else, so secret or internal
/// context never leaks through the value.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IdentusError {
    code: ErrorCode,
    kind: ErrorKind,
    capability: Option<CapabilityId>,
    public_message: &'static str,
}

impl IdentusError {
    /// Construct a public error attributed to a capability.
    pub const fn public(
        code: ErrorCode,
        kind: ErrorKind,
        capability: CapabilityId,
        public_message: &'static str,
    ) -> Self {
        Self {
            code,
            kind,
            capability: Some(capability),
            public_message,
        }
    }

    /// Construct an internal error with no capability attribution.
    pub const fn internal(code: ErrorCode, public_message: &'static str) -> Self {
        Self {
            code,
            kind: ErrorKind::Internal,
            capability: None,
            public_message,
        }
    }

    /// The stable error code.
    pub const fn code(&self) -> ErrorCode {
        self.code
    }

    /// The error family.
    pub const fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// The owning capability, if any (absent for internal errors).
    pub const fn capability(&self) -> Option<CapabilityId> {
        self.capability
    }

    /// The public, redaction-safe message.
    pub const fn public_message(&self) -> &'static str {
        self.public_message
    }
}

impl fmt::Display for IdentusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.public_message)
    }
}

impl std::error::Error for IdentusError {}

/// Result alias used at public crate boundaries.
pub type IdentusResult<T> = Result<T, IdentusError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_renders_code_and_public_message_only() {
        let err = IdentusError::public(
            ErrorCode::new("bad_input"),
            ErrorKind::InvalidInput,
            CapabilityId::new("did"),
            "unsupported method",
        );
        let rendered = err.to_string();
        assert_eq!(rendered, "bad_input: unsupported method");
        // The capability and kind must not appear as rendered fields.
        assert!(!rendered.contains("did"));
        assert!(!rendered.contains("InvalidInput"));
    }

    #[test]
    fn internal_sets_internal_kind_and_no_capability() {
        let err = IdentusError::internal(ErrorCode::new("unexpected_state"), "invariant violated");
        assert_eq!(err.kind(), ErrorKind::Internal);
        assert_eq!(err.capability(), None);
        assert_eq!(err.code().as_str(), "unexpected_state");
        assert_eq!(err.public_message(), "invariant violated");
    }

    #[test]
    fn capability_attribution_flows_through_public() {
        let err = IdentusError::public(
            ErrorCode::new("missing_key"),
            ErrorKind::NotFound,
            CapabilityId::new("crypto"),
            "key not found",
        );
        let cap = err.capability().expect("public error carries a capability");
        assert_eq!(cap.as_str(), "crypto");
    }

    #[test]
    fn error_code_round_trips_stable_string() {
        let code = ErrorCode::new("invalid_did_method");
        assert_eq!(code.as_str(), "invalid_did_method");
        assert_eq!(code.to_string(), "invalid_did_method");
    }

    #[test]
    fn all_error_kind_families_are_present() {
        let families = [
            ErrorKind::InvalidInput,
            ErrorKind::Unsupported,
            ErrorKind::NotFound,
            ErrorKind::Conflict,
            ErrorKind::PolicyViolation,
            ErrorKind::VerificationFailed,
            ErrorKind::Transport,
            ErrorKind::Storage,
            ErrorKind::Crypto,
            ErrorKind::Trust,
            ErrorKind::Internal,
        ];
        assert_eq!(families.len(), 11);
        // Ensures the variants remain distinct exhaustively when matched.
        for (i, a) in families.iter().enumerate() {
            for (j, b) in families.iter().enumerate() {
                if i == j {
                    assert_eq!(a, b);
                } else {
                    assert_ne!(a, b);
                }
            }
        }
    }

    #[test]
    fn component_self_describes() {
        assert_eq!(COMPONENT.name, "identus-core");
    }
}
