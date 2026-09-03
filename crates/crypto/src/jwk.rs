//! Validated, public-only JSON Web Key (JWK) values.
//!
//! The types in this module validate the structural JWK boundary. They do not
//! assign key-use policy or prove that an elliptic-curve point is on-curve.

use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;

use identus_core::{ErrorKind, IdentusError};
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[cfg(feature = "jwk-thumbprint")]
use sha2::{Digest, Sha256};

use crate::base64::Base64UrlStrNoPad;
use crate::error::{CAPABILITY, error_code};

const COORDINATE_SIZE: usize = 32;
const PRIVATE_PARAMETER: &str = "d";
const STRUCTURAL_PARAMETERS: [&str; 4] = ["kty", "crv", "x", "y"];

/// Supported JSON Web Key types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JwkKeyType {
    /// Elliptic curve key with affine `x` and `y` coordinates.
    #[serde(rename = "EC")]
    Ec,
    /// Octet key pair with one public `x` byte string.
    #[serde(rename = "OKP")]
    Okp,
}

impl JwkKeyType {
    /// Return the case-sensitive JOSE registry spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ec => "EC",
            Self::Okp => "OKP",
        }
    }
}

impl fmt::Display for JwkKeyType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Supported JSON Web Key curves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JwkCurve {
    /// Ed25519 signature key, represented as an OKP JWK.
    Ed25519,
    /// X25519 key-agreement key, represented as an OKP JWK.
    X25519,
    /// NIST P-256 key, represented as an EC JWK.
    #[serde(rename = "P-256")]
    P256,
    /// secp256k1 key, represented as an EC JWK.
    #[serde(rename = "secp256k1")]
    Secp256k1,
}

impl JwkCurve {
    /// Return the case-sensitive JOSE registry spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ed25519 => "Ed25519",
            Self::X25519 => "X25519",
            Self::P256 => "P-256",
            Self::Secp256k1 => "secp256k1",
        }
    }

    /// Return the only key type compatible with this curve.
    #[must_use]
    pub const fn key_type(self) -> JwkKeyType {
        match self {
            Self::Ed25519 | Self::X25519 => JwkKeyType::Okp,
            Self::P256 | Self::Secp256k1 => JwkKeyType::Ec,
        }
    }
}

impl fmt::Display for JwkCurve {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A JWK coordinate named in a validation error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JwkCoordinate {
    /// The public `x` member.
    X,
    /// The EC-only public `y` member.
    Y,
}

impl fmt::Display for JwkCoordinate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::X => "x",
            Self::Y => "y",
        })
    }
}

/// Structural errors produced at the public-key JWK boundary.
///
/// Variants contain profile names and lengths but never rejected coordinate or
/// extension values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JwkError {
    /// The declared key type is incompatible with the declared curve.
    IncompatibleProfile {
        /// Declared key type.
        key_type: JwkKeyType,
        /// Declared curve.
        curve: JwkCurve,
    },
    /// An EC profile omitted its `y` coordinate.
    MissingYCoordinate,
    /// An OKP profile supplied a `y` coordinate.
    UnexpectedYCoordinate,
    /// A coordinate is not canonical unpadded base64url.
    InvalidCoordinateEncoding {
        /// Coordinate that failed validation.
        coordinate: JwkCoordinate,
    },
    /// A coordinate does not have the required decoded length.
    InvalidCoordinateLength {
        /// Coordinate that failed validation.
        coordinate: JwkCoordinate,
        /// Required decoded byte length.
        expected: usize,
        /// Actual decoded byte length.
        actual: usize,
    },
    /// A private `d` parameter was supplied to the public-only type.
    PrivateKeyMaterial,
    /// An extension attempted to shadow a structural JWK parameter.
    ReservedExtension,
}

impl JwkError {
    /// Bridge to the stable, redaction-safe SDK error surface.
    pub fn to_identus_error(&self) -> IdentusError {
        IdentusError::public(
            error_code::INVALID_JWK,
            ErrorKind::InvalidInput,
            CAPABILITY,
            "invalid public JSON Web Key",
        )
    }
}

impl fmt::Display for JwkError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IncompatibleProfile { key_type, curve } => {
                write!(
                    formatter,
                    "JWK key type {key_type} is incompatible with curve {curve}"
                )
            }
            Self::MissingYCoordinate => formatter.write_str("EC JWK is missing its y coordinate"),
            Self::UnexpectedYCoordinate => {
                formatter.write_str("OKP JWK must not contain a y coordinate")
            }
            Self::InvalidCoordinateEncoding { coordinate } => write!(
                formatter,
                "JWK {coordinate} coordinate is not canonical unpadded base64url"
            ),
            Self::InvalidCoordinateLength {
                coordinate,
                expected,
                actual,
            } => write!(
                formatter,
                "JWK {coordinate} coordinate must decode to {expected} bytes, got {actual}"
            ),
            Self::PrivateKeyMaterial => {
                formatter.write_str("public JWK must not contain private key material")
            }
            Self::ReservedExtension => {
                formatter.write_str("JWK extension must not shadow a structural member")
            }
        }
    }
}

impl std::error::Error for JwkError {}

/// An RFC 7638 SHA-256 thumbprint of a public JSON Web Key.
///
/// A thumbprint identifies the required public key material only. It does not
/// authorize the key or bind optional JWK members such as `alg`, `kid`, `use`,
/// or `key_ops`.
#[cfg(feature = "jwk-thumbprint")]
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JwkThumbprint([u8; 32]);

#[cfg(feature = "jwk-thumbprint")]
impl JwkThumbprint {
    /// Borrow the 32-byte SHA-256 digest.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Encode the thumbprint as canonical unpadded base64url.
    #[must_use]
    pub fn to_base64url(&self) -> Base64UrlStrNoPad {
        Base64UrlStrNoPad::from(self.0)
    }
}

#[cfg(feature = "jwk-thumbprint")]
impl fmt::Display for JwkThumbprint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_base64url().fmt(formatter)
    }
}

/// A validated public-key JWK.
///
/// The type cannot contain the private `d` member. Unknown public members are
/// preserved as uninterpreted extensions for lossless resolver round trips.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PublicKeyJwk {
    kty: JwkKeyType,
    crv: JwkCurve,
    x: Base64UrlStrNoPad,
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<Base64UrlStrNoPad>,
    #[serde(flatten)]
    extensions: BTreeMap<String, Value>,
}

impl PublicKeyJwk {
    /// Construct an OKP public JWK from an exact-width public byte string.
    ///
    /// # Errors
    ///
    /// Returns [`JwkError::IncompatibleProfile`] when `curve` is not an OKP
    /// curve.
    pub fn new_okp(curve: JwkCurve, x: [u8; COORDINATE_SIZE]) -> Result<Self, JwkError> {
        let x = Base64UrlStrNoPad::from(x);
        Self::from_parts(JwkKeyType::Okp, curve, x.as_str(), None, BTreeMap::new())
    }

    /// Construct an EC public JWK from exact-width affine coordinates.
    ///
    /// # Errors
    ///
    /// Returns [`JwkError::IncompatibleProfile`] when `curve` is not an EC
    /// curve.
    pub fn new_ec(
        curve: JwkCurve,
        x: [u8; COORDINATE_SIZE],
        y: [u8; COORDINATE_SIZE],
    ) -> Result<Self, JwkError> {
        let x = Base64UrlStrNoPad::from(x);
        let y = Base64UrlStrNoPad::from(y);
        Self::from_parts(
            JwkKeyType::Ec,
            curve,
            x.as_str(),
            Some(y.as_str()),
            BTreeMap::new(),
        )
    }

    /// Parse and validate a public JWK from wire-shaped parts.
    ///
    /// Unknown public extension members are preserved. The private `d` member
    /// and structural names in `extensions` are rejected.
    ///
    /// # Errors
    ///
    /// Returns a [`JwkError`] when the profile, shape, encoding, coordinate
    /// length, or extension set violates the public-key boundary.
    pub fn from_parts(
        kty: JwkKeyType,
        crv: JwkCurve,
        x: &str,
        y: Option<&str>,
        extensions: BTreeMap<String, Value>,
    ) -> Result<Self, JwkError> {
        if kty != crv.key_type() {
            return Err(JwkError::IncompatibleProfile {
                key_type: kty,
                curve: crv,
            });
        }
        match (kty, y) {
            (JwkKeyType::Ec, None) => return Err(JwkError::MissingYCoordinate),
            (JwkKeyType::Okp, Some(_)) => return Err(JwkError::UnexpectedYCoordinate),
            _ => {}
        }
        if extensions.contains_key(PRIVATE_PARAMETER) {
            return Err(JwkError::PrivateKeyMaterial);
        }
        if STRUCTURAL_PARAMETERS
            .iter()
            .any(|parameter| extensions.contains_key(*parameter))
        {
            return Err(JwkError::ReservedExtension);
        }

        let x = parse_coordinate(x, JwkCoordinate::X)?;
        let y = y
            .map(|coordinate| parse_coordinate(coordinate, JwkCoordinate::Y))
            .transpose()?;

        Ok(Self {
            kty,
            crv,
            x,
            y,
            extensions,
        })
    }

    /// Return the key type.
    #[must_use]
    pub const fn kty(&self) -> JwkKeyType {
        self.kty
    }

    /// Return the curve.
    #[must_use]
    pub const fn crv(&self) -> JwkCurve {
        self.crv
    }

    /// Borrow the canonical unpadded base64url `x` coordinate.
    #[must_use]
    pub const fn x(&self) -> &Base64UrlStrNoPad {
        &self.x
    }

    /// Borrow the canonical unpadded base64url EC `y` coordinate, when present.
    #[must_use]
    pub fn y(&self) -> Option<&Base64UrlStrNoPad> {
        self.y.as_ref()
    }

    /// Borrow additional uninterpreted public JWK members.
    #[must_use]
    pub const fn extensions(&self) -> &BTreeMap<String, Value> {
        &self.extensions
    }

    /// Compute this public key's RFC 7638 SHA-256 JWK thumbprint.
    ///
    /// Only the required members participate. Optional and extension members
    /// are intentionally ignored, so they cannot change the key identifier.
    /// The canonical input is streamed into SHA-256 without JSON serialization
    /// or a canonicalization allocation.
    #[cfg(feature = "jwk-thumbprint")]
    pub fn thumbprint_sha256(&self) -> JwkThumbprint {
        let mut hasher = Sha256::new();
        self.visit_thumbprint_input(|part| hasher.update(part));
        JwkThumbprint(hasher.finalize().into())
    }

    #[cfg(feature = "jwk-thumbprint")]
    fn visit_thumbprint_input(&self, mut visit: impl FnMut(&[u8])) {
        visit(br#"{"crv":""#);
        visit(self.crv.as_str().as_bytes());
        match self.kty {
            JwkKeyType::Okp => visit(br#"","kty":"OKP","x":""#),
            JwkKeyType::Ec => visit(br#"","kty":"EC","x":""#),
        }
        visit(self.x.as_str().as_bytes());
        if let Some(y) = &self.y {
            visit(br#"","y":""#);
            visit(y.as_str().as_bytes());
        }
        visit(br#""}"#);
    }
}

fn parse_coordinate(value: &str, coordinate: JwkCoordinate) -> Result<Base64UrlStrNoPad, JwkError> {
    let encoded = Base64UrlStrNoPad::from_str(value)
        .map_err(|_| JwkError::InvalidCoordinateEncoding { coordinate })?;
    let actual = encoded.to_bytes().len();
    if actual != COORDINATE_SIZE {
        return Err(JwkError::InvalidCoordinateLength {
            coordinate,
            expected: COORDINATE_SIZE,
            actual,
        });
    }
    Ok(encoded)
}

#[derive(Deserialize)]
struct PublicKeyJwkWire {
    kty: JwkKeyType,
    crv: JwkCurve,
    x: String,
    #[serde(default)]
    y: Option<String>,
    #[serde(flatten)]
    extensions: BTreeMap<String, Value>,
}

impl<'de> Deserialize<'de> for PublicKeyJwk {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let wire = PublicKeyJwkWire::deserialize(deserializer)?;
        Self::from_parts(
            wire.kty,
            wire.crv,
            &wire.x,
            wire.y.as_deref(),
            wire.extensions,
        )
        .map_err(serde::de::Error::custom)
    }
}

/// Encode a public key as a validated [`PublicKeyJwk`].
pub trait EncodeJwk {
    /// Encode this public key as a validated [`PublicKeyJwk`].
    fn encode_jwk(&self) -> PublicKeyJwk;
}

#[cfg(all(test, feature = "jwk-thumbprint"))]
mod tests {
    use super::{JwkCurve, PublicKeyJwk};

    #[test]
    fn thumbprint_input_is_exact_for_each_supported_key_shape() {
        let x = [0_u8; 32];
        let y = [1_u8; 32];

        for (jwk, expected) in [
            (
                PublicKeyJwk::new_okp(JwkCurve::Ed25519, x).expect("OKP JWK"),
                r#"{"crv":"Ed25519","kty":"OKP","x":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"}"#,
            ),
            (
                PublicKeyJwk::new_ec(JwkCurve::P256, x, y).expect("EC JWK"),
                r#"{"crv":"P-256","kty":"EC","x":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","y":"AQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQE"}"#,
            ),
        ] {
            let mut actual = Vec::new();
            jwk.visit_thumbprint_input(|part| actual.extend_from_slice(part));
            assert_eq!(actual, expected.as_bytes());
        }
    }
}
