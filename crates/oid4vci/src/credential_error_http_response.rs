use crate::{
    CredentialErrorHttpResponseLimits, CredentialErrorResponseCore, CredentialOfferError,
    http_field::is_application_json,
};

impl CredentialErrorResponseCore {
    /// Validate a caller-supplied Final Credential payload-error HTTP response.
    ///
    /// The caller remains responsible for HTTP execution, response origin,
    /// request correlation, authorization errors, and recovery policy.
    pub fn parse_http_response(
        status_code: u16,
        content_type: &str,
        body: &str,
        limits: CredentialErrorHttpResponseLimits,
    ) -> Result<Self, CredentialOfferError> {
        if status_code != 400 {
            return Err(CredentialOfferError::InvalidCredentialErrorHttpStatus);
        }
        if content_type.len() > limits.max_content_type_bytes() {
            return Err(CredentialOfferError::CredentialErrorContentTypeTooLarge);
        }
        if !is_application_json(content_type.as_bytes()) {
            return Err(CredentialOfferError::InvalidCredentialErrorContentType);
        }

        let response = Self::parse(body, limits.response_limits())?;
        if response.error().as_str() == "invalid_request" {
            return Err(CredentialOfferError::GenericCredentialErrorCodeForbidden);
        }
        Ok(response)
    }
}
