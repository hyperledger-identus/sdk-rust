//! Bounded construction of plain OID4VP `direct_post` success responses.

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
};

use fluent_uri::UriRef;
use serde::de::{DeserializeSeed, Error as _, MapAccess, SeqAccess, Visitor};
use zeroize::{Zeroize, Zeroizing};

use crate::{
    AuthorizationResponseMode, DirectPostErrorResponseLimits, DirectPostResponseLimits,
    Oid4vpError, ValidatedAuthorizationRequest,
    authorization_request::AuthorizationResponseDestination,
    dcql::{DcqlResponseQueryRule, DcqlResponseRules},
};

/// HTTP method required for a plain direct-post response.
pub const DIRECT_POST_HTTP_METHOD: &str = "POST";

/// Media type required for a plain direct-post response body.
pub const DIRECT_POST_MEDIA_TYPE: &str = "application/x-www-form-urlencoded";

/// Closed RFC 6749 and OpenID4VP 1.0 Final authorization-error vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectPostErrorCode {
    InvalidRequest,
    InvalidClient,
    UnauthorizedClient,
    AccessDenied,
    UnsupportedResponseType,
    InvalidScope,
    ServerError,
    TemporarilyUnavailable,
    VpFormatsNotSupported,
    InvalidRequestUriMethod,
    InvalidTransactionData,
    WalletUnavailable,
}

impl DirectPostErrorCode {
    /// Return the exact case-sensitive wire value.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidRequest => "invalid_request",
            Self::InvalidClient => "invalid_client",
            Self::UnauthorizedClient => "unauthorized_client",
            Self::AccessDenied => "access_denied",
            Self::UnsupportedResponseType => "unsupported_response_type",
            Self::InvalidScope => "invalid_scope",
            Self::ServerError => "server_error",
            Self::TemporarilyUnavailable => "temporarily_unavailable",
            Self::VpFormatsNotSupported => "vp_formats_not_supported",
            Self::InvalidRequestUriMethod => "invalid_request_uri_method",
            Self::InvalidTransactionData => "invalid_transaction_data",
            Self::WalletUnavailable => "wallet_unavailable",
        }
    }
}

/// Optional bounded developer-oriented fields for a protocol-error response.
///
/// Syntax validation does not make either value safe for display or resolution.
pub struct DirectPostErrorDetails {
    pub(crate) description: Option<Zeroizing<String>>,
    pub(crate) uri: Option<Zeroizing<String>>,
}

impl DirectPostErrorDetails {
    /// Construct optional error details under explicit resource limits.
    pub fn new(
        description: Option<&str>,
        uri: Option<&str>,
        limits: DirectPostErrorResponseLimits,
    ) -> Result<Self, Oid4vpError> {
        let description = description
            .map(|value| validate_error_description(value, limits))
            .transpose()?;
        let uri = uri
            .map(|value| validate_error_uri(value, limits))
            .transpose()?;
        Ok(Self { description, uri })
    }

    /// Construct the privacy-minimal response without developer fields.
    pub const fn empty() -> Self {
        Self {
            description: None,
            uri: None,
        }
    }

    /// Report whether developer-oriented text is present.
    pub const fn description_present(&self) -> bool {
        self.description.is_some()
    }

    /// Report whether an untrusted URI-reference is present.
    pub const fn error_uri_present(&self) -> bool {
        self.uri.is_some()
    }
}

impl fmt::Debug for DirectPostErrorDetails {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DirectPostErrorDetails")
            .field("description_present", &self.description.is_some())
            .field("error_uri_present", &self.uri.is_some())
            .finish_non_exhaustive()
    }
}

/// Inert, bounded HTTP request material for one plain direct-post error response.
pub struct DirectPostErrorResponse {
    target: Zeroizing<String>,
    form_body: Zeroizing<String>,
    code: DirectPostErrorCode,
    state_present: bool,
    description_present: bool,
    error_uri_present: bool,
}

impl DirectPostErrorResponse {
    /// Return the required HTTP method.
    pub const fn http_method(&self) -> &'static str {
        DIRECT_POST_HTTP_METHOD
    }

    /// Return the required request-body media type.
    pub const fn media_type(&self) -> &'static str {
        DIRECT_POST_MEDIA_TYPE
    }

    /// Return the exact selected protocol error code.
    pub const fn error_code(&self) -> DirectPostErrorCode {
        self.code
    }

    /// Exact target URI byte count.
    pub fn target_len(&self) -> usize {
        self.target.len()
    }

    /// Exact encoded form-body byte count.
    pub fn form_body_len(&self) -> usize {
        self.form_body.len()
    }

    /// Whether exact request state is included.
    pub const fn state_present(&self) -> bool {
        self.state_present
    }

    /// Whether developer-oriented text is included.
    pub const fn description_present(&self) -> bool {
        self.description_present
    }

    /// Whether an untrusted URI-reference is included.
    pub const fn error_uri_present(&self) -> bool {
        self.error_uri_present
    }

    /// Explicitly reveal the validated HTTPS transport target.
    pub fn expose_sensitive_target(&self) -> &str {
        &self.target
    }

    /// Explicitly reveal the protocol-error form body for transport.
    pub fn expose_sensitive_form_body(&self) -> &str {
        &self.form_body
    }
}

impl fmt::Debug for DirectPostErrorResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DirectPostErrorResponse")
            .field("code", &self.code)
            .field("target_bytes", &self.target.len())
            .field("form_body_bytes", &self.form_body.len())
            .field("state_present", &self.state_present)
            .field("description_present", &self.description_present)
            .field("error_uri_present", &self.error_uri_present)
            .finish_non_exhaustive()
    }
}

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

    /// Validate and whitespace-minify one bounded JSON object presentation.
    pub fn object_json(
        input: &[u8],
        limits: DirectPostResponseLimits,
    ) -> Result<Self, Oid4vpError> {
        if input.len() > limits.max_presentation_bytes() {
            return Err(Oid4vpError::PresentationTooLarge);
        }
        let (kind, json_depth, json_nodes) = parse_bounded_json(input, limits)?;
        if kind != ParsedJsonKind::Object {
            return Err(Oid4vpError::InvalidPresentation);
        }
        let encoded = minify_exact_json(input)?;
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
        if self.response_mode() != AuthorizationResponseMode::DirectPost {
            return Err(Oid4vpError::ResponseModeMismatch);
        }
        if results.len() > limits.max_query_results() {
            return Err(Oid4vpError::TooManyQueryResults);
        }
        validate_result_limits(&results, limits)?;
        let (destination, state, query, _mode, _encryption_plan) = self.into_response_parts();
        let AuthorizationResponseDestination::ResponseUri(target) = destination else {
            return Err(Oid4vpError::ResponseModeMismatch);
        };
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

    /// Consume validated request lineage and construct a protocol-error envelope.
    ///
    /// This method does not translate SDK failures, authorize disclosure, obtain
    /// End-User consent, or execute transport. The caller must select the wire
    /// error explicitly after applying those policies.
    pub fn into_direct_post_error_response(
        self,
        code: DirectPostErrorCode,
        details: DirectPostErrorDetails,
        limits: DirectPostErrorResponseLimits,
    ) -> Result<DirectPostErrorResponse, Oid4vpError> {
        if self.response_mode() != AuthorizationResponseMode::DirectPost {
            return Err(Oid4vpError::ResponseModeMismatch);
        }
        validate_retained_error_details(&details, limits)?;
        let (destination, state, _query, _mode, _encryption_plan) = self.into_response_parts();
        let AuthorizationResponseDestination::ResponseUri(target) = destination else {
            return Err(Oid4vpError::ResponseModeMismatch);
        };
        let description_present = details.description.is_some();
        let error_uri_present = details.uri.is_some();
        let form_body = serialize_error_form(
            code,
            details.description.as_ref().map(|value| value.as_str()),
            details.uri.as_ref().map(|value| value.as_str()),
            state.as_ref().map(|value| value.as_str()),
            limits,
        )?;
        Ok(DirectPostErrorResponse {
            target,
            form_body,
            code,
            state_present: state.is_some(),
            description_present,
            error_uri_present,
        })
    }
}

fn validate_error_description(
    value: &str,
    limits: DirectPostErrorResponseLimits,
) -> Result<Zeroizing<String>, Oid4vpError> {
    if value.len() > limits.max_error_description_bytes() {
        return Err(Oid4vpError::ProtocolErrorDescriptionTooLarge);
    }
    if value.is_empty() || !value.bytes().all(is_nqschar) {
        return Err(Oid4vpError::InvalidProtocolErrorDescription);
    }
    Ok(Zeroizing::new(value.to_owned()))
}

fn validate_error_uri(
    value: &str,
    limits: DirectPostErrorResponseLimits,
) -> Result<Zeroizing<String>, Oid4vpError> {
    if value.len() > limits.max_error_uri_bytes() {
        return Err(Oid4vpError::ProtocolErrorUriTooLarge);
    }
    if value.is_empty()
        || !value.bytes().all(is_uri_reference_char)
        || UriRef::parse(value).is_err()
    {
        return Err(Oid4vpError::InvalidProtocolErrorUri);
    }
    Ok(Zeroizing::new(value.to_owned()))
}

pub(crate) fn validate_retained_error_details(
    details: &DirectPostErrorDetails,
    limits: DirectPostErrorResponseLimits,
) -> Result<(), Oid4vpError> {
    if let Some(description) = details.description.as_ref().map(|value| value.as_str()) {
        validate_error_description(description, limits)?;
    }
    if let Some(uri) = details.uri.as_ref().map(|value| value.as_str()) {
        validate_error_uri(uri, limits)?;
    }
    Ok(())
}

const fn is_nqschar(byte: u8) -> bool {
    matches!(byte, 0x20..=0x21 | 0x23..=0x5b | 0x5d..=0x7e)
}

const fn is_uri_reference_char(byte: u8) -> bool {
    matches!(byte, 0x21 | 0x23..=0x5b | 0x5d..=0x7e)
}

pub(crate) fn validate_result_limits(
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

pub(crate) fn validate_results<'a>(
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

pub(crate) fn serialize_vp_token(
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

pub(crate) fn serialize_form(
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

pub(crate) fn serialize_error_form(
    code: DirectPostErrorCode,
    description: Option<&str>,
    uri: Option<&str>,
    state: Option<&str>,
    limits: DirectPostErrorResponseLimits,
) -> Result<Zeroizing<String>, Oid4vpError> {
    let fields = [
        Some(("error", code.as_str())),
        description.map(|value| ("error_description", value)),
        uri.map(|value| ("error_uri", value)),
        state.map(|value| ("state", value)),
    ];
    let mut total = 0usize;
    for (count, (name, value)) in fields.into_iter().flatten().enumerate() {
        let pair = pair_len(name, value).ok_or(Oid4vpError::DirectPostErrorFormTooLarge)?;
        total = total
            .checked_add(usize::from(count != 0))
            .and_then(|length| length.checked_add(pair))
            .ok_or(Oid4vpError::DirectPostErrorFormTooLarge)?;
    }
    if total > limits.max_form_body_bytes() {
        return Err(Oid4vpError::DirectPostErrorFormTooLarge);
    }
    let mut output = Zeroizing::new(String::with_capacity(total));
    let mut first = true;
    for (name, value) in fields.into_iter().flatten() {
        if !first {
            output.push('&');
        }
        first = false;
        append_pair(&mut output, name, value);
    }
    debug_assert_eq!(output.len(), total);
    Ok(output)
}

pub(crate) fn pair_len(name: &str, value: &str) -> Option<usize> {
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

pub(crate) fn append_pair(output: &mut String, name: &str, value: &str) {
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum ParsedJsonKind {
    Object,
    Other,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct ObjectMemberName(String);

impl Drop for ObjectMemberName {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

fn parse_bounded_json(
    input: &[u8],
    limits: DirectPostResponseLimits,
) -> Result<(ParsedJsonKind, usize, usize), Oid4vpError> {
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

fn minify_exact_json(input: &[u8]) -> Result<String, Oid4vpError> {
    let mut output = Zeroizing::new(Vec::with_capacity(input.len()));
    let mut in_string = false;
    let mut escaped = false;
    for &byte in input {
        if in_string {
            output.push(byte);
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                in_string = false;
            }
        } else if byte == b'"' {
            in_string = true;
            output.push(byte);
        } else if !byte.is_ascii_whitespace() {
            output.push(byte);
        }
    }
    String::from_utf8(std::mem::take(&mut *output)).map_err(|_| Oid4vpError::InvalidPresentation)
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
    type Value = ParsedJsonKind;

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
    type Value = ParsedJsonKind;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a bounded JSON value")
    }

    fn visit_bool<E: serde::de::Error>(self, value: bool) -> Result<Self::Value, E> {
        self.work.visit_node()?;
        let _ = value;
        Ok(ParsedJsonKind::Other)
    }

    fn visit_i64<E: serde::de::Error>(self, value: i64) -> Result<Self::Value, E> {
        self.work.visit_node()?;
        let _ = value;
        Ok(ParsedJsonKind::Other)
    }

    fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<Self::Value, E> {
        self.work.visit_node()?;
        let _ = value;
        Ok(ParsedJsonKind::Other)
    }

    fn visit_f64<E: serde::de::Error>(self, value: f64) -> Result<Self::Value, E> {
        self.work.visit_node()?;
        let _ = value;
        Ok(ParsedJsonKind::Other)
    }

    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
        self.work.visit_node()?;
        let _ = value;
        Ok(ParsedJsonKind::Other)
    }

    fn visit_string<E: serde::de::Error>(self, value: String) -> Result<Self::Value, E> {
        self.work.visit_node()?;
        drop(Zeroizing::new(value));
        Ok(ParsedJsonKind::Other)
    }

    fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value, E> {
        self.work.visit_node()?;
        Ok(ParsedJsonKind::Other)
    }

    fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value, E> {
        self.work.visit_node()?;
        Ok(ParsedJsonKind::Other)
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
        while sequence
            .next_element_seed(BoundedValueSeed {
                work: &mut *self.work,
                parent_depth: depth,
            })?
            .is_some()
        {}
        Ok(ParsedJsonKind::Other)
    }

    fn visit_map<A>(self, mut object: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        self.work.visit_node()?;
        let depth = self.parent_depth + 1;
        self.work.enter_container(depth)?;
        let mut names = BTreeSet::<ObjectMemberName>::new();
        while let Some(name) = object.next_key::<String>()? {
            let name = ObjectMemberName(name);
            if !names.insert(name) {
                return Err(A::Error::custom("duplicate JSON object member"));
            }
            object.next_value_seed(BoundedValueSeed {
                work: &mut *self.work,
                parent_depth: depth,
            })?;
        }
        Ok(ParsedJsonKind::Object)
    }
}
