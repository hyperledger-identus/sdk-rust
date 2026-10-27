//! Composed OpenID4VP Final authorization and direct-post routing evidence.

use std::fmt;

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use fluent_uri::Uri as ParsedUri;
use identus_jose::{
    JweContentEncryptionAlgorithm, JweKeyManagementAlgorithm, JweLimits, JweRecipientPublicKey,
    JwsAlgorithm, ProtectedHeader,
};
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
#[non_exhaustive]
pub enum AuthorizationResponseMode {
    /// Form-encoded response sent to the request's response URI.
    DirectPost,
    /// Form-encoded response carrying one unsigned encrypted JWT.
    DirectPostJwt,
}

pub(crate) struct DirectPostEncryptionPlan {
    pub(crate) recipient: JweRecipientPublicKey,
    pub(crate) key_management: JweKeyManagementAlgorithm,
    pub(crate) content_encryption: JweContentEncryptionAlgorithm,
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
    state: Option<Zeroizing<String>>,
    response_uri: Zeroizing<String>,
    dcql_query: ValidatedDcqlQuery,
    encryption_plan: Option<DirectPostEncryptionPlan>,
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
        self.into_authorization_request_with_jwe_limits(
            routing_limits,
            dcql_limits,
            JweLimits::default(),
        )
    }

    /// Consume verified JAR evidence with an explicit encrypted-response policy.
    pub fn into_authorization_request_with_jwe_limits(
        self,
        routing_limits: AuthorizationRequestValidationLimits,
        dcql_limits: DcqlLimits,
        jwe_limits: JweLimits,
    ) -> Result<ValidatedAuthorizationRequest, Oid4vpError> {
        let parts = self.into_parts();
        let Value::Object(request) = serde_json::from_slice(parts.payload.as_slice())
            .map_err(|_| Oid4vpError::InvalidRequestObjectPayload)?
        else {
            return Err(Oid4vpError::InvalidRequestObjectPayload);
        };
        ValidatedAuthorizationRequest::from_request_map(
            request,
            parts,
            routing_limits,
            dcql_limits,
            jwe_limits,
        )
    }
}

impl ValidatedAuthorizationRequest {
    fn from_request_map(
        request: Map<String, Value>,
        parts: crate::request_object::VerifiedRequestObjectParts,
        routing_limits: AuthorizationRequestValidationLimits,
        dcql_limits: DcqlLimits,
        jwe_limits: JweLimits,
    ) -> Result<Self, Oid4vpError> {
        let response_type = validate_response_type(&request)?;
        let response_mode = validate_response_mode(&request)?;
        let nonce = validate_nonce(&request, routing_limits)?;
        let state = validate_state(&request, routing_limits)?;
        let response_uri = validate_response_destination(&request, routing_limits)?;
        let encryption_plan = validate_encryption_plan(&request, response_mode, jwe_limits)?;
        let dcql_query = ValidatedDcqlQuery::from_request_map(request, dcql_limits)?;

        Ok(Self {
            response_type,
            response_mode,
            protected_header: parts.protected_header,
            algorithm: parts.algorithm,
            client_id: parts.client_id,
            wallet_nonce: parts.wallet_nonce,
            nonce,
            state,
            response_uri,
            dcql_query,
            encryption_plan,
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

    /// Explicitly reveal the exact signed OAuth state, when present.
    pub fn expose_sensitive_state(&self) -> Option<&str> {
        self.state.as_ref().map(|state| state.as_str())
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

    pub(crate) fn into_direct_post_parts(
        self,
    ) -> (
        Zeroizing<String>,
        Option<Zeroizing<String>>,
        ValidatedDcqlQuery,
        AuthorizationResponseMode,
        Option<DirectPostEncryptionPlan>,
    ) {
        (
            self.response_uri,
            self.state,
            self.dcql_query,
            self.response_mode,
            self.encryption_plan,
        )
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
            .field("has_state", &self.state.is_some())
            .field("response_uri_len", &self.response_uri.len())
            .field(
                "credential_query_count",
                &self.dcql_query.credential_query_count(),
            )
            .field("has_encryption_plan", &self.encryption_plan.is_some())
            .finish_non_exhaustive()
    }
}

fn validate_state(
    request: &Map<String, Value>,
    limits: AuthorizationRequestValidationLimits,
) -> Result<Option<Zeroizing<String>>, Oid4vpError> {
    let Some(value) = request.get("state") else {
        return Ok(None);
    };
    let state = value.as_str().ok_or(Oid4vpError::InvalidState)?;
    if state.len() > limits.max_state_bytes() {
        return Err(Oid4vpError::StateTooLarge);
    }
    if state.is_empty() || !state.bytes().all(|byte| matches!(byte, 0x20..=0x7e)) {
        return Err(Oid4vpError::InvalidState);
    }
    Ok(Some(Zeroizing::new(state.to_owned())))
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
        Some("direct_post.jwt") => Ok(AuthorizationResponseMode::DirectPostJwt),
        _ => Err(Oid4vpError::UnsupportedResponseMode),
    }
}

fn validate_encryption_plan(
    request: &Map<String, Value>,
    mode: AuthorizationResponseMode,
    limits: JweLimits,
) -> Result<Option<DirectPostEncryptionPlan>, Oid4vpError> {
    if mode == AuthorizationResponseMode::DirectPost {
        return Ok(None);
    }
    let metadata = request
        .get("client_metadata")
        .and_then(Value::as_object)
        .ok_or(Oid4vpError::InvalidRequestObjectPayload)?;
    validate_content_encryption(metadata)?;
    let keys = metadata
        .get("jwks")
        .and_then(Value::as_object)
        .and_then(|jwks| jwks.get("keys"))
        .and_then(Value::as_array)
        .ok_or(Oid4vpError::InvalidRequestObjectPayload)?;
    if keys.is_empty() || keys.len() > 16 {
        return Err(Oid4vpError::InvalidRequestObjectPayload);
    }

    let mut selected = None;
    for value in keys {
        let key = value
            .as_object()
            .ok_or(Oid4vpError::InvalidRequestObjectPayload)?;
        if contains_private_jwk_material(key) {
            return Err(Oid4vpError::InvalidRequestObjectPayload);
        }
        if !is_supported_recipient(key)? {
            continue;
        }
        if selected.is_some() {
            return Err(Oid4vpError::InvalidRequestObjectPayload);
        }
        let x = decode_coordinate(key.get("x"))?;
        let y = decode_coordinate(key.get("y"))?;
        let kid = key
            .get("kid")
            .map(|value| {
                value
                    .as_str()
                    .ok_or(Oid4vpError::InvalidRequestObjectPayload)
            })
            .transpose()?;
        selected = Some(
            JweRecipientPublicKey::p256_coordinates(&x, &y, kid, limits)
                .map_err(|_| Oid4vpError::InvalidRequestObjectPayload)?,
        );
    }
    let recipient = selected.ok_or(Oid4vpError::InvalidRequestObjectPayload)?;
    Ok(Some(DirectPostEncryptionPlan {
        recipient,
        key_management: JweKeyManagementAlgorithm::EcdhEs,
        content_encryption: JweContentEncryptionAlgorithm::A128Gcm,
    }))
}

fn validate_content_encryption(metadata: &Map<String, Value>) -> Result<(), Oid4vpError> {
    let Some(values) = metadata.get("encrypted_response_enc_values_supported") else {
        return Ok(());
    };
    let values = values
        .as_array()
        .ok_or(Oid4vpError::InvalidRequestObjectPayload)?;
    if values.is_empty() || values.len() > 16 {
        return Err(Oid4vpError::InvalidRequestObjectPayload);
    }
    let mut supported = 0_usize;
    let mut seen = std::collections::BTreeSet::new();
    for value in values {
        let value = value
            .as_str()
            .ok_or(Oid4vpError::InvalidRequestObjectPayload)?;
        if value.is_empty() || !seen.insert(value) {
            return Err(Oid4vpError::InvalidRequestObjectPayload);
        }
        supported += usize::from(value == "A128GCM");
    }
    if supported != 1 {
        return Err(Oid4vpError::UnsupportedResponseMode);
    }
    Ok(())
}

fn contains_private_jwk_material(key: &Map<String, Value>) -> bool {
    ["d", "p", "q", "dp", "dq", "qi", "oth", "k"]
        .iter()
        .any(|name| key.contains_key(*name))
}

fn is_supported_recipient(key: &Map<String, Value>) -> Result<bool, Oid4vpError> {
    if key.get("kty").and_then(Value::as_str) != Some("EC")
        || key.get("crv").and_then(Value::as_str) != Some("P-256")
        || key.get("alg").and_then(Value::as_str) != Some("ECDH-ES")
    {
        return Ok(false);
    }
    if key
        .get("use")
        .is_some_and(|value| value.as_str() != Some("enc"))
    {
        return Ok(false);
    }
    if let Some(operations) = key.get("key_ops") {
        let operations = operations
            .as_array()
            .ok_or(Oid4vpError::InvalidRequestObjectPayload)?;
        let compatible = operations
            .iter()
            .any(|value| matches!(value.as_str(), Some("deriveKey" | "deriveBits")));
        if !compatible {
            return Ok(false);
        }
    }
    Ok(true)
}

fn decode_coordinate(value: Option<&Value>) -> Result<[u8; 32], Oid4vpError> {
    let encoded = value
        .and_then(Value::as_str)
        .ok_or(Oid4vpError::InvalidRequestObjectPayload)?;
    let decoded = URL_SAFE_NO_PAD
        .decode(encoded)
        .map_err(|_| Oid4vpError::InvalidRequestObjectPayload)?;
    decoded
        .try_into()
        .map_err(|_| Oid4vpError::InvalidRequestObjectPayload)
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
