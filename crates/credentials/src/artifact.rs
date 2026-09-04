use std::fmt;

use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::CredentialError;

/// Maximum encoded credential payload size accepted by the envelope.
pub const MAX_CREDENTIAL_PAYLOAD_BYTES: usize = 1_048_576;
/// Maximum detached proof size accepted by the envelope.
pub const MAX_CREDENTIAL_DETACHED_PROOF_BYTES: usize = 1_048_576;
/// Maximum format-private material size accepted by the envelope.
pub const MAX_CREDENTIAL_PRIVATE_MATERIAL_BYTES: usize = 262_144;

fn validate_artifact_size(
    bytes: &[u8],
    maximum: usize,
    empty: CredentialError,
    too_large: CredentialError,
) -> Result<(), CredentialError> {
    if bytes.is_empty() {
        return Err(empty);
    }
    if bytes.len() > maximum {
        return Err(too_large);
    }
    Ok(())
}

/// Bounded, uninterpreted encoded credential bytes.
#[must_use]
#[derive(Clone, PartialEq, Eq)]
pub struct CredentialPayload(Vec<u8>);

impl CredentialPayload {
    /// Validate and own encoded credential bytes.
    pub fn new(bytes: Vec<u8>) -> Result<Self, CredentialError> {
        validate_artifact_size(
            &bytes,
            MAX_CREDENTIAL_PAYLOAD_BYTES,
            CredentialError::EmptyPayload,
            CredentialError::PayloadTooLarge,
        )?;
        Ok(Self(bytes))
    }

    /// Borrow the exact format-produced bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Debug for CredentialPayload {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialPayload")
            .field("length", &self.0.len())
            .finish_non_exhaustive()
    }
}

/// Bounded, uninterpreted proof bytes stored separately from a credential.
#[must_use]
#[derive(Clone, PartialEq, Eq)]
pub struct CredentialDetachedProof(Vec<u8>);

impl CredentialDetachedProof {
    /// Validate and own detached proof bytes.
    pub fn new(bytes: Vec<u8>) -> Result<Self, CredentialError> {
        validate_artifact_size(
            &bytes,
            MAX_CREDENTIAL_DETACHED_PROOF_BYTES,
            CredentialError::EmptyDetachedProof,
            CredentialError::DetachedProofTooLarge,
        )?;
        Ok(Self(bytes))
    }

    /// Borrow the exact format-produced proof bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Debug for CredentialDetachedProof {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialDetachedProof")
            .field("length", &self.0.len())
            .finish_non_exhaustive()
    }
}

/// Bounded, format-owned private material associated with a credential.
///
/// The owned buffer is zeroized explicitly or on drop. Any bytes copied from
/// [`Self::as_bytes`] become caller-owned and are outside this lifecycle.
#[must_use]
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct CredentialPrivateMaterial(Vec<u8>);

impl CredentialPrivateMaterial {
    /// Validate and own format-private material.
    ///
    /// Rejected input is zeroized before the transferred allocation is
    /// released.
    pub fn new(mut bytes: Vec<u8>) -> Result<Self, CredentialError> {
        if let Err(error) = validate_artifact_size(
            &bytes,
            MAX_CREDENTIAL_PRIVATE_MATERIAL_BYTES,
            CredentialError::EmptyPrivateMaterial,
            CredentialError::PrivateMaterialTooLarge,
        ) {
            bytes.zeroize();
            return Err(error);
        }
        Ok(Self(bytes))
    }

    /// Borrow the private bytes explicitly.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Debug for CredentialPrivateMaterial {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialPrivateMaterial")
            .field("length", &self.0.len())
            .finish_non_exhaustive()
    }
}
