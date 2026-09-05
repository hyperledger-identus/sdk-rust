//! Closed protected-header representation for the bounded codec.

use std::fmt;

use serde::de::{Error as _, MapAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{JoseError, JwsLimits};

const MAX_ALGORITHM_BYTES: usize = 64;
const DUPLICATE_MARKER: &str = "identus-jose:duplicate-header";
const UNKNOWN_MARKER: &str = "identus-jose:unknown-header";
const MISSING_ALGORITHM_MARKER: &str = "identus-jose:missing-algorithm";
const INVALID_VALUE_MARKER: &str = "identus-jose:invalid-header-value";

/// A validated, deliberately closed JWS protected header.
///
/// The first slice supports only `alg`, optional `typ`, and optional `kid`.
/// Algorithm allowlisting and key selection belong to a verifier profile.
#[derive(Clone, PartialEq, Eq)]
pub struct ProtectedHeader {
    algorithm: String,
    type_: Option<String>,
    key_id: Option<String>,
}

impl ProtectedHeader {
    /// Construct a protected header under the supplied string limit.
    pub fn new(
        algorithm: &str,
        type_: Option<&str>,
        key_id: Option<&str>,
        limits: JwsLimits,
    ) -> Result<Self, JoseError> {
        validate_algorithm(algorithm, limits)?;
        if type_.is_some_and(|value| !valid_optional(value, limits))
            || key_id.is_some_and(|value| !valid_optional(value, limits))
        {
            return Err(JoseError::InvalidHeaderValue);
        }
        Ok(Self {
            algorithm: algorithm.to_owned(),
            type_: type_.map(str::to_owned),
            key_id: key_id.map(str::to_owned),
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
        self.key_id.as_deref()
    }

    pub(crate) fn parse(bytes: &[u8], limits: JwsLimits) -> Result<Self, JoseError> {
        let raw = serde_json::from_slice::<RawProtectedHeader>(bytes).map_err(map_json_error)?;
        Self::new(
            &raw.algorithm,
            raw.type_.as_deref(),
            raw.key_id.as_deref(),
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
                .key_id
                .as_deref()
                .is_some_and(|value| !valid_optional(value, limits))
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
            .field("has_key_id", &self.key_id.is_some())
            .finish()
    }
}

impl Serialize for ProtectedHeader {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let member_count =
            1 + usize::from(self.type_.is_some()) + usize::from(self.key_id.is_some());
        let mut state = serializer.serialize_struct("ProtectedHeader", member_count)?;
        state.serialize_field("alg", &self.algorithm)?;
        if let Some(type_) = &self.type_ {
            state.serialize_field("typ", type_)?;
        }
        if let Some(key_id) = &self.key_id {
            state.serialize_field("kid", key_id)?;
        }
        state.end()
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
    } else {
        JoseError::InvalidProtectedHeader
    }
}

struct RawProtectedHeader {
    algorithm: String,
    type_: Option<String>,
    key_id: Option<String>,
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
        while let Some(key) = map.next_key::<String>()? {
            let target = match key.as_str() {
                "alg" => &mut algorithm,
                "typ" => &mut type_,
                "kid" => &mut key_id,
                _ => return Err(M::Error::custom(UNKNOWN_MARKER)),
            };
            if target.is_some() {
                return Err(M::Error::custom(DUPLICATE_MARKER));
            }
            let value = map.next_value::<serde_json::Value>()?;
            let value = value
                .as_str()
                .ok_or_else(|| M::Error::custom(INVALID_VALUE_MARKER))?;
            *target = Some(value.to_owned());
        }
        let algorithm = algorithm.ok_or_else(|| M::Error::custom(MISSING_ALGORITHM_MARKER))?;
        Ok(RawProtectedHeader {
            algorithm,
            type_,
            key_id,
        })
    }
}
