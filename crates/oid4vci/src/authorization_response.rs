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
    pub(crate) fn into_token_request_parts(
        self,
    ) -> (
        AuthorizationRequest,
        Zeroizing<String>,
        AuthorizationResponseIssuerIdentification,
    ) {
        (*self.request, self.code, self.issuer_identification)
    }

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

impl ResponseFields {
    fn insert(&mut self, name: &str, value: Zeroizing<String>) {
        match name {
            "code" => self.code = Some(value),
            "error" => self.error = Some(value),
            "error_description" => self.error_description = Some(value),
            "error_uri" => self.error_uri = Some(value),
            "state" => self.state = Some(value),
            "iss" => self.issuer = Some(value),
            _ => {}
        }
    }

    fn into_branch(self) -> Result<ResponseBranch, CredentialOfferError> {
        match (self.code, self.error) {
            (Some(code), None) if self.error_description.is_none() && self.error_uri.is_none() => {
                Ok(ResponseBranch::Authorized(code))
            }
            (None, Some(error)) => Ok(ResponseBranch::Error {
                error,
                description: self.error_description,
                uri: self.error_uri,
            }),
            _ => Err(CredentialOfferError::InvalidAuthorizationResponse),
        }
    }
}

enum ResponseBranch {
    Authorized(Zeroizing<String>),
    Error {
        error: Zeroizing<String>,
        description: Option<Zeroizing<String>>,
        uri: Option<Zeroizing<String>>,
    },
}

struct DecodedAuthorizationResponse {
    response_len: usize,
    fields: ResponseFields,
}

struct BoundedAuthorizationResponseQuery<'a> {
    query: &'a str,
    limits: AuthorizationResponseLimits,
    names: BTreeSet<String>,
    fields: ResponseFields,
}

impl<'a> BoundedAuthorizationResponseQuery<'a> {
    fn new(query: &'a str, limits: AuthorizationResponseLimits) -> Self {
        Self {
            query,
            limits,
            names: BTreeSet::new(),
            fields: ResponseFields::default(),
        }
    }

    fn decode(mut self) -> Result<DecodedAuthorizationResponse, CredentialOfferError> {
        self.validate_envelope()?;
        for (index, field) in self.query.split('&').enumerate() {
            self.decode_field(index, field)?;
        }
        Ok(DecodedAuthorizationResponse {
            response_len: self.query.len(),
            fields: self.fields,
        })
    }

    fn validate_envelope(&self) -> Result<(), CredentialOfferError> {
        if self.query.is_empty() || self.query.starts_with('?') || self.query.contains('#') {
            return Err(CredentialOfferError::InvalidAuthorizationResponse);
        }
        if self.query.len() > self.limits.max_query_bytes() {
            return Err(CredentialOfferError::AuthorizationResponseTooLarge);
        }
        Ok(())
    }

    fn decode_field(&mut self, index: usize, field: &str) -> Result<(), CredentialOfferError> {
        if index == self.limits.max_parameters() {
            return Err(CredentialOfferError::TooManyAuthorizationResponseParameters);
        }
        if field.is_empty() {
            return Err(CredentialOfferError::InvalidAuthorizationResponse);
        }
        let (encoded_name, encoded_value) = field
            .split_once('=')
            .ok_or(CredentialOfferError::InvalidAuthorizationResponse)?;
        let name = decode_response_parameter_name(encoded_name, self.limits)?;
        if !self.names.insert(name.to_string()) {
            return Err(CredentialOfferError::DuplicateAuthorizationResponseParameter);
        }
        let value = decode_response_parameter_value(name.as_str(), encoded_value, self.limits)?;
        self.fields.insert(name.as_str(), value);
        Ok(())
    }
}

struct AuthorizationResponseCorrelator {
    request: AuthorizationRequest,
    response: DecodedAuthorizationResponse,
}

impl AuthorizationResponseCorrelator {
    fn correlate(self) -> Result<AuthorizationResponseOutcome, CredentialOfferError> {
        self.verify_state()?;
        let issuer_identification = self.verify_issuer()?;
        let response_len = self.response.response_len;
        match self.response.fields.into_branch()? {
            ResponseBranch::Authorized(code) => {
                validate_authorization_code(&code)?;
                Ok(AuthorizationResponseOutcome::Authorized(
                    CorrelatedAuthorizationCode {
                        request: Box::new(self.request),
                        code,
                        issuer_identification,
                    },
                ))
            }
            ResponseBranch::Error {
                error,
                description,
                uri,
            } => build_error_outcome(response_len, error, description, uri, issuer_identification),
        }
    }

    fn verify_state(&self) -> Result<(), CredentialOfferError> {
        let expected = self.request.authorization_request_input().state().as_str();
        if self
            .response
            .fields
            .state
            .as_ref()
            .map(|value| value.as_str())
            != Some(expected)
        {
            return Err(CredentialOfferError::AuthorizationResponseStateMismatch);
        }
        Ok(())
    }

    fn verify_issuer(
        &self,
    ) -> Result<AuthorizationResponseIssuerIdentification, CredentialOfferError> {
        let metadata = self
            .request
            .authorization_request_input()
            .credential_offer_with_authorization_code_server()
            .authorization_server_metadata();
        if metadata.effective_authorization_response_iss_parameter_supported() {
            if self
                .response
                .fields
                .issuer
                .as_ref()
                .map(|value| value.as_str())
                != Some(metadata.issuer().as_str())
            {
                return Err(CredentialOfferError::AuthorizationResponseIssuerMismatch);
            }
            Ok(AuthorizationResponseIssuerIdentification::VerifiedRfc9207)
        } else if self.response.fields.issuer.is_some() {
            Err(CredentialOfferError::AuthorizationResponseIssuerMismatch)
        } else {
            Ok(AuthorizationResponseIssuerIdentification::NotAdvertised)
        }
    }
}

impl AuthorizationRequest {
    /// Consume this request and correlate an already-extracted callback query.
    pub fn try_into_authorization_response(
        self,
        query: &str,
        limits: AuthorizationResponseLimits,
    ) -> Result<AuthorizationResponseOutcome, CredentialOfferError> {
        let response = BoundedAuthorizationResponseQuery::new(query, limits).decode()?;
        AuthorizationResponseCorrelator {
            request: self,
            response,
        }
        .correlate()
    }
}

fn decode_response_parameter_name(
    encoded_name: &str,
    limits: AuthorizationResponseLimits,
) -> Result<Zeroizing<String>, CredentialOfferError> {
    let name = decode_component(
        encoded_name,
        limits.max_name_bytes(),
        CredentialOfferError::InvalidAuthorizationResponseEncoding,
        CredentialOfferError::AuthorizationResponseComponentTooLarge,
    )?;
    if name.is_empty() {
        return Err(CredentialOfferError::InvalidAuthorizationResponse);
    }
    Ok(name)
}

fn decode_response_parameter_value(
    name: &str,
    encoded_value: &str,
    limits: AuthorizationResponseLimits,
) -> Result<Zeroizing<String>, CredentialOfferError> {
    let (maximum, too_large) = response_parameter_limit(name, limits);
    let value = decode_component(
        encoded_value,
        maximum,
        CredentialOfferError::InvalidAuthorizationResponseEncoding,
        too_large,
    )?;
    Ok(value)
}

fn response_parameter_limit(
    name: &str,
    limits: AuthorizationResponseLimits,
) -> (usize, CredentialOfferError) {
    match name {
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
    }
}

fn validate_authorization_code(code: &str) -> Result<(), CredentialOfferError> {
    if code.is_empty() || !is_vschar(code) {
        return Err(CredentialOfferError::InvalidAuthorizationCode);
    }
    Ok(())
}

fn build_error_outcome(
    response_len: usize,
    error: Zeroizing<String>,
    description: Option<Zeroizing<String>>,
    uri: Option<Zeroizing<String>>,
    issuer_identification: AuthorizationResponseIssuerIdentification,
) -> Result<AuthorizationResponseOutcome, CredentialOfferError> {
    if error.is_empty() || !is_nqschar(&error) {
        return Err(CredentialOfferError::InvalidAuthorizationEndpointErrorCode);
    }
    if description
        .as_ref()
        .is_some_and(|value| value.is_empty() || !is_nqschar(value))
    {
        return Err(CredentialOfferError::InvalidAuthorizationErrorDescription);
    }
    if uri.as_ref().is_some_and(|value| {
        value.is_empty() || !is_uri_reference_chars(value) || UriRef::parse(value.as_str()).is_err()
    }) {
        return Err(CredentialOfferError::InvalidAuthorizationErrorUri);
    }
    Ok(AuthorizationResponseOutcome::Error(
        AuthorizationErrorResponse {
            response_len,
            error: AuthorizationEndpointErrorCode { value: error },
            error_description: description,
            error_uri: uri.map(|value| AuthorizationErrorUri { value }),
            issuer_identification,
        },
    ))
}
