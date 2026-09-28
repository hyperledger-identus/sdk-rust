use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer, Serialize, de};
use serde_json::Value;

use crate::{
    Error, Uri,
    document::{JsonBudget, validate_json_map},
    error::ResolutionError,
    json_cleanup::{RejectionGuard, drop_json_values_iteratively},
};

use super::value::MediaType;

/// Maximum byte length of a problem title or detail.
pub const MAX_PROBLEM_DETAIL_BYTES: usize = 4_096;

const MAX_PROBLEM_TITLE_BYTES: usize = 1_024;

const ERROR_RESERVED: &[&str] = &["type", "title", "detail", "instance"];
const OPERATION_METADATA_RESERVED: &[&str] = &["contentType", "error"];
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
        let error = RejectionGuard::new(
            Self {
                type_,
                title,
                detail,
                instance,
                extensions,
            },
            drop_did_resolution_error_json,
        );
        error.owner().validate()?;
        Ok(error.into_owner())
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
            .ok_or(Error::InvalidResolution(ResolutionError::InvalidString))
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
        validate_json_map(&self.extensions, ERROR_RESERVED, budget)
    }
}

fn drop_did_resolution_error_json(error: DidResolutionError) {
    let mut roots = Vec::new();
    collect_did_resolution_error_json(error, &mut roots);
    drop_json_values_iteratively(roots);
}

fn collect_did_resolution_error_json(error: DidResolutionError, roots: &mut Vec<Value>) {
    let DidResolutionError {
        type_: _,
        title: _,
        detail: _,
        instance: _,
        extensions,
    } = error;
    roots.extend(extensions.into_values());
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
        let metadata = RejectionGuard::new(
            Self {
                content_type,
                error,
                extensions,
            },
            drop_operation_metadata_json,
        );
        metadata.owner().validate()?;
        Ok(metadata.into_owner())
    }

    fn validate(&self) -> Result<(), Error> {
        self.validate_with(&mut JsonBudget::default())
    }

    fn validate_with(&self, budget: &mut JsonBudget) -> Result<(), Error> {
        if let Some(error) = &self.error {
            error.validate_with(budget)?;
        }
        validate_json_map(&self.extensions, OPERATION_METADATA_RESERVED, budget)
    }
}

fn drop_operation_metadata_json(metadata: OperationMetadata) {
    let OperationMetadata {
        content_type: _,
        error,
        extensions,
    } = metadata;
    let mut roots = Vec::new();

    if let Some(error) = error {
        collect_did_resolution_error_json(error, &mut roots);
    }
    roots.extend(extensions.into_values());

    drop_json_values_iteratively(roots);
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

            pub(super) fn validate_with(&self, budget: &mut JsonBudget) -> Result<(), Error> {
                self.0.validate_with(budget)
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
fn validate_problem_text(value: &str, max_bytes: usize) -> Result<(), Error> {
    if value.is_empty()
        || value.len() > max_bytes
        || value.chars().any(char::is_control)
        || value.trim() != value
    {
        return Err(Error::InvalidResolution(ResolutionError::InvalidString));
    }
    Ok(())
}
