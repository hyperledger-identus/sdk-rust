use super::ErrorContract;
use crate::error::error_code;

pub(crate) const INVALID_SELECTION_CLAIMS: ErrorContract = ErrorContract::new(
    error_code::INVALID_SELECTION_CLAIMS,
    "presentation selection claim collection is invalid",
);
pub(crate) const DUPLICATE_SELECTION_CLAIM: ErrorContract = ErrorContract::new(
    error_code::DUPLICATE_SELECTION_CLAIM,
    "presentation selection repeats a claim path",
);
pub(crate) const INVALID_DISCLOSURE_SELECTIONS: ErrorContract = ErrorContract::new(
    error_code::INVALID_DISCLOSURE_SELECTIONS,
    "presentation disclosure selection collection is invalid",
);
pub(crate) const DUPLICATE_DISCLOSURE_SELECTION: ErrorContract = ErrorContract::new(
    error_code::DUPLICATE_DISCLOSURE_SELECTION,
    "presentation disclosure repeats a credential selection",
);
pub(crate) const UNKNOWN_SELECTION_QUERY: ErrorContract = ErrorContract::new(
    error_code::UNKNOWN_SELECTION_QUERY,
    "presentation selection references an unknown query",
);
pub(crate) const UNKNOWN_SELECTION_CANDIDATE: ErrorContract = ErrorContract::new(
    error_code::UNKNOWN_SELECTION_CANDIDATE,
    "presentation selection references an unknown candidate",
);
pub(crate) const SELECTION_UNREQUESTED_CLAIM: ErrorContract = ErrorContract::new(
    error_code::SELECTION_UNREQUESTED_CLAIM,
    "presentation selection contains an unrequested claim",
);
pub(crate) const SELECTION_CLAIM_INTENT_MISMATCH: ErrorContract = ErrorContract::new(
    error_code::SELECTION_CLAIM_INTENT_MISMATCH,
    "presentation selection claim intent does not match its query",
);
pub(crate) const SELECTION_UNAVAILABLE_CLAIM: ErrorContract = ErrorContract::new(
    error_code::SELECTION_UNAVAILABLE_CLAIM,
    "presentation selection contains an unavailable claim",
);
pub(crate) const SELECTION_MISSING_REQUIRED_CLAIM: ErrorContract = ErrorContract::new(
    error_code::SELECTION_MISSING_REQUIRED_CLAIM,
    "presentation selection omits a required claim",
);
pub(crate) const MISSING_QUERY_SELECTION: ErrorContract = ErrorContract::new(
    error_code::MISSING_QUERY_SELECTION,
    "presentation disclosure omits a query selection",
);
pub(crate) const QUERY_MULTIPLICITY_EXCEEDED: ErrorContract = ErrorContract::new(
    error_code::QUERY_MULTIPLICITY_EXCEEDED,
    "presentation disclosure exceeds query multiplicity",
);
pub(crate) const DISCLOSURE_REQUEST_MISMATCH: ErrorContract = ErrorContract::new(
    error_code::DISCLOSURE_REQUEST_MISMATCH,
    "presentation disclosure plan belongs to a different request",
);
