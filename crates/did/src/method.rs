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

use crate::{MAX_DID_BYTES, error::Error};

/// Maximum accepted byte length of a standalone [`DidMethod`].
///
/// This is the largest method that can fit in a maximum-size bare DID after
/// accounting for `did:`, the method separator and a one-byte method-specific
/// identifier. W3C DID Core does not impose this value; it is an SDK resource
/// policy aligned with [`MAX_DID_BYTES`].
pub const MAX_DID_METHOD_BYTES: usize = MAX_DID_BYTES - 6;

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
/// Accepts non-empty strings no larger than [`MAX_DID_METHOD_BYTES`] containing
/// lowercase ASCII letters and digits only, per the W3C DID Core
/// `method-name = 1*method-char` / `method-char = %x61-7A / DIGIT` grammar.
/// Borrowed macro paths invoke this validator before allocating the owned
/// success value; owned and serde paths invoke it against their existing
/// allocation.
fn validate_did_method(s: &str) -> Result<(), Error> {
    if s.is_empty() {
        return Err(Error::InvalidMethod("method name is empty".to_owned()));
    }
    if s.len() > MAX_DID_METHOD_BYTES {
        return Err(Error::InvalidMethod(
            "method name exceeds the SDK byte limit".to_owned(),
        ));
    }
    if !s
        .bytes()
        .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
    {
        return Err(Error::InvalidMethod(
            "method name contains an invalid character".to_owned(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        DeactivateRegistrationRequest, Did, RegistrationIdempotencyKey, RegistrationPublicData,
        RegistrationSecretMode,
    };
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
        for input in ["Key", "web-2", "web.example"] {
            let error = DidMethod::parse(input).unwrap_err();
            assert!(!error.to_string().contains(input));
            assert!(!format!("{error:?}").contains(input));
        }
    }

    #[test]
    fn method_exact_limit_is_accepted_by_every_construction_path() {
        let input = "a".repeat(MAX_DID_METHOD_BYTES);

        assert_eq!(DidMethod::parse(&input).unwrap().as_str(), input);
        assert_eq!(DidMethod::try_new(input.clone()).unwrap().as_str(), input);
        assert_eq!(DidMethod::try_from(input.clone()).unwrap().as_str(), input);

        let json = serde_json::to_string(&input).unwrap();
        assert_eq!(
            serde_json::from_str::<DidMethod>(&json).unwrap().as_str(),
            input
        );
    }

    #[test]
    fn method_one_over_limit_precedes_grammar_and_redacts_input() {
        let input = format!("{}CANARY", "a".repeat(MAX_DID_METHOD_BYTES));
        let expected_detail = "method name exceeds the SDK byte limit";

        let local_errors = [
            DidMethod::parse(&input).unwrap_err(),
            DidMethod::try_new(input.clone()).unwrap_err(),
            DidMethod::try_from(input.clone()).unwrap_err(),
        ];
        for error in local_errors {
            assert!(!error.to_string().contains("CANARY"));
            assert!(!format!("{error:?}").contains("CANARY"));
        }

        let serde_error =
            serde_json::from_str::<DidMethod>(&serde_json::to_string(&input).unwrap()).unwrap_err();
        assert!(!serde_error.to_string().contains("CANARY"));
        assert!(!format!("{serde_error:?}").contains("CANARY"));
        assert_eq!(
            DidMethod::parse(&input),
            Err(Error::InvalidMethod(expected_detail.to_owned()))
        );
    }

    #[test]
    fn maximum_method_remains_compatible_with_did_registration() {
        let method = "a".repeat(MAX_DID_METHOD_BYTES);
        let did = Did::parse(&format!("did:{method}:x")).unwrap();
        assert_eq!(did.as_str().len(), MAX_DID_BYTES);
        assert_eq!(DidMethod::parse(did.method()).unwrap().as_str(), method);

        let request = DeactivateRegistrationRequest::new(
            did,
            RegistrationPublicData::empty(),
            RegistrationSecretMode::ClientManaged,
            RegistrationIdempotencyKey::parse("request-1").unwrap(),
        )
        .unwrap();
        assert_eq!(request.method().as_str(), method);
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
