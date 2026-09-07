use std::fmt;

use zeroize::Zeroizing;

use crate::{
    CredentialErrorResponseLimits, CredentialOfferError,
    json::parse_credential_error_response_fields,
};

/// Closed classification of OID4VCI Final Credential Endpoint error codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CredentialEndpointErrorKind {
    InvalidCredentialRequest,
    UnknownCredentialConfiguration,
    UnknownCredentialIdentifier,
    InvalidProof,
    InvalidNonce,
    InvalidEncryptionParameters,
    CredentialRequestDenied,
    Extension,
}

/// A validated, exact OID4VCI Credential Endpoint error code.
pub struct CredentialEndpointErrorCode {
    value: Zeroizing<String>,
}

impl CredentialEndpointErrorCode {
    /// Borrow the exact case-sensitive error code.
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Classify exact Final values without assigning policy to extensions.
    pub fn kind(&self) -> CredentialEndpointErrorKind {
        match self.value.as_str() {
            "invalid_credential_request" => CredentialEndpointErrorKind::InvalidCredentialRequest,
            "unknown_credential_configuration" => {
                CredentialEndpointErrorKind::UnknownCredentialConfiguration
            }
            "unknown_credential_identifier" => {
                CredentialEndpointErrorKind::UnknownCredentialIdentifier
            }
            "invalid_proof" => CredentialEndpointErrorKind::InvalidProof,
            "invalid_nonce" => CredentialEndpointErrorKind::InvalidNonce,
            "invalid_encryption_parameters" => {
                CredentialEndpointErrorKind::InvalidEncryptionParameters
            }
            "credential_request_denied" => CredentialEndpointErrorKind::CredentialRequestDenied,
            _ => CredentialEndpointErrorKind::Extension,
        }
    }
}

impl fmt::Debug for CredentialEndpointErrorCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialEndpointErrorCode")
            .field("kind", &self.kind())
            .finish_non_exhaustive()
    }
}

/// A bounded partial OID4VCI Credential Error Response.
///
/// This state proves only the syntax of known fields. It does not prove HTTP
/// semantics, response provenance, truth, retryability, or display safety.
pub struct CredentialErrorResponseCore {
    response_len: usize,
    error: CredentialEndpointErrorCode,
    error_description: Option<Zeroizing<String>>,
}

impl CredentialErrorResponseCore {
    /// Parse a Credential Error Response core under explicit resource limits.
    ///
    /// The caller retains responsibility for erasing its input allocation.
    pub fn parse(
        json: &str,
        limits: CredentialErrorResponseLimits,
    ) -> Result<Self, CredentialOfferError> {
        if json.len() > limits.max_json_bytes() {
            return Err(CredentialOfferError::CredentialErrorResponseTooLarge);
        }
        let fields = parse_credential_error_response_fields(json.as_bytes(), limits)?;
        if !is_nqschar(&fields.error) {
            return Err(CredentialOfferError::InvalidCredentialEndpointErrorCode);
        }
        if fields
            .error_description
            .as_ref()
            .is_some_and(|description| !is_nqschar(description))
        {
            return Err(CredentialOfferError::InvalidCredentialErrorDescription);
        }
        Ok(Self {
            response_len: json.len(),
            error: CredentialEndpointErrorCode {
                value: fields.error,
            },
            error_description: fields.error_description,
        })
    }

    /// Return the exact response byte count.
    pub const fn response_len(&self) -> usize {
        self.response_len
    }

    /// Borrow the exact validated error code.
    pub const fn error(&self) -> &CredentialEndpointErrorCode {
        &self.error
    }

    /// Return the closed classification of the exact error code.
    pub fn error_kind(&self) -> CredentialEndpointErrorKind {
        self.error.kind()
    }

    /// Report whether developer-oriented error information is present.
    pub const fn error_description_present(&self) -> bool {
        self.error_description.is_some()
    }

    /// Borrow untrusted developer-oriented error information, if present.
    ///
    /// This text is remote, not localized, and not inherently safe to display.
    pub fn expose_untrusted_description(&self) -> Option<&str> {
        self.error_description.as_ref().map(|value| value.as_str())
    }
}

impl fmt::Debug for CredentialErrorResponseCore {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialErrorResponseCore")
            .field("response_bytes", &self.response_len)
            .field("error_kind", &self.error.kind())
            .field(
                "error_description_present",
                &self.error_description.is_some(),
            )
            .finish_non_exhaustive()
    }
}

fn is_nqschar(value: &str) -> bool {
    value
        .bytes()
        .all(|byte| matches!(byte, 0x20..=0x21 | 0x23..=0x5b | 0x5d..=0x7e))
}
