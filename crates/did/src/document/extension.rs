use std::{collections::BTreeMap, fmt};

use serde::{Deserialize, Deserializer, Serialize, de};
use serde_json::Value;

use crate::{
    Error, Uri,
    error::DocumentError,
    json_cleanup::{RejectionGuard, drop_json_values_iteratively},
};

use super::{
    MAX_DOCUMENT_ITEMS, MAX_EXTENSION_DEPTH, MAX_EXTENSION_NODES, MAX_EXTENSION_PROPERTIES,
    MAX_EXTENSION_STRING_BYTES, MAX_PROPERTY_NAME_BYTES, invalid,
};

/// A validated inline JSON-LD context object.
///
/// Construction enforces the DID JSON depth, node, property, collection,
/// property-name, and string limits before the recursive value can inhabit a
/// public DID domain object. Diagnostics intentionally reveal only shape.
#[derive(Clone, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct ContextObject(BTreeMap<String, Value>);

impl ContextObject {
    /// Validate an owned inline JSON-LD context map.
    pub fn new(values: BTreeMap<String, Value>) -> Result<Self, Error> {
        let object = RejectionGuard::new(Self(values), drop_context_object_json);
        validate_json_map(&object.owner().0, &[], &mut JsonBudget::default())?;
        Ok(object.into_owner())
    }

    /// Borrow the validated context map.
    #[must_use]
    pub const fn as_map(&self) -> &BTreeMap<String, Value> {
        &self.0
    }

    /// Consume the context object and return its validated map.
    #[must_use]
    pub fn into_map(self) -> BTreeMap<String, Value> {
        self.0
    }
}

impl TryFrom<BTreeMap<String, Value>> for ContextObject {
    type Error = Error;

    fn try_from(values: BTreeMap<String, Value>) -> Result<Self, Self::Error> {
        Self::new(values)
    }
}

impl fmt::Debug for ContextObject {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ContextObject")
            .field("property_count", &self.0.len())
            .finish_non_exhaustive()
    }
}

impl<'de> Deserialize<'de> for ContextObject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let values = BTreeMap::<String, Value>::deserialize(deserializer)?;
        Self::new(values).map_err(de::Error::custom)
    }
}

fn drop_context_object_json(object: ContextObject) {
    drop_json_values_iteratively(object.0.into_values());
}

/// One JSON-LD context entry retained by the representation-neutral model.
///
/// Raw maps cannot directly inhabit this enum. Construct a validated
/// [`ContextObject`] first:
///
/// ```compile_fail
/// use std::collections::BTreeMap;
/// use identus_did::ContextEntry;
///
/// let _ = ContextEntry::Object(BTreeMap::new());
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContextEntry {
    /// A context URI.
    Uri(Uri),
    /// An inline JSON-LD context definition.
    Object(ContextObject),
}
#[derive(Default)]
pub(crate) struct JsonBudget {
    nodes: usize,
}

impl JsonBudget {
    fn visit(&mut self) -> Result<(), Error> {
        self.nodes += 1;
        if self.nodes > MAX_EXTENSION_NODES {
            return Err(invalid(DocumentError::ExtensionTooLarge));
        }
        Ok(())
    }
}

pub(crate) fn validate_json_map(
    map: &BTreeMap<String, Value>,
    reserved: &[&str],
    budget: &mut JsonBudget,
) -> Result<(), Error> {
    budget.visit()?;
    if map.len() > MAX_EXTENSION_PROPERTIES {
        return Err(invalid(DocumentError::TooManyProperties));
    }
    for (key, value) in map {
        if key.is_empty() || key.len() > MAX_PROPERTY_NAME_BYTES {
            return Err(invalid(DocumentError::InvalidPropertyName));
        }
        if reserved.contains(&key.as_str()) {
            return Err(invalid(DocumentError::ReservedProperty));
        }
        validate_json_value(value, 1, budget)?;
    }
    Ok(())
}

fn validate_json_object(
    map: &serde_json::Map<String, Value>,
    depth: usize,
    budget: &mut JsonBudget,
) -> Result<(), Error> {
    if map.len() > MAX_EXTENSION_PROPERTIES {
        return Err(invalid(DocumentError::TooManyProperties));
    }
    for (key, value) in map {
        if key.is_empty() || key.len() > MAX_PROPERTY_NAME_BYTES {
            return Err(invalid(DocumentError::InvalidPropertyName));
        }
        validate_json_value(value, depth, budget)?;
    }
    Ok(())
}

pub(crate) fn validate_json_value(
    value: &Value,
    depth: usize,
    budget: &mut JsonBudget,
) -> Result<(), Error> {
    if depth > MAX_EXTENSION_DEPTH {
        return Err(invalid(DocumentError::ExtensionTooDeep));
    }
    budget.visit()?;
    match value {
        Value::String(value) if value.len() > MAX_EXTENSION_STRING_BYTES => {
            Err(invalid(DocumentError::InvalidString))
        }
        Value::Array(values) => {
            if values.len() > MAX_DOCUMENT_ITEMS {
                return Err(invalid(DocumentError::TooManyItems));
            }
            for value in values {
                validate_json_value(value, depth + 1, budget)?;
            }
            Ok(())
        }
        Value::Object(map) => validate_json_object(map, depth + 1, budget),
        _ => Ok(()),
    }
}
