//! Base64URL-no-pad string wrapper (ported from neoprism, without serde/utoipa).

use std::fmt;
use std::str::FromStr;

use base64::Engine;

use crate::{MAX_CRYPTO_TEXT_BYTES, error::Error};

/// A string holding the canonical base64url encoding (no padding) of some bytes.
///
/// Construct via [`Base64UrlStrNoPad::from`] (encoding) or
/// [`Base64UrlStrNoPad::from_str`] (decoding). The inner string is always a
/// valid base64url-no-pad encoding, so [`Base64UrlStrNoPad::to_bytes`] is
/// infallible. Parsing rejects text above [`crate::MAX_CRYPTO_TEXT_BYTES`]
/// before decoding. Encoding caller-owned bytes remains infallible and may
/// produce a value that is too large to reparse through [`FromStr`].
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
        Ok(bytes.as_slice().into())
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
        let encoded = Base64UrlStrNoPad::from(&bytes);
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
    fn valid_oversized_text_is_rejected_but_trusted_encoding_remains_infallible() {
        let bytes = vec![0u8; 3 * MAX_CRYPTO_TEXT_BYTES / 4 + 1];
        let encoded = Base64UrlStrNoPad::from(&bytes);
        assert_eq!(encoded.as_str().len(), MAX_CRYPTO_TEXT_BYTES + 2);
        assert_eq!(encoded.to_bytes(), bytes);

        let error =
            Base64UrlStrNoPad::from_str(encoded.as_str()).expect_err("valid oversized base64url");
        assert!(error.to_string().contains("4098 bytes"));
    }
}
