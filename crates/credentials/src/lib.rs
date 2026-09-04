//! Format-neutral credential semantics for the Identus Rust SDK.
//!
//! The crate owns a bounded, non-validating envelope for format-produced
//! credential artifacts, format-neutral descriptive metadata, and a canonical
//! report for verifier-produced evidence. It does not parse artifacts, perform
//! checks, decide trust, store data, or apply wallet policy.

#![forbid(unsafe_code)]

mod artifact;
mod descriptor;
mod envelope;
pub mod error;
mod format;
mod metadata;
mod schema;
mod verification;

pub use artifact::{
    CredentialDetachedProof, CredentialPayload, CredentialPrivateMaterial,
    MAX_CREDENTIAL_DETACHED_PROOF_BYTES, MAX_CREDENTIAL_PAYLOAD_BYTES,
    MAX_CREDENTIAL_PRIVATE_MATERIAL_BYTES,
};
pub use descriptor::{
    CredentialClaimId, CredentialClaimPathSegment, CredentialClaimValueType, CredentialEntityId,
    CredentialSchemaId, CredentialSchemaVersion, CredentialType, MAX_CREDENTIAL_CLAIM_ID_BYTES,
    MAX_CREDENTIAL_CLAIM_PATH_SEGMENT_BYTES, MAX_CREDENTIAL_CLAIM_VALUE_TYPE_BYTES,
    MAX_CREDENTIAL_ENTITY_ID_BYTES, MAX_CREDENTIAL_SCHEMA_ID_BYTES,
    MAX_CREDENTIAL_SCHEMA_VERSION_BYTES, MAX_CREDENTIAL_TYPE_BYTES,
};
pub use envelope::CredentialEnvelope;
pub use error::CredentialError;
pub use format::{CredentialFormat, MAX_CREDENTIAL_FORMAT_BYTES};
pub use metadata::{CredentialMetadata, MAX_CREDENTIAL_SCHEMAS, MAX_CREDENTIAL_SUBJECTS};
pub use schema::{
    CredentialClaimDescriptor, CredentialClaimDisclosure, CredentialClaimPath,
    CredentialSchemaDescriptor, MAX_CREDENTIAL_CLAIM_PATH_SEGMENTS, MAX_CREDENTIAL_SCHEMA_CLAIMS,
    MAX_CREDENTIAL_TYPES,
};
pub use verification::{
    MAX_VERIFICATION_REASON_CODE_BYTES, VERIFICATION_STAGE_COUNT, VerificationOutcome,
    VerificationReasonCode, VerificationReport, VerificationStage, VerificationStageName,
    VerificationStageStatus,
};

use identus_core::Component;

/// Metadata for the `identus-credentials` crate.
pub const COMPONENT: Component = Component {
    name: "identus-credentials",
    summary: "Bounded credential artifacts, descriptors and staged verification evidence.",
};
