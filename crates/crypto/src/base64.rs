//! Base64URL-no-pad string wrapper (ported from neoprism, without serde/utoipa).

use std::fmt;
use std::str::FromStr;

use base64::Engine;

use crate::error::Error;

/// A string holding the canonical base64url encoding (no padding) of some bytes.
///
/// Construct via [`Base64UrlStrNoPad::from`] (encoding) or
/// [`Base64UrlStrNoPad::from_str`] (decoding). The inner string is always a
/// valid base64url-no-pad encoding, so [`Base64UrlStrNoPad::to_bytes`] is
/// infallible.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Base64UrlStrNoPad(String);

impl Base64UrlStrNoPad {
    /// Decode the held string back to raw bytes. Infallible by construction.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(self.0.as_bytes())
            .expect("Base64UrlStrNoPad holds a valid base64url-no-pad encoding")
    }

    /// The inner encoded string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<B: AsRef<[u8]>> From<B> for Base64UrlStrNoPad {
    fn from(value: B) -> Self {
        Self(base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(value.as_ref()))
    }
}

impl fmt::Display for Base64UrlStrNoPad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for Base64UrlStrNoPad {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(feature = "jwk")]
impl serde::Serialize for Base64UrlStrNoPad {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl FromStr for Base64UrlStrNoPad {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let engine = base64::engine::general_purpose::URL_SAFE_NO_PAD;
        let bytes = engine
            .decode(s.as_bytes())
            .map_err(|source| Error::KeyParsing {
                source: Box::new(source),
            })?;
        Ok(bytes.as_slice().into())
    }
}
