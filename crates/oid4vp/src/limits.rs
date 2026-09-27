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
