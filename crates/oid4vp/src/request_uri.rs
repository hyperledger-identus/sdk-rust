use std::fmt;

use identus_openid4vc::{
    RequestRetrievalError, RequestRetrievalInput, RequestRetrievalLimits,
    RequestRetrievalRequest as SharedRetrievalRequest, RequestUriMethod as SharedRequestUriMethod,
    RequestUriReference,
};
use zeroize::Zeroizing;

use crate::{
    Oid4vpError, ReferencedAuthorizationRequest, RequestObjectValidationLimits, RequestUriMethod,
    RequestUriRetrievalLimits, UnverifiedRequestObject,
};

/// Media type required for a Request URI response and Accept header.
pub const REQUEST_OBJECT_MEDIA_TYPE: &str = identus_openid4vc::REQUEST_OBJECT_MEDIA_TYPE;
/// Media type required for an OpenID4VP Request URI POST body.
pub const FORM_MEDIA_TYPE: &str = identus_openid4vc::FORM_MEDIA_TYPE;

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
    shared: SharedRetrievalRequest,
    method: RequestUriMethod,
    client_id_len: usize,
    has_wallet_nonce: bool,
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
        let shared_limits = shared_limits(limits)?;
        let method = shared_method(requested_method);
        let reference =
            RequestUriReference::new(&outer_client_id, &endpoint, method, shared_limits)
                .map_err(map_shared_error)?;
        let shared_input = match input.method {
            RequestUriMethod::Get => RequestRetrievalInput::get(),
            RequestUriMethod::Post => {
                RequestRetrievalInput::post(input.wallet_metadata, input.wallet_nonce)
            }
        };
        let client_id_len = outer_client_id.len();
        let has_wallet_nonce = input.wallet_nonce.is_some();
        let shared = reference
            .prepare(shared_input, shared_limits)
            .map_err(map_shared_error)?;
        Ok(RequestUriRetrievalRequest {
            shared,
            method: requested_method,
            client_id_len,
            has_wallet_nonce,
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
        self.shared.expose_sensitive_endpoint()
    }

    /// Explicitly reveal the bounded POST body; GET returns an empty slice.
    pub fn expose_sensitive_body(&self) -> &[u8] {
        self.shared.expose_sensitive_body()
    }

    /// Encoded request body length.
    pub fn body_len(&self) -> usize {
        self.shared.expose_sensitive_body().len()
    }

    /// Consume this request and bind one HTTP response to an unverified JAR.
    pub fn bind_response(
        self,
        status_code: u16,
        content_type: Option<&str>,
        body: &[u8],
        validation_limits: RequestObjectValidationLimits,
    ) -> Result<UnverifiedRequestObject, Oid4vpError> {
        let retrieved = self
            .shared
            .bind_response(status_code, content_type, body)
            .map_err(map_shared_error)?;
        UnverifiedRequestObject::parse(
            retrieved.expose_sensitive_compact(),
            Zeroizing::new(retrieved.expose_sensitive_client_id().to_owned()),
            retrieved
                .expose_sensitive_wallet_nonce()
                .map(|value| Zeroizing::new(value.to_owned())),
            validation_limits,
        )
    }
}

impl fmt::Debug for RequestUriRetrievalRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RequestUriRetrievalRequest")
            .field("method", &self.method())
            .field("endpoint_len", &self.expose_sensitive_endpoint().len())
            .field("body_len", &self.body_len())
            .field("client_id_len", &self.client_id_len)
            .field("has_wallet_nonce", &self.has_wallet_nonce)
            .finish()
    }
}

fn shared_method(method: RequestUriMethod) -> SharedRequestUriMethod {
    match method {
        RequestUriMethod::Get => SharedRequestUriMethod::Get,
        RequestUriMethod::Post => SharedRequestUriMethod::Post,
    }
}

fn shared_limits(limits: RequestUriRetrievalLimits) -> Result<RequestRetrievalLimits, Oid4vpError> {
    RequestRetrievalLimits::new(
        usize::MAX,
        usize::MAX,
        limits.max_wallet_metadata_bytes(),
        limits.max_wallet_nonce_bytes(),
        limits.max_request_body_bytes(),
        limits.max_content_type_bytes(),
        limits.max_response_body_bytes(),
    )
    .map_err(map_shared_error)
}

fn map_shared_error(error: RequestRetrievalError) -> Oid4vpError {
    match error {
        RequestRetrievalError::InvalidLimits => Oid4vpError::InvalidLimits,
        RequestRetrievalError::InvalidClientId => Oid4vpError::InvalidInvocation,
        RequestRetrievalError::UnsafeEndpoint => Oid4vpError::UnsafeRequestUri,
        RequestRetrievalError::MethodMismatch => Oid4vpError::RetrievalMethodMismatch,
        RequestRetrievalError::WalletMetadataTooLarge => Oid4vpError::WalletMetadataTooLarge,
        RequestRetrievalError::InvalidWalletMetadata => Oid4vpError::InvalidWalletMetadata,
        RequestRetrievalError::WalletNonceTooLarge => Oid4vpError::WalletNonceTooLarge,
        RequestRetrievalError::InvalidWalletNonce => Oid4vpError::InvalidWalletNonce,
        RequestRetrievalError::RequestBodyTooLarge => Oid4vpError::RequestBodyTooLarge,
        RequestRetrievalError::HttpError => Oid4vpError::RequestUriHttpError,
        RequestRetrievalError::InvalidMediaType => Oid4vpError::InvalidRequestObjectMediaType,
        RequestRetrievalError::ContentTypeTooLarge => Oid4vpError::ResponseContentTypeTooLarge,
        RequestRetrievalError::EmptyResponse => Oid4vpError::EmptyRequestObject,
        RequestRetrievalError::ResponseBodyTooLarge => Oid4vpError::ResponseBodyTooLarge,
        RequestRetrievalError::InvalidCompact => Oid4vpError::InvalidRequestObject,
        RequestRetrievalError::UnsupportedEncryptedObject => {
            Oid4vpError::UnsupportedEncryptedRequestObject
        }
        _ => Oid4vpError::InvalidRequestObject,
    }
}
