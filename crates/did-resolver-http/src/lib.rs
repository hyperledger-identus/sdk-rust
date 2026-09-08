//! Bounded Axum binding for the W3C DID Resolution HTTP interface.
//!
//! This outer-boundary crate maps one fixed `GET /{did}` route to an injected
//! [`identus_did::DidResolver`]. It owns HTTP content negotiation, bounded
//! resolution-option decoding, status and response projection only. DID method
//! behavior, server execution, TLS,
//! middleware and deployment policy remain with the consumer.

use std::{
    collections::{BTreeMap, BTreeSet},
    str::FromStr,
    sync::Arc,
};

use axum::{
    Router,
    extract::{Path, RawQuery, State, rejection::PathRejection},
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
    routing::get,
};
use headers_accept::Accept;
use identus_core::Component;
use identus_did::{
    Did, DidResolutionDateTime, DidResolutionError, DidResolutionErrorKind, DidResolutionMetadata,
    DidResolutionResult, DidResolver, MediaType as DidMediaType, ResolutionOptions, VersionId,
};
use mediatype::{MediaType, Name, ReadParams, names};

/// W3C media type for the complete DID Resolution result envelope.
pub const APPLICATION_DID_RESOLUTION: &str = "application/did-resolution";
/// DID document media type used as the default representation.
pub const APPLICATION_DID: &str = "application/did";
/// Generic JSON DID document representation supported by this binding.
pub const APPLICATION_JSON: &str = "application/json";
/// Maximum aggregate bytes accepted across all `Accept` field values.
pub const MAX_ACCEPT_HEADER_BYTES: usize = 8 * 1_024;
/// Maximum media ranges accepted across all `Accept` field values.
pub const MAX_ACCEPT_MEDIA_RANGES: usize = 32;
/// Maximum raw bytes accepted in the resolution-options query.
pub const MAX_RESOLUTION_QUERY_BYTES: usize = 8 * 1_024;
/// Maximum parameters accepted in the resolution-options query.
pub const MAX_RESOLUTION_QUERY_PARAMETERS: usize = 32;
/// Maximum bytes accepted in one decoded resolution-option name.
pub const MAX_RESOLUTION_QUERY_NAME_BYTES: usize = 256;
/// Maximum bytes accepted in one decoded resolution-option value.
pub const MAX_RESOLUTION_QUERY_VALUE_BYTES: usize = 4 * 1_024;

const APPLICATION: Name<'static> = names::APPLICATION;
const DID: Name<'static> = Name::new_unchecked("did");
const DID_RESOLUTION: Name<'static> = Name::new_unchecked("did-resolution");
const APPLICATION_DID_MEDIA_TYPE: MediaType<'static> = MediaType::new(APPLICATION, DID);
const APPLICATION_JSON_MEDIA_TYPE: MediaType<'static> = MediaType::new(APPLICATION, names::JSON);
const APPLICATION_DID_RESOLUTION_MEDIA_TYPE: MediaType<'static> =
    MediaType::new(APPLICATION, DID_RESOLUTION);
const AVAILABLE_MEDIA_TYPES: &[MediaType<'static>] = &[
    APPLICATION_DID_MEDIA_TYPE,
    APPLICATION_JSON_MEDIA_TYPE,
    APPLICATION_DID_RESOLUTION_MEDIA_TYPE,
];

const INTERNAL_ERROR_BODY: &str = r#"{"didResolutionMetadata":{"error":{"type":"https://www.w3.org/ns/did#INTERNAL_ERROR"}},"didDocument":null,"didDocumentMetadata":{}}"#;

/// Metadata for the `identus-did-resolver-http` crate.
pub const COMPONENT: Component = Component {
    name: "identus-did-resolver-http",
    summary: "Bounded Axum binding for the W3C DID Resolution HTTP interface.",
};

#[derive(Clone)]
struct ResolverState {
    resolver: Arc<dyn DidResolver>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Representation {
    Document(&'static str),
    ResolutionResult,
}

/// Build a state-closed router containing the fixed `GET /{did}` route.
///
/// Consumers choose the externally visible endpoint by nesting this router at
/// a prefix. The returned router does not start a server or install deployment
/// middleware.
pub fn did_resolver_http_router(resolver: Arc<dyn DidResolver>) -> Router {
    Router::new()
        .route(
            "/{did}",
            get(resolve_did).fallback(method_not_allowed_response),
        )
        .fallback(route_not_found_response)
        .with_state(ResolverState { resolver })
}

async fn method_not_allowed_response() -> Response {
    empty_response(StatusCode::METHOD_NOT_ALLOWED)
}

async fn route_not_found_response() -> Response {
    empty_response(StatusCode::NOT_FOUND)
}

async fn resolve_did(
    State(state): State<ResolverState>,
    raw_query: RawQuery,
    path: Result<Path<String>, PathRejection>,
    headers: HeaderMap,
) -> Response {
    let Ok(Path(value)) = path else {
        return standard_error_response(DidResolutionErrorKind::InvalidDid);
    };
    let Ok(did) = Did::parse(&value) else {
        return standard_error_response(DidResolutionErrorKind::InvalidDid);
    };
    let representation = match negotiate(&headers) {
        Ok(value) => value,
        Err(kind) => return standard_error_response(kind),
    };
    let options = match decode_resolution_options(raw_query.0.as_deref(), representation) {
        Ok(value) => value,
        Err(kind) => return standard_error_response(kind),
    };

    let result = state.resolver.resolve(&did, &options).await;
    if result.validate_for(&did).is_err() {
        return standard_error_response(DidResolutionErrorKind::InternalError);
    }
    project_result(&result, representation)
}

fn decode_resolution_options(
    raw_query: Option<&str>,
    representation: Representation,
) -> Result<ResolutionOptions, DidResolutionErrorKind> {
    let accept = match representation {
        Representation::ResolutionResult => None,
        Representation::Document(media_type) => Some(
            DidMediaType::parse(media_type).map_err(|_| DidResolutionErrorKind::InternalError)?,
        ),
    };

    let Some(query) = raw_query.filter(|query| !query.is_empty()) else {
        return ResolutionOptions::new(accept, None, None, None, None, BTreeMap::new())
            .map_err(|_| DidResolutionErrorKind::InternalError);
    };
    if query.len() > MAX_RESOLUTION_QUERY_BYTES {
        return Err(DidResolutionErrorKind::InvalidOptions);
    }

    let mut names = BTreeSet::new();
    let mut expand_relative_urls = None;
    let mut no_cache = None;
    let mut version_id = None;
    let mut version_time = None;
    let mut extensions = BTreeMap::new();

    for (index, parameter) in query.split('&').enumerate() {
        if index >= MAX_RESOLUTION_QUERY_PARAMETERS {
            return Err(DidResolutionErrorKind::InvalidOptions);
        }
        let (raw_name, raw_value) = parameter
            .split_once('=')
            .ok_or(DidResolutionErrorKind::InvalidOptions)?;
        let name = percent_decode_query_component(raw_name, MAX_RESOLUTION_QUERY_NAME_BYTES)?;
        let value = percent_decode_query_component(raw_value, MAX_RESOLUTION_QUERY_VALUE_BYTES)?;
        if name.is_empty()
            || name.chars().any(char::is_control)
            || value.chars().any(char::is_control)
            || !names.insert(name.clone())
        {
            return Err(DidResolutionErrorKind::InvalidOptions);
        }

        match name.as_str() {
            "accept" => return Err(DidResolutionErrorKind::InvalidOptions),
            "expandRelativeUrls" => expand_relative_urls = Some(parse_query_bool(&value)?),
            "noCache" => no_cache = Some(parse_query_bool(&value)?),
            "versionId" => {
                version_id = Some(
                    VersionId::try_new(value)
                        .map_err(|_| DidResolutionErrorKind::InvalidOptions)?,
                );
            }
            "versionTime" => {
                version_time = Some(
                    DidResolutionDateTime::try_new(value)
                        .map_err(|_| DidResolutionErrorKind::InvalidOptions)?,
                );
            }
            _ => {
                extensions.insert(name, serde_json::Value::String(value));
            }
        }
    }

    if version_id.is_some() && version_time.is_some() {
        return Err(DidResolutionErrorKind::InvalidOptions);
    }
    ResolutionOptions::new(
        accept,
        expand_relative_urls,
        no_cache,
        version_id,
        version_time,
        extensions,
    )
    .map_err(|_| DidResolutionErrorKind::InvalidOptions)
}

fn percent_decode_query_component(
    value: &str,
    max_decoded_bytes: usize,
) -> Result<String, DidResolutionErrorKind> {
    let bytes = value.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len().min(max_decoded_bytes));
    let mut index = 0;
    while index < bytes.len() {
        let byte = if bytes[index] == b'%' {
            let high = bytes
                .get(index + 1)
                .and_then(|byte| hex_value(*byte))
                .ok_or(DidResolutionErrorKind::InvalidOptions)?;
            let low = bytes
                .get(index + 2)
                .and_then(|byte| hex_value(*byte))
                .ok_or(DidResolutionErrorKind::InvalidOptions)?;
            index += 3;
            (high << 4) | low
        } else {
            let byte = bytes[index];
            index += 1;
            byte
        };
        if decoded.len() >= max_decoded_bytes {
            return Err(DidResolutionErrorKind::InvalidOptions);
        }
        decoded.push(byte);
    }
    String::from_utf8(decoded).map_err(|_| DidResolutionErrorKind::InvalidOptions)
}

const fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn parse_query_bool(value: &str) -> Result<bool, DidResolutionErrorKind> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(DidResolutionErrorKind::InvalidOptions),
    }
}

fn negotiate(headers: &HeaderMap) -> Result<Representation, DidResolutionErrorKind> {
    let values: Vec<&HeaderValue> = headers.get_all(header::ACCEPT).iter().collect();
    if values.is_empty() {
        return Ok(Representation::Document(APPLICATION_DID));
    }

    let mut combined = String::new();
    for (index, value) in values.into_iter().enumerate() {
        let value = value
            .to_str()
            .map_err(|_| DidResolutionErrorKind::InvalidOptions)?;
        if !value
            .bytes()
            .all(|byte| byte == b'\t' || (0x20..=0x7e).contains(&byte))
        {
            return Err(DidResolutionErrorKind::InvalidOptions);
        }
        let separator_bytes = usize::from(index != 0);
        let new_len = combined
            .len()
            .checked_add(separator_bytes)
            .and_then(|length| length.checked_add(value.len()))
            .ok_or(DidResolutionErrorKind::InvalidOptions)?;
        if new_len > MAX_ACCEPT_HEADER_BYTES {
            return Err(DidResolutionErrorKind::InvalidOptions);
        }
        if index != 0 {
            combined.push(',');
        }
        combined.push_str(value);
    }

    validate_media_range_list(&combined)?;
    let accept = Accept::from_str(&combined).map_err(|_| DidResolutionErrorKind::InvalidOptions)?;
    validate_quality_values(&accept)?;

    let selected = accept
        .negotiate(AVAILABLE_MEDIA_TYPES)
        .ok_or(DidResolutionErrorKind::RepresentationNotSupported)?;
    if selected == &APPLICATION_DID_RESOLUTION_MEDIA_TYPE {
        Ok(Representation::ResolutionResult)
    } else if selected == &APPLICATION_JSON_MEDIA_TYPE {
        Ok(Representation::Document(APPLICATION_JSON))
    } else {
        Ok(Representation::Document(APPLICATION_DID))
    }
}

fn validate_media_range_list(value: &str) -> Result<(), DidResolutionErrorKind> {
    let mut in_quotes = false;
    let mut escaped = false;
    let mut start = 0;
    let mut ranges = 0usize;

    for (index, byte) in value.bytes().enumerate() {
        if escaped {
            escaped = false;
            continue;
        }
        match byte {
            b'\\' if in_quotes => escaped = true,
            b'"' => in_quotes = !in_quotes,
            b',' if !in_quotes => {
                validate_media_range_segment(&value[start..index])?;
                ranges += 1;
                if ranges >= MAX_ACCEPT_MEDIA_RANGES {
                    return Err(DidResolutionErrorKind::InvalidOptions);
                }
                start = index + 1;
            }
            _ => {}
        }
    }

    if in_quotes || escaped {
        return Err(DidResolutionErrorKind::InvalidOptions);
    }
    validate_media_range_segment(&value[start..])?;
    ranges += 1;
    if ranges > MAX_ACCEPT_MEDIA_RANGES {
        return Err(DidResolutionErrorKind::InvalidOptions);
    }
    Ok(())
}

fn validate_media_range_segment(value: &str) -> Result<(), DidResolutionErrorKind> {
    if value.trim_matches([' ', '\t']).is_empty() {
        Err(DidResolutionErrorKind::InvalidOptions)
    } else {
        Ok(())
    }
}

fn validate_quality_values(accept: &Accept) -> Result<(), DidResolutionErrorKind> {
    for media_type in accept.media_types() {
        let mut quality_seen = false;
        for (name, value) in media_type.params() {
            if name == names::Q {
                if quality_seen || !valid_quality(value.as_str()) {
                    return Err(DidResolutionErrorKind::InvalidOptions);
                }
                quality_seen = true;
            }
        }
    }
    Ok(())
}

fn valid_quality(value: &str) -> bool {
    match value.as_bytes() {
        b"0" | b"1" => true,
        [b'0', b'.', digits @ ..] => digits.len() <= 3 && digits.iter().all(u8::is_ascii_digit),
        [b'1', b'.', zeros @ ..] => zeros.len() <= 3 && zeros.iter().all(|digit| *digit == b'0'),
        _ => false,
    }
}

fn project_result(result: &DidResolutionResult, representation: Representation) -> Response {
    if result.document_metadata().deactivated() == Some(true) {
        return result_response(result, StatusCode::GONE, APPLICATION_DID_RESOLUTION);
    }
    if let Some(error) = result.metadata().error() {
        let status = error
            .kind()
            .map_or(StatusCode::INTERNAL_SERVER_ERROR, error_status);
        return result_response(result, status, APPLICATION_DID_RESOLUTION);
    }

    match representation {
        Representation::ResolutionResult => {
            result_response(result, StatusCode::OK, APPLICATION_DID_RESOLUTION)
        }
        Representation::Document(expected) => {
            let Some(document) = result.document() else {
                return standard_error_response(DidResolutionErrorKind::InternalError);
            };
            let Some(content_type) = result.metadata().content_type() else {
                return standard_error_response(DidResolutionErrorKind::InternalError);
            };
            if !content_type.as_str().eq_ignore_ascii_case(expected) {
                return standard_error_response(DidResolutionErrorKind::InternalError);
            }
            json_response(document, StatusCode::OK, expected)
        }
    }
}

fn error_status(kind: DidResolutionErrorKind) -> StatusCode {
    match kind {
        DidResolutionErrorKind::InvalidDid
        | DidResolutionErrorKind::InvalidDidUrl
        | DidResolutionErrorKind::InvalidOptions => StatusCode::BAD_REQUEST,
        DidResolutionErrorKind::NotFound => StatusCode::NOT_FOUND,
        DidResolutionErrorKind::RepresentationNotSupported => StatusCode::NOT_ACCEPTABLE,
        DidResolutionErrorKind::MethodNotSupported
        | DidResolutionErrorKind::FeatureNotSupported => StatusCode::NOT_IMPLEMENTED,
        DidResolutionErrorKind::InvalidDidDocument | DidResolutionErrorKind::InternalError => {
            StatusCode::INTERNAL_SERVER_ERROR
        }
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

fn standard_error_response(kind: DidResolutionErrorKind) -> Response {
    let metadata = DidResolutionMetadata::new(
        None,
        Some(DidResolutionError::standard(kind)),
        BTreeMap::new(),
    );
    let result = metadata.and_then(DidResolutionResult::failure);
    match result {
        Ok(result) => result_response(&result, error_status(kind), APPLICATION_DID_RESOLUTION),
        Err(_) => static_internal_error_response(),
    }
}

fn result_response(
    result: &DidResolutionResult,
    status: StatusCode,
    content_type: &'static str,
) -> Response {
    json_response(result, status, content_type)
}

fn json_response(
    value: &impl serde::Serialize,
    status: StatusCode,
    content_type: &str,
) -> Response {
    match serde_json::to_vec(value) {
        Ok(body) => (
            status,
            [
                (header::CONTENT_TYPE, content_type),
                (header::VARY, header::ACCEPT.as_str()),
            ],
            body,
        )
            .into_response(),
        Err(_) => static_internal_error_response(),
    }
}

fn static_internal_error_response() -> Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        [
            (header::CONTENT_TYPE, APPLICATION_DID_RESOLUTION),
            (header::VARY, header::ACCEPT.as_str()),
        ],
        INTERNAL_ERROR_BODY,
    )
        .into_response()
}

fn empty_response(status: StatusCode) -> Response {
    (status, [(header::VARY, header::ACCEPT.as_str())]).into_response()
}

#[cfg(test)]
mod tests;
