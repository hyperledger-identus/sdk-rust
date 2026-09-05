//! Static, redaction-safe errors for the JOSE wire boundary.

use std::fmt;

use identus_core::{CapabilityId, ErrorKind, IdentusError};

/// Owning capability for JWS Compact errors.
pub const CAPABILITY: CapabilityId = CapabilityId::new("jose");

/// Stable JOSE error codes used at the shared SDK boundary.
pub mod error_code {
    use identus_core::ErrorCode;

    /// A configured maximum was zero.
    pub const INVALID_LIMITS: ErrorCode = ErrorCode::new("jose.invalid_limits");
    /// The complete compact value exceeded its configured maximum.
    pub const COMPACT_TOO_LARGE: ErrorCode = ErrorCode::new("jose.compact_too_large");
    /// The compact value did not have the required three-segment structure.
    pub const INVALID_COMPACT_STRUCTURE: ErrorCode =
        ErrorCode::new("jose.invalid_compact_structure");
    /// A segment was not canonical unpadded base64url.
    pub const NON_CANONICAL_BASE64URL: ErrorCode = ErrorCode::new("jose.non_canonical_base64url");
    /// The decoded protected header exceeded its configured maximum.
    pub const PROTECTED_HEADER_TOO_LARGE: ErrorCode =
        ErrorCode::new("jose.protected_header_too_large");
    /// The decoded payload exceeded its configured maximum.
    pub const PAYLOAD_TOO_LARGE: ErrorCode = ErrorCode::new("jose.payload_too_large");
    /// The decoded signature exceeded its configured maximum.
    pub const SIGNATURE_TOO_LARGE: ErrorCode = ErrorCode::new("jose.signature_too_large");
    /// The protected header was not one complete supported JSON object.
    pub const INVALID_PROTECTED_HEADER: ErrorCode = ErrorCode::new("jose.invalid_protected_header");
    /// A protected-header member was repeated.
    pub const DUPLICATE_PROTECTED_HEADER: ErrorCode =
        ErrorCode::new("jose.duplicate_protected_header");
    /// The protected header used a member the bounded codec does not support.
    pub const UNKNOWN_PROTECTED_HEADER: ErrorCode = ErrorCode::new("jose.unknown_protected_header");
    /// The required algorithm member was absent.
    pub const MISSING_ALGORITHM: ErrorCode = ErrorCode::new("jose.missing_algorithm");
    /// A supported protected-header member had an invalid value.
    pub const INVALID_HEADER_VALUE: ErrorCode = ErrorCode::new("jose.invalid_header_value");
    /// A compact signature was empty.
    pub const EMPTY_SIGNATURE: ErrorCode = ErrorCode::new("jose.empty_signature");
    /// Size arithmetic could not be represented safely.
    pub const SIZE_OVERFLOW: ErrorCode = ErrorCode::new("jose.size_overflow");
}

/// A static reason that a bounded JWS Compact operation failed.
///
/// Variants deliberately carry no parser cause, offset, length, header value,
/// payload, signature, key identifier, or compact input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum JoseError {
    /// At least one configured maximum was zero.
    InvalidLimits,
    /// The complete compact representation is too large.
    CompactTooLarge,
    /// The input is not exactly three compact segments, or a required segment
    /// is empty.
    InvalidCompactStructure,
    /// A segment is not canonical unpadded base64url.
    NonCanonicalBase64Url,
    /// The decoded protected header is too large.
    ProtectedHeaderTooLarge,
    /// The decoded payload is too large.
    PayloadTooLarge,
    /// The decoded signature is too large.
    SignatureTooLarge,
    /// The protected header is not one complete supported UTF-8 JSON object.
    InvalidProtectedHeader,
    /// A protected-header member occurs more than once.
    DuplicateProtectedHeader,
    /// A protected-header member is outside the closed supported surface.
    UnknownProtectedHeader,
    /// The protected header has no algorithm member.
    MissingAlgorithm,
    /// A protected-header member has an invalid type, spelling, or size.
    InvalidHeaderValue,
    /// The signature byte sequence is empty.
    EmptySignature,
    /// Size arithmetic overflowed.
    SizeOverflow,
}

impl JoseError {
    /// Convert to the workspace-wide redaction-safe error contract.
    pub const fn to_identus_error(self) -> IdentusError {
        let (code, message) = match self {
            Self::InvalidLimits => (error_code::INVALID_LIMITS, "JWS limits are invalid"),
            Self::CompactTooLarge => (
                error_code::COMPACT_TOO_LARGE,
                "JWS compact input is too large",
            ),
            Self::InvalidCompactStructure => (
                error_code::INVALID_COMPACT_STRUCTURE,
                "JWS compact structure is invalid",
            ),
            Self::NonCanonicalBase64Url => (
                error_code::NON_CANONICAL_BASE64URL,
                "JWS segment encoding is invalid",
            ),
            Self::ProtectedHeaderTooLarge => (
                error_code::PROTECTED_HEADER_TOO_LARGE,
                "JWS protected header is too large",
            ),
            Self::PayloadTooLarge => (error_code::PAYLOAD_TOO_LARGE, "JWS payload is too large"),
            Self::SignatureTooLarge => (
                error_code::SIGNATURE_TOO_LARGE,
                "JWS signature is too large",
            ),
            Self::InvalidProtectedHeader => (
                error_code::INVALID_PROTECTED_HEADER,
                "JWS protected header is invalid",
            ),
            Self::DuplicateProtectedHeader => (
                error_code::DUPLICATE_PROTECTED_HEADER,
                "JWS protected header repeats a member",
            ),
            Self::UnknownProtectedHeader => (
                error_code::UNKNOWN_PROTECTED_HEADER,
                "JWS protected header member is unsupported",
            ),
            Self::MissingAlgorithm => (
                error_code::MISSING_ALGORITHM,
                "JWS protected header algorithm is missing",
            ),
            Self::InvalidHeaderValue => (
                error_code::INVALID_HEADER_VALUE,
                "JWS protected header value is invalid",
            ),
            Self::EmptySignature => (error_code::EMPTY_SIGNATURE, "JWS signature is empty"),
            Self::SizeOverflow => (error_code::SIZE_OVERFLOW, "JWS size is invalid"),
        };
        IdentusError::public(code, ErrorKind::InvalidInput, CAPABILITY, message)
    }
}

impl From<JoseError> for IdentusError {
    fn from(value: JoseError) -> Self {
        value.to_identus_error()
    }
}

impl fmt::Display for JoseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.to_identus_error().public_message())
    }
}

impl std::error::Error for JoseError {}
