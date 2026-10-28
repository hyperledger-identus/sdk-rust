use identus_oid4vp::{
    AuthorizationRequestInvocation, AuthorizationRequestInvocationLimits, FORM_MEDIA_TYPE,
    REQUEST_OBJECT_MEDIA_TYPE, RequestUriMethod, RequestUriRetrievalInput,
    RequestUriRetrievalLimits, RequestUriRetrievalRequest,
};
use serde::Deserialize;

const TYPESCRIPT_FIXTURE: &str = r#"{
    "clientId": "decentralized_identifier:did:example:verifier",
    "requestUri": "https://verifier.example/request",
    "requestUriMethod": "post",
    "walletMetadata": "{\"vp_formats_supported\":{\"dc+sd-jwt\":{}}}",
    "walletNonce": "adapter-nonce"
}"#;

const SWIFT_FIXTURE: &str = r#"{
    "client_id": "decentralized_identifier:did:example:verifier",
    "request_uri": "https://verifier.example/request",
    "request_uri_method": "get"
}"#;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TypeScriptAdapterInput {
    client_id: String,
    request_uri: String,
    request_uri_method: String,
    wallet_metadata: Option<String>,
    wallet_nonce: Option<String>,
}

#[derive(Deserialize)]
struct SwiftAdapterInput {
    client_id: String,
    request_uri: String,
    request_uri_method: String,
    wallet_metadata: Option<String>,
    wallet_nonce: Option<String>,
}

struct NativeAdapterInput {
    client_id: String,
    request_uri: String,
    request_uri_method: String,
    wallet_metadata: Option<String>,
    wallet_nonce: Option<String>,
}

impl From<TypeScriptAdapterInput> for NativeAdapterInput {
    fn from(input: TypeScriptAdapterInput) -> Self {
        Self {
            client_id: input.client_id,
            request_uri: input.request_uri,
            request_uri_method: input.request_uri_method,
            wallet_metadata: input.wallet_metadata,
            wallet_nonce: input.wallet_nonce,
        }
    }
}

impl From<SwiftAdapterInput> for NativeAdapterInput {
    fn from(input: SwiftAdapterInput) -> Self {
        Self {
            client_id: input.client_id,
            request_uri: input.request_uri,
            request_uri_method: input.request_uri_method,
            wallet_metadata: input.wallet_metadata,
            wallet_nonce: input.wallet_nonce,
        }
    }
}

fn percent_encode(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
            encoded.push(char::from(byte));
        } else {
            use std::fmt::Write as _;
            write!(&mut encoded, "%{byte:02X}").expect("writing to String cannot fail");
        }
    }
    encoded
}

fn translate(input: NativeAdapterInput) -> RequestUriRetrievalRequest {
    let retrieval = match input.request_uri_method.as_str() {
        "get" => RequestUriRetrievalInput::get(),
        "post" => RequestUriRetrievalInput::post(
            input.wallet_metadata.as_deref(),
            input.wallet_nonce.as_deref(),
        ),
        method => panic!("unsupported test adapter method: {method}"),
    };
    let invocation = format!(
        "openid4vp:?client_id={}&request_uri={}&request_uri_method={}",
        percent_encode(&input.client_id),
        percent_encode(&input.request_uri),
        percent_encode(&input.request_uri_method)
    );
    let AuthorizationRequestInvocation::Referenced(reference) =
        AuthorizationRequestInvocation::parse(
            &invocation,
            AuthorizationRequestInvocationLimits::default(),
        )
        .expect("adapter input maps to the Rust-native invocation");
    reference
        .prepare_retrieval(retrieval, RequestUriRetrievalLimits::default())
        .expect("adapter input maps to the Rust-native retrieval envelope")
}

fn assert_common_native_contract(request: &RequestUriRetrievalRequest) {
    assert_eq!(request.accept(), REQUEST_OBJECT_MEDIA_TYPE);
    assert_eq!(
        request.expose_sensitive_endpoint(),
        "https://verifier.example/request"
    );
}

fn assert_post_native_contract(request: &RequestUriRetrievalRequest) {
    assert_common_native_contract(request);
    assert_eq!(request.method(), RequestUriMethod::Post);
    assert_eq!(request.content_type(), Some(FORM_MEDIA_TYPE));
    assert_eq!(
        request.expose_sensitive_body(),
        b"wallet_metadata=%7B%22vp_formats_supported%22%3A%7B%22dc%2Bsd-jwt%22%3A%7B%7D%7D%7D&wallet_nonce=adapter-nonce"
    );
}

fn assert_get_native_contract(request: &RequestUriRetrievalRequest) {
    assert_common_native_contract(request);
    assert_eq!(request.method(), RequestUriMethod::Get);
    assert_eq!(request.content_type(), None);
    assert_eq!(request.expose_sensitive_body(), b"");
}

#[test]
fn typescript_shaped_input_stays_at_the_adapter_boundary() {
    let input: TypeScriptAdapterInput =
        serde_json::from_str(TYPESCRIPT_FIXTURE).expect("independent TypeScript-shaped fixture");
    assert_post_native_contract(&translate(input.into()));
}

#[test]
fn swift_shaped_input_stays_at_the_adapter_boundary() {
    let input: SwiftAdapterInput =
        serde_json::from_str(SWIFT_FIXTURE).expect("independent Swift-shaped fixture");
    assert_get_native_contract(&translate(input.into()));
}
