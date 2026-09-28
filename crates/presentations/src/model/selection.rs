use std::fmt;

use identus_credentials::{CredentialClaimPath, CredentialFormat};

use crate::PresentationError;

use super::{
    MAX_PRESENTATION_CANDIDATE_CLAIMS, MAX_PRESENTATION_CANDIDATES,
    MAX_PRESENTATION_DISCLOSURE_SELECTIONS, MAX_PRESENTATION_SELECTION_CLAIMS,
    PresentationClaimIntent, PresentationCredentialHandle, PresentationQueryId,
    PresentationRequest, has_duplicates,
};

/// One local credential reported as structurally able to satisfy a query.
#[must_use]
#[derive(Clone, PartialEq, Eq)]
pub struct PresentationCredentialCandidate {
    query_id: PresentationQueryId,
    credential_handle: PresentationCredentialHandle,
    format: CredentialFormat,
    satisfiable_claims: Vec<CredentialClaimPath>,
}

impl PresentationCredentialCandidate {
    /// Construct a candidate with a bounded unique set of satisfiable paths.
    pub fn new(
        query_id: PresentationQueryId,
        credential_handle: PresentationCredentialHandle,
        format: CredentialFormat,
        satisfiable_claims: Vec<CredentialClaimPath>,
    ) -> Result<Self, PresentationError> {
        if satisfiable_claims.len() > MAX_PRESENTATION_CANDIDATE_CLAIMS {
            return Err(PresentationError::InvalidCandidateClaims);
        }
        if has_duplicates(&satisfiable_claims) {
            return Err(PresentationError::DuplicateCandidateClaim);
        }
        Ok(Self {
            query_id,
            credential_handle,
            format,
            satisfiable_claims,
        })
    }

    /// Return the query this candidate addresses.
    pub const fn query_id(&self) -> &PresentationQueryId {
        &self.query_id
    }

    /// Return the opaque local credential handle.
    pub const fn credential_handle(&self) -> &PresentationCredentialHandle {
        &self.credential_handle
    }

    /// Return the credential format reported by the candidate source.
    pub const fn format(&self) -> &CredentialFormat {
        &self.format
    }

    /// Borrow the unique requested paths this candidate can satisfy.
    pub fn satisfiable_claims(&self) -> &[CredentialClaimPath] {
        &self.satisfiable_claims
    }
}

impl fmt::Debug for PresentationCredentialCandidate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PresentationCredentialCandidate")
            .field("query_id", &self.query_id)
            .field("credential_handle", &self.credential_handle)
            .field("format", &self.format)
            .field("claim_count", &self.satisfiable_claims.len())
            .finish_non_exhaustive()
    }
}

/// One value-free claim choice copied from a presentation request.
#[must_use]
#[derive(Clone, PartialEq, Eq)]
pub struct PresentationSelectedClaim {
    path: CredentialClaimPath,
    intent: PresentationClaimIntent,
}

impl PresentationSelectedClaim {
    /// Construct a selected claim from an already validated path and intent.
    pub const fn new(path: CredentialClaimPath, intent: PresentationClaimIntent) -> Self {
        Self { path, intent }
    }

    /// Return the selected credential claim path.
    pub const fn path(&self) -> &CredentialClaimPath {
        &self.path
    }

    /// Return the selected reveal-or-predicate intent.
    pub const fn intent(&self) -> PresentationClaimIntent {
        self.intent
    }
}

impl fmt::Debug for PresentationSelectedClaim {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PresentationSelectedClaim")
            .field("path_segments", &self.path.segments().len())
            .field("intent", &self.intent)
            .finish_non_exhaustive()
    }
}

/// One credential and its explicitly selected, value-free claims.
#[must_use]
#[derive(Clone, PartialEq, Eq)]
pub struct PresentationCredentialSelection {
    query_id: PresentationQueryId,
    credential_handle: PresentationCredentialHandle,
    selected_claims: Vec<PresentationSelectedClaim>,
}

impl PresentationCredentialSelection {
    /// Construct a bounded selection with unique complete claim paths.
    pub fn new(
        query_id: PresentationQueryId,
        credential_handle: PresentationCredentialHandle,
        selected_claims: Vec<PresentationSelectedClaim>,
    ) -> Result<Self, PresentationError> {
        if selected_claims.len() > MAX_PRESENTATION_SELECTION_CLAIMS {
            return Err(PresentationError::InvalidSelectionClaims);
        }
        if selected_claims.iter().enumerate().any(|(index, claim)| {
            selected_claims[..index]
                .iter()
                .any(|previous| previous.path() == claim.path())
        }) {
            return Err(PresentationError::DuplicateSelectionClaim);
        }
        Ok(Self {
            query_id,
            credential_handle,
            selected_claims,
        })
    }

    /// Return the request query this selection addresses.
    pub const fn query_id(&self) -> &PresentationQueryId {
        &self.query_id
    }

    /// Return the selected opaque local credential handle.
    pub const fn credential_handle(&self) -> &PresentationCredentialHandle {
        &self.credential_handle
    }

    /// Borrow the explicitly selected claim paths and intents.
    pub fn selected_claims(&self) -> &[PresentationSelectedClaim] {
        &self.selected_claims
    }
}

impl fmt::Debug for PresentationCredentialSelection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PresentationCredentialSelection")
            .field("query_id", &self.query_id)
            .field("credential_handle", &self.credential_handle)
            .field("claim_count", &self.selected_claims.len())
            .finish_non_exhaustive()
    }
}

/// Bounded candidates bound to the exact request they were validated against.
#[must_use]
#[derive(Clone, PartialEq, Eq)]
pub struct PresentationCandidateSet {
    request: PresentationRequest,
    candidates: Vec<PresentationCredentialCandidate>,
}

impl PresentationCandidateSet {
    /// Validate and retain a candidate vector against its presentation request.
    pub fn new(
        request: &PresentationRequest,
        candidates: Vec<PresentationCredentialCandidate>,
    ) -> Result<Self, PresentationError> {
        Self::validate_candidates(request, &candidates)?;
        Ok(Self {
            request: request.clone(),
            candidates,
        })
    }

    fn validate_against(&self, request: &PresentationRequest) -> Result<(), PresentationError> {
        if self.request != *request {
            return Err(PresentationError::CandidateRequestMismatch);
        }
        Self::validate_candidates(request, &self.candidates)
    }

    fn validate_candidates(
        request: &PresentationRequest,
        candidates: &[PresentationCredentialCandidate],
    ) -> Result<(), PresentationError> {
        if candidates.len() > MAX_PRESENTATION_CANDIDATES {
            return Err(PresentationError::InvalidCandidates);
        }
        if candidates.iter().enumerate().any(|(index, candidate)| {
            candidates[..index].iter().any(|previous| {
                previous.query_id() == candidate.query_id()
                    && previous.credential_handle() == candidate.credential_handle()
            })
        }) {
            return Err(PresentationError::DuplicateCandidate);
        }

        for candidate in candidates {
            let query = request
                .query(candidate.query_id())
                .ok_or(PresentationError::UnknownCandidateQuery)?;
            if candidate.format() != query.format() {
                return Err(PresentationError::CandidateFormatMismatch);
            }
            if candidate.satisfiable_claims().iter().any(|candidate_path| {
                !query
                    .claims()
                    .iter()
                    .any(|claim| claim.path() == candidate_path)
            }) {
                return Err(PresentationError::CandidateUnrequestedClaim);
            }
            if query.claims().iter().any(|claim| {
                claim.required()
                    && !candidate
                        .satisfiable_claims()
                        .iter()
                        .any(|candidate_path| candidate_path == claim.path())
            }) {
                return Err(PresentationError::CandidateMissingRequiredClaim);
            }
        }
        Ok(())
    }

    fn candidate(
        &self,
        query_id: &PresentationQueryId,
        credential_handle: &PresentationCredentialHandle,
    ) -> Option<&PresentationCredentialCandidate> {
        self.candidates.iter().find(|candidate| {
            candidate.query_id() == query_id && candidate.credential_handle() == credential_handle
        })
    }

    /// Borrow the ordered validated candidates.
    pub fn as_slice(&self) -> &[PresentationCredentialCandidate] {
        &self.candidates
    }

    /// Consume the set and return its candidate vector.
    pub fn into_vec(self) -> Vec<PresentationCredentialCandidate> {
        self.candidates
    }
}

impl fmt::Debug for PresentationCandidateSet {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PresentationCandidateSet")
            .field("candidate_count", &self.candidates.len())
            .finish_non_exhaustive()
    }
}

/// A bounded, structurally validated input for format-specific proof generation.
#[must_use]
#[derive(Clone, PartialEq, Eq)]
pub struct PresentationDisclosurePlan {
    request: PresentationRequest,
    selections: Vec<PresentationCredentialSelection>,
}

impl PresentationDisclosurePlan {
    /// Validate and retain already-made credential and claim selections.
    pub fn new(
        request: &PresentationRequest,
        candidates: &PresentationCandidateSet,
        selections: Vec<PresentationCredentialSelection>,
    ) -> Result<Self, PresentationError> {
        if selections.is_empty() || selections.len() > MAX_PRESENTATION_DISCLOSURE_SELECTIONS {
            return Err(PresentationError::InvalidDisclosureSelections);
        }
        candidates.validate_against(request)?;
        if selections.iter().enumerate().any(|(index, selection)| {
            selections[..index].iter().any(|previous| {
                previous.query_id() == selection.query_id()
                    && previous.credential_handle() == selection.credential_handle()
            })
        }) {
            return Err(PresentationError::DuplicateDisclosureSelection);
        }

        for selection in &selections {
            let query = request
                .query(selection.query_id())
                .ok_or(PresentationError::UnknownSelectionQuery)?;
            let candidate = candidates
                .candidate(selection.query_id(), selection.credential_handle())
                .ok_or(PresentationError::UnknownSelectionCandidate)?;

            for selected_claim in selection.selected_claims() {
                let requested_claim = query
                    .claims()
                    .iter()
                    .find(|claim| claim.path() == selected_claim.path())
                    .ok_or(PresentationError::SelectionUnrequestedClaim)?;
                if requested_claim.intent() != selected_claim.intent() {
                    return Err(PresentationError::SelectionClaimIntentMismatch);
                }
                if !candidate
                    .satisfiable_claims()
                    .iter()
                    .any(|path| path == selected_claim.path())
                {
                    return Err(PresentationError::SelectionUnavailableClaim);
                }
            }

            if query.claims().iter().any(|requested_claim| {
                requested_claim.required()
                    && !selection.selected_claims().iter().any(|selected_claim| {
                        selected_claim.path() == requested_claim.path()
                            && selected_claim.intent() == requested_claim.intent()
                    })
            }) {
                return Err(PresentationError::SelectionMissingRequiredClaim);
            }
        }

        for query in request.queries() {
            let count = selections
                .iter()
                .filter(|selection| selection.query_id() == query.id())
                .count();
            if count == 0 {
                return Err(PresentationError::MissingQuerySelection);
            }
            if !query.multiple() && count != 1 {
                return Err(PresentationError::QueryMultiplicityExceeded);
            }
        }

        Ok(Self {
            request: request.clone(),
            selections,
        })
    }

    pub(super) fn validate_against(
        &self,
        request: &PresentationRequest,
    ) -> Result<(), PresentationError> {
        if self.request != *request {
            return Err(PresentationError::DisclosureRequestMismatch);
        }
        Ok(())
    }

    /// Borrow the ordered credential selections.
    pub fn as_slice(&self) -> &[PresentationCredentialSelection] {
        &self.selections
    }

    pub(super) const fn request(&self) -> &PresentationRequest {
        &self.request
    }

    /// Consume the plan and return its credential selections.
    pub fn into_vec(self) -> Vec<PresentationCredentialSelection> {
        self.selections
    }
}

impl fmt::Debug for PresentationDisclosurePlan {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PresentationDisclosurePlan")
            .field("selection_count", &self.selections.len())
            .finish_non_exhaustive()
    }
}
