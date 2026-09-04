//! Redaction-safe credential construction errors.

use std::fmt;

use identus_core::{CapabilityId, ErrorKind, IdentusError};

/// Owning capability for credential construction errors.
pub const CAPABILITY: CapabilityId = CapabilityId::new("credential");

/// Stable error codes used by the shared SDK error boundary.
pub mod error_code {
    use identus_core::ErrorCode;

    pub const INVALID_FORMAT: ErrorCode = ErrorCode::new("credential.invalid_format");
    pub const EMPTY_PAYLOAD: ErrorCode = ErrorCode::new("credential.empty_payload");
    pub const PAYLOAD_TOO_LARGE: ErrorCode = ErrorCode::new("credential.payload_too_large");
    pub const EMPTY_DETACHED_PROOF: ErrorCode = ErrorCode::new("credential.empty_detached_proof");
    pub const DETACHED_PROOF_TOO_LARGE: ErrorCode =
        ErrorCode::new("credential.detached_proof_too_large");
    pub const EMPTY_PRIVATE_MATERIAL: ErrorCode =
        ErrorCode::new("credential.empty_private_material");
    pub const PRIVATE_MATERIAL_TOO_LARGE: ErrorCode =
        ErrorCode::new("credential.private_material_too_large");
}

/// Typed reason that a credential envelope value could not be constructed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum CredentialError {
    /// A format identifier violates the open token grammar or size limit.
    InvalidFormat,
    /// An encoded credential payload is empty.
    EmptyPayload,
    /// An encoded credential payload exceeds its size limit.
    PayloadTooLarge,
    /// A detached proof is empty.
    EmptyDetachedProof,
    /// A detached proof exceeds its size limit.
    DetachedProofTooLarge,
    /// Format-private material is empty.
    EmptyPrivateMaterial,
    /// Format-private material exceeds its size limit.
    PrivateMaterialTooLarge,
}

impl CredentialError {
    /// Convert to the stable, redaction-safe shared SDK error.
    pub fn to_identus_error(self) -> IdentusError {
        let (code, public_message) = match self {
            Self::InvalidFormat => (error_code::INVALID_FORMAT, "invalid credential format"),
            Self::EmptyPayload => (error_code::EMPTY_PAYLOAD, "credential payload is empty"),
            Self::PayloadTooLarge => (
                error_code::PAYLOAD_TOO_LARGE,
                "credential payload exceeds the size limit",
            ),
            Self::EmptyDetachedProof => (
                error_code::EMPTY_DETACHED_PROOF,
                "credential detached proof is empty",
            ),
            Self::DetachedProofTooLarge => (
                error_code::DETACHED_PROOF_TOO_LARGE,
                "credential detached proof exceeds the size limit",
            ),
            Self::EmptyPrivateMaterial => (
                error_code::EMPTY_PRIVATE_MATERIAL,
                "credential private material is empty",
            ),
            Self::PrivateMaterialTooLarge => (
                error_code::PRIVATE_MATERIAL_TOO_LARGE,
                "credential private material exceeds the size limit",
            ),
        };

        IdentusError::public(code, ErrorKind::InvalidInput, CAPABILITY, public_message)
    }
}

impl fmt::Display for CredentialError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidFormat => "credential format is invalid",
            Self::EmptyPayload => "credential payload is empty",
            Self::PayloadTooLarge => "credential payload exceeds the size limit",
            Self::EmptyDetachedProof => "credential detached proof is empty",
            Self::DetachedProofTooLarge => "credential detached proof exceeds the size limit",
            Self::EmptyPrivateMaterial => "credential private material is empty",
            Self::PrivateMaterialTooLarge => "credential private material exceeds the size limit",
        })
    }
}

impl std::error::Error for CredentialError {}
