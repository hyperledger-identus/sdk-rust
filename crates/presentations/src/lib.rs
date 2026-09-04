//! Format-neutral presentation request and candidate semantics.
//!
//! The crate owns bounded structural values shared by holder-side presentation
//! adapters. It does not decode protocol requests, discover credentials,
//! select disclosures, execute proofs, decide consent or trust, or store state.

#![forbid(unsafe_code)]

pub mod error;
mod model;

pub use error::PresentationError;
pub use model::{
    MAX_PRESENTATION_CANDIDATE_CLAIMS, MAX_PRESENTATION_CANDIDATES,
    MAX_PRESENTATION_CHALLENGE_BYTES, MAX_PRESENTATION_CREDENTIAL_HANDLE_BYTES,
    MAX_PRESENTATION_FILTER_VALUES, MAX_PRESENTATION_PURPOSE_BYTES, MAX_PRESENTATION_QUERY_CLAIMS,
    MAX_PRESENTATION_QUERY_ID_BYTES, MAX_PRESENTATION_REQUEST_QUERIES, PresentationCandidateSet,
    PresentationChallenge, PresentationClaimIntent, PresentationClaimRequest,
    PresentationCredentialCandidate, PresentationCredentialFilters, PresentationCredentialHandle,
    PresentationCredentialQuery, PresentationPurpose, PresentationQueryId, PresentationRequest,
};

use identus_core::Component;

/// Metadata for the `identus-presentations` crate.
pub const COMPONENT: Component = Component {
    name: "identus-presentations",
    summary: "Bounded format-neutral presentation request and candidate semantics.",
};
