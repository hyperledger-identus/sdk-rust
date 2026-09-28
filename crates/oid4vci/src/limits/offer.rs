use crate::CredentialOfferError;

use super::{MAX_CONFIGURABLE_JSON_DEPTH, policy};
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
            max_invocation_bytes: policy::DEFAULT_OFFER_MAX_INVOCATION_BYTES,
            max_embedded_json_bytes: policy::DEFAULT_OFFER_MAX_EMBEDDED_JSON_BYTES,
            max_reference_uri_bytes: policy::DEFAULT_OFFER_MAX_REFERENCE_URI_BYTES,
            max_json_depth: policy::DEFAULT_OFFER_MAX_JSON_DEPTH,
            max_json_nodes: policy::DEFAULT_OFFER_MAX_JSON_NODES,
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
            max_credential_issuer_bytes: policy::DEFAULT_OFFER_SEMANTIC_MAX_ISSUER_BYTES,
            max_credential_configuration_id_bytes:
                policy::DEFAULT_OFFER_SEMANTIC_MAX_CONFIGURATION_ID_BYTES,
            max_credential_configuration_ids: policy::DEFAULT_OFFER_SEMANTIC_MAX_CONFIGURATION_IDS,
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
            max_issuer_state_bytes: policy::DEFAULT_OFFER_GRANT_MAX_ISSUER_STATE_BYTES,
            max_pre_authorized_code_bytes:
                policy::DEFAULT_OFFER_GRANT_MAX_PRE_AUTHORIZED_CODE_BYTES,
            max_authorization_server_bytes:
                policy::DEFAULT_OFFER_GRANT_MAX_AUTHORIZATION_SERVER_BYTES,
            max_transaction_code_description_bytes:
                policy::DEFAULT_OFFER_GRANT_MAX_TRANSACTION_DESCRIPTION_BYTES,
            max_transaction_code_length: policy::DEFAULT_OFFER_GRANT_MAX_TRANSACTION_CODE_LENGTH,
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
            max_transaction_code_bytes: policy::DEFAULT_TRANSACTION_CODE_INPUT_MAX_BYTES,
        }
    }
}
