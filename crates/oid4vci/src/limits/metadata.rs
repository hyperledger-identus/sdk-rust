use crate::CredentialOfferError;

use super::{MAX_CONFIGURABLE_JSON_DEPTH, policy};
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
    #[expect(
        clippy::too_many_arguments,
        reason = "public positional limits constructor retained until a focused API migration"
    )]
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
    /// Maximum decoded bytes in each Credential, Nonce or Deferred Credential Endpoint URL.
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
            max_json_bytes: policy::DEFAULT_ISSUER_METADATA_MAX_JSON_BYTES,
            max_json_depth: policy::DEFAULT_ISSUER_METADATA_MAX_JSON_DEPTH,
            max_json_nodes: policy::DEFAULT_ISSUER_METADATA_MAX_JSON_NODES,
            max_credential_issuer_bytes: policy::DEFAULT_ISSUER_METADATA_MAX_ISSUER_BYTES,
            max_credential_endpoint_bytes: policy::DEFAULT_ISSUER_METADATA_MAX_ENDPOINT_BYTES,
            max_authorization_server_bytes:
                policy::DEFAULT_ISSUER_METADATA_MAX_AUTHORIZATION_SERVER_BYTES,
            max_authorization_servers: policy::DEFAULT_ISSUER_METADATA_MAX_AUTHORIZATION_SERVERS,
            max_credential_configuration_id_bytes:
                policy::DEFAULT_ISSUER_METADATA_MAX_CONFIGURATION_ID_BYTES,
            max_credential_format_bytes: policy::DEFAULT_ISSUER_METADATA_MAX_FORMAT_BYTES,
            max_credential_configurations: policy::DEFAULT_ISSUER_METADATA_MAX_CONFIGURATIONS,
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
            max_json_bytes: policy::DEFAULT_AUTHORIZATION_SERVER_METADATA_MAX_JSON_BYTES,
            max_json_depth: policy::DEFAULT_AUTHORIZATION_SERVER_METADATA_MAX_JSON_DEPTH,
            max_json_nodes: policy::DEFAULT_AUTHORIZATION_SERVER_METADATA_MAX_JSON_NODES,
            max_issuer_bytes: policy::DEFAULT_AUTHORIZATION_SERVER_METADATA_MAX_ISSUER_BYTES,
            max_endpoint_bytes: policy::DEFAULT_AUTHORIZATION_SERVER_METADATA_MAX_ENDPOINT_BYTES,
            max_grant_type_bytes:
                policy::DEFAULT_AUTHORIZATION_SERVER_METADATA_MAX_GRANT_TYPE_BYTES,
            max_grant_types: policy::DEFAULT_AUTHORIZATION_SERVER_METADATA_MAX_GRANT_TYPES,
        }
    }
}
