use identus_core::ErrorKind;

use super::ErrorContract;
use crate::error::error_code;

pub(crate) const AUTHORIZATION_CODE_TOKEN_CONFIGURATION_MISMATCH: ErrorContract =
    ErrorContract::new(
        error_code::AUTHORIZATION_CODE_TOKEN_CONFIGURATION_MISMATCH,
        ErrorKind::InvalidInput,
        "OID4VCI Authorization Code Token Response configuration does not match the request",
    );

pub(crate) const AMBIGUOUS_AUTHORIZATION_CODE_TOKEN_AUTHORIZATION_DETAILS: ErrorContract =
    ErrorContract::new(
        error_code::AMBIGUOUS_AUTHORIZATION_CODE_TOKEN_AUTHORIZATION_DETAILS,
        ErrorKind::InvalidInput,
        "OID4VCI Authorization Code Token Response Authorization Details are ambiguous",
    );
