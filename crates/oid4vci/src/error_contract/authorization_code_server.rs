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
