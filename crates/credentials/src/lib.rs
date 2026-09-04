//! Format-neutral credential semantics for the Identus Rust SDK.
//!
//! The crate currently owns only a bounded, non-validating envelope for
//! format-produced credential artifacts. Construction preserves bytes; it
//! does not parse, verify, trust, store, or apply wallet policy.

#![forbid(unsafe_code)]

mod artifact;
mod envelope;
pub mod error;
mod format;

pub use artifact::{
    CredentialDetachedProof, CredentialPayload, CredentialPrivateMaterial,
    MAX_CREDENTIAL_DETACHED_PROOF_BYTES, MAX_CREDENTIAL_PAYLOAD_BYTES,
    MAX_CREDENTIAL_PRIVATE_MATERIAL_BYTES,
};
pub use envelope::CredentialEnvelope;
pub use error::CredentialError;
pub use format::{CredentialFormat, MAX_CREDENTIAL_FORMAT_BYTES};

use identus_core::Component;

/// Metadata for the `identus-credentials` crate.
pub const COMPONENT: Component = Component {
    name: "identus-credentials",
    summary: "Bounded format-neutral credential artifacts and envelope semantics.",
};
