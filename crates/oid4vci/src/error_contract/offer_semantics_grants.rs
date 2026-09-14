use identus_core::ErrorKind;

use super::ErrorContract;
use crate::error::error_code;

pub(crate) const INVALID_SEMANTIC_LIMITS: ErrorContract = ErrorContract::new(
    error_code::INVALID_SEMANTIC_LIMITS,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Offer semantic limits are invalid",
);

pub(crate) const INVALID_OFFER_FIELDS: ErrorContract = ErrorContract::new(
    error_code::INVALID_OFFER_FIELDS,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Offer required fields are invalid",
);

pub(crate) const ISSUER_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::ISSUER_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Issuer Identifier is too large",
);

pub(crate) const UNSAFE_CREDENTIAL_ISSUER: ErrorContract = ErrorContract::new(
    error_code::UNSAFE_CREDENTIAL_ISSUER,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Issuer Identifier is unsafe",
);

pub(crate) const INVALID_CONFIGURATION_IDS: ErrorContract = ErrorContract::new(
    error_code::INVALID_CONFIGURATION_IDS,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Configuration IDs are invalid",
);

pub(crate) const CONFIGURATION_ID_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::CONFIGURATION_ID_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Configuration ID is too large",
);

pub(crate) const TOO_MANY_CONFIGURATION_IDS: ErrorContract = ErrorContract::new(
    error_code::TOO_MANY_CONFIGURATION_IDS,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Offer has too many configuration IDs",
);

pub(crate) const DUPLICATE_CONFIGURATION_ID: ErrorContract = ErrorContract::new(
    error_code::DUPLICATE_CONFIGURATION_ID,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Offer repeats a configuration ID",
);

pub(crate) const INVALID_GRANTS: ErrorContract = ErrorContract::new(
    error_code::INVALID_GRANTS,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Offer grants are invalid",
);

pub(crate) const INVALID_GRANT_LIMITS: ErrorContract = ErrorContract::new(
    error_code::INVALID_GRANT_LIMITS,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Offer grant limits are invalid",
);

pub(crate) const INVALID_AUTHORIZATION_CODE_GRANT: ErrorContract = ErrorContract::new(
    error_code::INVALID_AUTHORIZATION_CODE_GRANT,
    ErrorKind::InvalidInput,
    "OID4VCI Authorization Code grant is invalid",
);

pub(crate) const INVALID_PRE_AUTHORIZED_CODE_GRANT: ErrorContract = ErrorContract::new(
    error_code::INVALID_PRE_AUTHORIZED_CODE_GRANT,
    ErrorKind::InvalidInput,
    "OID4VCI Pre-Authorized Code grant is invalid",
);

pub(crate) const ISSUER_STATE_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::ISSUER_STATE_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI issuer state is too large",
);

pub(crate) const PRE_AUTHORIZED_CODE_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::PRE_AUTHORIZED_CODE_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Pre-Authorized Code is too large",
);

pub(crate) const AUTHORIZATION_SERVER_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::AUTHORIZATION_SERVER_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Authorization Server identifier is too large",
);

pub(crate) const UNSAFE_AUTHORIZATION_SERVER: ErrorContract = ErrorContract::new(
    error_code::UNSAFE_AUTHORIZATION_SERVER,
    ErrorKind::InvalidInput,
    "OID4VCI Authorization Server identifier is unsafe",
);

pub(crate) const INVALID_TRANSACTION_CODE: ErrorContract = ErrorContract::new(
    error_code::INVALID_TRANSACTION_CODE,
    ErrorKind::InvalidInput,
    "OID4VCI Transaction Code requirements are invalid",
);

pub(crate) const INVALID_TRANSACTION_CODE_MODE: ErrorContract = ErrorContract::new(
    error_code::INVALID_TRANSACTION_CODE_MODE,
    ErrorKind::InvalidInput,
    "OID4VCI Transaction Code input mode is invalid",
);

pub(crate) const INVALID_TRANSACTION_CODE_LENGTH: ErrorContract = ErrorContract::new(
    error_code::INVALID_TRANSACTION_CODE_LENGTH,
    ErrorKind::InvalidInput,
    "OID4VCI Transaction Code length is invalid",
);

pub(crate) const TRANSACTION_CODE_LENGTH_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::TRANSACTION_CODE_LENGTH_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Transaction Code length is too large",
);

pub(crate) const TRANSACTION_CODE_DESCRIPTION_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::TRANSACTION_CODE_DESCRIPTION_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Transaction Code description is too large",
);
