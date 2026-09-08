use std::sync::Mutex;

use axum::{
    body::{Body, to_bytes},
    http::{Request, header},
};
use identus_did::{
    DidDocument, DidDocumentMetadata, DidResolutionFuture, DidResolutionMetadata, Uri,
};
use serde_json::Value;
use tower::ServiceExt;

use super::*;

const TEST_DID: &str = "did:example:123";

#[derive(Debug)]
struct RecordingResolver {
    result: DidResolutionResult,
    calls: Mutex<Vec<(String, Option<String>)>>,
}

impl RecordingResolver {
    fn new(result: DidResolutionResult) -> Self {
        Self {
            result,
            calls: Mutex::new(Vec::new()),
        }
    }

    fn calls(&self) -> Vec<(String, Option<String>)> {
        self.calls.lock().unwrap().clone()
    }
}

impl DidResolver for RecordingResolver {
    fn resolve<'a>(
        &'a self,
        did: &'a Did,
        options: &'a ResolutionOptions,
    ) -> DidResolutionFuture<'a> {
        self.calls.lock().unwrap().push((
            did.as_str().to_owned(),
            options.accept().map(|value| value.as_str().to_owned()),
        ));
        let result = self.result.clone();
        Box::pin(async move { result })
    }
}

fn success_result(did: &str, content_type: Option<&str>) -> DidResolutionResult {
    let metadata = DidResolutionMetadata::new(
        content_type.map(|value| DidMediaType::parse(value).unwrap()),
        None,
        BTreeMap::new(),
    )
    .unwrap();
    let document = DidDocument::from_json_str(&format!(r#"{{"id":"{did}"}}"#)).unwrap();
    DidResolutionResult::success(metadata, document, DidDocumentMetadata::empty()).unwrap()
}

fn failure_result(kind: DidResolutionErrorKind) -> DidResolutionResult {
    let metadata = DidResolutionMetadata::new(
        None,
        Some(DidResolutionError::standard(kind)),
        BTreeMap::new(),
    )
    .unwrap();
    DidResolutionResult::failure(metadata).unwrap()
}

fn extension_failure_result() -> DidResolutionResult {
    let error = DidResolutionError::new(
        Uri::parse("https://example.com/problems/resolver").unwrap(),
        None,
        None,
        None,
        BTreeMap::new(),
    )
    .unwrap();
    let metadata = DidResolutionMetadata::new(None, Some(error), BTreeMap::new()).unwrap();
    DidResolutionResult::failure(metadata).unwrap()
}

fn deactivated_result() -> DidResolutionResult {
    let metadata = DidResolutionMetadata::empty();
    let document_metadata = DidDocumentMetadata::builder()
        .deactivated(true)
        .build()
        .unwrap();
    DidResolutionResult::deactivated(metadata, document_metadata).unwrap()
}

async fn send(
    resolver: Arc<RecordingResolver>,
    uri: &str,
    accept_values: &[&str],
) -> (StatusCode, HeaderMap, Value) {
    let mut request = Request::builder().uri(uri);
    for value in accept_values {
        request = request.header(header::ACCEPT, *value);
    }
    let response = did_resolver_http_router(resolver)
        .oneshot(request.body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let headers = response.headers().clone();
    let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let value = serde_json::from_slice(&body).unwrap();
    (status, headers, value)
}

fn assert_resolution_headers(headers: &HeaderMap, content_type: &str) {
    assert_eq!(headers[header::CONTENT_TYPE], content_type);
    assert_eq!(headers[header::VARY], "accept");
}

fn error_type(body: &Value) -> &str {
    body["didResolutionMetadata"]["error"]["type"]
        .as_str()
        .unwrap()
}

#[tokio::test]
async fn missing_accept_defaults_to_did_document() {
    let resolver = Arc::new(RecordingResolver::new(success_result(
        TEST_DID,
        Some(APPLICATION_DID),
    )));
    let (status, headers, body) = send(resolver.clone(), "/did:example:123", &[]).await;

    assert_eq!(status, StatusCode::OK);
    assert_resolution_headers(&headers, APPLICATION_DID);
    assert_eq!(body["id"], TEST_DID);
    assert!(body.get("didResolutionMetadata").is_none());
    assert_eq!(
        resolver.calls(),
        vec![(TEST_DID.to_owned(), Some(APPLICATION_DID.to_owned()))]
    );
}

#[tokio::test]
async fn router_nests_and_decodes_exactly_one_http_path_layer() {
    let resolver = Arc::new(RecordingResolver::new(success_result(
        TEST_DID,
        Some(APPLICATION_DID),
    )));
    let app = Router::new().nest(
        "/1.0/identifiers",
        did_resolver_http_router(resolver.clone()),
    );
    let response = app
        .oneshot(
            Request::builder()
                .uri("/1.0/identifiers/did%3Aexample%3A123")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(resolver.calls()[0].0, TEST_DID);

    let escaped_did = "did:example:abc%2Fdef";
    let resolver = Arc::new(RecordingResolver::new(success_result(
        escaped_did,
        Some(APPLICATION_DID),
    )));
    let (status, _, body) = send(resolver.clone(), "/did%3Aexample%3Aabc%252Fdef", &[]).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["id"], escaped_did);
    assert_eq!(resolver.calls()[0].0, escaped_did);
}

#[tokio::test]
async fn full_result_uses_empty_resolution_options() {
    let resolver = Arc::new(RecordingResolver::new(success_result(
        TEST_DID,
        Some(APPLICATION_DID),
    )));
    let (status, headers, body) = send(
        resolver.clone(),
        "/did:example:123",
        &[APPLICATION_DID_RESOLUTION],
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_resolution_headers(&headers, APPLICATION_DID_RESOLUTION);
    assert_eq!(body["didDocument"]["id"], TEST_DID);
    assert_eq!(resolver.calls(), vec![(TEST_DID.to_owned(), None)]);
}

#[tokio::test]
async fn json_document_passes_exact_accept_option() {
    let resolver = Arc::new(RecordingResolver::new(success_result(
        TEST_DID,
        Some(APPLICATION_JSON),
    )));
    let (status, headers, body) =
        send(resolver.clone(), "/did:example:123", &[APPLICATION_JSON]).await;

    assert_eq!(status, StatusCode::OK);
    assert_resolution_headers(&headers, APPLICATION_JSON);
    assert_eq!(body["id"], TEST_DID);
    assert_eq!(
        resolver.calls(),
        vec![(TEST_DID.to_owned(), Some(APPLICATION_JSON.to_owned()))]
    );
}

#[tokio::test]
async fn wildcard_quality_specificity_and_repeated_fields_are_honored() {
    let wildcard = Arc::new(RecordingResolver::new(success_result(
        TEST_DID,
        Some(APPLICATION_DID),
    )));
    let (status, headers, _) = send(wildcard.clone(), "/did:example:123", &["*/*"]).await;
    assert_eq!(status, StatusCode::OK);
    assert_resolution_headers(&headers, APPLICATION_DID);

    let quality = Arc::new(RecordingResolver::new(success_result(
        TEST_DID,
        Some(APPLICATION_JSON),
    )));
    let (status, headers, _) = send(
        quality.clone(),
        "/did:example:123",
        &["application/*;q=0.8", "application/did;q=0.1"],
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_resolution_headers(&headers, APPLICATION_JSON);
    assert_eq!(
        quality.calls(),
        vec![(TEST_DID.to_owned(), Some(APPLICATION_JSON.to_owned()))]
    );

    let client_order = Arc::new(RecordingResolver::new(success_result(
        TEST_DID,
        Some(APPLICATION_JSON),
    )));
    let (status, headers, _) = send(
        client_order,
        "/did:example:123",
        &["application/json, application/did"],
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_resolution_headers(&headers, APPLICATION_JSON);
}

#[tokio::test]
async fn valid_unsupported_accept_returns_406_without_resolver_call() {
    let resolver = Arc::new(RecordingResolver::new(success_result(
        TEST_DID,
        Some(APPLICATION_DID),
    )));
    let (status, headers, body) = send(resolver.clone(), "/did:example:123", &["text/plain"]).await;

    assert_eq!(status, StatusCode::NOT_ACCEPTABLE);
    assert_resolution_headers(&headers, APPLICATION_DID_RESOLUTION);
    assert!(error_type(&body).ends_with("#REPRESENTATION_NOT_SUPPORTED"));
    assert!(resolver.calls().is_empty());
}

#[tokio::test]
async fn malformed_quality_and_list_shape_return_400() {
    for accept in [
        "application/did;q=2",
        "application/did;q=\"0.5\"",
        "application/did;q=0.5;q=0.4",
        ",application/did",
        "application/did,",
        "application/did,,application/json",
        "application/did;note=\"unterminated",
    ] {
        let resolver = Arc::new(RecordingResolver::new(success_result(
            TEST_DID,
            Some(APPLICATION_DID),
        )));
        let (status, _, body) = send(resolver.clone(), "/did:example:123", &[accept]).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "accept={accept}");
        assert!(error_type(&body).ends_with("#INVALID_OPTIONS"));
        assert!(resolver.calls().is_empty());
    }
}

#[tokio::test]
async fn accept_resource_limits_fail_before_resolution() {
    let oversized = format!(
        "application/did;note={}",
        "a".repeat(MAX_ACCEPT_HEADER_BYTES)
    );
    let too_many = std::iter::repeat_n("application/did", MAX_ACCEPT_MEDIA_RANGES + 1)
        .collect::<Vec<_>>()
        .join(",");

    for accept in [&oversized, &too_many] {
        let resolver = Arc::new(RecordingResolver::new(success_result(
            TEST_DID,
            Some(APPLICATION_DID),
        )));
        let (status, _, body) = send(resolver.clone(), "/did:example:123", &[accept]).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(error_type(&body).ends_with("#INVALID_OPTIONS"));
        assert!(resolver.calls().is_empty());
    }
}

#[tokio::test]
async fn invalid_did_path_and_nonempty_query_fail_without_resolution() {
    for (uri, suffix) in [
        ("/not-a-did", "#INVALID_DID"),
        ("/%FF", "#INVALID_DID"),
        ("/did:example:123?versionId=1", "#INVALID_OPTIONS"),
    ] {
        let resolver = Arc::new(RecordingResolver::new(success_result(
            TEST_DID,
            Some(APPLICATION_DID),
        )));
        let (status, headers, body) = send(resolver.clone(), uri, &[]).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_resolution_headers(&headers, APPLICATION_DID_RESOLUTION);
        assert!(error_type(&body).ends_with(suffix));
        assert!(resolver.calls().is_empty());
    }
}

#[tokio::test]
async fn standard_resolution_errors_map_to_binding_statuses() {
    let cases = [
        (DidResolutionErrorKind::InvalidDid, StatusCode::BAD_REQUEST),
        (
            DidResolutionErrorKind::InvalidDidDocument,
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
        (DidResolutionErrorKind::NotFound, StatusCode::NOT_FOUND),
        (
            DidResolutionErrorKind::RepresentationNotSupported,
            StatusCode::NOT_ACCEPTABLE,
        ),
        (
            DidResolutionErrorKind::InvalidDidUrl,
            StatusCode::BAD_REQUEST,
        ),
        (
            DidResolutionErrorKind::MethodNotSupported,
            StatusCode::NOT_IMPLEMENTED,
        ),
        (
            DidResolutionErrorKind::InvalidOptions,
            StatusCode::BAD_REQUEST,
        ),
        (
            DidResolutionErrorKind::InternalError,
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
        (
            DidResolutionErrorKind::FeatureNotSupported,
            StatusCode::NOT_IMPLEMENTED,
        ),
    ];

    for (kind, expected_status) in cases {
        let resolver = Arc::new(RecordingResolver::new(failure_result(kind)));
        let (status, headers, body) = send(resolver, "/did:example:123", &[]).await;
        assert_eq!(status, expected_status, "kind={kind:?}");
        assert_resolution_headers(&headers, APPLICATION_DID_RESOLUTION);
        assert!(body["didDocument"].is_null());
    }
}

#[tokio::test]
async fn extension_failure_maps_to_internal_server_error() {
    let resolver = Arc::new(RecordingResolver::new(extension_failure_result()));
    let (status, headers, body) = send(resolver, "/did:example:123", &[]).await;

    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    assert_resolution_headers(&headers, APPLICATION_DID_RESOLUTION);
    assert_eq!(error_type(&body), "https://example.com/problems/resolver");
}

#[tokio::test]
async fn deactivated_result_maps_to_gone_and_full_envelope() {
    let resolver = Arc::new(RecordingResolver::new(deactivated_result()));
    let (status, headers, body) = send(resolver, "/did:example:123", &[]).await;

    assert_eq!(status, StatusCode::GONE);
    assert_resolution_headers(&headers, APPLICATION_DID_RESOLUTION);
    assert_eq!(body["didDocumentMetadata"]["deactivated"], true);
}

#[tokio::test]
async fn invalid_success_projection_returns_internal_error() {
    let cases = [
        success_result(TEST_DID, None),
        success_result(TEST_DID, Some(APPLICATION_JSON)),
        success_result("did:example:other", Some(APPLICATION_DID)),
    ];

    for result in cases {
        let resolver = Arc::new(RecordingResolver::new(result));
        let (status, headers, body) = send(resolver, "/did:example:123", &[]).await;
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_resolution_headers(&headers, APPLICATION_DID_RESOLUTION);
        assert!(error_type(&body).ends_with("#INTERNAL_ERROR"));
    }
}

#[tokio::test]
async fn rejected_input_is_not_reflected_in_error_body() {
    let sensitive = "not-a-did-secret-marker";
    let resolver = Arc::new(RecordingResolver::new(success_result(
        TEST_DID,
        Some(APPLICATION_DID),
    )));
    let (_, _, body) = send(resolver, &format!("/{sensitive}"), &[]).await;

    assert!(!body.to_string().contains(sensitive));
}
