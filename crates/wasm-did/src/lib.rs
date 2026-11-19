//! Experimental WASM facade for bounded DID values, documents, resolution, and
//! dereferencing.
//!
//! This unpublished outer boundary exposes only owned public identifier values
//! and stable error codes. `identus-did` remains binding-framework-free.

mod peer;
mod prism;
mod resolution;

use identus_did::{Did, DidDocument, DidUrl};
use serde_json::Value;
use wasm_bindgen::prelude::*;

/// Version of the JavaScript value and error contract.
pub const BINDING_API_VERSION: u32 = 5;

const INVALID_DID_CODE: &str = "did.invalid_did";
const INVALID_DID_URL_CODE: &str = "did.invalid_did_url";
const INVALID_DID_DOCUMENT_CODE: &str = "did.invalid_document";
const INVALID_RESOLUTION_OPTIONS_CODE: &str = "did.invalid_resolution_options";
const INVALID_DEREFERENCING_OPTIONS_CODE: &str = "did.invalid_dereferencing_options";
const INVALID_RESOLUTION_RESULT_CODE: &str = "did.invalid_resolution_result";
const INVALID_DEREFERENCING_RESULT_CODE: &str = "did.invalid_dereferencing_result";
const UNSUPPORTED_LOSSY_MAPPING_CODE: &str = "adapter.unsupported_lossy_mapping";
const BINDING_CONTRACT_CODE: &str = "adapter.binding_contract";
const PEER_WRONG_PROFILE_CODE: &str = "did.peer.wrong_profile";
const PEER_UNSUPPORTED_ALGORITHM_CODE: &str = "did.peer.unsupported_algorithm";
const PEER_MALFORMED_ENCODING_CODE: &str = "did.peer.malformed_encoding";
const PEER_UNSUPPORTED_CODEC_CODE: &str = "did.peer.unsupported_codec";
const PEER_UNSUPPORTED_PURPOSE_CODE: &str = "did.peer.unsupported_purpose";
const PEER_INVALID_INPUT_DOCUMENT_CODE: &str = "did.peer.invalid_input_document";
const PEER_HASH_MISMATCH_CODE: &str = "did.peer.hash_mismatch";
const PEER_LONG_FORM_REQUIRED_CODE: &str = "did.peer.long_form_required";
const PEER_RESOURCE_LIMIT_CODE: &str = "did.peer.resource_limit";
const PEER_PROJECTION_FAILURE_CODE: &str = "did.peer.projection_failure";
const PRISM_INVALID_SYNTAX_CODE: &str = "did.prism.invalid_syntax";
const PRISM_INVALID_WIRE_CODE: &str = "did.prism.invalid_wire";
const PRISM_INVALID_OPERATION_CODE: &str = "did.prism.invalid_operation";
const PRISM_INVALID_STATE_CODE: &str = "did.prism.invalid_state";
const MAX_SAFE_JAVASCRIPT_INTEGER: i64 = 9_007_199_254_740_991;

/// Owned JavaScript view of a validated DID.
#[derive(Debug)]
#[wasm_bindgen]
pub struct DidView {
    value: String,
    method: String,
    method_specific_id: String,
}

#[wasm_bindgen]
impl DidView {
    /// Exact validated DID value.
    #[wasm_bindgen(getter)]
    pub fn value(&self) -> String {
        self.value.clone()
    }

    /// DID method name.
    #[wasm_bindgen(getter)]
    pub fn method(&self) -> String {
        self.method.clone()
    }

    /// Method-specific identifier.
    #[wasm_bindgen(getter, js_name = methodSpecificId)]
    pub fn method_specific_id(&self) -> String {
        self.method_specific_id.clone()
    }
}

/// Owned JavaScript view of a validated DID URL.
#[derive(Debug)]
#[wasm_bindgen]
pub struct DidUrlView {
    value: String,
    did: String,
    method: String,
    method_specific_id: String,
    path: String,
    query: Option<String>,
    fragment: Option<String>,
}

/// Owned JavaScript view of a validated DID Document.
///
/// `serialized` is deterministic normalized JSON produced from the validated
/// Rust model. It preserves JSON cardinality and semantic values, but does not
/// claim original whitespace, object member order, or RFC 8785 canonical JSON.
#[derive(Debug)]
#[wasm_bindgen]
pub struct DidDocumentView {
    serialized: String,
    id: String,
}

#[wasm_bindgen]
impl DidDocumentView {
    /// Deterministic normalized JSON for the validated DID Document.
    #[wasm_bindgen(getter)]
    pub fn serialized(&self) -> String {
        self.serialized.clone()
    }

    /// Exact validated DID subject.
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.id.clone()
    }
}

#[wasm_bindgen]
impl DidUrlView {
    /// Exact validated DID URL value.
    #[wasm_bindgen(getter)]
    pub fn value(&self) -> String {
        self.value.clone()
    }

    /// Base DID portion.
    #[wasm_bindgen(getter)]
    pub fn did(&self) -> String {
        self.did.clone()
    }

    /// DID method name.
    #[wasm_bindgen(getter)]
    pub fn method(&self) -> String {
        self.method.clone()
    }

    /// Method-specific identifier.
    #[wasm_bindgen(getter, js_name = methodSpecificId)]
    pub fn method_specific_id(&self) -> String {
        self.method_specific_id.clone()
    }

    /// Path including its leading slash when present.
    #[wasm_bindgen(getter)]
    pub fn path(&self) -> String {
        self.path.clone()
    }

    /// Query without the leading question mark.
    #[wasm_bindgen(getter)]
    pub fn query(&self) -> Option<String> {
        self.query.clone()
    }

    /// Fragment without the leading hash.
    #[wasm_bindgen(getter)]
    pub fn fragment(&self) -> Option<String> {
        self.fragment.clone()
    }
}

/// Closed JavaScript failure carrying only a stable SDK-owned code.
#[derive(Debug)]
#[wasm_bindgen]
pub struct DidParseError {
    code: String,
}

#[wasm_bindgen]
impl DidParseError {
    /// Stable code without caller or implementation text.
    #[wasm_bindgen(getter)]
    pub fn code(&self) -> String {
        self.code.clone()
    }
}

impl DidParseError {
    pub(crate) fn invalid_did() -> Self {
        Self {
            code: INVALID_DID_CODE.to_owned(),
        }
    }

    pub(crate) fn invalid_did_url() -> Self {
        Self {
            code: INVALID_DID_URL_CODE.to_owned(),
        }
    }

    fn invalid_did_document() -> Self {
        Self {
            code: INVALID_DID_DOCUMENT_CODE.to_owned(),
        }
    }

    pub(crate) fn unsupported_lossy_mapping() -> Self {
        Self {
            code: UNSUPPORTED_LOSSY_MAPPING_CODE.to_owned(),
        }
    }

    pub(crate) fn invalid_resolution_options() -> Self {
        Self {
            code: INVALID_RESOLUTION_OPTIONS_CODE.to_owned(),
        }
    }

    pub(crate) fn invalid_dereferencing_options() -> Self {
        Self {
            code: INVALID_DEREFERENCING_OPTIONS_CODE.to_owned(),
        }
    }

    pub(crate) fn invalid_resolution_result() -> Self {
        Self {
            code: INVALID_RESOLUTION_RESULT_CODE.to_owned(),
        }
    }

    pub(crate) fn invalid_dereferencing_result() -> Self {
        Self {
            code: INVALID_DEREFERENCING_RESULT_CODE.to_owned(),
        }
    }

    pub(crate) fn peer(error: identus_did_peer::Error) -> Self {
        let code = match error {
            identus_did_peer::Error::WrongProfile => PEER_WRONG_PROFILE_CODE,
            identus_did_peer::Error::UnsupportedAlgorithm => PEER_UNSUPPORTED_ALGORITHM_CODE,
            identus_did_peer::Error::MalformedEncoding => PEER_MALFORMED_ENCODING_CODE,
            identus_did_peer::Error::UnsupportedCodec => PEER_UNSUPPORTED_CODEC_CODE,
            identus_did_peer::Error::UnsupportedPurpose => PEER_UNSUPPORTED_PURPOSE_CODE,
            identus_did_peer::Error::InvalidInputDocument => PEER_INVALID_INPUT_DOCUMENT_CODE,
            identus_did_peer::Error::HashMismatch => PEER_HASH_MISMATCH_CODE,
            identus_did_peer::Error::LongFormRequired => PEER_LONG_FORM_REQUIRED_CODE,
            identus_did_peer::Error::ResourceLimit => PEER_RESOURCE_LIMIT_CODE,
            identus_did_peer::Error::Projection => PEER_PROJECTION_FAILURE_CODE,
            _ => BINDING_CONTRACT_CODE,
        };
        Self {
            code: code.to_owned(),
        }
    }

    pub(crate) fn prism(error: identus_prism_did::Error) -> Self {
        let code = match error {
            identus_prism_did::Error::InvalidSyntax(_) => PRISM_INVALID_SYNTAX_CODE,
            identus_prism_did::Error::InvalidWire(_) => PRISM_INVALID_WIRE_CODE,
            identus_prism_did::Error::InvalidOperation(_) => PRISM_INVALID_OPERATION_CODE,
            identus_prism_did::Error::InvalidState(_) => PRISM_INVALID_STATE_CODE,
            _ => BINDING_CONTRACT_CODE,
        };
        Self {
            code: code.to_owned(),
        }
    }
}

/// Return the binding contract version.
#[wasm_bindgen(js_name = bindingApiVersion)]
pub fn binding_api_version() -> u32 {
    BINDING_API_VERSION
}

pub use peer::{
    PeerDidNumalgo2CompatibilityView, PeerDidNumalgo4ShortView, PeerDidNumalgo4View,
    create_peer_did_numalgo2_compatibility, create_peer_did_numalgo4,
    parse_peer_did_numalgo2_compatibility, parse_peer_did_numalgo4_long,
    parse_peer_did_numalgo4_short, resolve_peer_did_numalgo4_short,
};
pub use prism::{
    PrismDidStateView, PrismDidView, create_prism_did, parse_prism_did, reduce_prism_did_history,
    resolve_prism_did_long_form,
};

/// Parse a bounded DID into an owned JavaScript view.
#[wasm_bindgen(js_name = parseDid)]
pub fn parse_did(value: &str) -> Result<DidView, DidParseError> {
    Did::parse(value)
        .map(|did| DidView {
            value: did.as_str().to_owned(),
            method: did.method().to_owned(),
            method_specific_id: did.method_specific_id().to_owned(),
        })
        .map_err(|_| DidParseError::invalid_did())
}

/// Parse a bounded DID URL into an owned JavaScript view.
#[wasm_bindgen(js_name = parseDidUrl)]
pub fn parse_did_url(value: &str) -> Result<DidUrlView, DidParseError> {
    DidUrl::parse(value)
        .map(|did_url| DidUrlView {
            value: did_url.as_str().to_owned(),
            did: did_url.as_did_str().to_owned(),
            method: did_url.method().to_owned(),
            method_specific_id: did_url.method_specific_id().to_owned(),
            path: did_url.path().to_owned(),
            query: did_url.query().map(str::to_owned),
            fragment: did_url.fragment().map(str::to_owned),
        })
        .map_err(|_| DidParseError::invalid_did_url())
}

/// Parse a bounded DID Document into an owned JavaScript view.
///
/// Rust validates the raw JSON before any TypeScript projection. Documents
/// containing an integer outside JavaScript's exact safe range are valid Rust
/// domain values but cannot cross this typed adapter without corruption, so
/// they fail with an explicit unsupported-mapping code.
#[wasm_bindgen(js_name = parseDidDocument)]
pub fn parse_did_document(value: &str) -> Result<DidDocumentView, DidParseError> {
    let document =
        DidDocument::from_json_str(value).map_err(|_| DidParseError::invalid_did_document())?;
    let serialized =
        serde_json::to_string(&document).map_err(|_| DidParseError::invalid_did_document())?;
    let normalized: Value =
        serde_json::from_str(&serialized).map_err(|_| DidParseError::invalid_did_document())?;
    if contains_unsafe_javascript_integer(&normalized) {
        return Err(DidParseError::unsupported_lossy_mapping());
    }
    Ok(DidDocumentView {
        id: document.id().as_str().to_owned(),
        serialized,
    })
}

pub(crate) fn contains_unsafe_javascript_integer(root: &Value) -> bool {
    let mut pending = vec![root];
    while let Some(value) = pending.pop() {
        match value {
            Value::Array(values) => pending.extend(values),
            Value::Object(values) => pending.extend(values.values()),
            Value::Number(number) if number.is_i64() => {
                let Some(value) = number.as_i64() else {
                    return true;
                };
                if !(-MAX_SAFE_JAVASCRIPT_INTEGER..=MAX_SAFE_JAVASCRIPT_INTEGER).contains(&value) {
                    return true;
                }
            }
            Value::Number(number) if number.is_u64() => {
                let Some(value) = number.as_u64() else {
                    return true;
                };
                if value > MAX_SAFE_JAVASCRIPT_INTEGER as u64 {
                    return true;
                }
            }
            Value::Number(number) if number.is_f64() => {
                let Some(value) = number.as_f64() else {
                    return true;
                };
                if value.fract() == 0.0 && value.abs() > MAX_SAFE_JAVASCRIPT_INTEGER as f64 {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use identus_did::{MAX_DID_BYTES, MAX_DID_DOCUMENT_BYTES, MAX_DID_URL_BYTES};

    use super::*;

    #[test]
    fn version_and_component_views_are_stable() {
        assert_eq!(binding_api_version(), 5);

        let did = parse_did("did:example:123").unwrap();
        assert_eq!(did.value(), "did:example:123");
        assert_eq!(did.method(), "example");
        assert_eq!(did.method_specific_id(), "123");

        let url = parse_did_url("did:example:123/path?service=agent#key-1").unwrap();
        assert_eq!(url.value(), "did:example:123/path?service=agent#key-1");
        assert_eq!(url.did(), "did:example:123");
        assert_eq!(url.path(), "/path");
        assert_eq!(url.query().as_deref(), Some("service=agent"));
        assert_eq!(url.fragment().as_deref(), Some("key-1"));
    }

    #[test]
    fn errors_are_bounded_and_redacted() {
        let canary = "do-not-reflect";
        let error = parse_did(&format!("did:EXAMPLE:{canary}")).unwrap_err();
        assert_eq!(error.code(), INVALID_DID_CODE);
        assert!(!error.code().contains(canary));

        let oversized_did = format!("did:example:{}", "a".repeat(MAX_DID_BYTES));
        assert_eq!(
            parse_did(&oversized_did).unwrap_err().code(),
            INVALID_DID_CODE
        );

        let oversized_url = format!("did:example:123/{}", "a".repeat(MAX_DID_URL_BYTES));
        assert_eq!(
            parse_did_url(&oversized_url).unwrap_err().code(),
            INVALID_DID_URL_CODE
        );
    }

    #[test]
    fn prism_shaped_long_form_value_crosses_the_wasm_boundary() {
        let prefix = format!("did:prism:{}:", "0".repeat(64));
        let long_form = format!("{prefix}{}", "A".repeat(100_000 - prefix.len()));

        let accepted = parse_did(&long_form).unwrap();

        assert_eq!(accepted.value, long_form);
        assert_eq!(accepted.method, "prism");
    }

    #[test]
    fn document_round_trip_is_normalized_and_owned() {
        let input = r#"{
            "id":"did:example:123",
            "controller":["did:example:123"],
            "verificationMethod":[{
                "id":"did:example:123#key-1",
                "type":"Multikey",
                "controller":"did:example:123",
                "publicKeyMultibase":"z6MkiTBz1y"
            }],
            "authentication":["did:example:123#key-1"],
            "service":[{
                "id":"did:example:123#messages",
                "type":["DIDCommMessaging","ExampleInbox"],
                "serviceEndpoint":[
                    "https://example.com/inbox",
                    {"uri":"https://backup.example.com/inbox"}
                ]
            }],
            "profile":{"enabled":true,"priority":7}
        }"#;

        let view = parse_did_document(input).unwrap();
        assert_eq!(view.id(), "did:example:123");
        let normalized: DidDocument = serde_json::from_str(&view.serialized()).unwrap();
        assert_eq!(normalized.id().as_str(), "did:example:123");
        assert_eq!(normalized.services().unwrap().len(), 1);
        assert_eq!(normalized.extensions()["profile"]["priority"], 7);
        assert_eq!(
            view.serialized(),
            serde_json::to_string(&normalized).unwrap()
        );
    }

    #[test]
    fn document_failures_are_closed_and_redacted() {
        let marker = "caller-secret-document-marker";
        let duplicate =
            format!(r#"{{"id":"did:example:123","profile":{{"{marker}":1,"{marker}":2}}}}"#);
        let error = parse_did_document(&duplicate).unwrap_err();
        assert_eq!(error.code(), INVALID_DID_DOCUMENT_CODE);
        assert!(!error.code().contains(marker));

        let private = r#"{
            "id":"did:example:123",
            "verificationMethod":[{
                "id":"did:example:123#key-1",
                "type":"JsonWebKey2020",
                "controller":"did:example:123",
                "publicKeyJwk":{"kty":"OKP","d":"private"}
            }]
        }"#;
        assert_eq!(
            parse_did_document(private).unwrap_err().code(),
            INVALID_DID_DOCUMENT_CODE
        );
    }

    #[test]
    fn document_projection_rejects_unsafe_integers_without_rounding() {
        for unsafe_integer in [
            "9007199254740992",
            "-9007199254740992",
            "18446744073709551615",
            "9007199254740992.0",
            "9.007199254740992e15",
            "1e20",
            "-1e20",
        ] {
            let input = format!(r#"{{"id":"did:example:123","unsafe":{unsafe_integer}}}"#);
            let error = parse_did_document(&input).unwrap_err();
            assert_eq!(error.code(), UNSUPPORTED_LOSSY_MAPPING_CODE);
            assert!(!error.code().contains(unsafe_integer));
        }

        for safe_number in ["9007199254740991", "-9007199254740991", "1.25"] {
            let input = format!(r#"{{"id":"did:example:123","safe":{safe_number}}}"#);
            let view = parse_did_document(&input).unwrap();
            assert!(view.serialized().contains(safe_number));
        }
    }

    #[test]
    fn document_wire_limit_is_exact_and_redacted() {
        let base = r#"{"id":"did:example:123"}"#;
        let exact = format!("{base}{}", " ".repeat(MAX_DID_DOCUMENT_BYTES - base.len()));
        assert_eq!(exact.len(), MAX_DID_DOCUMENT_BYTES);
        assert_eq!(parse_did_document(&exact).unwrap().serialized(), base);

        let over_limit = format!("{exact} ");
        let error = parse_did_document(&over_limit).unwrap_err();
        assert_eq!(error.code(), INVALID_DID_DOCUMENT_CODE);
        assert!(!error.code().contains("did:example:123"));
    }
}
