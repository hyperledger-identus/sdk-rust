use std::fmt;

use identus_credentials::{CredentialEntityId, CredentialFormat};

use crate::PresentationError;

use super::{
    MAX_GENERATED_PRESENTATION_ARTIFACTS, MAX_GENERATED_PRESENTATION_BYTES,
    MAX_PRESENTATION_ARTIFACT_BINDINGS, MAX_PRESENTATION_ARTIFACT_BYTES,
    PresentationCredentialHandle, PresentationDisclosurePlan, PresentationPurpose,
    PresentationQueryId, PresentationRequest, PresentationSelectedClaim, has_duplicates,
};

/// One disclosure-plan selection represented by a generated artifact.
#[must_use]
#[derive(Clone, PartialEq, Eq)]
pub struct PresentationArtifactBinding {
    query_id: PresentationQueryId,
    credential_handle: PresentationCredentialHandle,
}

impl PresentationArtifactBinding {
    /// Identify one disclosure selection by query and opaque credential handle.
    pub const fn new(
        query_id: PresentationQueryId,
        credential_handle: PresentationCredentialHandle,
    ) -> Self {
        Self {
            query_id,
            credential_handle,
        }
    }

    /// Return the request query represented by the artifact.
    pub const fn query_id(&self) -> &PresentationQueryId {
        &self.query_id
    }

    /// Return the opaque local credential handle represented by the artifact.
    pub const fn credential_handle(&self) -> &PresentationCredentialHandle {
        &self.credential_handle
    }
}

impl fmt::Debug for PresentationArtifactBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PresentationArtifactBinding")
            .finish_non_exhaustive()
    }
}

/// Opaque bounded bytes produced by one format-specific presentation adapter.
#[must_use]
#[derive(Clone, PartialEq, Eq)]
pub struct PresentationArtifact {
    format: CredentialFormat,
    bindings: Vec<PresentationArtifactBinding>,
    bytes: Vec<u8>,
}

impl PresentationArtifact {
    /// Construct an opaque artifact with bounded unique selection bindings.
    pub fn new(
        format: CredentialFormat,
        bindings: Vec<PresentationArtifactBinding>,
        bytes: Vec<u8>,
    ) -> Result<Self, PresentationError> {
        if bindings.is_empty() || bindings.len() > MAX_PRESENTATION_ARTIFACT_BINDINGS {
            return Err(PresentationError::InvalidArtifactBindings);
        }
        if has_duplicates(&bindings) {
            return Err(PresentationError::DuplicateArtifactBinding);
        }
        if bytes.is_empty() || bytes.len() > MAX_PRESENTATION_ARTIFACT_BYTES {
            return Err(PresentationError::InvalidArtifactPayload);
        }
        Ok(Self {
            format,
            bindings,
            bytes,
        })
    }

    /// Return the format that owns the opaque artifact representation.
    pub const fn format(&self) -> &CredentialFormat {
        &self.format
    }

    /// Borrow the disclosure selections represented by this artifact.
    pub fn bindings(&self) -> &[PresentationArtifactBinding] {
        &self.bindings
    }

    /// Borrow the exact opaque artifact bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Consume the artifact and return its exact opaque byte vector.
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

impl fmt::Debug for PresentationArtifact {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PresentationArtifact")
            .field("format", &self.format)
            .field("binding_count", &self.bindings.len())
            .field("byte_length", &self.bytes.len())
            .finish_non_exhaustive()
    }
}

/// Validated generated presentation artifacts covering one disclosure plan.
#[must_use]
#[derive(Clone, PartialEq, Eq)]
pub struct GeneratedPresentation {
    plan: PresentationDisclosurePlan,
    artifacts: Vec<PresentationArtifact>,
}

struct GeneratedPresentationValidator<'a> {
    request: &'a PresentationRequest,
    plan: &'a PresentationDisclosurePlan,
    artifacts: &'a [PresentationArtifact],
}

impl<'a> GeneratedPresentationValidator<'a> {
    const fn new(
        request: &'a PresentationRequest,
        plan: &'a PresentationDisclosurePlan,
        artifacts: &'a [PresentationArtifact],
    ) -> Self {
        Self {
            request,
            plan,
            artifacts,
        }
    }

    fn validate(&self) -> Result<(), PresentationError> {
        self.validate_payload_budget()?;
        self.validate_artifact_bindings()?;
        self.validate_plan_coverage()
    }

    fn validate_payload_budget(&self) -> Result<(), PresentationError> {
        let total_bytes = self.artifacts.iter().try_fold(0_usize, |total, artifact| {
            total.checked_add(artifact.as_bytes().len())
        });
        if total_bytes.is_none_or(|total| total > MAX_GENERATED_PRESENTATION_BYTES) {
            return Err(PresentationError::ArtifactPayloadBudgetExceeded);
        }
        Ok(())
    }

    fn validate_artifact_bindings(&self) -> Result<(), PresentationError> {
        for (artifact_index, artifact) in self.artifacts.iter().enumerate() {
            for binding in artifact.bindings() {
                self.validate_binding(artifact_index, artifact, binding)?;
            }
        }
        Ok(())
    }

    fn validate_binding(
        &self,
        artifact_index: usize,
        artifact: &PresentationArtifact,
        binding: &PresentationArtifactBinding,
    ) -> Result<(), PresentationError> {
        let selection = self
            .plan
            .as_slice()
            .iter()
            .find(|selection| {
                selection.query_id() == binding.query_id()
                    && selection.credential_handle() == binding.credential_handle()
            })
            .ok_or(PresentationError::UnknownArtifactSelection)?;
        let query = self
            .request
            .query(selection.query_id())
            .ok_or(PresentationError::UnknownArtifactSelection)?;
        if artifact.format() != query.format() {
            return Err(PresentationError::ArtifactFormatMismatch);
        }
        if self.artifacts[..artifact_index].iter().any(|previous| {
            previous.bindings().iter().any(|previous_binding| {
                previous_binding.query_id() == binding.query_id()
                    && previous_binding.credential_handle() == binding.credential_handle()
            })
        }) {
            return Err(PresentationError::DuplicateGeneratedArtifactBinding);
        }
        Ok(())
    }

    fn validate_plan_coverage(&self) -> Result<(), PresentationError> {
        if self.plan.as_slice().iter().any(|selection| {
            !self.artifacts.iter().any(|artifact| {
                artifact.bindings().iter().any(|binding| {
                    binding.query_id() == selection.query_id()
                        && binding.credential_handle() == selection.credential_handle()
                })
            })
        }) {
            return Err(PresentationError::MissingArtifactSelection);
        }
        Ok(())
    }
}

impl GeneratedPresentation {
    /// Validate exact request, format, uniqueness and plan coverage invariants.
    pub fn new(
        request: &PresentationRequest,
        plan: PresentationDisclosurePlan,
        artifacts: Vec<PresentationArtifact>,
    ) -> Result<Self, PresentationError> {
        if artifacts.is_empty() || artifacts.len() > MAX_GENERATED_PRESENTATION_ARTIFACTS {
            return Err(PresentationError::InvalidGeneratedArtifacts);
        }
        plan.validate_against(request)?;
        GeneratedPresentationValidator::new(request, &plan, &artifacts).validate()?;

        Ok(Self { plan, artifacts })
    }

    /// Borrow the disclosure plan covered by the artifacts.
    pub const fn disclosure_plan(&self) -> &PresentationDisclosurePlan {
        &self.plan
    }

    /// Borrow the ordered generated artifacts.
    pub fn artifacts(&self) -> &[PresentationArtifact] {
        &self.artifacts
    }

    /// Derive a value-free owner-private input for downstream receipt policy.
    pub fn receipt_input(&self) -> PresentationReceiptInput {
        PresentationReceiptInput::from_generated(self)
    }

    /// Consume the generated presentation and return its artifact vector.
    pub fn into_artifacts(self) -> Vec<PresentationArtifact> {
        self.artifacts
    }
}

impl fmt::Debug for GeneratedPresentation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let binding_count = self
            .artifacts
            .iter()
            .map(|artifact| artifact.bindings.len())
            .sum::<usize>();
        let byte_length = self
            .artifacts
            .iter()
            .map(|artifact| artifact.bytes.len())
            .sum::<usize>();
        formatter
            .debug_struct("GeneratedPresentation")
            .field("selection_count", &self.plan.as_slice().len())
            .field("artifact_count", &self.artifacts.len())
            .field("binding_count", &binding_count)
            .field("byte_length", &byte_length)
            .finish_non_exhaustive()
    }
}

/// Value-free owner-private record of one generated disclosure selection.
#[must_use]
#[derive(Clone, PartialEq, Eq)]
pub struct PresentationReceiptEntry {
    query_id: PresentationQueryId,
    credential_handle: PresentationCredentialHandle,
    format: CredentialFormat,
    selected_claims: Vec<PresentationSelectedClaim>,
}

impl PresentationReceiptEntry {
    /// Return the request query represented by this receipt entry.
    pub const fn query_id(&self) -> &PresentationQueryId {
        &self.query_id
    }

    /// Return the owner-private local credential handle.
    pub const fn credential_handle(&self) -> &PresentationCredentialHandle {
        &self.credential_handle
    }

    /// Return the selected credential's format.
    pub const fn format(&self) -> &CredentialFormat {
        &self.format
    }

    /// Borrow the value-free selected claims in disclosure-plan order.
    pub fn selected_claims(&self) -> &[PresentationSelectedClaim] {
        &self.selected_claims
    }
}

impl fmt::Debug for PresentationReceiptEntry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PresentationReceiptEntry")
            .field("format", &self.format)
            .field("claim_count", &self.selected_claims.len())
            .finish_non_exhaustive()
    }
}

/// Value-free input for a consumer-owned presentation receipt service.
#[must_use]
#[derive(Clone, PartialEq, Eq)]
pub struct PresentationReceiptInput {
    verifier: CredentialEntityId,
    purpose: Option<PresentationPurpose>,
    entries: Vec<PresentationReceiptEntry>,
}

impl PresentationReceiptInput {
    fn from_generated(generated: &GeneratedPresentation) -> Self {
        let request = generated.plan.request();
        let entries = generated
            .plan
            .as_slice()
            .iter()
            .map(|selection| {
                let query = request
                    .query(selection.query_id())
                    .expect("validated disclosure plan query must remain present");
                PresentationReceiptEntry {
                    query_id: selection.query_id().clone(),
                    credential_handle: selection.credential_handle().clone(),
                    format: query.format().clone(),
                    selected_claims: selection.selected_claims().to_vec(),
                }
            })
            .collect();
        Self {
            verifier: request.verifier().clone(),
            purpose: request.purpose().cloned(),
            entries,
        }
    }

    /// Return the verifier from the exact validated presentation request.
    pub const fn verifier(&self) -> &CredentialEntityId {
        &self.verifier
    }

    /// Return the optional purpose without retaining the request challenge.
    pub const fn purpose(&self) -> Option<&PresentationPurpose> {
        self.purpose.as_ref()
    }

    /// Borrow receipt entries in disclosure-plan order.
    pub fn entries(&self) -> &[PresentationReceiptEntry] {
        &self.entries
    }
}

impl fmt::Debug for PresentationReceiptInput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PresentationReceiptInput")
            .field("has_purpose", &self.purpose.is_some())
            .field("entry_count", &self.entries.len())
            .finish_non_exhaustive()
    }
}
