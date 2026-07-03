//! Hex string wrapper (ported from neoprism, without serde/utoipa).

use std::fmt;
use std::str::FromStr;

use crate::error::Error;

/// A string holding the canonical lowercase hex encoding of some bytes.
///
/// Construct via [`HexStr::from`] (encoding) or [`HexStr::from_str`] (decoding).
/// The inner string is always a valid hex encoding, so [`HexStr::to_bytes`] is
/// infallible.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HexStr(String);

impl HexStr {
    /// Decode the held string back to raw bytes. Infallible by construction.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        hex::decode(&self.0).expect("HexStr holds a valid hex encoding")
    }

    /// The inner encoded string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<B: AsRef<[u8]>> From<B> for HexStr {
    fn from(value: B) -> Self {
        Self(hex::encode(value.as_ref()))
    }
}

impl fmt::Display for HexStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for HexStr {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl FromStr for HexStr {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = hex::decode(s).map_err(|source| Error::KeyParsing {
            source: Box::new(source),
        })?;
        Ok(bytes.as_slice().into())
    }
}
