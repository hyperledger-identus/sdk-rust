//! `Version` — a numeric domain primitive built with `#[derive(Newtype)]`.
//!
//! Dogfoods the numeric category of the derive: `Version(u8)` gets `new`,
//! `get`, `From<u8>`, `Display`, transparent serde (as a JSON number), and a
//! bounds-checked `parse`/`FromStr` backed by a hand-rolled validator. The
//! rich local error and `to_identus_error()` bridging are hand-written, per
//! the `core-error-conventions` two-surface split.

use identus_derive::Newtype;

use crate::error::Error;

/// A non-zero DID-spec version number.
///
/// Construct infallibly with [`Version::new`] (no validation) or fallibly
/// with [`Version::parse`] / `FromStr`, which run `validate_version`
/// (parses as a `u8` and is non-zero).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Newtype)]
#[newtype(display, serde)]
#[newtype(parse = validate_version, err = Error)]
pub struct Version(u8);

/// Hand-rolled bounds-checked validation.
///
/// Ensures the input parses as a `u8` (so the derive's re-parse cannot panic)
/// and is non-zero. Returns [`Error::InvalidVersion`] otherwise.
fn validate_version(s: &str) -> Result<(), Error> {
    let n: u8 = s
        .parse()
        .map_err(|_| Error::InvalidVersion(format!("`{s}` is not a valid version number")))?;
    if n == 0 {
        return Err(Error::InvalidVersion("version must be non-zero".to_owned()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use identus_core::ErrorKind;

    #[test]
    fn version_infallible_new_does_not_validate() {
        let v = Version::new(0);
        assert_eq!(v.get(), 0);
    }

    #[test]
    fn version_get_from_and_display() {
        let v = Version::new(2);
        assert_eq!(v.get(), 2);
        assert_eq!(Version::from(3).get(), 3);
        assert_eq!(v.to_string(), "2");
    }

    #[test]
    fn version_parse_accepts_nonzero_u8() {
        assert_eq!(Version::parse("1").unwrap().get(), 1);
        assert_eq!(Version::parse("255").unwrap().get(), 255);
    }

    #[test]
    fn version_parse_rejects_zero_and_non_numeric() {
        assert!(matches!(
            Version::parse("0").unwrap_err(),
            Error::InvalidVersion(_)
        ));
        assert!(Version::parse("abc").is_err());
        assert!(Version::parse("256").is_err()); // out of u8 range
    }

    #[test]
    fn version_serde_roundtrips_as_json_number() {
        let v = Version::new(2);
        let json = serde_json::to_string(&v).unwrap();
        assert_eq!(json, "2");
        let back: Version = serde_json::from_str("2").unwrap();
        assert_eq!(back, v);
    }

    #[test]
    fn version_error_bridges_to_redaction_safe_identus_error() {
        let err = Version::parse("0").unwrap_err();
        let ie = err.to_identus_error();
        assert_eq!(ie.code().as_str(), "did.invalid_version");
        assert_eq!(ie.capability().map(|c| c.as_str()), Some("did"));
        assert_eq!(ie.kind(), ErrorKind::InvalidInput);
        assert!(!ie.to_string().contains("non-zero"));
    }
}
