//! Bounded construction of plain OID4VP `direct_post` success responses.

use std::{collections::BTreeMap, fmt};

use serde::de::{DeserializeSeed, Error as _, MapAccess, SeqAccess, Visitor};
use serde_json::{Map, Value};
use zeroize::Zeroizing;

use crate::{
    DirectPostResponseLimits, Oid4vpError, ValidatedAuthorizationRequest,
    dcql::{DcqlResponseQueryRule, DcqlResponseRules},
};

/// HTTP method required for a plain direct-post response.
pub const DIRECT_POST_HTTP_METHOD: &str = "POST";

/// Media type required for a plain direct-post response body.
pub const DIRECT_POST_MEDIA_TYPE: &str = "application/x-www-form-urlencoded";

/// One bounded Final presentation value.
pub struct DirectPostPresentation {
    encoded: Zeroizing<String>,
    kind: PresentationKind,
    json_depth: usize,
    json_nodes: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PresentationKind {
    String,
    Object,
}

impl DirectPostPresentation {
    /// Construct a bounded JSON string presentation.
    pub fn string(value: &str, limits: DirectPostResponseLimits) -> Result<Self, Oid4vpError> {
        if value.is_empty() {
            return Err(Oid4vpError::InvalidPresentation);
        }
        if value.len() > limits.max_presentation_bytes() {
            return Err(Oid4vpError::PresentationTooLarge);
        }
        let encoded = serde_json::to_string(value).map_err(|_| Oid4vpError::InvalidPresentation)?;
        if encoded.len() > limits.max_presentation_bytes() {
            return Err(Oid4vpError::PresentationTooLarge);
        }
        Ok(Self {
            encoded: Zeroizing::new(encoded),
            kind: PresentationKind::String,
            json_depth: 0,
            json_nodes: 1,
        })
    }

    /// Parse and canonicalize one bounded JSON object presentation.
    pub fn object_json(
        input: &[u8],
        limits: DirectPostResponseLimits,
    ) -> Result<Self, Oid4vpError> {
        if input.len() > limits.max_presentation_bytes() {
            return Err(Oid4vpError::PresentationTooLarge);
        }
        let (value, json_depth, json_nodes) = parse_bounded_json(input, limits)?;
        if !value.is_object() {
            return Err(Oid4vpError::InvalidPresentation);
        }
        let encoded =
            serde_json::to_string(&value).map_err(|_| Oid4vpError::InvalidPresentation)?;
        if encoded.len() > limits.max_presentation_bytes() {
            return Err(Oid4vpError::PresentationTooLarge);
        }
        Ok(Self {
            encoded: Zeroizing::new(encoded),
            kind: PresentationKind::Object,
            json_depth,
            json_nodes,
        })
    }

    fn encoded(&self) -> &str {
        &self.encoded
    }
}

impl fmt::Debug for DirectPostPresentation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DirectPostPresentation")
            .field("kind", &self.kind)
            .field("encoded_bytes", &self.encoded.len())
            .finish()
    }
}

/// Presentations selected for one validated DCQL credential query.
pub struct DirectPostQueryResult {
    query_id: Zeroizing<String>,
    presentations: Vec<DirectPostPresentation>,
}

impl DirectPostQueryResult {
    /// Construct a bounded, non-empty query result.
    pub fn new(
        query_id: &str,
        presentations: Vec<DirectPostPresentation>,
        limits: DirectPostResponseLimits,
    ) -> Result<Self, Oid4vpError> {
        if query_id.is_empty() || query_id.len() > limits.max_query_id_bytes() {
            return Err(Oid4vpError::UnknownDcqlResult);
        }
        if presentations.is_empty() {
            return Err(Oid4vpError::EmptyPresentationList);
        }
        if presentations.len() > limits.max_presentations_per_query() {
            return Err(Oid4vpError::TooManyPresentations);
        }
        Ok(Self {
            query_id: Zeroizing::new(query_id.to_owned()),
            presentations,
        })
    }

    /// Explicitly reveal the verifier-provided query identifier.
    pub fn expose_sensitive_query_id(&self) -> &str {
        &self.query_id
    }

    /// Number of presentation values for this query.
    pub fn presentation_count(&self) -> usize {
        self.presentations.len()
    }
}

impl fmt::Debug for DirectPostQueryResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DirectPostQueryResult")
            .field("query_id_bytes", &self.query_id.len())
            .field("presentation_count", &self.presentations.len())
            .finish()
    }
}

/// Inert, bounded HTTP request material for one plain direct-post response.
pub struct DirectPostResponse {
    target: Zeroizing<String>,
    form_body: Zeroizing<String>,
    state_present: bool,
    query_result_count: usize,
}

impl ValidatedAuthorizationRequest {
    /// Consume validated request lineage and construct a direct-post envelope.
    ///
    /// # Errors
    ///
    /// Returns a static OID4VP category if the result identifiers, coverage,
    /// cardinality, presentation values, JSON output, or form body violate the
    /// validated request or supplied resource policy.
    pub fn into_direct_post_response(
        self,
        results: Vec<DirectPostQueryResult>,
        limits: DirectPostResponseLimits,
    ) -> Result<DirectPostResponse, Oid4vpError> {
        if results.len() > limits.max_query_results() {
            return Err(Oid4vpError::TooManyQueryResults);
        }
        validate_result_limits(&results, limits)?;
        let (target, state, query) = self.into_direct_post_parts();
        let ordered = validate_results(query.response_rules(), &results)?;
        let vp_token = serialize_vp_token(query.response_rules(), &ordered, limits)?;
        let form_body = serialize_form(
            &vp_token,
            state.as_ref().map(|value| value.as_str()),
            limits,
        )?;

        Ok(DirectPostResponse {
            target,
            form_body,
            state_present: state.is_some(),
            query_result_count: results.len(),
        })
    }
}

fn validate_result_limits(
    results: &[DirectPostQueryResult],
    limits: DirectPostResponseLimits,
) -> Result<(), Oid4vpError> {
    for result in results {
        if result.query_id.is_empty() || result.query_id.len() > limits.max_query_id_bytes() {
            return Err(Oid4vpError::UnknownDcqlResult);
        }
        if result.presentations.is_empty() {
            return Err(Oid4vpError::EmptyPresentationList);
        }
        if result.presentations.len() > limits.max_presentations_per_query() {
            return Err(Oid4vpError::TooManyPresentations);
        }
        for presentation in &result.presentations {
            if presentation.encoded.len() > limits.max_presentation_bytes()
                || presentation.json_depth > limits.max_json_depth()
                || presentation.json_nodes > limits.max_json_nodes()
            {
                return Err(Oid4vpError::PresentationTooLarge);
            }
        }
    }
    Ok(())
}

impl DirectPostResponse {
    /// Return the required HTTP method.
    pub const fn http_method(&self) -> &'static str {
        DIRECT_POST_HTTP_METHOD
    }

    /// Return the required request-body media type.
    pub const fn media_type(&self) -> &'static str {
        DIRECT_POST_MEDIA_TYPE
    }

    /// Exact target URI byte count.
    pub fn target_len(&self) -> usize {
        self.target.len()
    }

    /// Exact encoded form-body byte count.
    pub fn form_body_len(&self) -> usize {
        self.form_body.len()
    }

    /// Whether the response returns signed state.
    pub const fn state_present(&self) -> bool {
        self.state_present
    }

    /// Number of included DCQL query results.
    pub const fn query_result_count(&self) -> usize {
        self.query_result_count
    }

    /// Explicitly reveal the validated HTTPS transport target.
    ///
    /// This accessor does not authorize network I/O. The caller owns DNS,
    /// address policy, TLS, redirects, timeouts, retries, and SSRF defenses.
    pub fn expose_sensitive_target(&self) -> &str {
        &self.target
    }

    /// Explicitly reveal the presentation-bearing form body for transport.
    pub fn expose_sensitive_form_body(&self) -> &str {
        &self.form_body
    }
}

impl fmt::Debug for DirectPostResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DirectPostResponse")
            .field("target_bytes", &self.target.len())
            .field("form_body_bytes", &self.form_body.len())
            .field("state_present", &self.state_present)
            .field("query_result_count", &self.query_result_count)
            .finish_non_exhaustive()
    }
}

fn validate_results<'a>(
    rules: &DcqlResponseRules,
    results: &'a [DirectPostQueryResult],
) -> Result<BTreeMap<&'a str, &'a DirectPostQueryResult>, Oid4vpError> {
    let known: BTreeMap<_, _> = rules
        .queries()
        .iter()
        .map(|rule| (rule.id(), rule))
        .collect();
    let mut indexed = BTreeMap::new();
    for result in results {
        let id = result.query_id.as_str();
        let Some(rule) = known.get(id) else {
            return Err(Oid4vpError::UnknownDcqlResult);
        };
        if indexed.insert(id, result).is_some() {
            return Err(Oid4vpError::DuplicateDcqlResult);
        }
        validate_cardinality(rule, result)?;
    }

    match rules.credential_sets() {
        None => {
            if rules
                .queries()
                .iter()
                .any(|rule| !indexed.contains_key(rule.id()))
            {
                return Err(Oid4vpError::MissingDcqlResult);
            }
        }
        Some(sets) => {
            for set in sets.iter().filter(|set| set.required()) {
                let satisfied = set
                    .options()
                    .iter()
                    .any(|option| option.iter().all(|id| indexed.contains_key(id.as_str())));
                if !satisfied {
                    return Err(Oid4vpError::MissingDcqlResult);
                }
            }
        }
    }
    Ok(indexed)
}

fn validate_cardinality(
    rule: &DcqlResponseQueryRule,
    result: &DirectPostQueryResult,
) -> Result<(), Oid4vpError> {
    if !rule.multiple() && result.presentations.len() != 1 {
        Err(Oid4vpError::DcqlCardinalityMismatch)
    } else {
        Ok(())
    }
}

fn serialize_vp_token(
    rules: &DcqlResponseRules,
    results: &BTreeMap<&str, &DirectPostQueryResult>,
    limits: DirectPostResponseLimits,
) -> Result<Zeroizing<String>, Oid4vpError> {
    let mut output = Zeroizing::new(String::with_capacity(
        limits.max_vp_token_bytes().min(4_096),
    ));
    push_json(&mut output, "{", limits)?;
    let mut first_result = true;
    for rule in rules.queries() {
        let Some(result) = results.get(rule.id()) else {
            continue;
        };
        if !first_result {
            push_json(&mut output, ",", limits)?;
        }
        first_result = false;
        let name = Zeroizing::new(
            serde_json::to_string(rule.id()).map_err(|_| Oid4vpError::ResponseJsonTooLarge)?,
        );
        push_json(&mut output, &name, limits)?;
        push_json(&mut output, ":[", limits)?;
        for (index, presentation) in result.presentations.iter().enumerate() {
            if index != 0 {
                push_json(&mut output, ",", limits)?;
            }
            push_json(&mut output, presentation.encoded(), limits)?;
        }
        push_json(&mut output, "]", limits)?;
    }
    push_json(&mut output, "}", limits)?;
    Ok(output)
}

fn push_json(
    output: &mut String,
    value: &str,
    limits: DirectPostResponseLimits,
) -> Result<(), Oid4vpError> {
    let next = output
        .len()
        .checked_add(value.len())
        .ok_or(Oid4vpError::ResponseJsonTooLarge)?;
    if next > limits.max_vp_token_bytes() {
        return Err(Oid4vpError::ResponseJsonTooLarge);
    }
    output.push_str(value);
    Ok(())
}

fn serialize_form(
    vp_token: &str,
    state: Option<&str>,
    limits: DirectPostResponseLimits,
) -> Result<Zeroizing<String>, Oid4vpError> {
    let vp_pair = pair_len("vp_token", vp_token).ok_or(Oid4vpError::DirectPostFormTooLarge)?;
    let total = match state {
        Some(state) => vp_pair
            .checked_add(1)
            .and_then(|length| pair_len("state", state).and_then(|pair| length.checked_add(pair)))
            .ok_or(Oid4vpError::DirectPostFormTooLarge)?,
        None => vp_pair,
    };
    if total > limits.max_form_body_bytes() {
        return Err(Oid4vpError::DirectPostFormTooLarge);
    }
    let mut output = Zeroizing::new(String::with_capacity(total));
    append_pair(&mut output, "vp_token", vp_token);
    if let Some(state) = state {
        output.push('&');
        append_pair(&mut output, "state", state);
    }
    debug_assert_eq!(output.len(), total);
    Ok(output)
}

fn pair_len(name: &str, value: &str) -> Option<usize> {
    encoded_len(name)?
        .checked_add(1)?
        .checked_add(encoded_len(value)?)
}

fn encoded_len(value: &str) -> Option<usize> {
    value.as_bytes().iter().try_fold(0usize, |length, byte| {
        length.checked_add(if is_form_literal(*byte) || *byte == b' ' {
            1
        } else {
            3
        })
    })
}

fn append_pair(output: &mut String, name: &str, value: &str) {
    append_encoded(output, name);
    output.push('=');
    append_encoded(output, value);
}

fn append_encoded(output: &mut String, value: &str) {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    for byte in value.bytes() {
        if is_form_literal(byte) {
            output.push(char::from(byte));
        } else if byte == b' ' {
            output.push('+');
        } else {
            output.push('%');
            output.push(char::from(HEX[usize::from(byte >> 4)]));
            output.push(char::from(HEX[usize::from(byte & 0x0f)]));
        }
    }
}

const fn is_form_literal(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'*' | b'-' | b'.' | b'_')
}

const JSON_WORK_LIMIT_MARKER: &str = "identus-oid4vp:json-work-limit";

fn parse_bounded_json(
    input: &[u8],
    limits: DirectPostResponseLimits,
) -> Result<(Value, usize, usize), Oid4vpError> {
    let mut work = JsonWork {
        nodes: 0,
        maximum_depth: 0,
        limits,
    };
    let mut deserializer = serde_json::Deserializer::from_slice(input);
    let value = BoundedValueSeed {
        work: &mut work,
        parent_depth: 0,
    }
    .deserialize(&mut deserializer)
    .map_err(map_json_error)?;
    deserializer.end().map_err(map_json_error)?;
    Ok((value, work.maximum_depth, work.nodes))
}

fn map_json_error(error: serde_json::Error) -> Oid4vpError {
    if error.to_string().contains(JSON_WORK_LIMIT_MARKER) {
        Oid4vpError::PresentationTooLarge
    } else {
        Oid4vpError::InvalidPresentation
    }
}

struct JsonWork {
    nodes: usize,
    maximum_depth: usize,
    limits: DirectPostResponseLimits,
}

impl JsonWork {
    fn visit_node<E: serde::de::Error>(&mut self) -> Result<(), E> {
        self.nodes = self
            .nodes
            .checked_add(1)
            .ok_or_else(|| E::custom(JSON_WORK_LIMIT_MARKER))?;
        if self.nodes > self.limits.max_json_nodes() {
            return Err(E::custom(JSON_WORK_LIMIT_MARKER));
        }
        Ok(())
    }

    fn enter_container<E: serde::de::Error>(&mut self, depth: usize) -> Result<(), E> {
        if depth > self.limits.max_json_depth() {
            return Err(E::custom(JSON_WORK_LIMIT_MARKER));
        }
        self.maximum_depth = self.maximum_depth.max(depth);
        Ok(())
    }
}

struct BoundedValueSeed<'work> {
    work: &'work mut JsonWork,
    parent_depth: usize,
}

impl<'de> DeserializeSeed<'de> for BoundedValueSeed<'_> {
    type Value = Value;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(BoundedValueVisitor {
            work: self.work,
            parent_depth: self.parent_depth,
        })
    }
}

struct BoundedValueVisitor<'work> {
    work: &'work mut JsonWork,
    parent_depth: usize,
}

impl<'de> Visitor<'de> for BoundedValueVisitor<'_> {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a bounded JSON value")
    }

    fn visit_bool<E: serde::de::Error>(self, value: bool) -> Result<Self::Value, E> {
        self.work.visit_node()?;
        Ok(Value::Bool(value))
    }

    fn visit_i64<E: serde::de::Error>(self, value: i64) -> Result<Self::Value, E> {
        self.work.visit_node()?;
        Ok(Value::Number(value.into()))
    }

    fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<Self::Value, E> {
        self.work.visit_node()?;
        Ok(Value::Number(value.into()))
    }

    fn visit_f64<E: serde::de::Error>(self, value: f64) -> Result<Self::Value, E> {
        self.work.visit_node()?;
        serde_json::Number::from_f64(value)
            .map(Value::Number)
            .ok_or_else(|| E::custom("invalid JSON number"))
    }

    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
        self.work.visit_node()?;
        Ok(Value::String(value.to_owned()))
    }

    fn visit_string<E: serde::de::Error>(self, value: String) -> Result<Self::Value, E> {
        self.work.visit_node()?;
        Ok(Value::String(value))
    }

    fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value, E> {
        self.work.visit_node()?;
        Ok(Value::Null)
    }

    fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value, E> {
        self.work.visit_node()?;
        Ok(Value::Null)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        BoundedValueSeed {
            work: self.work,
            parent_depth: self.parent_depth,
        }
        .deserialize(deserializer)
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        self.work.visit_node()?;
        let depth = self.parent_depth + 1;
        self.work.enter_container(depth)?;
        let mut values = Vec::new();
        while let Some(value) = sequence.next_element_seed(BoundedValueSeed {
            work: &mut *self.work,
            parent_depth: depth,
        })? {
            values.push(value);
        }
        Ok(Value::Array(values))
    }

    fn visit_map<A>(self, mut object: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        self.work.visit_node()?;
        let depth = self.parent_depth + 1;
        self.work.enter_container(depth)?;
        let mut values = Map::new();
        while let Some(name) = object.next_key::<String>()? {
            if values.contains_key(&name) {
                return Err(A::Error::custom("duplicate JSON object member"));
            }
            let value = object.next_value_seed(BoundedValueSeed {
                work: &mut *self.work,
                parent_depth: depth,
            })?;
            values.insert(name, value);
        }
        Ok(Value::Object(values))
    }
}
