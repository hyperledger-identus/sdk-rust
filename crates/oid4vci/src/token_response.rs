use std::fmt;

use fluent_uri::UriRef;
use zeroize::Zeroizing;

use crate::{
    CredentialOfferError, TokenAuthorizationDetailsLimits, TokenResponseLimits,
    json::{parse_token_authorization_details_fields, parse_token_response_fields},
};

/// A validated OAuth access-token type.
pub struct TokenType {
    value: Zeroizing<String>,
}

impl TokenType {
    /// Borrow the exact token-type spelling from the response.
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Compare token types using the RFC-required ASCII case insensitivity.
    pub fn eq_ignore_ascii_case(&self, other: &str) -> bool {
        self.value.eq_ignore_ascii_case(other)
    }
}

impl fmt::Debug for TokenType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("TokenType").finish_non_exhaustive()
    }
}

/// A bounded partial successful Token Response.
///
/// This state proves only OAuth core syntax. It does not prove transport
/// provenance, token trust or freshness, or Authorization Details semantics.
pub struct TokenResponseCore {
    json: Zeroizing<String>,
    access_token: Zeroizing<String>,
    token_type: TokenType,
    expires_in: Option<u64>,
    refresh_token: Option<Zeroizing<String>>,
    scope: Option<Zeroizing<String>>,
    authorization_details_present: bool,
    parse_limits: TokenResponseLimits,
}

impl TokenResponseCore {
    /// Parse a successful Token Response core under explicit resource limits.
    ///
    /// The caller retains responsibility for erasing its input allocation.
    pub fn parse(json: &str, limits: TokenResponseLimits) -> Result<Self, CredentialOfferError> {
        if json.len() > limits.max_json_bytes() {
            return Err(CredentialOfferError::TokenResponseTooLarge);
        }
        let json = Zeroizing::new(json.to_owned());
        let fields = parse_token_response_fields(json.as_bytes(), limits)?;
        if !is_visible_ascii(&fields.access_token) {
            return Err(CredentialOfferError::InvalidAccessToken);
        }
        if !is_valid_token_type(&fields.token_type) {
            return Err(CredentialOfferError::InvalidTokenType);
        }
        if fields
            .refresh_token
            .as_ref()
            .is_some_and(|value| !is_visible_ascii(value))
        {
            return Err(CredentialOfferError::InvalidRefreshToken);
        }
        if fields
            .scope
            .as_ref()
            .is_some_and(|value| !is_valid_scope(value))
        {
            return Err(CredentialOfferError::InvalidTokenScope);
        }
        Ok(Self {
            json,
            access_token: fields.access_token,
            token_type: TokenType {
                value: fields.token_type,
            },
            expires_in: fields.expires_in,
            refresh_token: fields.refresh_token,
            scope: fields.scope,
            authorization_details_present: fields.authorization_details_present,
            parse_limits: limits,
        })
    }

    /// Return the exact response byte count.
    pub fn response_len(&self) -> usize {
        self.json.len()
    }

    /// Borrow the validated token type.
    pub const fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    /// Return the advertised access-token lifetime in seconds, if present.
    pub const fn expires_in_seconds(&self) -> Option<u64> {
        self.expires_in
    }

    /// Report whether a refresh token is present.
    pub const fn refresh_token_present(&self) -> bool {
        self.refresh_token.is_some()
    }

    /// Report whether a scope is present.
    pub const fn scope_present(&self) -> bool {
        self.scope.is_some()
    }

    /// Report whether unvalidated Authorization Details are present.
    pub const fn authorization_details_present(&self) -> bool {
        self.authorization_details_present
    }

    /// Validate credential Authorization Details through an explicit state transition.
    ///
    /// This consumes the presence-only core so a caller cannot confuse partial
    /// OAuth parsing with validated OpenID4VCI credential dataset identifiers.
    pub fn try_validate_authorization_details(
        self,
        limits: TokenAuthorizationDetailsLimits,
    ) -> Result<TokenResponseWithAuthorizationDetails, CredentialOfferError> {
        let fields = parse_token_authorization_details_fields(
            self.json.as_bytes(),
            self.parse_limits,
            limits,
        )?;
        let credential_details = fields
            .credential_details
            .into_iter()
            .map(|fields| CredentialAuthorizationDetail {
                credential_configuration_id: fields.credential_configuration_id,
                credential_identifiers: fields.credential_identifiers,
            })
            .collect();
        Ok(TokenResponseWithAuthorizationDetails {
            core: self,
            credential_details,
            unknown_type_count: fields.unknown_type_count,
        })
    }

    /// Borrow the access token for immediate protected-resource use.
    ///
    /// Keep the value out of logs, URLs, telemetry, caches, generic
    /// serializers, and long-lived storage.
    pub fn expose_sensitive_access_token(&self) -> &str {
        &self.access_token
    }

    /// Borrow the refresh token, if present, for an explicitly authorized flow.
    ///
    /// Keep the value out of logs, URLs, telemetry, caches, generic
    /// serializers, and long-lived storage.
    pub fn expose_sensitive_refresh_token(&self) -> Option<&str> {
        self.refresh_token.as_ref().map(|value| value.as_str())
    }

    /// Borrow the exact scope, if present.
    ///
    /// Scope can disclose authorization context. Keep it out of logs,
    /// telemetry, URLs, caches, and unrelated storage.
    pub fn expose_sensitive_scope(&self) -> Option<&str> {
        self.scope.as_ref().map(|value| value.as_str())
    }
}

/// One validated `openid_credential` authorization-detail entry.
pub struct CredentialAuthorizationDetail {
    credential_configuration_id: Zeroizing<String>,
    credential_identifiers: Vec<Zeroizing<String>>,
}

impl CredentialAuthorizationDetail {
    /// Borrow the Credential Configuration ID referenced by this entry.
    pub fn credential_configuration_id(&self) -> &str {
        &self.credential_configuration_id
    }

    /// Return the number of authorized Credential Dataset identifiers.
    pub fn credential_identifier_count(&self) -> usize {
        self.credential_identifiers.len()
    }

    /// Iterate over the authorized Credential Dataset identifiers.
    pub fn credential_identifiers(&self) -> impl ExactSizeIterator<Item = &str> {
        self.credential_identifiers
            .iter()
            .map(|value| value.as_str())
    }
}

impl fmt::Debug for CredentialAuthorizationDetail {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialAuthorizationDetail")
            .field(
                "credential_identifier_count",
                &self.credential_identifiers.len(),
            )
            .finish_non_exhaustive()
    }
}

/// A successful Token Response with validated credential Authorization Details.
///
/// This state proves syntax, bounds and identifier uniqueness only. It does
/// not establish token, issuer, metadata, dataset or Credential trust.
pub struct TokenResponseWithAuthorizationDetails {
    core: TokenResponseCore,
    credential_details: Vec<CredentialAuthorizationDetail>,
    unknown_type_count: usize,
}

impl TokenResponseWithAuthorizationDetails {
    /// Borrow the validated OAuth Token Response core.
    pub const fn token_response_core(&self) -> &TokenResponseCore {
        &self.core
    }

    /// Borrow recognized credential Authorization Details in source order.
    pub fn credential_authorization_details(&self) -> &[CredentialAuthorizationDetail] {
        &self.credential_details
    }

    /// Return the number of bounded unsupported authorization-detail types.
    pub const fn unknown_authorization_detail_count(&self) -> usize {
        self.unknown_type_count
    }
}

impl fmt::Debug for TokenResponseWithAuthorizationDetails {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TokenResponseWithAuthorizationDetails")
            .field("credential_detail_count", &self.credential_details.len())
            .field("unknown_type_count", &self.unknown_type_count)
            .finish_non_exhaustive()
    }
}

impl fmt::Debug for TokenResponseCore {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TokenResponseCore")
            .field("response_bytes", &self.json.len())
            .field("expires_in_present", &self.expires_in.is_some())
            .field("refresh_token_present", &self.refresh_token.is_some())
            .field("scope_present", &self.scope.is_some())
            .field(
                "authorization_details_present",
                &self.authorization_details_present,
            )
            .finish_non_exhaustive()
    }
}

fn is_visible_ascii(value: &str) -> bool {
    value.bytes().all(|byte| matches!(byte, 0x20..=0x7e))
}

fn is_valid_token_type(value: &str) -> bool {
    let is_type_name = value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_'));
    is_type_name || UriRef::parse(value).is_ok()
}

fn is_valid_scope(value: &str) -> bool {
    let mut previous_was_space = true;
    for byte in value.bytes() {
        if byte == b' ' {
            if previous_was_space {
                return false;
            }
            previous_was_space = true;
        } else if matches!(byte, 0x21 | 0x23..=0x5b | 0x5d..=0x7e) {
            previous_was_space = false;
        } else {
            return false;
        }
    }
    !previous_was_space
}
