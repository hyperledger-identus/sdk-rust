use identus_core::ErrorKind;

use super::ErrorContract;
use crate::error::error_code;

pub(crate) const INVALID_TRANSACTION_CODE_INPUT_LIMITS: ErrorContract = ErrorContract::new(
    error_code::INVALID_TRANSACTION_CODE_INPUT_LIMITS,
    ErrorKind::InvalidInput,
    "OID4VCI Transaction Code input limits are invalid",
);

pub(crate) const TRANSACTION_CODE_INPUT_REQUIRED: ErrorContract = ErrorContract::new(
    error_code::TRANSACTION_CODE_INPUT_REQUIRED,
    ErrorKind::InvalidInput,
    "OID4VCI Transaction Code input is required",
);

pub(crate) const TRANSACTION_CODE_INPUT_UNEXPECTED: ErrorContract = ErrorContract::new(
    error_code::TRANSACTION_CODE_INPUT_UNEXPECTED,
    ErrorKind::InvalidInput,
    "OID4VCI Transaction Code input is unexpected",
);

pub(crate) const TRANSACTION_CODE_INPUT_EMPTY: ErrorContract = ErrorContract::new(
    error_code::TRANSACTION_CODE_INPUT_EMPTY,
    ErrorKind::InvalidInput,
    "OID4VCI Transaction Code input is empty",
);

pub(crate) const TRANSACTION_CODE_INPUT_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::TRANSACTION_CODE_INPUT_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Transaction Code input is too large",
);

pub(crate) const INVALID_PRE_AUTHORIZED_TOKEN_REQUEST_LIMITS: ErrorContract = ErrorContract::new(
    error_code::INVALID_PRE_AUTHORIZED_TOKEN_REQUEST_LIMITS,
    ErrorKind::InvalidInput,
    "OID4VCI Pre-Authorized Token Request limits are invalid",
);

pub(crate) const PRE_AUTHORIZED_TOKEN_REQUEST_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::PRE_AUTHORIZED_TOKEN_REQUEST_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Pre-Authorized Token Request is too large",
);

pub(crate) const INVALID_TOKEN_RESPONSE_LIMITS: ErrorContract = ErrorContract::new(
    error_code::INVALID_TOKEN_RESPONSE_LIMITS,
    ErrorKind::InvalidInput,
    "OID4VCI Token Response limits are invalid",
);

pub(crate) const TOKEN_RESPONSE_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::TOKEN_RESPONSE_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Token Response is too large",
);

pub(crate) const INVALID_TOKEN_RESPONSE: ErrorContract = ErrorContract::new(
    error_code::INVALID_TOKEN_RESPONSE,
    ErrorKind::InvalidInput,
    "OID4VCI Token Response core is invalid",
);

pub(crate) const INVALID_ACCESS_TOKEN: ErrorContract = ErrorContract::new(
    error_code::INVALID_ACCESS_TOKEN,
    ErrorKind::InvalidInput,
    "OID4VCI access token is invalid",
);

pub(crate) const ACCESS_TOKEN_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::ACCESS_TOKEN_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI access token is too large",
);

pub(crate) const INVALID_TOKEN_TYPE: ErrorContract = ErrorContract::new(
    error_code::INVALID_TOKEN_TYPE,
    ErrorKind::InvalidInput,
    "OID4VCI token type is invalid",
);

pub(crate) const TOKEN_TYPE_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::TOKEN_TYPE_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI token type is too large",
);

pub(crate) const INVALID_TOKEN_EXPIRES_IN: ErrorContract = ErrorContract::new(
    error_code::INVALID_TOKEN_EXPIRES_IN,
    ErrorKind::InvalidInput,
    "OID4VCI token expiry is invalid",
);

pub(crate) const INVALID_REFRESH_TOKEN: ErrorContract = ErrorContract::new(
    error_code::INVALID_REFRESH_TOKEN,
    ErrorKind::InvalidInput,
    "OID4VCI refresh token is invalid",
);

pub(crate) const REFRESH_TOKEN_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::REFRESH_TOKEN_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI refresh token is too large",
);

pub(crate) const INVALID_TOKEN_SCOPE: ErrorContract = ErrorContract::new(
    error_code::INVALID_TOKEN_SCOPE,
    ErrorKind::InvalidInput,
    "OID4VCI token scope is invalid",
);

pub(crate) const TOKEN_SCOPE_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::TOKEN_SCOPE_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI token scope is too large",
);

pub(crate) const INVALID_TOKEN_AUTHORIZATION_DETAILS_LIMITS: ErrorContract = ErrorContract::new(
    error_code::INVALID_TOKEN_AUTHORIZATION_DETAILS_LIMITS,
    ErrorKind::InvalidInput,
    "OID4VCI Token Response Authorization Details limits are invalid",
);

pub(crate) const INVALID_TOKEN_AUTHORIZATION_DETAILS: ErrorContract = ErrorContract::new(
    error_code::INVALID_TOKEN_AUTHORIZATION_DETAILS,
    ErrorKind::InvalidInput,
    "OID4VCI Token Response Authorization Details are invalid",
);

pub(crate) const TOO_MANY_TOKEN_AUTHORIZATION_DETAILS: ErrorContract = ErrorContract::new(
    error_code::TOO_MANY_TOKEN_AUTHORIZATION_DETAILS,
    ErrorKind::InvalidInput,
    "OID4VCI Token Response has too many Authorization Details",
);

pub(crate) const TOKEN_AUTHORIZATION_DETAIL_VALUE_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::TOKEN_AUTHORIZATION_DETAIL_VALUE_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Token Response Authorization Details value is too large",
);

pub(crate) const TOO_MANY_CREDENTIAL_IDENTIFIERS: ErrorContract = ErrorContract::new(
    error_code::TOO_MANY_CREDENTIAL_IDENTIFIERS,
    ErrorKind::InvalidInput,
    "OID4VCI Token Response has too many Credential identifiers",
);

pub(crate) const DUPLICATE_CREDENTIAL_IDENTIFIER: ErrorContract = ErrorContract::new(
    error_code::DUPLICATE_CREDENTIAL_IDENTIFIER,
    ErrorKind::InvalidInput,
    "OID4VCI Token Response has a duplicate Credential identifier",
);

pub(crate) const INVALID_TOKEN_ERROR_RESPONSE_LIMITS: ErrorContract = ErrorContract::new(
    error_code::INVALID_TOKEN_ERROR_RESPONSE_LIMITS,
    ErrorKind::InvalidInput,
    "OID4VCI Token Error Response limits are invalid",
);

pub(crate) const TOKEN_ERROR_RESPONSE_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::TOKEN_ERROR_RESPONSE_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Token Error Response is too large",
);

pub(crate) const INVALID_TOKEN_ERROR_RESPONSE: ErrorContract = ErrorContract::new(
    error_code::INVALID_TOKEN_ERROR_RESPONSE,
    ErrorKind::InvalidInput,
    "OID4VCI Token Error Response core is invalid",
);

pub(crate) const INVALID_TOKEN_ENDPOINT_ERROR_CODE: ErrorContract = ErrorContract::new(
    error_code::INVALID_TOKEN_ENDPOINT_ERROR_CODE,
    ErrorKind::InvalidInput,
    "OID4VCI token endpoint error code is invalid",
);

pub(crate) const TOKEN_ENDPOINT_ERROR_CODE_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::TOKEN_ENDPOINT_ERROR_CODE_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI token endpoint error code is too large",
);

pub(crate) const INVALID_TOKEN_ERROR_DESCRIPTION: ErrorContract = ErrorContract::new(
    error_code::INVALID_TOKEN_ERROR_DESCRIPTION,
    ErrorKind::InvalidInput,
    "OID4VCI Token Error Response description is invalid",
);

pub(crate) const TOKEN_ERROR_DESCRIPTION_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::TOKEN_ERROR_DESCRIPTION_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Token Error Response description is too large",
);

pub(crate) const INVALID_TOKEN_ERROR_URI: ErrorContract = ErrorContract::new(
    error_code::INVALID_TOKEN_ERROR_URI,
    ErrorKind::InvalidInput,
    "OID4VCI Token Error Response URI is invalid",
);

pub(crate) const TOKEN_ERROR_URI_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::TOKEN_ERROR_URI_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Token Error Response URI is too large",
);
