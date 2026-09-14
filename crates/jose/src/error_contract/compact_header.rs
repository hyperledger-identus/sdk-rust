use identus_core::ErrorKind;

use super::ErrorContract;
use crate::error::error_code;

pub(crate) const INVALID_LIMITS: ErrorContract = ErrorContract::new(
    error_code::INVALID_LIMITS,
    ErrorKind::InvalidInput,
    "JWS limits are invalid",
);

pub(crate) const COMPACT_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::COMPACT_TOO_LARGE,
    ErrorKind::InvalidInput,
    "JWS compact input is too large",
);

pub(crate) const INVALID_COMPACT_STRUCTURE: ErrorContract = ErrorContract::new(
    error_code::INVALID_COMPACT_STRUCTURE,
    ErrorKind::InvalidInput,
    "JWS compact structure is invalid",
);

pub(crate) const NON_CANONICAL_BASE64URL: ErrorContract = ErrorContract::new(
    error_code::NON_CANONICAL_BASE64URL,
    ErrorKind::InvalidInput,
    "JWS segment encoding is invalid",
);

pub(crate) const PROTECTED_HEADER_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::PROTECTED_HEADER_TOO_LARGE,
    ErrorKind::InvalidInput,
    "JWS protected header is too large",
);

pub(crate) const PAYLOAD_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::PAYLOAD_TOO_LARGE,
    ErrorKind::InvalidInput,
    "JWS payload is too large",
);

pub(crate) const SIGNATURE_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::SIGNATURE_TOO_LARGE,
    ErrorKind::InvalidInput,
    "JWS signature is too large",
);

pub(crate) const INVALID_PROTECTED_HEADER: ErrorContract = ErrorContract::new(
    error_code::INVALID_PROTECTED_HEADER,
    ErrorKind::InvalidInput,
    "JWS protected header is invalid",
);

pub(crate) const DUPLICATE_PROTECTED_HEADER: ErrorContract = ErrorContract::new(
    error_code::DUPLICATE_PROTECTED_HEADER,
    ErrorKind::InvalidInput,
    "JWS protected header repeats a member",
);

pub(crate) const UNKNOWN_PROTECTED_HEADER: ErrorContract = ErrorContract::new(
    error_code::UNKNOWN_PROTECTED_HEADER,
    ErrorKind::InvalidInput,
    "JWS protected header member is unsupported",
);

pub(crate) const MISSING_ALGORITHM: ErrorContract = ErrorContract::new(
    error_code::MISSING_ALGORITHM,
    ErrorKind::InvalidInput,
    "JWS protected header algorithm is missing",
);

pub(crate) const INVALID_HEADER_VALUE: ErrorContract = ErrorContract::new(
    error_code::INVALID_HEADER_VALUE,
    ErrorKind::InvalidInput,
    "JWS protected header value is invalid",
);

pub(crate) const AMBIGUOUS_KEY_REFERENCE: ErrorContract = ErrorContract::new(
    error_code::AMBIGUOUS_KEY_REFERENCE,
    ErrorKind::InvalidInput,
    "JWS protected header key reference is ambiguous",
);

pub(crate) const EMPTY_SIGNATURE: ErrorContract = ErrorContract::new(
    error_code::EMPTY_SIGNATURE,
    ErrorKind::InvalidInput,
    "JWS signature is empty",
);

pub(crate) const SIZE_OVERFLOW: ErrorContract = ErrorContract::new(
    error_code::SIZE_OVERFLOW,
    ErrorKind::InvalidInput,
    "JWS size is invalid",
);
