//! Bounded inputs and runtime-neutral ports for DID resolution queries.
//!
//! The ports mirror the W3C abstract functions. Method dispatch, transport,
//! caching, registration and trust policy remain outside this module.

use std::{collections::BTreeMap, future::Future, pin::Pin};

use identus_derive as identus;
use identus_derive::Newtype;
use serde::{Deserialize, Deserializer, Serialize, de};
use serde_json::Value;

use crate::{
    Did, DidResolutionDateTime, DidResolutionResult, DidUrl, DidUrlDereferencingResult, Error,
    MediaType, VersionId,
    document::{JsonBudget, validate_json_map},
    error::ResolutionError,
};

/// Maximum raw JSON size accepted by a resolution or dereferencing option map.
pub const MAX_DID_RESOLUTION_OPTIONS_BYTES: usize = 64 * 1_024;
/// Maximum byte length of a verification-relationship option value.
pub const MAX_VERIFICATION_RELATIONSHIP_BYTES: usize = 256;

const RESOLUTION_RESERVED: &[&str] = &[
    "accept",
    "expandRelativeUrls",
    "noCache",
    "versionId",
    "versionTime",
];
const DEREFERENCING_RESERVED: &[&str] = &["accept", "verificationRelationship"];

/// A bounded open verification-relationship name used during dereferencing.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Newtype)]
#[newtype(
    display,
    serde,
    validate_fn = validate_verification_relationship,
    validate_err = Error
)]
pub struct VerificationRelationshipName(String);

/// Immutable W3C DID resolution input options.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct ResolutionOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    accept: Option<MediaType>,
    #[serde(rename = "expandRelativeUrls", skip_serializing_if = "Option::is_none")]
    expand_relative_urls: Option<bool>,
    #[serde(rename = "noCache", skip_serializing_if = "Option::is_none")]
    no_cache: Option<bool>,
    #[serde(rename = "versionId", skip_serializing_if = "Option::is_none")]
    version_id: Option<VersionId>,
    #[serde(rename = "versionTime", skip_serializing_if = "Option::is_none")]
    version_time: Option<DidResolutionDateTime>,
    #[serde(flatten)]
    extensions: BTreeMap<String, Value>,
}

impl ResolutionOptions {
    /// Construct and validate a resolution option map.
    pub fn new(
        accept: Option<MediaType>,
        expand_relative_urls: Option<bool>,
        no_cache: Option<bool>,
        version_id: Option<VersionId>,
        version_time: Option<DidResolutionDateTime>,
        extensions: BTreeMap<String, Value>,
    ) -> Result<Self, Error> {
        let options = Self {
            accept,
            expand_relative_urls,
            no_cache,
            version_id,
            version_time,
            extensions,
        };
        options.validate()?;
        Ok(options)
    }

    /// Begin immutable option construction.
    #[must_use]
    pub fn builder() -> ResolutionOptionsBuilder {
        ResolutionOptionsBuilder::default()
    }

    /// Return an empty option map.
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    /// Parse a raw JSON option map after enforcing the byte ceiling.
    pub fn from_json_slice(input: &[u8]) -> Result<Self, Error> {
        parse_options(input)
    }

    /// Parse a raw JSON option map string after enforcing the byte ceiling.
    pub fn from_json_str(input: &str) -> Result<Self, Error> {
        Self::from_json_slice(input.as_bytes())
    }

    /// Borrow the preferred DID document representation.
    #[must_use]
    pub const fn accept(&self) -> Option<&MediaType> {
        self.accept.as_ref()
    }

    /// Return the explicit relative-URL expansion choice, when supplied.
    #[must_use]
    pub const fn expand_relative_urls(&self) -> Option<bool> {
        self.expand_relative_urls
    }

    /// Return the explicit W3C generic-cache bypass choice, when supplied.
    #[must_use]
    pub const fn no_cache(&self) -> Option<bool> {
        self.no_cache
    }

    /// Borrow the requested document version identifier.
    #[must_use]
    pub const fn version_id(&self) -> Option<&VersionId> {
        self.version_id.as_ref()
    }

    /// Borrow the requested document version time.
    #[must_use]
    pub const fn version_time(&self) -> Option<&DidResolutionDateTime> {
        self.version_time.as_ref()
    }

    /// Borrow registered or method-defined extension options.
    #[must_use]
    pub const fn extensions(&self) -> &BTreeMap<String, Value> {
        &self.extensions
    }

    fn validate(&self) -> Result<(), Error> {
        validate_option_extensions(&self.extensions, RESOLUTION_RESERVED)
    }
}

impl<'de> Deserialize<'de> for ResolutionOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            accept: Option<MediaType>,
            #[serde(rename = "expandRelativeUrls")]
            expand_relative_urls: Option<bool>,
            #[serde(rename = "noCache")]
            no_cache: Option<bool>,
            #[serde(rename = "versionId")]
            version_id: Option<VersionId>,
            #[serde(rename = "versionTime")]
            version_time: Option<DidResolutionDateTime>,
            #[serde(flatten)]
            extensions: BTreeMap<String, Value>,
        }

        let wire = Wire::deserialize(deserializer)?;
        Self::new(
            wire.accept,
            wire.expand_relative_urls,
            wire.no_cache,
            wire.version_id,
            wire.version_time,
            wire.extensions,
        )
        .map_err(de::Error::custom)
    }
}

/// Builder for immutable [`ResolutionOptions`].
#[derive(Clone, Debug, Default)]
pub struct ResolutionOptionsBuilder(ResolutionOptions);

impl ResolutionOptionsBuilder {
    /// Set the preferred DID document representation.
    #[must_use]
    pub fn accept(mut self, value: MediaType) -> Self {
        self.0.accept = Some(value);
        self
    }

    /// Set whether relative DID URLs should be expanded.
    #[must_use]
    pub fn expand_relative_urls(mut self, value: bool) -> Self {
        self.0.expand_relative_urls = Some(value);
        self
    }

    /// Request or explicitly permit use of a generic DID document cache.
    #[must_use]
    pub fn no_cache(mut self, value: bool) -> Self {
        self.0.no_cache = Some(value);
        self
    }

    /// Select a method-defined document version.
    #[must_use]
    pub fn version_id(mut self, value: VersionId) -> Self {
        self.0.version_id = Some(value);
        self
    }

    /// Select the most recent document valid before a UTC time.
    #[must_use]
    pub fn version_time(mut self, value: DidResolutionDateTime) -> Self {
        self.0.version_time = Some(value);
        self
    }

    /// Set registered or method-defined extension options.
    #[must_use]
    pub fn extensions(mut self, values: BTreeMap<String, Value>) -> Self {
        self.0.extensions = values;
        self
    }

    /// Validate and return the immutable option map.
    pub fn build(self) -> Result<ResolutionOptions, Error> {
        self.0.validate()?;
        Ok(self.0)
    }
}

/// Immutable W3C DID URL dereferencing input options.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct DereferencingOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    accept: Option<MediaType>,
    #[serde(
        rename = "verificationRelationship",
        skip_serializing_if = "Option::is_none"
    )]
    verification_relationship: Option<VerificationRelationshipName>,
    #[serde(flatten)]
    extensions: BTreeMap<String, Value>,
}

impl DereferencingOptions {
    /// Construct and validate a dereferencing option map.
    pub fn new(
        accept: Option<MediaType>,
        verification_relationship: Option<VerificationRelationshipName>,
        extensions: BTreeMap<String, Value>,
    ) -> Result<Self, Error> {
        let options = Self {
            accept,
            verification_relationship,
            extensions,
        };
        options.validate()?;
        Ok(options)
    }

    /// Begin immutable option construction.
    #[must_use]
    pub fn builder() -> DereferencingOptionsBuilder {
        DereferencingOptionsBuilder::default()
    }

    /// Return an empty option map.
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    /// Parse a raw JSON option map after enforcing the byte ceiling.
    pub fn from_json_slice(input: &[u8]) -> Result<Self, Error> {
        parse_options(input)
    }

    /// Parse a raw JSON option map string after enforcing the byte ceiling.
    pub fn from_json_str(input: &str) -> Result<Self, Error> {
        Self::from_json_slice(input.as_bytes())
    }

    /// Borrow the preferred resource representation.
    #[must_use]
    pub const fn accept(&self) -> Option<&MediaType> {
        self.accept.as_ref()
    }

    /// Borrow the relationship that must authorize a dereferenced method.
    #[must_use]
    pub const fn verification_relationship(&self) -> Option<&VerificationRelationshipName> {
        self.verification_relationship.as_ref()
    }

    /// Borrow registered or method-defined extension options.
    #[must_use]
    pub const fn extensions(&self) -> &BTreeMap<String, Value> {
        &self.extensions
    }

    fn validate(&self) -> Result<(), Error> {
        validate_option_extensions(&self.extensions, DEREFERENCING_RESERVED)
    }
}

impl<'de> Deserialize<'de> for DereferencingOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            accept: Option<MediaType>,
            #[serde(rename = "verificationRelationship")]
            verification_relationship: Option<VerificationRelationshipName>,
            #[serde(flatten)]
            extensions: BTreeMap<String, Value>,
        }

        let wire = Wire::deserialize(deserializer)?;
        Self::new(wire.accept, wire.verification_relationship, wire.extensions)
            .map_err(de::Error::custom)
    }
}

/// Builder for immutable [`DereferencingOptions`].
#[derive(Clone, Debug, Default)]
pub struct DereferencingOptionsBuilder(DereferencingOptions);

impl DereferencingOptionsBuilder {
    /// Set the preferred resource representation.
    #[must_use]
    pub fn accept(mut self, value: MediaType) -> Self {
        self.0.accept = Some(value);
        self
    }

    /// Require authorization by one verification relationship.
    #[must_use]
    pub fn verification_relationship(mut self, value: VerificationRelationshipName) -> Self {
        self.0.verification_relationship = Some(value);
        self
    }

    /// Set registered or method-defined extension options.
    #[must_use]
    pub fn extensions(mut self, values: BTreeMap<String, Value>) -> Self {
        self.0.extensions = values;
        self
    }

    /// Validate and return the immutable option map.
    pub fn build(self) -> Result<DereferencingOptions, Error> {
        self.0.validate()?;
        Ok(self.0)
    }
}

/// Type-erased future returned by [`DidResolver`].
pub type DidResolutionFuture<'a> = Pin<Box<dyn Future<Output = DidResolutionResult> + Send + 'a>>;

/// Runtime-neutral port for the W3C DID resolution abstract function.
#[identus::port]
pub trait DidResolver: Send + Sync {
    /// Resolve a validated DID with a required, possibly empty option map.
    fn resolve<'a>(
        &'a self,
        did: &'a Did,
        options: &'a ResolutionOptions,
    ) -> DidResolutionFuture<'a>;
}

/// Type-erased future returned by [`DidUrlDereferencer`].
pub type DidUrlDereferencingFuture<'a> =
    Pin<Box<dyn Future<Output = DidUrlDereferencingResult> + Send + 'a>>;

/// Runtime-neutral port for the W3C DID URL dereferencing abstract function.
#[identus::port]
pub trait DidUrlDereferencer: Send + Sync {
    /// Dereference a validated DID URL with a required, possibly empty option map.
    fn dereference<'a>(
        &'a self,
        did_url: &'a DidUrl,
        options: &'a DereferencingOptions,
    ) -> DidUrlDereferencingFuture<'a>;
}

fn validate_verification_relationship(value: &str) -> Result<(), Error> {
    if value.is_empty()
        || value.len() > MAX_VERIFICATION_RELATIONSHIP_BYTES
        || !value.bytes().all(|byte| (0x20..=0x7e).contains(&byte))
        || value.trim() != value
    {
        return Err(invalid(ResolutionError::InvalidString));
    }
    Ok(())
}

fn validate_option_extensions(
    map: &BTreeMap<String, Value>,
    reserved: &[&str],
) -> Result<(), Error> {
    validate_json_map(map, reserved, &mut JsonBudget::default())
        .map_err(|_| invalid(ResolutionError::InvalidString))
}

fn parse_options<T>(input: &[u8]) -> Result<T, Error>
where
    T: for<'de> Deserialize<'de>,
{
    if input.len() > MAX_DID_RESOLUTION_OPTIONS_BYTES {
        return Err(invalid(ResolutionError::TooLarge));
    }
    serde_json::from_slice(input).map_err(|_| invalid(ResolutionError::MalformedJson))
}

const fn invalid(reason: ResolutionError) -> Error {
    Error::InvalidResolution(reason)
}
