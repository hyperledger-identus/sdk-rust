use crate::CredentialOfferError;

use super::{MAX_CONFIGURABLE_JSON_DEPTH, policy};
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
            max_json_bytes: policy::DEFAULT_CREDENTIAL_ERROR_MAX_JSON_BYTES,
            max_json_depth: policy::DEFAULT_CREDENTIAL_ERROR_MAX_JSON_DEPTH,
            max_json_nodes: policy::DEFAULT_CREDENTIAL_ERROR_MAX_JSON_NODES,
            max_error_code_bytes: policy::DEFAULT_CREDENTIAL_ERROR_MAX_CODE_BYTES,
            max_error_description_bytes: policy::DEFAULT_CREDENTIAL_ERROR_MAX_DESCRIPTION_BYTES,
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
            max_content_type_bytes: policy::DEFAULT_CREDENTIAL_ERROR_HTTP_MAX_CONTENT_TYPE_BYTES,
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
            max_json_bytes: policy::DEFAULT_CREDENTIAL_NONCE_MAX_JSON_BYTES,
            max_json_depth: policy::DEFAULT_CREDENTIAL_NONCE_MAX_JSON_DEPTH,
            max_json_nodes: policy::DEFAULT_CREDENTIAL_NONCE_MAX_JSON_NODES,
            max_nonce_bytes: policy::DEFAULT_CREDENTIAL_NONCE_MAX_NONCE_BYTES,
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
            max_content_type_bytes: policy::DEFAULT_CREDENTIAL_NONCE_HTTP_MAX_CONTENT_TYPE_BYTES,
            max_cache_control_bytes: policy::DEFAULT_CREDENTIAL_NONCE_HTTP_MAX_CACHE_CONTROL_BYTES,
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
            max_proofs: policy::DEFAULT_JWT_CREDENTIAL_REQUEST_MAX_PROOFS,
            max_proof_bytes: policy::DEFAULT_JWT_CREDENTIAL_REQUEST_MAX_PROOF_BYTES,
            max_json_body_bytes: policy::DEFAULT_JWT_CREDENTIAL_REQUEST_MAX_JSON_BODY_BYTES,
            max_authorization_bytes: policy::DEFAULT_JWT_CREDENTIAL_REQUEST_MAX_AUTHORIZATION_BYTES,
        }
    }
}

/// Resource limits for a constructed Final Deferred Credential Request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeferredCredentialRequestLimits {
    max_json_body_bytes: usize,
}

impl DeferredCredentialRequestLimits {
    /// Construct a positive complete JSON request-body policy.
    pub const fn new(max_json_body_bytes: usize) -> Result<Self, CredentialOfferError> {
        if max_json_body_bytes == 0 {
            return Err(CredentialOfferError::InvalidDeferredCredentialRequestLimits);
        }
        Ok(Self {
            max_json_body_bytes,
        })
    }

    /// Maximum bytes in the complete JSON request body.
    pub const fn max_json_body_bytes(self) -> usize {
        self.max_json_body_bytes
    }
}

impl Default for DeferredCredentialRequestLimits {
    fn default() -> Self {
        Self {
            max_json_body_bytes: policy::DEFAULT_DEFERRED_CREDENTIAL_REQUEST_MAX_JSON_BODY_BYTES,
        }
    }
}

/// Resource limits for a deferred OID4VCI Credential Response body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeferredCredentialResponseLimits {
    max_json_bytes: usize,
    max_json_depth: usize,
    max_json_nodes: usize,
    max_response_members: usize,
    max_transaction_id_bytes: usize,
    max_interval_bytes: usize,
}

impl DeferredCredentialResponseLimits {
    /// Construct a positive response policy with a supported JSON depth.
    pub const fn new(
        max_json_bytes: usize,
        max_json_depth: usize,
        max_json_nodes: usize,
        max_response_members: usize,
        max_transaction_id_bytes: usize,
        max_interval_bytes: usize,
    ) -> Result<Self, CredentialOfferError> {
        if max_json_bytes == 0
            || max_json_depth == 0
            || max_json_depth > MAX_CONFIGURABLE_JSON_DEPTH
            || max_json_nodes == 0
            || max_response_members == 0
            || max_transaction_id_bytes == 0
            || max_interval_bytes == 0
        {
            return Err(CredentialOfferError::InvalidDeferredCredentialResponseLimits);
        }
        Ok(Self {
            max_json_bytes,
            max_json_depth,
            max_json_nodes,
            max_response_members,
            max_transaction_id_bytes,
            max_interval_bytes,
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

    /// Maximum top-level response members.
    pub const fn max_response_members(self) -> usize {
        self.max_response_members
    }

    /// Maximum decoded UTF-8 bytes in the transaction identifier.
    pub const fn max_transaction_id_bytes(self) -> usize {
        self.max_transaction_id_bytes
    }

    /// Maximum bytes in the exact interval JSON-number lexeme.
    pub const fn max_interval_bytes(self) -> usize {
        self.max_interval_bytes
    }
}

impl Default for DeferredCredentialResponseLimits {
    fn default() -> Self {
        Self {
            max_json_bytes: policy::DEFAULT_DEFERRED_CREDENTIAL_RESPONSE_MAX_JSON_BYTES,
            max_json_depth: policy::DEFAULT_DEFERRED_CREDENTIAL_RESPONSE_MAX_JSON_DEPTH,
            max_json_nodes: policy::DEFAULT_DEFERRED_CREDENTIAL_RESPONSE_MAX_JSON_NODES,
            max_response_members: policy::DEFAULT_DEFERRED_CREDENTIAL_RESPONSE_MAX_MEMBERS,
            max_transaction_id_bytes:
                policy::DEFAULT_DEFERRED_CREDENTIAL_RESPONSE_MAX_TRANSACTION_ID_BYTES,
            max_interval_bytes: policy::DEFAULT_DEFERRED_CREDENTIAL_RESPONSE_MAX_INTERVAL_BYTES,
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
    #[expect(
        clippy::too_many_arguments,
        reason = "public positional limits constructor retained until a focused API migration"
    )]
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
            max_json_bytes: policy::DEFAULT_IMMEDIATE_CREDENTIAL_RESPONSE_MAX_JSON_BYTES,
            max_json_depth: policy::DEFAULT_IMMEDIATE_CREDENTIAL_RESPONSE_MAX_JSON_DEPTH,
            max_json_nodes: policy::DEFAULT_IMMEDIATE_CREDENTIAL_RESPONSE_MAX_JSON_NODES,
            max_response_members: policy::DEFAULT_IMMEDIATE_CREDENTIAL_RESPONSE_MAX_MEMBERS,
            max_credentials: policy::DEFAULT_IMMEDIATE_CREDENTIAL_RESPONSE_MAX_CREDENTIALS,
            max_credential_members:
                policy::DEFAULT_IMMEDIATE_CREDENTIAL_RESPONSE_MAX_CREDENTIAL_MEMBERS,
            max_credential_bytes:
                policy::DEFAULT_IMMEDIATE_CREDENTIAL_RESPONSE_MAX_CREDENTIAL_BYTES,
            max_total_credential_bytes:
                policy::DEFAULT_IMMEDIATE_CREDENTIAL_RESPONSE_MAX_TOTAL_CREDENTIAL_BYTES,
            max_notification_id_bytes:
                policy::DEFAULT_IMMEDIATE_CREDENTIAL_RESPONSE_MAX_NOTIFICATION_ID_BYTES,
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
            max_content_type_bytes:
                policy::DEFAULT_IMMEDIATE_CREDENTIAL_HTTP_MAX_CONTENT_TYPE_BYTES,
        }
    }
}

/// Resource limits for a caller-supplied Deferred Credential HTTP response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeferredCredentialHttpResponseLimits {
    immediate_response_limits: ImmediateCredentialResponseLimits,
    deferred_response_limits: DeferredCredentialResponseLimits,
    max_content_type_bytes: usize,
}

impl DeferredCredentialHttpResponseLimits {
    /// Combine both successful body policies with a positive Content-Type bound.
    pub const fn new(
        immediate_response_limits: ImmediateCredentialResponseLimits,
        deferred_response_limits: DeferredCredentialResponseLimits,
        max_content_type_bytes: usize,
    ) -> Result<Self, CredentialOfferError> {
        if max_content_type_bytes == 0 {
            return Err(CredentialOfferError::InvalidDeferredCredentialHttpResponseLimits);
        }
        Ok(Self {
            immediate_response_limits,
            deferred_response_limits,
            max_content_type_bytes,
        })
    }

    /// Return the bounded issued response-body policy.
    pub const fn immediate_response_limits(self) -> ImmediateCredentialResponseLimits {
        self.immediate_response_limits
    }

    /// Return the bounded still-pending response-body policy.
    pub const fn deferred_response_limits(self) -> DeferredCredentialResponseLimits {
        self.deferred_response_limits
    }

    /// Maximum bytes in the effective Content-Type field value.
    pub const fn max_content_type_bytes(self) -> usize {
        self.max_content_type_bytes
    }
}

impl Default for DeferredCredentialHttpResponseLimits {
    fn default() -> Self {
        Self {
            immediate_response_limits: ImmediateCredentialResponseLimits::default(),
            deferred_response_limits: DeferredCredentialResponseLimits::default(),
            max_content_type_bytes: policy::DEFAULT_DEFERRED_CREDENTIAL_HTTP_MAX_CONTENT_TYPE_BYTES,
        }
    }
}

/// Resource policies for every unencrypted Deferred Credential Endpoint response branch.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DeferredCredentialEndpointResponseLimits {
    success_response_limits: DeferredCredentialHttpResponseLimits,
    error_response_limits: CredentialErrorHttpResponseLimits,
}

impl DeferredCredentialEndpointResponseLimits {
    /// Compose already-validated policies for the `200`, `202`, and `400` branches.
    pub const fn new(
        success_response_limits: DeferredCredentialHttpResponseLimits,
        error_response_limits: CredentialErrorHttpResponseLimits,
    ) -> Self {
        Self {
            success_response_limits,
            error_response_limits,
        }
    }

    /// Return the successful (`200` or `202`) response policy.
    pub const fn success_response_limits(self) -> DeferredCredentialHttpResponseLimits {
        self.success_response_limits
    }

    /// Return the payload-error (`400`) response policy.
    pub const fn error_response_limits(self) -> CredentialErrorHttpResponseLimits {
        self.error_response_limits
    }
}

/// Resource policies for every unencrypted Credential Endpoint response branch.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CredentialEndpointResponseLimits {
    immediate_response_limits: ImmediateCredentialHttpResponseLimits,
    deferred_response_limits: DeferredCredentialHttpResponseLimits,
    error_response_limits: CredentialErrorHttpResponseLimits,
}

impl CredentialEndpointResponseLimits {
    /// Compose already-validated policies for the `200`, `202`, and `400` branches.
    pub const fn new(
        immediate_response_limits: ImmediateCredentialHttpResponseLimits,
        deferred_response_limits: DeferredCredentialHttpResponseLimits,
        error_response_limits: CredentialErrorHttpResponseLimits,
    ) -> Self {
        Self {
            immediate_response_limits,
            deferred_response_limits,
            error_response_limits,
        }
    }

    /// Return the immediate-success (`200`) response policy.
    pub const fn immediate_response_limits(self) -> ImmediateCredentialHttpResponseLimits {
        self.immediate_response_limits
    }

    /// Return the deferred-success (`202`) response policy.
    pub const fn deferred_response_limits(self) -> DeferredCredentialHttpResponseLimits {
        self.deferred_response_limits
    }

    /// Return the payload-error (`400`) response policy.
    pub const fn error_response_limits(self) -> CredentialErrorHttpResponseLimits {
        self.error_response_limits
    }
}
