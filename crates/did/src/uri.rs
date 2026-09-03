//! Bounded absolute RFC 3986 URI value used by DID documents.

use std::{fmt, net::Ipv6Addr, str::FromStr};

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

    let remainder = &bytes[colon + 1..];
    let fragment = remainder.iter().position(|byte| *byte == b'#');
    if let Some(fragment) = fragment {
        if remainder[fragment + 1..].contains(&b'#') {
            return Err(UriSyntaxError::MultipleFragments);
        }
    }

    let before_fragment = fragment.map_or(remainder, |index| &remainder[..index]);
    let query = before_fragment.iter().position(|byte| *byte == b'?');
    let hier_part = query.map_or(before_fragment, |index| &before_fragment[..index]);
    validate_hier_part(hier_part)?;

    if let Some(query) = query {
        validate_query_or_fragment(&before_fragment[query + 1..])?;
    }
    if let Some(fragment) = fragment {
        validate_query_or_fragment(&remainder[fragment + 1..])?;
    }
    Ok(())
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

fn validate_hier_part(value: &[u8]) -> Result<(), UriSyntaxError> {
    if let Some(after_prefix) = value.strip_prefix(b"//") {
        let path_start = after_prefix
            .iter()
            .position(|byte| *byte == b'/')
            .unwrap_or(after_prefix.len());
        validate_authority(&after_prefix[..path_start])?;
        validate_path(&after_prefix[path_start..])
    } else {
        validate_path(value)
    }
}

fn validate_authority(authority: &[u8]) -> Result<(), UriSyntaxError> {
    let mut at = None;
    let mut index = 0;
    while index < authority.len() {
        if authority[index] == b'%' {
            index = consume_percent(authority, index)?;
            continue;
        }
        if authority[index] == b'@' {
            if at.replace(index).is_some() {
                return Err(UriSyntaxError::InvalidAuthority);
            }
        } else if !is_authority_byte(authority[index]) {
            return Err(UriSyntaxError::InvalidAuthority);
        }
        index += 1;
    }

    let host_port = if let Some(at) = at {
        validate_userinfo(&authority[..at])?;
        &authority[at + 1..]
    } else {
        authority
    };
    if host_port.starts_with(b"[") {
        let Some(close) = host_port.iter().position(|byte| *byte == b']') else {
            return Err(UriSyntaxError::InvalidAuthority);
        };
        if close == 1 || !valid_ip_literal(&host_port[1..close]) {
            return Err(UriSyntaxError::InvalidAuthority);
        }
        let suffix = &host_port[close + 1..];
        if !suffix.is_empty() && (suffix[0] != b':' || !suffix[1..].iter().all(u8::is_ascii_digit))
        {
            return Err(UriSyntaxError::InvalidAuthority);
        }
        return Ok(());
    }

    let mut parts = host_port.split(|byte| *byte == b':');
    let host = parts.next().unwrap_or_default();
    let port = parts.next();
    if parts.next().is_some() || port.is_some_and(|value| !value.iter().all(u8::is_ascii_digit)) {
        return Err(UriSyntaxError::InvalidAuthority);
    }
    validate_reg_name(host)
}

fn valid_ip_literal(value: &[u8]) -> bool {
    if value
        .first()
        .is_some_and(|byte| matches!(byte, b'v' | b'V'))
    {
        let Some(dot) = value.iter().position(|byte| *byte == b'.') else {
            return false;
        };
        return dot > 1
            && dot + 1 < value.len()
            && value[1..dot].iter().all(u8::is_ascii_hexdigit)
            && value[dot + 1..]
                .iter()
                .all(|byte| is_unreserved(*byte) || is_sub_delim(*byte) || *byte == b':');
    }
    std::str::from_utf8(value)
        .ok()
        .and_then(|value| value.parse::<Ipv6Addr>().ok())
        .is_some()
}

fn validate_userinfo(value: &[u8]) -> Result<(), UriSyntaxError> {
    validate_component(value, |byte| {
        is_unreserved(byte) || is_sub_delim(byte) || byte == b':'
    })
    .map_err(|_| UriSyntaxError::InvalidAuthority)
}

fn validate_reg_name(value: &[u8]) -> Result<(), UriSyntaxError> {
    let mut index = 0;
    while index < value.len() {
        if value[index] == b'%' {
            index = consume_percent(value, index)?;
        } else if is_unreserved(value[index]) || is_sub_delim(value[index]) {
            index += 1;
        } else {
            return Err(UriSyntaxError::InvalidAuthority);
        }
    }
    Ok(())
}

fn validate_path(value: &[u8]) -> Result<(), UriSyntaxError> {
    validate_component(value, |byte| is_pchar(byte) || byte == b'/')
}

fn validate_query_or_fragment(value: &[u8]) -> Result<(), UriSyntaxError> {
    validate_component(value, |byte| is_pchar(byte) || matches!(byte, b'/' | b'?'))
}

fn validate_component(value: &[u8], allowed: impl Fn(u8) -> bool) -> Result<(), UriSyntaxError> {
    let mut index = 0;
    while index < value.len() {
        if value[index] == b'%' {
            index = consume_percent(value, index)?;
        } else if allowed(value[index]) {
            index += 1;
        } else {
            return Err(UriSyntaxError::InvalidCharacter);
        }
    }
    Ok(())
}

fn consume_percent(value: &[u8], index: usize) -> Result<usize, UriSyntaxError> {
    if index + 2 >= value.len()
        || !value[index + 1].is_ascii_hexdigit()
        || !value[index + 2].is_ascii_hexdigit()
    {
        return Err(UriSyntaxError::InvalidPercentEncoding);
    }
    Ok(index + 3)
}

const fn is_unreserved(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~')
}

const fn is_sub_delim(byte: u8) -> bool {
    matches!(
        byte,
        b'!' | b'$' | b'&' | b'\'' | b'(' | b')' | b'*' | b'+' | b',' | b';' | b'='
    )
}

const fn is_pchar(byte: u8) -> bool {
    is_unreserved(byte) || is_sub_delim(byte) || matches!(byte, b':' | b'@')
}

const fn is_authority_byte(byte: u8) -> bool {
    is_unreserved(byte) || is_sub_delim(byte) || matches!(byte, b':' | b'@' | b'[' | b']')
}
