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
