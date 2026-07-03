//! `DidMethod` — a validated DID method-name domain primitive.
//!
//! Dogfoods the string category of `#[derive(Newtype)]`: `DidMethod(String)`
//! gets `as_str`, `AsRef<str>`, `Display`, serde (validating on `Deserialize`),
//! `TryFrom<String>`/`try_new`/`parse`/`FromStr` (validated construction), and a
//! `pub(crate)` `new_unchecked` hatch, backed by a hand-rolled validator
//! matching the W3C DID Core `method-name` grammar
//! (`1*( lowercase-alpha / digit )`). The rich local error and its
//! `to_identus_error()` bridging are hand-written (the derive generates generic
//! boilerplate only), per the `core-error-conventions` two-surface split.

use identus_derive::Newtype;

use crate::error::Error;

/// A validated DID method name (e.g. `"key"`, `"web"`, `"prism"`).
///
/// Construct fallibly with [`DidMethod::try_new`] / [`DidMethod::parse`] /
/// [`DidMethod::try_from`] / `FromStr` — all run `validate_did_method` and
/// reject invalid grammar. `DidMethod::new_unchecked` is a `pub(crate)`
/// trusted hatch that bypasses validation (no `pub` widening is offered); use
/// it only when the caller has already proved the value is well-formed.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Newtype)]
#[newtype(display, serde, validate_fn = validate_did_method, validate_err = Error)]
pub struct DidMethod(String);

/// Hand-rolled DID method-name validation (no external dependency).
///
/// Accepts non-empty strings of lowercase ASCII letters and digits only, per
/// the W3C DID Core `method-name = 1*method-char` / `method-char = %x61-7A /
/// DIGIT` grammar. Invoked by the derive as `validate_did_method(&inner)`
/// where `inner: String`; `&String` deref-coerces to `&str`.
fn validate_did_method(s: &str) -> Result<(), Error> {
    if s.is_empty() {
        return Err(Error::InvalidMethod("method name is empty".to_owned()));
    }
    if !s
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
    {
        return Err(Error::InvalidMethod(format!(
            "method name `{s}` has invalid characters"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use identus_core::ErrorKind;

    #[test]
    fn method_new_unchecked_bypasses_validation() {
        // `pub(crate)` hatch: the defining crate can construct an invalid value,
        // owning the proof obligation. The infallible `new` is gone for
        // `validate_fn`-configured types.
        let m = DidMethod::new_unchecked("UPPER NOT VALID".to_owned());
        assert_eq!(m.as_str(), "UPPER NOT VALID");
    }

    #[test]
    fn method_try_new_validates() {
        assert_eq!(
            DidMethod::try_new("key".to_owned()).unwrap().as_str(),
            "key"
        );
        assert_eq!(
            DidMethod::try_from("web".to_owned()).unwrap().as_str(),
            "web"
        );
        assert!(DidMethod::try_new("BAD".to_owned()).is_err());
        assert!(DidMethod::try_from("".to_owned()).is_err());
    }

    #[test]
    fn method_parse_accepts_lowercase_alphanumeric() {
        assert_eq!(DidMethod::parse("key").unwrap().as_str(), "key");
        assert_eq!(DidMethod::parse("web").unwrap().as_str(), "web");
        assert_eq!(DidMethod::parse("prism").unwrap().as_str(), "prism");
        assert_eq!(DidMethod::parse("method1").unwrap().as_str(), "method1");
    }

    #[test]
    fn method_parse_rejects_empty() {
        let err = DidMethod::parse("").unwrap_err();
        assert!(matches!(err, Error::InvalidMethod(_)));
    }

    #[test]
    fn method_parse_rejects_uppercase_and_symbols() {
        assert!(DidMethod::parse("Key").is_err());
        assert!(DidMethod::parse("web-2").is_err());
        assert!(DidMethod::parse("web.example").is_err());
    }

    #[test]
    fn method_serde_roundtrips_as_plain_string() {
        let m = DidMethod::try_new("key".to_owned()).unwrap();
        let json = serde_json::to_string(&m).unwrap();
        assert_eq!(json, "\"key\"");
        let back: DidMethod = serde_json::from_str("\"web\"").unwrap();
        assert_eq!(back.as_str(), "web");
        // Validating `Deserialize`: an uppercase-invalid value is rejected.
        assert!(serde_json::from_str::<DidMethod>("\"BAD\"").is_err());
    }

    #[test]
    fn method_display_passes_through() {
        let m = DidMethod::try_new("prism".to_owned()).unwrap();
        assert_eq!(m.to_string(), "prism");
    }

    #[test]
    fn method_error_bridges_to_redaction_safe_identus_error() {
        let err = DidMethod::parse("BAD").unwrap_err();
        let ie = err.to_identus_error();
        assert_eq!(ie.code().as_str(), "did.invalid_method");
        assert_eq!(ie.capability().map(|c| c.as_str()), Some("did"));
        assert_eq!(ie.kind(), ErrorKind::InvalidInput);
        assert!(!ie.to_string().contains("BAD"));
    }
}
