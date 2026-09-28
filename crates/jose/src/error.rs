//! Static, redaction-safe errors for the JOSE wire boundary.

use std::fmt;

use identus_core::{CapabilityId, IdentusError};

use crate::error_contract::{ErrorContract, algorithm_key_registry_signing, compact_header};

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
    /// More than one mutually exclusive protected key reference was present.
    pub const AMBIGUOUS_KEY_REFERENCE: ErrorCode = ErrorCode::new("jose.ambiguous_key_reference");
    /// A compact signature was empty.
    pub const EMPTY_SIGNATURE: ErrorCode = ErrorCode::new("jose.empty_signature");
    /// Size arithmetic could not be represented safely.
    pub const SIZE_OVERFLOW: ErrorCode = ErrorCode::new("jose.size_overflow");
    /// A protected-header algorithm is outside the accepted closed set.
    pub const UNSUPPORTED_ALGORITHM: ErrorCode = ErrorCode::new("jose.unsupported_algorithm");
    /// Header, key, signer, or verifier algorithms did not match exactly.
    pub const ALGORITHM_MISMATCH: ErrorCode = ErrorCode::new("jose.algorithm_mismatch");
    /// A verification key was incompatible with its selected algorithm.
    pub const INVALID_VERIFICATION_KEY: ErrorCode = ErrorCode::new("jose.invalid_verification_key");
    /// The signature-suite registry capacity was invalid.
    pub const INVALID_REGISTRY_CAPACITY: ErrorCode =
        ErrorCode::new("jose.invalid_registry_capacity");
    /// The signature-suite registry had no remaining capacity.
    pub const REGISTRY_FULL: ErrorCode = ErrorCode::new("jose.registry_full");
    /// A signature suite duplicated an already registered algorithm.
    pub const DUPLICATE_ALGORITHM: ErrorCode = ErrorCode::new("jose.duplicate_algorithm");
    /// The selected algorithm was not in the caller's registry allowlist.
    pub const ALGORITHM_NOT_ALLOWED: ErrorCode = ErrorCode::new("jose.algorithm_not_allowed");
    /// A compact value carried a signature with the wrong fixed width.
    pub const INVALID_SIGNATURE_LENGTH: ErrorCode = ErrorCode::new("jose.invalid_signature_length");
    /// An external signer rejected the operation.
    pub const SIGNING_REJECTED: ErrorCode = ErrorCode::new("jose.signing_rejected");
    /// An external signer was unavailable.
    pub const SIGNER_UNAVAILABLE: ErrorCode = ErrorCode::new("jose.signer_unavailable");
    /// Cryptographic signature verification failed.
    pub const SIGNATURE_INVALID: ErrorCode = ErrorCode::new("jose.signature_invalid");
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
    /// More than one of `kid`, `jwk`, or `x5c` was present.
    AmbiguousKeyReference,
    /// The signature byte sequence is empty.
    EmptySignature,
    /// Size arithmetic overflowed.
    SizeOverflow,
    /// The protected-header algorithm is outside the accepted closed set.
    UnsupportedAlgorithm,
    /// Header, key, signer, or suite algorithms did not match exactly.
    AlgorithmMismatch,
    /// The selected public key is incompatible with the selected algorithm.
    InvalidVerificationKey,
    /// The signature-suite registry capacity is zero or above its hard bound.
    InvalidRegistryCapacity,
    /// The signature-suite registry has no remaining capacity.
    RegistryFull,
    /// A suite for the same algorithm is already registered.
    DuplicateAlgorithm,
    /// The selected algorithm is absent from the caller's registry.
    AlgorithmNotAllowed,
    /// A compact value supplied a signature of the wrong length.
    InvalidSignatureLength,
    /// An external signer rejected the requested operation.
    SigningRejected,
    /// An external signer could not service the requested operation.
    SignerUnavailable,
    /// The selected signature did not verify.
    SignatureInvalid,
}

macro_rules! define_jose_error_contracts {
    ($($variant:ident => $contract:path),+ $(,)?) => {
        impl JoseError {
            const fn contract(self) -> ErrorContract {
                match self {
                    $(Self::$variant => $contract),+
                }
            }

            #[cfg(test)]
            pub(crate) const CONTRACT_VARIANTS: &'static [Self] = &[
                $(Self::$variant),+
            ];
        }
    };
}

define_jose_error_contracts! {
    InvalidLimits => compact_header::INVALID_LIMITS,
    CompactTooLarge => compact_header::COMPACT_TOO_LARGE,
    InvalidCompactStructure => compact_header::INVALID_COMPACT_STRUCTURE,
    NonCanonicalBase64Url => compact_header::NON_CANONICAL_BASE64URL,
    ProtectedHeaderTooLarge => compact_header::PROTECTED_HEADER_TOO_LARGE,
    PayloadTooLarge => compact_header::PAYLOAD_TOO_LARGE,
    SignatureTooLarge => compact_header::SIGNATURE_TOO_LARGE,
    InvalidProtectedHeader => compact_header::INVALID_PROTECTED_HEADER,
    DuplicateProtectedHeader => compact_header::DUPLICATE_PROTECTED_HEADER,
    UnknownProtectedHeader => compact_header::UNKNOWN_PROTECTED_HEADER,
    MissingAlgorithm => compact_header::MISSING_ALGORITHM,
    InvalidHeaderValue => compact_header::INVALID_HEADER_VALUE,
    AmbiguousKeyReference => compact_header::AMBIGUOUS_KEY_REFERENCE,
    EmptySignature => compact_header::EMPTY_SIGNATURE,
    SizeOverflow => compact_header::SIZE_OVERFLOW,
    UnsupportedAlgorithm => algorithm_key_registry_signing::UNSUPPORTED_ALGORITHM,
    AlgorithmMismatch => algorithm_key_registry_signing::ALGORITHM_MISMATCH,
    InvalidVerificationKey => algorithm_key_registry_signing::INVALID_VERIFICATION_KEY,
    InvalidRegistryCapacity => algorithm_key_registry_signing::INVALID_REGISTRY_CAPACITY,
    RegistryFull => algorithm_key_registry_signing::REGISTRY_FULL,
    DuplicateAlgorithm => algorithm_key_registry_signing::DUPLICATE_ALGORITHM,
    AlgorithmNotAllowed => algorithm_key_registry_signing::ALGORITHM_NOT_ALLOWED,
    InvalidSignatureLength => algorithm_key_registry_signing::INVALID_SIGNATURE_LENGTH,
    SigningRejected => algorithm_key_registry_signing::SIGNING_REJECTED,
    SignerUnavailable => algorithm_key_registry_signing::SIGNER_UNAVAILABLE,
    SignatureInvalid => algorithm_key_registry_signing::SIGNATURE_INVALID,
}

impl JoseError {
    /// Convert to the workspace-wide redaction-safe error contract.
    pub const fn to_identus_error(self) -> IdentusError {
        self.contract().to_identus_error()
    }
}

impl From<JoseError> for IdentusError {
    fn from(value: JoseError) -> Self {
        value.to_identus_error()
    }
}

impl fmt::Display for JoseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.contract().message())
    }
}

impl std::error::Error for JoseError {}
