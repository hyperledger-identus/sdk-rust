//! Rich local error type for the `identus-did` crate.
//!
//! This is the rich local error surface (the `FromStr::Err` for the validated
//! domain newtypes in this crate). [`Error::to_identus_error`] bridges to the
//! redaction-safe [`IdentusError`] contract with stable [`ErrorCode`]s and
//! `CapabilityId("did")`; `IdentusError::Display` carries no runtime detail, so
//! the structured messages held here never leak through it.

use std::fmt;

use identus_core::{CapabilityId, ErrorCode, ErrorKind, IdentusError};

const CAPABILITY: CapabilityId = CapabilityId::new("did");
const INVALID_METHOD_CODE: ErrorCode = ErrorCode::new("did.invalid_method");
const INVALID_VERSION_CODE: ErrorCode = ErrorCode::new("did.invalid_version");
const INVALID_DID_CODE: ErrorCode = ErrorCode::new("did.invalid_did");
const INVALID_DID_URL_CODE: ErrorCode = ErrorCode::new("did.invalid_did_url");
const INVALID_URI_CODE: ErrorCode = ErrorCode::new("did.invalid_uri");
const INVALID_DOCUMENT_CODE: ErrorCode = ErrorCode::new("did.invalid_document");
const INVALID_RESOLUTION_CODE: ErrorCode = ErrorCode::new("did.invalid_resolution");
const INVALID_METHOD_REGISTRY_CODE: ErrorCode = ErrorCode::new("did.invalid_method_registry");
const INVALID_RESOLUTION_CACHE_CODE: ErrorCode = ErrorCode::new("did.invalid_resolution_cache");
const INVALID_REGISTRATION_CODE: ErrorCode = ErrorCode::new("did.invalid_registration");

/// A non-sensitive reason that a DID or DID URL failed lexical validation.
///
/// Reasons intentionally contain no caller-controlled text, so both local
/// diagnostics and the public [`IdentusError`] bridge are safe to display.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DidSyntaxError {
    /// The value exceeds the SDK's public byte limit.
    TooLong,
    /// The value does not begin with the exact lowercase `did:` prefix.
    MissingPrefix,
    /// The method name is empty.
    MissingMethod,
    /// The method name contains a byte outside `[a-z0-9]`.
    InvalidMethodCharacter,
    /// The method-specific identifier is empty.
    MissingMethodSpecificId,
    /// The method-specific identifier contains a byte outside DID Core.
    InvalidMethodSpecificIdCharacter,
    /// A percent escape is truncated or contains a non-hex digit.
    InvalidPercentEncoding,
    /// The method-specific identifier ends with `:`.
    TrailingColon,
    /// The DID URL path contains a byte outside RFC 3986 `path-abempty`.
    InvalidPath,
    /// The DID URL query contains a byte outside RFC 3986 `query`.
    InvalidQuery,
    /// The DID URL fragment contains a byte outside RFC 3986 `fragment`.
    InvalidFragment,
}

impl fmt::Display for DidSyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::TooLong => "value exceeds the SDK byte limit",
            Self::MissingPrefix => "lowercase did prefix is missing",
            Self::MissingMethod => "method is missing",
            Self::InvalidMethodCharacter => "method contains an invalid character",
            Self::MissingMethodSpecificId => "method-specific identifier is missing",
            Self::InvalidMethodSpecificIdCharacter => {
                "method-specific identifier contains an invalid character"
            }
            Self::InvalidPercentEncoding => "percent encoding is invalid",
            Self::TrailingColon => "method-specific identifier ends with a colon",
            Self::InvalidPath => "path contains an invalid character",
            Self::InvalidQuery => "query contains an invalid character",
            Self::InvalidFragment => "fragment contains an invalid character",
        };
        f.write_str(message)
    }
}

/// A non-sensitive reason that an absolute URI failed lexical validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum UriSyntaxError {
    /// The value exceeds the SDK's public byte limit.
    TooLong,
    /// The URI has no absolute scheme separator.
    MissingScheme,
    /// The scheme is outside the RFC 3986 scheme grammar.
    InvalidScheme,
    /// A component contains a byte outside its RFC 3986 grammar.
    InvalidCharacter,
    /// A percent escape is truncated or contains a non-hex digit.
    InvalidPercentEncoding,
    /// The authority component is structurally invalid.
    InvalidAuthority,
    /// More than one fragment delimiter was supplied.
    MultipleFragments,
}

impl fmt::Display for UriSyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::TooLong => "value exceeds the SDK byte limit",
            Self::MissingScheme => "absolute URI scheme is missing",
            Self::InvalidScheme => "URI scheme contains an invalid character",
            Self::InvalidCharacter => "URI component contains an invalid character",
            Self::InvalidPercentEncoding => "percent encoding is invalid",
            Self::InvalidAuthority => "URI authority is invalid",
            Self::MultipleFragments => "URI contains multiple fragment delimiters",
        };
        f.write_str(message)
    }
}

/// A non-sensitive DID document structural validation failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DocumentError {
    /// Raw input exceeds the SDK document byte limit.
    TooLarge,
    /// Input is not a valid DID document JSON representation.
    MalformedJson,
    /// Raw JSON repeats a decoded object member name.
    DuplicateJsonProperty,
    /// A present set or required string is empty.
    EmptyValue,
    /// A document collection exceeds its item limit.
    TooManyItems,
    /// A bounded string exceeds its byte limit or contains forbidden bytes.
    InvalidString,
    /// An extension map has too many entries.
    TooManyProperties,
    /// An extension property name is empty or too long.
    InvalidPropertyName,
    /// An arbitrary JSON tree exceeds its depth limit.
    ExtensionTooDeep,
    /// An arbitrary JSON tree exceeds its total node limit.
    ExtensionTooLarge,
    /// An extension shadows a reserved core property.
    ReservedProperty,
    /// A verification method identifier is duplicated.
    DuplicateVerificationMethod,
    /// A relationship, controller, alias, type or endpoint set has a duplicate.
    DuplicateSetMember,
    /// A service identifier is duplicated.
    DuplicateService,
    /// A public JWK contains registered private key material.
    PrivateKeyMaterial,
    /// More than one recognized verification material representation is present.
    MultipleVerificationMaterial,
    /// A recognized verification or service value has the wrong JSON shape.
    InvalidPropertyShape,
}

impl fmt::Display for DocumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::TooLarge => "DID document exceeds the SDK byte limit",
            Self::MalformedJson => "DID document JSON is malformed",
            Self::DuplicateJsonProperty => "DID document JSON contains a duplicate property",
            Self::EmptyValue => "a required DID document value is empty",
            Self::TooManyItems => "a DID document collection exceeds its item limit",
            Self::InvalidString => "a DID document string violates its resource policy",
            Self::TooManyProperties => "an extension map has too many properties",
            Self::InvalidPropertyName => "an extension property name is invalid",
            Self::ExtensionTooDeep => "an extension tree exceeds its depth limit",
            Self::ExtensionTooLarge => "an extension tree exceeds its node limit",
            Self::ReservedProperty => "an extension shadows a reserved property",
            Self::DuplicateVerificationMethod => "a verification method identifier is duplicated",
            Self::DuplicateSetMember => "a set contains a duplicate member",
            Self::DuplicateService => "a service identifier is duplicated",
            Self::PrivateKeyMaterial => "a public JWK contains private key material",
            Self::MultipleVerificationMaterial => {
                "a verification method has multiple recognized material representations"
            }
            Self::InvalidPropertyShape => "a DID document property has an invalid shape",
        };
        f.write_str(message)
    }
}

/// A non-sensitive DID resolution result validation failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ResolutionError {
    /// Raw input exceeds the SDK result byte limit.
    TooLarge,
    /// Input is not a valid JSON result representation.
    MalformedJson,
    /// Raw result JSON contains a duplicate decoded property name.
    DuplicateJsonProperty,
    /// Raw result JSON exceeds the preflight nesting limit.
    WireTooDeep,
    /// Raw result JSON exceeds the preflight node or live-name byte limit.
    WireTooLarge,
    /// One raw result object exceeds the preflight member limit.
    WireTooManyProperties,
    /// A required value is empty.
    EmptyValue,
    /// A resolution collection exceeds its item limit.
    TooManyItems,
    /// A bounded string violates its lexical or size policy.
    InvalidString,
    /// A media type is malformed.
    InvalidMediaType,
    /// A resolution datetime is malformed or outside calendar bounds.
    InvalidDateTime,
    /// A result combines incompatible success, failure or deactivation fields.
    InvalidState,
    /// A returned document does not match the requested DID.
    DocumentIdMismatch,
    /// Canonical or equivalent metadata uses another DID method.
    DifferentDidMethod,
    /// Equivalent DID metadata contains a duplicate.
    DuplicateEquivalentId,
    /// Dereferenced JSON content is null or cannot be projected as requested.
    InvalidContent,
}

impl fmt::Display for ResolutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::TooLarge => "DID resolution result exceeds the SDK byte limit",
            Self::MalformedJson => "DID resolution result JSON is malformed",
            Self::DuplicateJsonProperty => {
                "DID resolution result JSON contains a duplicate property"
            }
            Self::WireTooDeep => "DID resolution result JSON exceeds the nesting limit",
            Self::WireTooLarge => "DID resolution result JSON exceeds the structural limit",
            Self::WireTooManyProperties => {
                "a DID resolution result JSON object has too many properties"
            }
            Self::EmptyValue => "a required DID resolution value is empty",
            Self::TooManyItems => "a DID resolution collection exceeds its item limit",
            Self::InvalidString => "a DID resolution string violates its resource policy",
            Self::InvalidMediaType => "DID resolution media type is invalid",
            Self::InvalidDateTime => "DID resolution datetime is invalid",
            Self::InvalidState => "DID resolution result state is contradictory",
            Self::DocumentIdMismatch => "resolved DID document id does not match the request",
            Self::DifferentDidMethod => "DID metadata uses a different method",
            Self::DuplicateEquivalentId => "DID metadata contains a duplicate equivalent id",
            Self::InvalidContent => "dereferenced content has an invalid shape",
        };
        f.write_str(message)
    }
}

/// A non-sensitive DID method registry construction failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum RegistryError {
    /// More than one binding claimed the same exact DID method.
    DuplicateMethod,
    /// The registry exceeded the SDK's method-entry limit.
    TooManyMethods,
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::DuplicateMethod => "a DID method already has a registered binding",
            Self::TooManyMethods => "the DID method registry exceeds its entry limit",
        };
        f.write_str(message)
    }
}

/// A non-sensitive DID resolution cache construction failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum CacheError {
    /// A configured TTL is zero or exceeds its class ceiling.
    InvalidTtl,
    /// A cache backend declares no capacity or excessive capacity.
    InvalidCapacity,
    /// A normalized resolution request exceeds the cache-key byte ceiling.
    KeyTooLarge,
    /// An entry expiry is not strictly after its insertion tick.
    InvalidEntryLifetime,
}

impl fmt::Display for CacheError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidTtl => "a DID resolution cache TTL is invalid",
            Self::InvalidCapacity => "a DID resolution cache capacity is invalid",
            Self::KeyTooLarge => "a DID resolution cache key exceeds its byte limit",
            Self::InvalidEntryLifetime => "a DID resolution cache entry lifetime is invalid",
        })
    }
}

/// A non-sensitive DID Registration construction or state failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum RegistrationError {
    /// Raw public JSON exceeds its byte ceiling.
    TooLarge,
    /// Raw public JSON is malformed or is not an object.
    MalformedJson,
    /// A required collection or string is empty.
    EmptyValue,
    /// A registration collection exceeds its item limit.
    TooManyItems,
    /// A bounded string is invalid.
    InvalidString,
    /// A public-data object has too many properties.
    TooManyProperties,
    /// A public-data property name is invalid.
    InvalidPropertyName,
    /// A public-data property shadows a registration envelope member.
    ReservedProperty,
    /// A public-data tree exceeds its depth limit.
    TooDeep,
    /// A public-data tree exceeds its node limit.
    TooManyNodes,
    /// Public data contains a private-material-shaped member.
    PrivateMaterial,
    /// Internal generation would neither store material nor return a handle.
    InvalidSecretPolicy,
    /// A terminal or non-terminal state contradicts its job.
    InvalidState,
    /// A DID or method differs from its enclosing operation.
    MethodOrDidMismatch,
    /// An action response differs from the outstanding action.
    ActionMismatch,
    /// A returned job differs from the requested method-scoped job.
    JobMismatch,
}

impl fmt::Display for RegistrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::TooLarge => "DID Registration public data exceeds its byte limit",
            Self::MalformedJson => "DID Registration public data JSON is malformed",
            Self::EmptyValue => "a required DID Registration value is empty",
            Self::TooManyItems => "a DID Registration collection exceeds its item limit",
            Self::InvalidString => "a DID Registration string violates its resource policy",
            Self::TooManyProperties => "DID Registration public data has too many properties",
            Self::InvalidPropertyName => "a DID Registration property name is invalid",
            Self::ReservedProperty => "DID Registration public data shadows a reserved property",
            Self::TooDeep => "DID Registration public data exceeds its depth limit",
            Self::TooManyNodes => "DID Registration public data exceeds its node limit",
            Self::PrivateMaterial => "DID Registration public data resembles private material",
            Self::InvalidSecretPolicy => "DID Registration secret policy loses capability",
            Self::InvalidState => "DID Registration state contradicts its job",
            Self::MethodOrDidMismatch => "DID Registration method or DID does not match",
            Self::ActionMismatch => "DID Registration action does not match",
            Self::JobMismatch => "DID Registration job does not match",
        })
    }
}

/// A DID-domain validation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// A DID method name was empty, oversized or contained characters outside
    /// the `method-name = 1*( lowercase-alpha / digit )` grammar.
    InvalidMethod(String),
    /// A DID version did not parse to a supported, non-zero value.
    InvalidVersion(String),
    /// An absolute DID failed lexical validation.
    InvalidDid(DidSyntaxError),
    /// An absolute DID URL failed lexical validation.
    InvalidDidUrl(DidSyntaxError),
    /// An absolute URI failed lexical validation.
    InvalidUri(UriSyntaxError),
    /// A DID document failed structural or resource validation.
    InvalidDocument(DocumentError),
    /// A DID resolution result failed structural or state validation.
    InvalidResolution(ResolutionError),
    /// A DID method registry failed bounded deterministic construction.
    InvalidRegistry(RegistryError),
    /// A DID resolution cache value or configuration is invalid.
    InvalidCache(CacheError),
    /// A DID Registration value or state is invalid.
    InvalidRegistration(RegistrationError),
}

impl Error {
    /// Bridge this local error to the redaction-safe [`IdentusError`] contract.
    ///
    /// The structured detail held by each variant is dropped: the resulting
    /// `IdentusError` carries only a stable [`ErrorCode`], [`CapabilityId`],
    /// and a `&'static str` public message, so `IdentusError::Display` leaks no
    /// runtime detail.
    pub fn to_identus_error(&self) -> IdentusError {
        match self {
            Error::InvalidMethod(_) => IdentusError::public(
                INVALID_METHOD_CODE,
                ErrorKind::InvalidInput,
                CAPABILITY,
                "invalid DID method",
            ),
            Error::InvalidVersion(_) => IdentusError::public(
                INVALID_VERSION_CODE,
                ErrorKind::InvalidInput,
                CAPABILITY,
                "invalid DID version",
            ),
            Error::InvalidDid(_) => IdentusError::public(
                INVALID_DID_CODE,
                ErrorKind::InvalidInput,
                CAPABILITY,
                "invalid DID",
            ),
            Error::InvalidDidUrl(_) => IdentusError::public(
                INVALID_DID_URL_CODE,
                ErrorKind::InvalidInput,
                CAPABILITY,
                "invalid DID URL",
            ),
            Error::InvalidUri(_) => IdentusError::public(
                INVALID_URI_CODE,
                ErrorKind::InvalidInput,
                CAPABILITY,
                "invalid URI",
            ),
            Error::InvalidDocument(_) => IdentusError::public(
                INVALID_DOCUMENT_CODE,
                ErrorKind::InvalidInput,
                CAPABILITY,
                "invalid DID document",
            ),
            Error::InvalidResolution(_) => IdentusError::public(
                INVALID_RESOLUTION_CODE,
                ErrorKind::InvalidInput,
                CAPABILITY,
                "invalid DID resolution result",
            ),
            Error::InvalidRegistry(reason) => IdentusError::public(
                INVALID_METHOD_REGISTRY_CODE,
                match reason {
                    RegistryError::DuplicateMethod => ErrorKind::Conflict,
                    RegistryError::TooManyMethods => ErrorKind::InvalidInput,
                },
                CAPABILITY,
                "invalid DID method registry",
            ),
            Error::InvalidCache(_) => IdentusError::public(
                INVALID_RESOLUTION_CACHE_CODE,
                ErrorKind::InvalidInput,
                CAPABILITY,
                "invalid DID resolution cache configuration",
            ),
            Error::InvalidRegistration(_) => IdentusError::public(
                INVALID_REGISTRATION_CODE,
                ErrorKind::InvalidInput,
                CAPABILITY,
                "invalid DID Registration value",
            ),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidMethod(detail) => write!(f, "invalid DID method: {detail}"),
            Error::InvalidVersion(detail) => write!(f, "invalid DID version: {detail}"),
            Error::InvalidDid(reason) => write!(f, "invalid DID: {reason}"),
            Error::InvalidDidUrl(reason) => write!(f, "invalid DID URL: {reason}"),
            Error::InvalidUri(reason) => write!(f, "invalid URI: {reason}"),
            Error::InvalidDocument(reason) => write!(f, "invalid DID document: {reason}"),
            Error::InvalidResolution(reason) => {
                write!(f, "invalid DID resolution result: {reason}")
            }
            Error::InvalidRegistry(reason) => write!(f, "invalid DID method registry: {reason}"),
            Error::InvalidCache(reason) => write!(f, "invalid DID resolution cache: {reason}"),
            Error::InvalidRegistration(reason) => {
                write!(f, "invalid DID Registration value: {reason}")
            }
        }
    }
}

impl std::error::Error for Error {}
