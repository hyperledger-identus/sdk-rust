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
    INVALID_PRE_AUTHORIZED_TOKEN_HTTP_RESPONSE_LIMITS,
    INVALID_PRE_AUTHORIZED_TOKEN_HTTP_RESPONSE_LIMITS,
    "OID4VCI Pre-Authorized Token HTTP response limits are invalid"
);
contract!(
    INVALID_PRE_AUTHORIZED_TOKEN_HTTP_STATUS,
    INVALID_PRE_AUTHORIZED_TOKEN_HTTP_STATUS,
    "OID4VCI Pre-Authorized Token HTTP status is invalid"
);
contract!(
    PRE_AUTHORIZED_TOKEN_CONTENT_TYPE_TOO_LARGE,
    PRE_AUTHORIZED_TOKEN_CONTENT_TYPE_TOO_LARGE,
    "OID4VCI Pre-Authorized Token Content-Type is too large"
);
contract!(
    INVALID_PRE_AUTHORIZED_TOKEN_CONTENT_TYPE,
    INVALID_PRE_AUTHORIZED_TOKEN_CONTENT_TYPE,
    "OID4VCI Pre-Authorized Token Content-Type is invalid"
);
contract!(
    PRE_AUTHORIZED_TOKEN_CACHE_CONTROL_TOO_LARGE,
    PRE_AUTHORIZED_TOKEN_CACHE_CONTROL_TOO_LARGE,
    "OID4VCI Pre-Authorized Token Cache-Control is too large"
);
contract!(
    INVALID_PRE_AUTHORIZED_TOKEN_CACHE_CONTROL,
    INVALID_PRE_AUTHORIZED_TOKEN_CACHE_CONTROL,
    "OID4VCI Pre-Authorized Token Cache-Control is invalid"
);
contract!(
    PRE_AUTHORIZED_TOKEN_PRAGMA_TOO_LARGE,
    PRE_AUTHORIZED_TOKEN_PRAGMA_TOO_LARGE,
    "OID4VCI Pre-Authorized Token Pragma is too large"
);
contract!(
    INVALID_PRE_AUTHORIZED_TOKEN_PRAGMA,
    INVALID_PRE_AUTHORIZED_TOKEN_PRAGMA,
    "OID4VCI Pre-Authorized Token Pragma is invalid"
);
