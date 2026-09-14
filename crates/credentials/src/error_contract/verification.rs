use super::{ErrorContract, invalid_input};
use crate::error::error_code;

pub(crate) const INVALID_VERIFICATION_STAGE_NAME: ErrorContract = invalid_input(
    error_code::INVALID_VERIFICATION_STAGE_NAME,
    "credential verification stage name is invalid",
    "invalid credential verification stage name",
);
pub(crate) const INVALID_VERIFICATION_REASON_CODE: ErrorContract = invalid_input(
    error_code::INVALID_VERIFICATION_REASON_CODE,
    "credential verification reason code is invalid",
    "invalid credential verification reason code",
);
pub(crate) const MISSING_VERIFICATION_REASON: ErrorContract = invalid_input(
    error_code::MISSING_VERIFICATION_REASON,
    "credential verification reason is required",
    "credential verification reason is required",
);
pub(crate) const UNEXPECTED_VERIFICATION_REASON: ErrorContract = invalid_input(
    error_code::UNEXPECTED_VERIFICATION_REASON,
    "credential verification reason is not allowed",
    "credential verification reason is not allowed",
);
pub(crate) const NON_CANONICAL_VERIFICATION_REPORT: ErrorContract = invalid_input(
    error_code::NON_CANONICAL_VERIFICATION_REPORT,
    "credential verification report is not canonical",
    "credential verification report is not canonical",
);
pub(crate) const DUPLICATE_CREDENTIAL_VERIFIER_FORMAT: ErrorContract = invalid_input(
    error_code::DUPLICATE_CREDENTIAL_VERIFIER_FORMAT,
    "credential verifier format is duplicated",
    "credential verifier format is duplicated",
);
pub(crate) const TOO_MANY_CREDENTIAL_VERIFIER_FORMATS: ErrorContract = invalid_input(
    error_code::TOO_MANY_CREDENTIAL_VERIFIER_FORMATS,
    "credential verifier registry exceeds the format limit",
    "credential verifier registry exceeds the format limit",
);
