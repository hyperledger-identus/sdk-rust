use std::fmt;

use zeroize::Zeroizing;

use crate::{
    AuthorizationCodeTokenResponseLineage, CredentialOfferError,
    RequestBoundAuthorizationCodeTokenResponse, TokenAuthorizationDetailsLimits, TokenResponseCore,
};

/// A successful Authorization Code Token Response correlated to its exact request.
///
/// This state proves that the response contained exactly one recognized
/// `openid_credential` Authorization Details entry and that its Credential
/// Configuration ID matches the one selected for the Authorization Request.
/// It does not establish token, issuer, dataset or Credential trust.
pub struct CorrelatedAuthorizationCodeTokenResponse {
    lineage: AuthorizationCodeTokenResponseLineage,
    response: TokenResponseCore,
    authorized_credential_identifiers: Vec<Zeroizing<String>>,
    unknown_authorization_detail_count: usize,
}

impl CorrelatedAuthorizationCodeTokenResponse {
    /// Borrow the exact request lineage retained across the token exchange.
    pub const fn lineage(&self) -> &AuthorizationCodeTokenResponseLineage {
        &self.lineage
    }

    /// Borrow the bounded successful Token Response core.
    pub const fn token_response_core(&self) -> &TokenResponseCore {
        &self.response
    }

    /// Return the number of authorized Credential Dataset identifiers.
    pub fn authorized_credential_identifier_count(&self) -> usize {
        self.authorized_credential_identifiers.len()
    }

    /// Iterate over the authorized Credential Dataset identifiers in source order.
    pub fn authorized_credential_identifiers(&self) -> impl ExactSizeIterator<Item = &str> {
        self.authorized_credential_identifiers
            .iter()
            .map(|value| value.as_str())
    }

    /// Return the number of bounded unsupported authorization-detail types.
    pub const fn unknown_authorization_detail_count(&self) -> usize {
        self.unknown_authorization_detail_count
    }
}

impl fmt::Debug for CorrelatedAuthorizationCodeTokenResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CorrelatedAuthorizationCodeTokenResponse")
            .field("lineage", &self.lineage)
            .field(
                "authorized_credential_identifier_count",
                &self.authorized_credential_identifiers.len(),
            )
            .field(
                "unknown_authorization_detail_count",
                &self.unknown_authorization_detail_count,
            )
            .finish_non_exhaustive()
    }
}

impl RequestBoundAuthorizationCodeTokenResponse {
    /// Consume this success and correlate its Authorization Details to the request.
    ///
    /// The transition rejects any recognized configuration other than the exact
    /// request selection and rejects repeated recognized entries. Bounded unknown
    /// authorization-detail types are counted but confer no credential authority.
    pub fn try_correlate_authorization_details(
        self,
        limits: TokenAuthorizationDetailsLimits,
    ) -> Result<CorrelatedAuthorizationCodeTokenResponse, CredentialOfferError> {
        let (lineage, response) = self.into_parts();
        let validated = response.try_validate_authorization_details(limits)?;
        let (response, mut details, unknown_authorization_detail_count) = validated.into_parts();
        let selected_configuration = lineage.selected_credential_configuration().as_str();

        if details
            .iter()
            .any(|detail| detail.credential_configuration_id() != selected_configuration)
        {
            return Err(CredentialOfferError::AuthorizationCodeTokenConfigurationMismatch);
        }
        if details.len() != 1 {
            return Err(CredentialOfferError::AmbiguousAuthorizationCodeTokenAuthorizationDetails);
        }

        let Some(detail) = details.pop() else {
            return Err(CredentialOfferError::InvalidTokenAuthorizationDetails);
        };
        let (_, authorized_credential_identifiers) = detail.into_parts();

        Ok(CorrelatedAuthorizationCodeTokenResponse {
            lineage,
            response,
            authorized_credential_identifiers,
            unknown_authorization_detail_count,
        })
    }
}
