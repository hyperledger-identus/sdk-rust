//! Base64URL-no-pad string wrapper (ported from neoprism, without serde/utoipa).

use std::fmt;
use std::str::FromStr;

use base64::Engine;

use crate::{MAX_CRYPTO_TEXT_BYTES, error::Error};

const MAX_BASE64URL_BYTES: usize = (MAX_CRYPTO_TEXT_BYTES / 4) * 3;

/// A string holding the canonical base64url encoding (no padding) of some bytes.
///
/// Construct from bytes via [`Base64UrlStrNoPad::try_from_bytes`] or
/// [`TryFrom`], or parse encoded text via [`Base64UrlStrNoPad::from_str`]. The
/// inner string is always a valid base64url-no-pad encoding, so
/// [`Base64UrlStrNoPad::to_bytes`] is infallible. Both construction paths
/// enforce [`crate::MAX_CRYPTO_TEXT_BYTES`] before encoding or decoding.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Base64UrlStrNoPad(String);

impl Base64UrlStrNoPad {
    /// Encode raw bytes as canonical unpadded base64url.
    ///
    /// # Errors
    ///
    /// Returns [`Error::KeyParsing`] when the encoded value would exceed
    /// [`crate::MAX_CRYPTO_TEXT_BYTES`].
    pub fn try_from_bytes(value: impl AsRef<[u8]>) -> Result<Self, Error> {
        let bytes = value.as_ref();
        if bytes.len() > MAX_BASE64URL_BYTES {
            let trailing_len = match bytes.len() % 3 {
                0 => 0,
                1 => 2,
                2 => 3,
                _ => unreachable!("remainder modulo three"),
            };
            let encoded_len = (bytes.len() / 3)
                .checked_mul(4)
                .and_then(|len| len.checked_add(trailing_len))
                .unwrap_or(usize::MAX);
            return Err(Error::encoded_text_too_large(
                "base64url",
                MAX_CRYPTO_TEXT_BYTES,
                encoded_len,
            ));
        }
        Ok(Self::encode_trusted(bytes))
    }

    pub(crate) fn encode_trusted(value: &[u8]) -> Self {
        Self(base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(value))
    }

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

impl TryFrom<&[u8]> for Base64UrlStrNoPad {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::try_from_bytes(value)
    }
}

impl TryFrom<Vec<u8>> for Base64UrlStrNoPad {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from_bytes(value)
    }
}

impl<const N: usize> TryFrom<[u8; N]> for Base64UrlStrNoPad {
    type Error = Error;

    fn try_from(value: [u8; N]) -> Result<Self, Self::Error> {
        Self::try_from_bytes(value)
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for Base64UrlStrNoPad {
    type Error = Error;

    fn try_from(value: &[u8; N]) -> Result<Self, Self::Error> {
        Self::try_from_bytes(value)
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
        if s.len() > MAX_CRYPTO_TEXT_BYTES {
            return Err(Error::encoded_text_too_large(
                "base64url",
                MAX_CRYPTO_TEXT_BYTES,
                s.len(),
            ));
        }
        let engine = base64::engine::general_purpose::URL_SAFE_NO_PAD;
        let bytes = engine
            .decode(s.as_bytes())
            .map_err(|source| Error::KeyParsing {
                source: Box::new(source),
            })?;
        Ok(Self::encode_trusted(bytes.as_slice()))
    }
}

#[cfg(test)]
mod tests {
    use identus_core::{CapabilityId, ErrorKind};

    use super::{Base64UrlStrNoPad, MAX_CRYPTO_TEXT_BYTES};
    use std::str::FromStr;

    const SENTINEL: &str = "never-print-this-base64url-input";

    #[test]
    fn exact_limit_is_accepted_and_remains_canonical() {
        let bytes = vec![0u8; 3 * MAX_CRYPTO_TEXT_BYTES / 4];
        let encoded = Base64UrlStrNoPad::try_from_bytes(&bytes).expect("exact raw limit");
        assert_eq!(encoded.as_str().len(), MAX_CRYPTO_TEXT_BYTES);

        let parsed = Base64UrlStrNoPad::from_str(encoded.as_str()).expect("exact limit");
        assert_eq!(parsed, encoded);
        assert_eq!(parsed.to_bytes(), bytes);
    }

    #[test]
    fn one_over_limit_wins_before_malformed_base64url_and_is_redacted() {
        let input = format!(
            "{}{}",
            "A".repeat(MAX_CRYPTO_TEXT_BYTES + 1 - SENTINEL.len()),
            SENTINEL
        );
        assert_eq!(input.len(), MAX_CRYPTO_TEXT_BYTES + 1);

        let error = Base64UrlStrNoPad::from_str(&input).expect_err("one byte over");
        let local = error.to_string();
        assert!(local.contains("base64url input exceeds the 4096-byte limit"));
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
        let bytes = vec![0u8; 3 * MAX_CRYPTO_TEXT_BYTES / 4];
        let encoded = Base64UrlStrNoPad::try_from_bytes(&bytes).expect("exact raw limit");
        assert_eq!(encoded.as_str().len(), MAX_CRYPTO_TEXT_BYTES);
        assert_eq!(encoded.to_bytes(), bytes);

        let error = Base64UrlStrNoPad::try_from_bytes(vec![0u8; 3 * MAX_CRYPTO_TEXT_BYTES / 4 + 1])
            .expect_err("one byte over raw limit");
        assert!(error.to_string().contains("4098 bytes"));

        let array = [0u8];
        for converted in [
            Base64UrlStrNoPad::try_from(array.as_slice()).expect("borrowed slice"),
            Base64UrlStrNoPad::try_from(vec![0u8]).expect("owned vector"),
            Base64UrlStrNoPad::try_from(array).expect("owned array"),
            Base64UrlStrNoPad::try_from(&array).expect("borrowed array"),
        ] {
            assert_eq!(converted.as_str(), "AA");
        }
    }

    #[test]
    fn valid_oversized_text_is_rejected_but_trusted_encoding_remains_infallible() {
        let bytes = vec![0u8; 3 * MAX_CRYPTO_TEXT_BYTES / 4 + 1];
        let encoded = Base64UrlStrNoPad::encode_trusted(&bytes);
        assert_eq!(encoded.as_str().len(), MAX_CRYPTO_TEXT_BYTES + 2);
        assert_eq!(encoded.to_bytes(), bytes);

        let error =
            Base64UrlStrNoPad::from_str(encoded.as_str()).expect_err("valid oversized base64url");
        assert!(error.to_string().contains("4098 bytes"));
    }
}
