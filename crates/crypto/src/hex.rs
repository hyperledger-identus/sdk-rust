//! Hex string wrapper (ported from neoprism, without serde/utoipa).

use std::fmt;
use std::str::FromStr;

use crate::{MAX_CRYPTO_TEXT_BYTES, error::Error};

/// A string holding the canonical lowercase hex encoding of some bytes.
///
/// Construct via [`HexStr::from`] (encoding) or [`HexStr::from_str`] (decoding).
/// The inner string is always a valid hex encoding, so [`HexStr::to_bytes`] is
/// infallible. Parsing rejects text above [`crate::MAX_CRYPTO_TEXT_BYTES`]
/// before decoding. Encoding caller-owned bytes remains infallible and may
/// produce a value that is too large to reparse through [`FromStr`].
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
        if s.len() > MAX_CRYPTO_TEXT_BYTES {
            return Err(Error::encoded_text_too_large(
                "hex",
                MAX_CRYPTO_TEXT_BYTES,
                s.len(),
            ));
        }
        let bytes = hex::decode(s).map_err(|source| Error::KeyParsing {
            source: Box::new(source),
        })?;
        Ok(bytes.as_slice().into())
    }
}

#[cfg(test)]
mod tests {
    use identus_core::{CapabilityId, ErrorKind};

    use super::{HexStr, MAX_CRYPTO_TEXT_BYTES};
    use std::str::FromStr;

    const SENTINEL: &str = "never-print-this-hex-input";

    #[test]
    fn exact_limit_is_accepted_and_canonicalized() {
        let uppercase = "AB".repeat(MAX_CRYPTO_TEXT_BYTES / 2);
        let parsed = HexStr::from_str(&uppercase).expect("exact limit");

        assert_eq!(parsed.as_str(), "ab".repeat(MAX_CRYPTO_TEXT_BYTES / 2));
        assert_eq!(parsed.to_bytes(), vec![0xab; MAX_CRYPTO_TEXT_BYTES / 2]);
    }

    #[test]
    fn one_over_limit_wins_before_malformed_hex_and_is_redacted() {
        let input = format!(
            "{}{}",
            "g".repeat(MAX_CRYPTO_TEXT_BYTES + 1 - SENTINEL.len()),
            SENTINEL
        );
        assert_eq!(input.len(), MAX_CRYPTO_TEXT_BYTES + 1);

        let error = HexStr::from_str(&input).expect_err("one byte over");
        let local = error.to_string();
        assert!(local.contains("hex input exceeds the 4096-byte limit"));
        assert!(local.contains("4097 bytes"));
        assert!(!local.contains(SENTINEL));

        let bridged = error.to_identus_error();
        assert_eq!(bridged.code().as_str(), "crypto.key_parsing");
        assert_eq!(bridged.kind(), ErrorKind::InvalidInput);
        assert_eq!(bridged.capability(), Some(CapabilityId::new("crypto")));
        assert_eq!(
            bridged.to_string(),
            "crypto.key_parsing: key parsing failed"
        );
        assert!(!bridged.to_string().contains(SENTINEL));
    }

    #[test]
    fn valid_oversized_text_is_rejected_but_trusted_encoding_remains_infallible() {
        let bytes = vec![0xab; MAX_CRYPTO_TEXT_BYTES / 2 + 1];
        let encoded = HexStr::from(&bytes);
        assert_eq!(encoded.as_str().len(), MAX_CRYPTO_TEXT_BYTES + 2);
        assert_eq!(encoded.to_bytes(), bytes);

        let error = HexStr::from_str(encoded.as_str()).expect_err("valid oversized hex");
        assert!(error.to_string().contains("4098 bytes"));
    }
}
