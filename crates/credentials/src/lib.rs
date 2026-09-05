//! Format-neutral credential semantics for the Identus Rust SDK.
//!
//! The crate owns a bounded, non-validating envelope for format-produced
//! credential artifacts, format-neutral descriptive metadata, and a canonical
//! report plus execution seam for verifier-produced evidence. It does not parse
//! artifacts, implement checks, decide trust, store data, or apply wallet
//! policy.

#![forbid(unsafe_code)]

mod artifact;
mod descriptor;
mod envelope;
pub mod error;
mod format;
mod metadata;
mod schema;
mod status;
mod verification;
mod verifier;

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
pub use status::{
    CredentialStatusBinding, CredentialStatusBindings, CredentialStatusEvidence,
    CredentialStatusFreshness, CredentialStatusHandle, CredentialStatusMethod,
    CredentialStatusPurpose, CredentialStatusQuery, CredentialStatusReference,
    CredentialStatusRequirements, CredentialStatusRevision, CredentialStatusValue,
    MAX_CREDENTIAL_STATUS_BINDINGS, MAX_CREDENTIAL_STATUS_HANDLE_BYTES,
    MAX_CREDENTIAL_STATUS_METHOD_BYTES, MAX_CREDENTIAL_STATUS_PURPOSE_BYTES,
    MAX_CREDENTIAL_STATUS_REFERENCE_BYTES, MAX_CREDENTIAL_STATUS_REQUIREMENT_VALUES,
    MAX_CREDENTIAL_STATUS_REVISION_BYTES, MAX_CREDENTIAL_STATUS_VALUE_BYTES,
};
pub use verification::{
    MAX_VERIFICATION_REASON_CODE_BYTES, VERIFICATION_STAGE_COUNT, VerificationOutcome,
    VerificationReasonCode, VerificationReport, VerificationStage, VerificationStageName,
    VerificationStageStatus,
};
pub use verifier::{
    CredentialVerificationError, CredentialVerificationFuture, CredentialVerificationRequest,
    CredentialVerificationResult, CredentialVerifier, CredentialVerifierRegistry,
    CredentialVerifierRegistryBuilder, MAX_CREDENTIAL_VERIFIER_REGISTRY_ENTRIES,
};

use identus_core::Component;

/// Metadata for the `identus-credentials` crate.
pub const COMPONENT: Component = Component {
    name: "identus-credentials",
    summary: "Bounded credential artifacts, descriptors, status and verification seams.",
};
