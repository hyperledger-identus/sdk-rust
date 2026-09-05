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
    /// OID4VCI proof claims violated the bounded profile.
    pub const INVALID_PROOF_CLAIMS: ErrorCode = ErrorCode::new("jose.invalid_proof_claims");
    /// The proof did not carry the exact OID4VCI protected type.
    pub const INVALID_PROOF_TYPE: ErrorCode = ErrorCode::new("jose.invalid_proof_type");
    /// The proof omitted its signing-key reference.
    pub const MISSING_PROOF_KEY_REFERENCE: ErrorCode =
        ErrorCode::new("jose.missing_proof_key_reference");
    /// The selected proof key-reference form is not supported by this verifier.
    pub const UNSUPPORTED_PROOF_KEY_REFERENCE: ErrorCode =
        ErrorCode::new("jose.unsupported_proof_key_reference");
    /// DID key dereferencing or projection failed.
    pub const PROOF_KEY_RESOLUTION_FAILED: ErrorCode =
        ErrorCode::new("jose.proof_key_resolution_failed");
    /// The selected DID key was not authorized for authentication.
    pub const PROOF_KEY_NOT_AUTHORIZED: ErrorCode = ErrorCode::new("jose.proof_key_not_authorized");
    /// No certificate-key provider was supplied for an X.509 reference.
    pub const X5C_PROVIDER_REQUIRED: ErrorCode = ErrorCode::new("jose.x5c_provider_required");
    /// The certificate-key provider rejected the supplied chain.
    pub const X5C_REJECTED: ErrorCode = ErrorCode::new("jose.x5c_rejected");
    /// The certificate-key provider could not service the request.
    pub const X5C_PROVIDER_UNAVAILABLE: ErrorCode = ErrorCode::new("jose.x5c_provider_unavailable");
    /// Issuer proof policy inputs were invalid.
    pub const INVALID_PROOF_POLICY: ErrorCode = ErrorCode::new("jose.invalid_proof_policy");
    /// The proof issuer/client did not match policy.
    pub const PROOF_CLIENT_MISMATCH: ErrorCode = ErrorCode::new("jose.proof_client_mismatch");
    /// The proof audience did not match policy.
    pub const PROOF_AUDIENCE_MISMATCH: ErrorCode = ErrorCode::new("jose.proof_audience_mismatch");
    /// The proof nonce did not match policy.
    pub const PROOF_NONCE_MISMATCH: ErrorCode = ErrorCode::new("jose.proof_nonce_mismatch");
    /// The proof issuance time was older than policy permits.
    pub const PROOF_STALE: ErrorCode = ErrorCode::new("jose.proof_stale");
    /// The proof issuance time was too far in the future.
    pub const PROOF_ISSUED_IN_FUTURE: ErrorCode = ErrorCode::new("jose.proof_issued_in_future");
    /// The injected clock could not provide a trustworthy observation.
    pub const PROOF_CLOCK_UNAVAILABLE: ErrorCode = ErrorCode::new("jose.proof_clock_unavailable");
    /// The replay guard rejected a proof.
    pub const PROOF_REPLAY_REJECTED: ErrorCode = ErrorCode::new("jose.proof_replay_rejected");
    /// The replay guard could not service the request.
    pub const PROOF_REPLAY_UNAVAILABLE: ErrorCode = ErrorCode::new("jose.proof_replay_unavailable");
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
    /// OID4VCI proof claims are empty, unbounded, or otherwise invalid.
    InvalidProofClaims,
    /// The protected `typ` is not exactly `openid4vci-proof+jwt`.
    InvalidProofType,
    /// No signing-key reference is present in the protected header.
    MissingProofKeyReference,
    /// The selected signing-key reference cannot be resolved by this profile.
    UnsupportedProofKeyReference,
    /// A DID key could not be resolved or projected safely.
    ProofKeyResolutionFailed,
    /// A DID key was not authorized by the exact authentication relationship.
    ProofKeyNotAuthorized,
    /// An X.509 reference requires an injected certificate-key provider.
    X5cProviderRequired,
    /// The certificate-key provider rejected the chain.
    X5cRejected,
    /// The certificate-key provider was unavailable.
    X5cProviderUnavailable,
    /// Caller-supplied proof policy is invalid.
    InvalidProofPolicy,
    /// The proof issuer/client mode does not match caller policy.
    ProofClientMismatch,
    /// The proof audience does not match caller policy.
    ProofAudienceMismatch,
    /// The proof nonce does not match caller policy.
    ProofNonceMismatch,
    /// The proof is older than the caller's accepted window.
    ProofStale,
    /// The proof issuance time is too far in the future.
    ProofIssuedInFuture,
    /// The injected wall clock was unavailable.
    ProofClockUnavailable,
    /// The caller-owned replay guard rejected the proof.
    ProofReplayRejected,
    /// The caller-owned replay guard was unavailable.
    ProofReplayUnavailable,
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
            Self::AmbiguousKeyReference => (
                error_code::AMBIGUOUS_KEY_REFERENCE,
                "JWS protected header key reference is ambiguous",
            ),
            Self::EmptySignature => (error_code::EMPTY_SIGNATURE, "JWS signature is empty"),
            Self::SizeOverflow => (error_code::SIZE_OVERFLOW, "JWS size is invalid"),
            Self::UnsupportedAlgorithm => (
                error_code::UNSUPPORTED_ALGORITHM,
                "JWS algorithm is unsupported",
            ),
            Self::AlgorithmMismatch => (
                error_code::ALGORITHM_MISMATCH,
                "JWS algorithm binding does not match",
            ),
            Self::InvalidVerificationKey => (
                error_code::INVALID_VERIFICATION_KEY,
                "JWS verification key is invalid",
            ),
            Self::InvalidRegistryCapacity => (
                error_code::INVALID_REGISTRY_CAPACITY,
                "JWS signature registry capacity is invalid",
            ),
            Self::RegistryFull => (error_code::REGISTRY_FULL, "JWS signature registry is full"),
            Self::DuplicateAlgorithm => (
                error_code::DUPLICATE_ALGORITHM,
                "JWS signature algorithm is already registered",
            ),
            Self::AlgorithmNotAllowed => (
                error_code::ALGORITHM_NOT_ALLOWED,
                "JWS signature algorithm is not allowed",
            ),
            Self::InvalidSignatureLength => (
                error_code::INVALID_SIGNATURE_LENGTH,
                "JWS signature length is invalid",
            ),
            Self::SigningRejected => (
                error_code::SIGNING_REJECTED,
                "JWS signing operation was rejected",
            ),
            Self::SignerUnavailable => {
                (error_code::SIGNER_UNAVAILABLE, "JWS signer is unavailable")
            }
            Self::SignatureInvalid => (
                error_code::SIGNATURE_INVALID,
                "JWS signature verification failed",
            ),
            Self::InvalidProofClaims => (
                error_code::INVALID_PROOF_CLAIMS,
                "OID4VCI proof JWT claims are invalid",
            ),
            Self::InvalidProofType => (
                error_code::INVALID_PROOF_TYPE,
                "OID4VCI proof JWT type is invalid",
            ),
            Self::MissingProofKeyReference => (
                error_code::MISSING_PROOF_KEY_REFERENCE,
                "OID4VCI proof JWT key reference is missing",
            ),
            Self::UnsupportedProofKeyReference => (
                error_code::UNSUPPORTED_PROOF_KEY_REFERENCE,
                "OID4VCI proof JWT key reference is unsupported",
            ),
            Self::ProofKeyResolutionFailed => (
                error_code::PROOF_KEY_RESOLUTION_FAILED,
                "OID4VCI proof JWT key resolution failed",
            ),
            Self::ProofKeyNotAuthorized => (
                error_code::PROOF_KEY_NOT_AUTHORIZED,
                "OID4VCI proof JWT key is not authorized",
            ),
            Self::X5cProviderRequired => (
                error_code::X5C_PROVIDER_REQUIRED,
                "OID4VCI proof JWT certificate provider is required",
            ),
            Self::X5cRejected => (
                error_code::X5C_REJECTED,
                "OID4VCI proof JWT certificate chain was rejected",
            ),
            Self::X5cProviderUnavailable => (
                error_code::X5C_PROVIDER_UNAVAILABLE,
                "OID4VCI proof JWT certificate provider is unavailable",
            ),
            Self::InvalidProofPolicy => (
                error_code::INVALID_PROOF_POLICY,
                "OID4VCI proof JWT policy is invalid",
            ),
            Self::ProofClientMismatch => (
                error_code::PROOF_CLIENT_MISMATCH,
                "OID4VCI proof JWT client does not match",
            ),
            Self::ProofAudienceMismatch => (
                error_code::PROOF_AUDIENCE_MISMATCH,
                "OID4VCI proof JWT audience does not match",
            ),
            Self::ProofNonceMismatch => (
                error_code::PROOF_NONCE_MISMATCH,
                "OID4VCI proof JWT nonce does not match",
            ),
            Self::ProofStale => (error_code::PROOF_STALE, "OID4VCI proof JWT is stale"),
            Self::ProofIssuedInFuture => (
                error_code::PROOF_ISSUED_IN_FUTURE,
                "OID4VCI proof JWT issuance time is in the future",
            ),
            Self::ProofClockUnavailable => (
                error_code::PROOF_CLOCK_UNAVAILABLE,
                "OID4VCI proof JWT clock is unavailable",
            ),
            Self::ProofReplayRejected => (
                error_code::PROOF_REPLAY_REJECTED,
                "OID4VCI proof JWT replay was rejected",
            ),
            Self::ProofReplayUnavailable => (
                error_code::PROOF_REPLAY_UNAVAILABLE,
                "OID4VCI proof JWT replay guard is unavailable",
            ),
        };
        let kind = match self {
            Self::UnsupportedAlgorithm
            | Self::AlgorithmNotAllowed
            | Self::UnsupportedProofKeyReference => ErrorKind::Unsupported,
            Self::SigningRejected | Self::SignerUnavailable => ErrorKind::Crypto,
            Self::X5cProviderUnavailable
            | Self::ProofClockUnavailable
            | Self::ProofReplayUnavailable => ErrorKind::Internal,
            Self::SignatureInvalid
            | Self::ProofKeyResolutionFailed
            | Self::ProofKeyNotAuthorized
            | Self::X5cRejected
            | Self::ProofClientMismatch
            | Self::ProofAudienceMismatch
            | Self::ProofNonceMismatch
            | Self::ProofStale
            | Self::ProofIssuedInFuture
            | Self::ProofReplayRejected => ErrorKind::VerificationFailed,
            _ => ErrorKind::InvalidInput,
        };
        IdentusError::public(code, kind, CAPABILITY, message)
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
