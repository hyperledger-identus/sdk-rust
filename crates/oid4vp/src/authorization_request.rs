//! Composed OpenID4VP Final authorization and direct-post routing evidence.

use std::fmt;

use fluent_uri::Uri as ParsedUri;
use identus_jose::{JwsAlgorithm, ProtectedHeader};
use serde_json::{Map, Value};
use zeroize::Zeroizing;

use crate::{
    AuthorizationRequestValidationLimits, DcqlLimits, Oid4vpError, ValidatedDcqlQuery,
    VerifiedRequestObject,
};

/// Supported OpenID4VP Authorization Response type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationResponseType {
    /// A Verifiable Presentation token response.
    VpToken,
}

/// Supported OpenID4VP Authorization Response delivery mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationResponseMode {
    /// Form-encoded response sent to the request's response URI.
    DirectPost,
}

/// A signature-proven request with bounded Final authorization and routing.
///
/// This state proves the narrow `vp_token`/`direct_post` request profile and
/// one valid DCQL query. It does not authorize the verifier key, trust or
/// execute the response endpoint, verify credentials, record consent, or
/// construct a presentation response.
pub struct ValidatedAuthorizationRequest {
    response_type: AuthorizationResponseType,
    response_mode: AuthorizationResponseMode,
    protected_header: ProtectedHeader,
    algorithm: JwsAlgorithm,
    client_id: Zeroizing<String>,
    wallet_nonce: Option<Zeroizing<String>>,
    nonce: Zeroizing<String>,
    response_uri: Zeroizing<String>,
    dcql_query: ValidatedDcqlQuery,
}

impl VerifiedRequestObject {
    /// Consume verified JAR evidence into one bounded Final request state.
    ///
    /// # Errors
    ///
    /// Returns a static OID4VP category when required authorization or routing
    /// input is absent, unsupported, malformed, conflicting, or outside the
    /// supplied resource policies.
    pub fn into_authorization_request(
        self,
        routing_limits: AuthorizationRequestValidationLimits,
        dcql_limits: DcqlLimits,
    ) -> Result<ValidatedAuthorizationRequest, Oid4vpError> {
        let parts = self.into_parts();
        let Value::Object(request) = serde_json::from_slice(parts.payload.as_slice())
            .map_err(|_| Oid4vpError::InvalidRequestObjectPayload)?
        else {
            return Err(Oid4vpError::InvalidRequestObjectPayload);
        };
        ValidatedAuthorizationRequest::from_request_map(request, parts, routing_limits, dcql_limits)
    }
}

impl ValidatedAuthorizationRequest {
    fn from_request_map(
        request: Map<String, Value>,
        parts: crate::request_object::VerifiedRequestObjectParts,
        routing_limits: AuthorizationRequestValidationLimits,
        dcql_limits: DcqlLimits,
    ) -> Result<Self, Oid4vpError> {
        let response_type = validate_response_type(&request)?;
        let response_mode = validate_response_mode(&request)?;
        let nonce = validate_nonce(&request, routing_limits)?;
        let response_uri = validate_response_destination(&request, routing_limits)?;
        let dcql_query = ValidatedDcqlQuery::from_request_map(request, dcql_limits)?;

        Ok(Self {
            response_type,
            response_mode,
            protected_header: parts.protected_header,
            algorithm: parts.algorithm,
            client_id: parts.client_id,
            wallet_nonce: parts.wallet_nonce,
            nonce,
            response_uri,
            dcql_query,
        })
    }

    /// Exact supported response type.
    pub const fn response_type(&self) -> AuthorizationResponseType {
        self.response_type
    }

    /// Exact supported response delivery mode.
    pub const fn response_mode(&self) -> AuthorizationResponseMode {
        self.response_mode
    }

    /// Exact algorithm accepted during Request Object verification.
    pub const fn algorithm(&self) -> JwsAlgorithm {
        self.algorithm
    }

    /// Borrow the signature-protected Request Object header.
    pub const fn protected_header(&self) -> &ProtectedHeader {
        &self.protected_header
    }

    /// Explicitly reveal the correlated signed client identifier.
    pub fn expose_sensitive_client_id(&self) -> &str {
        &self.client_id
    }

    /// Explicitly reveal the signed wallet nonce, when present.
    pub fn expose_sensitive_wallet_nonce(&self) -> Option<&str> {
        self.wallet_nonce.as_ref().map(|nonce| nonce.as_str())
    }

    /// Explicitly reveal the authorization-request nonce.
    pub fn expose_sensitive_nonce(&self) -> &str {
        &self.nonce
    }

    /// Explicitly reveal the syntactically validated HTTPS response URI.
    ///
    /// This accessor confers no authority to perform network I/O. Callers own
    /// DNS, address policy, TLS, redirects, timeouts, and SSRF defenses.
    pub fn expose_sensitive_response_uri(&self) -> &str {
        &self.response_uri
    }

    /// Borrow the bounded structurally validated DCQL query.
    pub const fn dcql_query(&self) -> &ValidatedDcqlQuery {
        &self.dcql_query
    }
}

impl fmt::Debug for ValidatedAuthorizationRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ValidatedAuthorizationRequest")
            .field("response_type", &self.response_type)
            .field("response_mode", &self.response_mode)
            .field("algorithm", &self.algorithm)
            .field(
                "has_key_reference",
                &self.protected_header.key_reference().is_some(),
            )
            .field("client_id_len", &self.client_id.len())
            .field("has_wallet_nonce", &self.wallet_nonce.is_some())
            .field("nonce_len", &self.nonce.len())
            .field("response_uri_len", &self.response_uri.len())
            .field(
                "credential_query_count",
                &self.dcql_query.credential_query_count(),
            )
            .finish_non_exhaustive()
    }
}

fn validate_response_type(
    request: &Map<String, Value>,
) -> Result<AuthorizationResponseType, Oid4vpError> {
    let value = request
        .get("response_type")
        .ok_or(Oid4vpError::MissingResponseType)?;
    match value.as_str() {
        Some("vp_token") => Ok(AuthorizationResponseType::VpToken),
        _ => Err(Oid4vpError::UnsupportedResponseType),
    }
}

fn validate_response_mode(
    request: &Map<String, Value>,
) -> Result<AuthorizationResponseMode, Oid4vpError> {
    let value = request
        .get("response_mode")
        .ok_or(Oid4vpError::MissingResponseMode)?;
    match value.as_str() {
        Some("direct_post") => Ok(AuthorizationResponseMode::DirectPost),
        _ => Err(Oid4vpError::UnsupportedResponseMode),
    }
}

fn validate_nonce(
    request: &Map<String, Value>,
    limits: AuthorizationRequestValidationLimits,
) -> Result<Zeroizing<String>, Oid4vpError> {
    let nonce = request
        .get("nonce")
        .ok_or(Oid4vpError::MissingAuthorizationNonce)?
        .as_str()
        .ok_or(Oid4vpError::InvalidAuthorizationNonce)?;
    if nonce.len() > limits.max_nonce_bytes() {
        return Err(Oid4vpError::AuthorizationNonceTooLarge);
    }
    if nonce.is_empty() || !nonce.bytes().all(is_unreserved_ascii) {
        return Err(Oid4vpError::InvalidAuthorizationNonce);
    }
    Ok(Zeroizing::new(nonce.to_owned()))
}

const fn is_unreserved_ascii(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~')
}

fn validate_response_destination(
    request: &Map<String, Value>,
    limits: AuthorizationRequestValidationLimits,
) -> Result<Zeroizing<String>, Oid4vpError> {
    if request.contains_key("redirect_uri") {
        return Err(Oid4vpError::ConflictingResponseDestination);
    }
    let response_uri = request
        .get("response_uri")
        .ok_or(Oid4vpError::MissingResponseUri)?
        .as_str()
        .ok_or(Oid4vpError::UnsafeResponseUri)?;
    if response_uri.len() > limits.max_response_uri_bytes() {
        return Err(Oid4vpError::ResponseUriTooLarge);
    }
    validate_https_response_uri(response_uri)?;
    Ok(Zeroizing::new(response_uri.to_owned()))
}

fn validate_https_response_uri(value: &str) -> Result<(), Oid4vpError> {
    let parsed = ParsedUri::parse(value).map_err(|_| Oid4vpError::UnsafeResponseUri)?;
    let authority = parsed.authority().ok_or(Oid4vpError::UnsafeResponseUri)?;
    if !parsed.scheme().as_str().eq_ignore_ascii_case("https")
        || authority.host().is_empty()
        || authority.userinfo().is_some()
        || parsed.fragment().is_some()
    {
        return Err(Oid4vpError::UnsafeResponseUri);
    }
    Ok(())
}
