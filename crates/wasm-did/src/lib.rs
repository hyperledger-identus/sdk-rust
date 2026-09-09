//! Experimental browser WASM facade for bounded DID and DID URL parsing.
//!
//! This unpublished outer boundary exposes only owned public identifier values
//! and stable error codes. `identus-did` remains binding-framework-free.

use identus_did::{Did, DidUrl};
use wasm_bindgen::prelude::*;

/// Version of the browser JavaScript value and error contract.
pub const BINDING_API_VERSION: u32 = 1;

const INVALID_DID_CODE: &str = "did.invalid_did";
const INVALID_DID_URL_CODE: &str = "did.invalid_did_url";

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

/// Closed browser failure carrying only a stable SDK-owned code.
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
    fn invalid_did() -> Self {
        Self {
            code: INVALID_DID_CODE.to_owned(),
        }
    }

    fn invalid_did_url() -> Self {
        Self {
            code: INVALID_DID_URL_CODE.to_owned(),
        }
    }
}

/// Return the browser binding contract version.
#[wasm_bindgen(js_name = bindingApiVersion)]
pub fn binding_api_version() -> u32 {
    BINDING_API_VERSION
}

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

#[cfg(test)]
mod tests {
    use identus_did::{MAX_DID_BYTES, MAX_DID_URL_BYTES};

    use super::*;

    #[test]
    fn version_and_component_views_are_stable() {
        assert_eq!(binding_api_version(), 1);

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
}
