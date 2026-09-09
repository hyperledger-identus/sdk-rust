//! Research-only UniFFI facade for bounded DID and DID URL values.

use std::fmt;

use identus_did::{Did, DidUrl, Error};

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct DidView {
    pub value: String,
    pub method: String,
    pub method_specific_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct DidUrlView {
    pub value: String,
    pub did: String,
    pub method: String,
    pub method_specific_id: String,
    pub path: String,
    pub query: Option<String>,
    pub fragment: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Error)]
pub enum DidBindingError {
    InvalidDid,
    InvalidDidUrl,
}

impl DidBindingError {
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::InvalidDid => "did.invalid_did",
            Self::InvalidDidUrl => "did.invalid_did_url",
        }
    }
}

impl fmt::Display for DidBindingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for DidBindingError {}

#[uniffi::export]
pub fn parse_did(value: String) -> Result<DidView, DidBindingError> {
    Did::try_new(value)
        .map(|did| DidView {
            value: did.as_str().to_owned(),
            method: did.method().to_owned(),
            method_specific_id: did.method_specific_id().to_owned(),
        })
        .map_err(map_did_error)
}

#[uniffi::export]
pub fn parse_did_url(value: String) -> Result<DidUrlView, DidBindingError> {
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
}

fn map_did_error(error: Error) -> DidBindingError {
    debug_assert_eq!(error.to_identus_error().code().as_str(), "did.invalid_did");
    DidBindingError::InvalidDid
}

fn map_did_url_error(error: Error) -> DidBindingError {
    debug_assert_eq!(
        error.to_identus_error().code().as_str(),
        "did.invalid_did_url"
    );
    DidBindingError::InvalidDidUrl
}

uniffi::setup_scaffolding!();

#[cfg(test)]
mod tests {
    use identus_did::{MAX_DID_BYTES, MAX_DID_URL_BYTES};

    use super::*;

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
    fn errors_are_closed_redacted_codes() {
        let secret_like = "did:EXAMPLE:do-not-reflect";
        let error = parse_did(secret_like.to_owned()).unwrap_err();
        assert_eq!(error, DidBindingError::InvalidDid);
        assert_eq!(error.to_string(), "did.invalid_did");
        assert!(!error.to_string().contains(secret_like));
    }

    #[test]
    fn sdk_resource_limits_remain_at_the_wrapper_edge() {
        let oversized_did = format!("did:example:{}", "a".repeat(MAX_DID_BYTES));
        assert_eq!(
            parse_did(oversized_did).unwrap_err(),
            DidBindingError::InvalidDid
        );

        let oversized_url = format!("did:example:1/{}", "a".repeat(MAX_DID_URL_BYTES));
        assert_eq!(
            parse_did_url(oversized_url).unwrap_err(),
            DidBindingError::InvalidDidUrl
        );
    }
}
