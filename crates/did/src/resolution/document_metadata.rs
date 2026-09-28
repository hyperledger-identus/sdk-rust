use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer, Serialize, de};
use serde_json::Value;

use crate::{
    Did, Error,
    document::{JsonBudget, validate_json_map},
    error::ResolutionError,
    json_cleanup::{RejectionGuard, drop_json_values_iteratively},
};

use super::value::{DidResolutionDateTime, VersionId};

const DOCUMENT_METADATA_RESERVED: &[&str] = &[
    "created",
    "updated",
    "deactivated",
    "nextUpdate",
    "versionId",
    "nextVersionId",
    "equivalentId",
    "canonicalId",
];
/// Common and method-defined metadata about a resolved DID document.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct DidDocumentMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<DidResolutionDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated: Option<DidResolutionDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deactivated: Option<bool>,
    #[serde(rename = "nextUpdate", skip_serializing_if = "Option::is_none")]
    next_update: Option<DidResolutionDateTime>,
    #[serde(rename = "versionId", skip_serializing_if = "Option::is_none")]
    version_id: Option<VersionId>,
    #[serde(rename = "nextVersionId", skip_serializing_if = "Option::is_none")]
    next_version_id: Option<VersionId>,
    #[serde(rename = "equivalentId", skip_serializing_if = "Option::is_none")]
    equivalent_id: Option<Vec<Did>>,
    #[serde(rename = "canonicalId", skip_serializing_if = "Option::is_none")]
    canonical_id: Option<Did>,
    #[serde(flatten)]
    extensions: BTreeMap<String, Value>,
}

impl DidDocumentMetadata {
    /// Begin native construction of validated metadata.
    #[must_use]
    pub fn builder() -> DidDocumentMetadataBuilder {
        DidDocumentMetadataBuilder::default()
    }

    /// Return empty metadata.
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    /// Whether no common or extension member is present.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }

    /// Borrow the creation time.
    #[must_use]
    pub const fn created(&self) -> Option<&DidResolutionDateTime> {
        self.created.as_ref()
    }

    /// Borrow the update time.
    #[must_use]
    pub const fn updated(&self) -> Option<&DidResolutionDateTime> {
        self.updated.as_ref()
    }

    /// Return the explicit deactivation flag.
    #[must_use]
    pub const fn deactivated(&self) -> Option<bool> {
        self.deactivated
    }

    /// Borrow the next update time.
    #[must_use]
    pub const fn next_update(&self) -> Option<&DidResolutionDateTime> {
        self.next_update.as_ref()
    }

    /// Borrow the resolved version id.
    #[must_use]
    pub const fn version_id(&self) -> Option<&VersionId> {
        self.version_id.as_ref()
    }

    /// Borrow the next version id.
    #[must_use]
    pub const fn next_version_id(&self) -> Option<&VersionId> {
        self.next_version_id.as_ref()
    }

    /// Borrow equivalent DIDs.
    #[must_use]
    pub fn equivalent_ids(&self) -> Option<&[Did]> {
        self.equivalent_id.as_deref()
    }

    /// Borrow the canonical DID.
    #[must_use]
    pub const fn canonical_id(&self) -> Option<&Did> {
        self.canonical_id.as_ref()
    }

    /// Borrow proof and method-defined members.
    #[must_use]
    pub const fn extensions(&self) -> &BTreeMap<String, Value> {
        &self.extensions
    }

    /// Validate identifiers against a resolved or requested DID.
    pub fn validate_for(&self, did: &Did) -> Result<(), Error> {
        self.validate()?;
        self.validate_methods_for(did)
    }

    pub(super) fn validate_methods_for(&self, did: &Did) -> Result<(), Error> {
        if let Some(equivalent_ids) = &self.equivalent_id
            && equivalent_ids
                .iter()
                .any(|equivalent| equivalent.method() != did.method())
        {
            return Err(Error::InvalidResolution(
                ResolutionError::DifferentDidMethod,
            ));
        }
        if self
            .canonical_id
            .as_ref()
            .is_some_and(|canonical| canonical.method() != did.method())
        {
            return Err(Error::InvalidResolution(
                ResolutionError::DifferentDidMethod,
            ));
        }
        Ok(())
    }

    fn validate(&self) -> Result<(), Error> {
        self.validate_with(&mut JsonBudget::default())
    }

    pub(super) fn validate_with(&self, budget: &mut JsonBudget) -> Result<(), Error> {
        if let Some(equivalent_ids) = &self.equivalent_id {
            if equivalent_ids.is_empty() {
                return Err(Error::InvalidResolution(ResolutionError::EmptyValue));
            }
            if equivalent_ids.len() > crate::MAX_DOCUMENT_ITEMS {
                return Err(Error::InvalidResolution(ResolutionError::TooManyItems));
            }
            for (index, did) in equivalent_ids.iter().enumerate() {
                if equivalent_ids[..index].contains(did) {
                    return Err(Error::InvalidResolution(
                        ResolutionError::DuplicateEquivalentId,
                    ));
                }
            }
        }
        validate_json_map(&self.extensions, DOCUMENT_METADATA_RESERVED, budget)
    }
}

impl<'de> Deserialize<'de> for DidDocumentMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            created: Option<DidResolutionDateTime>,
            updated: Option<DidResolutionDateTime>,
            deactivated: Option<bool>,
            #[serde(rename = "nextUpdate")]
            next_update: Option<DidResolutionDateTime>,
            #[serde(rename = "versionId")]
            version_id: Option<VersionId>,
            #[serde(rename = "nextVersionId")]
            next_version_id: Option<VersionId>,
            #[serde(rename = "equivalentId")]
            equivalent_id: Option<Vec<Did>>,
            #[serde(rename = "canonicalId")]
            canonical_id: Option<Did>,
            #[serde(flatten)]
            extensions: BTreeMap<String, Value>,
        }

        let wire = Wire::deserialize(deserializer)?;
        let metadata = RejectionGuard::new(
            Self {
                created: wire.created,
                updated: wire.updated,
                deactivated: wire.deactivated,
                next_update: wire.next_update,
                version_id: wire.version_id,
                next_version_id: wire.next_version_id,
                equivalent_id: wire.equivalent_id,
                canonical_id: wire.canonical_id,
                extensions: wire.extensions,
            },
            drop_did_document_metadata_json,
        );
        metadata.owner().validate().map_err(de::Error::custom)?;
        Ok(metadata.into_owner())
    }
}

/// Builder for immutable [`DidDocumentMetadata`].
#[derive(Clone, Debug, Default)]
pub struct DidDocumentMetadataBuilder(DidDocumentMetadata);

impl DidDocumentMetadataBuilder {
    /// Set the creation time.
    #[must_use]
    pub fn created(mut self, value: DidResolutionDateTime) -> Self {
        self.0.created = Some(value);
        self
    }

    /// Set the last update time.
    #[must_use]
    pub fn updated(mut self, value: DidResolutionDateTime) -> Self {
        self.0.updated = Some(value);
        self
    }

    /// Set the deactivation flag.
    #[must_use]
    pub fn deactivated(mut self, value: bool) -> Self {
        self.0.deactivated = Some(value);
        self
    }

    /// Set the next update time.
    #[must_use]
    pub fn next_update(mut self, value: DidResolutionDateTime) -> Self {
        self.0.next_update = Some(value);
        self
    }

    /// Set the resolved version id.
    #[must_use]
    pub fn version_id(mut self, value: VersionId) -> Self {
        self.0.version_id = Some(value);
        self
    }

    /// Set the next version id.
    #[must_use]
    pub fn next_version_id(mut self, value: VersionId) -> Self {
        self.0.next_version_id = Some(value);
        self
    }

    /// Set the non-empty equivalent DID set.
    #[must_use]
    pub fn equivalent_ids(mut self, values: Vec<Did>) -> Self {
        self.0.equivalent_id = Some(values);
        self
    }

    /// Set the canonical DID.
    #[must_use]
    pub fn canonical_id(mut self, value: Did) -> Self {
        self.0.canonical_id = Some(value);
        self
    }

    /// Set proof and method-defined metadata.
    #[must_use]
    pub fn extensions(mut self, values: BTreeMap<String, Value>) -> Self {
        self.0.extensions = values;
        self
    }

    /// Validate and return the immutable metadata.
    pub fn build(self) -> Result<DidDocumentMetadata, Error> {
        let metadata = RejectionGuard::new(self.0, drop_did_document_metadata_json);
        metadata.owner().validate()?;
        Ok(metadata.into_owner())
    }
}

fn drop_did_document_metadata_json(metadata: DidDocumentMetadata) {
    let DidDocumentMetadata {
        created: _,
        updated: _,
        deactivated: _,
        next_update: _,
        version_id: _,
        next_version_id: _,
        equivalent_id: _,
        canonical_id: _,
        extensions,
    } = metadata;
    drop_json_values_iteratively(extensions.into_values());
}
