//! Error-bridging tests: every `crypto::Error` variant maps to a stable
//! `ErrorCode` + `CapabilityId("crypto")`, and the `IdentusError` `Display`
//! carries no runtime detail.

use identus_core::CapabilityId;
use identus_crypto::error::Error;

#[test]
fn invalid_key_size_bridges_to_stable_code() {
    let err = Error::InvalidKeySize {
        expected: 32,
        actual: 31,
        key_type: "Ed25519PublicKey",
    };
    let identus = err.to_identus_error();
    assert_eq!(identus.code().as_str(), "crypto.invalid_key_size");
    assert_eq!(identus.capability(), Some(CapabilityId::new("crypto")));
    let rendered = identus.to_string();
    assert!(!rendered.contains("31"));
    assert!(!rendered.contains("32"));
    assert!(!rendered.contains("Ed25519PublicKey"));
}

#[test]
fn key_parsing_bridges_to_stable_code() {
    let err = Error::KeyParsing {
        source: Box::new(std::io::Error::other("boom")),
    };
    let identus = err.to_identus_error();
    assert_eq!(identus.code().as_str(), "crypto.key_parsing");
    assert!(!identus.to_string().contains("boom"));
}

#[test]
fn signature_invalid_bridges_to_verification_failed() {
    use identus_core::ErrorKind;
    let err = Error::SignatureInvalid;
    let identus = err.to_identus_error();
    assert_eq!(identus.kind(), ErrorKind::VerificationFailed);
    assert_eq!(identus.code().as_str(), "crypto.signature_invalid");
}

#[test]
fn unsupported_curve_bridges_to_stable_code() {
    let identus = Error::UnsupportedCurve.to_identus_error();
    assert_eq!(identus.code().as_str(), "crypto.unsupported_curve");
}

#[test]
fn derivation_failed_bridges_to_stable_code() {
    let identus = Error::DerivationFailed.to_identus_error();
    assert_eq!(identus.code().as_str(), "crypto.derivation_failed");
}

#[test]
fn mnemonic_invalid_bridges_to_stable_code() {
    let identus = Error::MnemonicInvalid.to_identus_error();
    assert_eq!(identus.code().as_str(), "crypto.mnemonic_invalid");
}

#[test]
fn secure_random_failure_bridges_to_stable_code() {
    let identus = Error::SecureRandomFailure.to_identus_error();
    assert_eq!(identus.code().as_str(), "crypto.secure_random_failure");
}

#[test]
fn every_variant_carries_crypto_capability() {
    use identus_core::CapabilityId;
    let cases = [
        Error::InvalidKeySize {
            expected: 1,
            actual: 2,
            key_type: "T",
        }
        .to_identus_error(),
        Error::KeyParsing {
            source: Box::new(std::io::Error::other("x")),
        }
        .to_identus_error(),
        Error::SignatureInvalid.to_identus_error(),
        Error::UnsupportedCurve.to_identus_error(),
        Error::DerivationFailed.to_identus_error(),
        Error::MnemonicInvalid.to_identus_error(),
        Error::SecureRandomFailure.to_identus_error(),
    ];
    for identus in cases {
        assert_eq!(identus.capability(), Some(CapabilityId::new("crypto")));
    }
}
