//! Format-neutral credential semantics for the Identus Rust SDK.
//!
//! The crate owns a bounded, non-validating envelope for format-produced
//! credential artifacts and a canonical report for verifier-produced evidence.
//! It does not parse artifacts, perform checks, decide trust, store data, or
//! apply wallet policy.

#![forbid(unsafe_code)]

mod artifact;
mod envelope;
pub mod error;
mod format;
mod verification;

pub use artifact::{
    CredentialDetachedProof, CredentialPayload, CredentialPrivateMaterial,
    MAX_CREDENTIAL_DETACHED_PROOF_BYTES, MAX_CREDENTIAL_PAYLOAD_BYTES,
    MAX_CREDENTIAL_PRIVATE_MATERIAL_BYTES,
};
pub use envelope::CredentialEnvelope;
pub use error::CredentialError;
pub use format::{CredentialFormat, MAX_CREDENTIAL_FORMAT_BYTES};
pub use verification::{
    MAX_VERIFICATION_REASON_CODE_BYTES, VERIFICATION_STAGE_COUNT, VerificationOutcome,
    VerificationReasonCode, VerificationReport, VerificationStage, VerificationStageName,
    VerificationStageStatus,
};

use identus_core::Component;

/// Metadata for the `identus-credentials` crate.
pub const COMPONENT: Component = Component {
    name: "identus-credentials",
    summary: "Bounded credential artifacts, envelope and staged verification evidence.",
};
