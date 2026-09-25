use identus_core::ErrorKind;

use super::ErrorContract;
use crate::error::error_code;

macro_rules! contract {
    ($name:ident, $code:ident, $message:literal) => {
        pub(crate) const $name: ErrorContract =
            ErrorContract::new(error_code::$code, ErrorKind::InvalidInput, $message);
    };
}

contract!(
    INVALID_AUTHORIZATION_RESPONSE_LIMITS,
    INVALID_AUTHORIZATION_RESPONSE_LIMITS,
    "OID4VCI Authorization Response limits are invalid"
);
contract!(
    AUTHORIZATION_RESPONSE_TOO_LARGE,
    AUTHORIZATION_RESPONSE_TOO_LARGE,
    "OID4VCI Authorization Response is too large"
);
contract!(
    TOO_MANY_AUTHORIZATION_RESPONSE_PARAMETERS,
    TOO_MANY_AUTHORIZATION_RESPONSE_PARAMETERS,
    "OID4VCI Authorization Response has too many parameters"
);
contract!(
    AUTHORIZATION_RESPONSE_COMPONENT_TOO_LARGE,
    AUTHORIZATION_RESPONSE_COMPONENT_TOO_LARGE,
    "OID4VCI Authorization Response component is too large"
);
contract!(
    INVALID_AUTHORIZATION_RESPONSE_ENCODING,
    INVALID_AUTHORIZATION_RESPONSE_ENCODING,
    "OID4VCI Authorization Response form encoding is invalid"
);
contract!(
    DUPLICATE_AUTHORIZATION_RESPONSE_PARAMETER,
    DUPLICATE_AUTHORIZATION_RESPONSE_PARAMETER,
    "OID4VCI Authorization Response has a duplicate parameter"
);
contract!(
    AUTHORIZATION_RESPONSE_STATE_MISMATCH,
    AUTHORIZATION_RESPONSE_STATE_MISMATCH,
    "OID4VCI Authorization Response state does not match the request"
);
contract!(
    AUTHORIZATION_RESPONSE_ISSUER_MISMATCH,
    AUTHORIZATION_RESPONSE_ISSUER_MISMATCH,
    "OID4VCI Authorization Response issuer does not match selected-server policy"
);
contract!(
    INVALID_AUTHORIZATION_RESPONSE,
    INVALID_AUTHORIZATION_RESPONSE,
    "OID4VCI Authorization Response is invalid"
);
contract!(
    AUTHORIZATION_CODE_TOO_LARGE,
    AUTHORIZATION_CODE_TOO_LARGE,
    "OID4VCI authorization code is too large"
);
contract!(
    INVALID_AUTHORIZATION_CODE,
    INVALID_AUTHORIZATION_CODE,
    "OID4VCI authorization code is invalid"
);
contract!(
    AUTHORIZATION_ENDPOINT_ERROR_CODE_TOO_LARGE,
    AUTHORIZATION_ENDPOINT_ERROR_CODE_TOO_LARGE,
    "OID4VCI Authorization Endpoint error code is too large"
);
contract!(
    INVALID_AUTHORIZATION_ENDPOINT_ERROR_CODE,
    INVALID_AUTHORIZATION_ENDPOINT_ERROR_CODE,
    "OID4VCI Authorization Endpoint error code is invalid"
);
contract!(
    AUTHORIZATION_ERROR_DESCRIPTION_TOO_LARGE,
    AUTHORIZATION_ERROR_DESCRIPTION_TOO_LARGE,
    "OID4VCI Authorization Response error description is too large"
);
contract!(
    INVALID_AUTHORIZATION_ERROR_DESCRIPTION,
    INVALID_AUTHORIZATION_ERROR_DESCRIPTION,
    "OID4VCI Authorization Response error description is invalid"
);
contract!(
    AUTHORIZATION_ERROR_URI_TOO_LARGE,
    AUTHORIZATION_ERROR_URI_TOO_LARGE,
    "OID4VCI Authorization Response error URI is too large"
);
contract!(
    INVALID_AUTHORIZATION_ERROR_URI,
    INVALID_AUTHORIZATION_ERROR_URI,
    "OID4VCI Authorization Response error URI is invalid"
);
