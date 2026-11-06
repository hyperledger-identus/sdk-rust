//! Effect-neutral OID4VP query, fragment, and form-post response material.

use std::fmt;

use zeroize::Zeroizing;

use crate::{
    AuthorizationResponseMode, DirectPostErrorCode, DirectPostErrorDetails,
    DirectPostErrorResponseLimits, DirectPostQueryResult, DirectPostResponseLimits, Oid4vpError,
    RedirectResponseLimits, ValidatedAuthorizationRequest,
    authorization_request::AuthorizationResponseDestination,
    direct_post::{
        serialize_error_form, serialize_form, serialize_vp_token, validate_result_limits,
        validate_results, validate_retained_error_details,
    },
};

/// Whether inert redirect response material represents success or a protocol error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum RedirectResponseKind {
    /// A `vp_token` response satisfying the validated DCQL request.
    Success,
    /// An explicitly selected OAuth/OpenID4VP protocol error.
    Error(DirectPostErrorCode),
}

/// Bounded, inert response material for an OAuth redirect response mode.
///
/// Query and fragment modes expose a complete URI. Form-post mode exposes the
/// exact target plus an encoded body. This type never performs navigation,
/// renders HTML, or executes transport.
pub struct RedirectResponse {
    mode: AuthorizationResponseMode,
    kind: RedirectResponseKind,
    target: Zeroizing<String>,
    form_body: Option<Zeroizing<String>>,
    state_present: bool,
}

impl ValidatedAuthorizationRequest {
    /// Consume validated lineage and construct one success redirect response.
    pub fn into_redirect_response(
        self,
        results: Vec<DirectPostQueryResult>,
        response_limits: DirectPostResponseLimits,
        redirect_limits: RedirectResponseLimits,
    ) -> Result<RedirectResponse, Oid4vpError> {
        let mode = self.response_mode();
        require_redirect_mode(mode)?;
        if results.len() > response_limits.max_query_results() {
            return Err(Oid4vpError::TooManyQueryResults);
        }
        validate_result_limits(&results, response_limits)?;
        let (destination, state, query, _, encryption) =
            self.into_success_response_parts(&results)?;
        if encryption.is_some() {
            return Err(Oid4vpError::ResponseModeMismatch);
        }
        let AuthorizationResponseDestination::RedirectUri(destination) = destination else {
            return Err(Oid4vpError::ResponseModeMismatch);
        };
        let ordered = validate_results(query.response_rules(), &results)?;
        let vp_token = serialize_vp_token(query.response_rules(), &ordered, response_limits)?;
        let parameters = serialize_form(
            &vp_token,
            state.as_ref().map(|value| value.as_str()),
            response_limits,
        )?;
        build_response(
            mode,
            RedirectResponseKind::Success,
            destination,
            parameters,
            state.is_some(),
            redirect_limits,
        )
    }

    /// Consume validated lineage and construct one redirect-mode protocol error.
    pub fn into_redirect_error_response(
        self,
        code: DirectPostErrorCode,
        details: DirectPostErrorDetails,
        error_limits: DirectPostErrorResponseLimits,
        redirect_limits: RedirectResponseLimits,
    ) -> Result<RedirectResponse, Oid4vpError> {
        let mode = self.response_mode();
        require_redirect_mode(mode)?;
        validate_retained_error_details(&details, error_limits)?;
        let (destination, state, _query, _, encryption) = self.into_error_response_parts();
        if encryption.is_some() {
            return Err(Oid4vpError::ResponseModeMismatch);
        }
        let AuthorizationResponseDestination::RedirectUri(destination) = destination else {
            return Err(Oid4vpError::ResponseModeMismatch);
        };
        let parameters = serialize_error_form(
            code,
            details.description.as_ref().map(|value| value.as_str()),
            details.uri.as_ref().map(|value| value.as_str()),
            state.as_ref().map(|value| value.as_str()),
            error_limits,
        )?;
        build_response(
            mode,
            RedirectResponseKind::Error(code),
            destination,
            parameters,
            state.is_some(),
            redirect_limits,
        )
    }
}

impl RedirectResponse {
    /// Exact response mode selected by the signed request.
    pub const fn mode(&self) -> AuthorizationResponseMode {
        self.mode
    }

    /// Whether this is success or an explicitly selected protocol error.
    pub const fn kind(&self) -> RedirectResponseKind {
        self.kind
    }

    /// Whether exact request state is present in the output.
    pub const fn state_present(&self) -> bool {
        self.state_present
    }

    /// Explicitly reveal the complete redirect URI or the form-post target.
    ///
    /// The returned value is inert and does not authorize navigation.
    pub fn expose_sensitive_target(&self) -> &str {
        &self.target
    }

    /// Explicitly reveal form-post body material when that mode was selected.
    pub fn expose_sensitive_form_body(&self) -> Option<&str> {
        self.form_body.as_ref().map(|value| value.as_str())
    }
}

impl fmt::Debug for RedirectResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RedirectResponse")
            .field("mode", &self.mode)
            .field("kind", &self.kind)
            .field("target_bytes", &self.target.len())
            .field(
                "form_body_bytes",
                &self.form_body.as_ref().map(|value| value.len()),
            )
            .field("state_present", &self.state_present)
            .finish_non_exhaustive()
    }
}

fn require_redirect_mode(mode: AuthorizationResponseMode) -> Result<(), Oid4vpError> {
    if matches!(
        mode,
        AuthorizationResponseMode::Fragment
            | AuthorizationResponseMode::Query
            | AuthorizationResponseMode::FormPost
    ) {
        Ok(())
    } else {
        Err(Oid4vpError::ResponseModeMismatch)
    }
}

fn build_response(
    mode: AuthorizationResponseMode,
    kind: RedirectResponseKind,
    destination: Zeroizing<String>,
    parameters: Zeroizing<String>,
    state_present: bool,
    limits: RedirectResponseLimits,
) -> Result<RedirectResponse, Oid4vpError> {
    ensure_no_destination_collision(&destination, limits)?;
    match mode {
        AuthorizationResponseMode::FormPost => Ok(RedirectResponse {
            mode,
            kind,
            target: destination,
            form_body: Some(parameters),
            state_present,
        }),
        AuthorizationResponseMode::Query | AuthorizationResponseMode::Fragment => {
            let output = append_uri_component(&destination, &parameters, mode, limits)?;
            Ok(RedirectResponse {
                mode,
                kind,
                target: output,
                form_body: None,
                state_present,
            })
        }
        _ => Err(Oid4vpError::ResponseModeMismatch),
    }
}

fn append_uri_component(
    destination: &str,
    parameters: &str,
    mode: AuthorizationResponseMode,
    limits: RedirectResponseLimits,
) -> Result<Zeroizing<String>, Oid4vpError> {
    let (without_fragment, fragment) = destination
        .split_once('#')
        .map_or((destination, None), |(base, value)| (base, Some(value)));
    let (without_query, query) = without_fragment
        .split_once('?')
        .map_or((without_fragment, None), |(base, value)| {
            (base, Some(value))
        });
    let existing = match mode {
        AuthorizationResponseMode::Query => query,
        AuthorizationResponseMode::Fragment => fragment,
        _ => return Err(Oid4vpError::ResponseModeMismatch),
    };
    let separator = if existing.is_some_and(|value| !value.is_empty()) {
        "&"
    } else {
        ""
    };
    let mut output = Zeroizing::new(String::new());
    match mode {
        AuthorizationResponseMode::Query => {
            output.push_str(without_query);
            output.push('?');
            if let Some(value) = query {
                output.push_str(value);
            }
            output.push_str(separator);
            output.push_str(parameters);
            if let Some(value) = fragment {
                output.push('#');
                output.push_str(value);
            }
        }
        AuthorizationResponseMode::Fragment => {
            output.push_str(without_fragment);
            output.push('#');
            if let Some(value) = fragment {
                output.push_str(value);
            }
            output.push_str(separator);
            output.push_str(parameters);
        }
        _ => return Err(Oid4vpError::ResponseModeMismatch),
    }
    if output.len() > limits.max_output_uri_bytes() {
        return Err(Oid4vpError::ResponseUriTooLarge);
    }
    Ok(output)
}

fn ensure_no_destination_collision(
    destination: &str,
    limits: RedirectResponseLimits,
) -> Result<(), Oid4vpError> {
    let (without_fragment, fragment) = destination
        .split_once('#')
        .map_or((destination, None), |(base, value)| (base, Some(value)));
    let query = without_fragment.split_once('?').map(|(_, value)| value);
    if query.is_some_and(|value| reserved_collision(value, limits))
        || fragment.is_some_and(|value| reserved_collision(value, limits))
    {
        Err(Oid4vpError::ConflictingResponseDestination)
    } else {
        Ok(())
    }
}

fn reserved_collision(component: &str, limits: RedirectResponseLimits) -> bool {
    let mut count = 0usize;
    for pair in component.split('&') {
        count += 1;
        if count > limits.max_existing_parameters() {
            return true;
        }
        let name = pair.split_once('=').map_or(pair, |(name, _)| name);
        let Some(name) = percent_decode_name(name) else {
            return true;
        };
        if matches!(
            name.as_str(),
            "vp_token" | "state" | "error" | "error_description" | "error_uri"
        ) {
            return true;
        }
    }
    false
}

fn percent_decode_name(value: &str) -> Option<String> {
    let bytes = value.as_bytes();
    let mut output = Vec::with_capacity(bytes.len());
    let mut index = 0usize;
    while index < bytes.len() {
        match bytes[index] {
            b'%' if index + 2 < bytes.len() => {
                let high = hex(bytes[index + 1])?;
                let low = hex(bytes[index + 2])?;
                output.push((high << 4) | low);
                index += 3;
            }
            b'%' => return None,
            b'+' => {
                output.push(b' ');
                index += 1;
            }
            byte => {
                output.push(byte);
                index += 1;
            }
        }
    }
    String::from_utf8(output).ok()
}

const fn hex(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}
