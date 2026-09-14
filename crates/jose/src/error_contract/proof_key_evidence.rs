use identus_core::ErrorKind;

use super::ErrorContract;
use crate::error::error_code;

pub(crate) const INVALID_PROOF_CLAIMS: ErrorContract = ErrorContract::new(
    error_code::INVALID_PROOF_CLAIMS,
    ErrorKind::InvalidInput,
    "OID4VCI proof JWT claims are invalid",
);

pub(crate) const INVALID_PROOF_TYPE: ErrorContract = ErrorContract::new(
    error_code::INVALID_PROOF_TYPE,
    ErrorKind::InvalidInput,
    "OID4VCI proof JWT type is invalid",
);

pub(crate) const MISSING_PROOF_KEY_REFERENCE: ErrorContract = ErrorContract::new(
    error_code::MISSING_PROOF_KEY_REFERENCE,
    ErrorKind::InvalidInput,
    "OID4VCI proof JWT key reference is missing",
);

pub(crate) const UNSUPPORTED_PROOF_KEY_REFERENCE: ErrorContract = ErrorContract::new(
    error_code::UNSUPPORTED_PROOF_KEY_REFERENCE,
    ErrorKind::Unsupported,
    "OID4VCI proof JWT key reference is unsupported",
);

pub(crate) const PROOF_KEY_RESOLUTION_FAILED: ErrorContract = ErrorContract::new(
    error_code::PROOF_KEY_RESOLUTION_FAILED,
    ErrorKind::VerificationFailed,
    "OID4VCI proof JWT key resolution failed",
);

pub(crate) const PROOF_KEY_NOT_AUTHORIZED: ErrorContract = ErrorContract::new(
    error_code::PROOF_KEY_NOT_AUTHORIZED,
    ErrorKind::VerificationFailed,
    "OID4VCI proof JWT key is not authorized",
);

pub(crate) const X5C_PROVIDER_REQUIRED: ErrorContract = ErrorContract::new(
    error_code::X5C_PROVIDER_REQUIRED,
    ErrorKind::InvalidInput,
    "OID4VCI proof JWT certificate provider is required",
);

pub(crate) const X5C_REJECTED: ErrorContract = ErrorContract::new(
    error_code::X5C_REJECTED,
    ErrorKind::VerificationFailed,
    "OID4VCI proof JWT certificate chain was rejected",
);

pub(crate) const X5C_PROVIDER_UNAVAILABLE: ErrorContract = ErrorContract::new(
    error_code::X5C_PROVIDER_UNAVAILABLE,
    ErrorKind::Internal,
    "OID4VCI proof JWT certificate provider is unavailable",
);

pub(crate) const INVALID_PROOF_EVIDENCE: ErrorContract = ErrorContract::new(
    error_code::INVALID_PROOF_EVIDENCE,
    ErrorKind::InvalidInput,
    "OID4VCI proof JWT trust evidence is invalid",
);

pub(crate) const TRUST_CHAIN_PROVIDER_REQUIRED: ErrorContract = ErrorContract::new(
    error_code::TRUST_CHAIN_PROVIDER_REQUIRED,
    ErrorKind::InvalidInput,
    "OID4VCI proof JWT trust-chain provider is required",
);

pub(crate) const TRUST_CHAIN_REJECTED: ErrorContract = ErrorContract::new(
    error_code::TRUST_CHAIN_REJECTED,
    ErrorKind::VerificationFailed,
    "OID4VCI proof JWT trust chain was rejected",
);

pub(crate) const TRUST_CHAIN_PROVIDER_UNAVAILABLE: ErrorContract = ErrorContract::new(
    error_code::TRUST_CHAIN_PROVIDER_UNAVAILABLE,
    ErrorKind::Internal,
    "OID4VCI proof JWT trust-chain provider is unavailable",
);

pub(crate) const KEY_ATTESTATION_PROVIDER_REQUIRED: ErrorContract = ErrorContract::new(
    error_code::KEY_ATTESTATION_PROVIDER_REQUIRED,
    ErrorKind::InvalidInput,
    "OID4VCI proof JWT key-attestation validator is required",
);

pub(crate) const KEY_ATTESTATION_REJECTED: ErrorContract = ErrorContract::new(
    error_code::KEY_ATTESTATION_REJECTED,
    ErrorKind::VerificationFailed,
    "OID4VCI proof JWT key attestation was rejected",
);

pub(crate) const KEY_ATTESTATION_PROVIDER_UNAVAILABLE: ErrorContract = ErrorContract::new(
    error_code::KEY_ATTESTATION_PROVIDER_UNAVAILABLE,
    ErrorKind::Internal,
    "OID4VCI proof JWT key-attestation validator is unavailable",
);
