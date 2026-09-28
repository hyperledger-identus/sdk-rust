use std::fmt;

use serde_json::Value;
use zeroize::Zeroizing;

use crate::{
    Oid4vpError, ReferencedAuthorizationRequest, RequestObjectValidationLimits, RequestUriMethod,
    RequestUriRetrievalLimits, UnverifiedRequestObject,
};

/// Media type required for a Request URI response and Accept header.
pub const REQUEST_OBJECT_MEDIA_TYPE: &str = "application/oauth-authz-req+jwt";
/// Media type required for an OpenID4VP Request URI POST body.
pub const FORM_MEDIA_TYPE: &str = "application/x-www-form-urlencoded";

/// Caller-selected data for one exact retrieval method.
pub struct RequestUriRetrievalInput<'input> {
    method: RequestUriMethod,
    wallet_metadata: Option<&'input str>,
    wallet_nonce: Option<&'input str>,
}

impl<'input> RequestUriRetrievalInput<'input> {
    /// Select GET retrieval, which has no form body.
    #[must_use]
    pub const fn get() -> Self {
        Self {
            method: RequestUriMethod::Get,
            wallet_metadata: None,
            wallet_nonce: None,
        }
    }

    /// Select POST retrieval with optional caller-owned wallet values.
    ///
    /// The SDK does not generate nonce entropy or interpret metadata policy.
    #[must_use]
    pub const fn post(
        wallet_metadata: Option<&'input str>,
        wallet_nonce: Option<&'input str>,
    ) -> Self {
        Self {
            method: RequestUriMethod::Post,
            wallet_metadata,
            wallet_nonce,
        }
    }
}

impl fmt::Debug for RequestUriRetrievalInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RequestUriRetrievalInput")
            .field("method", &self.method)
            .field(
                "wallet_metadata_len",
                &self.wallet_metadata.map_or(0, str::len),
            )
            .field("wallet_nonce_len", &self.wallet_nonce.map_or(0, str::len))
            .finish()
    }
}

/// Runtime-neutral, bounded Request URI HTTP request data.
pub struct RequestUriRetrievalRequest {
    endpoint: Zeroizing<String>,
    method: RequestUriMethod,
    body: Zeroizing<Vec<u8>>,
    outer_client_id: Zeroizing<String>,
    sent_wallet_nonce: Option<Zeroizing<String>>,
    limits: RequestUriRetrievalLimits,
}

impl ReferencedAuthorizationRequest {
    /// Consume this reference into exact bounded HTTP request data.
    pub fn prepare_retrieval(
        self,
        input: RequestUriRetrievalInput<'_>,
        limits: RequestUriRetrievalLimits,
    ) -> Result<RequestUriRetrievalRequest, Oid4vpError> {
        let (outer_client_id, endpoint, requested_method) = self.into_parts();
        if input.method != requested_method {
            return Err(Oid4vpError::RetrievalMethodMismatch);
        }

        let prepared = match input.method {
            RequestUriMethod::Get => PreparedPostBody {
                body: Zeroizing::new(Vec::new()),
                sent_wallet_nonce: None,
            },
            RequestUriMethod::Post => {
                prepare_post_body(input.wallet_metadata, input.wallet_nonce, limits)?
            }
        };

        Ok(RequestUriRetrievalRequest {
            endpoint,
            method: requested_method,
            body: prepared.body,
            outer_client_id,
            sent_wallet_nonce: prepared.sent_wallet_nonce,
            limits,
        })
    }
}

impl RequestUriRetrievalRequest {
    /// HTTP method selected by the validated invocation.
    pub const fn method(&self) -> RequestUriMethod {
        self.method
    }

    /// Required static Accept header value.
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

    /// Explicitly reveal the validated HTTPS endpoint to a transport adapter.
    pub fn expose_sensitive_endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Explicitly reveal the bounded POST body; GET returns an empty slice.
    pub fn expose_sensitive_body(&self) -> &[u8] {
        &self.body
    }

    /// Encoded request body length.
    pub fn body_len(&self) -> usize {
        self.body.len()
    }

    /// Consume this request and bind one HTTP response to an unverified JAR.
    pub fn bind_response(
        self,
        status_code: u16,
        content_type: Option<&str>,
        body: &[u8],
        validation_limits: RequestObjectValidationLimits,
    ) -> Result<UnverifiedRequestObject, Oid4vpError> {
        if !(200..=299).contains(&status_code) {
            return Err(Oid4vpError::RequestUriHttpError);
        }
        let content_type = content_type.ok_or(Oid4vpError::InvalidRequestObjectMediaType)?;
        if content_type.len() > self.limits.max_content_type_bytes() {
            return Err(Oid4vpError::ResponseContentTypeTooLarge);
        }
        if !content_type
            .trim()
            .eq_ignore_ascii_case(REQUEST_OBJECT_MEDIA_TYPE)
        {
            return Err(Oid4vpError::InvalidRequestObjectMediaType);
        }
        if body.is_empty() {
            return Err(Oid4vpError::EmptyRequestObject);
        }
        if body.len() > self.limits.max_response_body_bytes() {
            return Err(Oid4vpError::ResponseBodyTooLarge);
        }
        let compact = std::str::from_utf8(body).map_err(|_| Oid4vpError::InvalidRequestObject)?;
        match compact.split('.').count() {
            5 => return Err(Oid4vpError::UnsupportedEncryptedRequestObject),
            3 => {}
            _ => return Err(Oid4vpError::InvalidRequestObject),
        }
        UnverifiedRequestObject::parse(
            compact,
            self.outer_client_id,
            self.sent_wallet_nonce,
            validation_limits,
        )
    }
}

impl fmt::Debug for RequestUriRetrievalRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RequestUriRetrievalRequest")
            .field("method", &self.method)
            .field("endpoint_len", &self.endpoint.len())
            .field("body_len", &self.body.len())
            .field("client_id_len", &self.outer_client_id.len())
            .field("has_wallet_nonce", &self.sent_wallet_nonce.is_some())
            .finish()
    }
}

struct PreparedPostBody {
    body: Zeroizing<Vec<u8>>,
    sent_wallet_nonce: Option<Zeroizing<String>>,
}

fn prepare_post_body(
    wallet_metadata: Option<&str>,
    wallet_nonce: Option<&str>,
    limits: RequestUriRetrievalLimits,
) -> Result<PreparedPostBody, Oid4vpError> {
    if let Some(metadata) = wallet_metadata {
        if metadata.len() > limits.max_wallet_metadata_bytes() {
            return Err(Oid4vpError::WalletMetadataTooLarge);
        }
        let value: Value =
            serde_json::from_str(metadata).map_err(|_| Oid4vpError::InvalidWalletMetadata)?;
        if !value.is_object() {
            return Err(Oid4vpError::InvalidWalletMetadata);
        }
    }
    let sent_wallet_nonce = wallet_nonce
        .map(|nonce| {
            if nonce.len() > limits.max_wallet_nonce_bytes() {
                return Err(Oid4vpError::WalletNonceTooLarge);
            }
            if nonce.is_empty() || nonce.chars().any(char::is_control) {
                return Err(Oid4vpError::InvalidWalletNonce);
            }
            Ok(Zeroizing::new(nonce.to_owned()))
        })
        .transpose()?;

    let mut body = Zeroizing::new(Vec::with_capacity(
        limits.max_request_body_bytes().min(1_024),
    ));
    if let Some(metadata) = wallet_metadata {
        append_form_pair(
            &mut body,
            b"wallet_metadata",
            metadata.as_bytes(),
            limits.max_request_body_bytes(),
        )?;
    }
    if let Some(nonce) = wallet_nonce {
        append_form_pair(
            &mut body,
            b"wallet_nonce",
            nonce.as_bytes(),
            limits.max_request_body_bytes(),
        )?;
    }
    Ok(PreparedPostBody {
        body,
        sent_wallet_nonce,
    })
}

fn append_form_pair(
    output: &mut Vec<u8>,
    name: &[u8],
    value: &[u8],
    maximum: usize,
) -> Result<(), Oid4vpError> {
    if !output.is_empty() {
        push_bounded(output, b'&', maximum)?;
    }
    for byte in name {
        push_bounded(output, *byte, maximum)?;
    }
    push_bounded(output, b'=', maximum)?;
    for byte in value {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
            push_bounded(output, *byte, maximum)?;
        } else {
            push_bounded(output, b'%', maximum)?;
            push_bounded(output, HEX[(byte >> 4) as usize], maximum)?;
            push_bounded(output, HEX[(byte & 0x0f) as usize], maximum)?;
        }
    }
    Ok(())
}

fn push_bounded(output: &mut Vec<u8>, byte: u8, maximum: usize) -> Result<(), Oid4vpError> {
    if output.len() == maximum {
        return Err(Oid4vpError::RequestBodyTooLarge);
    }
    output.push(byte);
    Ok(())
}

const HEX: &[u8; 16] = b"0123456789ABCDEF";
