use std::fmt;

use crate::{
    CredentialDetachedProof, CredentialFormat, CredentialPayload, CredentialPrivateMaterial,
};

/// Format-neutral owner of encoded credential artifacts.
///
/// An envelope is not evidence that its payload has been parsed, verified, or
/// trusted. Format adapters own those operations and their wire contracts.
#[must_use]
pub struct CredentialEnvelope {
    format: CredentialFormat,
    payload: CredentialPayload,
    detached_proof: Option<CredentialDetachedProof>,
    private_material: Option<CredentialPrivateMaterial>,
}

impl CredentialEnvelope {
    /// Construct an envelope from already validated artifact types.
    pub fn new(format: CredentialFormat, payload: CredentialPayload) -> Self {
        Self {
            format,
            payload,
            detached_proof: None,
            private_material: None,
        }
    }

    /// Attach a format-owned detached proof.
    pub fn with_detached_proof(mut self, detached_proof: CredentialDetachedProof) -> Self {
        self.detached_proof = Some(detached_proof);
        self
    }

    /// Attach format-owned private material.
    pub fn with_private_material(mut self, private_material: CredentialPrivateMaterial) -> Self {
        self.private_material = Some(private_material);
        self
    }

    /// Return the declared credential format.
    pub fn format(&self) -> &CredentialFormat {
        &self.format
    }

    /// Return the exact encoded credential payload.
    pub fn payload(&self) -> &CredentialPayload {
        &self.payload
    }

    /// Return the detached proof when the format stores it separately.
    pub fn detached_proof(&self) -> Option<&CredentialDetachedProof> {
        self.detached_proof.as_ref()
    }

    /// Return private format material through an explicit accessor.
    pub fn private_material(&self) -> Option<&CredentialPrivateMaterial> {
        self.private_material.as_ref()
    }
}

impl fmt::Debug for CredentialEnvelope {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialEnvelope")
            .field("format", &self.format)
            .field("payload_length", &self.payload.as_bytes().len())
            .field(
                "detached_proof_length",
                &self
                    .detached_proof
                    .as_ref()
                    .map(|proof| proof.as_bytes().len()),
            )
            .field(
                "private_material_length",
                &self
                    .private_material
                    .as_ref()
                    .map(|material| material.as_bytes().len()),
            )
            .finish_non_exhaustive()
    }
}
