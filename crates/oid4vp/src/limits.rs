use identus_jose::JwsLimits;

use crate::Oid4vpError;

/// Resource limits for one Authorization Request invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthorizationRequestInvocationLimits {
    max_invocation_bytes: usize,
    max_parameter_pairs: usize,
    max_parameter_name_bytes: usize,
    max_parameter_value_bytes: usize,
    max_client_id_bytes: usize,
    max_request_uri_bytes: usize,
}

impl AuthorizationRequestInvocationLimits {
    /// Construct a positive resource policy.
    pub const fn new(
        max_invocation_bytes: usize,
        max_parameter_pairs: usize,
        max_parameter_name_bytes: usize,
        max_parameter_value_bytes: usize,
        max_client_id_bytes: usize,
        max_request_uri_bytes: usize,
    ) -> Result<Self, Oid4vpError> {
        if max_invocation_bytes == 0
            || max_parameter_pairs == 0
            || max_parameter_name_bytes == 0
            || max_parameter_value_bytes == 0
            || max_client_id_bytes == 0
            || max_request_uri_bytes == 0
        {
            return Err(Oid4vpError::InvalidLimits);
        }
        Ok(Self {
            max_invocation_bytes,
            max_parameter_pairs,
            max_parameter_name_bytes,
            max_parameter_value_bytes,
            max_client_id_bytes,
            max_request_uri_bytes,
        })
    }

    /// Maximum bytes in the complete invocation.
    pub const fn max_invocation_bytes(self) -> usize {
        self.max_invocation_bytes
    }

    /// Maximum number of query parameter pairs.
    pub const fn max_parameter_pairs(self) -> usize {
        self.max_parameter_pairs
    }

    /// Maximum decoded bytes in one parameter name.
    pub const fn max_parameter_name_bytes(self) -> usize {
        self.max_parameter_name_bytes
    }

    /// Maximum decoded bytes in one parameter value before field-specific limits.
    pub const fn max_parameter_value_bytes(self) -> usize {
        self.max_parameter_value_bytes
    }

    /// Maximum decoded bytes in `client_id`.
    pub const fn max_client_id_bytes(self) -> usize {
        self.max_client_id_bytes
    }

    /// Maximum decoded bytes in `request_uri`.
    pub const fn max_request_uri_bytes(self) -> usize {
        self.max_request_uri_bytes
    }
}

impl Default for AuthorizationRequestInvocationLimits {
    fn default() -> Self {
        Self {
            max_invocation_bytes: 16_384,
            max_parameter_pairs: 32,
            max_parameter_name_bytes: 64,
            max_parameter_value_bytes: 8_192,
            max_client_id_bytes: 2_048,
            max_request_uri_bytes: 4_096,
        }
    }
}

/// Resource limits for Request URI construction and response binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequestUriRetrievalLimits {
    max_wallet_metadata_bytes: usize,
    max_wallet_nonce_bytes: usize,
    max_request_body_bytes: usize,
    max_content_type_bytes: usize,
    max_response_body_bytes: usize,
}

impl RequestUriRetrievalLimits {
    /// Construct a positive retrieval resource policy.
    pub const fn new(
        max_wallet_metadata_bytes: usize,
        max_wallet_nonce_bytes: usize,
        max_request_body_bytes: usize,
        max_content_type_bytes: usize,
        max_response_body_bytes: usize,
    ) -> Result<Self, Oid4vpError> {
        if max_wallet_metadata_bytes == 0
            || max_wallet_nonce_bytes == 0
            || max_request_body_bytes == 0
            || max_content_type_bytes == 0
            || max_response_body_bytes == 0
        {
            return Err(Oid4vpError::InvalidLimits);
        }
        Ok(Self {
            max_wallet_metadata_bytes,
            max_wallet_nonce_bytes,
            max_request_body_bytes,
            max_content_type_bytes,
            max_response_body_bytes,
        })
    }

    /// Maximum bytes in the optional wallet metadata JSON string.
    pub const fn max_wallet_metadata_bytes(self) -> usize {
        self.max_wallet_metadata_bytes
    }

    /// Maximum bytes in the optional wallet nonce.
    pub const fn max_wallet_nonce_bytes(self) -> usize {
        self.max_wallet_nonce_bytes
    }

    /// Maximum bytes in the encoded POST form body.
    pub const fn max_request_body_bytes(self) -> usize {
        self.max_request_body_bytes
    }

    /// Maximum bytes in the response content-type field value.
    pub const fn max_content_type_bytes(self) -> usize {
        self.max_content_type_bytes
    }

    /// Maximum bytes in the received Request Object body.
    pub const fn max_response_body_bytes(self) -> usize {
        self.max_response_body_bytes
    }
}

impl Default for RequestUriRetrievalLimits {
    fn default() -> Self {
        Self {
            max_wallet_metadata_bytes: 16_384,
            max_wallet_nonce_bytes: 256,
            max_request_body_bytes: 24_576,
            max_content_type_bytes: 128,
            max_response_body_bytes: 65_536,
        }
    }
}

/// Resource limits for signed Request Object parsing and claim correlation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequestObjectValidationLimits {
    jws: JwsLimits,
    max_json_depth: usize,
    max_json_nodes: usize,
    max_object_members: usize,
    max_string_bytes: usize,
}

impl RequestObjectValidationLimits {
    /// Construct a positive JAR validation resource policy.
    pub const fn new(
        jws: JwsLimits,
        max_json_depth: usize,
        max_json_nodes: usize,
        max_object_members: usize,
        max_string_bytes: usize,
    ) -> Result<Self, Oid4vpError> {
        if max_json_depth == 0
            || max_json_nodes == 0
            || max_object_members == 0
            || max_string_bytes == 0
        {
            return Err(Oid4vpError::InvalidLimits);
        }
        Ok(Self {
            jws,
            max_json_depth,
            max_json_nodes,
            max_object_members,
            max_string_bytes,
        })
    }

    /// Compact JWS and decoded-segment limits.
    pub const fn jws(self) -> JwsLimits {
        self.jws
    }

    /// Maximum nested JSON container depth.
    pub const fn max_json_depth(self) -> usize {
        self.max_json_depth
    }

    /// Maximum total JSON value nodes.
    pub const fn max_json_nodes(self) -> usize {
        self.max_json_nodes
    }

    /// Maximum members in any one JSON object.
    pub const fn max_object_members(self) -> usize {
        self.max_object_members
    }

    /// Maximum decoded bytes in any JSON string.
    pub const fn max_string_bytes(self) -> usize {
        self.max_string_bytes
    }
}

impl Default for RequestObjectValidationLimits {
    fn default() -> Self {
        Self {
            jws: JwsLimits::default(),
            max_json_depth: 64,
            max_json_nodes: 8_192,
            max_object_members: 256,
            max_string_bytes: 16_384,
        }
    }
}
