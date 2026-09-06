use std::fmt;

use crate::{
    AuthorizationServerMetadataCore, CredentialOfferError, CredentialOfferWithMetadata,
    PRE_AUTHORIZED_CODE_GRANT_TYPE,
};

/// A matched Credential Offer whose selected server can receive its
/// Pre-Authorized Code grant.
///
/// This state proves exact cross-document identifier agreement, explicit grant
/// advertisement, and Token Endpoint presence. It does not prove retrieval
/// provenance, trust, endpoint reachability, client eligibility, request
/// readiness, or successful issuance.
pub struct CredentialOfferWithPreAuthorizedServer {
    offer: CredentialOfferWithMetadata,
    authorization_server_metadata: AuthorizationServerMetadataCore,
}

impl CredentialOfferWithPreAuthorizedServer {
    /// Borrow the matched Credential Offer and Credential Issuer Metadata.
    pub const fn credential_offer_with_metadata(&self) -> &CredentialOfferWithMetadata {
        &self.offer
    }

    /// Borrow the selected partial Authorization Server Metadata core.
    pub const fn authorization_server_metadata(&self) -> &AuthorizationServerMetadataCore {
        &self.authorization_server_metadata
    }
}

impl fmt::Debug for CredentialOfferWithPreAuthorizedServer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialOfferWithPreAuthorizedServer")
            .finish_non_exhaustive()
    }
}

impl CredentialOfferWithMetadata {
    /// Consume these validated states after proving Pre-Authorized server
    /// eligibility.
    pub fn try_with_pre_authorized_server(
        self,
        authorization_server_metadata: AuthorizationServerMetadataCore,
    ) -> Result<CredentialOfferWithPreAuthorizedServer, CredentialOfferError> {
        let pre_authorized_grant = self
            .credential_offer()
            .pre_authorized_code()
            .ok_or(CredentialOfferError::PreAuthorizedCodeGrantMissing)?;
        let selected_issuer = authorization_server_metadata.issuer().as_str();

        if !self
            .credential_issuer_metadata()
            .has_effective_authorization_server(selected_issuer)
        {
            return Err(CredentialOfferError::AuthorizationServerNotAdvertised);
        }
        if pre_authorized_grant
            .authorization_server()
            .is_some_and(|hint| hint.as_str() != selected_issuer)
        {
            return Err(CredentialOfferError::PreAuthorizedServerHintMismatch);
        }
        if !(0..authorization_server_metadata.effective_grant_type_count()).any(|index| {
            authorization_server_metadata.effective_grant_type(index)
                == Some(PRE_AUTHORIZED_CODE_GRANT_TYPE)
        }) {
            return Err(CredentialOfferError::PreAuthorizedGrantNotSupported);
        }
        if authorization_server_metadata.token_endpoint().is_none() {
            return Err(CredentialOfferError::TokenEndpointRequired);
        }

        Ok(CredentialOfferWithPreAuthorizedServer {
            offer: self,
            authorization_server_metadata,
        })
    }
}
