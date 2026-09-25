use std::fmt;

use crate::{
    AUTHORIZATION_CODE_GRANT_TYPE, AuthorizationServerMetadataCore, CredentialOfferError,
    CredentialOfferWithMetadata,
};

/// A matched Credential Offer whose selected server can begin its
/// Authorization Code flow.
///
/// This state proves exact cross-document identifier agreement, effective
/// grant support, and Authorization Endpoint presence. It does not prove
/// metadata provenance, trust, endpoint reachability, client eligibility,
/// request readiness, callback issuer binding, authorization, or successful
/// issuance.
pub struct CredentialOfferWithAuthorizationCodeServer {
    offer: CredentialOfferWithMetadata,
    authorization_server_metadata: AuthorizationServerMetadataCore,
}

impl CredentialOfferWithAuthorizationCodeServer {
    pub(crate) fn into_parts(
        self,
    ) -> (CredentialOfferWithMetadata, AuthorizationServerMetadataCore) {
        (self.offer, self.authorization_server_metadata)
    }

    /// Borrow the matched Credential Offer and Credential Issuer Metadata.
    pub const fn credential_offer_with_metadata(&self) -> &CredentialOfferWithMetadata {
        &self.offer
    }

    /// Borrow the selected partial Authorization Server Metadata core.
    pub const fn authorization_server_metadata(&self) -> &AuthorizationServerMetadataCore {
        &self.authorization_server_metadata
    }
}

impl fmt::Debug for CredentialOfferWithAuthorizationCodeServer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialOfferWithAuthorizationCodeServer")
            .finish_non_exhaustive()
    }
}

impl CredentialOfferWithMetadata {
    /// Consume these validated states after proving Authorization Code server
    /// eligibility.
    pub fn try_with_authorization_code_server(
        self,
        authorization_server_metadata: AuthorizationServerMetadataCore,
    ) -> Result<CredentialOfferWithAuthorizationCodeServer, CredentialOfferError> {
        let authorization_code_grant = self
            .credential_offer()
            .authorization_code()
            .ok_or(CredentialOfferError::AuthorizationCodeGrantMissing)?;
        let selected_issuer = authorization_server_metadata.issuer().as_str();

        if !self
            .credential_issuer_metadata()
            .has_effective_authorization_server(selected_issuer)
        {
            return Err(CredentialOfferError::AuthorizationServerNotAdvertised);
        }
        if authorization_code_grant
            .authorization_server()
            .is_some_and(|hint| hint.as_str() != selected_issuer)
        {
            return Err(CredentialOfferError::AuthorizationCodeServerHintMismatch);
        }
        if !(0..authorization_server_metadata.effective_grant_type_count()).any(|index| {
            authorization_server_metadata.effective_grant_type(index)
                == Some(AUTHORIZATION_CODE_GRANT_TYPE)
        }) {
            return Err(CredentialOfferError::AuthorizationCodeGrantNotSupported);
        }
        if authorization_server_metadata
            .authorization_endpoint()
            .is_none()
        {
            return Err(CredentialOfferError::AuthorizationEndpointRequired);
        }

        Ok(CredentialOfferWithAuthorizationCodeServer {
            offer: self,
            authorization_server_metadata,
        })
    }
}
