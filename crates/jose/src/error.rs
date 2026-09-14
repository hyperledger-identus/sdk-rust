//! Static, redaction-safe errors for the JOSE wire boundary.

use std::fmt;

use identus_core::{CapabilityId, IdentusError};

use crate::error_contract::{
    ErrorContract, algorithm_key_registry_signing, compact_header, proof_key_evidence,
    proof_policy_time_replay,
};

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
    /// OID4VCI proof trust evidence violated its bounded profile.
    pub const INVALID_PROOF_EVIDENCE: ErrorCode = ErrorCode::new("jose.invalid_proof_evidence");
    /// No trust-chain provider was supplied for a federation-bound proof.
    pub const TRUST_CHAIN_PROVIDER_REQUIRED: ErrorCode =
        ErrorCode::new("jose.trust_chain_provider_required");
    /// The trust-chain provider rejected the supplied chain or key selection.
    pub const TRUST_CHAIN_REJECTED: ErrorCode = ErrorCode::new("jose.trust_chain_rejected");
    /// The trust-chain provider could not service the request.
    pub const TRUST_CHAIN_PROVIDER_UNAVAILABLE: ErrorCode =
        ErrorCode::new("jose.trust_chain_provider_unavailable");
    /// No key-attestation validator was supplied for an attested proof.
    pub const KEY_ATTESTATION_PROVIDER_REQUIRED: ErrorCode =
        ErrorCode::new("jose.key_attestation_provider_required");
    /// The key-attestation validator rejected the supplied evidence.
    pub const KEY_ATTESTATION_REJECTED: ErrorCode = ErrorCode::new("jose.key_attestation_rejected");
    /// The key-attestation validator could not service the request.
    pub const KEY_ATTESTATION_PROVIDER_UNAVAILABLE: ErrorCode =
        ErrorCode::new("jose.key_attestation_provider_unavailable");
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
    /// OID4VCI proof trust evidence is malformed or uses an ambiguous key source.
    InvalidProofEvidence,
    /// A federation-bound proof requires an injected trust-chain provider.
    TrustChainProviderRequired,
    /// The injected trust-chain provider rejected the chain.
    TrustChainRejected,
    /// The injected trust-chain provider was unavailable.
    TrustChainProviderUnavailable,
    /// An attested proof requires an injected key-attestation validator.
    KeyAttestationProviderRequired,
    /// The injected key-attestation validator rejected the evidence.
    KeyAttestationRejected,
    /// The injected key-attestation validator was unavailable.
    KeyAttestationProviderUnavailable,
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
    InvalidProofClaims => proof_key_evidence::INVALID_PROOF_CLAIMS,
    InvalidProofType => proof_key_evidence::INVALID_PROOF_TYPE,
    MissingProofKeyReference => proof_key_evidence::MISSING_PROOF_KEY_REFERENCE,
    UnsupportedProofKeyReference => proof_key_evidence::UNSUPPORTED_PROOF_KEY_REFERENCE,
    ProofKeyResolutionFailed => proof_key_evidence::PROOF_KEY_RESOLUTION_FAILED,
    ProofKeyNotAuthorized => proof_key_evidence::PROOF_KEY_NOT_AUTHORIZED,
    X5cProviderRequired => proof_key_evidence::X5C_PROVIDER_REQUIRED,
    X5cRejected => proof_key_evidence::X5C_REJECTED,
    X5cProviderUnavailable => proof_key_evidence::X5C_PROVIDER_UNAVAILABLE,
    InvalidProofEvidence => proof_key_evidence::INVALID_PROOF_EVIDENCE,
    TrustChainProviderRequired => proof_key_evidence::TRUST_CHAIN_PROVIDER_REQUIRED,
    TrustChainRejected => proof_key_evidence::TRUST_CHAIN_REJECTED,
    TrustChainProviderUnavailable => proof_key_evidence::TRUST_CHAIN_PROVIDER_UNAVAILABLE,
    KeyAttestationProviderRequired => proof_key_evidence::KEY_ATTESTATION_PROVIDER_REQUIRED,
    KeyAttestationRejected => proof_key_evidence::KEY_ATTESTATION_REJECTED,
    KeyAttestationProviderUnavailable => proof_key_evidence::KEY_ATTESTATION_PROVIDER_UNAVAILABLE,
    InvalidProofPolicy => proof_policy_time_replay::INVALID_PROOF_POLICY,
    ProofClientMismatch => proof_policy_time_replay::PROOF_CLIENT_MISMATCH,
    ProofAudienceMismatch => proof_policy_time_replay::PROOF_AUDIENCE_MISMATCH,
    ProofNonceMismatch => proof_policy_time_replay::PROOF_NONCE_MISMATCH,
    ProofStale => proof_policy_time_replay::PROOF_STALE,
    ProofIssuedInFuture => proof_policy_time_replay::PROOF_ISSUED_IN_FUTURE,
    ProofClockUnavailable => proof_policy_time_replay::PROOF_CLOCK_UNAVAILABLE,
    ProofReplayRejected => proof_policy_time_replay::PROOF_REPLAY_REJECTED,
    ProofReplayUnavailable => proof_policy_time_replay::PROOF_REPLAY_UNAVAILABLE,
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
