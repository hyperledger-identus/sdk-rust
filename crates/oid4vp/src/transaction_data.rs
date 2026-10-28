//! Bounded transaction-data parsing and selected-input linkage.

use std::{collections::BTreeSet, fmt};

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use serde_json::{Map, Value};
use zeroize::Zeroizing;

use crate::{CredentialFormat, CredentialFormatCapability, Oid4vpError, ValidatedDcqlQuery};

/// Resource policy for Final transaction data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransactionDataLimits {
    max_items: usize,
    max_encoded_bytes: usize,
    max_decoded_bytes: usize,
    max_credential_ids: usize,
    max_json_depth: usize,
    max_json_nodes: usize,
    max_object_members: usize,
    max_string_bytes: usize,
}

impl TransactionDataLimits {
    /// Construct a positive transaction-data resource policy.
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        max_items: usize,
        max_encoded_bytes: usize,
        max_decoded_bytes: usize,
        max_credential_ids: usize,
        max_json_depth: usize,
        max_json_nodes: usize,
        max_object_members: usize,
        max_string_bytes: usize,
    ) -> Result<Self, Oid4vpError> {
        if max_items == 0
            || max_encoded_bytes == 0
            || max_decoded_bytes == 0
            || max_credential_ids == 0
            || max_json_depth == 0
            || max_json_nodes == 0
            || max_object_members == 0
            || max_string_bytes == 0
        {
            return Err(Oid4vpError::InvalidLimits);
        }
        Ok(Self {
            max_items,
            max_encoded_bytes,
            max_decoded_bytes,
            max_credential_ids,
            max_json_depth,
            max_json_nodes,
            max_object_members,
            max_string_bytes,
        })
    }

    /// Maximum transaction-data strings in one request.
    pub const fn max_items(self) -> usize {
        self.max_items
    }

    /// Maximum bytes in one exact encoded value.
    pub const fn max_encoded_bytes(self) -> usize {
        self.max_encoded_bytes
    }

    /// Maximum decoded JSON bytes in one value.
    pub const fn max_decoded_bytes(self) -> usize {
        self.max_decoded_bytes
    }

    /// Maximum DCQL credential references in one value.
    pub const fn max_credential_ids(self) -> usize {
        self.max_credential_ids
    }

    /// Maximum decoded JSON nesting depth.
    pub const fn max_json_depth(self) -> usize {
        self.max_json_depth
    }

    /// Maximum decoded JSON nodes.
    pub const fn max_json_nodes(self) -> usize {
        self.max_json_nodes
    }

    /// Maximum members in one decoded JSON object.
    pub const fn max_object_members(self) -> usize {
        self.max_object_members
    }

    /// Maximum bytes in a decoded member name or string value.
    pub const fn max_string_bytes(self) -> usize {
        self.max_string_bytes
    }
}

impl Default for TransactionDataLimits {
    fn default() -> Self {
        Self {
            max_items: 16,
            max_encoded_bytes: 16_384,
            max_decoded_bytes: 12_288,
            max_credential_ids: 16,
            max_json_depth: 16,
            max_json_nodes: 1_024,
            max_object_members: 64,
            max_string_bytes: 4_096,
        }
    }
}

/// Static outcome returned by a transaction-type capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionDataCapabilityError {
    /// The object is invalid for the registered transaction type.
    Invalid,
    /// The object requests a valid but unsupported profile feature.
    Unsupported,
}

/// Caller-owned semantics for one transaction-data type.
pub trait TransactionDataCapability {
    /// Exact transaction-data type implemented by this capability.
    fn transaction_type(&self) -> &str;

    /// Validate an already resource-bounded complete transaction object.
    fn validate(&self, value: &Value) -> Result<(), TransactionDataCapabilityError>;
}

/// One caller-selected credential input used for presentation construction.
pub struct SelectedCredentialInput {
    query_id: Zeroizing<String>,
    format: CredentialFormat,
}

impl SelectedCredentialInput {
    /// Construct one selected input. The request transition applies its own
    /// exact query-ID policy before returning binding evidence.
    pub fn new(query_id: impl Into<String>, format: CredentialFormat) -> Self {
        Self {
            query_id: Zeroizing::new(query_id.into()),
            format,
        }
    }
}

impl fmt::Debug for SelectedCredentialInput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SelectedCredentialInput")
            .field("query_id_len", &self.query_id.len())
            .field("format", &self.format)
            .finish()
    }
}

/// Exact transaction material for one selected presentation input.
pub struct TransactionBinding {
    encoded: Zeroizing<String>,
    transaction_type: Zeroizing<String>,
    query_id: Zeroizing<String>,
}

impl TransactionBinding {
    /// Explicitly reveal the exact encoded transaction value required by the
    /// owning credential-format proof engine.
    pub fn expose_sensitive_encoded(&self) -> &str {
        &self.encoded
    }

    /// Exact transaction-data type.
    pub fn transaction_type(&self) -> &str {
        &self.transaction_type
    }

    /// Explicitly reveal the selected DCQL query identifier.
    pub fn expose_sensitive_query_id(&self) -> &str {
        &self.query_id
    }
}

impl fmt::Debug for TransactionBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TransactionBinding")
            .field("encoded_len", &self.encoded.len())
            .field("transaction_type_len", &self.transaction_type.len())
            .field("query_id_len", &self.query_id.len())
            .finish()
    }
}

pub(crate) struct ValidatedTransactionData {
    entries: Vec<TransactionEntry>,
}

struct TransactionEntry {
    encoded: Zeroizing<String>,
    transaction_type: Zeroizing<String>,
    compatible_queries: Vec<(Zeroizing<String>, CredentialFormat)>,
}

impl ValidatedTransactionData {
    pub(crate) fn parse(
        request: &Map<String, Value>,
        query: &ValidatedDcqlQuery,
        limits: TransactionDataLimits,
        format_capabilities: &[&dyn CredentialFormatCapability],
        transaction_capabilities: &[&dyn TransactionDataCapability],
    ) -> Result<Self, Oid4vpError> {
        if transaction_capabilities.len() > limits.max_items {
            return Err(Oid4vpError::InvalidTransactionData);
        }
        let Some(value) = request.get("transaction_data") else {
            return Ok(Self {
                entries: Vec::new(),
            });
        };
        let values = value
            .as_array()
            .ok_or(Oid4vpError::InvalidTransactionData)?;
        if values.is_empty() || values.len() > limits.max_items {
            return Err(Oid4vpError::InvalidTransactionData);
        }
        let mut entries = Vec::with_capacity(values.len());
        for value in values {
            entries.push(parse_entry(
                value,
                query,
                limits,
                format_capabilities,
                transaction_capabilities,
            )?);
        }
        Ok(Self { entries })
    }

    pub(crate) fn bindings(
        &self,
        selections: &[SelectedCredentialInput],
    ) -> Result<Vec<TransactionBinding>, Oid4vpError> {
        let mut ids = BTreeSet::new();
        for selection in selections {
            if selection.query_id.is_empty() || !ids.insert(selection.query_id.as_str()) {
                return Err(Oid4vpError::InvalidTransactionData);
            }
        }
        let mut bindings = Vec::new();
        for entry in &self.entries {
            let start = bindings.len();
            for selected in selections.iter().filter(|selection| {
                entry.compatible_queries.iter().any(|(id, format)| {
                    id.as_str() == selection.query_id.as_str() && format == &selection.format
                })
            }) {
                bindings.push(TransactionBinding {
                    encoded: entry.encoded.clone(),
                    transaction_type: entry.transaction_type.clone(),
                    query_id: selected.query_id.clone(),
                });
            }
            if bindings.len() == start {
                return Err(Oid4vpError::InvalidTransactionData);
            }
        }
        Ok(bindings)
    }

    pub(crate) fn len(&self) -> usize {
        self.entries.len()
    }

    pub(crate) fn referenced_query_ids(&self) -> Vec<Zeroizing<String>> {
        self.entries
            .iter()
            .flat_map(|entry| entry.compatible_queries.iter().map(|(id, _)| id.as_str()))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .map(|id| Zeroizing::new(id.to_owned()))
            .collect()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

fn parse_entry(
    value: &Value,
    query: &ValidatedDcqlQuery,
    limits: TransactionDataLimits,
    format_capabilities: &[&dyn CredentialFormatCapability],
    transaction_capabilities: &[&dyn TransactionDataCapability],
) -> Result<TransactionEntry, Oid4vpError> {
    let encoded = value.as_str().ok_or(Oid4vpError::InvalidTransactionData)?;
    if encoded.is_empty()
        || encoded.len() > limits.max_encoded_bytes
        || encoded.contains('=')
        || !encoded
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    {
        return Err(Oid4vpError::InvalidTransactionData);
    }
    let decoded = Zeroizing::new(
        URL_SAFE_NO_PAD
            .decode(encoded)
            .map_err(|_| Oid4vpError::InvalidTransactionData)?,
    );
    if decoded.len() > limits.max_decoded_bytes {
        return Err(Oid4vpError::InvalidTransactionData);
    }
    if URL_SAFE_NO_PAD.encode(decoded.as_slice()) != encoded {
        return Err(Oid4vpError::InvalidTransactionData);
    }
    let parsed: Value =
        serde_json::from_slice(&decoded).map_err(|_| Oid4vpError::InvalidTransactionData)?;
    validate_json_bounds(&parsed, limits, 0, &mut 0)?;
    let object = parsed
        .as_object()
        .ok_or(Oid4vpError::InvalidTransactionData)?;
    let transaction_type = required_text(object, "type", limits)?;
    let credential_ids = object
        .get("credential_ids")
        .and_then(Value::as_array)
        .ok_or(Oid4vpError::InvalidTransactionData)?;
    if credential_ids.is_empty() || credential_ids.len() > limits.max_credential_ids {
        return Err(Oid4vpError::InvalidTransactionData);
    }

    let mut ids = BTreeSet::new();
    for id in credential_ids {
        let id = id.as_str().ok_or(Oid4vpError::InvalidTransactionData)?;
        if id.is_empty()
            || id.len() > limits.max_string_bytes
            || !ids.insert(id)
            || !query.contains_query_id(id)
        {
            return Err(Oid4vpError::InvalidTransactionData);
        }
    }

    let mut type_capabilities = transaction_capabilities
        .iter()
        .filter(|capability| capability.transaction_type() == transaction_type);
    let type_capability = type_capabilities
        .next()
        .ok_or(Oid4vpError::InvalidTransactionData)?;
    if type_capabilities.next().is_some() {
        return Err(Oid4vpError::InvalidTransactionData);
    }
    type_capability
        .validate(&parsed)
        .map_err(|_| Oid4vpError::InvalidTransactionData)?;

    let mut compatible_queries = Vec::new();
    for id in ids {
        let format = query
            .credential_format(id)
            .ok_or(Oid4vpError::InvalidTransactionData)?;
        let matches: Vec<_> = format_capabilities
            .iter()
            .filter(|capability| {
                capability.format().expose_sensitive_identifier()
                    == format.expose_sensitive_identifier()
            })
            .collect();
        if matches.len() != 1 || !matches[0].supports_transaction_type(transaction_type) {
            continue;
        }
        compatible_queries.push((Zeroizing::new(id.to_owned()), format.clone()));
    }
    if compatible_queries.is_empty() {
        return Err(Oid4vpError::InvalidTransactionData);
    }

    Ok(TransactionEntry {
        encoded: Zeroizing::new(encoded.to_owned()),
        transaction_type: Zeroizing::new(transaction_type.to_owned()),
        compatible_queries,
    })
}

fn required_text<'a>(
    object: &'a Map<String, Value>,
    name: &str,
    limits: TransactionDataLimits,
) -> Result<&'a str, Oid4vpError> {
    let value = object
        .get(name)
        .and_then(Value::as_str)
        .ok_or(Oid4vpError::InvalidTransactionData)?;
    if value.is_empty() || value.len() > limits.max_string_bytes {
        return Err(Oid4vpError::InvalidTransactionData);
    }
    Ok(value)
}

fn validate_json_bounds(
    value: &Value,
    limits: TransactionDataLimits,
    depth: usize,
    nodes: &mut usize,
) -> Result<(), Oid4vpError> {
    *nodes = nodes
        .checked_add(1)
        .ok_or(Oid4vpError::InvalidTransactionData)?;
    if *nodes > limits.max_json_nodes || depth > limits.max_json_depth {
        return Err(Oid4vpError::InvalidTransactionData);
    }
    match value {
        Value::String(value) if value.len() > limits.max_string_bytes => {
            Err(Oid4vpError::InvalidTransactionData)
        }
        Value::Array(values) => values
            .iter()
            .try_for_each(|value| validate_json_bounds(value, limits, depth + 1, nodes)),
        Value::Object(values) => {
            if values.len() > limits.max_object_members {
                return Err(Oid4vpError::InvalidTransactionData);
            }
            for (name, value) in values {
                if name.len() > limits.max_string_bytes {
                    return Err(Oid4vpError::InvalidTransactionData);
                }
                validate_json_bounds(value, limits, depth + 1, nodes)?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}
