use identus_core::ErrorKind;

use super::ErrorContract;
use crate::error::error_code;

pub(crate) const INVALID_CREDENTIAL_ERROR_RESPONSE_LIMITS: ErrorContract = ErrorContract::new(
    error_code::INVALID_CREDENTIAL_ERROR_RESPONSE_LIMITS,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Error Response limits are invalid",
);

pub(crate) const CREDENTIAL_ERROR_RESPONSE_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::CREDENTIAL_ERROR_RESPONSE_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Error Response is too large",
);

pub(crate) const INVALID_CREDENTIAL_ERROR_RESPONSE: ErrorContract = ErrorContract::new(
    error_code::INVALID_CREDENTIAL_ERROR_RESPONSE,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Error Response core is invalid",
);

pub(crate) const INVALID_CREDENTIAL_ENDPOINT_ERROR_CODE: ErrorContract = ErrorContract::new(
    error_code::INVALID_CREDENTIAL_ENDPOINT_ERROR_CODE,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Endpoint error code is invalid",
);

pub(crate) const CREDENTIAL_ENDPOINT_ERROR_CODE_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::CREDENTIAL_ENDPOINT_ERROR_CODE_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Endpoint error code is too large",
);

pub(crate) const INVALID_CREDENTIAL_ERROR_DESCRIPTION: ErrorContract = ErrorContract::new(
    error_code::INVALID_CREDENTIAL_ERROR_DESCRIPTION,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Error Response description is invalid",
);

pub(crate) const CREDENTIAL_ERROR_DESCRIPTION_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::CREDENTIAL_ERROR_DESCRIPTION_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Error Response description is too large",
);

pub(crate) const INVALID_CREDENTIAL_ERROR_HTTP_RESPONSE_LIMITS: ErrorContract = ErrorContract::new(
    error_code::INVALID_CREDENTIAL_ERROR_HTTP_RESPONSE_LIMITS,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Error HTTP response limits are invalid",
);

pub(crate) const INVALID_CREDENTIAL_ERROR_HTTP_STATUS: ErrorContract = ErrorContract::new(
    error_code::INVALID_CREDENTIAL_ERROR_HTTP_STATUS,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Error HTTP status is invalid",
);

pub(crate) const CREDENTIAL_ERROR_CONTENT_TYPE_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::CREDENTIAL_ERROR_CONTENT_TYPE_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Error Content-Type is too large",
);

pub(crate) const INVALID_CREDENTIAL_ERROR_CONTENT_TYPE: ErrorContract = ErrorContract::new(
    error_code::INVALID_CREDENTIAL_ERROR_CONTENT_TYPE,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Error Content-Type is invalid",
);

pub(crate) const GENERIC_CREDENTIAL_ERROR_CODE_FORBIDDEN: ErrorContract = ErrorContract::new(
    error_code::GENERIC_CREDENTIAL_ERROR_CODE_FORBIDDEN,
    ErrorKind::InvalidInput,
    "OID4VCI Credential payload error uses a forbidden generic code",
);

pub(crate) const INVALID_CREDENTIAL_NONCE_RESPONSE_LIMITS: ErrorContract = ErrorContract::new(
    error_code::INVALID_CREDENTIAL_NONCE_RESPONSE_LIMITS,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Nonce Response limits are invalid",
);

pub(crate) const CREDENTIAL_NONCE_RESPONSE_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::CREDENTIAL_NONCE_RESPONSE_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Nonce Response is too large",
);

pub(crate) const INVALID_CREDENTIAL_NONCE_RESPONSE: ErrorContract = ErrorContract::new(
    error_code::INVALID_CREDENTIAL_NONCE_RESPONSE,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Nonce Response core is invalid",
);

pub(crate) const INVALID_CREDENTIAL_NONCE: ErrorContract = ErrorContract::new(
    error_code::INVALID_CREDENTIAL_NONCE,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Nonce is invalid",
);

pub(crate) const CREDENTIAL_NONCE_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::CREDENTIAL_NONCE_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Nonce is too large",
);

pub(crate) const INVALID_CREDENTIAL_NONCE_HTTP_RESPONSE_LIMITS: ErrorContract = ErrorContract::new(
    error_code::INVALID_CREDENTIAL_NONCE_HTTP_RESPONSE_LIMITS,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Nonce HTTP response limits are invalid",
);

pub(crate) const INVALID_CREDENTIAL_NONCE_HTTP_STATUS: ErrorContract = ErrorContract::new(
    error_code::INVALID_CREDENTIAL_NONCE_HTTP_STATUS,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Nonce HTTP status is invalid",
);

pub(crate) const CREDENTIAL_NONCE_CONTENT_TYPE_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::CREDENTIAL_NONCE_CONTENT_TYPE_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Nonce Content-Type is too large",
);

pub(crate) const INVALID_CREDENTIAL_NONCE_CONTENT_TYPE: ErrorContract = ErrorContract::new(
    error_code::INVALID_CREDENTIAL_NONCE_CONTENT_TYPE,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Nonce Content-Type is invalid",
);

pub(crate) const CREDENTIAL_NONCE_CACHE_CONTROL_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::CREDENTIAL_NONCE_CACHE_CONTROL_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Nonce Cache-Control is too large",
);

pub(crate) const INVALID_CREDENTIAL_NONCE_CACHE_CONTROL: ErrorContract = ErrorContract::new(
    error_code::INVALID_CREDENTIAL_NONCE_CACHE_CONTROL,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Nonce Cache-Control is invalid",
);

pub(crate) const INVALID_JWT_CREDENTIAL_REQUEST_LIMITS: ErrorContract = ErrorContract::new(
    error_code::INVALID_JWT_CREDENTIAL_REQUEST_LIMITS,
    ErrorKind::InvalidInput,
    "OID4VCI JWT Credential Request limits are invalid",
);

pub(crate) const CREDENTIAL_REQUEST_CONFIGURATION_MISSING: ErrorContract = ErrorContract::new(
    error_code::CREDENTIAL_REQUEST_CONFIGURATION_MISSING,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Request configuration is not offered",
);

pub(crate) const CREDENTIAL_REQUEST_AUTHORIZATION_DETAIL_MISSING: ErrorContract =
    ErrorContract::new(
        error_code::CREDENTIAL_REQUEST_AUTHORIZATION_DETAIL_MISSING,
        ErrorKind::InvalidInput,
        "OID4VCI Credential Request Authorization Detail is missing",
    );

pub(crate) const CREDENTIAL_REQUEST_IDENTIFIER_MISSING: ErrorContract = ErrorContract::new(
    error_code::CREDENTIAL_REQUEST_IDENTIFIER_MISSING,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Request identifier is missing",
);

pub(crate) const CREDENTIAL_REQUEST_AUTHORIZATION_CONFIGURATION_MISMATCH: ErrorContract =
    ErrorContract::new(
        error_code::CREDENTIAL_REQUEST_AUTHORIZATION_CONFIGURATION_MISMATCH,
        ErrorKind::InvalidInput,
        "OID4VCI Credential Request authorization configuration is not offered",
    );

pub(crate) const CREDENTIAL_REQUEST_AUTHORIZATION_DETAILS_UNSUPPORTED: ErrorContract =
    ErrorContract::new(
        error_code::CREDENTIAL_REQUEST_AUTHORIZATION_DETAILS_UNSUPPORTED,
        ErrorKind::Unsupported,
        "OID4VCI Credential Request Authorization Details are unsupported",
    );

pub(crate) const CREDENTIAL_REQUEST_TOKEN_TYPE_UNSUPPORTED: ErrorContract = ErrorContract::new(
    error_code::CREDENTIAL_REQUEST_TOKEN_TYPE_UNSUPPORTED,
    ErrorKind::Unsupported,
    "OID4VCI Credential Request token type is unsupported",
);

pub(crate) const INVALID_CREDENTIAL_REQUEST_BEARER_TOKEN: ErrorContract = ErrorContract::new(
    error_code::INVALID_CREDENTIAL_REQUEST_BEARER_TOKEN,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Request Bearer token is invalid",
);

pub(crate) const CREDENTIAL_REQUEST_PROOFS_REQUIRED: ErrorContract = ErrorContract::new(
    error_code::CREDENTIAL_REQUEST_PROOFS_REQUIRED,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Request requires JWT proofs",
);

pub(crate) const TOO_MANY_CREDENTIAL_REQUEST_PROOFS: ErrorContract = ErrorContract::new(
    error_code::TOO_MANY_CREDENTIAL_REQUEST_PROOFS,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Request has too many JWT proofs",
);

pub(crate) const CREDENTIAL_REQUEST_PROOF_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::CREDENTIAL_REQUEST_PROOF_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Request JWT proof is too large",
);

pub(crate) const CREDENTIAL_REQUEST_AUTHORIZATION_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::CREDENTIAL_REQUEST_AUTHORIZATION_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Request Authorization value is too large",
);

pub(crate) const CREDENTIAL_REQUEST_BODY_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::CREDENTIAL_REQUEST_BODY_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Request body is too large",
);
