use super::ErrorContract;
use crate::error::error_code;

pub(crate) const INVALID_QUERY_ID: ErrorContract = ErrorContract::new(
    error_code::INVALID_QUERY_ID,
    "presentation query id is invalid",
);
pub(crate) const INVALID_PURPOSE: ErrorContract = ErrorContract::new(
    error_code::INVALID_PURPOSE,
    "presentation purpose is invalid",
);
pub(crate) const INVALID_CHALLENGE: ErrorContract = ErrorContract::new(
    error_code::INVALID_CHALLENGE,
    "presentation challenge is invalid",
);
pub(crate) const INVALID_CREDENTIAL_HANDLE: ErrorContract = ErrorContract::new(
    error_code::INVALID_CREDENTIAL_HANDLE,
    "presentation credential handle is invalid",
);
pub(crate) const INVALID_CLAIM_INTENT: ErrorContract = ErrorContract::new(
    error_code::INVALID_CLAIM_INTENT,
    "presentation claim intent is invalid",
);
pub(crate) const INVALID_QUERY_FILTERS: ErrorContract = ErrorContract::new(
    error_code::INVALID_QUERY_FILTERS,
    "presentation query filters are invalid",
);
pub(crate) const DUPLICATE_ISSUER_FILTER: ErrorContract = ErrorContract::new(
    error_code::DUPLICATE_ISSUER_FILTER,
    "presentation query repeats an issuer filter",
);
pub(crate) const DUPLICATE_TYPE_FILTER: ErrorContract = ErrorContract::new(
    error_code::DUPLICATE_TYPE_FILTER,
    "presentation query repeats a credential type filter",
);
pub(crate) const DUPLICATE_SCHEMA_FILTER: ErrorContract = ErrorContract::new(
    error_code::DUPLICATE_SCHEMA_FILTER,
    "presentation query repeats a schema filter",
);
pub(crate) const INVALID_QUERY_CLAIMS: ErrorContract = ErrorContract::new(
    error_code::INVALID_QUERY_CLAIMS,
    "presentation query claim collection is invalid",
);
pub(crate) const DUPLICATE_QUERY_CLAIM: ErrorContract = ErrorContract::new(
    error_code::DUPLICATE_QUERY_CLAIM,
    "presentation query repeats a claim path",
);
pub(crate) const INVALID_REQUEST_QUERIES: ErrorContract = ErrorContract::new(
    error_code::INVALID_REQUEST_QUERIES,
    "presentation request query collection is invalid",
);
pub(crate) const DUPLICATE_QUERY_ID: ErrorContract = ErrorContract::new(
    error_code::DUPLICATE_QUERY_ID,
    "presentation request repeats a query id",
);
