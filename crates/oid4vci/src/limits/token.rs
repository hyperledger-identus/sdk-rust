use crate::CredentialOfferError;

use super::{MAX_CONFIGURABLE_JSON_DEPTH, policy};
/// Resource limits for a constructed Pre-Authorized Code Token Request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PreAuthorizedTokenRequestLimits {
    max_form_body_bytes: usize,
}

impl PreAuthorizedTokenRequestLimits {
    /// Construct a positive encoded form-body policy.
    pub const fn new(max_form_body_bytes: usize) -> Result<Self, CredentialOfferError> {
        if max_form_body_bytes == 0 {
            return Err(CredentialOfferError::InvalidPreAuthorizedTokenRequestLimits);
        }
        Ok(Self {
            max_form_body_bytes,
        })
    }

    /// Maximum encoded UTF-8 bytes in the complete form body.
    pub const fn max_form_body_bytes(self) -> usize {
        self.max_form_body_bytes
    }
}

impl Default for PreAuthorizedTokenRequestLimits {
    fn default() -> Self {
        Self {
            max_form_body_bytes: policy::DEFAULT_PRE_AUTHORIZED_TOKEN_REQUEST_MAX_FORM_BYTES,
        }
    }
}

/// Resource limits for a caller-supplied Pre-Authorized Token HTTP response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PreAuthorizedTokenHttpResponseLimits {
    success_response_limits: TokenResponseLimits,
    error_response_limits: TokenErrorResponseLimits,
    max_content_type_bytes: usize,
    max_cache_control_bytes: usize,
    max_pragma_bytes: usize,
}

impl PreAuthorizedTokenHttpResponseLimits {
    /// Combine bounded success/error bodies with positive response-header limits.
    pub const fn new(
        success_response_limits: TokenResponseLimits,
        error_response_limits: TokenErrorResponseLimits,
        max_content_type_bytes: usize,
        max_cache_control_bytes: usize,
        max_pragma_bytes: usize,
    ) -> Result<Self, CredentialOfferError> {
        if max_content_type_bytes == 0 || max_cache_control_bytes == 0 || max_pragma_bytes == 0 {
            return Err(CredentialOfferError::InvalidPreAuthorizedTokenHttpResponseLimits);
        }
        Ok(Self {
            success_response_limits,
            error_response_limits,
            max_content_type_bytes,
            max_cache_control_bytes,
            max_pragma_bytes,
        })
    }

    /// Return the bounded successful Token Response body policy.
    pub const fn success_response_limits(self) -> TokenResponseLimits {
        self.success_response_limits
    }

    /// Return the bounded Token Error Response body policy.
    pub const fn error_response_limits(self) -> TokenErrorResponseLimits {
        self.error_response_limits
    }

    /// Maximum bytes in the effective Content-Type field value.
    pub const fn max_content_type_bytes(self) -> usize {
        self.max_content_type_bytes
    }

    /// Maximum bytes in the effective Cache-Control field value.
    pub const fn max_cache_control_bytes(self) -> usize {
        self.max_cache_control_bytes
    }

    /// Maximum bytes in the effective Pragma field value.
    pub const fn max_pragma_bytes(self) -> usize {
        self.max_pragma_bytes
    }
}

impl Default for PreAuthorizedTokenHttpResponseLimits {
    fn default() -> Self {
        Self {
            success_response_limits: TokenResponseLimits::default(),
            error_response_limits: TokenErrorResponseLimits::default(),
            max_content_type_bytes:
                policy::DEFAULT_PRE_AUTHORIZED_TOKEN_HTTP_MAX_CONTENT_TYPE_BYTES,
            max_cache_control_bytes:
                policy::DEFAULT_PRE_AUTHORIZED_TOKEN_HTTP_MAX_CACHE_CONTROL_BYTES,
            max_pragma_bytes: policy::DEFAULT_PRE_AUTHORIZED_TOKEN_HTTP_MAX_PRAGMA_BYTES,
        }
    }
}

/// Resource limits for a constructed Authorization Code Token Request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthorizationCodeTokenRequestLimits {
    max_token_endpoint_bytes: usize,
    max_form_body_bytes: usize,
}

impl AuthorizationCodeTokenRequestLimits {
    /// Construct a positive endpoint and encoded form-body policy.
    pub const fn new(
        max_token_endpoint_bytes: usize,
        max_form_body_bytes: usize,
    ) -> Result<Self, CredentialOfferError> {
        if max_token_endpoint_bytes == 0 || max_form_body_bytes == 0 {
            return Err(CredentialOfferError::InvalidAuthorizationCodeTokenRequestLimits);
        }
        Ok(Self {
            max_token_endpoint_bytes,
            max_form_body_bytes,
        })
    }

    /// Maximum bytes in the selected Token Endpoint URL.
    pub const fn max_token_endpoint_bytes(self) -> usize {
        self.max_token_endpoint_bytes
    }

    /// Maximum encoded UTF-8 bytes in the complete form body.
    pub const fn max_form_body_bytes(self) -> usize {
        self.max_form_body_bytes
    }
}

impl Default for AuthorizationCodeTokenRequestLimits {
    fn default() -> Self {
        Self {
            max_token_endpoint_bytes:
                policy::DEFAULT_AUTHORIZATION_CODE_TOKEN_REQUEST_MAX_ENDPOINT_BYTES,
            max_form_body_bytes: policy::DEFAULT_AUTHORIZATION_CODE_TOKEN_REQUEST_MAX_FORM_BYTES,
        }
    }
}

/// Resource limits for a caller-supplied Authorization Code Token HTTP response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthorizationCodeTokenHttpResponseLimits {
    success_response_limits: TokenResponseLimits,
    error_response_limits: TokenErrorResponseLimits,
    max_content_type_bytes: usize,
    max_cache_control_bytes: usize,
    max_pragma_bytes: usize,
}

impl AuthorizationCodeTokenHttpResponseLimits {
    /// Combine bounded success/error bodies with positive response-header limits.
    pub const fn new(
        success_response_limits: TokenResponseLimits,
        error_response_limits: TokenErrorResponseLimits,
        max_content_type_bytes: usize,
        max_cache_control_bytes: usize,
        max_pragma_bytes: usize,
    ) -> Result<Self, CredentialOfferError> {
        if max_content_type_bytes == 0 || max_cache_control_bytes == 0 || max_pragma_bytes == 0 {
            return Err(CredentialOfferError::InvalidAuthorizationCodeTokenHttpResponseLimits);
        }
        Ok(Self {
            success_response_limits,
            error_response_limits,
            max_content_type_bytes,
            max_cache_control_bytes,
            max_pragma_bytes,
        })
    }

    /// Return the bounded successful Token Response body policy.
    pub const fn success_response_limits(self) -> TokenResponseLimits {
        self.success_response_limits
    }

    /// Return the bounded Token Error Response body policy.
    pub const fn error_response_limits(self) -> TokenErrorResponseLimits {
        self.error_response_limits
    }

    /// Maximum bytes in the effective Content-Type field value.
    pub const fn max_content_type_bytes(self) -> usize {
        self.max_content_type_bytes
    }

    /// Maximum bytes in the effective Cache-Control field value.
    pub const fn max_cache_control_bytes(self) -> usize {
        self.max_cache_control_bytes
    }

    /// Maximum bytes in the effective Pragma field value.
    pub const fn max_pragma_bytes(self) -> usize {
        self.max_pragma_bytes
    }
}

impl Default for AuthorizationCodeTokenHttpResponseLimits {
    fn default() -> Self {
        Self {
            success_response_limits: TokenResponseLimits::default(),
            error_response_limits: TokenErrorResponseLimits::default(),
            max_content_type_bytes:
                policy::DEFAULT_AUTHORIZATION_CODE_TOKEN_HTTP_MAX_CONTENT_TYPE_BYTES,
            max_cache_control_bytes:
                policy::DEFAULT_AUTHORIZATION_CODE_TOKEN_HTTP_MAX_CACHE_CONTROL_BYTES,
            max_pragma_bytes: policy::DEFAULT_AUTHORIZATION_CODE_TOKEN_HTTP_MAX_PRAGMA_BYTES,
        }
    }
}

/// Resource limits for a successful OAuth Token Response core.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenResponseLimits {
    max_json_bytes: usize,
    max_json_depth: usize,
    max_json_nodes: usize,
    max_access_token_bytes: usize,
    max_token_type_bytes: usize,
    max_refresh_token_bytes: usize,
    max_scope_bytes: usize,
}

impl TokenResponseLimits {
    /// Construct a positive response policy with a supported JSON depth.
    pub const fn new(
        max_json_bytes: usize,
        max_json_depth: usize,
        max_json_nodes: usize,
        max_access_token_bytes: usize,
        max_token_type_bytes: usize,
        max_refresh_token_bytes: usize,
        max_scope_bytes: usize,
    ) -> Result<Self, CredentialOfferError> {
        if max_json_bytes == 0
            || max_json_depth == 0
            || max_json_depth > MAX_CONFIGURABLE_JSON_DEPTH
            || max_json_nodes == 0
            || max_access_token_bytes == 0
            || max_token_type_bytes == 0
            || max_refresh_token_bytes == 0
            || max_scope_bytes == 0
        {
            return Err(CredentialOfferError::InvalidTokenResponseLimits);
        }
        Ok(Self {
            max_json_bytes,
            max_json_depth,
            max_json_nodes,
            max_access_token_bytes,
            max_token_type_bytes,
            max_refresh_token_bytes,
            max_scope_bytes,
        })
    }

    /// Maximum bytes in the complete JSON response.
    pub const fn max_json_bytes(self) -> usize {
        self.max_json_bytes
    }

    /// Maximum JSON container depth.
    pub const fn max_json_depth(self) -> usize {
        self.max_json_depth
    }

    /// Maximum aggregate JSON value nodes.
    pub const fn max_json_nodes(self) -> usize {
        self.max_json_nodes
    }

    /// Maximum decoded access-token bytes.
    pub const fn max_access_token_bytes(self) -> usize {
        self.max_access_token_bytes
    }

    /// Maximum decoded token-type bytes.
    pub const fn max_token_type_bytes(self) -> usize {
        self.max_token_type_bytes
    }

    /// Maximum decoded refresh-token bytes.
    pub const fn max_refresh_token_bytes(self) -> usize {
        self.max_refresh_token_bytes
    }

    /// Maximum decoded scope bytes.
    pub const fn max_scope_bytes(self) -> usize {
        self.max_scope_bytes
    }
}

impl Default for TokenResponseLimits {
    fn default() -> Self {
        Self {
            max_json_bytes: policy::DEFAULT_TOKEN_RESPONSE_MAX_JSON_BYTES,
            max_json_depth: policy::DEFAULT_TOKEN_RESPONSE_MAX_JSON_DEPTH,
            max_json_nodes: policy::DEFAULT_TOKEN_RESPONSE_MAX_JSON_NODES,
            max_access_token_bytes: policy::DEFAULT_TOKEN_RESPONSE_MAX_ACCESS_TOKEN_BYTES,
            max_token_type_bytes: policy::DEFAULT_TOKEN_RESPONSE_MAX_TOKEN_TYPE_BYTES,
            max_refresh_token_bytes: policy::DEFAULT_TOKEN_RESPONSE_MAX_REFRESH_TOKEN_BYTES,
            max_scope_bytes: policy::DEFAULT_TOKEN_RESPONSE_MAX_SCOPE_BYTES,
        }
    }
}

/// Resource limits for validating Token Response Authorization Details.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenAuthorizationDetailsLimits {
    max_authorization_details: usize,
    max_type_bytes: usize,
    max_credential_configuration_id_bytes: usize,
    max_credential_identifiers_per_detail: usize,
    max_credential_identifier_bytes: usize,
}

impl TokenAuthorizationDetailsLimits {
    /// Construct a positive Authorization Details resource policy.
    pub const fn new(
        max_authorization_details: usize,
        max_type_bytes: usize,
        max_credential_configuration_id_bytes: usize,
        max_credential_identifiers_per_detail: usize,
        max_credential_identifier_bytes: usize,
    ) -> Result<Self, CredentialOfferError> {
        if max_authorization_details == 0
            || max_type_bytes == 0
            || max_credential_configuration_id_bytes == 0
            || max_credential_identifiers_per_detail == 0
            || max_credential_identifier_bytes == 0
        {
            return Err(CredentialOfferError::InvalidTokenAuthorizationDetailsLimits);
        }
        Ok(Self {
            max_authorization_details,
            max_type_bytes,
            max_credential_configuration_id_bytes,
            max_credential_identifiers_per_detail,
            max_credential_identifier_bytes,
        })
    }

    /// Maximum number of entries in the Authorization Details array.
    pub const fn max_authorization_details(self) -> usize {
        self.max_authorization_details
    }

    /// Maximum decoded bytes in one authorization-detail type.
    pub const fn max_type_bytes(self) -> usize {
        self.max_type_bytes
    }

    /// Maximum decoded bytes in one Credential Configuration ID.
    pub const fn max_credential_configuration_id_bytes(self) -> usize {
        self.max_credential_configuration_id_bytes
    }

    /// Maximum Credential Dataset identifiers in one recognized entry.
    pub const fn max_credential_identifiers_per_detail(self) -> usize {
        self.max_credential_identifiers_per_detail
    }

    /// Maximum decoded bytes in one Credential Dataset identifier.
    pub const fn max_credential_identifier_bytes(self) -> usize {
        self.max_credential_identifier_bytes
    }
}

impl Default for TokenAuthorizationDetailsLimits {
    fn default() -> Self {
        Self {
            max_authorization_details: policy::DEFAULT_TOKEN_AUTHORIZATION_MAX_DETAILS,
            max_type_bytes: policy::DEFAULT_TOKEN_AUTHORIZATION_MAX_TYPE_BYTES,
            max_credential_configuration_id_bytes:
                policy::DEFAULT_TOKEN_AUTHORIZATION_MAX_CONFIGURATION_ID_BYTES,
            max_credential_identifiers_per_detail:
                policy::DEFAULT_TOKEN_AUTHORIZATION_MAX_IDENTIFIERS_PER_DETAIL,
            max_credential_identifier_bytes:
                policy::DEFAULT_TOKEN_AUTHORIZATION_MAX_IDENTIFIER_BYTES,
        }
    }
}

/// Resource limits for an OAuth Token Error Response core.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenErrorResponseLimits {
    max_json_bytes: usize,
    max_json_depth: usize,
    max_json_nodes: usize,
    max_error_code_bytes: usize,
    max_error_description_bytes: usize,
    max_error_uri_bytes: usize,
}

impl TokenErrorResponseLimits {
    /// Construct a positive error-response policy with a supported JSON depth.
    pub const fn new(
        max_json_bytes: usize,
        max_json_depth: usize,
        max_json_nodes: usize,
        max_error_code_bytes: usize,
        max_error_description_bytes: usize,
        max_error_uri_bytes: usize,
    ) -> Result<Self, CredentialOfferError> {
        if max_json_bytes == 0
            || max_json_depth == 0
            || max_json_depth > MAX_CONFIGURABLE_JSON_DEPTH
            || max_json_nodes == 0
            || max_error_code_bytes == 0
            || max_error_description_bytes == 0
            || max_error_uri_bytes == 0
        {
            return Err(CredentialOfferError::InvalidTokenErrorResponseLimits);
        }
        Ok(Self {
            max_json_bytes,
            max_json_depth,
            max_json_nodes,
            max_error_code_bytes,
            max_error_description_bytes,
            max_error_uri_bytes,
        })
    }

    /// Maximum bytes in the complete JSON response.
    pub const fn max_json_bytes(self) -> usize {
        self.max_json_bytes
    }

    /// Maximum JSON container depth.
    pub const fn max_json_depth(self) -> usize {
        self.max_json_depth
    }

    /// Maximum aggregate JSON value nodes.
    pub const fn max_json_nodes(self) -> usize {
        self.max_json_nodes
    }

    /// Maximum decoded bytes in the error code.
    pub const fn max_error_code_bytes(self) -> usize {
        self.max_error_code_bytes
    }

    /// Maximum decoded bytes in the optional developer description.
    pub const fn max_error_description_bytes(self) -> usize {
        self.max_error_description_bytes
    }

    /// Maximum decoded bytes in the optional URI-reference.
    pub const fn max_error_uri_bytes(self) -> usize {
        self.max_error_uri_bytes
    }
}

impl Default for TokenErrorResponseLimits {
    fn default() -> Self {
        Self {
            max_json_bytes: policy::DEFAULT_TOKEN_ERROR_MAX_JSON_BYTES,
            max_json_depth: policy::DEFAULT_TOKEN_ERROR_MAX_JSON_DEPTH,
            max_json_nodes: policy::DEFAULT_TOKEN_ERROR_MAX_JSON_NODES,
            max_error_code_bytes: policy::DEFAULT_TOKEN_ERROR_MAX_CODE_BYTES,
            max_error_description_bytes: policy::DEFAULT_TOKEN_ERROR_MAX_DESCRIPTION_BYTES,
            max_error_uri_bytes: policy::DEFAULT_TOKEN_ERROR_MAX_URI_BYTES,
        }
    }
}
