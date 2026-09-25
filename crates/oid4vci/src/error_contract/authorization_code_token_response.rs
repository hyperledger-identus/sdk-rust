use identus_core::ErrorKind;

use super::ErrorContract;
use crate::error::error_code;

pub(crate) const INVALID_AUTHORIZATION_CODE_TOKEN_HTTP_RESPONSE_LIMITS: ErrorContract =
    ErrorContract::new(
        error_code::INVALID_AUTHORIZATION_CODE_TOKEN_HTTP_RESPONSE_LIMITS,
        ErrorKind::InvalidInput,
        "OID4VCI Authorization Code Token HTTP response limits are invalid",
    );

pub(crate) const INVALID_AUTHORIZATION_CODE_TOKEN_HTTP_STATUS: ErrorContract = ErrorContract::new(
    error_code::INVALID_AUTHORIZATION_CODE_TOKEN_HTTP_STATUS,
    ErrorKind::InvalidInput,
    "OID4VCI Authorization Code Token HTTP status is invalid",
);

pub(crate) const AUTHORIZATION_CODE_TOKEN_HTTP_STATUS_ERROR_MISMATCH: ErrorContract =
    ErrorContract::new(
        error_code::AUTHORIZATION_CODE_TOKEN_HTTP_STATUS_ERROR_MISMATCH,
        ErrorKind::InvalidInput,
        "OID4VCI Authorization Code Token HTTP status and error do not match",
    );

pub(crate) const AUTHORIZATION_CODE_TOKEN_CONTENT_TYPE_TOO_LARGE: ErrorContract =
    ErrorContract::new(
        error_code::AUTHORIZATION_CODE_TOKEN_CONTENT_TYPE_TOO_LARGE,
        ErrorKind::InvalidInput,
        "OID4VCI Authorization Code Token Content-Type is too large",
    );

pub(crate) const INVALID_AUTHORIZATION_CODE_TOKEN_CONTENT_TYPE: ErrorContract = ErrorContract::new(
    error_code::INVALID_AUTHORIZATION_CODE_TOKEN_CONTENT_TYPE,
    ErrorKind::InvalidInput,
    "OID4VCI Authorization Code Token Content-Type is invalid",
);

pub(crate) const AUTHORIZATION_CODE_TOKEN_CACHE_CONTROL_TOO_LARGE: ErrorContract =
    ErrorContract::new(
        error_code::AUTHORIZATION_CODE_TOKEN_CACHE_CONTROL_TOO_LARGE,
        ErrorKind::InvalidInput,
        "OID4VCI Authorization Code Token Cache-Control is too large",
    );

pub(crate) const INVALID_AUTHORIZATION_CODE_TOKEN_CACHE_CONTROL: ErrorContract = ErrorContract::new(
    error_code::INVALID_AUTHORIZATION_CODE_TOKEN_CACHE_CONTROL,
    ErrorKind::InvalidInput,
    "OID4VCI Authorization Code Token Cache-Control is invalid",
);

pub(crate) const AUTHORIZATION_CODE_TOKEN_PRAGMA_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::AUTHORIZATION_CODE_TOKEN_PRAGMA_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Authorization Code Token Pragma is too large",
);

pub(crate) const INVALID_AUTHORIZATION_CODE_TOKEN_PRAGMA: ErrorContract = ErrorContract::new(
    error_code::INVALID_AUTHORIZATION_CODE_TOKEN_PRAGMA,
    ErrorKind::InvalidInput,
    "OID4VCI Authorization Code Token Pragma is invalid",
);
