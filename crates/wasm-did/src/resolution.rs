use identus_did::{
    DereferencingOptions, Did, DidResolutionResult, DidUrl, DidUrlDereferencingResult,
    ResolutionOptions,
};
use serde_json::Value;
use wasm_bindgen::prelude::*;

use super::{DidParseError, contains_unsafe_javascript_integer};

const SUCCESS_KIND: &str = "success";
const ERROR_KIND: &str = "error";
const DEACTIVATED_KIND: &str = "deactivated";

/// Owned normalized request for one DID resolution host effect.
#[derive(Debug)]
#[wasm_bindgen]
pub struct DidResolutionRequestView {
    did: String,
    method: String,
    method_specific_id: String,
    options: String,
}

#[wasm_bindgen]
impl DidResolutionRequestView {
    /// Exact validated DID.
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

    /// Deterministic normalized resolution options JSON.
    #[wasm_bindgen(getter)]
    pub fn options(&self) -> String {
        self.options.clone()
    }
}

/// Owned normalized request for one DID URL dereferencing host effect.
#[derive(Debug)]
#[wasm_bindgen]
pub struct DidDereferencingRequestView {
    did_url: String,
    did: String,
    method: String,
    method_specific_id: String,
    path: String,
    query: Option<String>,
    fragment: Option<String>,
    options: String,
}

#[wasm_bindgen]
impl DidDereferencingRequestView {
    /// Exact validated DID URL.
    #[wasm_bindgen(getter, js_name = didUrl)]
    pub fn did_url(&self) -> String {
        self.did_url.clone()
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

    /// Deterministic normalized dereferencing options JSON.
    #[wasm_bindgen(getter)]
    pub fn options(&self) -> String {
        self.options.clone()
    }
}

/// Owned normalized DID resolution result.
#[derive(Debug)]
#[wasm_bindgen]
pub struct DidResolutionResultView {
    serialized: String,
    kind: String,
}

#[wasm_bindgen]
impl DidResolutionResultView {
    /// Deterministic normalized W3C DID Resolution result JSON.
    #[wasm_bindgen(getter)]
    pub fn serialized(&self) -> String {
        self.serialized.clone()
    }

    /// Closed result branch: `success`, `error`, or `deactivated`.
    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> String {
        self.kind.clone()
    }
}

/// Owned normalized DID URL dereferencing result.
#[derive(Debug)]
#[wasm_bindgen]
pub struct DidDereferencingResultView {
    serialized: String,
    kind: String,
}

#[wasm_bindgen]
impl DidDereferencingResultView {
    /// Deterministic normalized W3C DID URL Dereferencing result JSON.
    #[wasm_bindgen(getter)]
    pub fn serialized(&self) -> String {
        self.serialized.clone()
    }

    /// Closed result branch: `success` or `error`.
    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> String {
        self.kind.clone()
    }
}

/// Validate and normalize one DID resolution request before host invocation.
#[wasm_bindgen(js_name = prepareDidResolution)]
pub fn prepare_did_resolution(
    did: &str,
    options_json: &str,
) -> Result<DidResolutionRequestView, DidParseError> {
    let did = Did::parse(did).map_err(|_| DidParseError::invalid_did())?;
    let options = ResolutionOptions::from_json_str(options_json)
        .map_err(|_| DidParseError::invalid_resolution_options())?;
    let options = serialize_checked(
        serde_json::to_string(&options),
        DidParseError::invalid_resolution_options,
    )?;
    Ok(DidResolutionRequestView {
        did: did.as_str().to_owned(),
        method: did.method().to_owned(),
        method_specific_id: did.method_specific_id().to_owned(),
        options,
    })
}

/// Validate and normalize one DID URL dereferencing request before host invocation.
#[wasm_bindgen(js_name = prepareDidDereferencing)]
pub fn prepare_did_dereferencing(
    did_url: &str,
    options_json: &str,
) -> Result<DidDereferencingRequestView, DidParseError> {
    let did_url = DidUrl::parse(did_url).map_err(|_| DidParseError::invalid_did_url())?;
    let options = DereferencingOptions::from_json_str(options_json)
        .map_err(|_| DidParseError::invalid_dereferencing_options())?;
    let options = serialize_checked(
        serde_json::to_string(&options),
        DidParseError::invalid_dereferencing_options,
    )?;
    Ok(DidDereferencingRequestView {
        did_url: did_url.as_str().to_owned(),
        did: did_url.as_did_str().to_owned(),
        method: did_url.method().to_owned(),
        method_specific_id: did_url.method_specific_id().to_owned(),
        path: did_url.path().to_owned(),
        query: did_url.query().map(str::to_owned),
        fragment: did_url.fragment().map(str::to_owned),
        options,
    })
}

/// Validate, correlate, and normalize raw DID resolution result JSON.
#[wasm_bindgen(js_name = parseDidResolutionResult)]
pub fn parse_did_resolution_result(
    did: &str,
    result_json: &str,
) -> Result<DidResolutionResultView, DidParseError> {
    let did = Did::parse(did).map_err(|_| DidParseError::invalid_did())?;
    let result = DidResolutionResult::from_json_str_for(result_json, &did)
        .map_err(|_| DidParseError::invalid_resolution_result())?;
    let kind = if result.document().is_some() {
        SUCCESS_KIND
    } else if result.metadata().error().is_some() {
        ERROR_KIND
    } else {
        DEACTIVATED_KIND
    };
    let serialized = serialize_checked(
        serde_json::to_string(&result),
        DidParseError::invalid_resolution_result,
    )?;
    Ok(DidResolutionResultView {
        serialized,
        kind: kind.to_owned(),
    })
}

/// Validate and normalize raw DID URL dereferencing result JSON.
#[wasm_bindgen(js_name = parseDidDereferencingResult)]
pub fn parse_did_dereferencing_result(
    result_json: &str,
) -> Result<DidDereferencingResultView, DidParseError> {
    let result = DidUrlDereferencingResult::from_json_str(result_json)
        .map_err(|_| DidParseError::invalid_dereferencing_result())?;
    let kind = if result.content().is_some() {
        SUCCESS_KIND
    } else {
        ERROR_KIND
    };
    let serialized = serialize_checked(
        serde_json::to_string(&result),
        DidParseError::invalid_dereferencing_result,
    )?;
    Ok(DidDereferencingResultView {
        serialized,
        kind: kind.to_owned(),
    })
}

fn serialize_checked(
    serialized: Result<String, serde_json::Error>,
    invalid: fn() -> DidParseError,
) -> Result<String, DidParseError> {
    let serialized = serialized.map_err(|_| invalid())?;
    let normalized: Value = serde_json::from_str(&serialized).map_err(|_| invalid())?;
    if contains_unsafe_javascript_integer(&normalized) {
        return Err(DidParseError::unsupported_lossy_mapping());
    }
    Ok(serialized)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DID: &str = "did:example:123";
    const DID_URL: &str = "did:example:123#key-1";

    #[test]
    fn resolution_request_is_validated_and_normalized() {
        let request = prepare_did_resolution(
            DID,
            r#"{"versionId":"v1","noCache":false,"network":{"height":7}}"#,
        )
        .unwrap();

        assert_eq!(request.did(), DID);
        assert_eq!(request.method(), "example");
        assert_eq!(request.method_specific_id(), "123");
        assert_eq!(
            request.options(),
            r#"{"noCache":false,"versionId":"v1","network":{"height":7}}"#
        );

        let error = prepare_did_resolution(DID, r#"{"noCache":"false"}"#).unwrap_err();
        assert_eq!(error.code(), super::super::INVALID_RESOLUTION_OPTIONS_CODE);
    }

    #[test]
    fn dereferencing_request_is_validated_and_normalized() {
        let request = prepare_did_dereferencing(
            DID_URL,
            r#"{"verificationRelationship":"authentication","proofPurpose":"issuer"}"#,
        )
        .unwrap();

        assert_eq!(request.did_url(), DID_URL);
        assert_eq!(request.did(), DID);
        assert_eq!(request.method(), "example");
        assert_eq!(request.method_specific_id(), "123");
        assert_eq!(request.fragment().as_deref(), Some("key-1"));
        assert_eq!(
            request.options(),
            r#"{"verificationRelationship":"authentication","proofPurpose":"issuer"}"#
        );

        let error =
            prepare_did_dereferencing(DID_URL, r#"{"verificationRelationship":" authentication"}"#)
                .unwrap_err();
        assert_eq!(
            error.code(),
            super::super::INVALID_DEREFERENCING_OPTIONS_CODE
        );
    }

    #[test]
    fn resolution_success_is_correlated_and_normalized() {
        let input = r#"{
            "didResolutionMetadata":{"contentType":"application/did+json"},
            "didDocument":{"id":"did:example:123"},
            "didDocumentMetadata":{"versionId":"v1"}
        }"#;
        let view = parse_did_resolution_result(DID, input).unwrap();

        assert_eq!(view.kind(), SUCCESS_KIND);
        let reparsed =
            DidResolutionResult::from_json_str_for(&view.serialized(), &Did::parse(DID).unwrap())
                .unwrap();
        assert_eq!(reparsed.document().unwrap().id().as_str(), DID);

        let substituted = input.replace(DID, "did:example:substituted");
        let error = parse_did_resolution_result(DID, &substituted).unwrap_err();
        assert_eq!(error.code(), super::super::INVALID_RESOLUTION_RESULT_CODE);
    }

    #[test]
    fn resolution_error_and_deactivation_states_remain_distinct() {
        let error = parse_did_resolution_result(
            DID,
            r#"{"didResolutionMetadata":{"error":{"type":"https://www.w3.org/ns/did#NOT_FOUND"}},"didDocument":null,"didDocumentMetadata":{}}"#,
        )
        .unwrap();
        assert_eq!(error.kind(), ERROR_KIND);

        let deactivated = parse_did_resolution_result(
            DID,
            r#"{"didResolutionMetadata":{},"didDocument":null,"didDocumentMetadata":{"deactivated":true}}"#,
        )
        .unwrap();
        assert_eq!(deactivated.kind(), DEACTIVATED_KIND);
    }

    #[test]
    fn raw_resolution_result_rejects_duplicates_and_unsafe_numbers() {
        let marker = "do-not-reflect-resolution";
        let duplicate = format!(
            r#"{{"didResolutionMetadata":{{"{marker}":1,"{marker}":2}},"didDocument":{{"id":"{DID}"}},"didDocumentMetadata":{{}}}}"#
        );
        let error = parse_did_resolution_result(DID, &duplicate).unwrap_err();
        assert_eq!(error.code(), super::super::INVALID_RESOLUTION_RESULT_CODE);
        assert!(!error.code().contains(marker));

        let unsafe_integer = format!(
            r#"{{"didResolutionMetadata":{{"height":9007199254740992}},"didDocument":{{"id":"{DID}"}},"didDocumentMetadata":{{}}}}"#
        );
        let error = parse_did_resolution_result(DID, &unsafe_integer).unwrap_err();
        assert_eq!(error.code(), super::super::UNSUPPORTED_LOSSY_MAPPING_CODE);
    }

    #[test]
    fn dereferencing_success_and_failure_states_are_normalized() {
        let success = parse_did_dereferencing_result(
            r#"{"didUrlDereferencingMetadata":{"contentType":"application/did+json"},"content":{"id":"did:example:123"},"contentMetadata":{"versionId":"v1"}}"#,
        )
        .unwrap();
        assert_eq!(success.kind(), SUCCESS_KIND);
        assert!(DidUrlDereferencingResult::from_json_str(&success.serialized()).is_ok());

        let failure = parse_did_dereferencing_result(
            r#"{"didUrlDereferencingMetadata":{"error":{"type":"https://www.w3.org/ns/did#NOT_FOUND"}},"content":null,"contentMetadata":{}}"#,
        )
        .unwrap();
        assert_eq!(failure.kind(), ERROR_KIND);

        let unsafe_integer = parse_did_dereferencing_result(
            r#"{"didUrlDereferencingMetadata":{},"content":{"height":9007199254740992},"contentMetadata":{}}"#,
        )
        .unwrap_err();
        assert_eq!(
            unsafe_integer.code(),
            super::super::UNSUPPORTED_LOSSY_MAPPING_CODE
        );
    }
}
