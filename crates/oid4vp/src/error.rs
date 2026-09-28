//! Static, redaction-safe OID4VP ingress errors.

use std::fmt;

use identus_core::{CapabilityId, ErrorKind, IdentusError};

/// Owning capability for OID4VP errors.
pub const CAPABILITY: CapabilityId = CapabilityId::new("oid4vp");

/// Stable OID4VP error codes used at the shared SDK boundary.
pub mod error_code {
    use identus_core::ErrorCode;

    pub const INVALID_LIMITS: ErrorCode = ErrorCode::new("oid4vp.invalid_limits");
    pub const INVOCATION_TOO_LARGE: ErrorCode = ErrorCode::new("oid4vp.invocation_too_large");
    pub const INVALID_INVOCATION: ErrorCode = ErrorCode::new("oid4vp.invalid_invocation");
    pub const TOO_MANY_PARAMETERS: ErrorCode = ErrorCode::new("oid4vp.too_many_parameters");
    pub const INVALID_FORM_ENCODING: ErrorCode = ErrorCode::new("oid4vp.invalid_form_encoding");
    pub const PARAMETER_NAME_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vp.parameter_name_too_large");
    pub const PARAMETER_VALUE_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vp.parameter_value_too_large");
    pub const DUPLICATE_PARAMETER: ErrorCode = ErrorCode::new("oid4vp.duplicate_parameter");
    pub const CLIENT_ID_TOO_LARGE: ErrorCode = ErrorCode::new("oid4vp.client_id_too_large");
    pub const REQUEST_URI_TOO_LARGE: ErrorCode = ErrorCode::new("oid4vp.request_uri_too_large");
    pub const MISSING_REQUIRED_PARAMETER: ErrorCode =
        ErrorCode::new("oid4vp.missing_required_parameter");
    pub const UNSUPPORTED_TRANSPORT: ErrorCode = ErrorCode::new("oid4vp.unsupported_transport");
    pub const UNSUPPORTED_PARAMETER: ErrorCode = ErrorCode::new("oid4vp.unsupported_parameter");
    pub const UNSUPPORTED_REQUEST_URI_METHOD: ErrorCode =
        ErrorCode::new("oid4vp.unsupported_request_uri_method");
    pub const UNSAFE_REQUEST_URI: ErrorCode = ErrorCode::new("oid4vp.unsafe_request_uri");
    pub const RETRIEVAL_METHOD_MISMATCH: ErrorCode =
        ErrorCode::new("oid4vp.retrieval_method_mismatch");
    pub const WALLET_METADATA_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vp.wallet_metadata_too_large");
    pub const INVALID_WALLET_METADATA: ErrorCode = ErrorCode::new("oid4vp.invalid_wallet_metadata");
    pub const WALLET_NONCE_TOO_LARGE: ErrorCode = ErrorCode::new("oid4vp.wallet_nonce_too_large");
    pub const INVALID_WALLET_NONCE: ErrorCode = ErrorCode::new("oid4vp.invalid_wallet_nonce");
    pub const REQUEST_BODY_TOO_LARGE: ErrorCode = ErrorCode::new("oid4vp.request_body_too_large");
    pub const RESPONSE_CONTENT_TYPE_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vp.response_content_type_too_large");
    pub const RESPONSE_BODY_TOO_LARGE: ErrorCode = ErrorCode::new("oid4vp.response_body_too_large");
    pub const REQUEST_URI_HTTP_ERROR: ErrorCode = ErrorCode::new("oid4vp.request_uri_http_error");
    pub const INVALID_REQUEST_OBJECT_MEDIA_TYPE: ErrorCode =
        ErrorCode::new("oid4vp.invalid_request_object_media_type");
    pub const EMPTY_REQUEST_OBJECT: ErrorCode = ErrorCode::new("oid4vp.empty_request_object");
    pub const UNSUPPORTED_ENCRYPTED_REQUEST_OBJECT: ErrorCode =
        ErrorCode::new("oid4vp.unsupported_encrypted_request_object");
    pub const INVALID_REQUEST_OBJECT: ErrorCode = ErrorCode::new("oid4vp.invalid_request_object");
    pub const INVALID_REQUEST_OBJECT_TYPE: ErrorCode =
        ErrorCode::new("oid4vp.invalid_request_object_type");
    pub const INVALID_REQUEST_OBJECT_SIGNATURE: ErrorCode =
        ErrorCode::new("oid4vp.invalid_request_object_signature");
    pub const INVALID_REQUEST_OBJECT_PAYLOAD: ErrorCode =
        ErrorCode::new("oid4vp.invalid_request_object_payload");
    pub const REQUEST_OBJECT_CLIENT_ID_MISMATCH: ErrorCode =
        ErrorCode::new("oid4vp.request_object_client_id_mismatch");
    pub const REQUEST_OBJECT_WALLET_NONCE_MISMATCH: ErrorCode =
        ErrorCode::new("oid4vp.request_object_wallet_nonce_mismatch");
}

/// Public error categories for OID4VP invocation parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Oid4vpError {
    InvalidLimits,
    InvocationTooLarge,
    InvalidInvocation,
    TooManyParameters,
    InvalidFormEncoding,
    ParameterNameTooLarge,
    ParameterValueTooLarge,
    DuplicateParameter,
    ClientIdTooLarge,
    RequestUriTooLarge,
    MissingRequiredParameter,
    UnsupportedTransport,
    UnsupportedParameter,
    UnsupportedRequestUriMethod,
    UnsafeRequestUri,
    RetrievalMethodMismatch,
    WalletMetadataTooLarge,
    InvalidWalletMetadata,
    WalletNonceTooLarge,
    InvalidWalletNonce,
    RequestBodyTooLarge,
    ResponseContentTypeTooLarge,
    ResponseBodyTooLarge,
    RequestUriHttpError,
    InvalidRequestObjectMediaType,
    EmptyRequestObject,
    UnsupportedEncryptedRequestObject,
    InvalidRequestObject,
    InvalidRequestObjectType,
    InvalidRequestObjectSignature,
    InvalidRequestObjectPayload,
    RequestObjectClientIdMismatch,
    RequestObjectWalletNonceMismatch,
}

impl Oid4vpError {
    /// Ordered error inventory frozen by the v1 contract fixture.
    pub const CONTRACT_VARIANTS: [Self; 33] = [
        Self::InvalidLimits,
        Self::InvocationTooLarge,
        Self::InvalidInvocation,
        Self::TooManyParameters,
        Self::InvalidFormEncoding,
        Self::ParameterNameTooLarge,
        Self::ParameterValueTooLarge,
        Self::DuplicateParameter,
        Self::ClientIdTooLarge,
        Self::RequestUriTooLarge,
        Self::MissingRequiredParameter,
        Self::UnsupportedTransport,
        Self::UnsupportedParameter,
        Self::UnsupportedRequestUriMethod,
        Self::UnsafeRequestUri,
        Self::RetrievalMethodMismatch,
        Self::WalletMetadataTooLarge,
        Self::InvalidWalletMetadata,
        Self::WalletNonceTooLarge,
        Self::InvalidWalletNonce,
        Self::RequestBodyTooLarge,
        Self::ResponseContentTypeTooLarge,
        Self::ResponseBodyTooLarge,
        Self::RequestUriHttpError,
        Self::InvalidRequestObjectMediaType,
        Self::EmptyRequestObject,
        Self::UnsupportedEncryptedRequestObject,
        Self::InvalidRequestObject,
        Self::InvalidRequestObjectType,
        Self::InvalidRequestObjectSignature,
        Self::InvalidRequestObjectPayload,
        Self::RequestObjectClientIdMismatch,
        Self::RequestObjectWalletNonceMismatch,
    ];

    const CONTRACTS: [(identus_core::ErrorCode, ErrorKind, &'static str); 33] = {
        use error_code as code;
        [
            (
                code::INVALID_LIMITS,
                ErrorKind::InvalidInput,
                "OID4VP resource limits are invalid",
            ),
            (
                code::INVOCATION_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VP invocation exceeds its byte limit",
            ),
            (
                code::INVALID_INVOCATION,
                ErrorKind::InvalidInput,
                "OID4VP invocation is invalid",
            ),
            (
                code::TOO_MANY_PARAMETERS,
                ErrorKind::InvalidInput,
                "OID4VP invocation has too many parameters",
            ),
            (
                code::INVALID_FORM_ENCODING,
                ErrorKind::InvalidInput,
                "OID4VP parameter encoding is invalid",
            ),
            (
                code::PARAMETER_NAME_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VP parameter name exceeds its byte limit",
            ),
            (
                code::PARAMETER_VALUE_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VP parameter value exceeds its byte limit",
            ),
            (
                code::DUPLICATE_PARAMETER,
                ErrorKind::InvalidInput,
                "OID4VP invocation contains a duplicate parameter",
            ),
            (
                code::CLIENT_ID_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VP client identifier exceeds its byte limit",
            ),
            (
                code::REQUEST_URI_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VP request URI exceeds its byte limit",
            ),
            (
                code::MISSING_REQUIRED_PARAMETER,
                ErrorKind::InvalidInput,
                "OID4VP invocation is missing a required parameter",
            ),
            (
                code::UNSUPPORTED_TRANSPORT,
                ErrorKind::Unsupported,
                "OID4VP transport is not supported",
            ),
            (
                code::UNSUPPORTED_PARAMETER,
                ErrorKind::Unsupported,
                "OID4VP parameter is not supported",
            ),
            (
                code::UNSUPPORTED_REQUEST_URI_METHOD,
                ErrorKind::Unsupported,
                "OID4VP request URI method is not supported",
            ),
            (
                code::UNSAFE_REQUEST_URI,
                ErrorKind::InvalidInput,
                "OID4VP request URI is unsafe",
            ),
            (
                code::RETRIEVAL_METHOD_MISMATCH,
                ErrorKind::InvalidInput,
                "OID4VP retrieval input does not match the requested method",
            ),
            (
                code::WALLET_METADATA_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VP wallet metadata exceeds its byte limit",
            ),
            (
                code::INVALID_WALLET_METADATA,
                ErrorKind::InvalidInput,
                "OID4VP wallet metadata is invalid",
            ),
            (
                code::WALLET_NONCE_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VP wallet nonce exceeds its byte limit",
            ),
            (
                code::INVALID_WALLET_NONCE,
                ErrorKind::InvalidInput,
                "OID4VP wallet nonce is invalid",
            ),
            (
                code::REQUEST_BODY_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VP retrieval request body exceeds its byte limit",
            ),
            (
                code::RESPONSE_CONTENT_TYPE_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VP response content type exceeds its byte limit",
            ),
            (
                code::RESPONSE_BODY_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VP response body exceeds its byte limit",
            ),
            (
                code::REQUEST_URI_HTTP_ERROR,
                ErrorKind::InvalidInput,
                "OID4VP Request URI returned an HTTP error",
            ),
            (
                code::INVALID_REQUEST_OBJECT_MEDIA_TYPE,
                ErrorKind::InvalidInput,
                "OID4VP Request Object media type is invalid",
            ),
            (
                code::EMPTY_REQUEST_OBJECT,
                ErrorKind::InvalidInput,
                "OID4VP Request Object is empty",
            ),
            (
                code::UNSUPPORTED_ENCRYPTED_REQUEST_OBJECT,
                ErrorKind::Unsupported,
                "Encrypted OID4VP Request Objects are not supported",
            ),
            (
                code::INVALID_REQUEST_OBJECT,
                ErrorKind::InvalidInput,
                "OID4VP Request Object is invalid",
            ),
            (
                code::INVALID_REQUEST_OBJECT_TYPE,
                ErrorKind::InvalidInput,
                "OID4VP Request Object type is invalid",
            ),
            (
                code::INVALID_REQUEST_OBJECT_SIGNATURE,
                ErrorKind::InvalidInput,
                "OID4VP Request Object signature is invalid",
            ),
            (
                code::INVALID_REQUEST_OBJECT_PAYLOAD,
                ErrorKind::InvalidInput,
                "OID4VP Request Object payload is invalid",
            ),
            (
                code::REQUEST_OBJECT_CLIENT_ID_MISMATCH,
                ErrorKind::InvalidInput,
                "OID4VP Request Object client identifier does not match",
            ),
            (
                code::REQUEST_OBJECT_WALLET_NONCE_MISMATCH,
                ErrorKind::InvalidInput,
                "OID4VP Request Object wallet nonce does not match",
            ),
        ]
    };

    const fn contract(self) -> (identus_core::ErrorCode, ErrorKind, &'static str) {
        Self::CONTRACTS[self as usize]
    }

    /// Convert to the shared, stable and redaction-safe SDK error.
    pub const fn to_identus_error(self) -> IdentusError {
        let (code, kind, message) = self.contract();
        IdentusError::public(code, kind, CAPABILITY, message)
    }
}

impl fmt::Display for Oid4vpError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.contract().2)
    }
}

impl std::error::Error for Oid4vpError {}

impl From<Oid4vpError> for IdentusError {
    fn from(error: Oid4vpError) -> Self {
        error.to_identus_error()
    }
}
