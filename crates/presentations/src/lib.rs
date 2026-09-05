//! Format-neutral presentation semantics and lifecycle state.
//!
//! The crate owns bounded structural values shared by holder-side presentation
//! adapters. It does not decode protocol requests, discover or rank
//! credentials, choose selections, execute proofs, decide consent or trust, or
//! persist state or perform external effects.

#![forbid(unsafe_code)]

pub mod error;
mod lifecycle;
mod model;

pub use error::PresentationError;
pub use lifecycle::{
    PresentationLifecyclePhase, PresentationProtocolState, PresentationTerminalOutcome,
};
pub use model::{
    GeneratedPresentation, MAX_GENERATED_PRESENTATION_ARTIFACTS, MAX_GENERATED_PRESENTATION_BYTES,
    MAX_PRESENTATION_ARTIFACT_BINDINGS, MAX_PRESENTATION_ARTIFACT_BYTES,
    MAX_PRESENTATION_CANDIDATE_CLAIMS, MAX_PRESENTATION_CANDIDATES,
    MAX_PRESENTATION_CHALLENGE_BYTES, MAX_PRESENTATION_CREDENTIAL_HANDLE_BYTES,
    MAX_PRESENTATION_DISCLOSURE_SELECTIONS, MAX_PRESENTATION_FILTER_VALUES,
    MAX_PRESENTATION_PURPOSE_BYTES, MAX_PRESENTATION_QUERY_CLAIMS, MAX_PRESENTATION_QUERY_ID_BYTES,
    MAX_PRESENTATION_REQUEST_QUERIES, MAX_PRESENTATION_SELECTION_CLAIMS, PresentationArtifact,
    PresentationArtifactBinding, PresentationCandidateSet, PresentationChallenge,
    PresentationClaimIntent, PresentationClaimRequest, PresentationCredentialCandidate,
    PresentationCredentialFilters, PresentationCredentialHandle, PresentationCredentialQuery,
    PresentationCredentialSelection, PresentationDisclosurePlan, PresentationPurpose,
    PresentationQueryId, PresentationReceiptEntry, PresentationReceiptInput, PresentationRequest,
    PresentationSelectedClaim,
};

use identus_core::Component;

/// Metadata for the `identus-presentations` crate.
pub const COMPONENT: Component = Component {
    name: "identus-presentations",
    summary: "Bounded format-neutral presentation semantics and lifecycle state.",
};
