use std::{error::Error, fmt};

/// Stable SIOPv2 protocol failure categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Siopv2Error {
    InvalidLimits,
    RequestObjectTooLarge,
    InvalidRequestObject,
    InvalidRequestObjectType,
    InvalidRequestSignature,
    InvalidRequestClaims,
    UnsupportedProfile,
    ClientBindingMismatch,
    WalletNonceMismatch,
    InvalidRedirectUri,
    InvalidTimeWindow,
    RequestExpired,
    InvalidSubject,
    InvalidKeyId,
    AlgorithmMismatch,
    ResponseTooLarge,
    ResponseEncodingFailed,
    SigningRejected,
    SignerUnavailable,
    InvalidEventId,
    InvalidTransition,
    EventConflict,
}

impl Siopv2Error {
    /// Stable SDK error code without sensitive context.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::InvalidLimits => "siopv2_invalid_limits",
            Self::RequestObjectTooLarge => "siopv2_request_object_too_large",
            Self::InvalidRequestObject => "siopv2_invalid_request_object",
            Self::InvalidRequestObjectType => "siopv2_invalid_request_object_type",
            Self::InvalidRequestSignature => "siopv2_invalid_request_signature",
            Self::InvalidRequestClaims => "siopv2_invalid_request_claims",
            Self::UnsupportedProfile => "siopv2_unsupported_profile",
            Self::ClientBindingMismatch => "siopv2_client_binding_mismatch",
            Self::WalletNonceMismatch => "siopv2_wallet_nonce_mismatch",
            Self::InvalidRedirectUri => "siopv2_invalid_redirect_uri",
            Self::InvalidTimeWindow => "siopv2_invalid_time_window",
            Self::RequestExpired => "siopv2_request_expired",
            Self::InvalidSubject => "siopv2_invalid_subject",
            Self::InvalidKeyId => "siopv2_invalid_key_id",
            Self::AlgorithmMismatch => "siopv2_algorithm_mismatch",
            Self::ResponseTooLarge => "siopv2_response_too_large",
            Self::ResponseEncodingFailed => "siopv2_response_encoding_failed",
            Self::SigningRejected => "siopv2_signing_rejected",
            Self::SignerUnavailable => "siopv2_signer_unavailable",
            Self::InvalidEventId => "siopv2_invalid_event_id",
            Self::InvalidTransition => "siopv2_invalid_transition",
            Self::EventConflict => "siopv2_event_conflict",
        }
    }
}

impl fmt::Display for Siopv2Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl Error for Siopv2Error {}
