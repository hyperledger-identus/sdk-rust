use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer, Serialize, de};
use serde_json::Value;

use crate::{
    DidDocument, Error, Service, Uri, VerificationMethod,
    document::{JsonBudget, validate_json_map, validate_json_value},
    error::ResolutionError,
    json_cleanup::{RejectionGuard, drop_json_values_iteratively},
};

use super::{
    document_metadata::DidDocumentMetadata,
    operation::DidUrlDereferencingMetadata,
    wire::{MAX_DID_RESOLUTION_RESULT_BYTES, validate_resolution_wire},
};

/// Bounded open JSON returned by serialized DID URL dereferencing.
///
/// Native non-JSON bytes remain binding-owned and must be paired with an exact
/// media type. This type never guesses a text or base64 representation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct DereferencedContent(Value);

impl DereferencedContent {
    /// Validate arbitrary non-null JSON content.
    pub fn new(value: Value) -> Result<Self, Error> {
        let content = RejectionGuard::new(Self(value), drop_dereferenced_content_json);
        if content.owner().0.is_null() {
            return Err(Error::InvalidResolution(ResolutionError::InvalidContent));
        }
        validate_json_value(&content.owner().0, 0, &mut JsonBudget::default())?;
        Ok(content.into_owner())
    }

    /// Construct content from a validated DID document.
    pub fn from_did_document(value: &DidDocument) -> Result<Self, Error> {
        Self::from_serializable(value)
    }

    /// Construct content from a validated verification method.
    pub fn from_verification_method(value: &VerificationMethod) -> Result<Self, Error> {
        Self::from_serializable(value)
    }

    /// Construct content from a validated service.
    pub fn from_service(value: &Service) -> Result<Self, Error> {
        Self::from_serializable(value)
    }

    /// Construct content from a validated URI.
    pub fn from_uri(value: &Uri) -> Result<Self, Error> {
        Self::from_serializable(value)
    }

    /// Borrow the exact semantic JSON value.
    #[must_use]
    pub const fn as_value(&self) -> &Value {
        &self.0
    }

    /// Validate this content as a DID document.
    pub fn to_did_document(&self) -> Result<DidDocument, Error> {
        self.project()
    }

    /// Validate this content as a verification method.
    pub fn to_verification_method(&self) -> Result<VerificationMethod, Error> {
        self.project()
    }

    /// Validate this content as a service.
    pub fn to_service(&self) -> Result<Service, Error> {
        self.project()
    }

    /// Validate this content as an absolute URI string.
    pub fn to_uri(&self) -> Result<Uri, Error> {
        self.project()
    }

    fn from_serializable<T: Serialize>(value: &T) -> Result<Self, Error> {
        serde_json::to_value(value)
            .map_err(|_| Error::InvalidResolution(ResolutionError::InvalidContent))
            .and_then(Self::new)
    }

    fn project<T: for<'de> Deserialize<'de>>(&self) -> Result<T, Error> {
        serde_json::from_value(self.0.clone())
            .map_err(|_| Error::InvalidResolution(ResolutionError::InvalidContent))
    }

    fn validate_with(&self, budget: &mut JsonBudget) -> Result<(), Error> {
        validate_json_value(&self.0, 0, budget)
    }
}

fn drop_dereferenced_content_json(content: DereferencedContent) {
    let DereferencedContent(value) = content;
    drop_json_values_iteratively([value]);
}

impl<'de> Deserialize<'de> for DereferencedContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::new(Value::deserialize(deserializer)?).map_err(de::Error::custom)
    }
}

/// Bounded open metadata about dereferenced content.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct DidUrlContentMetadata {
    #[serde(flatten)]
    values: BTreeMap<String, Value>,
}

impl DidUrlContentMetadata {
    /// Construct bounded open content metadata.
    pub fn new(values: BTreeMap<String, Value>) -> Result<Self, Error> {
        let metadata = RejectionGuard::new(Self { values }, drop_did_url_content_metadata_json);
        validate_json_map(&metadata.owner().values, &[], &mut JsonBudget::default())?;
        Ok(metadata.into_owner())
    }

    fn validate_with(&self, budget: &mut JsonBudget) -> Result<(), Error> {
        validate_json_map(&self.values, &[], budget)
    }

    /// Return empty content metadata.
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    /// Whether this metadata has no members.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Borrow all content metadata members.
    #[must_use]
    pub const fn values(&self) -> &BTreeMap<String, Value> {
        &self.values
    }

    /// Convert typed DID document metadata into content metadata.
    pub fn from_document_metadata(value: &DidDocumentMetadata) -> Result<Self, Error> {
        let Value::Object(values) = serde_json::to_value(value)
            .map_err(|_| Error::InvalidResolution(ResolutionError::InvalidContent))?
        else {
            return Err(Error::InvalidResolution(ResolutionError::InvalidContent));
        };
        Self::new(values.into_iter().collect())
    }

    /// Validate this metadata as common DID document metadata.
    pub fn to_document_metadata(&self) -> Result<DidDocumentMetadata, Error> {
        serde_json::from_value(Value::Object(self.values.clone().into_iter().collect()))
            .map_err(|_| Error::InvalidResolution(ResolutionError::InvalidContent))
    }
}

fn drop_did_url_content_metadata_json(metadata: DidUrlContentMetadata) {
    let DidUrlContentMetadata { values } = metadata;
    drop_json_values_iteratively(values.into_values());
}

impl<'de> Deserialize<'de> for DidUrlContentMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        BTreeMap::<String, Value>::deserialize(deserializer)
            .and_then(|values| Self::new(values).map_err(de::Error::custom))
    }
}

/// A validated serialized DID URL dereferencing result envelope.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DidUrlDereferencingResult {
    #[serde(rename = "didUrlDereferencingMetadata")]
    metadata: DidUrlDereferencingMetadata,
    content: Option<DereferencedContent>,
    #[serde(rename = "contentMetadata")]
    content_metadata: DidUrlContentMetadata,
}

impl DidUrlDereferencingResult {
    /// Construct a successful dereferencing result.
    pub fn success(
        metadata: DidUrlDereferencingMetadata,
        content: DereferencedContent,
        content_metadata: DidUrlContentMetadata,
    ) -> Result<Self, Error> {
        Self::validated(metadata, Some(content), content_metadata)
    }

    /// Construct a failed dereferencing result.
    pub fn failure(metadata: DidUrlDereferencingMetadata) -> Result<Self, Error> {
        Self::validated(metadata, None, DidUrlContentMetadata::empty())
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

    /// Parse a bounded JSON dereferencing result string.
    pub fn from_json_str(input: &str) -> Result<Self, Error> {
        Self::from_json_slice(input.as_bytes())
    }

    /// Borrow dereferencing-operation metadata.
    #[must_use]
    pub const fn metadata(&self) -> &DidUrlDereferencingMetadata {
        &self.metadata
    }

    /// Borrow dereferenced content when present.
    #[must_use]
    pub const fn content(&self) -> Option<&DereferencedContent> {
        self.content.as_ref()
    }

    /// Borrow content metadata.
    #[must_use]
    pub const fn content_metadata(&self) -> &DidUrlContentMetadata {
        &self.content_metadata
    }

    fn validated(
        metadata: DidUrlDereferencingMetadata,
        content: Option<DereferencedContent>,
        content_metadata: DidUrlContentMetadata,
    ) -> Result<Self, Error> {
        let result = Self {
            metadata,
            content,
            content_metadata,
        };
        result.validate()?;
        Ok(result)
    }

    fn validate(&self) -> Result<(), Error> {
        let mut budget = JsonBudget::default();
        self.metadata.validate_with(&mut budget)?;
        if let Some(content) = &self.content {
            content.validate_with(&mut budget)?;
        }
        self.content_metadata.validate_with(&mut budget)?;
        match (self.content.is_some(), self.metadata.error().is_some()) {
            (true, false) => Ok(()),
            (false, true) if self.content_metadata.is_empty() => Ok(()),
            _ => Err(Error::InvalidResolution(ResolutionError::InvalidState)),
        }
    }
}

impl<'de> Deserialize<'de> for DidUrlDereferencingResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            #[serde(rename = "didUrlDereferencingMetadata")]
            metadata: DidUrlDereferencingMetadata,
            #[serde(deserialize_with = "deserialize_nullable")]
            content: Option<DereferencedContent>,
            #[serde(rename = "contentMetadata")]
            content_metadata: DidUrlContentMetadata,
        }

        let wire = Wire::deserialize(deserializer)?;
        Self::validated(wire.metadata, wire.content, wire.content_metadata)
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
