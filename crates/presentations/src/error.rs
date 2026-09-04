//! Redaction-safe presentation construction errors.

use std::fmt;

use identus_core::{CapabilityId, ErrorCode, ErrorKind, IdentusError};

/// Owning capability for presentation construction errors.
pub const CAPABILITY: CapabilityId = CapabilityId::new("presentation");

/// Stable error codes used by the shared SDK error boundary.
pub mod error_code {
    use identus_core::ErrorCode;

    pub const INVALID_QUERY_ID: ErrorCode = ErrorCode::new("presentation.invalid_query_id");
    pub const INVALID_PURPOSE: ErrorCode = ErrorCode::new("presentation.invalid_purpose");
    pub const INVALID_CHALLENGE: ErrorCode = ErrorCode::new("presentation.invalid_challenge");
    pub const INVALID_CREDENTIAL_HANDLE: ErrorCode =
        ErrorCode::new("presentation.invalid_credential_handle");
    pub const INVALID_CLAIM_INTENT: ErrorCode = ErrorCode::new("presentation.invalid_claim_intent");
    pub const INVALID_QUERY_FILTERS: ErrorCode =
        ErrorCode::new("presentation.invalid_query_filters");
    pub const DUPLICATE_ISSUER_FILTER: ErrorCode =
        ErrorCode::new("presentation.duplicate_issuer_filter");
    pub const DUPLICATE_TYPE_FILTER: ErrorCode =
        ErrorCode::new("presentation.duplicate_type_filter");
    pub const DUPLICATE_SCHEMA_FILTER: ErrorCode =
        ErrorCode::new("presentation.duplicate_schema_filter");
    pub const INVALID_QUERY_CLAIMS: ErrorCode = ErrorCode::new("presentation.invalid_query_claims");
    pub const DUPLICATE_QUERY_CLAIM: ErrorCode =
        ErrorCode::new("presentation.duplicate_query_claim");
    pub const INVALID_REQUEST_QUERIES: ErrorCode =
        ErrorCode::new("presentation.invalid_request_queries");
    pub const DUPLICATE_QUERY_ID: ErrorCode = ErrorCode::new("presentation.duplicate_query_id");
    pub const INVALID_CANDIDATE_CLAIMS: ErrorCode =
        ErrorCode::new("presentation.invalid_candidate_claims");
    pub const DUPLICATE_CANDIDATE_CLAIM: ErrorCode =
        ErrorCode::new("presentation.duplicate_candidate_claim");
    pub const INVALID_CANDIDATES: ErrorCode = ErrorCode::new("presentation.invalid_candidates");
    pub const DUPLICATE_CANDIDATE: ErrorCode = ErrorCode::new("presentation.duplicate_candidate");
    pub const UNKNOWN_CANDIDATE_QUERY: ErrorCode =
        ErrorCode::new("presentation.unknown_candidate_query");
    pub const CANDIDATE_FORMAT_MISMATCH: ErrorCode =
        ErrorCode::new("presentation.candidate_format_mismatch");
    pub const CANDIDATE_UNREQUESTED_CLAIM: ErrorCode =
        ErrorCode::new("presentation.candidate_unrequested_claim");
    pub const CANDIDATE_MISSING_REQUIRED_CLAIM: ErrorCode =
        ErrorCode::new("presentation.candidate_missing_required_claim");
}

/// Typed reason that a presentation domain value could not be constructed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum PresentationError {
    /// A query identifier violates its token grammar or size limit.
    InvalidQueryId,
    /// A purpose violates its text bounds.
    InvalidPurpose,
    /// A challenge violates its text or byte bounds.
    InvalidChallenge,
    /// A local credential handle violates its text or byte bounds.
    InvalidCredentialHandle,
    /// A claim intent is outside the fixed generic vocabulary.
    InvalidClaimIntent,
    /// A present query filter collection is empty or exceeds its bound.
    InvalidQueryFilters,
    /// An issuer filter is repeated.
    DuplicateIssuerFilter,
    /// A credential type filter is repeated.
    DuplicateTypeFilter,
    /// A schema filter is repeated.
    DuplicateSchemaFilter,
    /// A query carries too many claim requests.
    InvalidQueryClaims,
    /// A query repeats a complete requested claim path.
    DuplicateQueryClaim,
    /// A request has no queries or exceeds its query bound.
    InvalidRequestQueries,
    /// A request repeats a query identifier.
    DuplicateQueryId,
    /// A candidate carries too many satisfiable claim paths.
    InvalidCandidateClaims,
    /// A candidate repeats a satisfiable claim path.
    DuplicateCandidateClaim,
    /// A candidate set exceeds its bound.
    InvalidCandidates,
    /// A query and local credential handle pair is repeated.
    DuplicateCandidate,
    /// A candidate references no query in its request.
    UnknownCandidateQuery,
    /// A candidate format differs from its query format.
    CandidateFormatMismatch,
    /// A candidate includes a claim path absent from its query.
    CandidateUnrequestedClaim,
    /// A candidate omits a required query claim path.
    CandidateMissingRequiredClaim,
}

impl PresentationError {
    /// Convert into the shared stable SDK error boundary.
    pub const fn to_identus_error(self) -> IdentusError {
        let (code, message) = self.contract();
        IdentusError::public(code, ErrorKind::InvalidInput, CAPABILITY, message)
    }

    const fn contract(self) -> (ErrorCode, &'static str) {
        use error_code as code;
        match self {
            Self::InvalidQueryId => (code::INVALID_QUERY_ID, "presentation query id is invalid"),
            Self::InvalidPurpose => (code::INVALID_PURPOSE, "presentation purpose is invalid"),
            Self::InvalidChallenge => {
                (code::INVALID_CHALLENGE, "presentation challenge is invalid")
            }
            Self::InvalidCredentialHandle => (
                code::INVALID_CREDENTIAL_HANDLE,
                "presentation credential handle is invalid",
            ),
            Self::InvalidClaimIntent => (
                code::INVALID_CLAIM_INTENT,
                "presentation claim intent is invalid",
            ),
            Self::InvalidQueryFilters => (
                code::INVALID_QUERY_FILTERS,
                "presentation query filters are invalid",
            ),
            Self::DuplicateIssuerFilter => (
                code::DUPLICATE_ISSUER_FILTER,
                "presentation query repeats an issuer filter",
            ),
            Self::DuplicateTypeFilter => (
                code::DUPLICATE_TYPE_FILTER,
                "presentation query repeats a credential type filter",
            ),
            Self::DuplicateSchemaFilter => (
                code::DUPLICATE_SCHEMA_FILTER,
                "presentation query repeats a schema filter",
            ),
            Self::InvalidQueryClaims => (
                code::INVALID_QUERY_CLAIMS,
                "presentation query claim collection is invalid",
            ),
            Self::DuplicateQueryClaim => (
                code::DUPLICATE_QUERY_CLAIM,
                "presentation query repeats a claim path",
            ),
            Self::InvalidRequestQueries => (
                code::INVALID_REQUEST_QUERIES,
                "presentation request query collection is invalid",
            ),
            Self::DuplicateQueryId => (
                code::DUPLICATE_QUERY_ID,
                "presentation request repeats a query id",
            ),
            Self::InvalidCandidateClaims => (
                code::INVALID_CANDIDATE_CLAIMS,
                "presentation candidate claim collection is invalid",
            ),
            Self::DuplicateCandidateClaim => (
                code::DUPLICATE_CANDIDATE_CLAIM,
                "presentation candidate repeats a claim path",
            ),
            Self::InvalidCandidates => (
                code::INVALID_CANDIDATES,
                "presentation candidate collection is invalid",
            ),
            Self::DuplicateCandidate => (
                code::DUPLICATE_CANDIDATE,
                "presentation candidate is repeated",
            ),
            Self::UnknownCandidateQuery => (
                code::UNKNOWN_CANDIDATE_QUERY,
                "presentation candidate references an unknown query",
            ),
            Self::CandidateFormatMismatch => (
                code::CANDIDATE_FORMAT_MISMATCH,
                "presentation candidate format does not match its query",
            ),
            Self::CandidateUnrequestedClaim => (
                code::CANDIDATE_UNREQUESTED_CLAIM,
                "presentation candidate contains an unrequested claim",
            ),
            Self::CandidateMissingRequiredClaim => (
                code::CANDIDATE_MISSING_REQUIRED_CLAIM,
                "presentation candidate omits a required claim",
            ),
        }
    }
}

impl From<PresentationError> for IdentusError {
    fn from(error: PresentationError) -> Self {
        error.to_identus_error()
    }
}

impl fmt::Display for PresentationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.contract().1)
    }
}

impl std::error::Error for PresentationError {}
