use std::fmt;

use zeroize::Zeroizing;

use crate::{
    AuthorizationServerIdentifier, CredentialIssuerIdentifier, CredentialIssuerMetadataLimits,
    CredentialOfferError, CredentialOfferWithGrants, json::parse_credential_issuer_metadata_fields,
    semantic::is_valid_https_endpoint,
};

/// A syntactically validated HTTPS Credential Endpoint URL.
pub struct CredentialEndpoint {
    value: Zeroizing<String>,
}

impl CredentialEndpoint {
    /// Borrow the exact Credential Endpoint URL.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl fmt::Debug for CredentialEndpoint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialEndpoint")
            .finish_non_exhaustive()
    }
}

/// An opaque Credential Format identifier from issuer metadata.
pub struct CredentialFormatIdentifier {
    value: Zeroizing<String>,
}

impl CredentialFormatIdentifier {
    /// Borrow the exact opaque format identifier.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl fmt::Debug for CredentialFormatIdentifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialFormatIdentifier")
            .finish_non_exhaustive()
    }
}

/// Bounded cross-protocol fields for one advertised Credential Configuration.
pub struct CredentialConfigurationSummary {
    id: Zeroizing<String>,
    format: CredentialFormatIdentifier,
}

impl CredentialConfigurationSummary {
    /// Borrow the exact Credential Configuration ID.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Borrow the opaque Credential Format identifier.
    pub const fn format(&self) -> &CredentialFormatIdentifier {
        &self.format
    }
}

impl fmt::Debug for CredentialConfigurationSummary {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialConfigurationSummary")
            .finish_non_exhaustive()
    }
}

/// Bounded unsigned Credential Issuer Metadata core.
///
/// This state proves syntax and an exact expected-issuer comparison. It does
/// not prove retrieval provenance, signer trust, endpoint safety, reachability,
/// or support for any advertised format.
pub struct CredentialIssuerMetadata {
    json: Zeroizing<String>,
    credential_issuer: CredentialIssuerIdentifier,
    authorization_servers: Option<Vec<AuthorizationServerIdentifier>>,
    credential_endpoint: CredentialEndpoint,
    credential_configurations: Vec<CredentialConfigurationSummary>,
}

impl CredentialIssuerMetadata {
    /// Parse bounded unsigned JSON and bind it to an expected issuer identifier.
    pub fn parse(
        json: &str,
        expected_credential_issuer: &str,
        limits: CredentialIssuerMetadataLimits,
    ) -> Result<Self, CredentialOfferError> {
        if json.len() > limits.max_json_bytes() {
            return Err(CredentialOfferError::MetadataTooLarge);
        }
        if expected_credential_issuer.len() > limits.max_credential_issuer_bytes() {
            return Err(CredentialOfferError::IssuerTooLarge);
        }
        let expected = CredentialIssuerIdentifier::try_from_value(Zeroizing::new(
            expected_credential_issuer.to_owned(),
        ))?;
        let json = Zeroizing::new(json.to_owned());
        let fields = parse_credential_issuer_metadata_fields(json.as_bytes(), limits)?;
        let credential_issuer =
            CredentialIssuerIdentifier::try_from_value(fields.credential_issuer)?;
        if credential_issuer.as_str() != expected.as_str() {
            return Err(CredentialOfferError::MetadataIssuerMismatch);
        }
        let authorization_servers = fields
            .authorization_servers
            .map(|servers| {
                servers
                    .into_iter()
                    .map(AuthorizationServerIdentifier::try_from_value)
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;
        if !is_valid_https_endpoint(fields.credential_endpoint.as_str()) {
            return Err(CredentialOfferError::UnsafeCredentialEndpoint);
        }
        let credential_configurations = fields
            .credential_configurations
            .into_iter()
            .map(|configuration| CredentialConfigurationSummary {
                id: configuration.id,
                format: CredentialFormatIdentifier {
                    value: configuration.format,
                },
            })
            .collect();
        Ok(Self {
            json,
            credential_issuer,
            authorization_servers,
            credential_endpoint: CredentialEndpoint {
                value: fields.credential_endpoint,
            },
            credential_configurations,
        })
    }

    /// Borrow the exact validated Credential Issuer Identifier.
    pub const fn credential_issuer(&self) -> &CredentialIssuerIdentifier {
        &self.credential_issuer
    }

    /// Borrow the advertised Authorization Servers, or `None` when omitted.
    pub fn advertised_authorization_servers(&self) -> Option<&[AuthorizationServerIdentifier]> {
        self.authorization_servers.as_deref()
    }

    /// Return the number of effective Authorization Servers.
    pub fn effective_authorization_server_count(&self) -> usize {
        self.authorization_servers.as_ref().map_or(1, Vec::len)
    }

    /// Borrow one effective Authorization Server by index.
    ///
    /// When no list was advertised, index zero is the Credential Issuer.
    pub fn effective_authorization_server(&self, index: usize) -> Option<&str> {
        match &self.authorization_servers {
            Some(servers) => servers
                .get(index)
                .map(AuthorizationServerIdentifier::as_str),
            None if index == 0 => Some(self.credential_issuer.as_str()),
            None => None,
        }
    }

    /// Borrow the validated Credential Endpoint.
    pub const fn credential_endpoint(&self) -> &CredentialEndpoint {
        &self.credential_endpoint
    }

    /// Borrow ordered Credential Configuration summaries.
    pub fn credential_configurations(&self) -> &[CredentialConfigurationSummary] {
        &self.credential_configurations
    }

    /// Look up a Credential Configuration summary by exact ID.
    pub fn credential_configuration(&self, id: &str) -> Option<&CredentialConfigurationSummary> {
        self.credential_configurations
            .iter()
            .find(|configuration| configuration.id() == id)
    }

    /// Borrow the exact unsigned JSON retained from validation.
    pub fn as_json(&self) -> &str {
        &self.json
    }

    fn accepts_hint(&self, hint: &AuthorizationServerIdentifier) -> bool {
        self.authorization_servers.as_ref().is_some_and(|servers| {
            servers.len() > 1
                && servers
                    .iter()
                    .any(|server| server.as_str() == hint.as_str())
        })
    }

    pub(crate) fn has_effective_authorization_server(&self, issuer: &str) -> bool {
        match &self.authorization_servers {
            Some(servers) => servers.iter().any(|server| server.as_str() == issuer),
            None => self.credential_issuer.as_str() == issuer,
        }
    }
}

impl fmt::Debug for CredentialIssuerMetadata {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialIssuerMetadata")
            .field(
                "authorization_servers_advertised",
                &self.authorization_servers.is_some(),
            )
            .field(
                "authorization_server_count",
                &self.effective_authorization_server_count(),
            )
            .field(
                "credential_configuration_count",
                &self.credential_configurations.len(),
            )
            .finish_non_exhaustive()
    }
}

/// A grant-validated Credential Offer matched to unsigned issuer metadata.
pub struct CredentialOfferWithMetadata {
    offer: CredentialOfferWithGrants,
    metadata: CredentialIssuerMetadata,
}

impl CredentialOfferWithMetadata {
    /// Borrow the grant-validated offer.
    pub const fn credential_offer(&self) -> &CredentialOfferWithGrants {
        &self.offer
    }

    /// Borrow the matched Credential Issuer Metadata.
    pub const fn credential_issuer_metadata(&self) -> &CredentialIssuerMetadata {
        &self.metadata
    }
}

impl fmt::Debug for CredentialOfferWithMetadata {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialOfferWithMetadata")
            .finish_non_exhaustive()
    }
}

impl CredentialOfferWithGrants {
    /// Consume this offer and metadata after validating exact agreement.
    pub fn try_with_metadata(
        self,
        metadata: CredentialIssuerMetadata,
    ) -> Result<CredentialOfferWithMetadata, CredentialOfferError> {
        if self.credential_offer().credential_issuer().as_str()
            != metadata.credential_issuer().as_str()
        {
            return Err(CredentialOfferError::OfferMetadataIssuerMismatch);
        }
        if self
            .credential_offer()
            .credential_configuration_ids()
            .iter()
            .any(|id| metadata.credential_configuration(id.as_str()).is_none())
        {
            return Err(CredentialOfferError::OfferedConfigurationMissing);
        }
        let authorization_hint = self
            .authorization_code()
            .and_then(|grant| grant.authorization_server());
        let pre_authorized_hint = self
            .pre_authorized_code()
            .and_then(|grant| grant.authorization_server());
        if authorization_hint
            .into_iter()
            .chain(pre_authorized_hint)
            .any(|hint| !metadata.accepts_hint(hint))
        {
            return Err(CredentialOfferError::InvalidAuthorizationServerHint);
        }
        Ok(CredentialOfferWithMetadata {
            offer: self,
            metadata,
        })
    }
}
