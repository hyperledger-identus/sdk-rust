use std::fmt;

use crate::{
    AuthorizationCodeTokenHttpResponseLimits, AuthorizationCodeTokenRequest,
    AuthorizationResponseIssuerIdentification, AuthorizationServerMetadataCore,
    CredentialConfigurationId, CredentialIssuerMetadata, CredentialOfferError,
    TokenEndpointErrorKind, TokenErrorResponseCore, TokenResponseCore,
    token_http_response::{TokenHttpHeaderError, validate_token_http_headers},
};

/// Public request lineage retained across one Authorization Code token exchange.
pub struct AuthorizationCodeTokenResponseLineage {
    credential_issuer_metadata: CredentialIssuerMetadata,
    authorization_server_metadata: AuthorizationServerMetadataCore,
    selected_configuration: CredentialConfigurationId,
    issuer_identification: AuthorizationResponseIssuerIdentification,
}

impl AuthorizationCodeTokenResponseLineage {
    /// Borrow the validated Credential Issuer Metadata selected before exchange.
    pub const fn credential_issuer_metadata(&self) -> &CredentialIssuerMetadata {
        &self.credential_issuer_metadata
    }

    /// Borrow the validated Authorization Server Metadata selected before exchange.
    pub const fn authorization_server_metadata(&self) -> &AuthorizationServerMetadataCore {
        &self.authorization_server_metadata
    }

    /// Borrow the exact offered Credential Configuration selected before exchange.
    pub const fn selected_credential_configuration(&self) -> &CredentialConfigurationId {
        &self.selected_configuration
    }

    /// Return unchanged Authorization Response issuer-identification evidence.
    pub const fn issuer_identification(&self) -> AuthorizationResponseIssuerIdentification {
        self.issuer_identification
    }
}

impl fmt::Debug for AuthorizationCodeTokenResponseLineage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthorizationCodeTokenResponseLineage")
            .field("issuer_identification", &self.issuer_identification)
            .finish_non_exhaustive()
    }
}

/// A successful bounded Token Response bound to its exact request lineage.
pub struct RequestBoundAuthorizationCodeTokenResponse {
    lineage: AuthorizationCodeTokenResponseLineage,
    response: TokenResponseCore,
}

impl RequestBoundAuthorizationCodeTokenResponse {
    /// Borrow the public request lineage retained across exchange.
    pub const fn lineage(&self) -> &AuthorizationCodeTokenResponseLineage {
        &self.lineage
    }

    /// Borrow the bounded successful Token Response core.
    pub const fn response(&self) -> &TokenResponseCore {
        &self.response
    }

    /// Consume this result into its public lineage and secret-bearing response core.
    pub fn into_parts(self) -> (AuthorizationCodeTokenResponseLineage, TokenResponseCore) {
        (self.lineage, self.response)
    }
}

impl fmt::Debug for RequestBoundAuthorizationCodeTokenResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RequestBoundAuthorizationCodeTokenResponse")
            .field("lineage", &self.lineage)
            .field("response", &self.response)
            .finish_non_exhaustive()
    }
}

/// A bounded OAuth Token Error Response bound to its exact request lineage.
pub struct RequestBoundAuthorizationCodeTokenErrorResponse {
    lineage: AuthorizationCodeTokenResponseLineage,
    response: TokenErrorResponseCore,
}

impl RequestBoundAuthorizationCodeTokenErrorResponse {
    /// Borrow the public request lineage retained across exchange.
    pub const fn lineage(&self) -> &AuthorizationCodeTokenResponseLineage {
        &self.lineage
    }

    /// Borrow the bounded OAuth Token Error Response core.
    pub const fn response(&self) -> &TokenErrorResponseCore {
        &self.response
    }

    /// Consume this result into its public lineage and bounded error response core.
    pub fn into_parts(
        self,
    ) -> (
        AuthorizationCodeTokenResponseLineage,
        TokenErrorResponseCore,
    ) {
        (self.lineage, self.response)
    }
}

impl fmt::Debug for RequestBoundAuthorizationCodeTokenErrorResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RequestBoundAuthorizationCodeTokenErrorResponse")
            .field("lineage", &self.lineage)
            .field("response", &self.response)
            .finish_non_exhaustive()
    }
}

/// Exclusive bounded result of one Authorization Code Token Endpoint response.
#[derive(Debug)]
pub enum AuthorizationCodeTokenResponseOutcome {
    Success(RequestBoundAuthorizationCodeTokenResponse),
    Error(RequestBoundAuthorizationCodeTokenErrorResponse),
}

enum ResponseClass {
    Success,
    Error { unauthorized: bool },
}

impl AuthorizationCodeTokenRequest {
    /// Consume this request and bind one caller-supplied final HTTP response.
    ///
    /// The request form body is erased before remote response validation. The
    /// caller remains responsible for HTTP execution, response origin, TLS,
    /// redirects, decompression, field combination, timeouts, cancellation,
    /// retry policy, token trust and erasing its borrowed input allocation.
    pub fn try_bind_response(
        self,
        status_code: u16,
        content_type: &str,
        cache_control: &str,
        pragma: &str,
        body: &str,
        limits: AuthorizationCodeTokenHttpResponseLimits,
    ) -> Result<AuthorizationCodeTokenResponseOutcome, CredentialOfferError> {
        let (
            credential_issuer_metadata,
            authorization_server_metadata,
            selected_configuration,
            issuer_identification,
        ) = self.into_response_parts();
        let lineage = AuthorizationCodeTokenResponseLineage {
            credential_issuer_metadata,
            authorization_server_metadata,
            selected_configuration,
            issuer_identification,
        };

        let response_class = match status_code {
            200 => ResponseClass::Success,
            400 => ResponseClass::Error {
                unauthorized: false,
            },
            401 => ResponseClass::Error { unauthorized: true },
            _ => return Err(CredentialOfferError::InvalidAuthorizationCodeTokenHttpStatus),
        };
        validate_headers(content_type, cache_control, pragma, limits)?;

        match response_class {
            ResponseClass::Success => {
                let response = TokenResponseCore::parse(body, limits.success_response_limits())?;
                Ok(AuthorizationCodeTokenResponseOutcome::Success(
                    RequestBoundAuthorizationCodeTokenResponse { lineage, response },
                ))
            }
            ResponseClass::Error { unauthorized } => {
                let response = TokenErrorResponseCore::parse(body, limits.error_response_limits())?;
                if unauthorized && response.error_kind() != TokenEndpointErrorKind::InvalidClient {
                    return Err(
                        CredentialOfferError::AuthorizationCodeTokenHttpStatusErrorMismatch,
                    );
                }
                Ok(AuthorizationCodeTokenResponseOutcome::Error(
                    RequestBoundAuthorizationCodeTokenErrorResponse { lineage, response },
                ))
            }
        }
    }
}

fn validate_headers(
    content_type: &str,
    cache_control: &str,
    pragma: &str,
    limits: AuthorizationCodeTokenHttpResponseLimits,
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
            CredentialOfferError::AuthorizationCodeTokenContentTypeTooLarge
        }
        TokenHttpHeaderError::InvalidContentType => {
            CredentialOfferError::InvalidAuthorizationCodeTokenContentType
        }
        TokenHttpHeaderError::CacheControlTooLarge => {
            CredentialOfferError::AuthorizationCodeTokenCacheControlTooLarge
        }
        TokenHttpHeaderError::InvalidCacheControl => {
            CredentialOfferError::InvalidAuthorizationCodeTokenCacheControl
        }
        TokenHttpHeaderError::PragmaTooLarge => {
            CredentialOfferError::AuthorizationCodeTokenPragmaTooLarge
        }
        TokenHttpHeaderError::InvalidPragma => {
            CredentialOfferError::InvalidAuthorizationCodeTokenPragma
        }
    })
}
