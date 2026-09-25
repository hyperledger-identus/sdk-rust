use std::fmt;

use crate::{
    AuthorizationServerMetadataCore, CredentialConfigurationId, CredentialIssuerMetadata,
    CredentialOfferError, PreAuthorizedTokenHttpResponseLimits, PreAuthorizedTokenRequest,
    TokenErrorResponseCore, TokenResponseCore,
    token_http_response::{TokenHttpHeaderError, validate_token_http_headers},
};

/// Public non-secret lineage retained across one Pre-Authorized Code exchange.
pub struct PreAuthorizedTokenResponseLineage {
    credential_issuer_metadata: CredentialIssuerMetadata,
    authorization_server_metadata: AuthorizationServerMetadataCore,
    offered_configurations: Vec<CredentialConfigurationId>,
    transaction_code_present: bool,
}

impl PreAuthorizedTokenResponseLineage {
    /// Borrow the exact Credential Issuer Metadata matched before exchange.
    pub const fn credential_issuer_metadata(&self) -> &CredentialIssuerMetadata {
        &self.credential_issuer_metadata
    }

    /// Borrow the exact Authorization Server Metadata selected before exchange.
    pub const fn authorization_server_metadata(&self) -> &AuthorizationServerMetadataCore {
        &self.authorization_server_metadata
    }

    /// Borrow the exact ordered Credential Configuration IDs in the matched offer.
    pub fn offered_credential_configurations(&self) -> &[CredentialConfigurationId] {
        &self.offered_configurations
    }

    /// Report whether the consumed request contained a Transaction Code.
    pub const fn transaction_code_present(&self) -> bool {
        self.transaction_code_present
    }
}

impl fmt::Debug for PreAuthorizedTokenResponseLineage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PreAuthorizedTokenResponseLineage")
            .field(
                "offered_configuration_count",
                &self.offered_configurations.len(),
            )
            .field("transaction_code_present", &self.transaction_code_present)
            .finish_non_exhaustive()
    }
}

/// A successful bounded Token Response bound to its pre-authorized request.
pub struct RequestBoundPreAuthorizedTokenResponse {
    lineage: PreAuthorizedTokenResponseLineage,
    response: TokenResponseCore,
}

impl RequestBoundPreAuthorizedTokenResponse {
    /// Borrow the public non-secret request lineage.
    pub const fn lineage(&self) -> &PreAuthorizedTokenResponseLineage {
        &self.lineage
    }

    /// Borrow the bounded successful Token Response core.
    pub const fn response(&self) -> &TokenResponseCore {
        &self.response
    }

    /// Consume this result into its lineage and secret-bearing response core.
    pub fn into_parts(self) -> (PreAuthorizedTokenResponseLineage, TokenResponseCore) {
        (self.lineage, self.response)
    }
}

impl fmt::Debug for RequestBoundPreAuthorizedTokenResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RequestBoundPreAuthorizedTokenResponse")
            .field("lineage", &self.lineage)
            .field("response", &self.response)
            .finish_non_exhaustive()
    }
}

/// A bounded OAuth Token Error Response bound to its pre-authorized request.
pub struct RequestBoundPreAuthorizedTokenErrorResponse {
    lineage: PreAuthorizedTokenResponseLineage,
    response: TokenErrorResponseCore,
}

impl RequestBoundPreAuthorizedTokenErrorResponse {
    /// Borrow the public non-secret request lineage.
    pub const fn lineage(&self) -> &PreAuthorizedTokenResponseLineage {
        &self.lineage
    }

    /// Borrow the bounded OAuth Token Error Response core.
    pub const fn response(&self) -> &TokenErrorResponseCore {
        &self.response
    }

    /// Consume this result into its lineage and bounded error response core.
    pub fn into_parts(self) -> (PreAuthorizedTokenResponseLineage, TokenErrorResponseCore) {
        (self.lineage, self.response)
    }
}

impl fmt::Debug for RequestBoundPreAuthorizedTokenErrorResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RequestBoundPreAuthorizedTokenErrorResponse")
            .field("lineage", &self.lineage)
            .field("response", &self.response)
            .finish_non_exhaustive()
    }
}

/// Exclusive bounded result of one Pre-Authorized Token Endpoint response.
#[derive(Debug)]
pub enum PreAuthorizedTokenResponseOutcome {
    Success(RequestBoundPreAuthorizedTokenResponse),
    Error(RequestBoundPreAuthorizedTokenErrorResponse),
}

impl PreAuthorizedTokenRequest {
    /// Consume this request and bind one caller-supplied final HTTP response.
    ///
    /// The request form is erased before remote response validation. The
    /// caller remains responsible for HTTP execution and provenance, TLS,
    /// redirects, decompression, timeouts, retries, token trust and erasing
    /// its borrowed response allocation.
    pub fn try_bind_response(
        self,
        status_code: u16,
        content_type: &str,
        cache_control: &str,
        pragma: &str,
        body: &str,
        limits: PreAuthorizedTokenHttpResponseLimits,
    ) -> Result<PreAuthorizedTokenResponseOutcome, CredentialOfferError> {
        let (
            credential_issuer_metadata,
            authorization_server_metadata,
            offered_configurations,
            transaction_code_present,
        ) = self.into_response_parts();
        let lineage = PreAuthorizedTokenResponseLineage {
            credential_issuer_metadata,
            authorization_server_metadata,
            offered_configurations,
            transaction_code_present,
        };

        let success = match status_code {
            200 => true,
            400 => false,
            _ => return Err(CredentialOfferError::InvalidPreAuthorizedTokenHttpStatus),
        };
        validate_headers(content_type, cache_control, pragma, limits)?;

        if success {
            let response = TokenResponseCore::parse(body, limits.success_response_limits())?;
            Ok(PreAuthorizedTokenResponseOutcome::Success(
                RequestBoundPreAuthorizedTokenResponse { lineage, response },
            ))
        } else {
            let response = TokenErrorResponseCore::parse(body, limits.error_response_limits())?;
            Ok(PreAuthorizedTokenResponseOutcome::Error(
                RequestBoundPreAuthorizedTokenErrorResponse { lineage, response },
            ))
        }
    }
}

fn validate_headers(
    content_type: &str,
    cache_control: &str,
    pragma: &str,
    limits: PreAuthorizedTokenHttpResponseLimits,
) -> Result<(), CredentialOfferError> {
    validate_token_http_headers(
        content_type,
        cache_control,
        pragma,
        limits.max_content_type_bytes(),
        limits.max_cache_control_bytes(),
        limits.max_pragma_bytes(),
    )
    .map_err(|error| match error {
        TokenHttpHeaderError::ContentTypeTooLarge => {
            CredentialOfferError::PreAuthorizedTokenContentTypeTooLarge
        }
        TokenHttpHeaderError::InvalidContentType => {
            CredentialOfferError::InvalidPreAuthorizedTokenContentType
        }
        TokenHttpHeaderError::CacheControlTooLarge => {
            CredentialOfferError::PreAuthorizedTokenCacheControlTooLarge
        }
        TokenHttpHeaderError::InvalidCacheControl => {
            CredentialOfferError::InvalidPreAuthorizedTokenCacheControl
        }
        TokenHttpHeaderError::PragmaTooLarge => {
            CredentialOfferError::PreAuthorizedTokenPragmaTooLarge
        }
        TokenHttpHeaderError::InvalidPragma => {
            CredentialOfferError::InvalidPreAuthorizedTokenPragma
        }
    })
}
