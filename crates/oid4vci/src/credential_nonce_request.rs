use std::fmt;

use crate::{CredentialIssuerMetadata, CredentialOfferError, NonceEndpoint};

/// HTTP method required for an OpenID4VCI Final Credential Nonce Request.
pub const NONCE_REQUEST_HTTP_METHOD: &str = "POST";

/// Canonical empty body for an OpenID4VCI Final Credential Nonce Request.
pub const NONCE_REQUEST_BODY: &[u8] = b"";

/// Transport-neutral description of a Final Credential Nonce Request.
///
/// This value does not execute HTTP or establish endpoint trust, network
/// safety, response provenance, or nonce lifecycle properties.
pub struct CredentialNonceRequest {
    nonce_endpoint: NonceEndpoint,
}

impl CredentialNonceRequest {
    /// Borrow the exact validated Nonce Endpoint advertised by the issuer.
    pub const fn nonce_endpoint(&self) -> &NonceEndpoint {
        &self.nonce_endpoint
    }

    /// Return the required HTTP method.
    pub const fn http_method(&self) -> &'static str {
        NONCE_REQUEST_HTTP_METHOD
    }

    /// Return the canonical empty request body.
    pub const fn body(&self) -> &'static [u8] {
        NONCE_REQUEST_BODY
    }

    /// Report whether the request requires an OAuth access token.
    pub const fn access_token_required(&self) -> bool {
        false
    }
}

impl fmt::Debug for CredentialNonceRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialNonceRequest")
            .field("body_bytes", &NONCE_REQUEST_BODY.len())
            .field("access_token_required", &false)
            .finish_non_exhaustive()
    }
}

impl CredentialIssuerMetadata {
    /// Construct an owned Final Credential Nonce Request from advertised metadata.
    pub fn try_nonce_request(&self) -> Result<CredentialNonceRequest, CredentialOfferError> {
        let nonce_endpoint = self
            .nonce_endpoint()
            .ok_or(CredentialOfferError::NonceEndpointRequired)?;
        Ok(CredentialNonceRequest {
            nonce_endpoint: nonce_endpoint.duplicate(),
        })
    }
}
