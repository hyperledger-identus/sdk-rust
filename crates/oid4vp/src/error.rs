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
}

/// Public error categories for OID4VP invocation parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
}

impl Oid4vpError {
    /// Ordered error inventory frozen by the v1 contract fixture.
    pub const CONTRACT_VARIANTS: [Self; 15] = [
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
    ];

    const fn contract(self) -> (identus_core::ErrorCode, ErrorKind, &'static str) {
        use error_code as code;
        match self {
            Self::InvalidLimits => (
                code::INVALID_LIMITS,
                ErrorKind::InvalidInput,
                "OID4VP resource limits are invalid",
            ),
            Self::InvocationTooLarge => (
                code::INVOCATION_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VP invocation exceeds its byte limit",
            ),
            Self::InvalidInvocation => (
                code::INVALID_INVOCATION,
                ErrorKind::InvalidInput,
                "OID4VP invocation is invalid",
            ),
            Self::TooManyParameters => (
                code::TOO_MANY_PARAMETERS,
                ErrorKind::InvalidInput,
                "OID4VP invocation has too many parameters",
            ),
            Self::InvalidFormEncoding => (
                code::INVALID_FORM_ENCODING,
                ErrorKind::InvalidInput,
                "OID4VP parameter encoding is invalid",
            ),
            Self::ParameterNameTooLarge => (
                code::PARAMETER_NAME_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VP parameter name exceeds its byte limit",
            ),
            Self::ParameterValueTooLarge => (
                code::PARAMETER_VALUE_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VP parameter value exceeds its byte limit",
            ),
            Self::DuplicateParameter => (
                code::DUPLICATE_PARAMETER,
                ErrorKind::InvalidInput,
                "OID4VP invocation contains a duplicate parameter",
            ),
            Self::ClientIdTooLarge => (
                code::CLIENT_ID_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VP client identifier exceeds its byte limit",
            ),
            Self::RequestUriTooLarge => (
                code::REQUEST_URI_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VP request URI exceeds its byte limit",
            ),
            Self::MissingRequiredParameter => (
                code::MISSING_REQUIRED_PARAMETER,
                ErrorKind::InvalidInput,
                "OID4VP invocation is missing a required parameter",
            ),
            Self::UnsupportedTransport => (
                code::UNSUPPORTED_TRANSPORT,
                ErrorKind::Unsupported,
                "OID4VP transport is not supported",
            ),
            Self::UnsupportedParameter => (
                code::UNSUPPORTED_PARAMETER,
                ErrorKind::Unsupported,
                "OID4VP parameter is not supported",
            ),
            Self::UnsupportedRequestUriMethod => (
                code::UNSUPPORTED_REQUEST_URI_METHOD,
                ErrorKind::Unsupported,
                "OID4VP request URI method is not supported",
            ),
            Self::UnsafeRequestUri => (
                code::UNSAFE_REQUEST_URI,
                ErrorKind::InvalidInput,
                "OID4VP request URI is unsafe",
            ),
        }
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
