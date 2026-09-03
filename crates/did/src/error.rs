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

/// A DID-domain validation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// A DID method name was empty or contained characters outside the
    /// `method-name = 1*( lowercase-alpha / digit )` grammar.
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
        }
    }
}

impl std::error::Error for Error {}
