use crate::{
    CredentialNonceHttpResponseLimits, CredentialNonceRequest, CredentialNonceResponseCore,
    CredentialOfferError,
    http_field::{has_bare_no_store, is_application_json},
};

impl CredentialNonceRequest {
    /// Validate caller-supplied Final Credential Nonce HTTP response metadata.
    ///
    /// The caller remains responsible for HTTP execution, response origin,
    /// network policy, DPoP handling, and nonce lifecycle policy.
    pub fn validate_response(
        &self,
        status_code: u16,
        content_type: &str,
        cache_control: &str,
        body: &str,
        limits: CredentialNonceHttpResponseLimits,
    ) -> Result<CredentialNonceResponseCore, CredentialOfferError> {
        if !(200..=299).contains(&status_code) {
            return Err(CredentialOfferError::InvalidCredentialNonceHttpStatus);
        }
        if content_type.len() > limits.max_content_type_bytes() {
            return Err(CredentialOfferError::CredentialNonceContentTypeTooLarge);
        }
        if !is_application_json(content_type.as_bytes()) {
            return Err(CredentialOfferError::InvalidCredentialNonceContentType);
        }
        if cache_control.len() > limits.max_cache_control_bytes() {
            return Err(CredentialOfferError::CredentialNonceCacheControlTooLarge);
        }
        if !has_bare_no_store(cache_control.as_bytes()) {
            return Err(CredentialOfferError::InvalidCredentialNonceCacheControl);
        }
        CredentialNonceResponseCore::parse(body, limits.response_limits())
    }
}
