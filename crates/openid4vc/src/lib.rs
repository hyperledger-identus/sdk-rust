//! Runtime-neutral OpenID request-object retrieval primitives.
//!
//! This crate prepares a bounded GET or POST request and binds a successful
//! media-typed response to an opaque compact request object. Protocol crates
//! remain responsible for invocation syntax, JAR verification and claims.

#![forbid(unsafe_code)]

use std::{error::Error, fmt};

use fluent_uri::Uri as ParsedUri;
use identus_core::Component;
use serde_json::Value;
use zeroize::Zeroizing;

/// Media type required for request-object retrieval responses.
pub const REQUEST_OBJECT_MEDIA_TYPE: &str = "application/oauth-authz-req+jwt";
/// Media type used by POST request-object retrieval.
pub const FORM_MEDIA_TYPE: &str = "application/x-www-form-urlencoded";

/// Transport method selected by a protocol-specific invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestUriMethod {
    Get,
    Post,
}

/// Finite resource limits for request-object retrieval.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequestRetrievalLimits {
    max_client_id_bytes: usize,
    max_endpoint_bytes: usize,
    max_wallet_metadata_bytes: usize,
    max_wallet_nonce_bytes: usize,
    max_request_body_bytes: usize,
    max_content_type_bytes: usize,
    max_response_body_bytes: usize,
}

impl RequestRetrievalLimits {
    /// Construct a strictly positive retrieval limit set.
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        max_client_id_bytes: usize,
        max_endpoint_bytes: usize,
        max_wallet_metadata_bytes: usize,
        max_wallet_nonce_bytes: usize,
        max_request_body_bytes: usize,
        max_content_type_bytes: usize,
        max_response_body_bytes: usize,
    ) -> Result<Self, RequestRetrievalError> {
        if max_client_id_bytes == 0
            || max_endpoint_bytes == 0
            || max_wallet_metadata_bytes == 0
            || max_wallet_nonce_bytes == 0
            || max_request_body_bytes == 0
            || max_content_type_bytes == 0
            || max_response_body_bytes == 0
        {
            return Err(RequestRetrievalError::InvalidLimits);
        }
        Ok(Self {
            max_client_id_bytes,
            max_endpoint_bytes,
            max_wallet_metadata_bytes,
            max_wallet_nonce_bytes,
            max_request_body_bytes,
            max_content_type_bytes,
            max_response_body_bytes,
        })
    }

    /// Maximum accepted response-body size in bytes.
    pub const fn max_response_body_bytes(self) -> usize {
        self.max_response_body_bytes
    }
}

impl Default for RequestRetrievalLimits {
    fn default() -> Self {
        Self {
            max_client_id_bytes: 2_048,
            max_endpoint_bytes: 4_096,
            max_wallet_metadata_bytes: 16_384,
            max_wallet_nonce_bytes: 256,
            max_request_body_bytes: 24_576,
            max_content_type_bytes: 128,
            max_response_body_bytes: 65_536,
        }
    }
}

/// Stable failures emitted by the protocol-neutral retrieval boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum RequestRetrievalError {
    InvalidLimits,
    InvalidClientId,
    UnsafeEndpoint,
    MethodMismatch,
    WalletMetadataTooLarge,
    InvalidWalletMetadata,
    WalletNonceTooLarge,
    InvalidWalletNonce,
    RequestBodyTooLarge,
    HttpError,
    InvalidMediaType,
    ContentTypeTooLarge,
    EmptyResponse,
    ResponseBodyTooLarge,
    InvalidCompact,
    UnsupportedEncryptedObject,
}

impl fmt::Display for RequestRetrievalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidLimits => "openid4vc_invalid_limits",
            Self::InvalidClientId => "openid4vc_invalid_client_id",
            Self::UnsafeEndpoint => "openid4vc_unsafe_endpoint",
            Self::MethodMismatch => "openid4vc_method_mismatch",
            Self::WalletMetadataTooLarge => "openid4vc_wallet_metadata_too_large",
            Self::InvalidWalletMetadata => "openid4vc_invalid_wallet_metadata",
            Self::WalletNonceTooLarge => "openid4vc_wallet_nonce_too_large",
            Self::InvalidWalletNonce => "openid4vc_invalid_wallet_nonce",
            Self::RequestBodyTooLarge => "openid4vc_request_body_too_large",
            Self::HttpError => "openid4vc_request_uri_http_error",
            Self::InvalidMediaType => "openid4vc_invalid_request_object_media_type",
            Self::ContentTypeTooLarge => "openid4vc_content_type_too_large",
            Self::EmptyResponse => "openid4vc_empty_request_object",
            Self::ResponseBodyTooLarge => "openid4vc_response_body_too_large",
            Self::InvalidCompact => "openid4vc_invalid_compact_request_object",
            Self::UnsupportedEncryptedObject => "openid4vc_encrypted_request_object_unsupported",
        })
    }
}

impl Error for RequestRetrievalError {}

/// Validated client and HTTPS endpoint reference awaiting request preparation.
pub struct RequestUriReference {
    client_id: Zeroizing<String>,
    endpoint: Zeroizing<String>,
    method: RequestUriMethod,
}

impl RequestUriReference {
    /// Validate a protocol-owned client identifier and safe HTTPS endpoint.
    pub fn new(
        client_id: &str,
        endpoint: &str,
        method: RequestUriMethod,
        limits: RequestRetrievalLimits,
    ) -> Result<Self, RequestRetrievalError> {
        if client_id.chars().any(char::is_control) {
            return Err(RequestRetrievalError::InvalidClientId);
        }
        Self::new_with_prevalidated_client_id(client_id, endpoint, method, limits)
    }

    /// Bind a client identifier already validated by its protocol owner.
    ///
    /// This retains the shared size bound but deliberately does not impose a
    /// second protocol-specific character policy. Callers MUST validate the
    /// identifier before selecting this constructor.
    pub fn new_with_prevalidated_client_id(
        client_id: &str,
        endpoint: &str,
        method: RequestUriMethod,
        limits: RequestRetrievalLimits,
    ) -> Result<Self, RequestRetrievalError> {
        if client_id.is_empty() || client_id.len() > limits.max_client_id_bytes {
            return Err(RequestRetrievalError::InvalidClientId);
        }
        if endpoint.len() > limits.max_endpoint_bytes || !safe_https(endpoint) {
            return Err(RequestRetrievalError::UnsafeEndpoint);
        }
        Ok(Self {
            client_id: Zeroizing::new(client_id.to_owned()),
            endpoint: Zeroizing::new(endpoint.to_owned()),
            method,
        })
    }

    /// Consume the reference into exact inert transport request data.
    pub fn prepare(
        self,
        input: RequestRetrievalInput<'_>,
        limits: RequestRetrievalLimits,
    ) -> Result<RequestRetrievalRequest, RequestRetrievalError> {
        if self.method != input.method {
            return Err(RequestRetrievalError::MethodMismatch);
        }
        let prepared = match input.method {
            RequestUriMethod::Get => PreparedForm {
                body: Zeroizing::new(Vec::new()),
                wallet_nonce: None,
            },
            RequestUriMethod::Post => {
                prepare_form(input.wallet_metadata, input.wallet_nonce, limits)?
            }
        };
        Ok(RequestRetrievalRequest {
            client_id: self.client_id,
            endpoint: self.endpoint,
            method: self.method,
            body: prepared.body,
            wallet_nonce: prepared.wallet_nonce,
            limits,
        })
    }
}

/// Caller-provided data for the selected retrieval method.
pub struct RequestRetrievalInput<'a> {
    method: RequestUriMethod,
    wallet_metadata: Option<&'a str>,
    wallet_nonce: Option<&'a str>,
}

impl<'a> RequestRetrievalInput<'a> {
    /// Select GET retrieval with no request body.
    pub const fn get() -> Self {
        Self {
            method: RequestUriMethod::Get,
            wallet_metadata: None,
            wallet_nonce: None,
        }
    }

    /// Select POST retrieval with optional bounded wallet metadata and nonce.
    pub const fn post(wallet_metadata: Option<&'a str>, wallet_nonce: Option<&'a str>) -> Self {
        Self {
            method: RequestUriMethod::Post,
            wallet_metadata,
            wallet_nonce,
        }
    }
}

/// Exact inert request data for a caller-owned HTTP adapter.
pub struct RequestRetrievalRequest {
    client_id: Zeroizing<String>,
    endpoint: Zeroizing<String>,
    method: RequestUriMethod,
    body: Zeroizing<Vec<u8>>,
    wallet_nonce: Option<Zeroizing<String>>,
    limits: RequestRetrievalLimits,
}

impl RequestRetrievalRequest {
    /// Selected HTTP method.
    pub const fn method(&self) -> RequestUriMethod {
        self.method
    }
    /// Required static `Accept` header value.
    pub const fn accept(&self) -> &'static str {
        REQUEST_OBJECT_MEDIA_TYPE
    }
    /// POST content type, or `None` for GET.
    pub const fn content_type(&self) -> Option<&'static str> {
        match self.method {
            RequestUriMethod::Get => None,
            RequestUriMethod::Post => Some(FORM_MEDIA_TYPE),
        }
    }
    /// Explicitly reveal the validated HTTPS endpoint.
    pub fn expose_sensitive_endpoint(&self) -> &str {
        &self.endpoint
    }
    /// Explicitly reveal the bounded request body.
    pub fn expose_sensitive_body(&self) -> &[u8] {
        &self.body
    }

    /// Consume this request and bind a successful typed response to it.
    pub fn bind_response(
        self,
        status: u16,
        content_type: Option<&str>,
        body: &[u8],
    ) -> Result<RetrievedRequestObject, RequestRetrievalError> {
        if !(200..=299).contains(&status) {
            return Err(RequestRetrievalError::HttpError);
        }
        let media = content_type.ok_or(RequestRetrievalError::InvalidMediaType)?;
        if media.len() > self.limits.max_content_type_bytes {
            return Err(RequestRetrievalError::ContentTypeTooLarge);
        }
        if !media.trim().eq_ignore_ascii_case(REQUEST_OBJECT_MEDIA_TYPE) {
            return Err(RequestRetrievalError::InvalidMediaType);
        }
        if body.is_empty() {
            return Err(RequestRetrievalError::EmptyResponse);
        }
        if body.len() > self.limits.max_response_body_bytes {
            return Err(RequestRetrievalError::ResponseBodyTooLarge);
        }
        let compact =
            std::str::from_utf8(body).map_err(|_| RequestRetrievalError::InvalidCompact)?;
        match compact.split('.').count() {
            3 => {}
            5 => return Err(RequestRetrievalError::UnsupportedEncryptedObject),
            _ => return Err(RequestRetrievalError::InvalidCompact),
        }
        Ok(RetrievedRequestObject {
            compact: Zeroizing::new(compact.to_owned()),
            client_id: self.client_id,
            wallet_nonce: self.wallet_nonce,
        })
    }
}

/// Retrieved compact request object bound to the originating request evidence.
pub struct RetrievedRequestObject {
    compact: Zeroizing<String>,
    client_id: Zeroizing<String>,
    wallet_nonce: Option<Zeroizing<String>>,
}

impl RetrievedRequestObject {
    /// Explicitly reveal the compact request object for protocol validation.
    pub fn expose_sensitive_compact(&self) -> &str {
        &self.compact
    }
    /// Explicitly reveal the outer client identifier for JAR binding.
    pub fn expose_sensitive_client_id(&self) -> &str {
        &self.client_id
    }
    /// Explicitly reveal the sent wallet nonce for request-object correlation.
    pub fn expose_sensitive_wallet_nonce(&self) -> Option<&str> {
        self.wallet_nonce.as_deref().map(String::as_str)
    }
}

fn safe_https(value: &str) -> bool {
    ParsedUri::parse(value).is_ok_and(|uri| {
        uri.scheme().as_str().eq_ignore_ascii_case("https")
            && uri.authority().is_some_and(|authority| {
                !authority.host().is_empty() && authority.userinfo().is_none()
            })
            && uri.fragment().is_none()
    })
}

struct PreparedForm {
    body: Zeroizing<Vec<u8>>,
    wallet_nonce: Option<Zeroizing<String>>,
}

fn prepare_form(
    metadata: Option<&str>,
    nonce: Option<&str>,
    limits: RequestRetrievalLimits,
) -> Result<PreparedForm, RequestRetrievalError> {
    if let Some(value) = metadata
        && value.len() > limits.max_wallet_metadata_bytes
    {
        return Err(RequestRetrievalError::WalletMetadataTooLarge);
    }
    if let Some(value) = metadata
        && serde_json::from_str::<Value>(value)
            .ok()
            .is_none_or(|json| !json.is_object())
    {
        return Err(RequestRetrievalError::InvalidWalletMetadata);
    }
    if nonce.is_some_and(|value| value.len() > limits.max_wallet_nonce_bytes) {
        return Err(RequestRetrievalError::WalletNonceTooLarge);
    }
    if nonce.is_some_and(|value| value.is_empty() || value.chars().any(char::is_control)) {
        return Err(RequestRetrievalError::InvalidWalletNonce);
    }
    let mut body = Zeroizing::new(Vec::new());
    if let Some(value) = metadata {
        append_pair(
            &mut body,
            b"wallet_metadata",
            value.as_bytes(),
            limits.max_request_body_bytes,
        )?;
    }
    if let Some(value) = nonce {
        append_pair(
            &mut body,
            b"wallet_nonce",
            value.as_bytes(),
            limits.max_request_body_bytes,
        )?;
    }
    Ok(PreparedForm {
        body,
        wallet_nonce: nonce.map(|value| Zeroizing::new(value.to_owned())),
    })
}

fn append_pair(
    output: &mut Vec<u8>,
    name: &[u8],
    value: &[u8],
    maximum: usize,
) -> Result<(), RequestRetrievalError> {
    if !output.is_empty() {
        push(output, b'&', maximum)?;
    }
    for byte in name {
        push(output, *byte, maximum)?;
    }
    push(output, b'=', maximum)?;
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    for byte in value {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
            push(output, *byte, maximum)?;
        } else {
            push(output, b'%', maximum)?;
            push(output, HEX[(byte >> 4) as usize], maximum)?;
            push(output, HEX[(byte & 15) as usize], maximum)?;
        }
    }
    Ok(())
}

fn push(output: &mut Vec<u8>, byte: u8, maximum: usize) -> Result<(), RequestRetrievalError> {
    if output.len() >= maximum {
        return Err(RequestRetrievalError::RequestBodyTooLarge);
    }
    output.push(byte);
    Ok(())
}

/// Metadata for the protocol-neutral request retrieval component.
pub const COMPONENT: Component = Component {
    name: "identus-openid4vc",
    summary: "Bounded runtime-neutral OpenID request-object retrieval primitives.",
};
