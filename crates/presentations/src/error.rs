//! Redaction-safe presentation construction errors.

use std::fmt;

use identus_core::{CapabilityId, IdentusError};

use crate::error_contract::{
    ErrorContract, artifact_assembly, candidate_matching, disclosure_selection, lifecycle,
    request_query,
};

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
    pub const CANDIDATE_REQUEST_MISMATCH: ErrorCode =
        ErrorCode::new("presentation.candidate_request_mismatch");
    pub const INVALID_SELECTION_CLAIMS: ErrorCode =
        ErrorCode::new("presentation.invalid_selection_claims");
    pub const DUPLICATE_SELECTION_CLAIM: ErrorCode =
        ErrorCode::new("presentation.duplicate_selection_claim");
    pub const INVALID_DISCLOSURE_SELECTIONS: ErrorCode =
        ErrorCode::new("presentation.invalid_disclosure_selections");
    pub const DUPLICATE_DISCLOSURE_SELECTION: ErrorCode =
        ErrorCode::new("presentation.duplicate_disclosure_selection");
    pub const UNKNOWN_SELECTION_QUERY: ErrorCode =
        ErrorCode::new("presentation.unknown_selection_query");
    pub const UNKNOWN_SELECTION_CANDIDATE: ErrorCode =
        ErrorCode::new("presentation.unknown_selection_candidate");
    pub const SELECTION_UNREQUESTED_CLAIM: ErrorCode =
        ErrorCode::new("presentation.selection_unrequested_claim");
    pub const SELECTION_CLAIM_INTENT_MISMATCH: ErrorCode =
        ErrorCode::new("presentation.selection_claim_intent_mismatch");
    pub const SELECTION_UNAVAILABLE_CLAIM: ErrorCode =
        ErrorCode::new("presentation.selection_unavailable_claim");
    pub const SELECTION_MISSING_REQUIRED_CLAIM: ErrorCode =
        ErrorCode::new("presentation.selection_missing_required_claim");
    pub const MISSING_QUERY_SELECTION: ErrorCode =
        ErrorCode::new("presentation.missing_query_selection");
    pub const QUERY_MULTIPLICITY_EXCEEDED: ErrorCode =
        ErrorCode::new("presentation.query_multiplicity_exceeded");
    pub const DISCLOSURE_REQUEST_MISMATCH: ErrorCode =
        ErrorCode::new("presentation.disclosure_request_mismatch");
    pub const INVALID_ARTIFACT_BINDINGS: ErrorCode =
        ErrorCode::new("presentation.invalid_artifact_bindings");
    pub const DUPLICATE_ARTIFACT_BINDING: ErrorCode =
        ErrorCode::new("presentation.duplicate_artifact_binding");
    pub const INVALID_ARTIFACT_PAYLOAD: ErrorCode =
        ErrorCode::new("presentation.invalid_artifact_payload");
    pub const INVALID_GENERATED_ARTIFACTS: ErrorCode =
        ErrorCode::new("presentation.invalid_generated_artifacts");
    pub const ARTIFACT_PAYLOAD_BUDGET_EXCEEDED: ErrorCode =
        ErrorCode::new("presentation.artifact_payload_budget_exceeded");
    pub const UNKNOWN_ARTIFACT_SELECTION: ErrorCode =
        ErrorCode::new("presentation.unknown_artifact_selection");
    pub const ARTIFACT_FORMAT_MISMATCH: ErrorCode =
        ErrorCode::new("presentation.artifact_format_mismatch");
    pub const DUPLICATE_GENERATED_ARTIFACT_BINDING: ErrorCode =
        ErrorCode::new("presentation.duplicate_generated_artifact_binding");
    pub const MISSING_ARTIFACT_SELECTION: ErrorCode =
        ErrorCode::new("presentation.missing_artifact_selection");
    pub const INVALID_LIFECYCLE_PHASE: ErrorCode =
        ErrorCode::new("presentation.invalid_lifecycle_phase");
    pub const INVALID_TERMINAL_OUTCOME: ErrorCode =
        ErrorCode::new("presentation.invalid_terminal_outcome");
    pub const INVALID_PROTOCOL_STATE: ErrorCode =
        ErrorCode::new("presentation.invalid_protocol_state");
    pub const INVALID_PROTOCOL_TRANSITION: ErrorCode =
        ErrorCode::new("presentation.invalid_protocol_transition");
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
    /// A candidate set was validated against a different request.
    CandidateRequestMismatch,
    /// A credential selection carries too many selected claims.
    InvalidSelectionClaims,
    /// A credential selection repeats a complete selected claim path.
    DuplicateSelectionClaim,
    /// A disclosure plan is empty or carries too many credential selections.
    InvalidDisclosureSelections,
    /// A disclosure plan repeats a query and credential handle pair.
    DuplicateDisclosureSelection,
    /// A disclosure selection references no query in the supplied request.
    UnknownSelectionQuery,
    /// A disclosure selection references no candidate in the supplied set.
    UnknownSelectionCandidate,
    /// A disclosure selection includes a claim absent from its query.
    SelectionUnrequestedClaim,
    /// A selected claim intent differs from its requested intent.
    SelectionClaimIntentMismatch,
    /// A selected claim was not reported satisfiable by its candidate.
    SelectionUnavailableClaim,
    /// A credential selection omits a required requested claim.
    SelectionMissingRequiredClaim,
    /// A disclosure plan does not cover one request query.
    MissingQuerySelection,
    /// A disclosure plan selects multiple credentials for a single-valued query.
    QueryMultiplicityExceeded,
    /// A disclosure plan was paired with a different presentation request.
    DisclosureRequestMismatch,
    /// An artifact binding collection is empty or exceeds its bound.
    InvalidArtifactBindings,
    /// An artifact repeats a query and credential handle binding.
    DuplicateArtifactBinding,
    /// An opaque artifact payload is empty or exceeds its item bound.
    InvalidArtifactPayload,
    /// A generated presentation artifact collection is empty or oversized.
    InvalidGeneratedArtifacts,
    /// Generated artifact payload bytes exceed the aggregate budget.
    ArtifactPayloadBudgetExceeded,
    /// An artifact binding references no selection in the disclosure plan.
    UnknownArtifactSelection,
    /// An artifact format differs from one of its bound request queries.
    ArtifactFormatMismatch,
    /// One selection binding occurs in more than one generated artifact.
    DuplicateGeneratedArtifactBinding,
    /// A disclosure-plan selection has no generated artifact binding.
    MissingArtifactSelection,
    /// A lifecycle phase spelling is outside the fixed generic vocabulary.
    InvalidLifecyclePhase,
    /// A terminal outcome spelling is outside the fixed generic vocabulary.
    InvalidTerminalOutcome,
    /// A protocol state spelling is outside the fixed generic vocabulary.
    InvalidProtocolState,
    /// A directed lifecycle transition violates the generic state machine.
    InvalidProtocolTransition,
}

macro_rules! define_presentation_error_contracts {
    ($($variant:ident => $contract:path),+ $(,)?) => {
        impl PresentationError {
            const fn contract(self) -> ErrorContract {
                match self {
                    $(Self::$variant => $contract),+
                }
            }

            #[cfg(test)]
            pub(crate) const CONTRACT_VARIANTS: &'static [Self] = &[
                $(Self::$variant),+
            ];
        }
    };
}

define_presentation_error_contracts! {
    InvalidQueryId => request_query::INVALID_QUERY_ID,
    InvalidPurpose => request_query::INVALID_PURPOSE,
    InvalidChallenge => request_query::INVALID_CHALLENGE,
    InvalidCredentialHandle => request_query::INVALID_CREDENTIAL_HANDLE,
    InvalidClaimIntent => request_query::INVALID_CLAIM_INTENT,
    InvalidQueryFilters => request_query::INVALID_QUERY_FILTERS,
    DuplicateIssuerFilter => request_query::DUPLICATE_ISSUER_FILTER,
    DuplicateTypeFilter => request_query::DUPLICATE_TYPE_FILTER,
    DuplicateSchemaFilter => request_query::DUPLICATE_SCHEMA_FILTER,
    InvalidQueryClaims => request_query::INVALID_QUERY_CLAIMS,
    DuplicateQueryClaim => request_query::DUPLICATE_QUERY_CLAIM,
    InvalidRequestQueries => request_query::INVALID_REQUEST_QUERIES,
    DuplicateQueryId => request_query::DUPLICATE_QUERY_ID,
    InvalidCandidateClaims => candidate_matching::INVALID_CANDIDATE_CLAIMS,
    DuplicateCandidateClaim => candidate_matching::DUPLICATE_CANDIDATE_CLAIM,
    InvalidCandidates => candidate_matching::INVALID_CANDIDATES,
    DuplicateCandidate => candidate_matching::DUPLICATE_CANDIDATE,
    UnknownCandidateQuery => candidate_matching::UNKNOWN_CANDIDATE_QUERY,
    CandidateFormatMismatch => candidate_matching::CANDIDATE_FORMAT_MISMATCH,
    CandidateUnrequestedClaim => candidate_matching::CANDIDATE_UNREQUESTED_CLAIM,
    CandidateMissingRequiredClaim => candidate_matching::CANDIDATE_MISSING_REQUIRED_CLAIM,
    CandidateRequestMismatch => candidate_matching::CANDIDATE_REQUEST_MISMATCH,
    InvalidSelectionClaims => disclosure_selection::INVALID_SELECTION_CLAIMS,
    DuplicateSelectionClaim => disclosure_selection::DUPLICATE_SELECTION_CLAIM,
    InvalidDisclosureSelections => disclosure_selection::INVALID_DISCLOSURE_SELECTIONS,
    DuplicateDisclosureSelection => disclosure_selection::DUPLICATE_DISCLOSURE_SELECTION,
    UnknownSelectionQuery => disclosure_selection::UNKNOWN_SELECTION_QUERY,
    UnknownSelectionCandidate => disclosure_selection::UNKNOWN_SELECTION_CANDIDATE,
    SelectionUnrequestedClaim => disclosure_selection::SELECTION_UNREQUESTED_CLAIM,
    SelectionClaimIntentMismatch => disclosure_selection::SELECTION_CLAIM_INTENT_MISMATCH,
    SelectionUnavailableClaim => disclosure_selection::SELECTION_UNAVAILABLE_CLAIM,
    SelectionMissingRequiredClaim => disclosure_selection::SELECTION_MISSING_REQUIRED_CLAIM,
    MissingQuerySelection => disclosure_selection::MISSING_QUERY_SELECTION,
    QueryMultiplicityExceeded => disclosure_selection::QUERY_MULTIPLICITY_EXCEEDED,
    DisclosureRequestMismatch => disclosure_selection::DISCLOSURE_REQUEST_MISMATCH,
    InvalidArtifactBindings => artifact_assembly::INVALID_ARTIFACT_BINDINGS,
    DuplicateArtifactBinding => artifact_assembly::DUPLICATE_ARTIFACT_BINDING,
    InvalidArtifactPayload => artifact_assembly::INVALID_ARTIFACT_PAYLOAD,
    InvalidGeneratedArtifacts => artifact_assembly::INVALID_GENERATED_ARTIFACTS,
    ArtifactPayloadBudgetExceeded => artifact_assembly::ARTIFACT_PAYLOAD_BUDGET_EXCEEDED,
    UnknownArtifactSelection => artifact_assembly::UNKNOWN_ARTIFACT_SELECTION,
    ArtifactFormatMismatch => artifact_assembly::ARTIFACT_FORMAT_MISMATCH,
    DuplicateGeneratedArtifactBinding => artifact_assembly::DUPLICATE_GENERATED_ARTIFACT_BINDING,
    MissingArtifactSelection => artifact_assembly::MISSING_ARTIFACT_SELECTION,
    InvalidLifecyclePhase => lifecycle::INVALID_LIFECYCLE_PHASE,
    InvalidTerminalOutcome => lifecycle::INVALID_TERMINAL_OUTCOME,
    InvalidProtocolState => lifecycle::INVALID_PROTOCOL_STATE,
    InvalidProtocolTransition => lifecycle::INVALID_PROTOCOL_TRANSITION,
}

impl PresentationError {
    /// Convert into the shared stable SDK error boundary.
    pub const fn to_identus_error(self) -> IdentusError {
        self.contract().to_identus_error()
    }
}

impl From<PresentationError> for IdentusError {
    fn from(error: PresentationError) -> Self {
        error.to_identus_error()
    }
}

impl fmt::Display for PresentationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.contract().message())
    }
}

impl std::error::Error for PresentationError {}
