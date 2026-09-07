use crate::CredentialOfferError;

/// Highest supported caller-configured JSON container depth.
///
/// This stays below the underlying JSON parser's recursion limit so every
/// accepted policy can be enforced by the SDK's own deterministic boundary.
pub const MAX_CONFIGURABLE_JSON_DEPTH: usize = 64;

/// Resource limits for Credential Offer transport parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CredentialOfferLimits {
    max_invocation_bytes: usize,
    max_embedded_json_bytes: usize,
    max_reference_uri_bytes: usize,
    max_json_depth: usize,
    max_json_nodes: usize,
}

impl CredentialOfferLimits {
    /// Construct a positive resource policy with a supported JSON depth.
    pub const fn new(
        max_invocation_bytes: usize,
        max_embedded_json_bytes: usize,
        max_reference_uri_bytes: usize,
        max_json_depth: usize,
        max_json_nodes: usize,
    ) -> Result<Self, CredentialOfferError> {
        if max_invocation_bytes == 0
            || max_embedded_json_bytes == 0
            || max_reference_uri_bytes == 0
            || max_json_depth == 0
            || max_json_depth > MAX_CONFIGURABLE_JSON_DEPTH
            || max_json_nodes == 0
        {
            return Err(CredentialOfferError::InvalidLimits);
        }
        Ok(Self {
            max_invocation_bytes,
            max_embedded_json_bytes,
            max_reference_uri_bytes,
            max_json_depth,
            max_json_nodes,
        })
    }

    /// Maximum bytes in the complete invocation.
    pub const fn max_invocation_bytes(self) -> usize {
        self.max_invocation_bytes
    }

    /// Maximum bytes in decoded embedded JSON.
    pub const fn max_embedded_json_bytes(self) -> usize {
        self.max_embedded_json_bytes
    }

    /// Maximum bytes in a decoded reference URI.
    pub const fn max_reference_uri_bytes(self) -> usize {
        self.max_reference_uri_bytes
    }

    /// Maximum JSON container depth.
    pub const fn max_json_depth(self) -> usize {
        self.max_json_depth
    }

    /// Maximum aggregate JSON value nodes.
    pub const fn max_json_nodes(self) -> usize {
        self.max_json_nodes
    }
}

impl Default for CredentialOfferLimits {
    fn default() -> Self {
        Self {
            max_invocation_bytes: 32_768,
            max_embedded_json_bytes: 16_384,
            max_reference_uri_bytes: 2_048,
            max_json_depth: 16,
            max_json_nodes: 128,
        }
    }
}

/// Resource limits for semantic Credential Offer fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CredentialOfferSemanticLimits {
    max_credential_issuer_bytes: usize,
    max_credential_configuration_id_bytes: usize,
    max_credential_configuration_ids: usize,
}

impl CredentialOfferSemanticLimits {
    /// Construct a positive semantic resource policy.
    pub const fn new(
        max_credential_issuer_bytes: usize,
        max_credential_configuration_id_bytes: usize,
        max_credential_configuration_ids: usize,
    ) -> Result<Self, CredentialOfferError> {
        if max_credential_issuer_bytes == 0
            || max_credential_configuration_id_bytes == 0
            || max_credential_configuration_ids == 0
        {
            return Err(CredentialOfferError::InvalidSemanticLimits);
        }
        Ok(Self {
            max_credential_issuer_bytes,
            max_credential_configuration_id_bytes,
            max_credential_configuration_ids,
        })
    }

    /// Maximum decoded UTF-8 bytes in the Credential Issuer Identifier.
    pub const fn max_credential_issuer_bytes(self) -> usize {
        self.max_credential_issuer_bytes
    }

    /// Maximum decoded UTF-8 bytes in one Credential Configuration ID.
    pub const fn max_credential_configuration_id_bytes(self) -> usize {
        self.max_credential_configuration_id_bytes
    }

    /// Maximum number of Credential Configuration IDs in one offer.
    pub const fn max_credential_configuration_ids(self) -> usize {
        self.max_credential_configuration_ids
    }
}

impl Default for CredentialOfferSemanticLimits {
    fn default() -> Self {
        Self {
            max_credential_issuer_bytes: 2_048,
            max_credential_configuration_id_bytes: 256,
            max_credential_configuration_ids: 32,
        }
    }
}

/// Resource limits for known Credential Offer grant members.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CredentialOfferGrantLimits {
    max_issuer_state_bytes: usize,
    max_pre_authorized_code_bytes: usize,
    max_authorization_server_bytes: usize,
    max_transaction_code_description_bytes: usize,
    max_transaction_code_length: usize,
}

impl CredentialOfferGrantLimits {
    /// Construct a positive grant resource policy.
    pub const fn new(
        max_issuer_state_bytes: usize,
        max_pre_authorized_code_bytes: usize,
        max_authorization_server_bytes: usize,
        max_transaction_code_description_bytes: usize,
        max_transaction_code_length: usize,
    ) -> Result<Self, CredentialOfferError> {
        if max_issuer_state_bytes == 0
            || max_pre_authorized_code_bytes == 0
            || max_authorization_server_bytes == 0
            || max_transaction_code_description_bytes == 0
            || max_transaction_code_length == 0
        {
            return Err(CredentialOfferError::InvalidGrantLimits);
        }
        Ok(Self {
            max_issuer_state_bytes,
            max_pre_authorized_code_bytes,
            max_authorization_server_bytes,
            max_transaction_code_description_bytes,
            max_transaction_code_length,
        })
    }

    /// Maximum decoded UTF-8 bytes in an opaque issuer state.
    pub const fn max_issuer_state_bytes(self) -> usize {
        self.max_issuer_state_bytes
    }

    /// Maximum decoded UTF-8 bytes in a Pre-Authorized Code.
    pub const fn max_pre_authorized_code_bytes(self) -> usize {
        self.max_pre_authorized_code_bytes
    }

    /// Maximum decoded UTF-8 bytes in an Authorization Server identifier.
    pub const fn max_authorization_server_bytes(self) -> usize {
        self.max_authorization_server_bytes
    }

    /// Maximum decoded UTF-8 bytes in Transaction Code guidance.
    pub const fn max_transaction_code_description_bytes(self) -> usize {
        self.max_transaction_code_description_bytes
    }

    /// Maximum advertised Transaction Code length.
    pub const fn max_transaction_code_length(self) -> usize {
        self.max_transaction_code_length
    }
}

impl Default for CredentialOfferGrantLimits {
    fn default() -> Self {
        Self {
            max_issuer_state_bytes: 2_048,
            max_pre_authorized_code_bytes: 4_096,
            max_authorization_server_bytes: 2_048,
            max_transaction_code_description_bytes: 1_200,
            max_transaction_code_length: 64,
        }
    }
}

/// Resource limits for caller-owned Transaction Code input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransactionCodeInputLimits {
    max_transaction_code_bytes: usize,
}

impl TransactionCodeInputLimits {
    /// Construct a positive Transaction Code input policy.
    pub const fn new(max_transaction_code_bytes: usize) -> Result<Self, CredentialOfferError> {
        if max_transaction_code_bytes == 0 {
            return Err(CredentialOfferError::InvalidTransactionCodeInputLimits);
        }
        Ok(Self {
            max_transaction_code_bytes,
        })
    }

    /// Maximum decoded UTF-8 bytes in caller-owned Transaction Code input.
    pub const fn max_transaction_code_bytes(self) -> usize {
        self.max_transaction_code_bytes
    }
}

impl Default for TransactionCodeInputLimits {
    fn default() -> Self {
        Self {
            max_transaction_code_bytes: 256,
        }
    }
}

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
            max_form_body_bytes: 16_384,
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
            max_json_bytes: 65_536,
            max_json_depth: 16,
            max_json_nodes: 1_024,
            max_access_token_bytes: 16_384,
            max_token_type_bytes: 256,
            max_refresh_token_bytes: 16_384,
            max_scope_bytes: 4_096,
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
            max_json_bytes: 32_768,
            max_json_depth: 16,
            max_json_nodes: 512,
            max_error_code_bytes: 256,
            max_error_description_bytes: 4_096,
            max_error_uri_bytes: 2_048,
        }
    }
}

/// Resource limits for an OID4VCI Credential Error Response core.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CredentialErrorResponseLimits {
    max_json_bytes: usize,
    max_json_depth: usize,
    max_json_nodes: usize,
    max_error_code_bytes: usize,
    max_error_description_bytes: usize,
}

impl CredentialErrorResponseLimits {
    /// Construct a positive error-response policy with a supported JSON depth.
    pub const fn new(
        max_json_bytes: usize,
        max_json_depth: usize,
        max_json_nodes: usize,
        max_error_code_bytes: usize,
        max_error_description_bytes: usize,
    ) -> Result<Self, CredentialOfferError> {
        if max_json_bytes == 0
            || max_json_depth == 0
            || max_json_depth > MAX_CONFIGURABLE_JSON_DEPTH
            || max_json_nodes == 0
            || max_error_code_bytes == 0
            || max_error_description_bytes == 0
        {
            return Err(CredentialOfferError::InvalidCredentialErrorResponseLimits);
        }
        Ok(Self {
            max_json_bytes,
            max_json_depth,
            max_json_nodes,
            max_error_code_bytes,
            max_error_description_bytes,
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
}

impl Default for CredentialErrorResponseLimits {
    fn default() -> Self {
        Self {
            max_json_bytes: 32_768,
            max_json_depth: 16,
            max_json_nodes: 512,
            max_error_code_bytes: 256,
            max_error_description_bytes: 4_096,
        }
    }
}

/// Resource limits for a caller-supplied Credential payload-error HTTP response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CredentialErrorHttpResponseLimits {
    response_limits: CredentialErrorResponseLimits,
    max_content_type_bytes: usize,
}

impl CredentialErrorHttpResponseLimits {
    /// Combine body limits with a positive Content-Type field-value bound.
    pub const fn new(
        response_limits: CredentialErrorResponseLimits,
        max_content_type_bytes: usize,
    ) -> Result<Self, CredentialOfferError> {
        if max_content_type_bytes == 0 {
            return Err(CredentialOfferError::InvalidCredentialErrorHttpResponseLimits);
        }
        Ok(Self {
            response_limits,
            max_content_type_bytes,
        })
    }

    /// Return the bounded Credential Error Response body policy.
    pub const fn response_limits(self) -> CredentialErrorResponseLimits {
        self.response_limits
    }

    /// Maximum bytes in the effective Content-Type field value.
    pub const fn max_content_type_bytes(self) -> usize {
        self.max_content_type_bytes
    }
}

impl Default for CredentialErrorHttpResponseLimits {
    fn default() -> Self {
        Self {
            response_limits: CredentialErrorResponseLimits::default(),
            max_content_type_bytes: 1_024,
        }
    }
}

/// Resource limits for an OID4VCI Credential Nonce Response core.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CredentialNonceResponseLimits {
    max_json_bytes: usize,
    max_json_depth: usize,
    max_json_nodes: usize,
    max_nonce_bytes: usize,
}

impl CredentialNonceResponseLimits {
    /// Construct a positive nonce-response policy with a supported JSON depth.
    pub const fn new(
        max_json_bytes: usize,
        max_json_depth: usize,
        max_json_nodes: usize,
        max_nonce_bytes: usize,
    ) -> Result<Self, CredentialOfferError> {
        if max_json_bytes == 0
            || max_json_depth == 0
            || max_json_depth > MAX_CONFIGURABLE_JSON_DEPTH
            || max_json_nodes == 0
            || max_nonce_bytes == 0
        {
            return Err(CredentialOfferError::InvalidCredentialNonceResponseLimits);
        }
        Ok(Self {
            max_json_bytes,
            max_json_depth,
            max_json_nodes,
            max_nonce_bytes,
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

    /// Maximum decoded UTF-8 bytes in the Credential Nonce.
    pub const fn max_nonce_bytes(self) -> usize {
        self.max_nonce_bytes
    }
}

impl Default for CredentialNonceResponseLimits {
    fn default() -> Self {
        Self {
            max_json_bytes: 16_384,
            max_json_depth: 16,
            max_json_nodes: 256,
            max_nonce_bytes: 4_096,
        }
    }
}

/// Resource limits for a caller-supplied Credential Nonce HTTP response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CredentialNonceHttpResponseLimits {
    response_limits: CredentialNonceResponseLimits,
    max_content_type_bytes: usize,
    max_cache_control_bytes: usize,
}

impl CredentialNonceHttpResponseLimits {
    /// Combine body limits with positive HTTP field-value byte bounds.
    pub const fn new(
        response_limits: CredentialNonceResponseLimits,
        max_content_type_bytes: usize,
        max_cache_control_bytes: usize,
    ) -> Result<Self, CredentialOfferError> {
        if max_content_type_bytes == 0 || max_cache_control_bytes == 0 {
            return Err(CredentialOfferError::InvalidCredentialNonceHttpResponseLimits);
        }
        Ok(Self {
            response_limits,
            max_content_type_bytes,
            max_cache_control_bytes,
        })
    }

    /// Return the bounded JSON response policy.
    pub const fn response_limits(self) -> CredentialNonceResponseLimits {
        self.response_limits
    }

    /// Maximum bytes in the effective Content-Type field value.
    pub const fn max_content_type_bytes(self) -> usize {
        self.max_content_type_bytes
    }

    /// Maximum bytes in the effective Cache-Control field value.
    pub const fn max_cache_control_bytes(self) -> usize {
        self.max_cache_control_bytes
    }
}

impl Default for CredentialNonceHttpResponseLimits {
    fn default() -> Self {
        Self {
            response_limits: CredentialNonceResponseLimits::default(),
            max_content_type_bytes: 1_024,
            max_cache_control_bytes: 4_096,
        }
    }
}

/// Resource limits for a constructed Final JWT Credential Request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JwtCredentialRequestLimits {
    max_proofs: usize,
    max_proof_bytes: usize,
    max_json_body_bytes: usize,
    max_authorization_bytes: usize,
}

impl JwtCredentialRequestLimits {
    /// Construct a positive Credential Request resource policy.
    pub const fn new(
        max_proofs: usize,
        max_proof_bytes: usize,
        max_json_body_bytes: usize,
        max_authorization_bytes: usize,
    ) -> Result<Self, CredentialOfferError> {
        if max_proofs == 0
            || max_proof_bytes == 0
            || max_json_body_bytes == 0
            || max_authorization_bytes == 0
        {
            return Err(CredentialOfferError::InvalidJwtCredentialRequestLimits);
        }
        Ok(Self {
            max_proofs,
            max_proof_bytes,
            max_json_body_bytes,
            max_authorization_bytes,
        })
    }

    /// Maximum JWT proof count in one request.
    pub const fn max_proofs(self) -> usize {
        self.max_proofs
    }

    /// Maximum bytes in one compact JWT proof.
    pub const fn max_proof_bytes(self) -> usize {
        self.max_proof_bytes
    }

    /// Maximum bytes in the complete JSON request body.
    pub const fn max_json_body_bytes(self) -> usize {
        self.max_json_body_bytes
    }

    /// Maximum bytes in the complete Authorization field value.
    pub const fn max_authorization_bytes(self) -> usize {
        self.max_authorization_bytes
    }
}

impl Default for JwtCredentialRequestLimits {
    fn default() -> Self {
        Self {
            max_proofs: 16,
            max_proof_bytes: 16_384,
            max_json_body_bytes: 262_144,
            max_authorization_bytes: 16_384,
        }
    }
}

/// Resource limits for an immediate OID4VCI Credential Response body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImmediateCredentialResponseLimits {
    max_json_bytes: usize,
    max_json_depth: usize,
    max_json_nodes: usize,
    max_response_members: usize,
    max_credentials: usize,
    max_credential_members: usize,
    max_credential_bytes: usize,
    max_total_credential_bytes: usize,
    max_notification_id_bytes: usize,
}

impl ImmediateCredentialResponseLimits {
    /// Construct a positive response policy with a supported JSON depth.
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        max_json_bytes: usize,
        max_json_depth: usize,
        max_json_nodes: usize,
        max_response_members: usize,
        max_credentials: usize,
        max_credential_members: usize,
        max_credential_bytes: usize,
        max_total_credential_bytes: usize,
        max_notification_id_bytes: usize,
    ) -> Result<Self, CredentialOfferError> {
        if max_json_bytes == 0
            || max_json_depth == 0
            || max_json_depth > MAX_CONFIGURABLE_JSON_DEPTH
            || max_json_nodes == 0
            || max_response_members == 0
            || max_credentials == 0
            || max_credential_members == 0
            || max_credential_bytes == 0
            || max_total_credential_bytes == 0
            || max_notification_id_bytes == 0
        {
            return Err(CredentialOfferError::InvalidImmediateCredentialResponseLimits);
        }
        Ok(Self {
            max_json_bytes,
            max_json_depth,
            max_json_nodes,
            max_response_members,
            max_credentials,
            max_credential_members,
            max_credential_bytes,
            max_total_credential_bytes,
            max_notification_id_bytes,
        })
    }

    /// Maximum bytes in the complete JSON response body.
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

    /// Maximum members in the top-level response object.
    pub const fn max_response_members(self) -> usize {
        self.max_response_members
    }

    /// Maximum credential entries in one immediate response.
    pub const fn max_credentials(self) -> usize {
        self.max_credentials
    }

    /// Maximum members in one credential entry object.
    pub const fn max_credential_members(self) -> usize {
        self.max_credential_members
    }

    /// Maximum exact JSON bytes retained for one credential value.
    pub const fn max_credential_bytes(self) -> usize {
        self.max_credential_bytes
    }

    /// Maximum aggregate exact JSON bytes retained for all credential values.
    pub const fn max_total_credential_bytes(self) -> usize {
        self.max_total_credential_bytes
    }

    /// Maximum decoded UTF-8 bytes in the optional notification identifier.
    pub const fn max_notification_id_bytes(self) -> usize {
        self.max_notification_id_bytes
    }
}

impl Default for ImmediateCredentialResponseLimits {
    fn default() -> Self {
        Self {
            max_json_bytes: 1_048_576,
            max_json_depth: 32,
            max_json_nodes: 16_384,
            max_response_members: 32,
            max_credentials: 64,
            max_credential_members: 32,
            max_credential_bytes: 262_144,
            max_total_credential_bytes: 786_432,
            max_notification_id_bytes: 4_096,
        }
    }
}

/// Resource limits for a caller-supplied immediate Credential HTTP response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImmediateCredentialHttpResponseLimits {
    response_limits: ImmediateCredentialResponseLimits,
    max_content_type_bytes: usize,
}

impl ImmediateCredentialHttpResponseLimits {
    /// Combine body limits with a positive Content-Type field-value bound.
    pub const fn new(
        response_limits: ImmediateCredentialResponseLimits,
        max_content_type_bytes: usize,
    ) -> Result<Self, CredentialOfferError> {
        if max_content_type_bytes == 0 {
            return Err(CredentialOfferError::InvalidImmediateCredentialHttpResponseLimits);
        }
        Ok(Self {
            response_limits,
            max_content_type_bytes,
        })
    }

    /// Return the bounded immediate response-body policy.
    pub const fn response_limits(self) -> ImmediateCredentialResponseLimits {
        self.response_limits
    }

    /// Maximum bytes in the effective Content-Type field value.
    pub const fn max_content_type_bytes(self) -> usize {
        self.max_content_type_bytes
    }
}

impl Default for ImmediateCredentialHttpResponseLimits {
    fn default() -> Self {
        Self {
            response_limits: ImmediateCredentialResponseLimits::default(),
            max_content_type_bytes: 1_024,
        }
    }
}

/// Resource limits for unsigned Credential Issuer Metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CredentialIssuerMetadataLimits {
    max_json_bytes: usize,
    max_json_depth: usize,
    max_json_nodes: usize,
    max_credential_issuer_bytes: usize,
    max_credential_endpoint_bytes: usize,
    max_authorization_server_bytes: usize,
    max_authorization_servers: usize,
    max_credential_configuration_id_bytes: usize,
    max_credential_format_bytes: usize,
    max_credential_configurations: usize,
}

impl CredentialIssuerMetadataLimits {
    /// Construct a positive metadata resource policy with a supported depth.
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        max_json_bytes: usize,
        max_json_depth: usize,
        max_json_nodes: usize,
        max_credential_issuer_bytes: usize,
        max_credential_endpoint_bytes: usize,
        max_authorization_server_bytes: usize,
        max_authorization_servers: usize,
        max_credential_configuration_id_bytes: usize,
        max_credential_format_bytes: usize,
        max_credential_configurations: usize,
    ) -> Result<Self, CredentialOfferError> {
        if max_json_bytes == 0
            || max_json_depth == 0
            || max_json_depth > MAX_CONFIGURABLE_JSON_DEPTH
            || max_json_nodes == 0
            || max_credential_issuer_bytes == 0
            || max_credential_endpoint_bytes == 0
            || max_authorization_server_bytes == 0
            || max_authorization_servers == 0
            || max_credential_configuration_id_bytes == 0
            || max_credential_format_bytes == 0
            || max_credential_configurations == 0
        {
            return Err(CredentialOfferError::InvalidMetadataLimits);
        }
        Ok(Self {
            max_json_bytes,
            max_json_depth,
            max_json_nodes,
            max_credential_issuer_bytes,
            max_credential_endpoint_bytes,
            max_authorization_server_bytes,
            max_authorization_servers,
            max_credential_configuration_id_bytes,
            max_credential_format_bytes,
            max_credential_configurations,
        })
    }

    /// Maximum bytes in the complete unsigned JSON document.
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
    /// Maximum decoded bytes in the Credential Issuer Identifier.
    pub const fn max_credential_issuer_bytes(self) -> usize {
        self.max_credential_issuer_bytes
    }
    /// Maximum decoded bytes in each Credential or Nonce Endpoint URL.
    ///
    /// The shared budget applies independently to each endpoint value.
    pub const fn max_credential_endpoint_bytes(self) -> usize {
        self.max_credential_endpoint_bytes
    }
    /// Maximum decoded bytes in one Authorization Server identifier.
    pub const fn max_authorization_server_bytes(self) -> usize {
        self.max_authorization_server_bytes
    }
    /// Maximum advertised Authorization Server count.
    pub const fn max_authorization_servers(self) -> usize {
        self.max_authorization_servers
    }
    /// Maximum decoded bytes in one Credential Configuration ID.
    pub const fn max_credential_configuration_id_bytes(self) -> usize {
        self.max_credential_configuration_id_bytes
    }
    /// Maximum decoded bytes in one Credential Format identifier.
    pub const fn max_credential_format_bytes(self) -> usize {
        self.max_credential_format_bytes
    }
    /// Maximum Credential Configuration count.
    pub const fn max_credential_configurations(self) -> usize {
        self.max_credential_configurations
    }
}

impl Default for CredentialIssuerMetadataLimits {
    fn default() -> Self {
        Self {
            max_json_bytes: 131_072,
            max_json_depth: 16,
            max_json_nodes: 1_024,
            max_credential_issuer_bytes: 2_048,
            max_credential_endpoint_bytes: 2_048,
            max_authorization_server_bytes: 2_048,
            max_authorization_servers: 16,
            max_credential_configuration_id_bytes: 256,
            max_credential_format_bytes: 128,
            max_credential_configurations: 128,
        }
    }
}

/// Resource limits for the partial Authorization Server Metadata core.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthorizationServerMetadataLimits {
    max_json_bytes: usize,
    max_json_depth: usize,
    max_json_nodes: usize,
    max_issuer_bytes: usize,
    max_endpoint_bytes: usize,
    max_grant_type_bytes: usize,
    max_grant_types: usize,
}

impl AuthorizationServerMetadataLimits {
    /// Construct a positive metadata policy with a supported JSON depth.
    pub const fn new(
        max_json_bytes: usize,
        max_json_depth: usize,
        max_json_nodes: usize,
        max_issuer_bytes: usize,
        max_endpoint_bytes: usize,
        max_grant_type_bytes: usize,
        max_grant_types: usize,
    ) -> Result<Self, CredentialOfferError> {
        if max_json_bytes == 0
            || max_json_depth == 0
            || max_json_depth > MAX_CONFIGURABLE_JSON_DEPTH
            || max_json_nodes == 0
            || max_issuer_bytes == 0
            || max_endpoint_bytes == 0
            || max_grant_type_bytes == 0
            || max_grant_types == 0
        {
            return Err(CredentialOfferError::InvalidAuthorizationServerMetadataLimits);
        }
        Ok(Self {
            max_json_bytes,
            max_json_depth,
            max_json_nodes,
            max_issuer_bytes,
            max_endpoint_bytes,
            max_grant_type_bytes,
            max_grant_types,
        })
    }

    /// Maximum bytes in the complete JSON object.
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

    /// Maximum decoded UTF-8 bytes in the issuer identifier.
    pub const fn max_issuer_bytes(self) -> usize {
        self.max_issuer_bytes
    }

    /// Maximum decoded UTF-8 bytes in either interpreted endpoint.
    pub const fn max_endpoint_bytes(self) -> usize {
        self.max_endpoint_bytes
    }

    /// Maximum decoded UTF-8 bytes in one grant type.
    pub const fn max_grant_type_bytes(self) -> usize {
        self.max_grant_type_bytes
    }

    /// Maximum number of explicitly advertised grant types.
    pub const fn max_grant_types(self) -> usize {
        self.max_grant_types
    }
}

impl Default for AuthorizationServerMetadataLimits {
    fn default() -> Self {
        Self {
            max_json_bytes: 131_072,
            max_json_depth: 16,
            max_json_nodes: 1_024,
            max_issuer_bytes: 2_048,
            max_endpoint_bytes: 2_048,
            max_grant_type_bytes: 256,
            max_grant_types: 32,
        }
    }
}
