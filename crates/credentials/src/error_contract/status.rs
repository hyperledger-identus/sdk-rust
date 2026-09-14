use super::{ErrorContract, invalid_input};
use crate::error::error_code;

pub(crate) const INVALID_STATUS_METHOD: ErrorContract = invalid_input(
    error_code::INVALID_STATUS_METHOD,
    "credential status method is invalid",
    "invalid credential status method",
);
pub(crate) const INVALID_STATUS_PURPOSE: ErrorContract = invalid_input(
    error_code::INVALID_STATUS_PURPOSE,
    "credential status purpose is invalid",
    "invalid credential status purpose",
);
pub(crate) const INVALID_STATUS_REFERENCE: ErrorContract = invalid_input(
    error_code::INVALID_STATUS_REFERENCE,
    "credential status reference is invalid",
    "invalid credential status reference",
);
pub(crate) const INVALID_STATUS_HANDLE: ErrorContract = invalid_input(
    error_code::INVALID_STATUS_HANDLE,
    "credential status handle is invalid",
    "invalid credential status handle",
);
pub(crate) const INVALID_STATUS_REVISION: ErrorContract = invalid_input(
    error_code::INVALID_STATUS_REVISION,
    "credential status revision is invalid",
    "invalid credential status revision",
);
pub(crate) const INVALID_STATUS_VALUE: ErrorContract = invalid_input(
    error_code::INVALID_STATUS_VALUE,
    "credential status value is invalid",
    "invalid credential status value",
);
pub(crate) const INVALID_STATUS_BINDING_COLLECTION: ErrorContract = invalid_input(
    error_code::INVALID_STATUS_BINDING_COLLECTION,
    "credential status binding collection is invalid",
    "invalid credential status binding collection",
);
pub(crate) const DUPLICATE_STATUS_BINDING: ErrorContract = invalid_input(
    error_code::DUPLICATE_STATUS_BINDING,
    "credential status binding is duplicated",
    "credential status binding is duplicated",
);
pub(crate) const INVALID_STATUS_FRESHNESS: ErrorContract = invalid_input(
    error_code::INVALID_STATUS_FRESHNESS,
    "credential status freshness is invalid",
    "invalid credential status freshness",
);
pub(crate) const INVALID_STATUS_REQUIREMENTS: ErrorContract = invalid_input(
    error_code::INVALID_STATUS_REQUIREMENTS,
    "credential status requirements are invalid",
    "invalid credential status requirements",
);
pub(crate) const DUPLICATE_STATUS_METHOD: ErrorContract = invalid_input(
    error_code::DUPLICATE_STATUS_METHOD,
    "credential status method is duplicated",
    "credential status method is duplicated",
);
pub(crate) const DUPLICATE_STATUS_PURPOSE: ErrorContract = invalid_input(
    error_code::DUPLICATE_STATUS_PURPOSE,
    "credential status purpose is duplicated",
    "credential status purpose is duplicated",
);
pub(crate) const INVALID_STATUS_EVIDENCE_RANGE: ErrorContract = invalid_input(
    error_code::INVALID_STATUS_EVIDENCE_RANGE,
    "credential status evidence range is invalid",
    "credential status evidence range is invalid",
);
pub(crate) const STATUS_QUERY_MISMATCH: ErrorContract = invalid_input(
    error_code::STATUS_QUERY_MISMATCH,
    "credential status query binding is not accepted",
    "credential status query binding is not accepted",
);
