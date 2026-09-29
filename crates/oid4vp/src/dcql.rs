//! Bounded DCQL facade over the private selection engine.

use std::{collections::BTreeSet, fmt};

use serde_json::{Map, Value};
use siros_dcql::{
    Candidate as EngineCandidate, Credential as EngineCredential, DcqlQuery as EngineQuery,
    Dropped as EngineDropped, ExactFormat, PathComponent as EnginePath,
    PathError as EnginePathError, SelectedClaim as EngineSelectedClaim, execute,
};
use zeroize::Zeroizing;

use crate::{DcqlLimits, Oid4vpError, VerifiedRequestObject};

/// One SDK-owned DCQL claims-path component.
#[derive(Clone, PartialEq, Eq)]
pub enum DcqlPathComponent {
    /// Select one object member.
    Key(Zeroizing<String>),
    /// Select one array element.
    Index(u64),
    /// Select every array element.
    All,
}

impl DcqlPathComponent {
    /// Explicitly reveal a key component.
    pub fn expose_sensitive_key(&self) -> Option<&str> {
        match self {
            Self::Key(key) => Some(key),
            Self::Index(_) | Self::All => None,
        }
    }

    /// Return an array index component.
    pub const fn index(&self) -> Option<u64> {
        match self {
            Self::Index(index) => Some(*index),
            Self::Key(_) | Self::All => None,
        }
    }

    /// Whether this component selects every array element.
    pub const fn is_all(&self) -> bool {
        matches!(self, Self::All)
    }
}

impl fmt::Debug for DcqlPathComponent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Key(key) => formatter
                .debug_struct("Key")
                .field("byte_len", &key.len())
                .finish(),
            Self::Index(_) => formatter.write_str("Index(<redacted>)"),
            Self::All => formatter.write_str("All"),
        }
    }
}

/// Caller-owned claim-resolution outcomes understood by DCQL selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DcqlClaimError {
    /// The credential does not contain the selected claim.
    Missing,
    /// A path component was applied to the wrong value type.
    TypeMismatch,
    /// The path is not valid for the credential format.
    Malformed,
}

/// Minimal credential capability consumed by the DCQL engine.
pub trait DcqlCredential {
    /// Explicitly reveal the stable inventory identifier used in results.
    fn expose_sensitive_id(&self) -> &str;

    /// Return the exact credential format identifier.
    fn format(&self) -> &str;

    /// Whether this credential has cryptographic holder binding.
    fn has_cryptographic_holder_binding(&self) -> bool;

    /// Resolve a bounded claims path and return matching JSON values.
    ///
    /// # Errors
    ///
    /// Returns a static path category. Implementations must not put secrets or
    /// verifier-controlled content in diagnostics.
    fn resolve_claim(&self, path: &[DcqlPathComponent]) -> Result<Vec<Value>, DcqlClaimError>;
}

/// A signature-proven and structurally validated DCQL query.
///
/// This type does not prove full Authorization Request validity, verifier
/// trust, consent, or credential authenticity.
pub struct ValidatedDcqlQuery {
    engine: EngineQuery,
    limits: DcqlLimits,
    credential_query_count: usize,
    claim_query_count: usize,
}

impl VerifiedRequestObject {
    /// Consume signature evidence into a bounded, structurally valid DCQL query.
    ///
    /// # Errors
    ///
    /// Returns a static OID4VP category when the query is absent, ambiguous,
    /// unsupported, malformed, or outside the supplied resource policy.
    pub fn into_dcql_query(self, limits: DcqlLimits) -> Result<ValidatedDcqlQuery, Oid4vpError> {
        ValidatedDcqlQuery::parse(self.into_sensitive_payload(), limits)
    }
}

impl ValidatedDcqlQuery {
    fn parse(payload: Zeroizing<Vec<u8>>, limits: DcqlLimits) -> Result<Self, Oid4vpError> {
        let Value::Object(request) = serde_json::from_slice(payload.as_slice())
            .map_err(|_| Oid4vpError::InvalidDcqlQuery)?
        else {
            return Err(Oid4vpError::InvalidDcqlQuery);
        };
        Self::from_request_map(request, limits)
    }

    pub(crate) fn from_request_map(
        mut request: Map<String, Value>,
        limits: DcqlLimits,
    ) -> Result<Self, Oid4vpError> {
        if request.contains_key("scope") {
            return Err(Oid4vpError::UnsupportedDcqlScope);
        }
        let query = request
            .remove("dcql_query")
            .ok_or(Oid4vpError::MissingDcqlQuery)?;
        if !query.is_object() {
            return Err(Oid4vpError::InvalidDcqlQuery);
        }

        let counts = validate_query(&query, limits)?;
        let encoded =
            Zeroizing::new(serde_json::to_vec(&query).map_err(|_| Oid4vpError::InvalidDcqlQuery)?);
        if encoded.len() > limits.max_query_bytes() {
            return Err(Oid4vpError::DcqlQueryTooLarge);
        }
        let text = std::str::from_utf8(&encoded).map_err(|_| Oid4vpError::InvalidDcqlQuery)?;
        let engine = EngineQuery::from_json(text).map_err(|_| Oid4vpError::InvalidDcqlQuery)?;

        Ok(Self {
            engine,
            limits,
            credential_query_count: counts.credential_queries,
            claim_query_count: counts.claim_queries,
        })
    }

    /// Number of credential queries retained by the validated request.
    pub const fn credential_query_count(&self) -> usize {
        self.credential_query_count
    }

    /// Number of claim queries retained across all credential queries.
    pub const fn claim_query_count(&self) -> usize {
        self.claim_query_count
    }

    /// Evaluate bounded credential descriptors through the private engine.
    ///
    /// # Errors
    ///
    /// Returns a static category if descriptor or checked work bounds fail.
    pub fn evaluate<C: DcqlCredential>(
        &self,
        credentials: &[C],
    ) -> Result<DcqlEvaluation, Oid4vpError> {
        self.validate_credentials(credentials)?;
        let adapters: Vec<_> = credentials.iter().map(CredentialAdapter).collect();
        let result = execute(&self.engine, &adapters, &ExactFormat);
        let combinations = result.combinations(self.limits.max_combinations());

        Ok(DcqlEvaluation {
            satisfiable: result.satisfiable,
            matches: result
                .matches
                .iter()
                .map(|query| DcqlQueryMatch {
                    query_id: Zeroizing::new(query.query_id.clone()),
                    multiple: query.multiple,
                    candidates: query.candidates.iter().map(map_candidate).collect(),
                })
                .collect(),
            combinations: combinations
                .combinations
                .iter()
                .map(|combination| DcqlCombination {
                    members: combination
                        .members
                        .iter()
                        .map(|(query_id, candidate)| DcqlCombinationMember {
                            query_id: Zeroizing::new(query_id.clone()),
                            candidate: map_candidate(candidate),
                        })
                        .collect(),
                })
                .collect(),
            dropped: match combinations.dropped {
                EngineDropped::Exact(count) => DcqlDropped::Exact(count),
                EngineDropped::AtLeast(count) => DcqlDropped::AtLeast(count),
            },
        })
    }

    fn validate_credentials<C: DcqlCredential>(
        &self,
        credentials: &[C],
    ) -> Result<(), Oid4vpError> {
        if credentials.len() > self.limits.max_credentials() {
            return Err(Oid4vpError::DcqlWorkLimitExceeded);
        }
        let mut ids = BTreeSet::new();
        for credential in credentials {
            let id = credential.expose_sensitive_id();
            let format = credential.format();
            if id.is_empty()
                || id.len() > self.limits.max_string_bytes()
                || format.is_empty()
                || format.len() > self.limits.max_string_bytes()
                || !ids.insert(id)
            {
                return Err(Oid4vpError::InvalidDcqlCredential);
            }
        }
        let per_credential = self
            .credential_query_count
            .checked_add(self.claim_query_count)
            .ok_or(Oid4vpError::DcqlWorkLimitExceeded)?;
        let work = per_credential
            .checked_mul(credentials.len())
            .ok_or(Oid4vpError::DcqlWorkLimitExceeded)?;
        if work > self.limits.max_evaluation_work() {
            return Err(Oid4vpError::DcqlWorkLimitExceeded);
        }
        Ok(())
    }
}

impl fmt::Debug for ValidatedDcqlQuery {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ValidatedDcqlQuery")
            .field("credential_query_count", &self.credential_query_count)
            .field("claim_query_count", &self.claim_query_count)
            .finish_non_exhaustive()
    }
}

struct CredentialAdapter<'a, C>(&'a C);

impl<C: DcqlCredential> EngineCredential for CredentialAdapter<'_, C> {
    fn id(&self) -> &str {
        self.0.expose_sensitive_id()
    }

    fn format(&self) -> &str {
        self.0.format()
    }

    fn claim(&self, path: &[EnginePath]) -> Result<Vec<Value>, EnginePathError> {
        let owned: Vec<_> = path.iter().map(map_path).collect();
        self.0.resolve_claim(&owned).map_err(|error| match error {
            DcqlClaimError::Missing => EnginePathError::Empty,
            DcqlClaimError::TypeMismatch => EnginePathError::TypeMismatch { at: 0 },
            DcqlClaimError::Malformed => EnginePathError::Malformed,
        })
    }

    fn has_cryptographic_holder_binding(&self) -> bool {
        self.0.has_cryptographic_holder_binding()
    }
}

fn map_path(component: &EnginePath) -> DcqlPathComponent {
    match component {
        EnginePath::Key(key) => DcqlPathComponent::Key(Zeroizing::new(key.clone())),
        EnginePath::Index(index) => DcqlPathComponent::Index(*index),
        EnginePath::Null => DcqlPathComponent::All,
    }
}

fn map_candidate(candidate: &EngineCandidate) -> DcqlCandidate {
    DcqlCandidate {
        credential_id: Zeroizing::new(candidate.credential_id.clone()),
        claims: candidate.claims.iter().map(map_selected_claim).collect(),
    }
}

fn map_selected_claim(claim: &EngineSelectedClaim) -> DcqlSelectedClaim {
    DcqlSelectedClaim {
        claim_id: claim.claim_id.clone().map(Zeroizing::new),
        path: claim.path.iter().map(map_path).collect(),
    }
}

#[derive(Clone, Copy)]
struct QueryCounts {
    credential_queries: usize,
    claim_queries: usize,
}

fn validate_query(query: &Value, limits: DcqlLimits) -> Result<QueryCounts, Oid4vpError> {
    validate_value_bounds(query, limits)?;
    let object = query.as_object().ok_or(Oid4vpError::InvalidDcqlQuery)?;
    let credentials = required_nonempty_array(object, "credentials", limits)?;
    let mut credential_ids = BTreeSet::new();
    let mut claim_queries = 0_usize;

    for credential in credentials {
        let (id, claim_count) = validate_credential_query(credential, limits)?;
        if !credential_ids.insert(id.to_owned()) {
            return Err(Oid4vpError::InvalidDcqlQuery);
        }
        claim_queries = claim_queries
            .checked_add(claim_count)
            .ok_or(Oid4vpError::DcqlQueryTooLarge)?;
    }
    validate_credential_sets(object, &credential_ids, limits)?;

    Ok(QueryCounts {
        credential_queries: credentials.len(),
        claim_queries,
    })
}

fn validate_credential_query(
    value: &Value,
    limits: DcqlLimits,
) -> Result<(&str, usize), Oid4vpError> {
    let credential = value.as_object().ok_or(Oid4vpError::InvalidDcqlQuery)?;
    let id = required_identifier(credential, "id", limits)?;
    required_string(credential, "format", limits)?;
    validate_empty_metadata(credential)?;

    let claim_ids = validate_claims(credential, limits)?;
    let claim_count = credential
        .get("claims")
        .and_then(Value::as_array)
        .map_or(0, Vec::len);
    validate_claim_sets(credential, &claim_ids, claim_count, limits)?;
    Ok((id, claim_count))
}

fn validate_empty_metadata(credential: &Map<String, Value>) -> Result<(), Oid4vpError> {
    let metadata = credential
        .get("meta")
        .and_then(Value::as_object)
        .ok_or(Oid4vpError::InvalidDcqlQuery)?;
    if !metadata.is_empty() || credential.contains_key("trusted_authorities") {
        Err(Oid4vpError::InvalidDcqlQuery)
    } else {
        Ok(())
    }
}

fn validate_claims(
    credential: &Map<String, Value>,
    limits: DcqlLimits,
) -> Result<BTreeSet<String>, Oid4vpError> {
    let Some(value) = credential.get("claims") else {
        return Ok(BTreeSet::new());
    };
    let claims = nonempty_array(value, limits)?;
    let mut ids = BTreeSet::new();
    let mut paths = BTreeSet::new();
    for claim in claims {
        validate_claim(claim, limits, &mut ids, &mut paths)?;
    }
    Ok(ids)
}

fn validate_claim(
    value: &Value,
    limits: DcqlLimits,
    ids: &mut BTreeSet<String>,
    paths: &mut BTreeSet<String>,
) -> Result<(), Oid4vpError> {
    let claim = value.as_object().ok_or(Oid4vpError::InvalidDcqlQuery)?;
    if let Some(id) = optional_identifier(claim, "id", limits)?
        && !ids.insert(id.to_owned())
    {
        return Err(Oid4vpError::InvalidDcqlQuery);
    }
    let path = required_nonempty_array(claim, "path", limits)?;
    validate_path(path, limits)?;
    let path_key = serde_json::to_string(path).map_err(|_| Oid4vpError::InvalidDcqlQuery)?;
    if !paths.insert(path_key) {
        return Err(Oid4vpError::InvalidDcqlQuery);
    }
    if let Some(values) = claim.get("values") {
        validate_claim_values(values, limits)?;
    }
    Ok(())
}

fn validate_path(path: &[Value], limits: DcqlLimits) -> Result<(), Oid4vpError> {
    if path.len() > limits.max_path_components() {
        return Err(Oid4vpError::DcqlQueryTooLarge);
    }
    for component in path {
        let valid = match component {
            Value::String(key) => !key.is_empty() && key.len() <= limits.max_string_bytes(),
            Value::Number(index) => index.as_u64().is_some(),
            Value::Null => true,
            _ => false,
        };
        if !valid {
            return Err(Oid4vpError::InvalidDcqlQuery);
        }
    }
    Ok(())
}

fn validate_claim_values(value: &Value, limits: DcqlLimits) -> Result<(), Oid4vpError> {
    for value in nonempty_array(value, limits)? {
        let valid = match value {
            Value::String(value) => value.len() <= limits.max_string_bytes(),
            Value::Number(value) => value.as_i64().is_some() || value.as_u64().is_some(),
            Value::Bool(_) => true,
            _ => false,
        };
        if !valid {
            return Err(Oid4vpError::InvalidDcqlQuery);
        }
    }
    Ok(())
}

fn validate_claim_sets(
    credential: &Map<String, Value>,
    claim_ids: &BTreeSet<String>,
    claim_count: usize,
    limits: DcqlLimits,
) -> Result<(), Oid4vpError> {
    let Some(value) = credential.get("claim_sets") else {
        return Ok(());
    };
    if !credential.contains_key("claims") || claim_ids.len() != claim_count {
        return Err(Oid4vpError::InvalidDcqlQuery);
    }
    for option in nonempty_array(value, limits)? {
        let mut option_ids = BTreeSet::new();
        for id in nonempty_array(option, limits)? {
            let id = id.as_str().ok_or(Oid4vpError::InvalidDcqlQuery)?;
            if !claim_ids.contains(id) || !option_ids.insert(id) {
                return Err(Oid4vpError::InvalidDcqlQuery);
            }
        }
    }
    Ok(())
}

fn validate_credential_sets(
    query: &Map<String, Value>,
    credential_ids: &BTreeSet<String>,
    limits: DcqlLimits,
) -> Result<(), Oid4vpError> {
    let Some(value) = query.get("credential_sets") else {
        return Ok(());
    };
    for set in nonempty_array(value, limits)? {
        validate_credential_set(set, credential_ids, limits)?;
    }
    Ok(())
}

fn validate_credential_set(
    value: &Value,
    credential_ids: &BTreeSet<String>,
    limits: DcqlLimits,
) -> Result<(), Oid4vpError> {
    let set = value.as_object().ok_or(Oid4vpError::InvalidDcqlQuery)?;
    if set
        .get("purpose")
        .is_some_and(|value| !valid_purpose(value))
    {
        return Err(Oid4vpError::InvalidDcqlQuery);
    }
    for option in required_nonempty_array(set, "options", limits)? {
        validate_credential_option(option, credential_ids, limits)?;
    }
    Ok(())
}

fn valid_purpose(value: &Value) -> bool {
    match value {
        Value::String(_) | Value::Object(_) => true,
        Value::Number(value) => value.as_i64().is_some() || value.as_u64().is_some(),
        _ => false,
    }
}

fn validate_credential_option(
    value: &Value,
    credential_ids: &BTreeSet<String>,
    limits: DcqlLimits,
) -> Result<(), Oid4vpError> {
    let mut option_ids = BTreeSet::new();
    for id in nonempty_array(value, limits)? {
        let id = id.as_str().ok_or(Oid4vpError::InvalidDcqlQuery)?;
        if !credential_ids.contains(id) || !option_ids.insert(id) {
            return Err(Oid4vpError::InvalidDcqlQuery);
        }
    }
    Ok(())
}

fn validate_value_bounds(value: &Value, limits: DcqlLimits) -> Result<(), Oid4vpError> {
    match value {
        Value::String(value) if value.len() > limits.max_string_bytes() => {
            Err(Oid4vpError::DcqlQueryTooLarge)
        }
        Value::Array(values) => {
            if values.len() > limits.max_collection_items() {
                return Err(Oid4vpError::DcqlQueryTooLarge);
            }
            values
                .iter()
                .try_for_each(|value| validate_value_bounds(value, limits))
        }
        Value::Object(values) => {
            if values.len() > limits.max_collection_items() {
                return Err(Oid4vpError::DcqlQueryTooLarge);
            }
            for (name, value) in values {
                if name.len() > limits.max_string_bytes() {
                    return Err(Oid4vpError::DcqlQueryTooLarge);
                }
                validate_value_bounds(value, limits)?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

fn required_nonempty_array<'a>(
    object: &'a Map<String, Value>,
    name: &str,
    limits: DcqlLimits,
) -> Result<&'a [Value], Oid4vpError> {
    nonempty_array(
        object.get(name).ok_or(Oid4vpError::InvalidDcqlQuery)?,
        limits,
    )
}

fn nonempty_array(value: &Value, limits: DcqlLimits) -> Result<&[Value], Oid4vpError> {
    let array = value.as_array().ok_or(Oid4vpError::InvalidDcqlQuery)?;
    if array.is_empty() {
        return Err(Oid4vpError::InvalidDcqlQuery);
    }
    if array.len() > limits.max_collection_items() {
        return Err(Oid4vpError::DcqlQueryTooLarge);
    }
    Ok(array)
}

fn required_identifier<'a>(
    object: &'a Map<String, Value>,
    name: &str,
    limits: DcqlLimits,
) -> Result<&'a str, Oid4vpError> {
    optional_identifier(object, name, limits)?.ok_or(Oid4vpError::InvalidDcqlQuery)
}

fn optional_identifier<'a>(
    object: &'a Map<String, Value>,
    name: &str,
    limits: DcqlLimits,
) -> Result<Option<&'a str>, Oid4vpError> {
    let Some(value) = object.get(name) else {
        return Ok(None);
    };
    let value = value.as_str().ok_or(Oid4vpError::InvalidDcqlQuery)?;
    if value.is_empty()
        || value.len() > limits.max_string_bytes()
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
    {
        return Err(Oid4vpError::InvalidDcqlQuery);
    }
    Ok(Some(value))
}

fn required_string<'a>(
    object: &'a Map<String, Value>,
    name: &str,
    limits: DcqlLimits,
) -> Result<&'a str, Oid4vpError> {
    let value = object
        .get(name)
        .and_then(Value::as_str)
        .ok_or(Oid4vpError::InvalidDcqlQuery)?;
    if value.is_empty() || value.len() > limits.max_string_bytes() {
        return Err(Oid4vpError::InvalidDcqlQuery);
    }
    Ok(value)
}

/// One selected claim in a DCQL candidate.
pub struct DcqlSelectedClaim {
    claim_id: Option<Zeroizing<String>>,
    path: Vec<DcqlPathComponent>,
}

impl DcqlSelectedClaim {
    /// Explicitly reveal the optional query claim identifier.
    pub fn expose_sensitive_claim_id(&self) -> Option<&str> {
        self.claim_id.as_deref().map(String::as_str)
    }

    /// Borrow the selected SDK-owned path.
    pub fn path(&self) -> &[DcqlPathComponent] {
        &self.path
    }
}

/// One credential candidate satisfying one query.
pub struct DcqlCandidate {
    credential_id: Zeroizing<String>,
    claims: Vec<DcqlSelectedClaim>,
}

impl DcqlCandidate {
    /// Explicitly reveal the inventory credential identifier.
    pub fn expose_sensitive_credential_id(&self) -> &str {
        &self.credential_id
    }

    /// Selected claims required from this credential.
    pub fn claims(&self) -> &[DcqlSelectedClaim] {
        &self.claims
    }
}

/// Candidates for one credential query.
pub struct DcqlQueryMatch {
    query_id: Zeroizing<String>,
    multiple: bool,
    candidates: Vec<DcqlCandidate>,
}

impl DcqlQueryMatch {
    /// Explicitly reveal the verifier query identifier.
    pub fn expose_sensitive_query_id(&self) -> &str {
        &self.query_id
    }

    /// Whether the query permits multiple credentials.
    pub const fn multiple(&self) -> bool {
        self.multiple
    }

    /// Bounded matching credentials in caller inventory order.
    pub fn candidates(&self) -> &[DcqlCandidate] {
        &self.candidates
    }
}

/// One query-to-credential member of a satisfying combination.
pub struct DcqlCombinationMember {
    query_id: Zeroizing<String>,
    candidate: DcqlCandidate,
}

impl DcqlCombinationMember {
    /// Explicitly reveal the verifier query identifier.
    pub fn expose_sensitive_query_id(&self) -> &str {
        &self.query_id
    }

    /// Selected credential and claims.
    pub const fn candidate(&self) -> &DcqlCandidate {
        &self.candidate
    }
}

/// One bounded way to satisfy all required query groups.
pub struct DcqlCombination {
    members: Vec<DcqlCombinationMember>,
}

impl DcqlCombination {
    /// Combination members presented together.
    pub fn members(&self) -> &[DcqlCombinationMember] {
        &self.members
    }
}

/// Count of combinations omitted by the configured cap.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DcqlDropped {
    /// Exact omitted count.
    Exact(usize),
    /// Lower bound when the complete product overflowed `usize`.
    AtLeast(usize),
}

impl DcqlDropped {
    /// Numeric exact count or lower bound.
    pub const fn count(self) -> usize {
        match self {
            Self::Exact(count) | Self::AtLeast(count) => count,
        }
    }

    /// Whether [`Self::count`] is exact.
    pub const fn is_exact(self) -> bool {
        matches!(self, Self::Exact(_))
    }
}

/// Bounded, candidate-free DCQL evaluation outcome.
pub struct DcqlEvaluation {
    satisfiable: bool,
    matches: Vec<DcqlQueryMatch>,
    combinations: Vec<DcqlCombination>,
    dropped: DcqlDropped,
}

impl DcqlEvaluation {
    /// Whether all required query constraints can be satisfied.
    pub const fn is_satisfiable(&self) -> bool {
        self.satisfiable
    }

    /// Per-query candidate outcomes.
    pub fn matches(&self) -> &[DcqlQueryMatch] {
        &self.matches
    }

    /// Capped satisfying combinations.
    pub fn combinations(&self) -> &[DcqlCombination] {
        &self.combinations
    }

    /// Count or lower bound omitted by the combination cap.
    pub const fn dropped(&self) -> DcqlDropped {
        self.dropped
    }
}

impl fmt::Debug for DcqlEvaluation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DcqlEvaluation")
            .field("satisfiable", &self.satisfiable)
            .field("query_count", &self.matches.len())
            .field("combination_count", &self.combinations.len())
            .field("dropped", &self.dropped)
            .finish_non_exhaustive()
    }
}
