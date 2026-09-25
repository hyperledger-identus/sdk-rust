use identus_core::ErrorKind;

use super::ErrorContract;
use crate::error::error_code;

pub(crate) const INVALID_AUTHORIZATION_CODE_TOKEN_REQUEST_LIMITS: ErrorContract =
    ErrorContract::new(
        error_code::INVALID_AUTHORIZATION_CODE_TOKEN_REQUEST_LIMITS,
        ErrorKind::InvalidInput,
        "OID4VCI Authorization Code Token Request limits are invalid",
    );

pub(crate) const AUTHORIZATION_CODE_TOKEN_ENDPOINT_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::AUTHORIZATION_CODE_TOKEN_ENDPOINT_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Authorization Code Token Endpoint is too large",
);

pub(crate) const AUTHORIZATION_CODE_TOKEN_REQUEST_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::AUTHORIZATION_CODE_TOKEN_REQUEST_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Authorization Code Token Request is too large",
);
