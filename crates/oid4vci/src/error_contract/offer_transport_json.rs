use identus_core::ErrorKind;

use super::ErrorContract;
use crate::error::error_code;

pub(crate) const INVALID_LIMITS: ErrorContract = ErrorContract::new(
    error_code::INVALID_LIMITS,
    ErrorKind::InvalidInput,
    "OID4VCI limits are invalid",
);

pub(crate) const INVOCATION_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::INVOCATION_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Offer invocation is too large",
);

pub(crate) const INVALID_INVOCATION: ErrorContract = ErrorContract::new(
    error_code::INVALID_INVOCATION,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Offer invocation is invalid",
);

pub(crate) const UNSUPPORTED_TRANSPORT: ErrorContract = ErrorContract::new(
    error_code::UNSUPPORTED_TRANSPORT,
    ErrorKind::Unsupported,
    "OID4VCI Credential Offer transport is unsupported",
);

pub(crate) const INVALID_FORM_ENCODING: ErrorContract = ErrorContract::new(
    error_code::INVALID_FORM_ENCODING,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Offer encoding is invalid",
);

pub(crate) const EMBEDDED_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::EMBEDDED_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI embedded Credential Offer is too large",
);

pub(crate) const INVALID_EMBEDDED_JSON: ErrorContract = ErrorContract::new(
    error_code::INVALID_EMBEDDED_JSON,
    ErrorKind::InvalidInput,
    "OID4VCI JSON object is invalid",
);

pub(crate) const DUPLICATE_JSON_PROPERTY: ErrorContract = ErrorContract::new(
    error_code::DUPLICATE_JSON_PROPERTY,
    ErrorKind::InvalidInput,
    "OID4VCI JSON object repeats a member",
);

pub(crate) const JSON_TOO_DEEP: ErrorContract = ErrorContract::new(
    error_code::JSON_TOO_DEEP,
    ErrorKind::InvalidInput,
    "OID4VCI JSON object is too deep",
);

pub(crate) const JSON_TOO_MANY_NODES: ErrorContract = ErrorContract::new(
    error_code::JSON_TOO_MANY_NODES,
    ErrorKind::InvalidInput,
    "OID4VCI JSON object has too many nodes",
);

pub(crate) const REFERENCE_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::REFERENCE_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Offer reference is too large",
);

pub(crate) const UNSAFE_REFERENCE_URI: ErrorContract = ErrorContract::new(
    error_code::UNSAFE_REFERENCE_URI,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Offer reference URI is unsafe",
);
