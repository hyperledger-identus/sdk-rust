use std::fmt;

use zeroize::Zeroizing;

use crate::{
    CredentialNonceResponseLimits, CredentialOfferError,
    json::parse_credential_nonce_response_fields,
};

/// An exact opaque challenge returned by an OID4VCI Credential Nonce Endpoint.
pub struct CredentialNonce {
    value: Zeroizing<String>,
}

impl CredentialNonce {
    /// Borrow the challenge for immediate proof construction.
    ///
    /// Keep the value out of logs, URLs, telemetry, caches, generic
    /// serializers, and unrelated or long-lived storage.
    pub fn expose_sensitive_nonce(&self) -> &str {
        &self.value
    }
}

impl fmt::Debug for CredentialNonce {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialNonce")
            .finish_non_exhaustive()
    }
}

/// A bounded partial OID4VCI Credential Nonce Response.
///
/// This state proves only JSON body syntax. It does not prove transport
/// semantics, Issuer provenance, unpredictability, freshness, or replay safety.
pub struct CredentialNonceResponseCore {
    response_len: usize,
    nonce: CredentialNonce,
}

impl CredentialNonceResponseCore {
    /// Parse a Credential Nonce Response core under explicit resource limits.
    ///
    /// The caller retains responsibility for erasing its input allocation.
    pub fn parse(
        json: &str,
        limits: CredentialNonceResponseLimits,
    ) -> Result<Self, CredentialOfferError> {
        if json.len() > limits.max_json_bytes() {
            return Err(CredentialOfferError::CredentialNonceResponseTooLarge);
        }
        let fields = parse_credential_nonce_response_fields(json.as_bytes(), limits)?;
        Ok(Self {
            response_len: json.len(),
            nonce: CredentialNonce {
                value: fields.nonce,
            },
        })
    }

    /// Return the exact response byte count.
    pub const fn response_len(&self) -> usize {
        self.response_len
    }

    /// Borrow the opaque Credential Nonce.
    pub const fn nonce(&self) -> &CredentialNonce {
        &self.nonce
    }
}

impl fmt::Debug for CredentialNonceResponseCore {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialNonceResponseCore")
            .field("response_bytes", &self.response_len)
            .finish_non_exhaustive()
    }
}
