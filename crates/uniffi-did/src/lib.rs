//! Experimental UniFFI value facade for bounded DID and DID URL parsing.
//!
//! This crate is an unpublished outer boundary. It exposes only owned public
//! identifier values and closed errors; `identus-did` remains binding-free.

use std::{fmt, panic::UnwindSafe};

use identus_did::{Did, DidUrl, Error};

/// Version of the cross-language value and error contract.
pub const BINDING_API_VERSION: u32 = 1;

const INVALID_DID_CODE: &str = "did.invalid_did";
const INVALID_DID_URL_CODE: &str = "did.invalid_did_url";
const INTERNAL_CODE: &str = "bindings.internal";

/// Owned cross-language view of a validated DID.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct DidView {
    /// Exact validated DID value.
    pub value: String,
    /// DID method name.
    pub method: String,
    /// Method-specific identifier.
    pub method_specific_id: String,
}

/// Owned cross-language view of a validated DID URL.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct DidUrlView {
    /// Exact validated DID URL value.
    pub value: String,
    /// Base DID portion.
    pub did: String,
    /// DID method name.
    pub method: String,
    /// Method-specific identifier.
    pub method_specific_id: String,
    /// Path, including its leading slash when present.
    pub path: String,
    /// Query without the leading question mark.
    pub query: Option<String>,
    /// Fragment without the leading hash.
    pub fragment: Option<String>,
}

/// Closed cross-language failures with stable, redacted machine codes.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Error)]
pub enum DidBindingError {
    /// The DID is invalid or exceeds its resource bound.
    InvalidDid {
        /// Stable SDK error code.
        code: String,
    },
    /// The DID URL is invalid or exceeds its resource bound.
    InvalidDidUrl {
        /// Stable SDK error code.
        code: String,
    },
    /// Authored wrapper logic unexpectedly unwound.
    Internal {
        /// Stable SDK error code.
        code: String,
    },
}

impl DidBindingError {
    /// Stable error code without caller or implementation text.
    #[must_use]
    pub fn code(&self) -> &str {
        match self {
            Self::InvalidDid { code } | Self::InvalidDidUrl { code } | Self::Internal { code } => {
                code
            }
        }
    }

    fn invalid_did() -> Self {
        Self::InvalidDid {
            code: INVALID_DID_CODE.to_owned(),
        }
    }

    fn invalid_did_url() -> Self {
        Self::InvalidDidUrl {
            code: INVALID_DID_URL_CODE.to_owned(),
        }
    }

    fn internal() -> Self {
        Self::Internal {
            code: INTERNAL_CODE.to_owned(),
        }
    }
}

impl fmt::Display for DidBindingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for DidBindingError {}

/// Return the cross-language contract version.
#[uniffi::export]
pub fn binding_api_version() -> u32 {
    BINDING_API_VERSION
}

/// Parse a bounded DID into an owned cross-language view.
#[uniffi::export]
pub fn parse_did(value: String) -> Result<DidView, DidBindingError> {
    contain_unwind(|| {
        Did::try_new(value)
            .map(|did| DidView {
                value: did.as_str().to_owned(),
                method: did.method().to_owned(),
                method_specific_id: did.method_specific_id().to_owned(),
            })
            .map_err(map_did_error)
    })
}

/// Parse a bounded DID URL into an owned cross-language view.
#[uniffi::export]
pub fn parse_did_url(value: String) -> Result<DidUrlView, DidBindingError> {
    contain_unwind(|| {
        DidUrl::try_new(value)
            .map(|did_url| DidUrlView {
                value: did_url.as_str().to_owned(),
                did: did_url.as_did_str().to_owned(),
                method: did_url.method().to_owned(),
                method_specific_id: did_url.method_specific_id().to_owned(),
                path: did_url.path().to_owned(),
                query: did_url.query().map(str::to_owned),
                fragment: did_url.fragment().map(str::to_owned),
            })
            .map_err(map_did_url_error)
    })
}

fn contain_unwind<T>(
    operation: impl FnOnce() -> Result<T, DidBindingError> + UnwindSafe,
) -> Result<T, DidBindingError> {
    std::panic::catch_unwind(operation).unwrap_or_else(|_| Err(DidBindingError::internal()))
}

fn map_did_error(error: Error) -> DidBindingError {
    debug_assert_eq!(error.to_identus_error().code().as_str(), INVALID_DID_CODE);
    DidBindingError::invalid_did()
}

fn map_did_url_error(error: Error) -> DidBindingError {
    debug_assert_eq!(
        error.to_identus_error().code().as_str(),
        INVALID_DID_URL_CODE
    );
    DidBindingError::invalid_did_url()
}

uniffi::setup_scaffolding!();

#[cfg(test)]
mod tests {
    use identus_did::{MAX_DID_BYTES, MAX_DID_URL_BYTES};

    use super::*;

    #[test]
    fn api_version_is_explicit() {
        assert_eq!(binding_api_version(), 1);
    }

    #[test]
    fn did_round_trip_and_components_are_stable() {
        let view = parse_did("did:example:123".to_owned()).unwrap();
        assert_eq!(view.value, "did:example:123");
        assert_eq!(view.method, "example");
        assert_eq!(view.method_specific_id, "123");
    }

    #[test]
    fn did_url_round_trip_and_components_are_stable() {
        let view = parse_did_url("did:example:123/path?service=agent#key-1".to_owned()).unwrap();
        assert_eq!(view.value, "did:example:123/path?service=agent#key-1");
        assert_eq!(view.did, "did:example:123");
        assert_eq!(view.method, "example");
        assert_eq!(view.method_specific_id, "123");
        assert_eq!(view.path, "/path");
        assert_eq!(view.query.as_deref(), Some("service=agent"));
        assert_eq!(view.fragment.as_deref(), Some("key-1"));
    }

    #[test]
    fn failures_are_closed_codes_without_caller_text() {
        let caller_text = "did:EXAMPLE:do-not-reflect";
        let error = parse_did(caller_text.to_owned()).unwrap_err();
        assert_eq!(error.code(), INVALID_DID_CODE);
        assert_eq!(error.to_string(), INVALID_DID_CODE);
        assert!(!format!("{error:?}").contains(caller_text));
    }

    #[test]
    fn sdk_resource_limits_remain_at_the_wrapper_edge() {
        let oversized_did = format!("did:example:{}", "a".repeat(MAX_DID_BYTES));
        assert_eq!(
            parse_did(oversized_did).unwrap_err().code(),
            INVALID_DID_CODE
        );

        let oversized_url = format!("did:example:1/{}", "a".repeat(MAX_DID_URL_BYTES));
        assert_eq!(
            parse_did_url(oversized_url).unwrap_err().code(),
            INVALID_DID_URL_CODE
        );
    }

    #[test]
    fn unexpected_unwind_returns_only_the_internal_code() {
        let error = contain_unwind::<()>(|| panic!("panic-canary-do-not-reflect")).unwrap_err();
        assert_eq!(error.code(), INTERNAL_CODE);
        assert_eq!(error.to_string(), INTERNAL_CODE);
        assert!(!format!("{error:?}").contains("panic-canary"));
    }
}
