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

/// A DID-domain validation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// A DID method name was empty or contained characters outside the
    /// `method-name = 1*( lowercase-alpha / digit )` grammar.
    InvalidMethod(String),
    /// A DID version did not parse to a supported, non-zero value.
    InvalidVersion(String),
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
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidMethod(detail) => write!(f, "invalid DID method: {detail}"),
            Error::InvalidVersion(detail) => write!(f, "invalid DID version: {detail}"),
        }
    }
}

impl std::error::Error for Error {}
