use std::fmt;

use zeroize::Zeroizing;

use crate::{
    AuthorizationServerIdentifier, AuthorizationServerMetadataLimits, CredentialOfferError,
    json::parse_authorization_server_metadata_fields, semantic::is_valid_https_endpoint,
};

/// RFC 8414 default grant type used when no list is advertised.
pub const AUTHORIZATION_CODE_GRANT_TYPE: &str = "authorization_code";

/// RFC 8414 default grant type used when no list is advertised.
pub const IMPLICIT_GRANT_TYPE: &str = "implicit";

/// OID4VCI 1.0 Final Pre-Authorized Code grant type identifier.
pub const PRE_AUTHORIZED_CODE_GRANT_TYPE: &str =
    "urn:ietf:params:oauth:grant-type:pre-authorized_code";

/// A syntactically validated HTTPS OAuth Authorization Endpoint URL.
pub struct AuthorizationEndpoint {
    value: Zeroizing<String>,
}

impl AuthorizationEndpoint {
    /// Borrow the exact Authorization Endpoint URL.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl fmt::Debug for AuthorizationEndpoint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthorizationEndpoint")
            .finish_non_exhaustive()
    }
}

/// A syntactically validated HTTPS OAuth Token Endpoint URL.
pub struct TokenEndpoint {
    value: Zeroizing<String>,
}

impl TokenEndpoint {
    pub(crate) fn duplicate(&self) -> Self {
        Self {
            value: Zeroizing::new(self.value.to_string()),
        }
    }

    /// Borrow the exact Token Endpoint URL.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl fmt::Debug for TokenEndpoint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TokenEndpoint")
            .finish_non_exhaustive()
    }
}

/// An opaque OAuth grant type identifier advertised by metadata.
pub struct GrantTypeIdentifier {
    value: Zeroizing<String>,
}

impl GrantTypeIdentifier {
    /// Borrow the exact grant type identifier.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl fmt::Debug for GrantTypeIdentifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GrantTypeIdentifier")
            .finish_non_exhaustive()
    }
}

/// Bounded partial projection of OID4VCI Authorization Server Metadata.
///
/// This state proves syntax and an exact expected-issuer comparison only for
/// the fields it exposes. It does not prove complete RFC 8414 conformance,
/// retrieval provenance, server trust, endpoint reachability, or support for
/// an advertised grant.
pub struct AuthorizationServerMetadataCore {
    json: Zeroizing<String>,
    issuer: AuthorizationServerIdentifier,
    authorization_endpoint: Option<AuthorizationEndpoint>,
    token_endpoint: Option<TokenEndpoint>,
    grant_types_supported: Option<Vec<GrantTypeIdentifier>>,
    anonymous_pre_authorized_access: Option<bool>,
}

impl AuthorizationServerMetadataCore {
    /// Parse bounded unsigned JSON and bind its issuer to an expected value.
    pub fn parse(
        json: &str,
        expected_issuer: &str,
        limits: AuthorizationServerMetadataLimits,
    ) -> Result<Self, CredentialOfferError> {
        if json.len() > limits.max_json_bytes() {
            return Err(CredentialOfferError::AuthorizationServerMetadataTooLarge);
        }
        if expected_issuer.len() > limits.max_issuer_bytes() {
            return Err(CredentialOfferError::AuthorizationServerTooLarge);
        }
        let expected = AuthorizationServerIdentifier::try_from_value(Zeroizing::new(
            expected_issuer.to_owned(),
        ))?;
        let json = Zeroizing::new(json.to_owned());
        let fields = parse_authorization_server_metadata_fields(json.as_bytes(), limits)?;
        let issuer = AuthorizationServerIdentifier::try_from_value(fields.issuer)?;
        if issuer.as_str() != expected.as_str() {
            return Err(CredentialOfferError::AuthorizationServerMetadataIssuerMismatch);
        }
        let authorization_endpoint = fields
            .authorization_endpoint
            .map(|value| {
                if is_valid_https_endpoint(value.as_str()) {
                    Ok(AuthorizationEndpoint { value })
                } else {
                    Err(CredentialOfferError::UnsafeAuthorizationEndpoint)
                }
            })
            .transpose()?;
        let token_endpoint = fields
            .token_endpoint
            .map(|value| {
                if is_valid_https_endpoint(value.as_str()) {
                    Ok(TokenEndpoint { value })
                } else {
                    Err(CredentialOfferError::UnsafeTokenEndpoint)
                }
            })
            .transpose()?;
        let grant_types_supported = fields.grant_types_supported.map(|values| {
            values
                .into_iter()
                .map(|value| GrantTypeIdentifier { value })
                .collect()
        });
        Ok(Self {
            json,
            issuer,
            authorization_endpoint,
            token_endpoint,
            grant_types_supported,
            anonymous_pre_authorized_access: fields.anonymous_pre_authorized_access,
        })
    }

    /// Borrow the exact validated Authorization Server issuer identifier.
    pub const fn issuer(&self) -> &AuthorizationServerIdentifier {
        &self.issuer
    }

    /// Borrow the advertised Authorization Endpoint, if present.
    pub const fn authorization_endpoint(&self) -> Option<&AuthorizationEndpoint> {
        self.authorization_endpoint.as_ref()
    }

    /// Borrow the advertised Token Endpoint, if present.
    pub const fn token_endpoint(&self) -> Option<&TokenEndpoint> {
        self.token_endpoint.as_ref()
    }

    /// Borrow explicitly advertised grant types, or `None` when omitted.
    pub fn advertised_grant_types(&self) -> Option<&[GrantTypeIdentifier]> {
        self.grant_types_supported.as_deref()
    }

    /// Return the number of effective grant types.
    pub fn effective_grant_type_count(&self) -> usize {
        self.grant_types_supported.as_ref().map_or(2, Vec::len)
    }

    /// Borrow one effective grant type by index.
    ///
    /// An omitted list uses the RFC 8414 defaults `authorization_code` and
    /// `implicit`, in that order.
    pub fn effective_grant_type(&self, index: usize) -> Option<&str> {
        match &self.grant_types_supported {
            Some(grant_types) => grant_types.get(index).map(GrantTypeIdentifier::as_str),
            None => [AUTHORIZATION_CODE_GRANT_TYPE, IMPLICIT_GRANT_TYPE]
                .get(index)
                .copied(),
        }
    }

    /// Return the advertised anonymous Pre-Authorized Code flag, if present.
    pub const fn advertised_anonymous_pre_authorized_access(&self) -> Option<bool> {
        self.anonymous_pre_authorized_access
    }

    /// Return the OID4VCI defaulted anonymous Pre-Authorized Code flag.
    pub fn effective_anonymous_pre_authorized_access(&self) -> bool {
        self.anonymous_pre_authorized_access.unwrap_or(false)
    }

    /// Borrow the exact unsigned JSON retained from validation.
    pub fn as_json(&self) -> &str {
        &self.json
    }
}

impl fmt::Debug for AuthorizationServerMetadataCore {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthorizationServerMetadataCore")
            .field(
                "authorization_endpoint_present",
                &self.authorization_endpoint.is_some(),
            )
            .field("token_endpoint_present", &self.token_endpoint.is_some())
            .field(
                "grant_types_advertised",
                &self.grant_types_supported.is_some(),
            )
            .field("grant_type_count", &self.effective_grant_type_count())
            .field(
                "anonymous_access_advertised",
                &self.anonymous_pre_authorized_access.is_some(),
            )
            .finish_non_exhaustive()
    }
}
