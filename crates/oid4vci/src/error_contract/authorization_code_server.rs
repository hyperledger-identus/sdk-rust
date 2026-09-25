use identus_core::ErrorKind;

use super::ErrorContract;
use crate::error::error_code;

pub(crate) const AUTHORIZATION_CODE_GRANT_MISSING: ErrorContract = ErrorContract::new(
    error_code::AUTHORIZATION_CODE_GRANT_MISSING,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Offer has no Authorization Code grant",
);

pub(crate) const AUTHORIZATION_CODE_SERVER_HINT_MISMATCH: ErrorContract = ErrorContract::new(
    error_code::AUTHORIZATION_CODE_SERVER_HINT_MISMATCH,
    ErrorKind::InvalidInput,
    "OID4VCI selected Authorization Server does not match the Authorization Code hint",
);

pub(crate) const AUTHORIZATION_CODE_GRANT_NOT_SUPPORTED: ErrorContract = ErrorContract::new(
    error_code::AUTHORIZATION_CODE_GRANT_NOT_SUPPORTED,
    ErrorKind::InvalidInput,
    "OID4VCI selected Authorization Server does not support the Authorization Code grant",
);

pub(crate) const AUTHORIZATION_ENDPOINT_REQUIRED: ErrorContract = ErrorContract::new(
    error_code::AUTHORIZATION_ENDPOINT_REQUIRED,
    ErrorKind::InvalidInput,
    "OID4VCI selected Authorization Server has no Authorization Endpoint",
);

pub(crate) const INVALID_AUTHORIZATION_REQUEST_INPUT_LIMITS: ErrorContract = ErrorContract::new(
    error_code::INVALID_AUTHORIZATION_REQUEST_INPUT_LIMITS,
    ErrorKind::InvalidInput,
    "OID4VCI Authorization Request input limits are invalid",
);

pub(crate) const AUTHORIZATION_REQUEST_CONFIGURATION_MISSING: ErrorContract = ErrorContract::new(
    error_code::AUTHORIZATION_REQUEST_CONFIGURATION_MISSING,
    ErrorKind::InvalidInput,
    "OID4VCI Authorization Request Credential Configuration is missing",
);

pub(crate) const INVALID_AUTHORIZATION_REQUEST_CLIENT_ID: ErrorContract = ErrorContract::new(
    error_code::INVALID_AUTHORIZATION_REQUEST_CLIENT_ID,
    ErrorKind::InvalidInput,
    "OID4VCI Authorization Request client identifier is invalid",
);

pub(crate) const AUTHORIZATION_REQUEST_CLIENT_ID_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::AUTHORIZATION_REQUEST_CLIENT_ID_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Authorization Request client identifier is too large",
);

pub(crate) const INVALID_AUTHORIZATION_REQUEST_REDIRECT_URI: ErrorContract = ErrorContract::new(
    error_code::INVALID_AUTHORIZATION_REQUEST_REDIRECT_URI,
    ErrorKind::InvalidInput,
    "OID4VCI Authorization Request redirect URI is invalid",
);

pub(crate) const AUTHORIZATION_REQUEST_REDIRECT_URI_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::AUTHORIZATION_REQUEST_REDIRECT_URI_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Authorization Request redirect URI is too large",
);

pub(crate) const INVALID_AUTHORIZATION_REQUEST_STATE: ErrorContract = ErrorContract::new(
    error_code::INVALID_AUTHORIZATION_REQUEST_STATE,
    ErrorKind::InvalidInput,
    "OID4VCI Authorization Request state is invalid",
);

pub(crate) const AUTHORIZATION_REQUEST_STATE_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::AUTHORIZATION_REQUEST_STATE_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Authorization Request state is too large",
);

pub(crate) const INVALID_PKCE_CODE_VERIFIER: ErrorContract = ErrorContract::new(
    error_code::INVALID_PKCE_CODE_VERIFIER,
    ErrorKind::InvalidInput,
    "OID4VCI PKCE code verifier is invalid",
);

pub(crate) const PKCE_CODE_VERIFIER_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::PKCE_CODE_VERIFIER_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI PKCE code verifier is too large",
);

pub(crate) const INVALID_AUTHORIZATION_REQUEST_LIMITS: ErrorContract = ErrorContract::new(
    error_code::INVALID_AUTHORIZATION_REQUEST_LIMITS,
    ErrorKind::InvalidInput,
    "OID4VCI Authorization Request limits are invalid",
);

pub(crate) const AUTHORIZATION_DETAILS_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::AUTHORIZATION_DETAILS_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Authorization Details are too large",
);

pub(crate) const TOO_MANY_AUTHORIZATION_ENDPOINT_QUERY_PARAMETERS: ErrorContract =
    ErrorContract::new(
        error_code::TOO_MANY_AUTHORIZATION_ENDPOINT_QUERY_PARAMETERS,
        ErrorKind::InvalidInput,
        "OID4VCI Authorization Endpoint has too many query parameters",
    );

pub(crate) const AUTHORIZATION_ENDPOINT_QUERY_COMPONENT_TOO_LARGE: ErrorContract =
    ErrorContract::new(
        error_code::AUTHORIZATION_ENDPOINT_QUERY_COMPONENT_TOO_LARGE,
        ErrorKind::InvalidInput,
        "OID4VCI Authorization Endpoint query component is too large",
    );

pub(crate) const INVALID_AUTHORIZATION_ENDPOINT_QUERY: ErrorContract = ErrorContract::new(
    error_code::INVALID_AUTHORIZATION_ENDPOINT_QUERY,
    ErrorKind::InvalidInput,
    "OID4VCI Authorization Endpoint query is invalid",
);

pub(crate) const AUTHORIZATION_ENDPOINT_QUERY_PARAMETER_COLLISION: ErrorContract =
    ErrorContract::new(
        error_code::AUTHORIZATION_ENDPOINT_QUERY_PARAMETER_COLLISION,
        ErrorKind::InvalidInput,
        "OID4VCI Authorization Endpoint query parameter collides with request construction",
    );

pub(crate) const AUTHORIZATION_REQUEST_URI_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::AUTHORIZATION_REQUEST_URI_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Authorization Request URI is too large",
);
