//! Hex string wrapper (ported from neoprism, without serde/utoipa).

use std::fmt;
use std::str::FromStr;

use crate::{MAX_CRYPTO_TEXT_BYTES, error::Error};

const MAX_HEX_BYTES: usize = MAX_CRYPTO_TEXT_BYTES / 2;

/// A string holding the canonical lowercase hex encoding of some bytes.
///
/// Construct from bytes via [`HexStr::try_from_bytes`] or [`TryFrom`], or parse
/// encoded text via [`HexStr::from_str`]. The inner string is always a valid
/// hex encoding, so [`HexStr::to_bytes`] is infallible. Both construction paths
/// enforce [`crate::MAX_CRYPTO_TEXT_BYTES`] before encoding or decoding.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HexStr(String);

impl HexStr {
    /// Encode raw bytes as canonical lowercase hex.
    ///
    /// # Errors
    ///
    /// Returns [`Error::KeyParsing`] when the encoded value would exceed
    /// [`crate::MAX_CRYPTO_TEXT_BYTES`].
    pub fn try_from_bytes(value: impl AsRef<[u8]>) -> Result<Self, Error> {
        let bytes = value.as_ref();
        if bytes.len() > MAX_HEX_BYTES {
            let encoded_len = bytes.len().saturating_mul(2);
            return Err(Error::encoded_text_too_large(
                "hex",
                MAX_CRYPTO_TEXT_BYTES,
                encoded_len,
            ));
        }
        Ok(Self::encode_trusted(bytes))
    }

    pub(crate) fn encode_trusted(value: &[u8]) -> Self {
        Self(hex::encode(value))
    }

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

impl TryFrom<&[u8]> for HexStr {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::try_from_bytes(value)
    }
}

impl TryFrom<Vec<u8>> for HexStr {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from_bytes(value)
    }
}

impl<const N: usize> TryFrom<[u8; N]> for HexStr {
    type Error = Error;

    fn try_from(value: [u8; N]) -> Result<Self, Self::Error> {
        Self::try_from_bytes(value)
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for HexStr {
    type Error = Error;

    fn try_from(value: &[u8; N]) -> Result<Self, Self::Error> {
        Self::try_from_bytes(value)
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
        Ok(Self::encode_trusted(bytes.as_slice()))
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
    fn byte_encoding_is_bounded_and_conversion_forms_compile() {
        let bytes = vec![0xab; MAX_CRYPTO_TEXT_BYTES / 2];
        let encoded = HexStr::try_from_bytes(&bytes).expect("exact raw limit");
        assert_eq!(encoded.as_str().len(), MAX_CRYPTO_TEXT_BYTES);
        assert_eq!(encoded.to_bytes(), bytes);

        let error = HexStr::try_from_bytes(vec![0xab; MAX_CRYPTO_TEXT_BYTES / 2 + 1])
            .expect_err("one byte over raw limit");
        assert!(error.to_string().contains("4098 bytes"));

        let array = [0xab];
        for converted in [
            HexStr::try_from(array.as_slice()).expect("borrowed slice"),
            HexStr::try_from(vec![0xab]).expect("owned vector"),
            HexStr::try_from(array).expect("owned array"),
            HexStr::try_from(&array).expect("borrowed array"),
        ] {
            assert_eq!(converted.as_str(), "ab");
        }
    }

    #[test]
    fn valid_oversized_text_is_rejected_but_trusted_encoding_remains_infallible() {
        let bytes = vec![0xab; MAX_CRYPTO_TEXT_BYTES / 2 + 1];
        let encoded = HexStr::encode_trusted(&bytes);
        assert_eq!(encoded.as_str().len(), MAX_CRYPTO_TEXT_BYTES + 2);
        assert_eq!(encoded.to_bytes(), bytes);

        let error = HexStr::from_str(encoded.as_str()).expect_err("valid oversized hex");
        assert!(error.to_string().contains("4098 bytes"));
    }
}
