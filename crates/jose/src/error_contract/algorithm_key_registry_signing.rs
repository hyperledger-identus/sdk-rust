use identus_core::ErrorKind;

use super::ErrorContract;
use crate::error::error_code;

pub(crate) const UNSUPPORTED_ALGORITHM: ErrorContract = ErrorContract::new(
    error_code::UNSUPPORTED_ALGORITHM,
    ErrorKind::Unsupported,
    "JWS algorithm is unsupported",
);

pub(crate) const ALGORITHM_MISMATCH: ErrorContract = ErrorContract::new(
    error_code::ALGORITHM_MISMATCH,
    ErrorKind::InvalidInput,
    "JWS algorithm binding does not match",
);

pub(crate) const INVALID_VERIFICATION_KEY: ErrorContract = ErrorContract::new(
    error_code::INVALID_VERIFICATION_KEY,
    ErrorKind::InvalidInput,
    "JWS verification key is invalid",
);

pub(crate) const INVALID_REGISTRY_CAPACITY: ErrorContract = ErrorContract::new(
    error_code::INVALID_REGISTRY_CAPACITY,
    ErrorKind::InvalidInput,
    "JWS signature registry capacity is invalid",
);

pub(crate) const REGISTRY_FULL: ErrorContract = ErrorContract::new(
    error_code::REGISTRY_FULL,
    ErrorKind::InvalidInput,
    "JWS signature registry is full",
);

pub(crate) const DUPLICATE_ALGORITHM: ErrorContract = ErrorContract::new(
    error_code::DUPLICATE_ALGORITHM,
    ErrorKind::InvalidInput,
    "JWS signature algorithm is already registered",
);

pub(crate) const ALGORITHM_NOT_ALLOWED: ErrorContract = ErrorContract::new(
    error_code::ALGORITHM_NOT_ALLOWED,
    ErrorKind::Unsupported,
    "JWS signature algorithm is not allowed",
);

pub(crate) const INVALID_SIGNATURE_LENGTH: ErrorContract = ErrorContract::new(
    error_code::INVALID_SIGNATURE_LENGTH,
    ErrorKind::InvalidInput,
    "JWS signature length is invalid",
);

pub(crate) const SIGNING_REJECTED: ErrorContract = ErrorContract::new(
    error_code::SIGNING_REJECTED,
    ErrorKind::Crypto,
    "JWS signing operation was rejected",
);

pub(crate) const SIGNER_UNAVAILABLE: ErrorContract = ErrorContract::new(
    error_code::SIGNER_UNAVAILABLE,
    ErrorKind::Crypto,
    "JWS signer is unavailable",
);

pub(crate) const SIGNATURE_INVALID: ErrorContract = ErrorContract::new(
    error_code::SIGNATURE_INVALID,
    ErrorKind::VerificationFailed,
    "JWS signature verification failed",
);
