//! Bounded construction of OID4VP Final `direct_post.jwt` responses.

use std::fmt;

use identus_jose::{CompactJwe, JweEncrypter, JweEncryptionRequest, JweLimits};
use zeroize::Zeroizing;

use crate::{
    AuthorizationResponseMode, DirectPostQueryResult, DirectPostResponseLimits, Oid4vpError,
    ValidatedAuthorizationRequest,
    authorization_request::AuthorizationResponseDestination,
    direct_post::{
        DIRECT_POST_HTTP_METHOD, DIRECT_POST_MEDIA_TYPE, append_pair, pair_len, serialize_vp_token,
        validate_result_limits, validate_results,
    },
};

/// Inert, bounded HTTP request material for one encrypted direct-post response.
pub struct EncryptedDirectPostResponse {
    target: Zeroizing<String>,
    form_body: Zeroizing<String>,
    compact_bytes: usize,
    state_present: bool,
    query_result_count: usize,
}

impl ValidatedAuthorizationRequest {
    /// Consume validated lineage and construct a Final `direct_post.jwt` envelope.
    ///
    /// The injected encrypter is invoked once after all deterministic request,
    /// cardinality, plaintext, and worst-case form bounds have passed.
    pub fn into_encrypted_direct_post_response(
        self,
        results: Vec<DirectPostQueryResult>,
        response_limits: DirectPostResponseLimits,
        jwe_limits: JweLimits,
        encrypter: &impl JweEncrypter,
    ) -> Result<EncryptedDirectPostResponse, Oid4vpError> {
        if self.response_mode() != AuthorizationResponseMode::DirectPostJwt {
            return Err(Oid4vpError::ResponseModeMismatch);
        }
        if results.len() > response_limits.max_query_results() {
            return Err(Oid4vpError::TooManyQueryResults);
        }
        validate_result_limits(&results, response_limits)?;
        let (destination, state, query, _mode, encryption_plan) =
            self.into_success_response_parts(&results)?;
        let AuthorizationResponseDestination::ResponseUri(target) = destination else {
            return Err(Oid4vpError::ResponseModeMismatch);
        };
        let encryption_plan = encryption_plan.ok_or(Oid4vpError::ResponseModeMismatch)?;
        let ordered = validate_results(query.response_rules(), &results)?;
        let vp_token = serialize_vp_token(query.response_rules(), &ordered, response_limits)?;
        let plaintext = serialize_encrypted_payload(
            &vp_token,
            state.as_ref().map(|value| value.as_str()),
            jwe_limits,
        )?;
        validate_encrypted_form_upper_bound(response_limits, jwe_limits)?;
        let expected_key_id = encryption_plan
            .recipient
            .expose_key_id()
            .map(|value| Zeroizing::new(value.to_owned()));
        let request = JweEncryptionRequest::new(
            encryption_plan.recipient,
            encryption_plan.key_management,
            encryption_plan.content_encryption,
            plaintext.as_bytes(),
            jwe_limits,
        )
        .map_err(|_| Oid4vpError::ResponseEncryptionFailed)?;
        let compact = encrypter
            .encrypt(request)
            .map_err(|_| Oid4vpError::ResponseEncryptionFailed)?;
        if compact.len() > jwe_limits.max_compact_bytes() {
            return Err(Oid4vpError::ResponseEncryptionFailed);
        }
        let compact = CompactJwe::new(
            compact.expose_sensitive_compact().to_owned(),
            jwe_limits.max_compact_bytes(),
        )
        .map_err(|_| Oid4vpError::ResponseEncryptionFailed)?;
        if compact.expose_key_id() != expected_key_id.as_ref().map(|value| value.as_str()) {
            return Err(Oid4vpError::ResponseEncryptionFailed);
        }
        let form_body = serialize_encrypted_form(
            compact.expose_sensitive_compact(),
            response_limits.max_form_body_bytes(),
        )?;
        Ok(EncryptedDirectPostResponse {
            target,
            form_body,
            compact_bytes: compact.len(),
            state_present: state.is_some(),
            query_result_count: results.len(),
        })
    }
}

impl EncryptedDirectPostResponse {
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

    /// Exact compact JWE byte count before form encoding.
    pub const fn compact_len(&self) -> usize {
        self.compact_bytes
    }

    /// Whether the encrypted plaintext carries signed request state.
    pub const fn state_present(&self) -> bool {
        self.state_present
    }

    /// Number of included DCQL query results.
    pub const fn query_result_count(&self) -> usize {
        self.query_result_count
    }

    /// Explicitly reveal the validated HTTPS transport target.
    pub fn expose_sensitive_target(&self) -> &str {
        &self.target
    }

    /// Explicitly reveal the encrypted-response form body for transport.
    pub fn expose_sensitive_form_body(&self) -> &str {
        &self.form_body
    }
}

impl fmt::Debug for EncryptedDirectPostResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("EncryptedDirectPostResponse")
            .field("target_bytes", &self.target.len())
            .field("form_body_bytes", &self.form_body.len())
            .field("compact_bytes", &self.compact_bytes)
            .field("state_present", &self.state_present)
            .field("query_result_count", &self.query_result_count)
            .finish_non_exhaustive()
    }
}

fn serialize_encrypted_payload(
    vp_token: &str,
    state: Option<&str>,
    limits: JweLimits,
) -> Result<Zeroizing<String>, Oid4vpError> {
    let state = state
        .map(serde_json::to_string)
        .transpose()
        .map_err(|_| Oid4vpError::ResponseJsonTooLarge)?
        .map(Zeroizing::new);
    let mut length = "{\"vp_token\":"
        .len()
        .checked_add(vp_token.len())
        .and_then(|value| value.checked_add(1))
        .ok_or(Oid4vpError::ResponseJsonTooLarge)?;
    if let Some(state) = state.as_ref() {
        length = length
            .checked_add(",\"state\":".len())
            .and_then(|value| value.checked_add(state.len()))
            .ok_or(Oid4vpError::ResponseJsonTooLarge)?;
    }
    if length > limits.max_plaintext_bytes() {
        return Err(Oid4vpError::ResponseJsonTooLarge);
    }
    let mut output = Zeroizing::new(String::with_capacity(length));
    output.push_str("{\"vp_token\":");
    output.push_str(vp_token);
    if let Some(state) = state.as_ref() {
        output.push_str(",\"state\":");
        output.push_str(state);
    }
    output.push('}');
    debug_assert_eq!(output.len(), length);
    Ok(output)
}

fn validate_encrypted_form_upper_bound(
    response_limits: DirectPostResponseLimits,
    jwe_limits: JweLimits,
) -> Result<(), Oid4vpError> {
    let maximum = jwe_limits
        .max_compact_bytes()
        .checked_add("response=".len())
        .ok_or(Oid4vpError::DirectPostFormTooLarge)?;
    if maximum > response_limits.max_form_body_bytes() {
        return Err(Oid4vpError::DirectPostFormTooLarge);
    }
    Ok(())
}

fn serialize_encrypted_form(
    compact: &str,
    max_form_body_bytes: usize,
) -> Result<Zeroizing<String>, Oid4vpError> {
    let total = pair_len("response", compact).ok_or(Oid4vpError::DirectPostFormTooLarge)?;
    if total > max_form_body_bytes {
        return Err(Oid4vpError::DirectPostFormTooLarge);
    }
    let mut output = Zeroizing::new(String::with_capacity(total));
    append_pair(&mut output, "response", compact);
    Ok(output)
}
