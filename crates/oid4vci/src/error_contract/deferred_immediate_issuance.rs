use identus_core::ErrorKind;

use super::ErrorContract;
use crate::error::error_code;

pub(crate) const INVALID_DEFERRED_CREDENTIAL_REQUEST_LIMITS: ErrorContract = ErrorContract::new(
    error_code::INVALID_DEFERRED_CREDENTIAL_REQUEST_LIMITS,
    ErrorKind::InvalidInput,
    "OID4VCI Deferred Credential Request limits are invalid",
);

pub(crate) const DEFERRED_CREDENTIAL_ENDPOINT_REQUIRED: ErrorContract = ErrorContract::new(
    error_code::DEFERRED_CREDENTIAL_ENDPOINT_REQUIRED,
    ErrorKind::InvalidInput,
    "OID4VCI Deferred Credential Endpoint is required",
);

pub(crate) const DEFERRED_CREDENTIAL_REQUEST_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::DEFERRED_CREDENTIAL_REQUEST_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Deferred Credential Request is too large",
);

pub(crate) const INVALID_DEFERRED_CREDENTIAL_RESPONSE_LIMITS: ErrorContract = ErrorContract::new(
    error_code::INVALID_DEFERRED_CREDENTIAL_RESPONSE_LIMITS,
    ErrorKind::InvalidInput,
    "OID4VCI deferred Credential Response limits are invalid",
);

pub(crate) const DEFERRED_CREDENTIAL_RESPONSE_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::DEFERRED_CREDENTIAL_RESPONSE_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI deferred Credential Response is too large",
);

pub(crate) const INVALID_DEFERRED_CREDENTIAL_RESPONSE: ErrorContract = ErrorContract::new(
    error_code::INVALID_DEFERRED_CREDENTIAL_RESPONSE,
    ErrorKind::InvalidInput,
    "OID4VCI deferred Credential Response core is invalid",
);

pub(crate) const TOO_MANY_DEFERRED_CREDENTIAL_RESPONSE_MEMBERS: ErrorContract = ErrorContract::new(
    error_code::TOO_MANY_DEFERRED_CREDENTIAL_RESPONSE_MEMBERS,
    ErrorKind::InvalidInput,
    "OID4VCI deferred Credential Response has too many members",
);

pub(crate) const INVALID_DEFERRED_TRANSACTION_ID: ErrorContract = ErrorContract::new(
    error_code::INVALID_DEFERRED_TRANSACTION_ID,
    ErrorKind::InvalidInput,
    "OID4VCI deferred transaction identifier is invalid",
);

pub(crate) const DEFERRED_TRANSACTION_ID_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::DEFERRED_TRANSACTION_ID_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI deferred transaction identifier is too large",
);

pub(crate) const INVALID_DEFERRED_CREDENTIAL_INTERVAL: ErrorContract = ErrorContract::new(
    error_code::INVALID_DEFERRED_CREDENTIAL_INTERVAL,
    ErrorKind::InvalidInput,
    "OID4VCI deferred Credential interval is invalid",
);

pub(crate) const DEFERRED_CREDENTIAL_INTERVAL_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::DEFERRED_CREDENTIAL_INTERVAL_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI deferred Credential interval is too large",
);

pub(crate) const DEFERRED_CREDENTIAL_RESPONSE_BRANCH_CONFLICT: ErrorContract = ErrorContract::new(
    error_code::DEFERRED_CREDENTIAL_RESPONSE_BRANCH_CONFLICT,
    ErrorKind::InvalidInput,
    "OID4VCI deferred Credential Response branch is ambiguous",
);

pub(crate) const INVALID_IMMEDIATE_CREDENTIAL_RESPONSE_LIMITS: ErrorContract = ErrorContract::new(
    error_code::INVALID_IMMEDIATE_CREDENTIAL_RESPONSE_LIMITS,
    ErrorKind::InvalidInput,
    "OID4VCI immediate Credential Response limits are invalid",
);

pub(crate) const IMMEDIATE_CREDENTIAL_RESPONSE_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::IMMEDIATE_CREDENTIAL_RESPONSE_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI immediate Credential Response is too large",
);

pub(crate) const INVALID_IMMEDIATE_CREDENTIAL_RESPONSE: ErrorContract = ErrorContract::new(
    error_code::INVALID_IMMEDIATE_CREDENTIAL_RESPONSE,
    ErrorKind::InvalidInput,
    "OID4VCI immediate Credential Response core is invalid",
);

pub(crate) const DEFERRED_CREDENTIAL_RESPONSE_UNSUPPORTED: ErrorContract = ErrorContract::new(
    error_code::DEFERRED_CREDENTIAL_RESPONSE_UNSUPPORTED,
    ErrorKind::Unsupported,
    "OID4VCI deferred Credential Response is unsupported",
);

pub(crate) const TOO_MANY_CREDENTIAL_RESPONSE_MEMBERS: ErrorContract = ErrorContract::new(
    error_code::TOO_MANY_CREDENTIAL_RESPONSE_MEMBERS,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Response has too many members",
);

pub(crate) const TOO_MANY_ISSUED_CREDENTIALS: ErrorContract = ErrorContract::new(
    error_code::TOO_MANY_ISSUED_CREDENTIALS,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Response has too many credentials",
);

pub(crate) const TOO_MANY_ISSUED_CREDENTIAL_MEMBERS: ErrorContract = ErrorContract::new(
    error_code::TOO_MANY_ISSUED_CREDENTIAL_MEMBERS,
    ErrorKind::InvalidInput,
    "OID4VCI issued credential has too many members",
);

pub(crate) const INVALID_ISSUED_CREDENTIAL: ErrorContract = ErrorContract::new(
    error_code::INVALID_ISSUED_CREDENTIAL,
    ErrorKind::InvalidInput,
    "OID4VCI issued credential is invalid",
);

pub(crate) const ISSUED_CREDENTIAL_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::ISSUED_CREDENTIAL_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI issued credential is too large",
);

pub(crate) const ISSUED_CREDENTIALS_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::ISSUED_CREDENTIALS_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI issued credentials are too large",
);

pub(crate) const INVALID_CREDENTIAL_NOTIFICATION_ID: ErrorContract = ErrorContract::new(
    error_code::INVALID_CREDENTIAL_NOTIFICATION_ID,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Response notification identifier is invalid",
);

pub(crate) const CREDENTIAL_NOTIFICATION_ID_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::CREDENTIAL_NOTIFICATION_ID_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Response notification identifier is too large",
);

pub(crate) const INVALID_IMMEDIATE_CREDENTIAL_HTTP_RESPONSE_LIMITS: ErrorContract =
    ErrorContract::new(
        error_code::INVALID_IMMEDIATE_CREDENTIAL_HTTP_RESPONSE_LIMITS,
        ErrorKind::InvalidInput,
        "OID4VCI immediate Credential HTTP response limits are invalid",
    );

pub(crate) const INVALID_IMMEDIATE_CREDENTIAL_HTTP_STATUS: ErrorContract = ErrorContract::new(
    error_code::INVALID_IMMEDIATE_CREDENTIAL_HTTP_STATUS,
    ErrorKind::InvalidInput,
    "OID4VCI immediate Credential HTTP status is invalid",
);

pub(crate) const IMMEDIATE_CREDENTIAL_CONTENT_TYPE_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::IMMEDIATE_CREDENTIAL_CONTENT_TYPE_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI immediate Credential Content-Type is too large",
);

pub(crate) const INVALID_IMMEDIATE_CREDENTIAL_CONTENT_TYPE: ErrorContract = ErrorContract::new(
    error_code::INVALID_IMMEDIATE_CREDENTIAL_CONTENT_TYPE,
    ErrorKind::InvalidInput,
    "OID4VCI immediate Credential Content-Type is invalid",
);

pub(crate) const CREDENTIAL_RESPONSE_EXCEEDS_PROOF_COUNT: ErrorContract = ErrorContract::new(
    error_code::CREDENTIAL_RESPONSE_EXCEEDS_PROOF_COUNT,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Response exceeds request proof count",
);
