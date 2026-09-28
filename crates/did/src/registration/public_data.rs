use std::{collections::BTreeMap, fmt};

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use serde_json::Value;

use crate::{
    DidDocument, Error,
    error::RegistrationError,
    json_cleanup::{RejectionGuard, drop_json_values_iteratively},
};

use super::{MAX_DID_REGISTRATION_BYTES, MAX_REGISTRATION_ITEMS};

const MAX_PROPERTIES: usize = 64;
const MAX_PROPERTY_NAME_BYTES: usize = 256;
const MAX_DEPTH: usize = 32;
const MAX_NODES: usize = 4_096;
const MAX_STRING_BYTES: usize = 64 * 1_024;
const PRIVATE_MEMBER_NAMES: &[&str] = &[
    "secret",
    "secrets",
    "privatekey",
    "privatekeyjwk",
    "privatekeymultibase",
    "seed",
    "password",
    "passphrase",
    "mnemonic",
    "decryptedpayload",
    "plaintext",
];
const PRIVATE_JWK_MEMBERS: &[&str] = &["d", "p", "q", "dp", "dq", "qi", "oth", "k"];
const RESERVED_MEMBER_NAMES: &[&str] = &[
    "jobid",
    "didstate",
    "didregistrationmetadata",
    "diddocumentmetadata",
];

/// Bounded JSON object containing public protocol data only.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct RegistrationPublicData(BTreeMap<String, Value>);

impl RegistrationPublicData {
    /// Validate a native public-data map.
    pub fn new(values: BTreeMap<String, Value>) -> Result<Self, Error> {
        let data = RejectionGuard::new(Self(values), drop_registration_public_data_json);
        let mut budget = JsonBudget::default();
        validate_map(&data.owner().0, 1, &mut budget)?;
        Ok(data.into_owner())
    }

    /// Return an empty public-data object.
    #[must_use]
    pub const fn empty() -> Self {
        Self(BTreeMap::new())
    }

    /// Parse a bounded public-data JSON object.
    pub fn from_json_slice(input: &[u8]) -> Result<Self, Error> {
        if input.len() > MAX_DID_REGISTRATION_BYTES {
            return Err(invalid(RegistrationError::TooLarge));
        }
        let values =
            serde_json::from_slice(input).map_err(|_| invalid(RegistrationError::MalformedJson))?;
        Self::new(values)
    }

    /// Parse a bounded public-data JSON object string.
    pub fn from_json_str(input: &str) -> Result<Self, Error> {
        Self::from_json_slice(input.as_bytes())
    }

    /// Borrow the validated object.
    #[must_use]
    pub const fn as_map(&self) -> &BTreeMap<String, Value> {
        &self.0
    }

    /// Consume the value and return the validated object.
    #[must_use]
    pub fn into_map(self) -> BTreeMap<String, Value> {
        self.0
    }
}

fn drop_registration_public_data_json(data: RegistrationPublicData) {
    let RegistrationPublicData(values) = data;
    drop_json_values_iteratively(values.into_values());
}

impl fmt::Debug for RegistrationPublicData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RegistrationPublicData")
            .field("property_count", &self.0.len())
            .finish()
    }
}

impl Serialize for RegistrationPublicData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RegistrationPublicData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let values = BTreeMap::<String, Value>::deserialize(deserializer)?;
        Self::new(values).map_err(de::Error::custom)
    }
}
pub(super) fn validate_public_document(document: &DidDocument) -> Result<(), Error> {
    let value =
        serde_json::to_value(document).map_err(|_| invalid(RegistrationError::MalformedJson))?;
    let Value::Object(map) = value else {
        return Err(invalid(RegistrationError::MalformedJson));
    };
    validate_object(&map, 1, &mut JsonBudget::default())
}
#[derive(Default)]
struct JsonBudget {
    nodes: usize,
}

impl JsonBudget {
    fn visit(&mut self) -> Result<(), Error> {
        self.nodes += 1;
        if self.nodes > MAX_NODES {
            return Err(invalid(RegistrationError::TooManyNodes));
        }
        Ok(())
    }
}

fn validate_map(
    map: &BTreeMap<String, Value>,
    depth: usize,
    budget: &mut JsonBudget,
) -> Result<(), Error> {
    budget.visit()?;
    if map.len() > MAX_PROPERTIES {
        return Err(invalid(RegistrationError::TooManyProperties));
    }
    validate_object_members(
        map.iter().map(|(key, value)| (key.as_str(), value)),
        depth,
        budget,
    )
}

fn validate_object(
    map: &serde_json::Map<String, Value>,
    depth: usize,
    budget: &mut JsonBudget,
) -> Result<(), Error> {
    budget.visit()?;
    if map.len() > MAX_PROPERTIES {
        return Err(invalid(RegistrationError::TooManyProperties));
    }
    validate_object_members(
        map.iter().map(|(key, value)| (key.as_str(), value)),
        depth,
        budget,
    )
}

fn validate_object_members<'a>(
    members: impl Iterator<Item = (&'a str, &'a Value)> + Clone,
    depth: usize,
    budget: &mut JsonBudget,
) -> Result<(), Error> {
    let is_jwk = members.clone().any(|(key, _)| key == "kty");
    for (key, value) in members {
        if key.is_empty()
            || key.len() > MAX_PROPERTY_NAME_BYTES
            || key.trim() != key
            || key.chars().any(char::is_control)
        {
            return Err(invalid(RegistrationError::InvalidPropertyName));
        }
        let normalized: String = key
            .chars()
            .filter(|character| !matches!(character, '-' | '_' | '.'))
            .flat_map(char::to_lowercase)
            .collect();
        if PRIVATE_MEMBER_NAMES.contains(&normalized.as_str())
            || RESERVED_MEMBER_NAMES.contains(&normalized.as_str())
            || normalized.contains("privatekey")
            || matches!(normalized.as_str(), "secretkey" | "recoveryphrase")
            || (is_jwk && PRIVATE_JWK_MEMBERS.contains(&key))
        {
            return Err(invalid(
                if RESERVED_MEMBER_NAMES.contains(&normalized.as_str()) {
                    RegistrationError::ReservedProperty
                } else {
                    RegistrationError::PrivateMaterial
                },
            ));
        }
        validate_value(value, depth + 1, budget)?;
    }
    Ok(())
}

fn validate_value(value: &Value, depth: usize, budget: &mut JsonBudget) -> Result<(), Error> {
    if depth > MAX_DEPTH {
        return Err(invalid(RegistrationError::TooDeep));
    }
    match value {
        Value::String(value)
            if value.len() > MAX_STRING_BYTES || value.chars().any(char::is_control) =>
        {
            Err(invalid(RegistrationError::InvalidString))
        }
        Value::Array(values) => {
            budget.visit()?;
            if values.len() > MAX_REGISTRATION_ITEMS {
                return Err(invalid(RegistrationError::TooManyItems));
            }
            for value in values {
                validate_value(value, depth + 1, budget)?;
            }
            Ok(())
        }
        Value::Object(map) => validate_object(map, depth, budget),
        _ => budget.visit(),
    }
}

const fn invalid(reason: RegistrationError) -> Error {
    Error::InvalidRegistration(reason)
}
