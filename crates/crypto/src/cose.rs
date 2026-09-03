//! Validated, public-only CBOR Object Signing and Encryption (COSE) keys.
//!
//! This module validates key representation. It does not assign key-use,
//! algorithm or trust policy, and it does not prove that an EC point is on the
//! declared curve.

use std::fmt;

use coset::cbor::value::Value;
use coset::iana;
use coset::{AsCborValue, CoseKey, KeyType, Label, RegisteredLabel};
use identus_core::{ErrorKind, IdentusError};

use crate::error::{CAPABILITY, error_code};

const COORDINATE_SIZE: usize = 32;
const KID_LABEL: i64 = 2;
const BASE_IV_LABEL: i64 = 5;
const CURVE_LABEL: i64 = -1;
const X_LABEL: i64 = -2;
const Y_LABEL: i64 = -3;
const PRIVATE_LABEL: i64 = -4;

/// Maximum accepted encoded COSE Key size.
pub const MAX_COSE_KEY_BYTES: usize = 4096;
/// Maximum retained non-structural top-level parameters.
pub const MAX_COSE_ADDITIONAL_PARAMETERS: usize = 32;
/// Maximum accepted CBOR nesting depth.
pub const MAX_COSE_NESTING_DEPTH: usize = 16;

/// Supported COSE key types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CoseKeyType {
    /// Octet key pair with one public byte string.
    Okp,
    /// Elliptic curve key with a coordinate pair.
    Ec2,
}

impl CoseKeyType {
    /// Return the assigned COSE registry value.
    #[must_use]
    pub const fn assigned(self) -> i64 {
        match self {
            Self::Okp => 1,
            Self::Ec2 => 2,
        }
    }

    /// Return the registered text spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Okp => "OKP",
            Self::Ec2 => "EC2",
        }
    }
}

impl fmt::Display for CoseKeyType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Supported COSE elliptic curves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CoseCurve {
    /// Ed25519 signature key.
    Ed25519,
    /// X25519 key-agreement key.
    X25519,
    /// NIST P-256 key.
    P256,
    /// SECG secp256k1 key.
    Secp256k1,
}

impl CoseCurve {
    /// Return the assigned COSE elliptic-curve registry value.
    #[must_use]
    pub const fn assigned(self) -> i64 {
        match self {
            Self::P256 => 1,
            Self::X25519 => 4,
            Self::Ed25519 => 6,
            Self::Secp256k1 => 8,
        }
    }

    /// Return the registered text spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ed25519 => "Ed25519",
            Self::X25519 => "X25519",
            Self::P256 => "P-256",
            Self::Secp256k1 => "secp256k1",
        }
    }

    /// Return the only compatible COSE key type.
    #[must_use]
    pub const fn key_type(self) -> CoseKeyType {
        match self {
            Self::Ed25519 | Self::X25519 => CoseKeyType::Okp,
            Self::P256 | Self::Secp256k1 => CoseKeyType::Ec2,
        }
    }
}

impl fmt::Display for CoseCurve {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// An EC2 `y` parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CoseEcY {
    /// Full affine coordinate.
    Coordinate([u8; COORDINATE_SIZE]),
    /// Registered compressed-point sign bit.
    Sign(bool),
}

/// A coordinate named in a validation error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CoseCoordinate {
    /// The public `x` parameter.
    X,
    /// The EC2-only public `y` parameter.
    Y,
}

impl fmt::Display for CoseCoordinate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::X => "x",
            Self::Y => "y",
        })
    }
}

/// Errors produced at the public COSE Key boundary.
///
/// Variants contain invariant names and lengths, never raw CBOR or parameter
/// values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoseKeyError {
    /// The encoded input exceeds the key-object bound.
    InputTooLarge { max: usize, actual: usize },
    /// The CBOR item could not be decoded within the configured bounds.
    InvalidCbor,
    /// The top-level CBOR item is not an untagged map.
    ExpectedMap,
    /// Another CBOR item follows the key object.
    TrailingData,
    /// A map contains the same deterministic key more than once.
    DuplicateMapKey,
    /// A floating-point extension cannot be emitted in the required profile.
    FloatingPointValue,
    /// The COSE key type is not one of the supported public profiles.
    UnsupportedKeyType,
    /// The key has no curve parameter.
    MissingCurve,
    /// The curve is not one of the supported profiles.
    UnsupportedCurve,
    /// The key type and curve are incompatible.
    IncompatibleProfile {
        key_type: CoseKeyType,
        curve: CoseCurve,
    },
    /// The public `x` parameter is absent.
    MissingXCoordinate,
    /// An EC2 key omitted `y`.
    MissingYCoordinate,
    /// An OKP key supplied `y`.
    UnexpectedYCoordinate,
    /// A coordinate has the wrong CBOR type.
    InvalidCoordinateType { coordinate: CoseCoordinate },
    /// A coordinate has the wrong byte length.
    InvalidCoordinateLength {
        coordinate: CoseCoordinate,
        expected: usize,
        actual: usize,
    },
    /// Private key material was supplied to the public-only type.
    PrivateKeyMaterial,
    /// Too many non-structural parameters were supplied.
    TooManyParameters { max: usize, actual: usize },
    /// Deterministic CBOR encoding failed.
    EncodingFailed,
    /// A compressed EC2 point cannot be represented as a JWK without
    /// decompression.
    CompressedCoordinate,
}

impl CoseKeyError {
    /// Bridge to the stable, redaction-safe SDK error surface.
    pub fn to_identus_error(&self) -> IdentusError {
        IdentusError::public(
            error_code::INVALID_COSE_KEY,
            ErrorKind::InvalidInput,
            CAPABILITY,
            "invalid public COSE Key",
        )
    }
}

impl fmt::Display for CoseKeyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputTooLarge { max, actual } => {
                write!(
                    formatter,
                    "COSE Key must be at most {max} bytes, got {actual}"
                )
            }
            Self::InvalidCbor => formatter.write_str("COSE Key contains invalid CBOR"),
            Self::ExpectedMap => formatter.write_str("COSE Key must be an untagged CBOR map"),
            Self::TrailingData => formatter.write_str("COSE Key has trailing CBOR data"),
            Self::DuplicateMapKey => formatter.write_str("COSE Key contains a duplicate map key"),
            Self::FloatingPointValue => {
                formatter.write_str("COSE Key extensions must not contain floating-point values")
            }
            Self::UnsupportedKeyType => formatter.write_str("unsupported COSE key type"),
            Self::MissingCurve => formatter.write_str("COSE Key is missing its curve"),
            Self::UnsupportedCurve => formatter.write_str("unsupported COSE curve"),
            Self::IncompatibleProfile { key_type, curve } => write!(
                formatter,
                "COSE key type {key_type} is incompatible with curve {curve}"
            ),
            Self::MissingXCoordinate => formatter.write_str("COSE Key is missing its x coordinate"),
            Self::MissingYCoordinate => {
                formatter.write_str("EC2 COSE Key is missing its y coordinate")
            }
            Self::UnexpectedYCoordinate => {
                formatter.write_str("OKP COSE Key must not contain a y coordinate")
            }
            Self::InvalidCoordinateType { coordinate } => {
                write!(
                    formatter,
                    "COSE Key {coordinate} coordinate has an invalid type"
                )
            }
            Self::InvalidCoordinateLength {
                coordinate,
                expected,
                actual,
            } => write!(
                formatter,
                "COSE Key {coordinate} coordinate must be {expected} bytes, got {actual}"
            ),
            Self::PrivateKeyMaterial => {
                formatter.write_str("public COSE Key must not contain private key material")
            }
            Self::TooManyParameters { max, actual } => write!(
                formatter,
                "COSE Key must contain at most {max} additional parameters, got {actual}"
            ),
            Self::EncodingFailed => formatter.write_str("COSE Key encoding failed"),
            Self::CompressedCoordinate => {
                formatter.write_str("compressed EC2 coordinate cannot be converted to JWK")
            }
        }
    }
}

impl std::error::Error for CoseKeyError {}

/// A validated public COSE Key.
///
/// Unknown public/common parameters are retained for wire round trips but are
/// deliberately not interpreted or exposed through mutable codec types.
#[derive(Clone, PartialEq)]
pub struct PublicKeyCose {
    key_type: CoseKeyType,
    curve: CoseCurve,
    x: [u8; COORDINATE_SIZE],
    y: Option<CoseEcY>,
    additional_parameters: usize,
    wire: Value,
}

impl fmt::Debug for PublicKeyCose {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PublicKeyCose")
            .field("key_type", &self.key_type)
            .field("curve", &self.curve)
            .field("x", &"[public coordinate]")
            .field(
                "y",
                &self.y.map(|value| match value {
                    CoseEcY::Coordinate(_) => "full coordinate",
                    CoseEcY::Sign(_) => "sign bit",
                }),
            )
            .field("additional_parameters", &self.additional_parameters)
            .finish()
    }
}

impl PublicKeyCose {
    /// Construct an OKP public key from exact-width bytes.
    pub fn new_okp(curve: CoseCurve, x: [u8; COORDINATE_SIZE]) -> Result<Self, CoseKeyError> {
        let wire = CoseKey {
            kty: KeyType::Assigned(iana::KeyType::OKP),
            params: vec![
                (Label::Int(CURVE_LABEL), Value::from(curve.assigned())),
                (Label::Int(X_LABEL), Value::Bytes(x.to_vec())),
            ],
            ..CoseKey::default()
        };
        Self::from_cose_key(wire)
    }

    /// Construct an EC2 public key with full affine coordinates.
    pub fn new_ec(
        curve: CoseCurve,
        x: [u8; COORDINATE_SIZE],
        y: [u8; COORDINATE_SIZE],
    ) -> Result<Self, CoseKeyError> {
        Self::new_ec_y(curve, x, CoseEcY::Coordinate(y))
    }

    /// Construct an EC2 public key with a compressed-point sign bit.
    pub fn new_ec_compressed(
        curve: CoseCurve,
        x: [u8; COORDINATE_SIZE],
        y_sign: bool,
    ) -> Result<Self, CoseKeyError> {
        Self::new_ec_y(curve, x, CoseEcY::Sign(y_sign))
    }

    fn new_ec_y(
        curve: CoseCurve,
        x: [u8; COORDINATE_SIZE],
        y: CoseEcY,
    ) -> Result<Self, CoseKeyError> {
        let y = match y {
            CoseEcY::Coordinate(value) => Value::Bytes(value.to_vec()),
            CoseEcY::Sign(value) => Value::Bool(value),
        };
        let wire = CoseKey {
            kty: KeyType::Assigned(iana::KeyType::EC2),
            params: vec![
                (Label::Int(CURVE_LABEL), Value::from(curve.assigned())),
                (Label::Int(X_LABEL), Value::Bytes(x.to_vec())),
                (Label::Int(Y_LABEL), y),
            ],
            ..CoseKey::default()
        };
        Self::from_cose_key(wire)
    }

    /// Parse one bounded, untagged public COSE Key.
    pub fn from_cbor(encoded: &[u8]) -> Result<Self, CoseKeyError> {
        if encoded.len() > MAX_COSE_KEY_BYTES {
            return Err(CoseKeyError::InputTooLarge {
                max: MAX_COSE_KEY_BYTES,
                actual: encoded.len(),
            });
        }

        let mut input = encoded;
        let value =
            coset::cbor::de::from_reader_with_recursion_limit(&mut input, MAX_COSE_NESTING_DEPTH)
                .map_err(|_| CoseKeyError::InvalidCbor)?;
        if !input.is_empty() {
            return Err(CoseKeyError::TrailingData);
        }
        if !matches!(value, Value::Map(_)) {
            return Err(CoseKeyError::ExpectedMap);
        }

        Self::from_value(value)
    }

    /// Emit deterministic, untagged CBOR for this public key.
    pub fn to_cbor(&self) -> Result<Vec<u8>, CoseKeyError> {
        let mut value = self.wire.clone();
        normalize_value(&mut value)?;
        encode_value(&value)
    }

    /// Return the key type.
    #[must_use]
    pub const fn key_type(&self) -> CoseKeyType {
        self.key_type
    }

    /// Return the curve.
    #[must_use]
    pub const fn curve(&self) -> CoseCurve {
        self.curve
    }

    /// Borrow the exact public `x` bytes.
    #[must_use]
    pub const fn x(&self) -> &[u8; COORDINATE_SIZE] {
        &self.x
    }

    /// Return the EC2 `y` representation, when present.
    #[must_use]
    pub const fn y(&self) -> Option<CoseEcY> {
        self.y
    }

    /// Return the number of retained non-structural parameters.
    #[must_use]
    pub const fn additional_parameter_count(&self) -> usize {
        self.additional_parameters
    }

    fn from_cose_key(wire: CoseKey) -> Result<Self, CoseKeyError> {
        let value = wire
            .to_cbor_value()
            .map_err(|_| CoseKeyError::EncodingFailed)?;
        Self::from_value(value)
    }

    fn from_value(mut value: Value) -> Result<Self, CoseKeyError> {
        normalize_value(&mut value)?;
        let additional_parameters = count_additional_parameters(&value)?;
        let interpretation = cose_interpretation_value(&value);
        let wire =
            CoseKey::from_cbor_value(interpretation).map_err(|_| CoseKeyError::InvalidCbor)?;

        // Reject private material before profile dispatch so `-4` cannot be
        // hidden behind an unsupported or deliberately confusing `kty`.
        if wire
            .params
            .iter()
            .any(|(label, _)| matches!(label, Label::Int(PRIVATE_LABEL)))
        {
            return Err(CoseKeyError::PrivateKeyMaterial);
        }

        let key_type = parse_key_type(&wire.kty)?;

        let mut curve = None;
        let mut x = None;
        let mut y = None;

        for (label, value) in &wire.params {
            match label {
                Label::Int(CURVE_LABEL) => {
                    let parsed = parse_curve(value)?;
                    curve = Some(parsed);
                }
                Label::Int(X_LABEL) => {
                    x = Some(parse_coordinate(value, CoseCoordinate::X)?);
                }
                Label::Int(Y_LABEL) => {
                    y = Some(parse_y(value)?);
                }
                _ => {}
            }
        }

        if additional_parameters > MAX_COSE_ADDITIONAL_PARAMETERS {
            return Err(CoseKeyError::TooManyParameters {
                max: MAX_COSE_ADDITIONAL_PARAMETERS,
                actual: additional_parameters,
            });
        }

        let curve = curve.ok_or(CoseKeyError::MissingCurve)?;
        if curve.key_type() != key_type {
            return Err(CoseKeyError::IncompatibleProfile { key_type, curve });
        }
        let x = x.ok_or(CoseKeyError::MissingXCoordinate)?;
        match (key_type, y) {
            (CoseKeyType::Ec2, None) => return Err(CoseKeyError::MissingYCoordinate),
            (CoseKeyType::Okp, Some(_)) => return Err(CoseKeyError::UnexpectedYCoordinate),
            _ => {}
        }

        normalize_structural_identifiers(&mut value, key_type, curve)?;

        Ok(Self {
            key_type,
            curve,
            x,
            y,
            additional_parameters,
            wire: value,
        })
    }
}

fn count_additional_parameters(value: &Value) -> Result<usize, CoseKeyError> {
    let Value::Map(entries) = value else {
        return Err(CoseKeyError::ExpectedMap);
    };
    Ok(entries
        .iter()
        .filter(|(label, _)| {
            !matches!(
                integer_value(label),
                Some(1 | CURVE_LABEL | X_LABEL | Y_LABEL)
            )
        })
        .count())
}

fn cose_interpretation_value(value: &Value) -> Value {
    let mut interpretation = value.clone();
    let Value::Map(entries) = &mut interpretation else {
        return interpretation;
    };
    // RFC 9052 permits empty byte strings for `kid` and Base IV, while coset
    // intentionally treats empty named fields as absent and rejects them.
    // Omit them only from the temporary typed view; the normalized source map
    // remains the round-trip source of truth.
    entries.retain(|(label, value)| {
        !matches!(integer_value(label), Some(KID_LABEL | BASE_IV_LABEL))
            || !matches!(value, Value::Bytes(bytes) if bytes.is_empty())
    });
    interpretation
}

fn normalize_structural_identifiers(
    value: &mut Value,
    key_type: CoseKeyType,
    curve: CoseCurve,
) -> Result<(), CoseKeyError> {
    let Value::Map(entries) = value else {
        return Err(CoseKeyError::ExpectedMap);
    };
    for (label, value) in entries {
        match integer_value(label) {
            Some(1) => *value = Value::from(key_type.assigned()),
            Some(CURVE_LABEL) => *value = Value::from(curve.assigned()),
            _ => {}
        }
    }
    Ok(())
}

fn integer_value(value: &Value) -> Option<i64> {
    let Value::Integer(value) = value else {
        return None;
    };
    i64::try_from(*value).ok()
}

fn parse_key_type(key_type: &KeyType) -> Result<CoseKeyType, CoseKeyError> {
    match key_type {
        RegisteredLabel::Assigned(iana::KeyType::OKP) => Ok(CoseKeyType::Okp),
        RegisteredLabel::Assigned(iana::KeyType::EC2) => Ok(CoseKeyType::Ec2),
        RegisteredLabel::Text(value) if value == "OKP" => Ok(CoseKeyType::Okp),
        RegisteredLabel::Text(value) if value == "EC2" => Ok(CoseKeyType::Ec2),
        _ => Err(CoseKeyError::UnsupportedKeyType),
    }
}

fn parse_curve(value: &Value) -> Result<CoseCurve, CoseKeyError> {
    match value {
        Value::Integer(value) => match i64::try_from(*value).ok() {
            Some(1) => Ok(CoseCurve::P256),
            Some(4) => Ok(CoseCurve::X25519),
            Some(6) => Ok(CoseCurve::Ed25519),
            Some(8) => Ok(CoseCurve::Secp256k1),
            _ => Err(CoseKeyError::UnsupportedCurve),
        },
        Value::Text(value) => match value.as_str() {
            "P-256" => Ok(CoseCurve::P256),
            "X25519" => Ok(CoseCurve::X25519),
            "Ed25519" => Ok(CoseCurve::Ed25519),
            "secp256k1" => Ok(CoseCurve::Secp256k1),
            _ => Err(CoseKeyError::UnsupportedCurve),
        },
        _ => Err(CoseKeyError::UnsupportedCurve),
    }
}

fn parse_coordinate(
    value: &Value,
    coordinate: CoseCoordinate,
) -> Result<[u8; COORDINATE_SIZE], CoseKeyError> {
    let Value::Bytes(value) = value else {
        return Err(CoseKeyError::InvalidCoordinateType { coordinate });
    };
    value
        .as_slice()
        .try_into()
        .map_err(|_| CoseKeyError::InvalidCoordinateLength {
            coordinate,
            expected: COORDINATE_SIZE,
            actual: value.len(),
        })
}

fn parse_y(value: &Value) -> Result<CoseEcY, CoseKeyError> {
    match value {
        Value::Bytes(_) => parse_coordinate(value, CoseCoordinate::Y).map(CoseEcY::Coordinate),
        Value::Bool(value) => Ok(CoseEcY::Sign(*value)),
        _ => Err(CoseKeyError::InvalidCoordinateType {
            coordinate: CoseCoordinate::Y,
        }),
    }
}

fn normalize_value(value: &mut Value) -> Result<(), CoseKeyError> {
    match value {
        Value::Float(_) => return Err(CoseKeyError::FloatingPointValue),
        Value::Tag(_, value) => normalize_value(value)?,
        Value::Array(values) => {
            for value in values {
                normalize_value(value)?;
            }
        }
        Value::Map(entries) => {
            let mut keyed = Vec::with_capacity(entries.len());
            for (mut key, mut value) in std::mem::take(entries) {
                normalize_value(&mut key)?;
                normalize_value(&mut value)?;
                keyed.push((encode_value(&key)?, key, value));
            }
            keyed.sort_by(|left, right| {
                left.0
                    .len()
                    .cmp(&right.0.len())
                    .then_with(|| left.0.cmp(&right.0))
            });
            if keyed.windows(2).any(|pair| pair[0].0 == pair[1].0) {
                return Err(CoseKeyError::DuplicateMapKey);
            }
            *entries = keyed
                .into_iter()
                .map(|(_, key, value)| (key, value))
                .collect();
        }
        Value::Integer(_) | Value::Bytes(_) | Value::Text(_) | Value::Bool(_) | Value::Null => {}
        _ => return Err(CoseKeyError::InvalidCbor),
    }
    Ok(())
}

fn encode_value(value: &Value) -> Result<Vec<u8>, CoseKeyError> {
    let mut encoded = Vec::new();
    coset::cbor::into_writer(value, &mut encoded).map_err(|_| CoseKeyError::EncodingFailed)?;
    Ok(encoded)
}

/// Encode a public key as a validated COSE Key.
pub trait EncodeCose {
    /// Return the public key material in its registered COSE profile.
    fn encode_cose(&self) -> PublicKeyCose;
}

#[cfg(feature = "jwk")]
mod jwk_conversion {
    use super::{CoseCurve, CoseEcY, CoseKeyError, PublicKeyCose};
    use crate::{JwkCurve, PublicKeyJwk};

    impl TryFrom<&PublicKeyJwk> for PublicKeyCose {
        type Error = CoseKeyError;

        fn try_from(jwk: &PublicKeyJwk) -> Result<Self, Self::Error> {
            let curve = match jwk.crv() {
                JwkCurve::Ed25519 => CoseCurve::Ed25519,
                JwkCurve::X25519 => CoseCurve::X25519,
                JwkCurve::P256 => CoseCurve::P256,
                JwkCurve::Secp256k1 => CoseCurve::Secp256k1,
            };
            let x = jwk
                .x()
                .to_bytes()
                .try_into()
                .expect("validated JWK x coordinate is 32 bytes");
            match jwk.y() {
                Some(y) => PublicKeyCose::new_ec(
                    curve,
                    x,
                    y.to_bytes()
                        .try_into()
                        .expect("validated JWK y coordinate is 32 bytes"),
                ),
                None => PublicKeyCose::new_okp(curve, x),
            }
        }
    }

    impl TryFrom<&PublicKeyCose> for PublicKeyJwk {
        type Error = CoseKeyError;

        fn try_from(cose: &PublicKeyCose) -> Result<Self, Self::Error> {
            let curve = match cose.curve() {
                CoseCurve::Ed25519 => JwkCurve::Ed25519,
                CoseCurve::X25519 => JwkCurve::X25519,
                CoseCurve::P256 => JwkCurve::P256,
                CoseCurve::Secp256k1 => JwkCurve::Secp256k1,
            };
            match cose.y() {
                Some(CoseEcY::Coordinate(y)) => PublicKeyJwk::new_ec(curve, *cose.x(), y)
                    .map_err(|_| CoseKeyError::EncodingFailed),
                Some(CoseEcY::Sign(_)) => Err(CoseKeyError::CompressedCoordinate),
                None => PublicKeyJwk::new_okp(curve, *cose.x())
                    .map_err(|_| CoseKeyError::EncodingFailed),
            }
        }
    }
}
