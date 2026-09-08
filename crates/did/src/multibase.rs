//! Private multibase carrier validation for DID verification material.
//!
//! This module validates only the interoperable `z` (base58-btc) and `u`
//! (base64url-no-pad) carriers. It deliberately does not interpret multicodec
//! prefixes or cryptographic key semantics.

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};

pub(crate) fn is_canonical_public_key_carrier(value: &str) -> bool {
    let Some((prefix, encoded)) = value.as_bytes().split_first() else {
        return false;
    };
    if encoded.is_empty() {
        return false;
    }

    let decoded = match prefix {
        b'z' => match bs58::decode(encoded).into_vec() {
            Ok(decoded) => decoded,
            Err(_) => return false,
        },
        b'u' => match URL_SAFE_NO_PAD.decode(encoded) {
            Ok(decoded) => decoded,
            Err(_) => return false,
        },
        _ => return false,
    };
    if decoded.is_empty() {
        return false;
    }

    match prefix {
        b'z' => bs58::encode(decoded).into_string().as_bytes() == encoded,
        b'u' => URL_SAFE_NO_PAD.encode(decoded).as_bytes() == encoded,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::is_canonical_public_key_carrier;

    #[test]
    fn accepts_canonical_supported_bases() {
        assert!(is_canonical_public_key_carrier("zCn8eVZg"));
        assert!(is_canonical_public_key_carrier("uaGVsbG8"));
    }

    #[test]
    fn rejects_empty_malformed_noncanonical_and_unsupported_values() {
        for value in [
            "",
            "z",
            "u",
            "z0",
            "uAQ==",
            "uAR",
            "f68656c6c6f",
            "R%69 VD92EX0",
            "🚀🚀",
        ] {
            assert!(!is_canonical_public_key_carrier(value), "{value}");
        }
    }
}
