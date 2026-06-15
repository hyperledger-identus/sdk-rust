//! Shared primitives for the Identus Rust SDK workspace.

use std::error::Error;
use std::fmt::{Display, Formatter};

/// Compile-time metadata for a workspace component.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Component {
    /// Stable component name.
    pub name: &'static str,
    /// Short statement of responsibility.
    pub summary: &'static str,
}

/// Core component metadata.
pub const COMPONENT: Component = Component {
    name: "identus-core",
    summary: "Shared DTO, error, fixture, and observability primitives.",
};

/// Stable capability identifier used by errors, fixtures, and binding DTOs.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CapabilityId(&'static str);

impl CapabilityId {
    /// Create a capability identifier.
    #[must_use]
    pub const fn new(value: &'static str) -> Self {
        Self(value)
    }

    /// Borrow the stable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

/// Stable typed error code that can be carried across Rust and wrappers.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ErrorCode(&'static str);

impl ErrorCode {
    /// Create a typed error code.
    #[must_use]
    pub const fn new(value: &'static str) -> Self {
        Self(value)
    }

    /// Borrow the stable code.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

/// High-level error family used for policy decisions and binding mapping.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    /// Input syntax, shape, or semantic validation failed.
    InvalidInput,
    /// Requested method, format, algorithm, protocol, or feature is not
    /// supported by the current crate configuration.
    Unsupported,
    /// Requested record, DID, message, credential, or capability was not found.
    NotFound,
    /// State transition or storage write conflicted with current state.
    Conflict,
    /// A policy decision rejected the requested operation.
    PolicyViolation,
    /// Verification failed for cryptographic, credential, presentation, trust,
    /// or protocol reasons.
    VerificationFailed,
    /// Transport operation failed outside core semantics.
    Transport,
    /// Storage operation failed outside core semantics.
    Storage,
    /// Cryptographic operation failed.
    Crypto,
    /// Trust, status, federation, or attestation evaluation failed.
    Trust,
    /// Unexpected internal invariant failure.
    Internal,
}

/// Redaction policy for diagnostics that may cross process or language
/// boundaries.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RedactionPolicy {
    /// Safe for public logs and binding DTOs.
    Public,
    /// Contains no secret material, but should stay in local diagnostics.
    Internal,
    /// Contains sensitive values and must not be rendered by default.
    Secret,
}

/// Redaction-safe SDK error.
///
/// `Display` intentionally renders only the stable code and public message.
/// Secret or internal context belongs in adapter-local logs, not in this core
/// value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentusError {
    code: ErrorCode,
    kind: ErrorKind,
    capability: Option<CapabilityId>,
    public_message: &'static str,
    redaction: RedactionPolicy,
}

impl IdentusError {
    /// Create a public, redaction-safe error.
    #[must_use]
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
            redaction: RedactionPolicy::Public,
        }
    }

    /// Create an internal redaction-safe error without a capability id.
    #[must_use]
    pub const fn internal(code: ErrorCode, public_message: &'static str) -> Self {
        Self {
            code,
            kind: ErrorKind::Internal,
            capability: None,
            public_message,
            redaction: RedactionPolicy::Internal,
        }
    }

    /// Stable typed error code.
    #[must_use]
    pub const fn code(&self) -> ErrorCode {
        self.code
    }

    /// High-level error family.
    #[must_use]
    pub const fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// Capability that produced the error, when available.
    #[must_use]
    pub const fn capability(&self) -> Option<CapabilityId> {
        self.capability
    }

    /// Redaction-safe public message.
    #[must_use]
    pub const fn public_message(&self) -> &'static str {
        self.public_message
    }

    /// Redaction policy for this error.
    #[must_use]
    pub const fn redaction(&self) -> RedactionPolicy {
        self.redaction
    }
}

impl Display for IdentusError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}: {}", self.code.as_str(), self.public_message)
    }
}

impl Error for IdentusError {}

/// SDK result type used by core-facing APIs.
pub type IdentusResult<T> = Result<T, IdentusError>;

/// Binding-friendly result envelope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ResultEnvelope<T> {
    /// Successful result payload.
    Ok(T),
    /// Redaction-safe error payload.
    Err(ErrorEnvelope),
}

impl<T> ResultEnvelope<T> {
    /// Convert a Rust result into a binding-friendly envelope.
    #[must_use]
    pub fn from_result(result: IdentusResult<T>) -> Self {
        match result {
            Ok(value) => Self::Ok(value),
            Err(error) => Self::Err(ErrorEnvelope::from_error(&error)),
        }
    }
}

/// Binding-facing error DTO with stable strings only.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ErrorEnvelope {
    /// Stable typed error code.
    pub code: &'static str,
    /// Stable error family.
    pub kind: ErrorKind,
    /// Capability id, when available.
    pub capability: Option<&'static str>,
    /// Redaction-safe public message.
    pub message: &'static str,
}

impl ErrorEnvelope {
    /// Convert a core error into a binding-safe DTO.
    #[must_use]
    pub fn from_error(error: &IdentusError) -> Self {
        Self {
            code: error.code().as_str(),
            kind: error.kind(),
            capability: error.capability().map(CapabilityId::as_str),
            message: error.public_message(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        COMPONENT, CapabilityId, ErrorCode, ErrorEnvelope, ErrorKind, IdentusError,
        RedactionPolicy, ResultEnvelope,
    };

    #[test]
    fn component_name_is_stable() {
        assert_eq!(COMPONENT.name, "identus-core");
    }

    #[test]
    fn error_display_is_redaction_safe() {
        let error = IdentusError::public(
            ErrorCode::new("credential_revoked"),
            ErrorKind::VerificationFailed,
            CapabilityId::new("credential"),
            "credential status rejected verification",
        );

        assert_eq!(
            error.to_string(),
            "credential_revoked: credential status rejected verification"
        );
        assert_eq!(error.redaction(), RedactionPolicy::Public);
        assert_eq!(
            error.capability().map(CapabilityId::as_str),
            Some("credential")
        );
    }

    #[test]
    fn result_envelope_maps_typed_error_fields() {
        let error = IdentusError::public(
            ErrorCode::new("unsupported_did_method"),
            ErrorKind::Unsupported,
            CapabilityId::new("did"),
            "DID method is not supported",
        );

        let envelope = ResultEnvelope::<()>::from_result(Err(error));
        assert_eq!(
            envelope,
            ResultEnvelope::Err(ErrorEnvelope {
                code: "unsupported_did_method",
                kind: ErrorKind::Unsupported,
                capability: Some("did"),
                message: "DID method is not supported",
            })
        );
    }
}
