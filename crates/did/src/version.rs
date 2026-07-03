//! `Version` — a numeric domain primitive built with `#[derive(Newtype)]`.
//!
//! Dogfoods the numeric category of the derive: `Version(u8)` gets `get`,
//! `Display`, serde (validating on `Deserialize`), `TryFrom<u8>`/`try_new`
//! (validated number-in paths), and a `pub(crate)` `new_unchecked` hatch,
//! backed by a hand-rolled non-zero validator. `parse`/`FromStr` are not
//! generated for the numeric category. The rich local error and
//! `to_identus_error()` bridging are hand-written, per the
//! `core-error-conventions` two-surface split.

use identus_derive::Newtype;

use crate::error::Error;

/// A non-zero DID-spec version number.
///
/// Construct fallibly with [`Version::try_new`] / [`Version::try_from`] — both
/// run `validate_version` (non-zero). `Version::new_unchecked` is a
/// `pub(crate)` trusted hatch that bypasses validation (no `pub` widening).
/// `parse`/`FromStr` are not available for `Version` (numeric drops the
/// string-shaped entry); a caller with a string does `s.parse::<u8>()?` then
/// `Version::try_new(n)?`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Newtype)]
#[newtype(display, serde, validate_fn = validate_version, validate_err = Error)]
pub struct Version(u8);

/// Hand-rolled non-zero validation.
///
/// Ensures the version is non-zero. The validator reasons about the inner `u8`
/// directly (invoked by the derive as `validate_version(&inner)` where
/// `inner: u8`); there is no string round-trip. Returns [`Error::InvalidVersion`]
/// for zero.
fn validate_version(n: &u8) -> Result<(), Error> {
    if *n == 0 {
        return Err(Error::InvalidVersion("version must be non-zero".to_owned()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use identus_core::ErrorKind;

    #[test]
    fn version_try_new_validates_nonzero() {
        assert_eq!(Version::try_new(1).unwrap().get(), 1);
        assert_eq!(Version::try_new(255).unwrap().get(), 255);
        assert!(matches!(
            Version::try_new(0).unwrap_err(),
            Error::InvalidVersion(_)
        ));
        assert!(matches!(
            Version::try_from(0).unwrap_err(),
            Error::InvalidVersion(_)
        ));
    }

    #[test]
    fn version_new_unchecked_bypasses_validation() {
        // `pub(crate)` hatch: the infallible `new` is gone for `validate_fn`
        // types; `new_unchecked` constructs without validating, owning the proof
        // obligation. `new` is not callable for `Version`.
        let v = Version::new_unchecked(0);
        assert_eq!(v.get(), 0);
    }

    #[test]
    fn version_get_and_display() {
        let v = Version::try_new(2).unwrap();
        assert_eq!(v.get(), 2);
        assert_eq!(Version::try_from(3).unwrap().get(), 3);
        assert_eq!(v.to_string(), "2");
    }

    #[test]
    fn version_serde_roundtrips_as_json_number() {
        let v = Version::try_new(2).unwrap();
        let json = serde_json::to_string(&v).unwrap();
        assert_eq!(json, "2");
        let back: Version = serde_json::from_str("2").unwrap();
        assert_eq!(back, v);
        // Validating `Deserialize`: zero is rejected.
        assert!(serde_json::from_str::<Version>("0").is_err());
    }

    #[test]
    fn version_error_bridges_to_redaction_safe_identus_error() {
        let err = Version::try_new(0).unwrap_err();
        let ie = err.to_identus_error();
        assert_eq!(ie.code().as_str(), "did.invalid_version");
        assert_eq!(ie.capability().map(|c| c.as_str()), Some("did"));
        assert_eq!(ie.kind(), ErrorKind::InvalidInput);
        assert!(!ie.to_string().contains("non-zero"));
    }
}
