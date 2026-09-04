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
    pub const INVALID_VERIFICATION_REASON_CODE: ErrorCode =
        ErrorCode::new("credential.invalid_verification_reason_code");
    pub const MISSING_VERIFICATION_REASON: ErrorCode =
        ErrorCode::new("credential.missing_verification_reason");
    pub const UNEXPECTED_VERIFICATION_REASON: ErrorCode =
        ErrorCode::new("credential.unexpected_verification_reason");
    pub const NON_CANONICAL_VERIFICATION_REPORT: ErrorCode =
        ErrorCode::new("credential.non_canonical_verification_report");
}

/// Typed reason that a credential domain value could not be constructed.
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
    /// A verification reason violates its token grammar or size limit.
    InvalidVerificationReasonCode,
    /// A failed or not-checked stage has no machine reason.
    MissingVerificationReason,
    /// A passed stage incorrectly carries a machine reason.
    UnexpectedVerificationReason,
    /// Verification stages are not complete and canonically ordered.
    NonCanonicalVerificationReport,
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
            Self::InvalidVerificationReasonCode => (
                error_code::INVALID_VERIFICATION_REASON_CODE,
                "invalid credential verification reason code",
            ),
            Self::MissingVerificationReason => (
                error_code::MISSING_VERIFICATION_REASON,
                "credential verification reason is required",
            ),
            Self::UnexpectedVerificationReason => (
                error_code::UNEXPECTED_VERIFICATION_REASON,
                "credential verification reason is not allowed",
            ),
            Self::NonCanonicalVerificationReport => (
                error_code::NON_CANONICAL_VERIFICATION_REPORT,
                "credential verification report is not canonical",
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
            Self::InvalidVerificationReasonCode => "credential verification reason code is invalid",
            Self::MissingVerificationReason => "credential verification reason is required",
            Self::UnexpectedVerificationReason => "credential verification reason is not allowed",
            Self::NonCanonicalVerificationReport => {
                "credential verification report is not canonical"
            }
        })
    }
}

impl std::error::Error for CredentialError {}
