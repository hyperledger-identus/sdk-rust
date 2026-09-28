use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer, Serialize, de};

use crate::{Did, DidDocument, Error, document::JsonBudget, error::ResolutionError};

use super::{
    document_metadata::DidDocumentMetadata,
    operation::{DidResolutionError, DidResolutionErrorKind, DidResolutionMetadata},
    wire::{MAX_DID_RESOLUTION_RESULT_BYTES, validate_resolution_wire},
};

/// A validated W3C DID resolution result envelope.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DidResolutionResult {
    #[serde(rename = "didResolutionMetadata")]
    metadata: DidResolutionMetadata,
    #[serde(rename = "didDocument")]
    document: Option<DidDocument>,
    #[serde(rename = "didDocumentMetadata")]
    document_metadata: DidDocumentMetadata,
}

impl DidResolutionResult {
    /// Construct a successful result.
    pub fn success(
        metadata: DidResolutionMetadata,
        document: DidDocument,
        document_metadata: DidDocumentMetadata,
    ) -> Result<Self, Error> {
        Self::validated(metadata, Some(document), document_metadata)
    }

    /// Construct an ordinary failed result.
    pub fn failure(metadata: DidResolutionMetadata) -> Result<Self, Error> {
        Self::validated(metadata, None, DidDocumentMetadata::empty())
    }

    /// Construct a deactivated result.
    pub fn deactivated(
        metadata: DidResolutionMetadata,
        document_metadata: DidDocumentMetadata,
    ) -> Result<Self, Error> {
        Self::validated(metadata, None, document_metadata)
    }

    /// Parse bounded, duplicate-free JSON from an untrusted byte boundary.
    pub fn from_json_slice(input: &[u8]) -> Result<Self, Error> {
        if input.len() > MAX_DID_RESOLUTION_RESULT_BYTES {
            return Err(Error::InvalidResolution(ResolutionError::TooLarge));
        }
        validate_resolution_wire(input)?;
        serde_json::from_slice(input)
            .map_err(|_| Error::InvalidResolution(ResolutionError::MalformedJson))
    }

    /// Parse a bounded JSON result string.
    pub fn from_json_str(input: &str) -> Result<Self, Error> {
        Self::from_json_slice(input.as_bytes())
    }

    /// Parse and validate a bounded result for the requested DID.
    pub fn from_json_slice_for(input: &[u8], did: &Did) -> Result<Self, Error> {
        let result = Self::from_json_slice(input)?;
        result.validate_for(did)?;
        Ok(result)
    }

    /// Parse and validate a bounded result string for the requested DID.
    pub fn from_json_str_for(input: &str, did: &Did) -> Result<Self, Error> {
        Self::from_json_slice_for(input.as_bytes(), did)
    }

    /// Validate document identity and metadata method against the request.
    pub fn validate_for(&self, did: &Did) -> Result<(), Error> {
        self.validate()?;
        if self
            .document
            .as_ref()
            .is_some_and(|document| document.id() != did)
        {
            return Err(Error::InvalidResolution(
                ResolutionError::DocumentIdMismatch,
            ));
        }
        self.document_metadata.validate_methods_for(did)
    }

    /// Borrow resolution-operation metadata.
    #[must_use]
    pub const fn metadata(&self) -> &DidResolutionMetadata {
        &self.metadata
    }

    /// Borrow the resolved document when present.
    #[must_use]
    pub const fn document(&self) -> Option<&DidDocument> {
        self.document.as_ref()
    }

    /// Borrow document metadata.
    #[must_use]
    pub const fn document_metadata(&self) -> &DidDocumentMetadata {
        &self.document_metadata
    }

    fn validated(
        metadata: DidResolutionMetadata,
        document: Option<DidDocument>,
        document_metadata: DidDocumentMetadata,
    ) -> Result<Self, Error> {
        let result = Self {
            metadata,
            document,
            document_metadata,
        };
        result.validate()?;
        if let Some(document) = &result.document {
            result
                .document_metadata
                .validate_methods_for(document.id())?;
        }
        Ok(result)
    }

    fn validate(&self) -> Result<(), Error> {
        let mut budget = JsonBudget::default();
        self.metadata.validate_with(&mut budget)?;
        self.document_metadata.validate_with(&mut budget)?;
        match (
            self.document.is_some(),
            self.metadata.error().is_some(),
            self.document_metadata.deactivated(),
        ) {
            (true, false, Some(true)) | (true, true, _) => {
                Err(Error::InvalidResolution(ResolutionError::InvalidState))
            }
            (true, false, _) => Ok(()),
            (false, true, _) if self.document_metadata.is_empty() => Ok(()),
            (false, false, Some(true)) => Ok(()),
            _ => Err(Error::InvalidResolution(ResolutionError::InvalidState)),
        }
    }
}

/// Construct a standard, content-free DID resolution failure for crate-owned
/// resolver orchestration.
pub(crate) fn standard_resolution_failure(kind: DidResolutionErrorKind) -> DidResolutionResult {
    let metadata = DidResolutionMetadata::new(
        None,
        Some(DidResolutionError::standard(kind)),
        BTreeMap::new(),
    )
    .expect("standard error metadata is valid");
    DidResolutionResult::failure(metadata).expect("standard resolution failure is valid")
}

impl<'de> Deserialize<'de> for DidResolutionResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            #[serde(rename = "didResolutionMetadata")]
            metadata: DidResolutionMetadata,
            #[serde(rename = "didDocument")]
            #[serde(deserialize_with = "deserialize_nullable")]
            document: Option<DidDocument>,
            #[serde(rename = "didDocumentMetadata")]
            document_metadata: DidDocumentMetadata,
        }

        let wire = Wire::deserialize(deserializer)?;
        Self::validated(wire.metadata, wire.document, wire.document_metadata)
            .map_err(de::Error::custom)
    }
}

fn deserialize_nullable<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::<T>::deserialize(deserializer)
}
