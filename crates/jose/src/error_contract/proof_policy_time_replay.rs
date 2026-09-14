use identus_core::ErrorKind;

use super::ErrorContract;
use crate::error::error_code;

pub(crate) const INVALID_PROOF_POLICY: ErrorContract = ErrorContract::new(
    error_code::INVALID_PROOF_POLICY,
    ErrorKind::InvalidInput,
    "OID4VCI proof JWT policy is invalid",
);

pub(crate) const PROOF_CLIENT_MISMATCH: ErrorContract = ErrorContract::new(
    error_code::PROOF_CLIENT_MISMATCH,
    ErrorKind::VerificationFailed,
    "OID4VCI proof JWT client does not match",
);

pub(crate) const PROOF_AUDIENCE_MISMATCH: ErrorContract = ErrorContract::new(
    error_code::PROOF_AUDIENCE_MISMATCH,
    ErrorKind::VerificationFailed,
    "OID4VCI proof JWT audience does not match",
);

pub(crate) const PROOF_NONCE_MISMATCH: ErrorContract = ErrorContract::new(
    error_code::PROOF_NONCE_MISMATCH,
    ErrorKind::VerificationFailed,
    "OID4VCI proof JWT nonce does not match",
);

pub(crate) const PROOF_STALE: ErrorContract = ErrorContract::new(
    error_code::PROOF_STALE,
    ErrorKind::VerificationFailed,
    "OID4VCI proof JWT is stale",
);

pub(crate) const PROOF_ISSUED_IN_FUTURE: ErrorContract = ErrorContract::new(
    error_code::PROOF_ISSUED_IN_FUTURE,
    ErrorKind::VerificationFailed,
    "OID4VCI proof JWT issuance time is in the future",
);

pub(crate) const PROOF_CLOCK_UNAVAILABLE: ErrorContract = ErrorContract::new(
    error_code::PROOF_CLOCK_UNAVAILABLE,
    ErrorKind::Internal,
    "OID4VCI proof JWT clock is unavailable",
);

pub(crate) const PROOF_REPLAY_REJECTED: ErrorContract = ErrorContract::new(
    error_code::PROOF_REPLAY_REJECTED,
    ErrorKind::VerificationFailed,
    "OID4VCI proof JWT replay was rejected",
);

pub(crate) const PROOF_REPLAY_UNAVAILABLE: ErrorContract = ErrorContract::new(
    error_code::PROOF_REPLAY_UNAVAILABLE,
    ErrorKind::Internal,
    "OID4VCI proof JWT replay guard is unavailable",
);
