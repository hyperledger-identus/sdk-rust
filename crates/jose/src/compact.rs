//! Bounded JWS Compact parsing and staged encoding.

use std::fmt;

use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;

use crate::{JoseError, JwsLimits, ProtectedHeader};

/// Canonically encoded protected-header and payload bytes awaiting a signature.
///
/// Sign [`JwsSigningInput::as_bytes`] through an external signing capability,
/// then attach the result with [`JwsSigningInput::attach_signature`].
#[derive(Clone, PartialEq, Eq)]
pub struct JwsSigningInput {
    protected_header: ProtectedHeader,
    payload: Vec<u8>,
    encoded: String,
    limits: JwsLimits,
}

impl JwsSigningInput {
    /// Encode a validated protected header and arbitrary payload canonically.
    pub fn new(
        protected_header: ProtectedHeader,
        payload: Vec<u8>,
        limits: JwsLimits,
    ) -> Result<Self, JoseError> {
        protected_header.validate_for(limits)?;
        if payload.len() > limits.max_payload_bytes() {
            return Err(JoseError::PayloadTooLarge);
        }
        let header_bytes =
            serde_json::to_vec(&protected_header).map_err(|_| JoseError::InvalidProtectedHeader)?;
        if header_bytes.len() > limits.max_protected_header_bytes() {
            return Err(JoseError::ProtectedHeaderTooLarge);
        }
        let encoded_header = URL_SAFE_NO_PAD.encode(header_bytes);
        let encoded_payload = URL_SAFE_NO_PAD.encode(&payload);
        let encoded_len = encoded_header
            .len()
            .checked_add(1)
            .and_then(|length| length.checked_add(encoded_payload.len()))
            .ok_or(JoseError::SizeOverflow)?;
        // The final form needs the last separator plus at least two base64url
        // characters for the required one-byte signature.
        let minimum_compact_len = encoded_len.checked_add(3).ok_or(JoseError::SizeOverflow)?;
        if minimum_compact_len > limits.max_compact_bytes() {
            return Err(JoseError::CompactTooLarge);
        }
        let mut encoded = String::with_capacity(encoded_len);
        encoded.push_str(&encoded_header);
        encoded.push('.');
        encoded.push_str(&encoded_payload);
        Ok(Self {
            protected_header,
            payload,
            encoded,
            limits,
        })
    }

    /// Exact ASCII bytes that an external signer must sign.
    pub fn as_bytes(&self) -> &[u8] {
        self.encoded.as_bytes()
    }

    /// Validated protected header associated with this signing input.
    pub const fn protected_header(&self) -> &ProtectedHeader {
        &self.protected_header
    }

    /// Exact payload octets encoded by this signing input.
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    /// Attach external signature bytes and create an explicitly unverified JWS.
    pub fn attach_signature(self, signature: Vec<u8>) -> Result<UnverifiedCompactJws, JoseError> {
        if signature.is_empty() {
            return Err(JoseError::EmptySignature);
        }
        if signature.len() > self.limits.max_signature_bytes() {
            return Err(JoseError::SignatureTooLarge);
        }
        let encoded_signature = URL_SAFE_NO_PAD.encode(&signature);
        let compact_len = self
            .encoded
            .len()
            .checked_add(1)
            .and_then(|length| length.checked_add(encoded_signature.len()))
            .ok_or(JoseError::SizeOverflow)?;
        if compact_len > self.limits.max_compact_bytes() {
            return Err(JoseError::CompactTooLarge);
        }
        let signing_input_end = self.encoded.len();
        let mut compact = self.encoded;
        compact.push('.');
        compact.push_str(&encoded_signature);
        Ok(UnverifiedCompactJws {
            protected_header: self.protected_header,
            payload: self.payload,
            signature,
            compact,
            signing_input_end,
        })
    }
}

impl fmt::Debug for JwsSigningInput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("JwsSigningInput")
            .field("algorithm", &self.protected_header.algorithm())
            .field("type", &self.protected_header.type_())
            .field("has_key_id", &self.protected_header.key_id().is_some())
            .field("payload_len", &self.payload.len())
            .field("signing_input_len", &self.encoded.len())
            .finish()
    }
}

/// A structurally valid but cryptographically unverified JWS Compact value.
#[derive(Clone, PartialEq, Eq)]
pub struct UnverifiedCompactJws {
    protected_header: ProtectedHeader,
    payload: Vec<u8>,
    signature: Vec<u8>,
    compact: String,
    signing_input_end: usize,
}

impl UnverifiedCompactJws {
    /// Parse one bounded canonical JWS Compact representation.
    pub fn parse(compact: &str, limits: JwsLimits) -> Result<Self, JoseError> {
        if compact.len() > limits.max_compact_bytes() {
            return Err(JoseError::CompactTooLarge);
        }
        let (encoded_header, remainder) = compact
            .split_once('.')
            .ok_or(JoseError::InvalidCompactStructure)?;
        let (encoded_payload, encoded_signature) = remainder
            .split_once('.')
            .ok_or(JoseError::InvalidCompactStructure)?;
        if encoded_header.is_empty()
            || encoded_signature.is_empty()
            || encoded_signature.contains('.')
        {
            return Err(JoseError::InvalidCompactStructure);
        }

        let header_bytes = decode_segment(
            encoded_header,
            limits.max_protected_header_bytes(),
            JoseError::ProtectedHeaderTooLarge,
        )?;
        let payload = decode_segment(
            encoded_payload,
            limits.max_payload_bytes(),
            JoseError::PayloadTooLarge,
        )?;
        let signature = decode_segment(
            encoded_signature,
            limits.max_signature_bytes(),
            JoseError::SignatureTooLarge,
        )?;
        if signature.is_empty() {
            return Err(JoseError::EmptySignature);
        }
        let protected_header = ProtectedHeader::parse(&header_bytes, limits)?;
        let signing_input_end = encoded_header
            .len()
            .checked_add(1)
            .and_then(|length| length.checked_add(encoded_payload.len()))
            .ok_or(JoseError::SizeOverflow)?;
        Ok(Self {
            protected_header,
            payload,
            signature,
            compact: compact.to_owned(),
            signing_input_end,
        })
    }

    /// The validated protected header. Its values remain untrusted policy input.
    pub const fn protected_header(&self) -> &ProtectedHeader {
        &self.protected_header
    }

    /// Exact decoded payload octets.
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    /// Exact decoded, unverified signature octets.
    pub fn signature(&self) -> &[u8] {
        &self.signature
    }

    /// Exact original compact representation.
    pub fn compact(&self) -> &str {
        &self.compact
    }

    /// Exact received ASCII protected-header and payload segments to verify.
    pub fn signing_input(&self) -> &[u8] {
        &self.compact.as_bytes()[..self.signing_input_end]
    }
}

impl fmt::Debug for UnverifiedCompactJws {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("UnverifiedCompactJws")
            .field("algorithm", &self.protected_header.algorithm())
            .field("type", &self.protected_header.type_())
            .field("has_key_id", &self.protected_header.key_id().is_some())
            .field("payload_len", &self.payload.len())
            .field("signature_len", &self.signature.len())
            .field("compact_len", &self.compact.len())
            .finish()
    }
}

fn decode_segment(
    encoded: &str,
    max_decoded_bytes: usize,
    size_error: JoseError,
) -> Result<Vec<u8>, JoseError> {
    if !encoded
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    {
        return Err(JoseError::NonCanonicalBase64Url);
    }
    let estimate =
        decoded_length_estimate(encoded.len()).ok_or(JoseError::NonCanonicalBase64Url)?;
    if estimate > max_decoded_bytes {
        return Err(size_error);
    }
    let decoded = URL_SAFE_NO_PAD
        .decode(encoded)
        .map_err(|_| JoseError::NonCanonicalBase64Url)?;
    if decoded.len() > max_decoded_bytes {
        return Err(size_error);
    }
    if URL_SAFE_NO_PAD.encode(&decoded) != encoded {
        return Err(JoseError::NonCanonicalBase64Url);
    }
    Ok(decoded)
}

fn decoded_length_estimate(encoded_length: usize) -> Option<usize> {
    let complete = encoded_length.checked_div(4)?.checked_mul(3)?;
    let remainder = match encoded_length % 4 {
        0 => 0,
        2 => 1,
        3 => 2,
        1 => return None,
        _ => unreachable!("remainder modulo four is bounded"),
    };
    complete.checked_add(remainder)
}
