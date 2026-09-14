use super::{ErrorContract, invalid_input};
use crate::error::error_code;

pub(crate) const INVALID_ENTITY_IDENTIFIER: ErrorContract = invalid_input(
    error_code::INVALID_ENTITY_IDENTIFIER,
    "credential entity identifier is invalid",
    "invalid credential entity identifier",
);
pub(crate) const INVALID_CREDENTIAL_TYPE: ErrorContract = invalid_input(
    error_code::INVALID_CREDENTIAL_TYPE,
    "credential type is invalid",
    "invalid credential type",
);
pub(crate) const INVALID_SCHEMA_IDENTIFIER: ErrorContract = invalid_input(
    error_code::INVALID_SCHEMA_IDENTIFIER,
    "credential schema identifier is invalid",
    "invalid credential schema identifier",
);
pub(crate) const INVALID_SCHEMA_VERSION: ErrorContract = invalid_input(
    error_code::INVALID_SCHEMA_VERSION,
    "credential schema version is invalid",
    "invalid credential schema version",
);
pub(crate) const INVALID_CLAIM_IDENTIFIER: ErrorContract = invalid_input(
    error_code::INVALID_CLAIM_IDENTIFIER,
    "credential claim identifier is invalid",
    "invalid credential claim identifier",
);
pub(crate) const INVALID_CLAIM_VALUE_TYPE: ErrorContract = invalid_input(
    error_code::INVALID_CLAIM_VALUE_TYPE,
    "credential claim value type is invalid",
    "invalid credential claim value type",
);
pub(crate) const INVALID_CLAIM_PATH_SEGMENT: ErrorContract = invalid_input(
    error_code::INVALID_CLAIM_PATH_SEGMENT,
    "credential claim path segment is invalid",
    "invalid credential claim path segment",
);
pub(crate) const INVALID_CLAIM_PATH: ErrorContract = invalid_input(
    error_code::INVALID_CLAIM_PATH,
    "credential claim path is invalid",
    "invalid credential claim path",
);
pub(crate) const INVALID_CLAIM_DISCLOSURE: ErrorContract = invalid_input(
    error_code::INVALID_CLAIM_DISCLOSURE,
    "credential claim disclosure is invalid",
    "invalid credential claim disclosure",
);
pub(crate) const INVALID_DESCRIPTOR_COLLECTION: ErrorContract = invalid_input(
    error_code::INVALID_DESCRIPTOR_COLLECTION,
    "credential descriptor collection is invalid",
    "invalid credential descriptor collection",
);
pub(crate) const DUPLICATE_CREDENTIAL_SUBJECT: ErrorContract = invalid_input(
    error_code::DUPLICATE_CREDENTIAL_SUBJECT,
    "credential subject identifier is duplicated",
    "credential subject identifier is duplicated",
);
pub(crate) const DUPLICATE_CREDENTIAL_TYPE: ErrorContract = invalid_input(
    error_code::DUPLICATE_CREDENTIAL_TYPE,
    "credential type is duplicated",
    "credential type is duplicated",
);
pub(crate) const DUPLICATE_SCHEMA_IDENTIFIER: ErrorContract = invalid_input(
    error_code::DUPLICATE_SCHEMA_IDENTIFIER,
    "credential schema identifier is duplicated",
    "credential schema identifier is duplicated",
);
pub(crate) const DUPLICATE_CLAIM_IDENTIFIER: ErrorContract = invalid_input(
    error_code::DUPLICATE_CLAIM_IDENTIFIER,
    "credential claim identifier is duplicated",
    "credential claim identifier is duplicated",
);
pub(crate) const DUPLICATE_CLAIM_PATH: ErrorContract = invalid_input(
    error_code::DUPLICATE_CLAIM_PATH,
    "credential claim path is duplicated",
    "credential claim path is duplicated",
);
pub(crate) const INVALID_VALIDITY_RANGE: ErrorContract = invalid_input(
    error_code::INVALID_VALIDITY_RANGE,
    "credential validity range is invalid",
    "credential validity range is invalid",
);
