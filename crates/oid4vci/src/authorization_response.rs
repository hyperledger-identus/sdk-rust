use std::{collections::BTreeSet, fmt};

use fluent_uri::UriRef;
use zeroize::Zeroizing;

use crate::{
    AuthorizationRequest, CredentialOfferError,
    form::decode_component,
    oauth::{is_nqschar, is_uri_reference_chars, is_vschar},
};

/// Positive resource limits for an already-extracted Authorization Response query.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthorizationResponseLimits {
    max_query_bytes: usize,
    max_parameters: usize,
    max_name_bytes: usize,
    max_value_bytes: usize,
    max_code_bytes: usize,
    max_error_bytes: usize,
    max_error_description_bytes: usize,
    max_error_uri_bytes: usize,
}

impl AuthorizationResponseLimits {
    /// Construct a positive Authorization Response resource policy.
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        max_query_bytes: usize,
        max_parameters: usize,
        max_name_bytes: usize,
        max_value_bytes: usize,
        max_code_bytes: usize,
        max_error_bytes: usize,
        max_error_description_bytes: usize,
        max_error_uri_bytes: usize,
    ) -> Result<Self, CredentialOfferError> {
        if max_query_bytes == 0
            || max_parameters == 0
            || max_name_bytes == 0
            || max_value_bytes == 0
            || max_code_bytes == 0
            || max_error_bytes == 0
            || max_error_description_bytes == 0
            || max_error_uri_bytes == 0
        {
            return Err(CredentialOfferError::InvalidAuthorizationResponseLimits);
        }
        Ok(Self {
            max_query_bytes,
            max_parameters,
            max_name_bytes,
            max_value_bytes,
            max_code_bytes,
            max_error_bytes,
            max_error_description_bytes,
            max_error_uri_bytes,
        })
    }

    /// Maximum encoded query bytes.
    pub const fn max_query_bytes(self) -> usize {
        self.max_query_bytes
    }
    /// Maximum decoded parameter count.
    pub const fn max_parameters(self) -> usize {
        self.max_parameters
    }
    /// Maximum decoded parameter-name bytes.
    pub const fn max_name_bytes(self) -> usize {
        self.max_name_bytes
    }
    /// Maximum decoded bytes in state, issuer or an extension value.
    pub const fn max_value_bytes(self) -> usize {
        self.max_value_bytes
    }
    /// Maximum decoded authorization-code bytes.
    pub const fn max_code_bytes(self) -> usize {
        self.max_code_bytes
    }
    /// Maximum decoded OAuth error-code bytes.
    pub const fn max_error_bytes(self) -> usize {
        self.max_error_bytes
    }
    /// Maximum decoded OAuth error-description bytes.
    pub const fn max_error_description_bytes(self) -> usize {
        self.max_error_description_bytes
    }
    /// Maximum decoded OAuth error URI-reference bytes.
    pub const fn max_error_uri_bytes(self) -> usize {
        self.max_error_uri_bytes
    }
}

impl Default for AuthorizationResponseLimits {
    fn default() -> Self {
        Self {
            max_query_bytes: 16_384,
            max_parameters: 32,
            max_name_bytes: 256,
            max_value_bytes: 4_096,
            max_code_bytes: 4_096,
            max_error_bytes: 256,
            max_error_description_bytes: 4_096,
            max_error_uri_bytes: 2_048,
        }
    }
}

/// Evidence retained about Authorization Response issuer identification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationResponseIssuerIdentification {
    /// The exact selected Authorization Server issuer was verified under RFC 9207.
    VerifiedRfc9207,
    /// RFC 9207 support was not advertised; this is not mix-up protection.
    NotAdvertised,
}

/// Closed classification of OAuth Authorization Endpoint errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationEndpointErrorKind {
    InvalidRequest,
    UnauthorizedClient,
    AccessDenied,
    UnsupportedResponseType,
    InvalidScope,
    ServerError,
    TemporarilyUnavailable,
    Extension,
}

/// A validated exact OAuth Authorization Endpoint error code.
pub struct AuthorizationEndpointErrorCode {
    value: Zeroizing<String>,
}

impl AuthorizationEndpointErrorCode {
    /// Borrow the exact case-sensitive error code.
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Classify standard RFC 6749 error codes without assigning policy to extensions.
    pub fn kind(&self) -> AuthorizationEndpointErrorKind {
        match self.value.as_str() {
            "invalid_request" => AuthorizationEndpointErrorKind::InvalidRequest,
            "unauthorized_client" => AuthorizationEndpointErrorKind::UnauthorizedClient,
            "access_denied" => AuthorizationEndpointErrorKind::AccessDenied,
            "unsupported_response_type" => AuthorizationEndpointErrorKind::UnsupportedResponseType,
            "invalid_scope" => AuthorizationEndpointErrorKind::InvalidScope,
            "server_error" => AuthorizationEndpointErrorKind::ServerError,
            "temporarily_unavailable" => AuthorizationEndpointErrorKind::TemporarilyUnavailable,
            _ => AuthorizationEndpointErrorKind::Extension,
        }
    }
}

impl fmt::Debug for AuthorizationEndpointErrorCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthorizationEndpointErrorCode")
            .field("kind", &self.kind())
            .finish_non_exhaustive()
    }
}

/// A validated but untrusted Authorization Endpoint error URI-reference.
pub struct AuthorizationErrorUri {
    value: Zeroizing<String>,
}

impl AuthorizationErrorUri {
    /// Borrow the exact URI-reference without resolving or dereferencing it.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl fmt::Debug for AuthorizationErrorUri {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthorizationErrorUri")
            .finish_non_exhaustive()
    }
}

/// A correlated authorization code retaining complete code-exchange lineage.
pub struct CorrelatedAuthorizationCode {
    request: Box<AuthorizationRequest>,
    code: Zeroizing<String>,
    issuer_identification: AuthorizationResponseIssuerIdentification,
}

impl CorrelatedAuthorizationCode {
    /// Borrow the consumed request lineage needed by a later code exchange.
    pub const fn authorization_request(&self) -> &AuthorizationRequest {
        &self.request
    }
    /// Borrow the sensitive exact authorization code for immediate protocol use.
    pub fn expose_sensitive_code(&self) -> &str {
        &self.code
    }
    /// Return the issuer-identification evidence attached to the response.
    pub const fn issuer_identification(&self) -> AuthorizationResponseIssuerIdentification {
        self.issuer_identification
    }
}

impl fmt::Debug for CorrelatedAuthorizationCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CorrelatedAuthorizationCode")
            .field("issuer_identification", &self.issuer_identification)
            .finish_non_exhaustive()
    }
}

/// A correlated OAuth Authorization Endpoint error response.
pub struct AuthorizationErrorResponse {
    response_len: usize,
    error: AuthorizationEndpointErrorCode,
    error_description: Option<Zeroizing<String>>,
    error_uri: Option<AuthorizationErrorUri>,
    issuer_identification: AuthorizationResponseIssuerIdentification,
}

impl AuthorizationErrorResponse {
    /// Return the encoded response query byte count.
    pub const fn response_len(&self) -> usize {
        self.response_len
    }
    /// Borrow the exact validated error code.
    pub const fn error(&self) -> &AuthorizationEndpointErrorCode {
        &self.error
    }
    /// Return the closed classification of the exact error code.
    pub fn error_kind(&self) -> AuthorizationEndpointErrorKind {
        self.error.kind()
    }
    /// Report whether developer-oriented information is present.
    pub const fn error_description_present(&self) -> bool {
        self.error_description.is_some()
    }
    /// Borrow untrusted developer-oriented information, if present.
    pub fn expose_untrusted_description(&self) -> Option<&str> {
        self.error_description.as_ref().map(|value| value.as_str())
    }
    /// Borrow the validated URI-reference without resolving or following it.
    pub const fn error_uri(&self) -> Option<&AuthorizationErrorUri> {
        self.error_uri.as_ref()
    }
    /// Return the issuer-identification evidence attached to the response.
    pub const fn issuer_identification(&self) -> AuthorizationResponseIssuerIdentification {
        self.issuer_identification
    }
}

impl fmt::Debug for AuthorizationErrorResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthorizationErrorResponse")
            .field("response_bytes", &self.response_len)
            .field("error_kind", &self.error.kind())
            .field(
                "error_description_present",
                &self.error_description.is_some(),
            )
            .field("error_uri_present", &self.error_uri.is_some())
            .field("issuer_identification", &self.issuer_identification)
            .finish_non_exhaustive()
    }
}

/// Exclusive correlated Authorization Response outcome.
pub enum AuthorizationResponseOutcome {
    Authorized(CorrelatedAuthorizationCode),
    Error(AuthorizationErrorResponse),
}

impl fmt::Debug for AuthorizationResponseOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Authorized(value) => formatter.debug_tuple("Authorized").field(value).finish(),
            Self::Error(value) => formatter.debug_tuple("Error").field(value).finish(),
        }
    }
}

#[derive(Default)]
struct ResponseFields {
    code: Option<Zeroizing<String>>,
    error: Option<Zeroizing<String>>,
    error_description: Option<Zeroizing<String>>,
    error_uri: Option<Zeroizing<String>>,
    state: Option<Zeroizing<String>>,
    issuer: Option<Zeroizing<String>>,
}

impl AuthorizationRequest {
    /// Consume this request and correlate an already-extracted callback query.
    pub fn try_into_authorization_response(
        self,
        query: &str,
        limits: AuthorizationResponseLimits,
    ) -> Result<AuthorizationResponseOutcome, CredentialOfferError> {
        let expected_state = self.authorization_request_input().state().as_str();
        let metadata = self
            .authorization_request_input()
            .credential_offer_with_authorization_code_server()
            .authorization_server_metadata();
        let issuer_supported = metadata.effective_authorization_response_iss_parameter_supported();
        let expected_issuer = metadata.issuer().as_str();
        let query_len = query.len();
        let fields = parse_query(query, limits)?;

        if fields.state.as_ref().map(|value| value.as_str()) != Some(expected_state) {
            return Err(CredentialOfferError::AuthorizationResponseStateMismatch);
        }
        let issuer_identification = if issuer_supported {
            if fields.issuer.as_ref().map(|value| value.as_str()) != Some(expected_issuer) {
                return Err(CredentialOfferError::AuthorizationResponseIssuerMismatch);
            }
            AuthorizationResponseIssuerIdentification::VerifiedRfc9207
        } else {
            if fields.issuer.is_some() {
                return Err(CredentialOfferError::AuthorizationResponseIssuerMismatch);
            }
            AuthorizationResponseIssuerIdentification::NotAdvertised
        };

        match (fields.code, fields.error) {
            (Some(code), None)
                if fields.error_description.is_none() && fields.error_uri.is_none() =>
            {
                if code.is_empty() || !is_vschar(&code) {
                    return Err(CredentialOfferError::InvalidAuthorizationCode);
                }
                Ok(AuthorizationResponseOutcome::Authorized(
                    CorrelatedAuthorizationCode {
                        request: Box::new(self),
                        code,
                        issuer_identification,
                    },
                ))
            }
            (None, Some(error)) => {
                if error.is_empty() || !is_nqschar(&error) {
                    return Err(CredentialOfferError::InvalidAuthorizationEndpointErrorCode);
                }
                if fields
                    .error_description
                    .as_ref()
                    .is_some_and(|value| value.is_empty() || !is_nqschar(value))
                {
                    return Err(CredentialOfferError::InvalidAuthorizationErrorDescription);
                }
                if fields.error_uri.as_ref().is_some_and(|value| {
                    value.is_empty()
                        || !is_uri_reference_chars(value)
                        || UriRef::parse(value.as_str()).is_err()
                }) {
                    return Err(CredentialOfferError::InvalidAuthorizationErrorUri);
                }
                Ok(AuthorizationResponseOutcome::Error(
                    AuthorizationErrorResponse {
                        response_len: query_len,
                        error: AuthorizationEndpointErrorCode { value: error },
                        error_description: fields.error_description,
                        error_uri: fields
                            .error_uri
                            .map(|value| AuthorizationErrorUri { value }),
                        issuer_identification,
                    },
                ))
            }
            _ => Err(CredentialOfferError::InvalidAuthorizationResponse),
        }
    }
}

fn parse_query(
    query: &str,
    limits: AuthorizationResponseLimits,
) -> Result<ResponseFields, CredentialOfferError> {
    if query.is_empty() || query.starts_with('?') || query.contains('#') {
        return Err(CredentialOfferError::InvalidAuthorizationResponse);
    }
    if query.len() > limits.max_query_bytes() {
        return Err(CredentialOfferError::AuthorizationResponseTooLarge);
    }

    let mut names = BTreeSet::new();
    let mut fields = ResponseFields::default();
    for (index, field) in query.split('&').enumerate() {
        if index == limits.max_parameters() {
            return Err(CredentialOfferError::TooManyAuthorizationResponseParameters);
        }
        if field.is_empty() {
            return Err(CredentialOfferError::InvalidAuthorizationResponse);
        }
        let (encoded_name, encoded_value) = field
            .split_once('=')
            .ok_or(CredentialOfferError::InvalidAuthorizationResponse)?;
        let name = decode_component(
            encoded_name,
            limits.max_name_bytes(),
            CredentialOfferError::InvalidAuthorizationResponseEncoding,
            CredentialOfferError::AuthorizationResponseComponentTooLarge,
        )?;
        if name.is_empty() {
            return Err(CredentialOfferError::InvalidAuthorizationResponse);
        }
        if !names.insert(name.to_string()) {
            return Err(CredentialOfferError::DuplicateAuthorizationResponseParameter);
        }
        let (maximum, too_large) = match name.as_str() {
            "code" => (
                limits.max_code_bytes(),
                CredentialOfferError::AuthorizationCodeTooLarge,
            ),
            "error" => (
                limits.max_error_bytes(),
                CredentialOfferError::AuthorizationEndpointErrorCodeTooLarge,
            ),
            "error_description" => (
                limits.max_error_description_bytes(),
                CredentialOfferError::AuthorizationErrorDescriptionTooLarge,
            ),
            "error_uri" => (
                limits.max_error_uri_bytes(),
                CredentialOfferError::AuthorizationErrorUriTooLarge,
            ),
            _ => (
                limits.max_value_bytes(),
                CredentialOfferError::AuthorizationResponseComponentTooLarge,
            ),
        };
        let value = decode_component(
            encoded_value,
            maximum,
            CredentialOfferError::InvalidAuthorizationResponseEncoding,
            too_large,
        )?;
        match name.as_str() {
            "code" => fields.code = Some(value),
            "error" => fields.error = Some(value),
            "error_description" => fields.error_description = Some(value),
            "error_uri" => fields.error_uri = Some(value),
            "state" => fields.state = Some(value),
            "iss" => fields.issuer = Some(value),
            _ => {}
        }
    }
    Ok(fields)
}
