//! Static, redaction-safe errors for the OID4VCI proof profile.

use std::fmt;

use identus_core::IdentusError;
use identus_jose::JoseError;

use crate::error_contract::{ErrorContract, proof_key_evidence, proof_policy_time_replay};

/// Stable error codes owned by the OID4VCI proof profile.
pub mod error_code {
    use identus_core::ErrorCode;

    pub const INVALID_PROOF_CLAIMS: ErrorCode = ErrorCode::new("oid4vci.invalid_proof_claims");
    pub const INVALID_PROOF_TYPE: ErrorCode = ErrorCode::new("oid4vci.invalid_proof_type");
    pub const MISSING_PROOF_KEY_REFERENCE: ErrorCode =
        ErrorCode::new("oid4vci.missing_proof_key_reference");
    pub const UNSUPPORTED_PROOF_KEY_REFERENCE: ErrorCode =
        ErrorCode::new("oid4vci.unsupported_proof_key_reference");
    pub const PROOF_KEY_RESOLUTION_FAILED: ErrorCode =
        ErrorCode::new("oid4vci.proof_key_resolution_failed");
    pub const PROOF_KEY_NOT_AUTHORIZED: ErrorCode =
        ErrorCode::new("oid4vci.proof_key_not_authorized");
    pub const X5C_PROVIDER_REQUIRED: ErrorCode = ErrorCode::new("oid4vci.x5c_provider_required");
    pub const X5C_REJECTED: ErrorCode = ErrorCode::new("oid4vci.x5c_rejected");
    pub const X5C_PROVIDER_UNAVAILABLE: ErrorCode =
        ErrorCode::new("oid4vci.x5c_provider_unavailable");
    pub const INVALID_PROOF_EVIDENCE: ErrorCode = ErrorCode::new("oid4vci.invalid_proof_evidence");
    pub const TRUST_CHAIN_PROVIDER_REQUIRED: ErrorCode =
        ErrorCode::new("oid4vci.trust_chain_provider_required");
    pub const TRUST_CHAIN_REJECTED: ErrorCode = ErrorCode::new("oid4vci.trust_chain_rejected");
    pub const TRUST_CHAIN_PROVIDER_UNAVAILABLE: ErrorCode =
        ErrorCode::new("oid4vci.trust_chain_provider_unavailable");
    pub const KEY_ATTESTATION_PROVIDER_REQUIRED: ErrorCode =
        ErrorCode::new("oid4vci.key_attestation_provider_required");
    pub const KEY_ATTESTATION_REJECTED: ErrorCode =
        ErrorCode::new("oid4vci.key_attestation_rejected");
    pub const KEY_ATTESTATION_PROVIDER_UNAVAILABLE: ErrorCode =
        ErrorCode::new("oid4vci.key_attestation_provider_unavailable");
    pub const INVALID_PROOF_POLICY: ErrorCode = ErrorCode::new("oid4vci.invalid_proof_policy");
    pub const PROOF_CLIENT_MISMATCH: ErrorCode = ErrorCode::new("oid4vci.proof_client_mismatch");
    pub const PROOF_AUDIENCE_MISMATCH: ErrorCode =
        ErrorCode::new("oid4vci.proof_audience_mismatch");
    pub const PROOF_NONCE_MISMATCH: ErrorCode = ErrorCode::new("oid4vci.proof_nonce_mismatch");
    pub const PROOF_STALE: ErrorCode = ErrorCode::new("oid4vci.proof_stale");
    pub const PROOF_ISSUED_IN_FUTURE: ErrorCode = ErrorCode::new("oid4vci.proof_issued_in_future");
    pub const PROOF_CLOCK_UNAVAILABLE: ErrorCode =
        ErrorCode::new("oid4vci.proof_clock_unavailable");
    pub const PROOF_REPLAY_REJECTED: ErrorCode = ErrorCode::new("oid4vci.proof_replay_rejected");
    pub const PROOF_REPLAY_UNAVAILABLE: ErrorCode =
        ErrorCode::new("oid4vci.proof_replay_unavailable");
}

/// Static proof-profile failure.
///
/// Generic compact/signature failures retain their JOSE capability through
/// [`Self::Jose`]. Profile policy failures map to the OID4VCI capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Oid4vciProofError {
    /// A protocol-neutral JOSE operation failed.
    Jose(JoseError),
    /// Proof claims are invalid for the bounded Final profile.
    InvalidProofClaims,
    /// The protected type is not the required proof-JWT type.
    InvalidProofType,
    /// The proof has no signing-key reference.
    MissingProofKeyReference,
    /// The selected signing-key reference is unsupported.
    UnsupportedProofKeyReference,
    /// DID key resolution or projection failed.
    ProofKeyResolutionFailed,
    /// The resolved DID key is not authorized for authentication.
    ProofKeyNotAuthorized,
    /// An X.509 reference requires a certificate-key provider.
    X5cProviderRequired,
    /// The certificate-key provider rejected the chain.
    X5cRejected,
    /// The certificate-key provider is unavailable.
    X5cProviderUnavailable,
    /// Proof trust evidence is malformed or ambiguous.
    InvalidProofEvidence,
    /// Federation evidence requires a trust-chain provider.
    TrustChainProviderRequired,
    /// The trust-chain provider rejected the evidence.
    TrustChainRejected,
    /// The trust-chain provider is unavailable.
    TrustChainProviderUnavailable,
    /// Attested proof evidence requires an attestation validator.
    KeyAttestationProviderRequired,
    /// The attestation validator rejected the evidence.
    KeyAttestationRejected,
    /// The attestation validator is unavailable.
    KeyAttestationProviderUnavailable,
    /// Caller-supplied proof policy is invalid.
    InvalidProofPolicy,
    /// The proof client does not match policy.
    ProofClientMismatch,
    /// The proof audience does not match policy.
    ProofAudienceMismatch,
    /// The proof nonce does not match policy.
    ProofNonceMismatch,
    /// The proof is older than policy permits.
    ProofStale,
    /// The proof issuance time is too far in the future.
    ProofIssuedInFuture,
    /// The injected clock is unavailable.
    ProofClockUnavailable,
    /// The replay guard rejected the proof.
    ProofReplayRejected,
    /// The replay guard is unavailable.
    ProofReplayUnavailable,
}

macro_rules! define_proof_error_contracts {
    ($($variant:ident => $contract:path),+ $(,)?) => {
        impl Oid4vciProofError {
            const fn profile_contract(self) -> Option<ErrorContract> {
                match self {
                    Self::Jose(_) => None,
                    $(Self::$variant => Some($contract)),+
                }
            }

            #[cfg(test)]
            pub(crate) const PROFILE_CONTRACT_VARIANTS: &'static [Self] = &[
                $(Self::$variant),+
            ];
        }
    };
}

define_proof_error_contracts! {
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

impl Oid4vciProofError {
    /// Convert to the workspace-wide static public error contract.
    pub const fn to_identus_error(self) -> IdentusError {
        match self {
            Self::Jose(error) => error.to_identus_error(),
            _ => match self.profile_contract() {
                Some(contract) => contract.to_identus_error(),
                None => unreachable!(),
            },
        }
    }
}

impl From<JoseError> for Oid4vciProofError {
    fn from(value: JoseError) -> Self {
        Self::Jose(value)
    }
}

impl From<Oid4vciProofError> for IdentusError {
    fn from(value: Oid4vciProofError) -> Self {
        value.to_identus_error()
    }
}

impl fmt::Display for Oid4vciProofError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Jose(error) => error.fmt(formatter),
            _ => formatter.write_str(
                self.profile_contract()
                    .expect("profile variant has a static contract")
                    .message(),
            ),
        }
    }
}

impl std::error::Error for Oid4vciProofError {}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::Oid4vciProofError;
    use crate::CAPABILITY;

    #[test]
    fn profile_contract_inventory_is_unique_and_oid4vci_owned() {
        let variants = Oid4vciProofError::PROFILE_CONTRACT_VARIANTS;
        let names: BTreeSet<_> = variants.iter().map(|error| format!("{error:?}")).collect();

        assert_eq!(variants.len(), 25);
        assert_eq!(names.len(), variants.len());
        assert!(variants.iter().all(|error| {
            let public = error.to_identus_error();
            public.capability() == Some(CAPABILITY)
                && public.code().as_str().starts_with("oid4vci.")
        }));
    }
}
