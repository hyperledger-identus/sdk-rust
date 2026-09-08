use std::{fmt, str};

use fluent_uri::Uri as ParsedUri;
use zeroize::Zeroizing;

use crate::{CredentialOfferError, CredentialOfferLimits, json::validate_json};

const SCHEME: &str = "openid-credential-offer";

/// A validated Credential Offer invocation transport.
#[derive(Debug)]
pub enum CredentialOfferRequest {
    Embedded(EmbeddedCredentialOffer),
    Referenced(CredentialOfferReference),
}

impl CredentialOfferRequest {
    /// Parse one complete, bounded Credential Offer invocation.
    pub fn parse(input: &str, limits: CredentialOfferLimits) -> Result<Self, CredentialOfferError> {
        if input.len() > limits.max_invocation_bytes() {
            return Err(CredentialOfferError::InvocationTooLarge);
        }
        let (scheme, remainder) = input
            .split_at_checked(SCHEME.len())
            .ok_or(CredentialOfferError::InvalidInvocation)?;
        if !scheme.eq_ignore_ascii_case(SCHEME) || !remainder.starts_with("://?") {
            return Err(CredentialOfferError::InvalidInvocation);
        }
        let query = &remainder[4..];
        if query.is_empty() || query.contains('#') {
            return Err(CredentialOfferError::InvalidInvocation);
        }
        if query.contains('&') {
            return Err(CredentialOfferError::UnsupportedTransport);
        }
        let (name, encoded_value) = query
            .split_once('=')
            .ok_or(CredentialOfferError::UnsupportedTransport)?;
        if encoded_value.is_empty() {
            return Err(CredentialOfferError::UnsupportedTransport);
        }

        match name {
            "credential_offer" => {
                let value = decode_form_value(
                    encoded_value,
                    limits.max_embedded_json_bytes(),
                    CredentialOfferError::EmbeddedTooLarge,
                )?;
                EmbeddedCredentialOffer::try_from_json(&value, limits).map(Self::Embedded)
            }
            "credential_offer_uri" => {
                let value = decode_form_value(
                    encoded_value,
                    limits.max_reference_uri_bytes(),
                    CredentialOfferError::ReferenceTooLarge,
                )?;
                CredentialOfferReference::try_from_uri(&value, limits).map(Self::Referenced)
            }
            _ => Err(CredentialOfferError::UnsupportedTransport),
        }
    }
}

/// An embedded Credential Offer whose transport JSON is bounded and unambiguous.
pub struct EmbeddedCredentialOffer {
    json: Zeroizing<String>,
    limits: CredentialOfferLimits,
}

impl EmbeddedCredentialOffer {
    /// Validate and retain exact embedded Credential Offer JSON.
    pub fn try_from_json(
        json: &str,
        limits: CredentialOfferLimits,
    ) -> Result<Self, CredentialOfferError> {
        if json.len() > limits.max_embedded_json_bytes() {
            return Err(CredentialOfferError::EmbeddedTooLarge);
        }
        validate_json(
            json.as_bytes(),
            limits.max_json_depth(),
            limits.max_json_nodes(),
        )?;
        Ok(Self {
            json: Zeroizing::new(json.to_owned()),
            limits,
        })
    }

    /// Borrow the exact decoded JSON object.
    pub fn as_json(&self) -> &str {
        &self.json
    }

    pub(crate) const fn limits(&self) -> CredentialOfferLimits {
        self.limits
    }
}

impl fmt::Debug for EmbeddedCredentialOffer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("EmbeddedCredentialOffer")
            .finish_non_exhaustive()
    }
}

/// A syntactically safe HTTPS Credential Offer reference.
pub struct CredentialOfferReference {
    uri: Zeroizing<String>,
}

impl CredentialOfferReference {
    /// Validate and retain an exact HTTPS reference URI without fetching it.
    pub fn try_from_uri(
        uri: &str,
        limits: CredentialOfferLimits,
    ) -> Result<Self, CredentialOfferError> {
        if uri.len() > limits.max_reference_uri_bytes() {
            return Err(CredentialOfferError::ReferenceTooLarge);
        }
        let bytes = uri.as_bytes();
        if !bytes
            .get(..8)
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case(b"https://"))
        {
            return Err(CredentialOfferError::UnsafeReferenceUri);
        }
        let authority = &bytes[8..bytes[8..]
            .iter()
            .position(|byte| matches!(byte, b'/' | b'?' | b'#'))
            .map_or(bytes.len(), |offset| 8 + offset)];
        if authority.is_empty() || authority.first() == Some(&b':') {
            return Err(CredentialOfferError::UnsafeReferenceUri);
        }
        let parsed = ParsedUri::parse(uri).map_err(|_| CredentialOfferError::UnsafeReferenceUri)?;
        let authority = parsed
            .authority()
            .ok_or(CredentialOfferError::UnsafeReferenceUri)?;
        if !parsed.scheme().as_str().eq_ignore_ascii_case("https")
            || authority.host().is_empty()
            || authority.userinfo().is_some()
            || parsed.fragment().is_some()
        {
            return Err(CredentialOfferError::UnsafeReferenceUri);
        }
        Ok(Self {
            uri: Zeroizing::new(uri.to_owned()),
        })
    }

    /// Borrow the exact decoded reference URI.
    pub fn as_uri(&self) -> &str {
        &self.uri
    }
}

impl fmt::Debug for CredentialOfferReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialOfferReference")
            .finish_non_exhaustive()
    }
}

fn decode_form_value(
    encoded: &str,
    max_decoded_bytes: usize,
    too_large: CredentialOfferError,
) -> Result<Zeroizing<String>, CredentialOfferError> {
    let input = encoded.as_bytes();
    let mut decoded = Zeroizing::new(Vec::with_capacity(input.len().min(max_decoded_bytes)));
    let mut cursor = 0;

    while cursor < input.len() {
        let byte = match input[cursor] {
            b'+' => {
                cursor += 1;
                b' '
            }
            b'%' => {
                let high = input
                    .get(cursor + 1)
                    .and_then(|value| hex_nibble(*value))
                    .ok_or(CredentialOfferError::InvalidFormEncoding)?;
                let low = input
                    .get(cursor + 2)
                    .and_then(|value| hex_nibble(*value))
                    .ok_or(CredentialOfferError::InvalidFormEncoding)?;
                cursor += 3;
                (high << 4) | low
            }
            byte if byte.is_ascii() => {
                cursor += 1;
                byte
            }
            _ => return Err(CredentialOfferError::InvalidFormEncoding),
        };
        if byte == 0 {
            return Err(CredentialOfferError::InvalidFormEncoding);
        }
        if decoded.len() == max_decoded_bytes {
            return Err(too_large);
        }
        decoded.push(byte);
    }

    let decoded_str =
        str::from_utf8(&decoded).map_err(|_| CredentialOfferError::InvalidFormEncoding)?;
    Ok(Zeroizing::new(decoded_str.to_owned()))
}

const fn hex_nibble(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}
