use std::fmt;

use crate::{
    CredentialOfferError, ImmediateCredentialHttpResponseLimits, ImmediateCredentialResponseCore,
    JwtCredentialRequest, http_field::is_application_json,
};

/// An immediate Credential Response with its necessary request-count bound.
///
/// This state does not prove HTTP execution or origin, proof-key uniqueness,
/// credential-to-key binding, credential validity, trust, or storage safety.
pub struct RequestBoundImmediateCredentialResponse {
    response: ImmediateCredentialResponseCore,
    request_proof_count: usize,
}

impl RequestBoundImmediateCredentialResponse {
    /// Return the originating request's JWT proof count.
    pub const fn request_proof_count(&self) -> usize {
        self.request_proof_count
    }

    /// Borrow the bounded immediate response core.
    pub const fn response(&self) -> &ImmediateCredentialResponseCore {
        &self.response
    }

    /// Consume this binding into the bounded immediate response core.
    pub fn into_response(self) -> ImmediateCredentialResponseCore {
        self.response
    }
}

impl fmt::Debug for RequestBoundImmediateCredentialResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RequestBoundImmediateCredentialResponse")
            .field("request_proof_count", &self.request_proof_count)
            .field("response", &self.response)
            .finish_non_exhaustive()
    }
}

impl JwtCredentialRequest {
    /// Validate an unencrypted Final immediate Credential HTTP response.
    ///
    /// The caller remains responsible for HTTP execution, response origin,
    /// network and cache policy, DPoP, retry/replay behavior, proof-key
    /// uniqueness, credential verification, trust, notifications, and storage.
    pub fn validate_immediate_response(
        &self,
        status_code: u16,
        content_type: &str,
        body: &str,
        limits: ImmediateCredentialHttpResponseLimits,
    ) -> Result<RequestBoundImmediateCredentialResponse, CredentialOfferError> {
        if status_code == 202 {
            return Err(CredentialOfferError::DeferredCredentialResponseUnsupported);
        }
        if status_code != 200 {
            return Err(CredentialOfferError::InvalidImmediateCredentialHttpStatus);
        }
        if content_type.len() > limits.max_content_type_bytes() {
            return Err(CredentialOfferError::ImmediateCredentialContentTypeTooLarge);
        }
        if !is_application_json(content_type.as_bytes()) {
            return Err(CredentialOfferError::InvalidImmediateCredentialContentType);
        }

        let response = ImmediateCredentialResponseCore::parse(body, limits.response_limits())?;
        if response.credentials().len() > self.proof_count() {
            return Err(CredentialOfferError::CredentialResponseExceedsProofCount);
        }

        Ok(RequestBoundImmediateCredentialResponse {
            response,
            request_proof_count: self.proof_count(),
        })
    }
}
