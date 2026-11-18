#![cfg(target_arch = "wasm32")]

use identus_did::{MAX_DID_BYTES, MAX_DID_URL_BYTES};
use identus_wasm_did::{
    binding_api_version, create_peer_did_numalgo2_compatibility, create_peer_did_numalgo4,
    parse_did, parse_did_document, parse_did_url, parse_peer_did_numalgo2_compatibility,
    parse_peer_did_numalgo4_short, resolve_peer_did_numalgo4_short,
};
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

    export function consumeDocument(value) {
        try {
            return `${value.id}|${value.serialized}`;
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

    export function consumePeer4(value) {
        try {
            return `${value.profile}|${value.longForm}|${value.shortForm}|${value.inputDocument}|${value.resolvedDocument}`;
        } finally {
            value.free();
        }
    }

    export function consumePeer2(value) {
        try {
            return `${value.profile}|${value.did}|${value.resolvedDocument}`;
        } finally {
            value.free();
        }
    }
")]
extern "C" {
    #[wasm_bindgen(js_name = consumeDid)]
    fn consume_did(value: JsValue) -> String;

    #[wasm_bindgen(js_name = consumeDocument)]
    fn consume_document(value: JsValue) -> String;

    #[wasm_bindgen(js_name = consumeError)]
    fn consume_error(error: JsValue) -> String;

    #[wasm_bindgen(js_name = consumePeer4)]
    fn consume_peer4(value: JsValue) -> String;

    #[wasm_bindgen(js_name = consumePeer2)]
    fn consume_peer2(value: JsValue) -> String;
}

#[wasm_bindgen_test]
fn browser_executes_version_and_component_contract() {
    assert_eq!(binding_api_version(), 4);
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
fn browser_executes_explicit_peer_profiles_and_ownership() {
    let current = create_peer_did_numalgo4(r#"{"hello":"world"}"#).unwrap();
    let short = current.short_form();
    let long = current.long_form();
    assert_eq!(current.profile(), "numalgo4");
    assert_eq!(
        parse_peer_did_numalgo4_short(&short).unwrap().short_form(),
        short
    );
    assert_eq!(
        resolve_peer_did_numalgo4_short(&short, &long)
            .unwrap()
            .resolved_document()
            .contains(&short),
        true
    );
    let consumed = consume_peer4(JsValue::from(current));
    assert!(consumed.starts_with("numalgo4|did:peer:4"));
    assert!(consumed.contains("|{\"hello\":\"world\"}|"));

    const DID: &str = "did:peer:2.Vz6MkiToqovww7vYtxm1xNM15u9JzqzUFZ1k7s7MazYJUyAxv.EzQ3shQLqRUza6AMJFbPuMdvFRFWm1wKviQRnQSC1fScovJN4s.SeyJ0IjoiRElEQ29tbU1lc3NhZ2luZyIsInMiOnsidXJpIjoiaHR0cHM6Ly8xMjcuMC4wLjE6NzAzNyIsImEiOlsiZGlkY29tbS92MiJdLCJyIjpbXX19";
    let compatibility = create_peer_did_numalgo2_compatibility(
        r#"{"keys":[{"purpose":"verification","multibase":"z6MkiToqovww7vYtxm1xNM15u9JzqzUFZ1k7s7MazYJUyAxv"},{"purpose":"encryption","multibase":"zQ3shQLqRUza6AMJFbPuMdvFRFWm1wKviQRnQSC1fScovJN4s"}],"services":[{"type":"DIDCommMessaging","endpoint":{"uri":"https://127.0.0.1:7037","accept":["didcomm/v2"],"routingKeys":[]}}]}"#,
    )
    .unwrap();
    assert_eq!(compatibility.did(), DID);
    assert_eq!(
        parse_peer_did_numalgo2_compatibility(DID).unwrap().did(),
        DID
    );
    let consumed = consume_peer2(JsValue::from(compatibility));
    assert!(consumed.starts_with("numalgo2-compatibility|did:peer:2."));
}

#[wasm_bindgen_test]
fn browser_executes_document_ownership_contract() {
    let document = parse_did_document(
        r#"{
            "id":"did:example:browser-document",
            "controller":"did:example:controller",
            "profile":{"enabled":true}
        }"#,
    )
    .unwrap();
    assert_eq!(document.id(), "did:example:browser-document");
    assert_eq!(
        consume_document(JsValue::from(document)),
        "did:example:browser-document|{\"id\":\"did:example:browser-document\",\"controller\":\"did:example:controller\",\"profile\":{\"enabled\":true}}"
    );
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

    let invalid_document = parse_did_document(&format!(
        r#"{{"id":"did:example:browser","profile":{{"secret":"{canary}"}},"id":"did:example:shadow"}}"#
    ))
    .unwrap_err();
    assert_eq!(invalid_document.code(), "did.invalid_document");
    assert!(!invalid_document.code().contains(canary));

    let invalid_document_for_js =
        parse_did_document(r#"{"id":"did:example:browser","id":"did:example:shadow"}"#)
            .unwrap_err();
    assert_eq!(
        consume_error(JsValue::from(invalid_document_for_js)),
        "DidParseError|did.invalid_document"
    );

    let unsupported =
        parse_did_document(r#"{"id":"did:example:browser","unsafe":9007199254740992}"#)
            .unwrap_err();
    assert_eq!(unsupported.code(), "adapter.unsupported_lossy_mapping");
}
