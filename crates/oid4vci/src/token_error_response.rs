use std::fmt;

use uriparse::URIReference;
use zeroize::Zeroizing;

use crate::{
    CredentialOfferError, TokenErrorResponseLimits, json::parse_token_error_response_fields,
};

/// Closed classification of OAuth 2.0 token-endpoint error codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenEndpointErrorKind {
    InvalidRequest,
    InvalidClient,
    InvalidGrant,
    UnauthorizedClient,
    UnsupportedGrantType,
    InvalidScope,
    Extension,
}

/// A validated, exact OAuth token-endpoint error code.
pub struct TokenEndpointErrorCode {
    value: Zeroizing<String>,
}

impl TokenEndpointErrorCode {
    /// Borrow the exact case-sensitive error code.
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Classify exact RFC 6749 values without assigning policy to extensions.
    pub fn kind(&self) -> TokenEndpointErrorKind {
        match self.value.as_str() {
            "invalid_request" => TokenEndpointErrorKind::InvalidRequest,
            "invalid_client" => TokenEndpointErrorKind::InvalidClient,
            "invalid_grant" => TokenEndpointErrorKind::InvalidGrant,
            "unauthorized_client" => TokenEndpointErrorKind::UnauthorizedClient,
            "unsupported_grant_type" => TokenEndpointErrorKind::UnsupportedGrantType,
            "invalid_scope" => TokenEndpointErrorKind::InvalidScope,
            _ => TokenEndpointErrorKind::Extension,
        }
    }
}

impl fmt::Debug for TokenEndpointErrorCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TokenEndpointErrorCode")
            .field("kind", &self.kind())
            .finish_non_exhaustive()
    }
}

/// A validated but untrusted Token Error Response URI-reference.
pub struct TokenErrorUri {
    value: Zeroizing<String>,
}

impl TokenErrorUri {
    /// Borrow the exact URI-reference without resolving or dereferencing it.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl fmt::Debug for TokenErrorUri {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TokenErrorUri")
            .finish_non_exhaustive()
    }
}

/// A bounded partial OAuth Token Error Response.
///
/// This state proves only the syntax of known fields. It does not prove HTTP
/// semantics, response provenance, truth, retryability, or display safety.
pub struct TokenErrorResponseCore {
    response_len: usize,
    error: TokenEndpointErrorCode,
    error_description: Option<Zeroizing<String>>,
    error_uri: Option<TokenErrorUri>,
}

impl TokenErrorResponseCore {
    /// Parse a Token Error Response core under explicit resource limits.
    ///
    /// The caller retains responsibility for erasing its input allocation.
    pub fn parse(
        json: &str,
        limits: TokenErrorResponseLimits,
    ) -> Result<Self, CredentialOfferError> {
        if json.len() > limits.max_json_bytes() {
            return Err(CredentialOfferError::TokenErrorResponseTooLarge);
        }
        let fields = parse_token_error_response_fields(json.as_bytes(), limits)?;
        if !is_nqschar(&fields.error) {
            return Err(CredentialOfferError::InvalidTokenEndpointErrorCode);
        }
        if fields
            .error_description
            .as_ref()
            .is_some_and(|description| !is_nqschar(description))
        {
            return Err(CredentialOfferError::InvalidTokenErrorDescription);
        }
        if fields.error_uri.as_ref().is_some_and(|uri| {
            !is_uri_reference_chars(uri) || URIReference::try_from(uri.as_str()).is_err()
        }) {
            return Err(CredentialOfferError::InvalidTokenErrorUri);
        }
        Ok(Self {
            response_len: json.len(),
            error: TokenEndpointErrorCode {
                value: fields.error,
            },
            error_description: fields.error_description,
            error_uri: fields.error_uri.map(|value| TokenErrorUri { value }),
        })
    }

    /// Return the exact response byte count.
    pub const fn response_len(&self) -> usize {
        self.response_len
    }

    /// Borrow the exact validated error code.
    pub const fn error(&self) -> &TokenEndpointErrorCode {
        &self.error
    }

    /// Return the closed classification of the exact error code.
    pub fn error_kind(&self) -> TokenEndpointErrorKind {
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

    /// Report whether a validated but untrusted URI-reference is present.
    pub const fn error_uri_present(&self) -> bool {
        self.error_uri.is_some()
    }

    /// Borrow the validated URI-reference without resolving or following it.
    pub const fn error_uri(&self) -> Option<&TokenErrorUri> {
        self.error_uri.as_ref()
    }
}

impl fmt::Debug for TokenErrorResponseCore {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TokenErrorResponseCore")
            .field("response_bytes", &self.response_len)
            .field("error_kind", &self.error.kind())
            .field(
                "error_description_present",
                &self.error_description.is_some(),
            )
            .field("error_uri_present", &self.error_uri.is_some())
            .finish_non_exhaustive()
    }
}

fn is_nqschar(value: &str) -> bool {
    value
        .bytes()
        .all(|byte| matches!(byte, 0x20..=0x21 | 0x23..=0x5b | 0x5d..=0x7e))
}

fn is_uri_reference_chars(value: &str) -> bool {
    value
        .bytes()
        .all(|byte| matches!(byte, 0x21 | 0x23..=0x5b | 0x5d..=0x7e))
}
