#![cfg(target_arch = "wasm32")]

use identus_did::{MAX_DID_BYTES, MAX_DID_URL_BYTES};
use identus_wasm_did::{binding_api_version, parse_did, parse_did_url};
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen(inline_js = "
    export function consumeDid(value) {
        try {
            return `${value.value}|${value.method}|${value.methodSpecificId}`;
        } finally {
            value.free();
        }
    }

    export function consumeError(error) {
        try {
            return `${error.constructor.name}|${error.code}`;
        } finally {
            error.free();
        }
    }
")]
extern "C" {
    #[wasm_bindgen(js_name = consumeDid)]
    fn consume_did(value: JsValue) -> String;

    #[wasm_bindgen(js_name = consumeError)]
    fn consume_error(error: JsValue) -> String;
}

#[wasm_bindgen_test]
fn browser_executes_version_and_component_contract() {
    assert_eq!(binding_api_version(), 1);
    let did = parse_did("did:example:browser").unwrap();
    assert_eq!(did.value(), "did:example:browser");
    assert_eq!(did.method(), "example");
    assert_eq!(did.method_specific_id(), "browser");

    let did_for_js = parse_did("did:example:javascript").unwrap();
    assert_eq!(
        consume_did(JsValue::from(did_for_js)),
        "did:example:javascript|example|javascript"
    );

    let url = parse_did_url("did:example:browser/path?service=agent#key-1").unwrap();
    assert_eq!(url.did(), "did:example:browser");
    assert_eq!(url.path(), "/path");
    assert_eq!(url.query().as_deref(), Some("service=agent"));
    assert_eq!(url.fragment().as_deref(), Some("key-1"));
}

#[wasm_bindgen_test]
fn browser_rejects_bounded_input_without_reflection() {
    let canary = "browser-canary-do-not-reflect";
    let invalid = parse_did(&format!("did:EXAMPLE:{canary}")).unwrap_err();
    assert_eq!(invalid.code(), "did.invalid_did");
    assert!(!invalid.code().contains(canary));

    let invalid_for_js = parse_did(&format!("did:EXAMPLE:{canary}")).unwrap_err();
    assert_eq!(
        consume_error(JsValue::from(invalid_for_js)),
        "DidParseError|did.invalid_did"
    );

    let oversized_did = format!("did:example:{}", "a".repeat(MAX_DID_BYTES));
    assert_eq!(
        parse_did(&oversized_did).unwrap_err().code(),
        "did.invalid_did"
    );

    let oversized_url = format!("did:example:browser/{}", "a".repeat(MAX_DID_URL_BYTES));
    assert_eq!(
        parse_did_url(&oversized_url).unwrap_err().code(),
        "did.invalid_did_url"
    );
}
