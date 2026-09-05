//! Closed protected-header representation for the bounded codec.

use std::fmt;

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use identus_crypto::PublicKeyJwk;
use serde::de::{Error as _, MapAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{JoseError, JwsLimits};

const MAX_ALGORITHM_BYTES: usize = 64;
const DUPLICATE_MARKER: &str = "identus-jose:duplicate-header";
const UNKNOWN_MARKER: &str = "identus-jose:unknown-header";
const MISSING_ALGORITHM_MARKER: &str = "identus-jose:missing-algorithm";
const INVALID_VALUE_MARKER: &str = "identus-jose:invalid-header-value";
const AMBIGUOUS_KEY_REFERENCE_MARKER: &str = "identus-jose:ambiguous-key-reference";

/// Maximum number of certificates accepted in one protected `x5c` chain.
pub const MAX_X5C_CERTIFICATES: usize = 8;

/// One exclusive public key reference carried by a protected JWS header.
#[derive(Clone, PartialEq, Eq)]
pub enum JwsKeyReference {
    /// A caller-interpreted key identifier.
    KeyId(String),
    /// A validated public-only JSON Web Key.
    Jwk(PublicKeyJwk),
    /// A leaf-first X.509 certificate chain encoded with standard base64.
    X5c(Vec<String>),
}

impl fmt::Debug for JwsKeyReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::KeyId(_) => formatter.write_str("JwsKeyReference::KeyId(..)"),
            Self::Jwk(_) => formatter.write_str("JwsKeyReference::Jwk(..)"),
            Self::X5c(values) => formatter
                .debug_tuple("JwsKeyReference::X5c")
                .field(&values.len())
                .finish(),
        }
    }
}

/// A validated, deliberately closed JWS protected header.
///
/// The supported surface is `alg`, optional `typ`, and at most one `kid`,
/// public `jwk`, or `x5c`. Algorithm allowlisting, certificate validation and
/// key authorization belong to verifier profiles.
#[derive(Clone, PartialEq, Eq)]
pub struct ProtectedHeader {
    algorithm: String,
    type_: Option<String>,
    key_reference: Option<JwsKeyReference>,
}

impl ProtectedHeader {
    /// Construct a protected header under the supplied string limit.
    pub fn new(
        algorithm: &str,
        type_: Option<&str>,
        key_id: Option<&str>,
        limits: JwsLimits,
    ) -> Result<Self, JoseError> {
        let key_reference = key_id.map(|value| JwsKeyReference::KeyId(value.to_owned()));
        Self::with_key_reference(algorithm, type_, key_reference, limits)
    }

    /// Construct a protected header with one exclusive public key reference.
    pub fn with_key_reference(
        algorithm: &str,
        type_: Option<&str>,
        key_reference: Option<JwsKeyReference>,
        limits: JwsLimits,
    ) -> Result<Self, JoseError> {
        validate_algorithm(algorithm, limits)?;
        if type_.is_some_and(|value| !valid_optional(value, limits))
            || key_reference
                .as_ref()
                .is_some_and(|value| !valid_key_reference(value, limits))
        {
            return Err(JoseError::InvalidHeaderValue);
        }
        Ok(Self {
            algorithm: algorithm.to_owned(),
            type_: type_.map(str::to_owned),
            key_reference,
        })
    }

    /// Case-sensitive algorithm identifier declared by the sender.
    ///
    /// This value is untrusted until a verifier binds it to an explicit
    /// allowlist, key, and signature suite.
    pub fn algorithm(&self) -> &str {
        &self.algorithm
    }

    /// Optional explicit media/type discriminator.
    pub fn type_(&self) -> Option<&str> {
        self.type_.as_deref()
    }

    /// Optional key identifier. It has not been resolved or authorized.
    pub fn key_id(&self) -> Option<&str> {
        match &self.key_reference {
            Some(JwsKeyReference::KeyId(value)) => Some(value),
            _ => None,
        }
    }

    /// Borrow the exclusive key-reference shape, when present.
    pub const fn key_reference(&self) -> Option<&JwsKeyReference> {
        self.key_reference.as_ref()
    }

    /// Borrow an inline validated public JWK, when selected.
    pub fn public_jwk(&self) -> Option<&PublicKeyJwk> {
        match &self.key_reference {
            Some(JwsKeyReference::Jwk(value)) => Some(value),
            _ => None,
        }
    }

    /// Borrow a leaf-first encoded X.509 chain, when selected.
    pub fn certificate_chain(&self) -> Option<&[String]> {
        match &self.key_reference {
            Some(JwsKeyReference::X5c(value)) => Some(value),
            _ => None,
        }
    }

    pub(crate) fn parse(bytes: &[u8], limits: JwsLimits) -> Result<Self, JoseError> {
        let raw = serde_json::from_slice::<RawProtectedHeader>(bytes).map_err(map_json_error)?;
        Self::with_key_reference(
            &raw.algorithm,
            raw.type_.as_deref(),
            raw.key_reference,
            limits,
        )
    }

    pub(crate) fn validate_for(&self, limits: JwsLimits) -> Result<(), JoseError> {
        validate_algorithm(&self.algorithm, limits)?;
        if self
            .type_
            .as_deref()
            .is_some_and(|value| !valid_optional(value, limits))
            || self
                .key_reference
                .as_ref()
                .is_some_and(|value| !valid_key_reference(value, limits))
        {
            return Err(JoseError::InvalidHeaderValue);
        }
        Ok(())
    }
}

impl fmt::Debug for ProtectedHeader {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProtectedHeader")
            .field("algorithm", &self.algorithm)
            .field("type", &self.type_)
            .field("key_reference", &self.key_reference_kind())
            .finish()
    }
}

impl Serialize for ProtectedHeader {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let member_count =
            1 + usize::from(self.type_.is_some()) + usize::from(self.key_reference.is_some());
        let mut state = serializer.serialize_struct("ProtectedHeader", member_count)?;
        state.serialize_field("alg", &self.algorithm)?;
        if let Some(type_) = &self.type_ {
            state.serialize_field("typ", type_)?;
        }
        match &self.key_reference {
            Some(JwsKeyReference::KeyId(value)) => state.serialize_field("kid", value)?,
            Some(JwsKeyReference::Jwk(value)) => state.serialize_field("jwk", value)?,
            Some(JwsKeyReference::X5c(value)) => state.serialize_field("x5c", value)?,
            None => {}
        }
        state.end()
    }
}

impl ProtectedHeader {
    fn key_reference_kind(&self) -> Option<&'static str> {
        self.key_reference.as_ref().map(|value| match value {
            JwsKeyReference::KeyId(_) => "kid",
            JwsKeyReference::Jwk(_) => "jwk",
            JwsKeyReference::X5c(_) => "x5c",
        })
    }
}

fn validate_algorithm(value: &str, limits: JwsLimits) -> Result<(), JoseError> {
    if value.is_empty()
        || value.len() > MAX_ALGORITHM_BYTES
        || value.len() > limits.max_header_string_bytes()
        || !value.bytes().all(|byte| byte.is_ascii_graphic())
        || value == "none"
    {
        return Err(JoseError::InvalidHeaderValue);
    }
    Ok(())
}

fn valid_optional(value: &str, limits: JwsLimits) -> bool {
    !value.is_empty()
        && value.len() <= limits.max_header_string_bytes()
        && !value.chars().any(char::is_control)
}

fn map_json_error(error: serde_json::Error) -> JoseError {
    let rendered = error.to_string();
    if rendered.contains(DUPLICATE_MARKER) {
        JoseError::DuplicateProtectedHeader
    } else if rendered.contains(UNKNOWN_MARKER) {
        JoseError::UnknownProtectedHeader
    } else if rendered.contains(MISSING_ALGORITHM_MARKER) {
        JoseError::MissingAlgorithm
    } else if rendered.contains(INVALID_VALUE_MARKER) {
        JoseError::InvalidHeaderValue
    } else if rendered.contains(AMBIGUOUS_KEY_REFERENCE_MARKER) {
        JoseError::AmbiguousKeyReference
    } else {
        JoseError::InvalidProtectedHeader
    }
}

struct RawProtectedHeader {
    algorithm: String,
    type_: Option<String>,
    key_reference: Option<JwsKeyReference>,
}

impl<'de> Deserialize<'de> for RawProtectedHeader {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(RawProtectedHeaderVisitor)
    }
}

struct RawProtectedHeaderVisitor;

impl<'de> Visitor<'de> for RawProtectedHeaderVisitor {
    type Value = RawProtectedHeader;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a supported JWS protected header object")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut algorithm = None;
        let mut type_ = None;
        let mut key_id = None;
        let mut jwk = None;
        let mut x5c = None;
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "alg" => read_string(&mut map, &mut algorithm)?,
                "typ" => read_string(&mut map, &mut type_)?,
                "kid" => read_string(&mut map, &mut key_id)?,
                "jwk" => {
                    if jwk.is_some() {
                        return Err(M::Error::custom(DUPLICATE_MARKER));
                    }
                    jwk = Some(
                        map.next_value::<PublicKeyJwk>()
                            .map_err(|_| M::Error::custom(INVALID_VALUE_MARKER))?,
                    );
                }
                "x5c" => {
                    if x5c.is_some() {
                        return Err(M::Error::custom(DUPLICATE_MARKER));
                    }
                    x5c = Some(
                        map.next_value::<Vec<String>>()
                            .map_err(|_| M::Error::custom(INVALID_VALUE_MARKER))?,
                    );
                }
                _ => return Err(M::Error::custom(UNKNOWN_MARKER)),
            }
        }
        let algorithm = algorithm.ok_or_else(|| M::Error::custom(MISSING_ALGORITHM_MARKER))?;
        if usize::from(key_id.is_some()) + usize::from(jwk.is_some()) + usize::from(x5c.is_some())
            > 1
        {
            return Err(M::Error::custom(AMBIGUOUS_KEY_REFERENCE_MARKER));
        }
        let key_reference = key_id
            .map(JwsKeyReference::KeyId)
            .or_else(|| jwk.map(JwsKeyReference::Jwk))
            .or_else(|| x5c.map(JwsKeyReference::X5c));
        Ok(RawProtectedHeader {
            algorithm,
            type_,
            key_reference,
        })
    }
}

fn read_string<'de, M>(map: &mut M, target: &mut Option<String>) -> Result<(), M::Error>
where
    M: MapAccess<'de>,
{
    if target.is_some() {
        return Err(M::Error::custom(DUPLICATE_MARKER));
    }
    let value = map.next_value::<serde_json::Value>()?;
    *target = Some(
        value
            .as_str()
            .ok_or_else(|| M::Error::custom(INVALID_VALUE_MARKER))?
            .to_owned(),
    );
    Ok(())
}

fn valid_key_reference(value: &JwsKeyReference, limits: JwsLimits) -> bool {
    match value {
        JwsKeyReference::KeyId(value) => valid_optional(value, limits),
        JwsKeyReference::Jwk(_) => true,
        JwsKeyReference::X5c(values) => {
            !values.is_empty()
                && values.len() <= MAX_X5C_CERTIFICATES
                && values.iter().all(|value| {
                    valid_optional(value, limits)
                        && STANDARD.decode(value).is_ok_and(|bytes| !bytes.is_empty())
                })
        }
    }
}
