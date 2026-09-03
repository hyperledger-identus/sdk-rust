//! Validated W3C DID and DID URL lexical value objects.
//!
//! This module implements only generic syntax. Method registration,
//! method-specific semantics, resolution and trust policy belong above it.

use std::{fmt, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

use crate::error::{DidSyntaxError, Error};

/// Maximum accepted byte length of a bare [`Did`].
///
/// W3C DID Core does not impose this value; it is an SDK resource policy.
pub const MAX_DID_BYTES: usize = 2_048;

/// Maximum accepted byte length of a [`DidUrl`].
///
/// W3C DID Core does not impose this value; it is an SDK resource policy.
pub const MAX_DID_URL_BYTES: usize = 4_096;

const DID_PREFIX: &[u8] = b"did:";

#[derive(Clone, Copy, Debug)]
struct DidParts {
    method_end: usize,
    did_end: usize,
}

#[derive(Clone, Copy, Debug)]
struct DidUrlParts {
    did: DidParts,
    query_start: Option<usize>,
    fragment_start: Option<usize>,
}

/// An immutable, syntactically valid absolute W3C DID.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Did {
    value: String,
    method_end: usize,
}

impl Did {
    /// Parse and validate a borrowed DID, allocating only after validation.
    pub fn parse(value: &str) -> Result<Self, Error> {
        let parts = parse_did(value.as_bytes(), MAX_DID_BYTES, false).map_err(Error::InvalidDid)?;
        Ok(Self {
            value: value.to_owned(),
            method_end: parts.method_end,
        })
    }

    /// Validate an owned DID while retaining its allocation.
    pub fn try_new(value: String) -> Result<Self, Error> {
        let parts = parse_did(value.as_bytes(), MAX_DID_BYTES, false).map_err(Error::InvalidDid)?;
        Ok(Self {
            value,
            method_end: parts.method_end,
        })
    }

    /// Return the exact validated DID representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Return the validated DID method without delimiters.
    #[must_use]
    pub fn method(&self) -> &str {
        &self.value[DID_PREFIX.len()..self.method_end]
    }

    /// Return the method-specific identifier without its leading colon.
    #[must_use]
    pub fn method_specific_id(&self) -> &str {
        &self.value[self.method_end + 1..]
    }

    /// Consume the value and return its exact string representation.
    #[must_use]
    pub fn into_string(self) -> String {
        self.value
    }
}

impl AsRef<str> for Did {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Did {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Did {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl TryFrom<String> for Did {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl Serialize for Did {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Did {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::try_new(value).map_err(de::Error::custom)
    }
}

/// An immutable, syntactically valid absolute W3C DID URL.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DidUrl {
    value: String,
    method_end: usize,
    did_end: usize,
    query_start: Option<usize>,
    fragment_start: Option<usize>,
}

impl DidUrl {
    /// Parse and validate a borrowed DID URL, allocating only after validation.
    pub fn parse(value: &str) -> Result<Self, Error> {
        let parts = parse_did_url(value.as_bytes()).map_err(Error::InvalidDidUrl)?;
        Ok(Self::from_parts(value.to_owned(), parts))
    }

    /// Validate an owned DID URL while retaining its allocation.
    pub fn try_new(value: String) -> Result<Self, Error> {
        let parts = parse_did_url(value.as_bytes()).map_err(Error::InvalidDidUrl)?;
        Ok(Self::from_parts(value, parts))
    }

    fn from_parts(value: String, parts: DidUrlParts) -> Self {
        Self {
            value,
            method_end: parts.did.method_end,
            did_end: parts.did.did_end,
            query_start: parts.query_start,
            fragment_start: parts.fragment_start,
        }
    }

    /// Return the exact validated DID URL representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Return the validated DID prefix as a borrowed string.
    #[must_use]
    pub fn as_did_str(&self) -> &str {
        &self.value[..self.did_end]
    }

    /// Allocate a standalone [`Did`] from the validated DID prefix.
    #[must_use]
    pub fn to_did(&self) -> Did {
        Did {
            value: self.as_did_str().to_owned(),
            method_end: self.method_end,
        }
    }

    /// Return the validated DID method without delimiters.
    #[must_use]
    pub fn method(&self) -> &str {
        &self.value[DID_PREFIX.len()..self.method_end]
    }

    /// Return the method-specific identifier without its leading colon.
    #[must_use]
    pub fn method_specific_id(&self) -> &str {
        &self.value[self.method_end + 1..self.did_end]
    }

    /// Return the path including its leading slash, or `""` when absent.
    #[must_use]
    pub fn path(&self) -> &str {
        let end = self
            .query_start
            .map(|start| start - 1)
            .or_else(|| self.fragment_start.map(|start| start - 1))
            .unwrap_or(self.value.len());
        &self.value[self.did_end..end]
    }

    /// Return the query without `?`, distinguishing absence from an empty query.
    #[must_use]
    pub fn query(&self) -> Option<&str> {
        self.query_start.map(|start| {
            let end = self
                .fragment_start
                .map(|fragment_start| fragment_start - 1)
                .unwrap_or(self.value.len());
            &self.value[start..end]
        })
    }

    /// Return the fragment without `#`, distinguishing absence from an empty fragment.
    #[must_use]
    pub fn fragment(&self) -> Option<&str> {
        self.fragment_start.map(|start| &self.value[start..])
    }

    /// Consume the value and return its exact string representation.
    #[must_use]
    pub fn into_string(self) -> String {
        self.value
    }
}

impl From<Did> for DidUrl {
    fn from(did: Did) -> Self {
        let did_end = did.value.len();
        Self {
            value: did.value,
            method_end: did.method_end,
            did_end,
            query_start: None,
            fragment_start: None,
        }
    }
}

impl AsRef<str> for DidUrl {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for DidUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for DidUrl {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl TryFrom<String> for DidUrl {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl Serialize for DidUrl {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for DidUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::try_new(value).map_err(de::Error::custom)
    }
}

fn parse_did(
    bytes: &[u8],
    max_len: usize,
    allow_url_suffix: bool,
) -> Result<DidParts, DidSyntaxError> {
    if bytes.len() > max_len {
        return Err(DidSyntaxError::TooLong);
    }
    if !bytes.starts_with(DID_PREFIX) {
        return Err(DidSyntaxError::MissingPrefix);
    }

    let mut cursor = DID_PREFIX.len();
    let method_start = cursor;
    while cursor < bytes.len() && bytes[cursor] != b':' {
        if !is_method_char(bytes[cursor]) {
            return Err(DidSyntaxError::InvalidMethodCharacter);
        }
        cursor += 1;
    }
    if cursor == method_start {
        return Err(DidSyntaxError::MissingMethod);
    }
    if cursor == bytes.len() {
        return Err(DidSyntaxError::MissingMethodSpecificId);
    }

    let method_end = cursor;
    cursor += 1;
    let msi_start = cursor;
    while cursor < bytes.len() {
        match bytes[cursor] {
            b'/' | b'?' | b'#' if allow_url_suffix => break,
            b'%' => consume_percent_escape(bytes, &mut cursor)?,
            byte if is_id_char(byte) => cursor += 1,
            _ => return Err(DidSyntaxError::InvalidMethodSpecificIdCharacter),
        }
    }
    if cursor == msi_start {
        return Err(DidSyntaxError::MissingMethodSpecificId);
    }
    if bytes[cursor - 1] == b':' {
        return Err(DidSyntaxError::TrailingColon);
    }

    Ok(DidParts {
        method_end,
        did_end: cursor,
    })
}

fn parse_did_url(bytes: &[u8]) -> Result<DidUrlParts, DidSyntaxError> {
    if bytes.len() > MAX_DID_URL_BYTES {
        return Err(DidSyntaxError::TooLong);
    }
    let did = parse_did(bytes, MAX_DID_URL_BYTES, true)?;
    if did.did_end > MAX_DID_BYTES {
        return Err(DidSyntaxError::TooLong);
    }
    let mut cursor = did.did_end;

    while cursor < bytes.len() && bytes[cursor] == b'/' {
        cursor += 1;
        while cursor < bytes.len() && !matches!(bytes[cursor], b'/' | b'?' | b'#') {
            if bytes[cursor] == b'%' {
                consume_percent_escape(bytes, &mut cursor)?;
            } else if is_pchar(bytes[cursor]) {
                cursor += 1;
            } else {
                return Err(DidSyntaxError::InvalidPath);
            }
        }
    }
    if cursor < bytes.len() && !matches!(bytes[cursor], b'?' | b'#') {
        return Err(DidSyntaxError::InvalidPath);
    }

    let query_start = if bytes.get(cursor) == Some(&b'?') {
        cursor += 1;
        let start = cursor;
        while cursor < bytes.len() && bytes[cursor] != b'#' {
            if bytes[cursor] == b'%' {
                consume_percent_escape(bytes, &mut cursor)?;
            } else if is_query_or_fragment_char(bytes[cursor]) {
                cursor += 1;
            } else {
                return Err(DidSyntaxError::InvalidQuery);
            }
        }
        Some(start)
    } else {
        None
    };

    let fragment_start = if bytes.get(cursor) == Some(&b'#') {
        cursor += 1;
        let start = cursor;
        while cursor < bytes.len() {
            if bytes[cursor] == b'%' {
                consume_percent_escape(bytes, &mut cursor)?;
            } else if is_query_or_fragment_char(bytes[cursor]) {
                cursor += 1;
            } else {
                return Err(DidSyntaxError::InvalidFragment);
            }
        }
        Some(start)
    } else {
        None
    };

    if cursor != bytes.len() {
        return Err(DidSyntaxError::InvalidFragment);
    }

    Ok(DidUrlParts {
        did,
        query_start,
        fragment_start,
    })
}

fn consume_percent_escape(bytes: &[u8], cursor: &mut usize) -> Result<(), DidSyntaxError> {
    if bytes
        .get(*cursor + 1..*cursor + 3)
        .is_none_or(|digits| digits.len() != 2 || !digits.iter().all(u8::is_ascii_hexdigit))
    {
        return Err(DidSyntaxError::InvalidPercentEncoding);
    }
    *cursor += 3;
    Ok(())
}

const fn is_method_char(byte: u8) -> bool {
    byte.is_ascii_lowercase() || byte.is_ascii_digit()
}

const fn is_id_char(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'_' | b':')
}

const fn is_pchar(byte: u8) -> bool {
    is_unreserved(byte) || is_sub_delim(byte) || matches!(byte, b':' | b'@')
}

const fn is_query_or_fragment_char(byte: u8) -> bool {
    is_pchar(byte) || matches!(byte, b'/' | b'?')
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
