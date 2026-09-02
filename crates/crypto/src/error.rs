//! Two-surface error bridging for `identus-crypto`.
//!
//! `Error` is the rich, idiomatic enum carrying runtime detail for local
//! debugging (logs, `?`-propagation of source errors). `to_identus_error()`
//! maps each variant to a redaction-safe [`identus_core::IdentusError`] with a
//! stable [`ErrorCode`](identus_core::ErrorCode) catalogue and [`CapabilityId"]`("crypto")`; the
//! `IdentusError` `Display` carries only `"{code}: {public_message}"`, so
//! runtime detail like key sizes or key-type names never crosses the surface.
//!
//! This is the first adopter of the `core-error-conventions` bridging pattern.

use identus_core::{CapabilityId, ErrorKind, IdentusError};
use std::fmt;

/// Owning capability for every bridged crypto error.
pub const CAPABILITY: CapabilityId = CapabilityId::new("crypto");

/// Stable error-code catalogue (matched by conformance fixtures and bindings).
pub mod error_code {
    use identus_core::ErrorCode;

    pub const INVALID_KEY_SIZE: ErrorCode = ErrorCode::new("crypto.invalid_key_size");
    pub const INVALID_JWK: ErrorCode = ErrorCode::new("crypto.invalid_jwk");
    pub const KEY_PARSING: ErrorCode = ErrorCode::new("crypto.key_parsing");
    pub const SIGNATURE_INVALID: ErrorCode = ErrorCode::new("crypto.signature_invalid");
    pub const UNSUPPORTED_CURVE: ErrorCode = ErrorCode::new("crypto.unsupported_curve");
    pub const DERIVATION_FAILED: ErrorCode = ErrorCode::new("crypto.derivation_failed");
    pub const MNEMONIC_INVALID: ErrorCode = ErrorCode::new("crypto.mnemonic_invalid");
    pub const SECURE_RANDOM_FAILURE: ErrorCode = ErrorCode::new("crypto.secure_random_failure");
}

/// Rich, idiomatic crypto error carrying runtime detail for local debugging.
///
/// Use [`Error::to_identus_error`] to bridge to the redaction-safe
/// [`IdentusError`] at public crate boundaries.
#[derive(Debug)]
pub enum Error {
    /// A key byte slice had the wrong length.
    InvalidKeySize {
        expected: usize,
        actual: usize,
        key_type: &'static str,
    },
    /// A key (or other structured input) could not be parsed.
    KeyParsing {
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
    /// A signature failed verification.
    SignatureInvalid,
    /// The requested curve is not supported.
    UnsupportedCurve,
    /// Hierarchical (or other) key derivation failed.
    DerivationFailed,
    /// A mnemonic code is invalid (unknown word, bad length, bad checksum).
    MnemonicInvalid,
    /// Secure-random generation failed.
    SecureRandomFailure,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidKeySize {
                expected,
                actual,
                key_type,
            } => write!(
                f,
                "expected {key_type} key size to be {expected}, got size {actual}"
            ),
            Error::KeyParsing { source } => write!(f, "unable to parse key: {source}"),
            Error::SignatureInvalid => write!(f, "signature is invalid"),
            Error::UnsupportedCurve => write!(f, "unsupported curve"),
            Error::DerivationFailed => write!(f, "key derivation failed"),
            Error::MnemonicInvalid => write!(f, "invalid mnemonic code"),
            Error::SecureRandomFailure => write!(f, "secure random generation failed"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::KeyParsing { source } => Some(source.as_ref()),
            _ => None,
        }
    }
}

impl Error {
    /// Bridge to the redaction-safe [`IdentusError`] with a stable code and
    /// `CapabilityId("crypto")`. The public message is a `&'static str`
    /// carrying no runtime detail.
    pub fn to_identus_error(&self) -> IdentusError {
        let (code, kind, public_message) = match self {
            Error::InvalidKeySize { .. } => (
                error_code::INVALID_KEY_SIZE,
                ErrorKind::InvalidInput,
                "invalid key size",
            ),
            Error::KeyParsing { .. } => (
                error_code::KEY_PARSING,
                ErrorKind::InvalidInput,
                "key parsing failed",
            ),
            Error::SignatureInvalid => (
                error_code::SIGNATURE_INVALID,
                ErrorKind::VerificationFailed,
                "signature verification failed",
            ),
            Error::UnsupportedCurve => (
                error_code::UNSUPPORTED_CURVE,
                ErrorKind::Unsupported,
                "unsupported curve",
            ),
            Error::DerivationFailed => (
                error_code::DERIVATION_FAILED,
                ErrorKind::Crypto,
                "key derivation failed",
            ),
            Error::MnemonicInvalid => (
                error_code::MNEMONIC_INVALID,
                ErrorKind::InvalidInput,
                "invalid mnemonic",
            ),
            Error::SecureRandomFailure => (
                error_code::SECURE_RANDOM_FAILURE,
                ErrorKind::Crypto,
                "secure random generation failed",
            ),
        };
        IdentusError::public(code, kind, CAPABILITY, public_message)
    }
}

#[cfg(feature = "ed25519")]
#[derive(Debug)]
struct EdSignatureError(ed25519_dalek::SignatureError);

#[cfg(feature = "ed25519")]
impl fmt::Display for EdSignatureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(feature = "ed25519")]
impl std::error::Error for EdSignatureError {}

#[cfg(feature = "ed25519")]
impl From<ed25519_dalek::SignatureError> for Error {
    fn from(source: ed25519_dalek::SignatureError) -> Self {
        Error::KeyParsing {
            source: Box::new(EdSignatureError(source)),
        }
    }
}

// `k256` and `p256` both re-export the same `elliptic-curve` crate's `Error`,
// so a single wrapper + `From` impl covers both curves (avoiding E0119). The
// `elliptic-curve` `std` feature is intentionally NOT enabled (it would pull
// `getrandom` via `rand_core/std`, breaking the `wasm32` build), so we wrap the
// error and impl `std::error::Error` ourselves (the underlying type impls
// `Debug` + `Display` without `std`).
#[cfg(feature = "secp256k1")]
type EllipticCurveError = k256::elliptic_curve::Error;
#[cfg(all(not(feature = "secp256k1"), feature = "secp256r1"))]
type EllipticCurveError = p256::elliptic_curve::Error;

#[cfg(any(feature = "secp256k1", feature = "secp256r1"))]
#[derive(Debug)]
struct EcKeyError(EllipticCurveError);

#[cfg(any(feature = "secp256k1", feature = "secp256r1"))]
impl fmt::Display for EcKeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Delegate to the underlying `elliptic_curve::Error` `Display`.
        self.0.fmt(f)
    }
}

#[cfg(any(feature = "secp256k1", feature = "secp256r1"))]
impl std::error::Error for EcKeyError {}

#[cfg(any(feature = "secp256k1", feature = "secp256r1"))]
impl From<EllipticCurveError> for Error {
    fn from(source: EllipticCurveError) -> Self {
        Error::KeyParsing {
            source: Box::new(EcKeyError(source)),
        }
    }
}
