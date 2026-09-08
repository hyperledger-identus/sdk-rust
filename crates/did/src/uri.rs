//! Bounded absolute RFC 3986 URI value used by DID documents.

use std::{fmt, str::FromStr};

use fluent_uri::{ParseErrorKind, Uri as ParsedUri};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

use crate::{
    Did, DidUrl,
    error::{Error, UriSyntaxError},
};

/// Maximum accepted byte length of a [`Uri`].
///
/// RFC 3986 does not impose this value; it is an SDK resource policy.
pub const MAX_URI_BYTES: usize = 4_096;

/// An immutable, syntactically valid absolute RFC 3986 URI.
///
/// This type validates generic URI syntax only. It does not dereference,
/// normalize, authorize or apply scheme-specific policy to a URI.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Uri(String);

impl Uri {
    /// Parse and validate a borrowed URI, allocating only after validation.
    pub fn parse(value: &str) -> Result<Self, Error> {
        validate_uri(value.as_bytes()).map_err(Error::InvalidUri)?;
        Ok(Self(value.to_owned()))
    }

    /// Validate an owned URI while retaining its allocation.
    pub fn try_new(value: String) -> Result<Self, Error> {
        validate_uri(value.as_bytes()).map_err(Error::InvalidUri)?;
        Ok(Self(value))
    }

    /// Return the exact validated URI representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Return the URI scheme without its trailing colon.
    #[must_use]
    pub fn scheme(&self) -> &str {
        let end = self
            .0
            .find(':')
            .expect("validated absolute URI always contains a scheme");
        &self.0[..end]
    }

    /// Consume the value and return its exact representation.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }

    /// Parse this URI as the stronger DID URL profile.
    ///
    /// This allocates a standalone [`DidUrl`]; generic URI validity alone does
    /// not imply that the URI is DID-shaped.
    pub fn to_did_url(&self) -> Result<DidUrl, Error> {
        DidUrl::parse(self.as_str())
    }
}

impl From<Did> for Uri {
    fn from(value: Did) -> Self {
        Self(value.into_string())
    }
}

impl From<DidUrl> for Uri {
    fn from(value: DidUrl) -> Self {
        Self(value.into_string())
    }
}

impl AsRef<str> for Uri {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Uri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Uri {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl TryFrom<String> for Uri {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl Serialize for Uri {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Uri {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::try_new(value).map_err(de::Error::custom)
    }
}

fn validate_uri(bytes: &[u8]) -> Result<(), UriSyntaxError> {
    if bytes.len() > MAX_URI_BYTES {
        return Err(UriSyntaxError::TooLong);
    }

    let colon = bytes
        .iter()
        .position(|byte| *byte == b':')
        .ok_or(UriSyntaxError::MissingScheme)?;
    validate_scheme(&bytes[..colon])?;

    if bytes[colon + 1..]
        .iter()
        .filter(|byte| **byte == b'#')
        .count()
        > 1
    {
        return Err(UriSyntaxError::MultipleFragments);
    }

    ParsedUri::parse(std::str::from_utf8(bytes).expect("URI input originated as UTF-8"))
        .map(|_| ())
        .map_err(|error| match error.kind() {
            ParseErrorKind::InvalidPctEncodedOctet => UriSyntaxError::InvalidPercentEncoding,
            ParseErrorKind::InvalidIpv6Addr => UriSyntaxError::InvalidAuthority,
            ParseErrorKind::UnexpectedChar
                if index_is_in_authority(bytes, colon, error.index()) =>
            {
                UriSyntaxError::InvalidAuthority
            }
            ParseErrorKind::UnexpectedChar => UriSyntaxError::InvalidCharacter,
        })
}

fn validate_scheme(scheme: &[u8]) -> Result<(), UriSyntaxError> {
    let Some(first) = scheme.first() else {
        return Err(UriSyntaxError::MissingScheme);
    };
    if !first.is_ascii_alphabetic()
        || !scheme[1..]
            .iter()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'+' | b'-' | b'.'))
    {
        return Err(UriSyntaxError::InvalidScheme);
    }
    Ok(())
}

fn index_is_in_authority(bytes: &[u8], colon: usize, index: usize) -> bool {
    let Some(authority) = bytes[colon + 1..].strip_prefix(b"//") else {
        return false;
    };
    let start = colon + 3;
    let end = start
        + authority
            .iter()
            .position(|byte| matches!(byte, b'/' | b'?' | b'#'))
            .unwrap_or(authority.len());
    (start..end).contains(&index)
}
