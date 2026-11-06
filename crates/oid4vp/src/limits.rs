use identus_jose::JwsLimits;

use crate::Oid4vpError;

const MAX_PRESENTATION_JSON_DEPTH: usize = 64;

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

/// Resource limits for Final Authorization Request routing validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthorizationRequestValidationLimits {
    max_nonce_bytes: usize,
    max_response_uri_bytes: usize,
    max_state_bytes: usize,
}

impl AuthorizationRequestValidationLimits {
    /// Construct a positive routing resource policy.
    pub const fn new(
        max_nonce_bytes: usize,
        max_response_uri_bytes: usize,
    ) -> Result<Self, Oid4vpError> {
        if max_nonce_bytes == 0 || max_response_uri_bytes == 0 {
            return Err(Oid4vpError::InvalidLimits);
        }
        Ok(Self {
            max_nonce_bytes,
            max_response_uri_bytes,
            max_state_bytes: 1_024,
        })
    }

    /// Override the maximum UTF-8 bytes in an optional OAuth state value.
    pub const fn with_max_state_bytes(
        mut self,
        max_state_bytes: usize,
    ) -> Result<Self, Oid4vpError> {
        if max_state_bytes == 0 {
            return Err(Oid4vpError::InvalidLimits);
        }
        self.max_state_bytes = max_state_bytes;
        Ok(self)
    }

    /// Maximum UTF-8 bytes in the authorization nonce.
    pub const fn max_nonce_bytes(self) -> usize {
        self.max_nonce_bytes
    }

    /// Maximum UTF-8 bytes in the direct-post response URI.
    pub const fn max_response_uri_bytes(self) -> usize {
        self.max_response_uri_bytes
    }

    /// Maximum UTF-8 bytes in the optional OAuth state value.
    pub const fn max_state_bytes(self) -> usize {
        self.max_state_bytes
    }
}

impl Default for AuthorizationRequestValidationLimits {
    fn default() -> Self {
        Self {
            max_nonce_bytes: 256,
            max_response_uri_bytes: 4_096,
            max_state_bytes: 1_024,
        }
    }
}

/// Resource limits for one plain OID4VP direct-post success response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectPostResponseLimits {
    max_query_results: usize,
    max_query_id_bytes: usize,
    max_presentations_per_query: usize,
    max_presentation_bytes: usize,
    max_json_depth: usize,
    max_json_nodes: usize,
    max_vp_token_bytes: usize,
    max_form_body_bytes: usize,
}

impl DirectPostResponseLimits {
    /// Construct a positive response resource policy.
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        max_query_results: usize,
        max_query_id_bytes: usize,
        max_presentations_per_query: usize,
        max_presentation_bytes: usize,
        max_json_depth: usize,
        max_json_nodes: usize,
        max_vp_token_bytes: usize,
        max_form_body_bytes: usize,
    ) -> Result<Self, Oid4vpError> {
        if max_query_results == 0
            || max_query_id_bytes == 0
            || max_presentations_per_query == 0
            || max_presentation_bytes == 0
            || max_json_depth == 0
            || max_json_depth > MAX_PRESENTATION_JSON_DEPTH
            || max_json_nodes == 0
            || max_vp_token_bytes == 0
            || max_form_body_bytes == 0
        {
            return Err(Oid4vpError::InvalidLimits);
        }
        Ok(Self {
            max_query_results,
            max_query_id_bytes,
            max_presentations_per_query,
            max_presentation_bytes,
            max_json_depth,
            max_json_nodes,
            max_vp_token_bytes,
            max_form_body_bytes,
        })
    }

    /// Maximum number of query results supplied by a caller.
    pub const fn max_query_results(self) -> usize {
        self.max_query_results
    }

    /// Maximum UTF-8 bytes in one supplied query identifier.
    pub const fn max_query_id_bytes(self) -> usize {
        self.max_query_id_bytes
    }

    /// Maximum presentations supplied for one query.
    pub const fn max_presentations_per_query(self) -> usize {
        self.max_presentations_per_query
    }

    /// Maximum encoded bytes in one presentation value.
    pub const fn max_presentation_bytes(self) -> usize {
        self.max_presentation_bytes
    }

    /// Maximum nested JSON container depth in an object presentation.
    pub const fn max_json_depth(self) -> usize {
        self.max_json_depth
    }

    /// Maximum JSON nodes in an object presentation.
    pub const fn max_json_nodes(self) -> usize {
        self.max_json_nodes
    }

    /// Maximum bytes in the serialized `vp_token` JSON object.
    pub const fn max_vp_token_bytes(self) -> usize {
        self.max_vp_token_bytes
    }

    /// Maximum bytes in the encoded direct-post form body.
    pub const fn max_form_body_bytes(self) -> usize {
        self.max_form_body_bytes
    }
}

impl Default for DirectPostResponseLimits {
    fn default() -> Self {
        Self {
            max_query_results: 64,
            max_query_id_bytes: 256,
            max_presentations_per_query: 16,
            max_presentation_bytes: 65_536,
            max_json_depth: 64,
            max_json_nodes: 8_192,
            max_vp_token_bytes: 262_144,
            max_form_body_bytes: 786_432,
        }
    }
}

/// Resource limits for one plain direct-post Authorization Error Response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectPostErrorResponseLimits {
    max_error_description_bytes: usize,
    max_error_uri_bytes: usize,
    max_form_body_bytes: usize,
}

impl DirectPostErrorResponseLimits {
    /// Construct a positive protocol-error response policy.
    pub const fn new(
        max_error_description_bytes: usize,
        max_error_uri_bytes: usize,
        max_form_body_bytes: usize,
    ) -> Result<Self, Oid4vpError> {
        if max_error_description_bytes == 0 || max_error_uri_bytes == 0 || max_form_body_bytes == 0
        {
            return Err(Oid4vpError::InvalidLimits);
        }
        Ok(Self {
            max_error_description_bytes,
            max_error_uri_bytes,
            max_form_body_bytes,
        })
    }

    /// Maximum decoded bytes in optional developer-oriented text.
    pub const fn max_error_description_bytes(self) -> usize {
        self.max_error_description_bytes
    }

    /// Maximum decoded bytes in the optional URI-reference.
    pub const fn max_error_uri_bytes(self) -> usize {
        self.max_error_uri_bytes
    }

    /// Maximum bytes in the encoded direct-post error form body.
    pub const fn max_form_body_bytes(self) -> usize {
        self.max_form_body_bytes
    }
}

impl Default for DirectPostErrorResponseLimits {
    fn default() -> Self {
        Self {
            max_error_description_bytes: 1_024,
            max_error_uri_bytes: 2_048,
            max_form_body_bytes: 8_192,
        }
    }
}

/// Resource limits for one inert redirect-mode response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RedirectResponseLimits {
    max_output_uri_bytes: usize,
    max_existing_parameters: usize,
}

impl RedirectResponseLimits {
    /// Construct a positive redirect-output policy.
    pub const fn new(
        max_output_uri_bytes: usize,
        max_existing_parameters: usize,
    ) -> Result<Self, Oid4vpError> {
        if max_output_uri_bytes == 0 || max_existing_parameters == 0 {
            return Err(Oid4vpError::InvalidLimits);
        }
        Ok(Self {
            max_output_uri_bytes,
            max_existing_parameters,
        })
    }

    /// Maximum bytes in the complete query or fragment redirect URI.
    pub const fn max_output_uri_bytes(self) -> usize {
        self.max_output_uri_bytes
    }

    /// Maximum existing query or fragment parameters inspected for collisions.
    pub const fn max_existing_parameters(self) -> usize {
        self.max_existing_parameters
    }
}

impl Default for RedirectResponseLimits {
    fn default() -> Self {
        Self {
            max_output_uri_bytes: 1_048_576,
            max_existing_parameters: 64,
        }
    }
}

/// Resource limits for one direct-post endpoint acknowledgement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectPostAcknowledgementLimits {
    max_content_type_bytes: usize,
    max_cache_control_bytes: usize,
    max_body_bytes: usize,
    max_members: usize,
    max_member_name_bytes: usize,
    max_redirect_uri_bytes: usize,
    max_response_code_bytes: usize,
}

impl DirectPostAcknowledgementLimits {
    /// Construct a positive acknowledgement parsing policy.
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        max_content_type_bytes: usize,
        max_cache_control_bytes: usize,
        max_body_bytes: usize,
        max_members: usize,
        max_member_name_bytes: usize,
        max_redirect_uri_bytes: usize,
        max_response_code_bytes: usize,
    ) -> Result<Self, Oid4vpError> {
        if max_content_type_bytes == 0
            || max_cache_control_bytes == 0
            || max_body_bytes == 0
            || max_members == 0
            || max_member_name_bytes == 0
            || max_redirect_uri_bytes == 0
            || max_response_code_bytes == 0
        {
            return Err(Oid4vpError::InvalidLimits);
        }
        Ok(Self {
            max_content_type_bytes,
            max_cache_control_bytes,
            max_body_bytes,
            max_members,
            max_member_name_bytes,
            max_redirect_uri_bytes,
            max_response_code_bytes,
        })
    }

    pub const fn max_content_type_bytes(self) -> usize {
        self.max_content_type_bytes
    }
    pub const fn max_cache_control_bytes(self) -> usize {
        self.max_cache_control_bytes
    }
    pub const fn max_body_bytes(self) -> usize {
        self.max_body_bytes
    }
    pub const fn max_members(self) -> usize {
        self.max_members
    }
    pub const fn max_member_name_bytes(self) -> usize {
        self.max_member_name_bytes
    }
    pub const fn max_redirect_uri_bytes(self) -> usize {
        self.max_redirect_uri_bytes
    }
    pub const fn max_response_code_bytes(self) -> usize {
        self.max_response_code_bytes
    }
}

impl Default for DirectPostAcknowledgementLimits {
    fn default() -> Self {
        Self {
            max_content_type_bytes: 256,
            max_cache_control_bytes: 512,
            max_body_bytes: 16_384,
            max_members: 32,
            max_member_name_bytes: 128,
            max_redirect_uri_bytes: 4_096,
            max_response_code_bytes: 1_024,
        }
    }
}

/// Resource and work limits for one DCQL query evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DcqlLimits {
    max_query_bytes: usize,
    max_collection_items: usize,
    max_string_bytes: usize,
    max_path_components: usize,
    max_credentials: usize,
    max_evaluation_work: usize,
    max_combinations: usize,
}

impl DcqlLimits {
    /// Construct a positive DCQL resource policy.
    pub const fn new(
        max_query_bytes: usize,
        max_collection_items: usize,
        max_string_bytes: usize,
        max_path_components: usize,
        max_credentials: usize,
        max_evaluation_work: usize,
        max_combinations: usize,
    ) -> Result<Self, Oid4vpError> {
        if max_query_bytes == 0
            || max_collection_items == 0
            || max_string_bytes == 0
            || max_path_components == 0
            || max_credentials == 0
            || max_evaluation_work == 0
            || max_combinations == 0
        {
            return Err(Oid4vpError::InvalidLimits);
        }
        Ok(Self {
            max_query_bytes,
            max_collection_items,
            max_string_bytes,
            max_path_components,
            max_credentials,
            max_evaluation_work,
            max_combinations,
        })
    }

    /// Maximum serialized bytes in the extracted `dcql_query`.
    pub const fn max_query_bytes(self) -> usize {
        self.max_query_bytes
    }

    /// Maximum elements in any DCQL-owned collection.
    pub const fn max_collection_items(self) -> usize {
        self.max_collection_items
    }

    /// Maximum UTF-8 bytes in an identifier, format, or path key.
    pub const fn max_string_bytes(self) -> usize {
        self.max_string_bytes
    }

    /// Maximum components in one claims path pointer.
    pub const fn max_path_components(self) -> usize {
        self.max_path_components
    }

    /// Maximum credential descriptors supplied for one evaluation.
    pub const fn max_credentials(self) -> usize {
        self.max_credentials
    }

    /// Maximum checked query/claim-by-credential work units.
    pub const fn max_evaluation_work(self) -> usize {
        self.max_evaluation_work
    }

    /// Maximum satisfying combinations retained in one outcome.
    pub const fn max_combinations(self) -> usize {
        self.max_combinations
    }
}

impl Default for DcqlLimits {
    fn default() -> Self {
        Self {
            max_query_bytes: 16_384,
            max_collection_items: 64,
            max_string_bytes: 256,
            max_path_components: 32,
            max_credentials: 256,
            max_evaluation_work: 65_536,
            max_combinations: 256,
        }
    }
}
