//! Portable, bounded W3C DID URL dereferencing over an injected resolver.

use std::{collections::BTreeMap, fmt, sync::Arc};

use serde_json::Value;

use crate::{
    DereferencedContent, DereferencingOptions, DidDocument, DidResolutionDateTime,
    DidResolutionError, DidResolutionErrorKind, DidResolver, DidUrl, DidUrlContentMetadata,
    DidUrlDereferencer, DidUrlDereferencingFuture, DidUrlDereferencingMetadata,
    DidUrlDereferencingResult, MAX_DID_URL_BYTES, MediaType, ResolutionOptions, Service,
    ServiceEndpoint, ServiceEndpointValue, Uri, VerificationMethod, VerificationRelationship,
    VersionId,
};

const DID_JSON: &str = "application/did+json";
const DID_LD_JSON: &str = "application/did+ld+json";
const URI_LIST: &str = "text/uri-list";
const INVALID_VERIFICATION_METHOD_URI: &str =
    "https://w3id.org/security#INVALID_VERIFICATION_METHOD";
const INVALID_RELATIONSHIP_URI: &str =
    "https://w3id.org/security#INVALID_RELATIONSHIP_FOR_VERIFICATION_METHOD";
const MAX_DECODE_PASSES: usize = 4;

/// Opt-in chain-neutral implementation of the portable DID URL dereferencing
/// algorithm.
///
/// The adapter resolves a DID exactly once, projects document resources and
/// service endpoint URIs, and performs no endpoint retrieval or ambient I/O.
#[derive(Clone)]
pub struct GenericDidUrlDereferencer {
    resolver: Arc<dyn DidResolver>,
}

impl GenericDidUrlDereferencer {
    /// Wrap an object-safe DID resolver.
    #[must_use]
    pub fn new(resolver: Arc<dyn DidResolver>) -> Self {
        Self { resolver }
    }
}

impl fmt::Debug for GenericDidUrlDereferencer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GenericDidUrlDereferencer")
            .finish_non_exhaustive()
    }
}

impl DidUrlDereferencer for GenericDidUrlDereferencer {
    fn dereference<'a>(
        &'a self,
        did_url: &'a DidUrl,
        options: &'a DereferencingOptions,
    ) -> DidUrlDereferencingFuture<'a> {
        Box::pin(async move {
            let prepared = match PreparedRequest::new(did_url, options) {
                Ok(value) => value,
                Err(failure) => return failure_result(failure),
            };

            let did = did_url.to_did();
            let resolved = self.resolver.resolve(&did, &prepared.resolution).await;
            if let Some(error) = resolved.metadata().error() {
                return error_result(error.clone());
            }
            if resolved.validate_for(&did).is_err() {
                return standard_failure(DidResolutionErrorKind::InvalidDidDocument);
            }
            let Some(document) = resolved.document() else {
                return standard_failure(DidResolutionErrorKind::NotFound);
            };
            let content_metadata =
                match DidUrlContentMetadata::from_document_metadata(resolved.document_metadata()) {
                    Ok(value) => value,
                    Err(_) => return standard_failure(DidResolutionErrorKind::InternalError),
                };

            if !did_url.path().is_empty() || prepared.custom_resource {
                return standard_failure(DidResolutionErrorKind::NotFound);
            }

            if prepared.has_service_selector() {
                return dereference_services(
                    did_url,
                    document,
                    content_metadata,
                    resolved.metadata().content_type(),
                    &prepared,
                );
            }
            if let Some(fragment) = did_url.fragment() {
                return dereference_fragment(document, fragment, content_metadata, options);
            }
            if options.verification_relationship().is_some() {
                return standard_failure(DidResolutionErrorKind::InvalidOptions);
            }
            document_result(
                document,
                content_metadata,
                resolved.metadata().content_type().cloned(),
            )
        })
    }
}

#[derive(Debug)]
struct PreparedRequest {
    resolution: ResolutionOptions,
    service: Option<String>,
    service_type: Option<String>,
    relative_ref: Option<String>,
    custom_resource: bool,
}

impl PreparedRequest {
    fn new(did_url: &DidUrl, options: &DereferencingOptions) -> Result<Self, Failure> {
        let mut accept = options.accept().cloned();
        let mut expand_relative_urls = None;
        let mut no_cache = None;
        let mut version_id = None;
        let mut version_time = None;
        let mut extensions = options.extensions().clone();
        if let Some(relationship) = options.verification_relationship() {
            insert_unique(
                &mut extensions,
                "verificationRelationship".to_owned(),
                Value::String(relationship.as_str().to_owned()),
                Failure::InvalidOptions,
            )?;
        }

        let mut service = None;
        let mut service_type = None;
        let mut relative_ref = None;
        let mut custom_resource = false;

        if let Some(query) = did_url.query() {
            let parameters = parse_parameters(query)?;
            for (name, value) in parameters {
                match name.as_str() {
                    "accept" => {
                        if accept.is_some() {
                            return Err(Failure::InvalidDidUrl);
                        }
                        accept =
                            Some(MediaType::try_new(value).map_err(|_| Failure::InvalidDidUrl)?);
                    }
                    "expandRelativeUrls" => {
                        expand_relative_urls = Some(parse_bool(&value)?);
                    }
                    "noCache" => {
                        no_cache = Some(parse_bool(&value)?);
                    }
                    "versionId" => {
                        version_id =
                            Some(VersionId::try_new(value).map_err(|_| Failure::InvalidDidUrl)?);
                    }
                    "versionTime" => {
                        version_time = Some(
                            DidResolutionDateTime::try_new(value)
                                .map_err(|_| Failure::InvalidDidUrl)?,
                        );
                    }
                    "service" => {
                        let value = non_empty(value)?;
                        insert_unique(
                            &mut extensions,
                            name,
                            Value::String(value.clone()),
                            Failure::InvalidDidUrl,
                        )?;
                        service = Some(value);
                    }
                    "serviceType" => {
                        let value = non_empty(value)?;
                        insert_unique(
                            &mut extensions,
                            name,
                            Value::String(value.clone()),
                            Failure::InvalidDidUrl,
                        )?;
                        service_type = Some(value);
                    }
                    "relativeRef" => {
                        validate_relative_ref(&value)?;
                        insert_unique(
                            &mut extensions,
                            name,
                            Value::String(value.clone()),
                            Failure::InvalidDidUrl,
                        )?;
                        relative_ref = Some(value);
                    }
                    "hl" => insert_unique(
                        &mut extensions,
                        name,
                        Value::String(value),
                        Failure::InvalidDidUrl,
                    )?,
                    _ => {
                        custom_resource = true;
                        insert_unique(
                            &mut extensions,
                            name,
                            Value::String(value),
                            Failure::InvalidDidUrl,
                        )?;
                    }
                }
            }
        }

        let resolution = ResolutionOptions::new(
            accept,
            expand_relative_urls,
            no_cache,
            version_id,
            version_time,
            extensions,
        )
        .map_err(|_| Failure::InvalidOptions)?;

        if relative_ref.is_some() && service.is_none() && service_type.is_none() {
            return Err(Failure::InvalidOptions);
        }
        if options.verification_relationship().is_some()
            && (did_url.fragment().is_none()
                || service.is_some()
                || service_type.is_some()
                || relative_ref.is_some())
        {
            return Err(Failure::InvalidOptions);
        }

        Ok(Self {
            resolution,
            service,
            service_type,
            relative_ref,
            custom_resource,
        })
    }

    fn has_service_selector(&self) -> bool {
        self.service.is_some() || self.service_type.is_some()
    }
}

#[derive(Clone, Copy, Debug)]
enum Failure {
    InvalidDidUrl,
    InvalidOptions,
}

fn parse_parameters(query: &str) -> Result<BTreeMap<String, String>, Failure> {
    let mut parameters = BTreeMap::new();
    for parameter in query.split('&') {
        let (name, value) = parameter.split_once('=').unwrap_or((parameter, ""));
        let name = percent_decode(name)?;
        let value = percent_decode(value)?;
        if name.is_empty() || !is_safe_parameter_text(&name) || !is_safe_parameter_text(&value) {
            return Err(Failure::InvalidDidUrl);
        }
        if parameters.insert(name, value).is_some() {
            return Err(Failure::InvalidDidUrl);
        }
    }
    Ok(parameters)
}

fn percent_decode(value: &str) -> Result<String, Failure> {
    let bytes = value.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            let high = bytes
                .get(index + 1)
                .and_then(|byte| hex_value(*byte))
                .ok_or(Failure::InvalidDidUrl)?;
            let low = bytes
                .get(index + 2)
                .and_then(|byte| hex_value(*byte))
                .ok_or(Failure::InvalidDidUrl)?;
            decoded.push((high << 4) | low);
            index += 3;
        } else {
            decoded.push(bytes[index]);
            index += 1;
        }
    }
    String::from_utf8(decoded).map_err(|_| Failure::InvalidDidUrl)
}

const fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn is_safe_parameter_text(value: &str) -> bool {
    value.len() <= MAX_DID_URL_BYTES && value.chars().all(|character| !character.is_control())
}

fn parse_bool(value: &str) -> Result<bool, Failure> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(Failure::InvalidDidUrl),
    }
}

fn non_empty(value: String) -> Result<String, Failure> {
    if value.is_empty() {
        Err(Failure::InvalidDidUrl)
    } else {
        Ok(value)
    }
}

fn insert_unique(
    map: &mut BTreeMap<String, Value>,
    name: String,
    value: Value,
    failure: Failure,
) -> Result<(), Failure> {
    if map.insert(name, value).is_some() {
        return Err(failure);
    }
    Ok(())
}

fn dereference_fragment(
    document: &DidDocument,
    fragment: &str,
    metadata: DidUrlContentMetadata,
    options: &DereferencingOptions,
) -> DidUrlDereferencingResult {
    let target = format!("{}#{fragment}", document.id());
    if let Some(relationship_name) = options.verification_relationship() {
        let Some(method) = find_verification_method(document, &target) else {
            return custom_failure(INVALID_VERIFICATION_METHOD_URI);
        };
        let Some(relationship) = relationship(document, relationship_name.as_str()) else {
            return custom_failure(INVALID_RELATIONSHIP_URI);
        };
        if !relationship_contains(relationship, &target) {
            return custom_failure(INVALID_RELATIONSHIP_URI);
        }
        return resource_result(
            DereferencedContent::from_verification_method(method),
            metadata,
        );
    }

    if let Some(method) = find_verification_method(document, &target) {
        return resource_result(
            DereferencedContent::from_verification_method(method),
            metadata,
        );
    }
    if let Some(service) = document
        .services()
        .unwrap_or_default()
        .iter()
        .find(|service| service.id().as_str() == target)
    {
        return resource_result(DereferencedContent::from_service(service), metadata);
    }
    standard_failure(DidResolutionErrorKind::NotFound)
}

fn find_verification_method<'a>(
    document: &'a DidDocument,
    target: &str,
) -> Option<&'a VerificationMethod> {
    document
        .verification_methods()
        .unwrap_or_default()
        .iter()
        .find(|method| method.id().as_str() == target)
        .or_else(|| {
            relationships(document)
                .flatten()
                .find_map(|entry| match entry {
                    VerificationRelationship::Embedded(method)
                        if method.id().as_str() == target =>
                    {
                        Some(method)
                    }
                    VerificationRelationship::Reference(_)
                    | VerificationRelationship::Embedded(_) => None,
                })
        })
}

fn relationships(
    document: &DidDocument,
) -> impl Iterator<Item = std::slice::Iter<'_, VerificationRelationship>> {
    [
        document.authentication(),
        document.assertion_method(),
        document.key_agreement(),
        document.capability_invocation(),
        document.capability_delegation(),
    ]
    .into_iter()
    .flatten()
    .map(<[VerificationRelationship]>::iter)
}

fn relationship<'a>(
    document: &'a DidDocument,
    name: &str,
) -> Option<&'a [VerificationRelationship]> {
    match name {
        "authentication" => document.authentication(),
        "assertionMethod" => document.assertion_method(),
        "keyAgreement" => document.key_agreement(),
        "capabilityInvocation" => document.capability_invocation(),
        "capabilityDelegation" => document.capability_delegation(),
        _ => None,
    }
}

fn relationship_contains(values: &[VerificationRelationship], target: &str) -> bool {
    values.iter().any(|value| match value {
        VerificationRelationship::Reference(uri) => uri.as_str() == target,
        VerificationRelationship::Embedded(method) => method.id().as_str() == target,
    })
}

fn dereference_services(
    did_url: &DidUrl,
    document: &DidDocument,
    content_metadata: DidUrlContentMetadata,
    resolved_content_type: Option<&MediaType>,
    prepared: &PreparedRequest,
) -> DidUrlDereferencingResult {
    let service_id = match prepared
        .service
        .as_deref()
        .map(|value| expand_service_id(document, value))
        .transpose()
    {
        Ok(value) => value,
        Err(failure) => return failure_result(failure),
    };

    let services: Vec<Service> = document
        .services()
        .unwrap_or_default()
        .iter()
        .filter(|service| {
            service_id
                .as_deref()
                .is_none_or(|target| service.id().as_str() == target)
                && prepared.service_type.as_deref().is_none_or(|target| {
                    service
                        .service_types()
                        .as_slice()
                        .iter()
                        .any(|value| value == target)
                })
        })
        .cloned()
        .collect();
    if services.is_empty() {
        return standard_failure(DidResolutionErrorKind::NotFound);
    }

    let force_uri = prepared.relative_ref.is_some() || did_url.fragment().is_some();
    match prepared.resolution.accept().map(MediaType::as_str) {
        None if force_uri => endpoint_result(did_url, &services, prepared, content_metadata),
        None => filtered_document_result(
            document,
            services,
            content_metadata,
            resolved_content_type.cloned(),
        ),
        Some(value) if is_uri_list(value) => {
            endpoint_result(did_url, &services, prepared, content_metadata)
        }
        Some(value) if is_did_document_media_type(value) => {
            if force_uri {
                endpoint_result(did_url, &services, prepared, content_metadata)
            } else {
                filtered_document_result(
                    document,
                    services,
                    content_metadata,
                    prepared.resolution.accept().cloned(),
                )
            }
        }
        Some(_) => standard_failure(DidResolutionErrorKind::RepresentationNotSupported),
    }
}

fn expand_service_id(document: &DidDocument, value: &str) -> Result<String, Failure> {
    let candidate = if value.starts_with('#') {
        format!("{}{value}", document.id())
    } else if !value.contains(':') {
        format!("{}#{value}", document.id())
    } else {
        value.to_owned()
    };
    Uri::try_new(candidate)
        .map(Uri::into_string)
        .map_err(|_| Failure::InvalidDidUrl)
}

fn filtered_document_result(
    document: &DidDocument,
    services: Vec<Service>,
    metadata: DidUrlContentMetadata,
    content_type: Option<MediaType>,
) -> DidUrlDereferencingResult {
    match document.with_services(Some(services)) {
        Ok(document) => document_result(&document, metadata, content_type),
        Err(_) => standard_failure(DidResolutionErrorKind::InternalError),
    }
}

fn endpoint_result(
    did_url: &DidUrl,
    services: &[Service],
    prepared: &PreparedRequest,
    content_metadata: DidUrlContentMetadata,
) -> DidUrlDereferencingResult {
    let mut endpoints = collect_service_uris(services);
    if endpoints.is_empty() {
        return standard_failure(DidResolutionErrorKind::NotFound);
    }
    if let Some(relative_ref) = &prepared.relative_ref {
        let mut resolved = Vec::with_capacity(endpoints.len());
        for endpoint in &endpoints {
            let uri = match resolve_relative_ref(endpoint, relative_ref) {
                Ok(value) => value,
                Err(failure) => return failure_result(failure),
            };
            resolved.push(uri);
        }
        endpoints = resolved;
    }
    if let Some(fragment) = did_url.fragment() {
        if endpoints.len() != 1 {
            return standard_failure(DidResolutionErrorKind::InvalidOptions);
        }
        let endpoint = &endpoints[0];
        if endpoint.as_str().contains('#') {
            return standard_failure(DidResolutionErrorKind::InvalidDidUrl);
        }
        let value = format!("{}#{fragment}", endpoint.as_str());
        endpoints[0] = match Uri::try_new(value) {
            Ok(value) => value,
            Err(_) => return standard_failure(DidResolutionErrorKind::InvalidDidUrl),
        };
    }

    let content = DereferencedContent::new(Value::Array(
        endpoints
            .iter()
            .map(|endpoint| Value::String(endpoint.as_str().to_owned()))
            .collect(),
    ));
    let media_type = MediaType::parse(URI_LIST).expect("static media type is valid");
    result(
        DidUrlDereferencingMetadata::new(Some(media_type), None, BTreeMap::new()),
        content,
        content_metadata,
    )
}

fn collect_service_uris(services: &[Service]) -> Vec<Uri> {
    let mut uris = Vec::new();
    for service in services {
        match service.endpoint() {
            ServiceEndpoint::Uri(uri) => uris.push(uri.clone()),
            ServiceEndpoint::Map(_) => {}
            ServiceEndpoint::Set(values) => {
                uris.extend(values.iter().filter_map(|value| match value {
                    ServiceEndpointValue::Uri(uri) => Some(uri.clone()),
                    ServiceEndpointValue::Map(_) => None,
                }));
            }
        }
    }
    uris
}

#[derive(Debug)]
struct AbsoluteUri<'a> {
    prefix: &'a str,
    path: &'a str,
    query: Option<&'a str>,
    has_authority: bool,
}

fn resolve_relative_ref(base: &Uri, relative: &str) -> Result<Uri, Failure> {
    validate_relative_ref(relative)?;
    let base = split_absolute_uri(base.as_str());
    validate_decoded_layers(base.path)?;
    let (reference, fragment) = split_once_optional(relative, '#');
    let (reference_path, reference_query) = split_once_optional(reference, '?');

    let normalized_base_path = remove_dot_segments(base.path);
    let base_scope = base_directory(&normalized_base_path);
    let (path, query) = if reference_path.is_empty() {
        (
            normalized_base_path,
            reference_query.or(base.query).map(str::to_owned),
        )
    } else {
        let merged = if reference_path.starts_with('/') {
            reference_path.to_owned()
        } else if base.has_authority && base.path.is_empty() {
            format!("/{reference_path}")
        } else {
            format!("{}{reference_path}", base_directory(base.path))
        };
        (
            remove_dot_segments(&merged),
            reference_query.map(str::to_owned),
        )
    };
    if !path.starts_with(&base_scope) {
        return Err(Failure::InvalidOptions);
    }

    let mut output = String::with_capacity(base.prefix.len() + path.len() + relative.len() + 2);
    output.push_str(base.prefix);
    output.push_str(&path);
    if let Some(query) = query {
        output.push('?');
        output.push_str(&query);
    }
    if let Some(fragment) = fragment {
        output.push('#');
        output.push_str(fragment);
    }
    Uri::try_new(output).map_err(|_| Failure::InvalidOptions)
}

fn validate_relative_ref(relative: &str) -> Result<(), Failure> {
    if relative.len() > MAX_DID_URL_BYTES || relative.starts_with("//") {
        return Err(Failure::InvalidOptions);
    }
    let before_fragment = relative
        .split_once('#')
        .map_or(relative, |(value, _)| value);
    let path = before_fragment
        .split_once('?')
        .map_or(before_fragment, |(value, _)| value);
    let first_segment = path.split('/').next().unwrap_or_default();
    if first_segment.contains(':') {
        return Err(Failure::InvalidOptions);
    }
    Uri::parse(&format!("relative:{relative}")).map_err(|_| Failure::InvalidOptions)?;

    validate_decoded_layers(relative)
}

fn validate_decoded_layers(value: &str) -> Result<(), Failure> {
    let mut layer = value.to_owned();
    for _ in 0..MAX_DECODE_PASSES {
        reject_unsafe_decoded_layer(&layer)?;
        if !layer.contains('%') {
            return Ok(());
        }
        let decoded = percent_decode(&layer).map_err(|_| Failure::InvalidOptions)?;
        if decoded == layer {
            return Ok(());
        }
        layer = decoded;
    }
    if layer.contains('%') {
        return Err(Failure::InvalidOptions);
    }
    reject_unsafe_decoded_layer(&layer)
}

fn reject_unsafe_decoded_layer(value: &str) -> Result<(), Failure> {
    if value.contains('\\') || value.chars().any(char::is_control) || value.starts_with("//") {
        return Err(Failure::InvalidOptions);
    }
    let before_fragment = value.split_once('#').map_or(value, |(path, _)| path);
    let path = before_fragment
        .split_once('?')
        .map_or(before_fragment, |(path, _)| path);
    if path.split('/').any(|segment| matches!(segment, "." | "..")) {
        return Err(Failure::InvalidOptions);
    }
    Ok(())
}

fn split_absolute_uri(value: &str) -> AbsoluteUri<'_> {
    let colon = value
        .find(':')
        .expect("validated absolute URI always contains a scheme");
    let without_fragment = value.split_once('#').map_or(value, |(value, _)| value);
    let (hierarchy, query) = split_once_optional(without_fragment, '?');
    let has_authority = value[colon + 1..].starts_with("//");
    let path_start = if has_authority {
        value[colon + 3..]
            .find('/')
            .map_or(hierarchy.len(), |offset| colon + 3 + offset)
    } else {
        colon + 1
    };
    AbsoluteUri {
        prefix: &value[..path_start],
        path: &hierarchy[path_start..],
        query,
        has_authority,
    }
}

fn split_once_optional(value: &str, delimiter: char) -> (&str, Option<&str>) {
    value
        .split_once(delimiter)
        .map_or((value, None), |(left, right)| (left, Some(right)))
}

fn base_directory(path: &str) -> String {
    if path.ends_with('/') {
        return path.to_owned();
    }
    path.rfind('/')
        .map_or_else(String::new, |index| path[..=index].to_owned())
}

fn remove_dot_segments(path: &str) -> String {
    let absolute = path.starts_with('/');
    let trailing = path.ends_with('/') || path.ends_with("/.") || path.ends_with("/..");
    let mut segments = Vec::new();
    for segment in path.split('/') {
        match segment {
            "." => {}
            ".." => {
                if segments.len() > usize::from(absolute) {
                    segments.pop();
                }
            }
            value => segments.push(value),
        }
    }
    let mut output = segments.join("/");
    if trailing && !output.ends_with('/') {
        output.push('/');
    }
    output
}

fn is_did_document_media_type(value: &str) -> bool {
    let essence = value.split(';').next().unwrap_or_default().trim();
    essence.eq_ignore_ascii_case(DID_JSON) || essence.eq_ignore_ascii_case(DID_LD_JSON)
}

fn is_uri_list(value: &str) -> bool {
    value
        .split(';')
        .next()
        .unwrap_or_default()
        .trim()
        .eq_ignore_ascii_case(URI_LIST)
}

fn document_result(
    document: &DidDocument,
    content_metadata: DidUrlContentMetadata,
    content_type: Option<MediaType>,
) -> DidUrlDereferencingResult {
    result(
        DidUrlDereferencingMetadata::new(content_type, None, BTreeMap::new()),
        DereferencedContent::from_did_document(document),
        content_metadata,
    )
}

fn resource_result(
    content: Result<DereferencedContent, crate::Error>,
    metadata: DidUrlContentMetadata,
) -> DidUrlDereferencingResult {
    result(Ok(DidUrlDereferencingMetadata::empty()), content, metadata)
}

fn result(
    metadata: Result<DidUrlDereferencingMetadata, crate::Error>,
    content: Result<DereferencedContent, crate::Error>,
    content_metadata: DidUrlContentMetadata,
) -> DidUrlDereferencingResult {
    match metadata.and_then(|metadata| {
        content.and_then(|content| {
            DidUrlDereferencingResult::success(metadata, content, content_metadata)
        })
    }) {
        Ok(result) => result,
        Err(_) => standard_failure(DidResolutionErrorKind::InternalError),
    }
}

fn failure_result(failure: Failure) -> DidUrlDereferencingResult {
    match failure {
        Failure::InvalidDidUrl => standard_failure(DidResolutionErrorKind::InvalidDidUrl),
        Failure::InvalidOptions => standard_failure(DidResolutionErrorKind::InvalidOptions),
    }
}

fn custom_failure(type_uri: &str) -> DidUrlDereferencingResult {
    let error = DidResolutionError::new(
        Uri::parse(type_uri).expect("static error URI is valid"),
        None,
        None,
        None,
        BTreeMap::new(),
    )
    .expect("static error object is valid");
    error_result(error)
}

fn standard_failure(kind: DidResolutionErrorKind) -> DidUrlDereferencingResult {
    error_result(DidResolutionError::standard(kind))
}

fn error_result(error: DidResolutionError) -> DidUrlDereferencingResult {
    let metadata = DidUrlDereferencingMetadata::new(None, Some(error), BTreeMap::new())
        .expect("bounded error metadata is valid");
    DidUrlDereferencingResult::failure(metadata).expect("failure envelope is valid")
}
