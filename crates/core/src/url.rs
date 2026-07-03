//! `Url` — a validated URL domain primitive built with `#[derive(Newtype)]`.
//!
//! Dogfoods the string category of the derive: `Url(String)` gets `as_str`,
//! `AsRef<str>`, `Display`, serde (validating on `Deserialize`), and
//! `TryFrom<String>`/`try_new`/`parse`/`FromStr` (validated construction) backed
//! by a hand-rolled validator (no external `url`/`uriparse` dependency —
//! foundation stays external-dep-free beyond `serde`, which the derive's serde
//! impls require). A `pub(crate)` `new_unchecked` hatch bypasses validation.
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
/// Construct fallibly with [`Url::try_new`] / [`Url::parse`] /
/// [`Url::try_from`] / `FromStr` — all run `validate_url` (scheme + `://` +
/// non-empty authority/path structure). `Url::new_unchecked` is a
/// `pub(crate)` trusted hatch that bypasses validation (no `pub` widening);
/// use it only when the caller has already proved the value is well-formed.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Newtype)]
#[newtype(display, serde, validate_fn = validate_url, validate_err = UrlError)]
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
/// Invoked by the derive as `validate_url(&inner)` where `inner: String`;
/// `&String` deref-coerces to `&str`.
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
    fn url_new_unchecked_bypasses_validation() {
        // `pub(crate)` hatch: the infallible `new` is gone for `validate_fn`
        // types; `new_unchecked` lets the defining crate construct an invalid
        // value, owning the proof obligation.
        let u = Url::new_unchecked("not a url at all".to_owned());
        assert_eq!(u.as_str(), "not a url at all");
    }

    #[test]
    fn url_try_new_validates() {
        assert!(Url::try_new("https://example.com".to_owned()).is_ok());
        assert!(Url::try_new("not a url".to_owned()).is_err());
        assert!(Url::try_from("not a url".to_owned()).is_err());
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
        let u = Url::try_new("https://example.com".to_owned()).unwrap();
        let json = serde_json::to_string(&u).unwrap();
        assert_eq!(json, "\"https://example.com\"");
        let back: Url = serde_json::from_str("\"https://example.com\"").unwrap();
        assert_eq!(back, u);
        // Validating `Deserialize`: an invalid URL is rejected.
        assert!(serde_json::from_str::<Url>("\"not a url\"").is_err());
    }

    #[test]
    fn url_display_passes_through() {
        let u = Url::try_new("https://example.com".to_owned()).unwrap();
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
