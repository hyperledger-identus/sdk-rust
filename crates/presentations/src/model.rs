mod artifact;
mod request;
mod selection;
mod value;

pub use artifact::{
    GeneratedPresentation, PresentationArtifact, PresentationArtifactBinding,
    PresentationReceiptEntry, PresentationReceiptInput,
};
pub use request::{
    PresentationClaimRequest, PresentationCredentialFilters, PresentationCredentialQuery,
    PresentationRequest,
};
pub use selection::{
    PresentationCandidateSet, PresentationCredentialCandidate, PresentationCredentialSelection,
    PresentationDisclosurePlan, PresentationSelectedClaim,
};
pub use value::{
    PresentationChallenge, PresentationClaimIntent, PresentationCredentialHandle,
    PresentationPurpose, PresentationQueryId,
};

/// Maximum encoded length of a presentation query identifier.
pub const MAX_PRESENTATION_QUERY_ID_BYTES: usize = 128;
/// Maximum encoded length of a presentation purpose.
pub const MAX_PRESENTATION_PURPOSE_BYTES: usize = 2_048;
/// Maximum encoded length of a presentation replay challenge.
pub const MAX_PRESENTATION_CHALLENGE_BYTES: usize = 1_024;
/// Maximum encoded length of a local credential handle.
pub const MAX_PRESENTATION_CREDENTIAL_HANDLE_BYTES: usize = 1_024;
/// Maximum number of values in one present query filter.
pub const MAX_PRESENTATION_FILTER_VALUES: usize = 16;
/// Maximum number of claim requests in one credential query.
pub const MAX_PRESENTATION_QUERY_CLAIMS: usize = 64;
/// Maximum number of credential queries in one presentation request.
pub const MAX_PRESENTATION_REQUEST_QUERIES: usize = 16;
/// Maximum number of satisfiable claims recorded by one candidate.
pub const MAX_PRESENTATION_CANDIDATE_CLAIMS: usize = 64;
/// Maximum number of credential candidates in one validated set.
pub const MAX_PRESENTATION_CANDIDATES: usize = 64;
/// Maximum number of explicitly selected claims for one credential.
pub const MAX_PRESENTATION_SELECTION_CLAIMS: usize = 64;
/// Maximum number of credential selections in one disclosure plan.
pub const MAX_PRESENTATION_DISCLOSURE_SELECTIONS: usize = 64;
/// Maximum number of selection bindings carried by one presentation artifact.
pub const MAX_PRESENTATION_ARTIFACT_BINDINGS: usize = 64;
/// Maximum byte length of one opaque presentation artifact.
pub const MAX_PRESENTATION_ARTIFACT_BYTES: usize = 4 * 1_024 * 1_024;
/// Maximum number of artifacts in one generated presentation.
pub const MAX_GENERATED_PRESENTATION_ARTIFACTS: usize = 64;
/// Maximum aggregate artifact bytes in one generated presentation.
pub const MAX_GENERATED_PRESENTATION_BYTES: usize = 16 * 1_024 * 1_024;

fn has_duplicates<T: PartialEq>(values: &[T]) -> bool {
    values
        .iter()
        .enumerate()
        .any(|(index, value)| values[..index].contains(value))
}
