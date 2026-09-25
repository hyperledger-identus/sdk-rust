use std::fmt;

use crate::{
    CredentialEndpointResponseLimits, CredentialErrorResponseCore, CredentialOfferError,
    DeferredCredentialResponseCore, JwtCredentialRequest, RequestBoundImmediateCredentialResponse,
    http_field::is_application_json, immediate_credential_http_response::bind_immediate_response,
};

/// A deferred Credential Response bound to the originating request proof count.
pub struct RequestBoundDeferredCredentialResponse {
    response: DeferredCredentialResponseCore,
    request_proof_count: usize,
}

impl RequestBoundDeferredCredentialResponse {
    /// Return the originating request's JWT proof count.
    pub const fn request_proof_count(&self) -> usize {
        self.request_proof_count
    }

    /// Borrow the bounded deferred response core.
    pub const fn response(&self) -> &DeferredCredentialResponseCore {
        &self.response
    }

    /// Consume this binding into the bounded deferred response core.
    pub fn into_response(self) -> DeferredCredentialResponseCore {
        self.response
    }
}

impl fmt::Debug for RequestBoundDeferredCredentialResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RequestBoundDeferredCredentialResponse")
            .field("request_proof_count", &self.request_proof_count)
            .field("response", &self.response)
            .finish_non_exhaustive()
    }
}

/// A Credential Endpoint payload-error response bound to its request proof count.
pub struct RequestBoundCredentialErrorResponse {
    response: CredentialErrorResponseCore,
    request_proof_count: usize,
}

impl RequestBoundCredentialErrorResponse {
    /// Return the originating request's JWT proof count.
    pub const fn request_proof_count(&self) -> usize {
        self.request_proof_count
    }

    /// Borrow the bounded payload-error response core.
    pub const fn response(&self) -> &CredentialErrorResponseCore {
        &self.response
    }

    /// Consume this binding into the bounded payload-error response core.
    pub fn into_response(self) -> CredentialErrorResponseCore {
        self.response
    }
}

impl fmt::Debug for RequestBoundCredentialErrorResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RequestBoundCredentialErrorResponse")
            .field("request_proof_count", &self.request_proof_count)
            .field("response", &self.response)
            .finish_non_exhaustive()
    }
}

/// Closed classification of an unencrypted Final Credential Endpoint response.
pub enum CredentialEndpointResponseOutcome {
    /// The issuer returned one or more credentials immediately (`200`).
    Issued(RequestBoundImmediateCredentialResponse),
    /// The issuer accepted issuance for later completion (`202`).
    Deferred(RequestBoundDeferredCredentialResponse),
    /// The issuer returned a Credential Request payload error (`400`).
    Error(RequestBoundCredentialErrorResponse),
}

impl fmt::Debug for CredentialEndpointResponseOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Issued(response) => formatter.debug_tuple("Issued").field(response).finish(),
            Self::Deferred(response) => formatter.debug_tuple("Deferred").field(response).finish(),
            Self::Error(response) => formatter.debug_tuple("Error").field(response).finish(),
        }
    }
}

impl JwtCredentialRequest {
    /// Consume this request and classify its unencrypted Credential Endpoint response.
    ///
    /// The request's bearer credential and proof body are dropped before any
    /// untrusted response field is parsed. The caller remains responsible for
    /// HTTP execution, origin binding, encrypted responses, authorization
    /// errors, retry or polling policy, credential verification, and storage.
    pub fn try_into_credential_endpoint_response(
        self,
        status_code: u16,
        content_type: &str,
        body: &str,
        limits: CredentialEndpointResponseLimits,
    ) -> Result<CredentialEndpointResponseOutcome, CredentialOfferError> {
        let request_proof_count = self.proof_count();
        drop(self);

        match status_code {
            200 => bind_immediate_response(
                request_proof_count,
                status_code,
                content_type,
                body,
                limits.immediate_response_limits(),
            )
            .map(CredentialEndpointResponseOutcome::Issued),
            202 => {
                let deferred_limits = limits.deferred_response_limits();
                if content_type.len() > deferred_limits.max_content_type_bytes() {
                    return Err(CredentialOfferError::DeferredCredentialContentTypeTooLarge);
                }
                if !is_application_json(content_type.as_bytes()) {
                    return Err(CredentialOfferError::InvalidDeferredCredentialContentType);
                }
                let response = DeferredCredentialResponseCore::parse(
                    body,
                    deferred_limits.deferred_response_limits(),
                )?;
                Ok(CredentialEndpointResponseOutcome::Deferred(
                    RequestBoundDeferredCredentialResponse {
                        response,
                        request_proof_count,
                    },
                ))
            }
            400 => CredentialErrorResponseCore::parse_http_response(
                status_code,
                content_type,
                body,
                limits.error_response_limits(),
            )
            .map(|response| {
                CredentialEndpointResponseOutcome::Error(RequestBoundCredentialErrorResponse {
                    response,
                    request_proof_count,
                })
            }),
            _ => Err(CredentialOfferError::InvalidCredentialEndpointHttpStatus),
        }
    }
}
