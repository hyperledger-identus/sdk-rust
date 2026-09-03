//! Transport-free W3C DID resolution and DID URL dereferencing results.
//!
//! This module owns bounded result data only. Resolution algorithms, DID
//! methods, network bindings, caching and trust policy belong in higher rings.
//! Use the explicit `from_json_*` result entry points for untrusted bytes: they
//! reject duplicate decoded names before typed deserialization. Direct serde is
//! a semantic conversion for representations whose unique-name property has
//! already been established.

use std::{collections::BTreeMap, fmt, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use serde_json::Value;

use crate::{
    Did, DidDocument, Error, Service, Uri, VerificationMethod,
    document::{JsonBudget, validate_json_map, validate_json_value},
    error::ResolutionError,
    wire_json::{JsonWireError, JsonWireLimits, validate_unique_object_names},
};

/// Maximum raw JSON size accepted by resolution result entry points.
pub const MAX_DID_RESOLUTION_RESULT_BYTES: usize = 512 * 1_024;
/// Maximum containers nested in raw resolution result JSON during preflight.
pub const MAX_DID_RESOLUTION_WIRE_DEPTH: usize = 64;
/// Maximum JSON values visited during raw resolution result preflight.
pub const MAX_DID_RESOLUTION_WIRE_NODES: usize = 16_384;
/// Maximum members permitted in one raw resolution result JSON object.
pub const MAX_DID_RESOLUTION_WIRE_OBJECT_MEMBERS: usize = 128;
/// Maximum decoded object-name bytes retained simultaneously during preflight.
pub const MAX_DID_RESOLUTION_WIRE_LIVE_KEY_BYTES: usize = 128 * 1_024;
/// Maximum byte length of a media type.
pub const MAX_MEDIA_TYPE_BYTES: usize = 1_024;
/// Maximum byte length of a DID Resolution datetime.
pub const MAX_DID_RESOLUTION_DATETIME_BYTES: usize = 128;
/// Maximum byte length of a version identifier.
pub const MAX_VERSION_ID_BYTES: usize = 1_024;
/// Maximum byte length of a problem title or detail.
pub const MAX_PROBLEM_DETAIL_BYTES: usize = 4_096;

const MAX_PROBLEM_TITLE_BYTES: usize = 1_024;

const ERROR_RESERVED: &[&str] = &["type", "title", "detail", "instance"];
const OPERATION_METADATA_RESERVED: &[&str] = &["contentType", "error"];
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

const INVALID_DID_URI: &str = "https://www.w3.org/ns/did#INVALID_DID";
const INVALID_DID_DOCUMENT_URI: &str = "https://www.w3.org/ns/did#INVALID_DID_DOCUMENT";
const NOT_FOUND_URI: &str = "https://www.w3.org/ns/did#NOT_FOUND";
const REPRESENTATION_NOT_SUPPORTED_URI: &str =
    "https://www.w3.org/ns/did#REPRESENTATION_NOT_SUPPORTED";
const INVALID_DID_URL_URI: &str = "https://www.w3.org/ns/did#INVALID_DID_URL";
const METHOD_NOT_SUPPORTED_URI: &str = "https://www.w3.org/ns/did#METHOD_NOT_SUPPORTED";
const INVALID_OPTIONS_URI: &str = "https://www.w3.org/ns/did#INVALID_OPTIONS";
const INTERNAL_ERROR_URI: &str = "https://www.w3.org/ns/did#INTERNAL_ERROR";
const FEATURE_NOT_SUPPORTED_URI: &str = "https://www.w3.org/ns/did#FEATURE_NOT_SUPPORTED";

/// A bounded, syntactically valid media type value.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MediaType(String);

impl MediaType {
    /// Parse a single media type and preserve its exact valid spelling.
    pub fn parse(value: &str) -> Result<Self, Error> {
        validate_media_type(value)?;
        Ok(Self(value.to_owned()))
    }

    /// Validate an owned media type while retaining its allocation.
    pub fn try_new(value: String) -> Result<Self, Error> {
        validate_media_type(&value)?;
        Ok(Self(value))
    }

    /// Borrow the exact validated value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume this value.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

/// A bounded XML Schema 1.1 whole-second UTC datetime used by DID Resolution.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DidResolutionDateTime(String);

impl DidResolutionDateTime {
    /// Parse the XML Schema 1.1 profile adjusted to UTC whole seconds.
    pub fn parse(value: &str) -> Result<Self, Error> {
        validate_datetime(value)?;
        Ok(Self(value.to_owned()))
    }

    /// Validate an owned datetime while retaining its allocation.
    pub fn try_new(value: String) -> Result<Self, Error> {
        validate_datetime(&value)?;
        Ok(Self(value))
    }

    /// Borrow the exact validated value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume this value.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

/// A bounded opaque DID document version identifier.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VersionId(String);

impl VersionId {
    /// Parse a non-empty, trimmed printable ASCII version id.
    pub fn parse(value: &str) -> Result<Self, Error> {
        validate_version_id(value)?;
        Ok(Self(value.to_owned()))
    }

    /// Validate an owned version id while retaining its allocation.
    pub fn try_new(value: String) -> Result<Self, Error> {
        validate_version_id(&value)?;
        Ok(Self(value))
    }

    /// Borrow the exact validated value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume this value.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

macro_rules! impl_string_value {
    ($name:ty) => {
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl FromStr for $name {
            type Err = Error;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::parse(value)
            }
        }

        impl TryFrom<String> for $name {
            type Error = Error;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(value)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;
                Self::try_new(value).map_err(de::Error::custom)
            }
        }
    };
}

impl_string_value!(MediaType);
impl_string_value!(DidResolutionDateTime);
impl_string_value!(VersionId);

/// The nine standard error type URLs in the pinned W3C Candidate
/// Recommendation Draft.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DidResolutionErrorKind {
    /// The input DID is invalid.
    InvalidDid,
    /// The resolved DID document is invalid.
    InvalidDidDocument,
    /// The requested DID or resource was not found.
    NotFound,
    /// The requested representation is unsupported.
    RepresentationNotSupported,
    /// The input DID URL is invalid.
    InvalidDidUrl,
    /// The DID method is unsupported.
    MethodNotSupported,
    /// One or more options are invalid.
    InvalidOptions,
    /// Resolution failed unexpectedly.
    InternalError,
    /// The requested feature is unsupported.
    FeatureNotSupported,
}

impl DidResolutionErrorKind {
    /// Return the standard absolute error type URL.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidDid => INVALID_DID_URI,
            Self::InvalidDidDocument => INVALID_DID_DOCUMENT_URI,
            Self::NotFound => NOT_FOUND_URI,
            Self::RepresentationNotSupported => REPRESENTATION_NOT_SUPPORTED_URI,
            Self::InvalidDidUrl => INVALID_DID_URL_URI,
            Self::MethodNotSupported => METHOD_NOT_SUPPORTED_URI,
            Self::InvalidOptions => INVALID_OPTIONS_URI,
            Self::InternalError => INTERNAL_ERROR_URI,
            Self::FeatureNotSupported => FEATURE_NOT_SUPPORTED_URI,
        }
    }

    fn from_uri(value: &str) -> Option<Self> {
        Some(match value {
            INVALID_DID_URI => Self::InvalidDid,
            INVALID_DID_DOCUMENT_URI => Self::InvalidDidDocument,
            NOT_FOUND_URI => Self::NotFound,
            REPRESENTATION_NOT_SUPPORTED_URI => Self::RepresentationNotSupported,
            INVALID_DID_URL_URI => Self::InvalidDidUrl,
            METHOD_NOT_SUPPORTED_URI => Self::MethodNotSupported,
            INVALID_OPTIONS_URI => Self::InvalidOptions,
            INTERNAL_ERROR_URI => Self::InternalError,
            FEATURE_NOT_SUPPORTED_URI => Self::FeatureNotSupported,
            _ => return None,
        })
    }

    fn from_legacy(value: &str) -> Option<Self> {
        Some(match value {
            "invalidDid" => Self::InvalidDid,
            "invalidDidDocument" => Self::InvalidDidDocument,
            "notFound" => Self::NotFound,
            "representationNotSupported" => Self::RepresentationNotSupported,
            "invalidDidUrl" => Self::InvalidDidUrl,
            "methodNotSupported" => Self::MethodNotSupported,
            "invalidOptions" => Self::InvalidOptions,
            "internalError" => Self::InternalError,
            "featureNotSupported" => Self::FeatureNotSupported,
            _ => return None,
        })
    }
}

/// A bounded RFC 9457-style DID resolution error object.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DidResolutionError {
    #[serde(rename = "type")]
    type_: Uri,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<Uri>,
    #[serde(flatten)]
    extensions: BTreeMap<String, Value>,
}

impl DidResolutionError {
    /// Construct and validate a resolution error object.
    pub fn new(
        type_: Uri,
        title: Option<String>,
        detail: Option<String>,
        instance: Option<Uri>,
        extensions: BTreeMap<String, Value>,
    ) -> Result<Self, Error> {
        let value = Self {
            type_,
            title,
            detail,
            instance,
            extensions,
        };
        value.validate()?;
        Ok(value)
    }

    /// Construct a standard W3C error without optional problem details.
    pub fn standard(kind: DidResolutionErrorKind) -> Self {
        Self {
            type_: Uri::parse(kind.as_str()).expect("standard error URL is valid"),
            title: None,
            detail: None,
            instance: None,
            extensions: BTreeMap::new(),
        }
    }

    /// Deliberately migrate a recognized legacy estate keyword.
    ///
    /// Strict result JSON never accepts these keywords directly. An adapter
    /// calls this helper before constructing current URL-valued error metadata.
    pub fn from_legacy_keyword(value: &str) -> Result<Self, Error> {
        DidResolutionErrorKind::from_legacy(value)
            .map(Self::standard)
            .ok_or_else(|| invalid(ResolutionError::InvalidString))
    }

    /// Borrow the exact absolute error type URI.
    #[must_use]
    pub const fn type_uri(&self) -> &Uri {
        &self.type_
    }

    /// Classify this error when it uses a standard W3C URL.
    #[must_use]
    pub fn kind(&self) -> Option<DidResolutionErrorKind> {
        DidResolutionErrorKind::from_uri(self.type_.as_str())
    }

    /// Borrow the optional short human-readable title.
    #[must_use]
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Borrow the optional human-readable detail.
    #[must_use]
    pub fn detail(&self) -> Option<&str> {
        self.detail.as_deref()
    }

    /// Borrow the optional problem occurrence URI.
    #[must_use]
    pub const fn instance(&self) -> Option<&Uri> {
        self.instance.as_ref()
    }

    /// Borrow extension members.
    #[must_use]
    pub const fn extensions(&self) -> &BTreeMap<String, Value> {
        &self.extensions
    }

    fn validate(&self) -> Result<(), Error> {
        self.validate_with(&mut JsonBudget::default())
    }

    fn validate_with(&self, budget: &mut JsonBudget) -> Result<(), Error> {
        if let Some(title) = &self.title {
            validate_problem_text(title, MAX_PROBLEM_TITLE_BYTES)?;
        }
        if let Some(detail) = &self.detail {
            validate_problem_text(detail, MAX_PROBLEM_DETAIL_BYTES)?;
        }
        validate_extensions_with(&self.extensions, ERROR_RESERVED, budget)
    }
}

impl<'de> Deserialize<'de> for DidResolutionError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            #[serde(rename = "type")]
            type_: Uri,
            title: Option<String>,
            detail: Option<String>,
            instance: Option<Uri>,
            #[serde(flatten)]
            extensions: BTreeMap<String, Value>,
        }

        let wire = Wire::deserialize(deserializer)?;
        Self::new(
            wire.type_,
            wire.title,
            wire.detail,
            wire.instance,
            wire.extensions,
        )
        .map_err(de::Error::custom)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
struct OperationMetadata {
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    content_type: Option<MediaType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<DidResolutionError>,
    #[serde(flatten)]
    extensions: BTreeMap<String, Value>,
}

impl OperationMetadata {
    fn new(
        content_type: Option<MediaType>,
        error: Option<DidResolutionError>,
        extensions: BTreeMap<String, Value>,
    ) -> Result<Self, Error> {
        let metadata = Self {
            content_type,
            error,
            extensions,
        };
        metadata.validate()?;
        Ok(metadata)
    }

    fn validate(&self) -> Result<(), Error> {
        self.validate_with(&mut JsonBudget::default())
    }

    fn validate_with(&self, budget: &mut JsonBudget) -> Result<(), Error> {
        if let Some(error) = &self.error {
            error.validate_with(budget)?;
        }
        validate_extensions_with(&self.extensions, OPERATION_METADATA_RESERVED, budget)
    }
}

impl<'de> Deserialize<'de> for OperationMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            #[serde(rename = "contentType")]
            content_type: Option<MediaType>,
            error: Option<DidResolutionError>,
            #[serde(flatten)]
            extensions: BTreeMap<String, Value>,
        }

        let wire = Wire::deserialize(deserializer)?;
        Self::new(wire.content_type, wire.error, wire.extensions).map_err(de::Error::custom)
    }
}

macro_rules! operation_metadata {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(OperationMetadata);

        impl $name {
            /// Construct validated operation metadata.
            pub fn new(
                content_type: Option<MediaType>,
                error: Option<DidResolutionError>,
                extensions: BTreeMap<String, Value>,
            ) -> Result<Self, Error> {
                OperationMetadata::new(content_type, error, extensions).map(Self)
            }

            /// Return empty successful metadata.
            #[must_use]
            pub fn empty() -> Self {
                Self(OperationMetadata::default())
            }

            /// Borrow the returned content media type.
            #[must_use]
            pub const fn content_type(&self) -> Option<&MediaType> {
                self.0.content_type.as_ref()
            }

            /// Borrow the resolution problem, when present.
            #[must_use]
            pub const fn error(&self) -> Option<&DidResolutionError> {
                self.0.error.as_ref()
            }

            /// Borrow extension members.
            #[must_use]
            pub const fn extensions(&self) -> &BTreeMap<String, Value> {
                &self.0.extensions
            }
        }
    };
}

operation_metadata!(
    DidResolutionMetadata,
    "Metadata about one DID resolution operation."
);
operation_metadata!(
    DidUrlDereferencingMetadata,
    "Metadata about one DID URL dereferencing operation."
);

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

    fn validate_methods_for(&self, did: &Did) -> Result<(), Error> {
        if let Some(equivalent_ids) = &self.equivalent_id {
            if equivalent_ids
                .iter()
                .any(|equivalent| equivalent.method() != did.method())
            {
                return Err(invalid(ResolutionError::DifferentDidMethod));
            }
        }
        if self
            .canonical_id
            .as_ref()
            .is_some_and(|canonical| canonical.method() != did.method())
        {
            return Err(invalid(ResolutionError::DifferentDidMethod));
        }
        Ok(())
    }

    fn validate(&self) -> Result<(), Error> {
        self.validate_with(&mut JsonBudget::default())
    }

    fn validate_with(&self, budget: &mut JsonBudget) -> Result<(), Error> {
        if let Some(equivalent_ids) = &self.equivalent_id {
            if equivalent_ids.is_empty() {
                return Err(invalid(ResolutionError::EmptyValue));
            }
            if equivalent_ids.len() > crate::MAX_DOCUMENT_ITEMS {
                return Err(invalid(ResolutionError::TooManyItems));
            }
            for (index, did) in equivalent_ids.iter().enumerate() {
                if equivalent_ids[..index].contains(did) {
                    return Err(invalid(ResolutionError::DuplicateEquivalentId));
                }
            }
        }
        validate_extensions_with(&self.extensions, DOCUMENT_METADATA_RESERVED, budget)
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
        let metadata = Self {
            created: wire.created,
            updated: wire.updated,
            deactivated: wire.deactivated,
            next_update: wire.next_update,
            version_id: wire.version_id,
            next_version_id: wire.next_version_id,
            equivalent_id: wire.equivalent_id,
            canonical_id: wire.canonical_id,
            extensions: wire.extensions,
        };
        metadata.validate().map_err(de::Error::custom)?;
        Ok(metadata)
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
        self.0.validate()?;
        Ok(self.0)
    }
}

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
            return Err(invalid(ResolutionError::TooLarge));
        }
        validate_resolution_wire(input)?;
        serde_json::from_slice(input).map_err(|_| invalid(ResolutionError::MalformedJson))
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
            return Err(invalid(ResolutionError::DocumentIdMismatch));
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
        self.metadata.0.validate_with(&mut budget)?;
        self.document_metadata.validate_with(&mut budget)?;
        match (
            self.document.is_some(),
            self.metadata.error().is_some(),
            self.document_metadata.deactivated(),
        ) {
            (true, false, Some(true)) | (true, true, _) => {
                Err(invalid(ResolutionError::InvalidState))
            }
            (true, false, _) => Ok(()),
            (false, true, _) if self.document_metadata.is_empty() => Ok(()),
            (false, false, Some(true)) => Ok(()),
            _ => Err(invalid(ResolutionError::InvalidState)),
        }
    }
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
        if value.is_null() {
            return Err(invalid(ResolutionError::InvalidContent));
        }
        validate_json_value(&value, 0, &mut JsonBudget::default())?;
        Ok(Self(value))
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
            .map_err(|_| invalid(ResolutionError::InvalidContent))
            .and_then(Self::new)
    }

    fn project<T: for<'de> Deserialize<'de>>(&self) -> Result<T, Error> {
        serde_json::from_value(self.0.clone()).map_err(|_| invalid(ResolutionError::InvalidContent))
    }

    fn validate_with(&self, budget: &mut JsonBudget) -> Result<(), Error> {
        validate_json_value(&self.0, 0, budget)
    }
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
        validate_extensions(&values, &[])?;
        Ok(Self { values })
    }

    fn validate_with(&self, budget: &mut JsonBudget) -> Result<(), Error> {
        validate_extensions_with(&self.values, &[], budget)
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
        let Value::Object(values) =
            serde_json::to_value(value).map_err(|_| invalid(ResolutionError::InvalidContent))?
        else {
            return Err(invalid(ResolutionError::InvalidContent));
        };
        Self::new(values.into_iter().collect())
    }

    /// Validate this metadata as common DID document metadata.
    pub fn to_document_metadata(&self) -> Result<DidDocumentMetadata, Error> {
        serde_json::from_value(Value::Object(self.values.clone().into_iter().collect()))
            .map_err(|_| invalid(ResolutionError::InvalidContent))
    }
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
            return Err(invalid(ResolutionError::TooLarge));
        }
        validate_resolution_wire(input)?;
        serde_json::from_slice(input).map_err(|_| invalid(ResolutionError::MalformedJson))
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
        self.metadata.0.validate_with(&mut budget)?;
        if let Some(content) = &self.content {
            content.validate_with(&mut budget)?;
        }
        self.content_metadata.validate_with(&mut budget)?;
        match (self.content.is_some(), self.metadata.error().is_some()) {
            (true, false) => Ok(()),
            (false, true) if self.content_metadata.is_empty() => Ok(()),
            _ => Err(invalid(ResolutionError::InvalidState)),
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

fn validate_extensions(map: &BTreeMap<String, Value>, reserved: &[&str]) -> Result<(), Error> {
    validate_extensions_with(map, reserved, &mut JsonBudget::default())
}

fn validate_extensions_with(
    map: &BTreeMap<String, Value>,
    reserved: &[&str],
    budget: &mut JsonBudget,
) -> Result<(), Error> {
    validate_json_map(map, reserved, budget)
}

fn validate_problem_text(value: &str, max_bytes: usize) -> Result<(), Error> {
    if value.is_empty()
        || value.len() > max_bytes
        || value.chars().any(char::is_control)
        || value.trim() != value
    {
        return Err(invalid(ResolutionError::InvalidString));
    }
    Ok(())
}

fn validate_version_id(value: &str) -> Result<(), Error> {
    if value.is_empty()
        || value.len() > MAX_VERSION_ID_BYTES
        || !value.bytes().all(|byte| (0x20..=0x7e).contains(&byte))
        || value.trim() != value
    {
        return Err(invalid(ResolutionError::InvalidString));
    }
    Ok(())
}

fn validate_datetime(value: &str) -> Result<(), Error> {
    let bytes = value.as_bytes();
    if bytes.len() < 20
        || bytes.len() > MAX_DID_RESOLUTION_DATETIME_BYTES
        || !bytes.is_ascii()
        || bytes.last() != Some(&b'Z')
    {
        return Err(invalid(ResolutionError::InvalidDateTime));
    }

    let year_start = usize::from(bytes.first() == Some(&b'-'));
    let Some(year_end) = bytes[year_start..]
        .iter()
        .position(|byte| *byte == b'-')
        .map(|offset| year_start + offset)
    else {
        return Err(invalid(ResolutionError::InvalidDateTime));
    };
    let year = &bytes[year_start..year_end];
    if year.len() < 4
        || !year.iter().all(u8::is_ascii_digit)
        || (year.len() > 4 && year.first() == Some(&b'0'))
    {
        return Err(invalid(ResolutionError::InvalidDateTime));
    }

    let date_time_tail = &bytes[year_end..];
    if date_time_tail.len() != 16
        || date_time_tail[0] != b'-'
        || date_time_tail[3] != b'-'
        || date_time_tail[6] != b'T'
        || date_time_tail[9] != b':'
        || date_time_tail[12] != b':'
        || date_time_tail[15] != b'Z'
        || date_time_tail
            .iter()
            .enumerate()
            .filter(|(index, _)| ![0, 3, 6, 9, 12, 15].contains(index))
            .any(|(_, byte)| !byte.is_ascii_digit())
    {
        return Err(invalid(ResolutionError::InvalidDateTime));
    }

    let number = |start: usize, end: usize| -> u32 {
        date_time_tail[start..end]
            .iter()
            .fold(0, |value, byte| value * 10 + u32::from(byte - b'0'))
    };
    let month = number(1, 3);
    let day = number(4, 6);
    let hour = number(7, 9);
    let minute = number(10, 12);
    let second = number(13, 15);
    let year_mod_400 = year.iter().fold(0_u16, |value, byte| {
        (value * 10 + u16::from(byte - b'0')) % 400
    });
    let leap = year_mod_400 % 4 == 0 && (year_mod_400 % 100 != 0 || year_mod_400 == 0);
    let max_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => 0,
    };
    let valid_time =
        hour <= 23 && minute <= 59 && second <= 59 || hour == 24 && minute == 0 && second == 0;
    if day == 0 || day > max_day || !valid_time {
        return Err(invalid(ResolutionError::InvalidDateTime));
    }
    Ok(())
}

fn validate_resolution_wire(input: &[u8]) -> Result<(), Error> {
    validate_unique_object_names(
        input,
        JsonWireLimits {
            max_depth: MAX_DID_RESOLUTION_WIRE_DEPTH,
            max_nodes: MAX_DID_RESOLUTION_WIRE_NODES,
            max_object_members: MAX_DID_RESOLUTION_WIRE_OBJECT_MEMBERS,
            max_live_key_bytes: MAX_DID_RESOLUTION_WIRE_LIVE_KEY_BYTES,
        },
    )
    .map_err(|reason| invalid(map_wire_error(reason)))
}

const fn map_wire_error(reason: JsonWireError) -> ResolutionError {
    match reason {
        JsonWireError::DuplicateName => ResolutionError::DuplicateJsonProperty,
        JsonWireError::TooDeep => ResolutionError::WireTooDeep,
        JsonWireError::TooManyNodes | JsonWireError::TooManyLiveKeyBytes => {
            ResolutionError::WireTooLarge
        }
        JsonWireError::TooManyMembers => ResolutionError::WireTooManyProperties,
        JsonWireError::Malformed => ResolutionError::MalformedJson,
    }
}

fn validate_media_type(value: &str) -> Result<(), Error> {
    let bytes = value.as_bytes();
    if bytes.is_empty() || bytes.len() > MAX_MEDIA_TYPE_BYTES || !bytes.is_ascii() {
        return Err(invalid(ResolutionError::InvalidMediaType));
    }

    let mut index = consume_token(bytes, 0)?;
    if bytes.get(index) != Some(&b'/') {
        return Err(invalid(ResolutionError::InvalidMediaType));
    }
    index = consume_token(bytes, index + 1)?;
    while index < bytes.len() {
        index = consume_ows(bytes, index);
        if bytes.get(index) != Some(&b';') {
            return Err(invalid(ResolutionError::InvalidMediaType));
        }
        index = consume_ows(bytes, index + 1);
        index = consume_token(bytes, index)?;
        index = consume_ows(bytes, index);
        if bytes.get(index) != Some(&b'=') {
            return Err(invalid(ResolutionError::InvalidMediaType));
        }
        index = consume_ows(bytes, index + 1);
        if bytes.get(index) == Some(&b'"') {
            index = consume_quoted(bytes, index + 1)?;
        } else {
            index = consume_token(bytes, index)?;
        }
    }
    Ok(())
}

fn consume_token(bytes: &[u8], start: usize) -> Result<usize, Error> {
    let mut index = start;
    while bytes.get(index).is_some_and(|byte| is_token(*byte)) {
        index += 1;
    }
    if index == start {
        return Err(invalid(ResolutionError::InvalidMediaType));
    }
    Ok(index)
}

fn consume_ows(bytes: &[u8], mut index: usize) -> usize {
    while bytes
        .get(index)
        .is_some_and(|byte| matches!(byte, b' ' | b'\t'))
    {
        index += 1;
    }
    index
}

fn consume_quoted(bytes: &[u8], mut index: usize) -> Result<usize, Error> {
    while let Some(byte) = bytes.get(index) {
        match byte {
            b'"' => return Ok(index + 1),
            b'\\' => {
                index += 1;
                if !bytes
                    .get(index)
                    .is_some_and(|escaped| matches!(escaped, b'\t' | 0x20..=0x7e))
                {
                    return Err(invalid(ResolutionError::InvalidMediaType));
                }
            }
            b'\t' | 0x20..=0x21 | 0x23..=0x5b | 0x5d..=0x7e => {}
            _ => return Err(invalid(ResolutionError::InvalidMediaType)),
        }
        index += 1;
    }
    Err(invalid(ResolutionError::InvalidMediaType))
}

const fn is_token(byte: u8) -> bool {
    byte.is_ascii_alphanumeric()
        || matches!(
            byte,
            b'!' | b'#'
                | b'$'
                | b'%'
                | b'&'
                | b'\''
                | b'*'
                | b'+'
                | b'-'
                | b'.'
                | b'^'
                | b'_'
                | b'`'
                | b'|'
                | b'~'
        )
}

const fn invalid(reason: ResolutionError) -> Error {
    Error::InvalidResolution(reason)
}
