use super::ErrorContract;
use crate::error::error_code;

pub(crate) const INVALID_CANDIDATE_CLAIMS: ErrorContract = ErrorContract::new(
    error_code::INVALID_CANDIDATE_CLAIMS,
    "presentation candidate claim collection is invalid",
);
pub(crate) const DUPLICATE_CANDIDATE_CLAIM: ErrorContract = ErrorContract::new(
    error_code::DUPLICATE_CANDIDATE_CLAIM,
    "presentation candidate repeats a claim path",
);
pub(crate) const INVALID_CANDIDATES: ErrorContract = ErrorContract::new(
    error_code::INVALID_CANDIDATES,
    "presentation candidate collection is invalid",
);
pub(crate) const DUPLICATE_CANDIDATE: ErrorContract = ErrorContract::new(
    error_code::DUPLICATE_CANDIDATE,
    "presentation candidate is repeated",
);
pub(crate) const UNKNOWN_CANDIDATE_QUERY: ErrorContract = ErrorContract::new(
    error_code::UNKNOWN_CANDIDATE_QUERY,
    "presentation candidate references an unknown query",
);
pub(crate) const CANDIDATE_FORMAT_MISMATCH: ErrorContract = ErrorContract::new(
    error_code::CANDIDATE_FORMAT_MISMATCH,
    "presentation candidate format does not match its query",
);
pub(crate) const CANDIDATE_UNREQUESTED_CLAIM: ErrorContract = ErrorContract::new(
    error_code::CANDIDATE_UNREQUESTED_CLAIM,
    "presentation candidate contains an unrequested claim",
);
pub(crate) const CANDIDATE_MISSING_REQUIRED_CLAIM: ErrorContract = ErrorContract::new(
    error_code::CANDIDATE_MISSING_REQUIRED_CLAIM,
    "presentation candidate omits a required claim",
);
pub(crate) const CANDIDATE_REQUEST_MISMATCH: ErrorContract = ErrorContract::new(
    error_code::CANDIDATE_REQUEST_MISMATCH,
    "presentation candidate set belongs to a different request",
);
