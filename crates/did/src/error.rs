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
        }
    }
}

impl std::error::Error for Error {}
