use std::fmt;

use crate::{
    CredentialOfferError, DeferredCredentialEndpointResponseLimits, DeferredCredentialOutcome,
    RequestBoundDeferredCredentialErrorResponse, RequestBoundDeferredCredentialRequest,
    RequestBoundDeferredCredentialResponse, RequestBoundImmediateCredentialResponse,
    deferred_credential_error_http_response::parse_deferred_error_response,
    deferred_credential_http_response::parse_deferred_success_response,
};

/// Closed classification of an unencrypted Final Deferred Credential Endpoint response.
pub enum DeferredCredentialEndpointResponseOutcome {
    /// The issuer returned one or more credentials (`200`).
    Issued(RequestBoundImmediateCredentialResponse),
    /// The issuer still requires more time (`202`).
    Pending(RequestBoundDeferredCredentialResponse),
    /// The issuer returned a Deferred Credential payload error (`400`).
    Error(RequestBoundDeferredCredentialErrorResponse),
}

impl fmt::Debug for DeferredCredentialEndpointResponseOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Issued(response) => formatter.debug_tuple("Issued").field(response).finish(),
            Self::Pending(response) => formatter.debug_tuple("Pending").field(response).finish(),
            Self::Error(response) => formatter.debug_tuple("Error").field(response).finish(),
        }
    }
}

impl RequestBoundDeferredCredentialRequest {
    /// Consume this request and classify its unencrypted Deferred Credential response.
    ///
    /// Only an exact correlated HTTP 202 retains the request's continuation
    /// authority. Every terminal, unsupported or invalid branch erases that
    /// authority. The caller remains responsible for HTTP execution and origin,
    /// token validity, interval/retry policy, credential processing and storage.
    pub fn try_into_deferred_credential_endpoint_response(
        self,
        status_code: u16,
        content_type: &str,
        body: &str,
        limits: DeferredCredentialEndpointResponseLimits,
    ) -> Result<DeferredCredentialEndpointResponseOutcome, CredentialOfferError> {
        match status_code {
            200 => {
                let request_proof_count = self.request_proof_count();
                drop(self);
                let outcome = parse_deferred_success_response(
                    "",
                    status_code,
                    content_type,
                    body,
                    limits.success_response_limits(),
                )?;
                let DeferredCredentialOutcome::Issued(response) = outcome else {
                    unreachable!("status selected issued response")
                };
                RequestBoundImmediateCredentialResponse::try_from_response(
                    response,
                    request_proof_count,
                )
                .map(DeferredCredentialEndpointResponseOutcome::Issued)
            }
            202 => {
                let (transaction_id, authority) = self.into_response_parts();
                let outcome = parse_deferred_success_response(
                    &transaction_id,
                    status_code,
                    content_type,
                    body,
                    limits.success_response_limits(),
                )?;
                let DeferredCredentialOutcome::Pending(response) = outcome else {
                    unreachable!("status selected pending response")
                };
                Ok(DeferredCredentialEndpointResponseOutcome::Pending(
                    RequestBoundDeferredCredentialResponse::new(response, authority),
                ))
            }
            400 => {
                let request_proof_count = self.request_proof_count();
                drop(self);
                let response = parse_deferred_error_response(
                    status_code,
                    content_type,
                    body,
                    limits.error_response_limits(),
                )?;
                Ok(DeferredCredentialEndpointResponseOutcome::Error(
                    RequestBoundDeferredCredentialErrorResponse::new(response, request_proof_count),
                ))
            }
            _ => {
                drop(self);
                Err(CredentialOfferError::InvalidDeferredCredentialHttpStatus)
            }
        }
    }
}
