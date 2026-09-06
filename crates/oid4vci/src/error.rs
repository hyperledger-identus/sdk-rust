//! Static, redaction-safe Credential Offer transport errors.

use std::fmt;

use identus_core::{CapabilityId, ErrorKind, IdentusError};

/// Owning capability for OID4VCI errors.
pub const CAPABILITY: CapabilityId = CapabilityId::new("oid4vci");

/// Stable OID4VCI error codes used at the shared SDK boundary.
pub mod error_code {
    use identus_core::ErrorCode;

    pub const INVALID_LIMITS: ErrorCode = ErrorCode::new("oid4vci.invalid_limits");
    pub const INVOCATION_TOO_LARGE: ErrorCode = ErrorCode::new("oid4vci.invocation_too_large");
    pub const INVALID_INVOCATION: ErrorCode = ErrorCode::new("oid4vci.invalid_invocation");
    pub const UNSUPPORTED_TRANSPORT: ErrorCode = ErrorCode::new("oid4vci.unsupported_transport");
    pub const INVALID_FORM_ENCODING: ErrorCode = ErrorCode::new("oid4vci.invalid_form_encoding");
    pub const EMBEDDED_TOO_LARGE: ErrorCode = ErrorCode::new("oid4vci.embedded_too_large");
    pub const INVALID_EMBEDDED_JSON: ErrorCode = ErrorCode::new("oid4vci.invalid_embedded_json");
    pub const DUPLICATE_JSON_PROPERTY: ErrorCode =
        ErrorCode::new("oid4vci.duplicate_json_property");
    pub const JSON_TOO_DEEP: ErrorCode = ErrorCode::new("oid4vci.json_too_deep");
    pub const JSON_TOO_MANY_NODES: ErrorCode = ErrorCode::new("oid4vci.json_too_many_nodes");
    pub const REFERENCE_TOO_LARGE: ErrorCode = ErrorCode::new("oid4vci.reference_too_large");
    pub const UNSAFE_REFERENCE_URI: ErrorCode = ErrorCode::new("oid4vci.unsafe_reference_uri");
}

/// A static reason that Credential Offer transport validation failed.
///
/// Variants deliberately carry no input, offset, JSON, URI, or parser cause.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum CredentialOfferError {
    InvalidLimits,
    InvocationTooLarge,
    InvalidInvocation,
    UnsupportedTransport,
    InvalidFormEncoding,
    EmbeddedTooLarge,
    InvalidEmbeddedJson,
    DuplicateJsonProperty,
    JsonTooDeep,
    JsonTooManyNodes,
    ReferenceTooLarge,
    UnsafeReferenceUri,
}

impl CredentialOfferError {
    /// Convert to the workspace-wide redaction-safe error contract.
    pub const fn to_identus_error(self) -> IdentusError {
        let (code, kind, message) = match self {
            Self::InvalidLimits => (
                error_code::INVALID_LIMITS,
                ErrorKind::InvalidInput,
                "OID4VCI limits are invalid",
            ),
            Self::InvocationTooLarge => (
                error_code::INVOCATION_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Offer invocation is too large",
            ),
            Self::InvalidInvocation => (
                error_code::INVALID_INVOCATION,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Offer invocation is invalid",
            ),
            Self::UnsupportedTransport => (
                error_code::UNSUPPORTED_TRANSPORT,
                ErrorKind::Unsupported,
                "OID4VCI Credential Offer transport is unsupported",
            ),
            Self::InvalidFormEncoding => (
                error_code::INVALID_FORM_ENCODING,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Offer encoding is invalid",
            ),
            Self::EmbeddedTooLarge => (
                error_code::EMBEDDED_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI embedded Credential Offer is too large",
            ),
            Self::InvalidEmbeddedJson => (
                error_code::INVALID_EMBEDDED_JSON,
                ErrorKind::InvalidInput,
                "OID4VCI embedded Credential Offer JSON is invalid",
            ),
            Self::DuplicateJsonProperty => (
                error_code::DUPLICATE_JSON_PROPERTY,
                ErrorKind::InvalidInput,
                "OID4VCI embedded Credential Offer repeats a JSON member",
            ),
            Self::JsonTooDeep => (
                error_code::JSON_TOO_DEEP,
                ErrorKind::InvalidInput,
                "OID4VCI embedded Credential Offer JSON is too deep",
            ),
            Self::JsonTooManyNodes => (
                error_code::JSON_TOO_MANY_NODES,
                ErrorKind::InvalidInput,
                "OID4VCI embedded Credential Offer JSON has too many nodes",
            ),
            Self::ReferenceTooLarge => (
                error_code::REFERENCE_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Offer reference is too large",
            ),
            Self::UnsafeReferenceUri => (
                error_code::UNSAFE_REFERENCE_URI,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Offer reference URI is unsafe",
            ),
        };
        IdentusError::public(code, kind, CAPABILITY, message)
    }
}

impl From<CredentialOfferError> for IdentusError {
    fn from(value: CredentialOfferError) -> Self {
        value.to_identus_error()
    }
}

impl fmt::Display for CredentialOfferError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.to_identus_error().public_message())
    }
}

impl std::error::Error for CredentialOfferError {}
