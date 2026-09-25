use std::fmt;

use crate::{
    CredentialEndpointErrorKind, CredentialErrorHttpResponseLimits, CredentialErrorResponseCore,
    CredentialOfferError, DeferredCredentialRequest,
};

/// Deferred Credential Endpoint classification of a bounded payload error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeferredCredentialErrorKind {
    /// The transaction identifier is unknown to the issuer or already used.
    InvalidTransactionId,
    /// The issuer can no longer issue the requested credential.
    CredentialRequestDenied,
    /// Another Credential Request payload error inherited from section 8.3.1.
    Inherited(CredentialEndpointErrorKind),
}

/// A bounded unencrypted Final Deferred Credential payload-error response.
///
/// This value proves only the syntax and endpoint-specific classification of
/// caller-supplied response data. It does not prove response origin,
/// transaction correlation, truth, authorization, retry safety, terminal
/// state, or that any lifecycle action occurred.
pub struct DeferredCredentialErrorResponse {
    core: CredentialErrorResponseCore,
}

/// A terminal Deferred Credential payload error bound to its request proof count.
pub struct RequestBoundDeferredCredentialErrorResponse {
    response: DeferredCredentialErrorResponse,
    request_proof_count: usize,
}

impl RequestBoundDeferredCredentialErrorResponse {
    pub(crate) const fn new(
        response: DeferredCredentialErrorResponse,
        request_proof_count: usize,
    ) -> Self {
        Self {
            response,
            request_proof_count,
        }
    }

    /// Return the originating Credential Request JWT proof count.
    pub const fn request_proof_count(&self) -> usize {
        self.request_proof_count
    }

    /// Borrow the bounded deferred payload-error response.
    pub const fn response(&self) -> &DeferredCredentialErrorResponse {
        &self.response
    }

    /// Consume this binding into the bounded deferred payload-error response.
    pub fn into_response(self) -> DeferredCredentialErrorResponse {
        self.response
    }
}

impl fmt::Debug for RequestBoundDeferredCredentialErrorResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RequestBoundDeferredCredentialErrorResponse")
            .field("request_proof_count", &self.request_proof_count)
            .field("response", &self.response)
            .finish_non_exhaustive()
    }
}

impl DeferredCredentialErrorResponse {
    /// Return the deferred endpoint classification of the exact error code.
    pub fn kind(&self) -> DeferredCredentialErrorKind {
        match self.core.error().as_str() {
            "invalid_transaction_id" => DeferredCredentialErrorKind::InvalidTransactionId,
            "credential_request_denied" => DeferredCredentialErrorKind::CredentialRequestDenied,
            _ => DeferredCredentialErrorKind::Inherited(self.core.error_kind()),
        }
    }

    /// Borrow the bounded inherited Credential Error Response core.
    pub const fn core(&self) -> &CredentialErrorResponseCore {
        &self.core
    }

    /// Report the Final section 9.3 guidance to stop polling this transaction.
    ///
    /// This method returns `true` only for exact `credential_request_denied`.
    /// It performs no timer, retry, cancellation, invalidation, deletion,
    /// storage, or other lifecycle operation.
    pub fn should_stop_polling(&self) -> bool {
        matches!(
            self.kind(),
            DeferredCredentialErrorKind::CredentialRequestDenied
        )
    }
}

impl fmt::Debug for DeferredCredentialErrorResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DeferredCredentialErrorResponse")
            .field("kind", &self.kind())
            .field("core", &self.core)
            .finish_non_exhaustive()
    }
}

impl DeferredCredentialRequest {
    /// Validate an unencrypted Final Deferred Credential payload-error response.
    ///
    /// The caller remains responsible for HTTP execution and origin,
    /// authorization challenges, token handling, retry and invalidation policy,
    /// trust, localization, display safety, and every lifecycle side effect.
    pub fn validate_error_response(
        &self,
        status_code: u16,
        content_type: &str,
        body: &str,
        limits: CredentialErrorHttpResponseLimits,
    ) -> Result<DeferredCredentialErrorResponse, CredentialOfferError> {
        parse_deferred_error_response(status_code, content_type, body, limits)
    }
}

pub(crate) fn parse_deferred_error_response(
    status_code: u16,
    content_type: &str,
    body: &str,
    limits: CredentialErrorHttpResponseLimits,
) -> Result<DeferredCredentialErrorResponse, CredentialOfferError> {
    let core =
        CredentialErrorResponseCore::parse_http_response(status_code, content_type, body, limits)?;
    Ok(DeferredCredentialErrorResponse { core })
}
