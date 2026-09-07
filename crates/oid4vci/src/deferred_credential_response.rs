use std::fmt;

use zeroize::Zeroizing;

use crate::{
    CredentialOfferError, DeferredCredentialResponseLimits,
    json::parse_deferred_credential_response_fields,
};

/// An opaque handle for a deferred OID4VCI issuance transaction.
pub struct DeferredTransactionId {
    value: Zeroizing<String>,
}

impl DeferredTransactionId {
    /// Borrow the exact transaction handle for a later bounded request.
    ///
    /// Keep the value out of logs, URLs, telemetry, caches, generic
    /// serializers, and unrelated or long-lived storage.
    pub fn expose_sensitive_transaction_id(&self) -> &str {
        &self.value
    }
}

impl fmt::Debug for DeferredTransactionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DeferredTransactionId")
            .finish_non_exhaustive()
    }
}

/// The exact mathematically positive JSON number supplied as a polling hint.
pub struct DeferredCredentialInterval {
    value: Zeroizing<String>,
}

impl DeferredCredentialInterval {
    /// Borrow the exact validated JSON-number lexeme.
    ///
    /// This value is remote protocol input, not an SDK scheduling decision.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl fmt::Debug for DeferredCredentialInterval {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DeferredCredentialInterval")
            .finish_non_exhaustive()
    }
}

/// A bounded unencrypted deferred OID4VCI Credential Response body.
///
/// This state proves only JSON body syntax. It does not prove HTTP semantics,
/// transport or Issuer provenance, request correlation, transaction validity,
/// or polling safety.
pub struct DeferredCredentialResponseCore {
    response_len: usize,
    transaction_id: DeferredTransactionId,
    interval: DeferredCredentialInterval,
}

impl DeferredCredentialResponseCore {
    /// Parse a deferred Credential Response under explicit resource limits.
    ///
    /// The caller retains responsibility for erasing its input allocation.
    pub fn parse(
        json: &str,
        limits: DeferredCredentialResponseLimits,
    ) -> Result<Self, CredentialOfferError> {
        if json.len() > limits.max_json_bytes() {
            return Err(CredentialOfferError::DeferredCredentialResponseTooLarge);
        }
        let fields = parse_deferred_credential_response_fields(json.as_bytes(), limits)?;
        Ok(Self {
            response_len: json.len(),
            transaction_id: DeferredTransactionId {
                value: fields.transaction_id,
            },
            interval: DeferredCredentialInterval {
                value: fields.interval,
            },
        })
    }

    /// Return the exact response byte count.
    pub const fn response_len(&self) -> usize {
        self.response_len
    }

    /// Borrow the opaque deferred transaction handle.
    pub const fn transaction_id(&self) -> &DeferredTransactionId {
        &self.transaction_id
    }

    /// Borrow the exact positive interval value without numeric conversion.
    pub const fn interval(&self) -> &DeferredCredentialInterval {
        &self.interval
    }
}

impl fmt::Debug for DeferredCredentialResponseCore {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DeferredCredentialResponseCore")
            .field("response_bytes", &self.response_len)
            .finish_non_exhaustive()
    }
}
