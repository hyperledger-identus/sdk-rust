//! `Url` — a validated URL domain primitive built with `#[derive(Newtype)]`.
//!
//! Dogfoods the string category of the derive: `Url(String)` gets `as_str`,
//! `AsRef<str>`, `From<String>`/`From<&str>`, `Display`, transparent serde,
//! and a fallible `parse`/`FromStr` backed by a hand-rolled validator (no
//! external `url`/`uriparse` dependency — foundation stays external-dep-free
//! beyond `serde`, which the derive's serde impls require).
//!
//! The rich local error `UrlError` carries the structured validation outcome;
//! its hand-written `to_identus_error()` bridges to the redaction-safe
//! `IdentusError` contract with a stable `ErrorCode` (`core.invalid_url`) and
//! `CapabilityId("core")`. `IdentusError::Display` carries no runtime detail.

use core::fmt;

use crate::{CapabilityId, ErrorCode, ErrorKind, IdentusError};
use identus_derive::Newtype;

const CAPABILITY: CapabilityId = CapabilityId::new("core");
const URL_ERROR_CODE: ErrorCode = ErrorCode::new("core.invalid_url");

/// A validated URL.
///
/// Construct infallibly with [`Url::new`] (no validation) or fallibly with
/// [`Url::parse`] / `FromStr`, which run `validate_url` (scheme + `://` +
/// non-empty authority/path structure).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Newtype)]
#[newtype(display, serde)]
#[newtype(parse = validate_url, err = UrlError)]
pub struct Url(String);

/// Structured validation outcome for [`Url`].
///
/// This is the rich local error surface (the `FromStr::Err`); it carries the
/// reason a URL was rejected. [`UrlError::to_identus_error`] bridges to the
/// redaction-safe [`IdentusError`], whose `Display` carries no runtime detail.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UrlError {
    /// The input had no `://` separator or an empty scheme.
    MissingScheme,
    /// The scheme contained characters outside the RFC 3986 scheme grammar.
    InvalidScheme,
    /// The `://` separator was present but the authority was empty.
    MissingAuthority,
}

impl UrlError {
    /// Bridge this local error to the redaction-safe [`IdentusError`] contract.
    ///
    /// The resulting `IdentusError` carries a stable [`ErrorCode`] and
    /// [`CapabilityId`] but only a `&'static str` public message, so the
    /// structured reason above never leaks through `IdentusError`'s `Display`.
    pub fn to_identus_error(&self) -> IdentusError {
        let public_message = match self {
            UrlError::MissingScheme => "URL is missing a scheme",
            UrlError::InvalidScheme => "URL has an invalid scheme",
            UrlError::MissingAuthority => "URL is missing an authority",
        };
        IdentusError::public(
            URL_ERROR_CODE,
            ErrorKind::InvalidInput,
            CAPABILITY,
            public_message,
        )
    }
}

impl fmt::Display for UrlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            UrlError::MissingScheme => "missing scheme",
            UrlError::InvalidScheme => "invalid scheme",
            UrlError::MissingAuthority => "missing authority",
        };
        f.write_str(s)
    }
}

impl std::error::Error for UrlError {}

/// Hand-rolled URL validation (no external dependency).
///
/// Accepts inputs of the form `scheme://authority[/path][?query][#fragment]`
/// where `scheme` matches the RFC 3986 scheme grammar
/// (`ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )`) and `authority` is non-empty.
fn validate_url(s: &str) -> Result<(), UrlError> {
    let Some((scheme, rest)) = s.split_once("://") else {
        return Err(UrlError::MissingScheme);
    };
    if scheme.is_empty() {
        return Err(UrlError::MissingScheme);
    }
    let mut chars = scheme.chars();
    let first = chars.next().expect("scheme is non-empty");
    if !first.is_ascii_alphabetic()
        || !chars.all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '-' || c == '.')
    {
        return Err(UrlError::InvalidScheme);
    }
    let authority_end = rest.find(['/', '?', '#']).unwrap_or(rest.len());
    let authority = &rest[..authority_end];
    if authority.is_empty() {
        return Err(UrlError::MissingAuthority);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_infallible_new_does_not_validate() {
        let u = Url::new("not a url at all".to_owned());
        assert_eq!(u.as_str(), "not a url at all");
    }

    #[test]
    fn url_parse_accepts_well_formed() {
        let u = Url::parse("https://example.com/path").unwrap();
        assert_eq!(u.as_str(), "https://example.com/path");
        let parsed: Url = "https://did.example.com/did".parse().unwrap();
        assert_eq!(parsed.as_str(), "https://did.example.com/did");
    }

    #[test]
    fn url_parse_rejects_missing_scheme() {
        assert_eq!(
            Url::parse("example.com").unwrap_err(),
            UrlError::MissingScheme
        );
        assert_eq!(
            Url::parse("://no-scheme").unwrap_err(),
            UrlError::MissingScheme
        );
    }

    #[test]
    fn url_parse_rejects_invalid_scheme() {
        assert_eq!(Url::parse("1bad://x").unwrap_err(), UrlError::InvalidScheme);
        assert_eq!(
            Url::parse("sch eme://x").unwrap_err(),
            UrlError::InvalidScheme
        );
    }

    #[test]
    fn url_parse_rejects_missing_authority() {
        assert_eq!(
            Url::parse("https:///path").unwrap_err(),
            UrlError::MissingAuthority
        );
    }

    #[test]
    fn url_serde_roundtrips_as_plain_string() {
        let u = Url::new("https://example.com".to_owned());
        let json = serde_json::to_string(&u).unwrap();
        assert_eq!(json, "\"https://example.com\"");
        let back: Url = serde_json::from_str("\"https://example.com\"").unwrap();
        assert_eq!(back, u);
    }

    #[test]
    fn url_display_passes_through() {
        let u = Url::new("https://example.com".to_owned());
        assert_eq!(u.to_string(), "https://example.com");
    }

    #[test]
    fn url_error_bridges_to_redaction_safe_identus_error() {
        for err in [
            UrlError::MissingScheme,
            UrlError::InvalidScheme,
            UrlError::MissingAuthority,
        ] {
            let ie = err.to_identus_error();
            assert_eq!(ie.code(), URL_ERROR_CODE);
            assert_eq!(ie.capability(), Some(CAPABILITY));
            assert_eq!(ie.kind(), ErrorKind::InvalidInput);
            let rendered = ie.to_string();
            assert!(rendered.starts_with("core.invalid_url: "));
            // No runtime detail leaks through IdentusError::Display.
            assert!(!rendered.contains("example.com"));
        }
    }
}
