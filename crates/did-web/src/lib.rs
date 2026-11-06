//! Portable, bounded `did:web` resolution over a caller-injected fetch port.
//!
//! This crate owns strict method syntax, deterministic HTTPS request derivation,
//! response-envelope validation, and exact DID document correlation. Callers own
//! DNS, TLS, redirects, proxies, timeouts, cancellation, retries, caching,
//! privacy, telemetry, and SSRF policy.

#![forbid(unsafe_code)]

use std::{collections::BTreeMap, fmt, future::Future, pin::Pin, sync::Arc};

use identus_core::Component;
use identus_derive as identus;
use identus_did::{
    ContextEntry, Did, DidDocument, DidDocumentMetadata, DidMethod, DidMethodBinding,
    DidResolutionError, DidResolutionErrorKind, DidResolutionFuture, DidResolutionMetadata,
    DidResolutionResult, DidResolver, MAX_DID_DOCUMENT_BYTES, MediaType, ResolutionOptions,
};

const DID_PREFIX: &str = "did:web:";
const DID_JSON: &str = "application/did+json";
const DID_LD_JSON: &str = "application/did+ld+json";
const DID_CORE_CONTEXT: &str = "https://www.w3.org/ns/did/v1";
const JSON: &str = "application/json";
const DEFAULT_MAX_RESPONSE_BODY_BYTES: usize = 128 * 1_024;

/// Metadata for the `identus-did-web` crate.
pub const COMPONENT: Component = Component {
    name: "identus-did-web",
    summary: "Portable bounded did:web resolution over an injected fetch port.",
};

/// Finite work ceilings for parsing and remote response binding.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DidWebLimits {
    max_identifier_bytes: usize,
    max_path_segments: usize,
    max_path_segment_bytes: usize,
    max_url_bytes: usize,
    max_content_type_bytes: usize,
    max_response_body_bytes: usize,
}

impl DidWebLimits {
    /// Construct a strictly positive limit set.
    pub const fn new(
        max_identifier_bytes: usize,
        max_path_segments: usize,
        max_path_segment_bytes: usize,
        max_url_bytes: usize,
        max_content_type_bytes: usize,
        max_response_body_bytes: usize,
    ) -> Result<Self, Error> {
        if max_identifier_bytes == 0
            || max_path_segments == 0
            || max_path_segment_bytes == 0
            || max_url_bytes == 0
            || max_content_type_bytes == 0
            || max_response_body_bytes == 0
            || max_response_body_bytes > MAX_DID_DOCUMENT_BYTES
        {
            return Err(Error::InvalidLimits);
        }
        Ok(Self {
            max_identifier_bytes,
            max_path_segments,
            max_path_segment_bytes,
            max_url_bytes,
            max_content_type_bytes,
            max_response_body_bytes,
        })
    }

    /// Maximum accepted response body size.
    #[must_use]
    pub const fn max_response_body_bytes(self) -> usize {
        self.max_response_body_bytes
    }
}

impl Default for DidWebLimits {
    fn default() -> Self {
        Self {
            max_identifier_bytes: 2_048,
            max_path_segments: 32,
            max_path_segment_bytes: 128,
            max_url_bytes: 4_096,
            max_content_type_bytes: 128,
            max_response_body_bytes: DEFAULT_MAX_RESPONSE_BODY_BYTES,
        }
    }
}

/// Stable, content-free method failures.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// A limit set contains zero or exceeds the generic document envelope.
    InvalidLimits,
    /// The DID method is not `web`.
    WrongMethod,
    /// The method-specific identifier is outside the strict first profile.
    InvalidIdentifier,
    /// A finite work ceiling was exceeded.
    ResourceLimit,
    /// The injected transport could not obtain a response.
    Transport,
    /// The remote status is not successful.
    HttpStatus,
    /// The response media type is missing, malformed, or unsupported.
    InvalidMediaType,
    /// The response body is empty or not a valid bounded DID document.
    InvalidDocument,
    /// The returned document identifies a different DID.
    DocumentIdMismatch,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidLimits => "did:web invalid limits",
            Self::WrongMethod => "did:web method mismatch",
            Self::InvalidIdentifier => "did:web invalid identifier",
            Self::ResourceLimit => "did:web resource limit exceeded",
            Self::Transport => "did:web transport failure",
            Self::HttpStatus => "did:web unsuccessful response status",
            Self::InvalidMediaType => "did:web invalid response media type",
            Self::InvalidDocument => "did:web invalid document",
            Self::DocumentIdMismatch => "did:web document identifier mismatch",
        })
    }
}

impl std::error::Error for Error {}

/// Content-free failure returned by a caller-owned transport adapter.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TransportFailure;

impl fmt::Display for TransportFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("did:web transport failure")
    }
}

impl std::error::Error for TransportFailure {}

/// A validated `did:web` identifier and its deterministic HTTPS request URL.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DidWeb {
    did: Did,
    url: String,
    limits: DidWebLimits,
}

impl DidWeb {
    /// Parse with the conservative default limits.
    pub fn parse(input: &str) -> Result<Self, Error> {
        Self::parse_with_limits(input, DidWebLimits::default())
    }

    /// Parse with caller-selected finite limits.
    pub fn parse_with_limits(input: &str, limits: DidWebLimits) -> Result<Self, Error> {
        if input.len() > limits.max_identifier_bytes {
            return Err(Error::ResourceLimit);
        }
        let specific = input.strip_prefix(DID_PREFIX).ok_or(Error::WrongMethod)?;
        let url = derive_url(specific, limits)?;
        let did = Did::parse(input).map_err(|_| Error::InvalidIdentifier)?;
        Ok(Self { did, url, limits })
    }

    /// Validate an already parsed generic DID.
    pub fn from_did(did: Did, limits: DidWebLimits) -> Result<Self, Error> {
        if did.method() != "web" {
            return Err(Error::WrongMethod);
        }
        if did.as_str().len() > limits.max_identifier_bytes {
            return Err(Error::ResourceLimit);
        }
        let url = derive_url(did.method_specific_id(), limits)?;
        Ok(Self { did, url, limits })
    }

    /// Borrow the exact validated DID.
    #[must_use]
    pub const fn did(&self) -> &Did {
        &self.did
    }

    /// Borrow the exact deterministic HTTPS request URL.
    #[must_use]
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Prepare the least-authority transport request.
    #[must_use]
    pub fn request(&self) -> DidWebRequest {
        DidWebRequest {
            url: self.url.clone(),
            limits: self.limits,
            representation: RequestedRepresentation::DidJson,
            no_cache: false,
        }
    }

    fn request_for(&self, selected: &str, no_cache: bool) -> DidWebRequest {
        DidWebRequest {
            url: self.url.clone(),
            limits: self.limits,
            representation: if selected == DID_LD_JSON {
                RequestedRepresentation::DidLdJson
            } else {
                RequestedRepresentation::DidJson
            },
            no_cache,
        }
    }

    /// Validate a response and bind its DID document to this identifier.
    pub fn bind_response(&self, response: DidWebResponse) -> Result<DidDocument, Error> {
        if response.status != 200 {
            return Err(Error::HttpStatus);
        }
        validate_media_type(response.content_type.as_deref(), self.limits)?;
        if response.body.is_empty() {
            return Err(Error::InvalidDocument);
        }
        if response.body.len() > self.limits.max_response_body_bytes {
            return Err(Error::ResourceLimit);
        }
        let document =
            DidDocument::from_json_slice(&response.body).map_err(|_| Error::InvalidDocument)?;
        if document.id() != &self.did {
            return Err(Error::DocumentIdMismatch);
        }
        Ok(document)
    }
}

/// Immutable caller-executed GET request data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DidWebRequest {
    url: String,
    limits: DidWebLimits,
    representation: RequestedRepresentation,
    no_cache: bool,
}

impl DidWebRequest {
    /// Borrow the exact HTTPS URL. Adapters MUST NOT redirect unless their own
    /// policy explicitly authorizes it.
    #[must_use]
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Media types accepted by the portable core.
    #[must_use]
    pub const fn accept(&self) -> &'static str {
        match self.representation {
            RequestedRepresentation::DidJson => "application/did+json, application/json",
            RequestedRepresentation::DidLdJson => "application/did+ld+json",
        }
    }

    /// Whether the caller requested that runtime-owned caches be bypassed.
    #[must_use]
    pub const fn no_cache(&self) -> bool {
        self.no_cache
    }

    /// Maximum body bytes the adapter may retain.
    #[must_use]
    pub const fn max_response_body_bytes(&self) -> usize {
        self.limits.max_response_body_bytes
    }

    /// Bind raw adapter output to this request's public limits.
    pub fn response(
        &self,
        status: u16,
        content_type: Option<&str>,
        body: Vec<u8>,
    ) -> Result<DidWebResponse, Error> {
        if content_type.is_some_and(|value| value.len() > self.limits.max_content_type_bytes)
            || body.len() > self.limits.max_response_body_bytes
        {
            return Err(Error::ResourceLimit);
        }
        Ok(DidWebResponse {
            status,
            content_type: content_type.map(str::to_owned),
            body,
        })
    }
}

/// Owned bounded response returned by the injected fetch port.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DidWebResponse {
    status: u16,
    content_type: Option<String>,
    body: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RequestedRepresentation {
    DidJson,
    DidLdJson,
}

/// Type-erased future returned by [`DidWebFetcher`].
pub type DidWebFetchFuture<'a> =
    Pin<Box<dyn Future<Output = Result<DidWebResponse, TransportFailure>> + Send + 'a>>;

/// Runtime-neutral, caller-implemented retrieval port.
#[identus::port]
pub trait DidWebFetcher: Send + Sync {
    /// Execute exactly the prepared GET under caller-owned network policy.
    fn fetch<'a>(&'a self, request: &'a DidWebRequest) -> DidWebFetchFuture<'a>;
}

/// Generic DID resolver backed by one caller-owned fetch adapter.
#[derive(Clone, Debug)]
pub struct DidWebResolver<F> {
    fetcher: F,
    limits: DidWebLimits,
}

impl<F> DidWebResolver<F> {
    /// Construct a resolver with explicit finite limits.
    pub const fn new(fetcher: F, limits: DidWebLimits) -> Self {
        Self { fetcher, limits }
    }
}

impl<F> DidWebResolver<F>
where
    F: DidWebFetcher + 'static,
{
    /// Consume this resolver into an explicit `web` method binding.
    #[must_use]
    pub fn into_method_binding(self) -> DidMethodBinding {
        DidMethodBinding::new(
            DidMethod::parse("web").expect("fixed method name is valid"),
            Arc::new(self),
        )
    }
}

impl<F: DidWebFetcher> DidResolver for DidWebResolver<F> {
    fn resolve<'a>(
        &'a self,
        did: &'a Did,
        options: &'a ResolutionOptions,
    ) -> DidResolutionFuture<'a> {
        Box::pin(async move {
            if options.version_id().is_some()
                || options.version_time().is_some()
                || options.expand_relative_urls() == Some(true)
                || !options.extensions().is_empty()
            {
                return resolution_failure(DidResolutionErrorKind::FeatureNotSupported);
            }
            let selected = match options.accept().map(MediaType::as_str) {
                None => DID_JSON,
                Some(value) if value.eq_ignore_ascii_case(DID_JSON) => DID_JSON,
                Some(value) if value.eq_ignore_ascii_case(DID_LD_JSON) => DID_LD_JSON,
                Some(_) => {
                    return resolution_failure(DidResolutionErrorKind::RepresentationNotSupported);
                }
            };
            let web = match DidWeb::from_did(did.clone(), self.limits) {
                Ok(value) => value,
                Err(Error::WrongMethod) => {
                    return resolution_failure(DidResolutionErrorKind::MethodNotSupported);
                }
                Err(_) => return resolution_failure(DidResolutionErrorKind::InvalidDid),
            };
            let request = web.request_for(selected, options.no_cache() == Some(true));
            let response = match self.fetcher.fetch(&request).await {
                Ok(value) => value,
                Err(_) => return resolution_failure(DidResolutionErrorKind::InternalError),
            };
            if response.status == 404 {
                return resolution_failure(DidResolutionErrorKind::NotFound);
            }
            if response.status != 200 {
                return resolution_failure(DidResolutionErrorKind::InternalError);
            }
            let representation =
                match parse_media_type(response.content_type.as_deref(), self.limits) {
                    Ok(value) => value,
                    Err(Error::InvalidMediaType) => {
                        return resolution_failure(
                            DidResolutionErrorKind::RepresentationNotSupported,
                        );
                    }
                    Err(_) => {
                        return resolution_failure(DidResolutionErrorKind::InvalidDidDocument);
                    }
                };
            if !representation.satisfies(selected) {
                return resolution_failure(DidResolutionErrorKind::RepresentationNotSupported);
            }
            let document = match web.bind_response(response) {
                Ok(value) => value,
                Err(Error::InvalidMediaType) => {
                    return resolution_failure(DidResolutionErrorKind::RepresentationNotSupported);
                }
                Err(_) => return resolution_failure(DidResolutionErrorKind::InvalidDidDocument),
            };
            if selected == DID_LD_JSON && !has_required_did_context(&document) {
                return resolution_failure(DidResolutionErrorKind::InvalidDidDocument);
            }
            let metadata = match MediaType::parse(selected).and_then(|media_type| {
                DidResolutionMetadata::new(Some(media_type), None, BTreeMap::new())
            }) {
                Ok(value) => value,
                Err(_) => return resolution_failure(DidResolutionErrorKind::InternalError),
            };
            DidResolutionResult::success(metadata, document, DidDocumentMetadata::empty())
                .unwrap_or_else(|_| resolution_failure(DidResolutionErrorKind::InvalidDidDocument))
        })
    }
}

fn has_required_did_context(document: &DidDocument) -> bool {
    matches!(
        document
            .context()
            .and_then(|context| context.as_slice().first()),
        Some(ContextEntry::Uri(uri)) if uri.as_str() == DID_CORE_CONTEXT
    )
}

fn derive_url(specific: &str, limits: DidWebLimits) -> Result<String, Error> {
    if specific.is_empty() || specific.len() > limits.max_identifier_bytes {
        return Err(if specific.is_empty() {
            Error::InvalidIdentifier
        } else {
            Error::ResourceLimit
        });
    }
    let mut parts = specific.split(':');
    let authority = parts.next().ok_or(Error::InvalidIdentifier)?;
    let (host, port) = parse_authority(authority)?;
    validate_host(host)?;
    let mut url =
        String::with_capacity(limits.max_url_bytes.min(specific.len().saturating_add(40)));
    url.push_str("https://");
    url.push_str(host);
    if let Some(port) = port {
        use std::fmt::Write as _;
        write!(&mut url, ":{port}").map_err(|_| Error::ResourceLimit)?;
    }
    let mut has_segment = false;
    for (index, segment) in parts.enumerate() {
        if index >= limits.max_path_segments || segment.len() > limits.max_path_segment_bytes {
            return Err(Error::ResourceLimit);
        }
        if !valid_segment(segment) {
            return Err(Error::InvalidIdentifier);
        }
        if url
            .len()
            .checked_add(1 + segment.len() + "/did.json".len())
            .is_none_or(|length| length > limits.max_url_bytes)
        {
            return Err(Error::ResourceLimit);
        }
        url.push('/');
        url.push_str(segment);
        has_segment = true;
    }
    if !has_segment {
        url.push_str("/.well-known/did.json");
    } else {
        url.push_str("/did.json");
    }
    if url.len() > limits.max_url_bytes {
        return Err(Error::ResourceLimit);
    }
    Ok(url)
}

fn parse_authority(authority: &str) -> Result<(&str, Option<u16>), Error> {
    let Some(index) = authority.find('%') else {
        return Ok((authority, None));
    };
    if !authority[index..].starts_with("%3A") || authority[index + 3..].contains('%') {
        return Err(Error::InvalidIdentifier);
    }
    let host = &authority[..index];
    let raw = &authority[index + 3..];
    if raw.is_empty()
        || (raw.len() > 1 && raw.starts_with('0'))
        || !raw.bytes().all(|b| b.is_ascii_digit())
    {
        return Err(Error::InvalidIdentifier);
    }
    let port = raw.parse::<u16>().map_err(|_| Error::InvalidIdentifier)?;
    if port == 0 || port == 443 {
        return Err(Error::InvalidIdentifier);
    }
    Ok((host, Some(port)))
}

fn validate_host(host: &str) -> Result<(), Error> {
    if host.is_empty() || host.len() > 253 || !host.contains('.') || host.ends_with('.') {
        return Err(Error::InvalidIdentifier);
    }
    if host.rsplit('.').next().is_some_and(is_ipv4_number) {
        return Err(Error::InvalidIdentifier);
    }
    for label in host.split('.') {
        if label.is_empty()
            || label.len() > 63
            || label.starts_with('-')
            || label.ends_with('-')
            || !label
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        {
            return Err(Error::InvalidIdentifier);
        }
    }
    Ok(())
}

fn is_ipv4_number(label: &str) -> bool {
    !label.is_empty()
        && (label.bytes().all(|byte| byte.is_ascii_digit())
            || label.strip_prefix("0x").is_some_and(|digits| {
                !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_hexdigit())
            }))
}

fn valid_segment(segment: &str) -> bool {
    !segment.is_empty()
        && !matches!(segment, "." | "..")
        && segment.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_uppercase()
                || byte.is_ascii_digit()
                || matches!(byte, b'-' | b'.' | b'_')
        })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Representation {
    DidJson,
    DidLdJson,
    Json,
}

impl Representation {
    fn satisfies(self, selected: &str) -> bool {
        match selected {
            DID_JSON => matches!(self, Self::DidJson | Self::Json),
            DID_LD_JSON => self == Self::DidLdJson,
            _ => false,
        }
    }
}

fn validate_media_type(value: Option<&str>, limits: DidWebLimits) -> Result<(), Error> {
    parse_media_type(value, limits).map(|_| ())
}

fn parse_media_type(value: Option<&str>, limits: DidWebLimits) -> Result<Representation, Error> {
    let value = value.ok_or(Error::InvalidMediaType)?;
    if value.len() > limits.max_content_type_bytes {
        return Err(Error::ResourceLimit);
    }
    if value.is_empty()
        || !value
            .bytes()
            .all(|byte| byte == b'\t' || (0x20..=0x7e).contains(&byte))
    {
        return Err(Error::InvalidMediaType);
    }
    let (base, parameters) = match value.split_once(';') {
        Some((_, "")) => return Err(Error::InvalidMediaType),
        Some(parts) => parts,
        None => (value, ""),
    };
    let representation = if base.trim().eq_ignore_ascii_case(DID_JSON) {
        Representation::DidJson
    } else if base.trim().eq_ignore_ascii_case(DID_LD_JSON) {
        Representation::DidLdJson
    } else if base.trim().eq_ignore_ascii_case(JSON) {
        Representation::Json
    } else {
        return Err(Error::InvalidMediaType);
    };
    validate_parameters(parameters)?;
    Ok(representation)
}

fn validate_parameters(mut input: &str) -> Result<(), Error> {
    while !input.is_empty() {
        input = input.trim_start_matches([' ', '\t']);
        let (name, rest) = take_token(input);
        if name.is_empty() {
            return Err(Error::InvalidMediaType);
        }
        input = rest.trim_start_matches([' ', '\t']);
        input = input.strip_prefix('=').ok_or(Error::InvalidMediaType)?;
        input = input.trim_start_matches([' ', '\t']);
        if let Some(rest) = input.strip_prefix('"') {
            input = take_quoted_value(rest)?;
        } else {
            let (value, rest) = take_token(input);
            if value.is_empty() {
                return Err(Error::InvalidMediaType);
            }
            input = rest;
        }
        input = input.trim_start_matches([' ', '\t']);
        if input.is_empty() {
            return Ok(());
        }
        input = input.strip_prefix(';').ok_or(Error::InvalidMediaType)?;
        if input.trim_start_matches([' ', '\t']).is_empty() {
            return Err(Error::InvalidMediaType);
        }
    }
    Ok(())
}

fn take_token(input: &str) -> (&str, &str) {
    let end = input
        .bytes()
        .position(|byte| !is_token_byte(byte))
        .unwrap_or(input.len());
    input.split_at(end)
}

const fn is_token_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric()
        || matches!(
            byte,
            b'!' | b'#'
                | b'$'
                | b'%'
                | b'&'
                | b'\''
                | b'*'
                | b'+'
                | b'-'
                | b'.'
                | b'^'
                | b'_'
                | b'`'
                | b'|'
                | b'~'
        )
}

fn take_quoted_value(input: &str) -> Result<&str, Error> {
    let bytes = input.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'"' => return Ok(&input[index + 1..]),
            b'\\' => {
                index += 1;
                let escaped = bytes.get(index).copied().ok_or(Error::InvalidMediaType)?;
                if !matches!(escaped, b'\t' | b' ' | 0x21..=0x7e) {
                    return Err(Error::InvalidMediaType);
                }
            }
            b'\t' | b' ' | 0x21 | 0x23..=0x5b | 0x5d..=0x7e => {}
            _ => return Err(Error::InvalidMediaType),
        }
        index += 1;
    }
    Err(Error::InvalidMediaType)
}

fn resolution_failure(kind: DidResolutionErrorKind) -> DidResolutionResult {
    let metadata = DidResolutionMetadata::new(
        None,
        Some(DidResolutionError::standard(kind)),
        BTreeMap::new(),
    )
    .expect("standard did:web resolution error is valid");
    DidResolutionResult::failure(metadata).expect("standard did:web resolution failure is valid")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_bare_path_and_port_forms() {
        for (did, url) in [
            (
                "did:web:example.com",
                "https://example.com/.well-known/did.json",
            ),
            (
                "did:web:example.com:user:alice",
                "https://example.com/user/alice/did.json",
            ),
            (
                "did:web:example.com%3A3000:user",
                "https://example.com:3000/user/did.json",
            ),
        ] {
            assert_eq!(DidWeb::parse(did).unwrap().url(), url);
        }
    }

    #[test]
    fn rejects_ambiguous_authorities_and_paths() {
        for did in [
            "did:web:localhost",
            "did:web:127.0.0.1",
            "did:web:Example.com",
            "did:web:-example.com",
            "did:web:example.com%3a3000",
            "did:web:example.com%3A0443",
            "did:web:example.com%3A443",
            "did:web:example.com:",
            "did:web:example.com:..",
            "did:web:example.com:user%2Falice",
        ] {
            assert!(DidWeb::parse(did).is_err(), "accepted {did}");
        }
    }

    #[test]
    fn errors_are_content_free() {
        assert_eq!(
            Error::DocumentIdMismatch.to_string(),
            "did:web document identifier mismatch"
        );
        assert!(!Error::InvalidDocument.to_string().contains("example.com"));
    }
}
