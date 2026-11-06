use std::fmt;

use identus_credentials::{
    CredentialStatusBinding, CredentialStatusEvidence, CredentialStatusHandle,
    CredentialStatusMethod, CredentialStatusPurpose, CredentialStatusReference,
};
use uriparse::URIReference;

use crate::{SdJwtVcError, SdJwtVcLimits};

/// Exact draft status-list reference mapped to generic credential vocabulary.
#[derive(Clone, PartialEq, Eq)]
pub struct StatusListBinding {
    index: u64,
    uri: String,
    generic: CredentialStatusBinding,
}

/// Caller-obtained generic status observation whose associated Status List
/// Token was validated as JWT, as required by draft-22.
#[derive(Clone)]
pub struct StatusListEvidence(CredentialStatusEvidence);

impl StatusListEvidence {
    pub fn jwt(evidence: CredentialStatusEvidence) -> Self {
        Self(evidence)
    }

    pub const fn generic(&self) -> &CredentialStatusEvidence {
        &self.0
    }
}

impl fmt::Debug for StatusListEvidence {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("StatusListEvidence")
            .field("token_format", &"jwt")
            .field("generic", &self.0)
            .finish()
    }
}

impl StatusListBinding {
    pub fn new(index: u64, uri: &str, limits: SdJwtVcLimits) -> Result<Self, SdJwtVcError> {
        if uri.is_empty() || uri.len() > limits.issuer_identifier_bytes {
            return Err(SdJwtVcError::ResourceLimit);
        }
        let parsed = URIReference::try_from(uri).map_err(|_| SdJwtVcError::Status)?;
        if parsed.scheme().is_none() {
            return Err(SdJwtVcError::Status);
        }
        let method = CredentialStatusMethod::parse("status_list")
            .map_err(|_| SdJwtVcError::InternalInvariant)?;
        let purpose = CredentialStatusPurpose::parse("credential-status")
            .map_err(|_| SdJwtVcError::InternalInvariant)?;
        let reference =
            CredentialStatusReference::from_text(uri).map_err(|_| SdJwtVcError::ResourceLimit)?;
        let handle = CredentialStatusHandle::from_text(&index.to_string())
            .map_err(|_| SdJwtVcError::InternalInvariant)?;
        Ok(Self {
            index,
            uri: uri.to_owned(),
            generic: CredentialStatusBinding::new(method, purpose, reference, handle),
        })
    }

    pub const fn generic(&self) -> &CredentialStatusBinding {
        &self.generic
    }

    pub(crate) const fn retained_bytes(&self) -> usize {
        self.uri.len()
    }
}

impl fmt::Debug for StatusListBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("StatusListBinding")
            .field("index", &self.index)
            .field("uri_bytes", &self.uri.len())
            .finish()
    }
}

/// Correlate already obtained status evidence without interpreting its value.
pub fn correlate_status_evidence(
    binding: &StatusListBinding,
    evidence: &StatusListEvidence,
) -> Result<(), SdJwtVcError> {
    if binding.generic() != evidence.generic().binding() {
        return Err(SdJwtVcError::EvidenceMismatch);
    }
    Ok(())
}
