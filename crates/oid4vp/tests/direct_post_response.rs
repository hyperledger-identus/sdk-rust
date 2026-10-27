use identus_crypto::{Ed25519PrivateKey, EncodeJwk, PublicKeyJwk};
use identus_jose::{
    Ed25519Signer, JwsAlgorithm, JwsLimits, JwsSigningInput, JwsVerificationKey, ProtectedHeader,
    SignatureSuiteRegistry,
};
use identus_oid4vp::{
    AuthorizationRequestInvocation, AuthorizationRequestInvocationLimits,
    AuthorizationRequestValidationLimits, DIRECT_POST_HTTP_METHOD, DIRECT_POST_MEDIA_TYPE,
    DcqlLimits, DirectPostPresentation, DirectPostQueryResult, DirectPostResponseLimits,
    Oid4vpError, REQUEST_OBJECT_JWT_TYPE, REQUEST_OBJECT_MEDIA_TYPE, RequestObjectValidationLimits,
    RequestUriRetrievalInput, RequestUriRetrievalLimits, ValidatedAuthorizationRequest,
};
use serde_json::{Value, json};

const PRIVATE_BYTES: [u8; 32] = [0x71; 32];
const CLIENT_ID: &str = "decentralized_identifier:did:example:verifier";
const REFERENCE: &str = "openid4vp:?client_id=decentralized_identifier%3Adid%3Aexample%3Averifier&request_uri=https%3A%2F%2Fverifier.example%2Frequest";
const PACKET: &str =
    include_str!("../../../docs/conformance/fixtures/oid4vp-direct-post/v1/vectors.json");

fn private_key() -> Ed25519PrivateKey {
    Ed25519PrivateKey::from_slice(&PRIVATE_BYTES).expect("fixed private key")
}

fn signed(payload: &[u8]) -> String {
    let header = ProtectedHeader::new(
        "Ed25519",
        Some(REQUEST_OBJECT_JWT_TYPE),
        Some("did:example:verifier#key-1"),
        JwsLimits::default(),
    )
    .expect("fixed protected header");
    JwsSigningInput::new(header, payload.to_vec(), JwsLimits::default())
        .expect("bounded input")
        .sign_with(&Ed25519Signer::new(&private_key()))
        .expect("fixed signature")
        .compact()
        .to_owned()
}

fn bind_verified(compact: &str, key: &PublicKeyJwk) -> ValidatedAuthorizationRequest {
    let AuthorizationRequestInvocation::Referenced(reference) =
        AuthorizationRequestInvocation::parse(
            REFERENCE,
            AuthorizationRequestInvocationLimits::default(),
        )
        .expect("fixed invocation");
    let unverified = reference
        .prepare_retrieval(
            RequestUriRetrievalInput::get(),
            RequestUriRetrievalLimits::default(),
        )
        .expect("retrieval")
        .bind_response(
            200,
            Some(REQUEST_OBJECT_MEDIA_TYPE),
            compact.as_bytes(),
            RequestObjectValidationLimits::default(),
        )
        .expect("bound request object");
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, key).expect("bound key");
    unverified
        .verify(&SignatureSuiteRegistry::recommended(), &key)
        .expect("verified request object")
        .into_authorization_request(
            AuthorizationRequestValidationLimits::default(),
            DcqlLimits::default(),
        )
        .expect("validated authorization request")
}

fn validated(request: Value) -> ValidatedAuthorizationRequest {
    let payload = serde_json::to_vec(&request).expect("fixed JSON");
    let compact = signed(&payload);
    bind_verified(&compact, &private_key().to_public_key().encode_jwk())
}

fn request(dcql_query: Value) -> Value {
    json!({
        "client_id": CLIENT_ID,
        "response_type": "vp_token",
        "response_mode": "direct_post",
        "nonce": "fresh-nonce_1",
        "response_uri": "https://verifier.example/response",
        "state": "state-123",
        "dcql_query": dcql_query
    })
}

fn credential(id: &str, multiple: bool) -> Value {
    json!({
        "id": id,
        "format": "dc+sd-jwt",
        "multiple": multiple,
        "meta": {}
    })
}

fn string_result(id: &str, values: &[&str]) -> DirectPostQueryResult {
    let limits = DirectPostResponseLimits::default();
    let presentations = values
        .iter()
        .map(|value| DirectPostPresentation::string(value, limits).expect("presentation"))
        .collect();
    DirectPostQueryResult::new(id, presentations, limits).expect("query result")
}

#[test]
fn packet_executes_full_invocation_jar_dcql_and_response_lineage() {
    let packet: Value = serde_json::from_str(PACKET).expect("packet");
    let case = &packet["cases"][0];
    let literal: Value =
        serde_json::from_str(case["input"]["literal"].as_str().expect("literal")).expect("input");
    assert_eq!(literal["outer_client_id"], CLIENT_ID);
    let key: PublicKeyJwk =
        serde_json::from_value(literal["verification_jwk"].clone()).expect("public JWK");
    let validated = bind_verified(literal["request_object"].as_str().expect("compact"), &key);
    let presentation = literal["results"]["degree"][0]
        .as_str()
        .expect("string presentation");
    let response = validated
        .into_direct_post_response(
            vec![string_result("degree", &[presentation])],
            DirectPostResponseLimits::default(),
        )
        .expect("direct-post response");

    assert_eq!(response.http_method(), DIRECT_POST_HTTP_METHOD);
    assert_eq!(response.media_type(), DIRECT_POST_MEDIA_TYPE);
    assert_eq!(
        response.expose_sensitive_target(),
        "https://verifier.example/response"
    );
    assert_eq!(
        response.expose_sensitive_form_body(),
        case["expected"]["fields"]["serialized"]
            .as_str()
            .expect("expected form")
    );
    assert!(response.state_present());
    assert_eq!(response.query_result_count(), 1);
}

#[test]
fn object_presentations_preserve_type_and_query_order() {
    let validated = validated(request(json!({
        "credentials": [credential("first", false), credential("second", false)]
    })));
    let limits = DirectPostResponseLimits::default();
    let object =
        DirectPostPresentation::object_json(br#"{ "proof": {"z": 1}, "vp": true }"#, limits)
            .expect("object presentation");
    let first = DirectPostQueryResult::new("first", vec![object], limits).expect("first");
    let response = validated
        .into_direct_post_response(vec![string_result("second", &["compact"]), first], limits)
        .expect("response");
    assert_eq!(
        response.expose_sensitive_form_body(),
        "vp_token=%7B%22first%22%3A%5B%7B%22proof%22%3A%7B%22z%22%3A1%7D%2C%22vp%22%3Atrue%7D%5D%2C%22second%22%3A%5B%22compact%22%5D%7D&state=state-123"
    );
}

#[test]
fn required_alternatives_optional_sets_and_multiple_cardinality_are_enforced() {
    let query = json!({
        "credentials": [
            credential("pid", false),
            credential("mdl", false),
            credential("extras", true)
        ],
        "credential_sets": [
            {"options": [["pid"], ["mdl"]]},
            {"options": [["extras"]], "required": false}
        ]
    });
    validated(request(query.clone()))
        .into_direct_post_response(
            vec![string_result("pid", &["one"])],
            DirectPostResponseLimits::default(),
        )
        .expect("one required alternative is enough");

    let missing = validated(request(query.clone()))
        .into_direct_post_response(Vec::new(), DirectPostResponseLimits::default())
        .unwrap_err();
    assert_eq!(missing, Oid4vpError::MissingDcqlResult);

    let cardinality = validated(request(query))
        .into_direct_post_response(
            vec![string_result("pid", &["one", "two"])],
            DirectPostResponseLimits::default(),
        )
        .unwrap_err();
    assert_eq!(cardinality, Oid4vpError::DcqlCardinalityMismatch);
}

#[test]
fn unknown_duplicate_and_absent_all_query_results_are_rejected() {
    let query = json!({
        "credentials": [credential("pid", false), credential("mdl", false)]
    });
    let unknown = validated(request(query.clone()))
        .into_direct_post_response(
            vec![string_result("unknown", &["one"])],
            DirectPostResponseLimits::default(),
        )
        .unwrap_err();
    assert_eq!(unknown, Oid4vpError::UnknownDcqlResult);

    let duplicate = validated(request(query.clone()))
        .into_direct_post_response(
            vec![
                string_result("pid", &["one"]),
                string_result("pid", &["two"]),
            ],
            DirectPostResponseLimits::default(),
        )
        .unwrap_err();
    assert_eq!(duplicate, Oid4vpError::DuplicateDcqlResult);

    let missing = validated(request(query))
        .into_direct_post_response(
            vec![string_result("pid", &["one"])],
            DirectPostResponseLimits::default(),
        )
        .unwrap_err();
    assert_eq!(missing, Oid4vpError::MissingDcqlResult);
}

#[test]
fn presentation_and_aggregate_limits_fail_before_returning_an_envelope() {
    let limits = DirectPostResponseLimits::new(1, 8, 1, 8, 2, 2, 8, 32).unwrap();
    assert_eq!(
        DirectPostPresentation::string("12345678", limits).unwrap_err(),
        Oid4vpError::PresentationTooLarge
    );
    assert_eq!(
        DirectPostPresentation::object_json(br#"{"a":{"b":1}}"#, limits).unwrap_err(),
        Oid4vpError::PresentationTooLarge
    );

    let validated = validated(request(json!({
        "credentials": [credential("pid", false)]
    })));
    let presentation = DirectPostPresentation::string("ok", limits).expect("presentation");
    let result = DirectPostQueryResult::new("pid", vec![presentation], limits).expect("result");
    assert_eq!(
        validated
            .into_direct_post_response(vec![result], limits)
            .unwrap_err(),
        Oid4vpError::ResponseJsonTooLarge
    );
}

#[test]
fn response_rechecks_values_created_under_a_different_limit_policy() {
    let broad = DirectPostResponseLimits::default();
    let narrow = DirectPostResponseLimits::new(1, 8, 1, 4, 2, 2, 64, 256).unwrap();
    let presentation = DirectPostPresentation::string("five!", broad).unwrap();
    let result = DirectPostQueryResult::new("pid", vec![presentation], broad).unwrap();
    let validated = validated(request(json!({
        "credentials": [credential("pid", false)]
    })));
    assert_eq!(
        validated
            .into_direct_post_response(vec![result], narrow)
            .unwrap_err(),
        Oid4vpError::PresentationTooLarge
    );
}

#[test]
fn response_limit_policy_requires_every_positive_budget() {
    let valid = [1; 8];
    for zero in 0..valid.len() {
        let mut values = valid;
        values[zero] = 0;
        assert_eq!(
            DirectPostResponseLimits::new(
                values[0], values[1], values[2], values[3], values[4], values[5], values[6],
                values[7]
            )
            .unwrap_err(),
            Oid4vpError::InvalidLimits
        );
    }
}

#[test]
fn diagnostics_do_not_expose_presentation_or_routing_values() {
    const CANARY: &str = "VERIFIER_SECRET_CANARY";
    let limits = DirectPostResponseLimits::default();
    let presentation = DirectPostPresentation::string(CANARY, limits).expect("presentation");
    let result = DirectPostQueryResult::new(CANARY, vec![presentation], limits).expect("result");
    let request = validated(request(json!({
        "credentials": [credential(CANARY, false)]
    })));
    let response = request
        .into_direct_post_response(vec![result], limits)
        .expect("response");
    let rendered = format!("{response:?}");
    assert!(!rendered.contains(CANARY));
    assert!(!rendered.contains("verifier.example"));
}

#[test]
fn direct_post_property_generated_matrix_round_trips() {
    let limits = DirectPostResponseLimits::default();
    for state in ["s", "space state", "+&%"] {
        for multiple in [false, true] {
            for count in 1..=if multiple { 3 } else { 1 } {
                let mut authorization = request(json!({
                    "credentials": [credential("generated", multiple)]
                }));
                authorization["state"] = json!(state);
                let values: Vec<_> = (0..count)
                    .map(|index| format!("value {index} +&%"))
                    .collect();
                let presentations = values
                    .iter()
                    .map(|value| DirectPostPresentation::string(value, limits).unwrap())
                    .collect();
                let result =
                    DirectPostQueryResult::new("generated", presentations, limits).unwrap();
                let response = validated(authorization)
                    .into_direct_post_response(vec![result], limits)
                    .unwrap();
                let decoded = decode_form(response.expose_sensitive_form_body());
                assert_eq!(decoded.get("state").map(String::as_str), Some(state));
                let vp_token: Value =
                    serde_json::from_str(decoded.get("vp_token").expect("vp_token")).unwrap();
                assert_eq!(vp_token["generated"], json!(values));
            }
        }
    }
}

fn decode_form(input: &str) -> std::collections::BTreeMap<String, String> {
    input
        .split('&')
        .map(|pair| {
            let (name, value) = pair.split_once('=').expect("form pair");
            (decode_component(name), decode_component(value))
        })
        .collect()
}

fn decode_component(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut output = Vec::with_capacity(bytes.len());
    let mut cursor = 0;
    while cursor < bytes.len() {
        match bytes[cursor] {
            b'+' => {
                output.push(b' ');
                cursor += 1;
            }
            b'%' => {
                let high = hex(bytes[cursor + 1]);
                let low = hex(bytes[cursor + 2]);
                output.push((high << 4) | low);
                cursor += 3;
            }
            byte => {
                output.push(byte);
                cursor += 1;
            }
        }
    }
    String::from_utf8(output).expect("UTF-8 form component")
}

fn hex(byte: u8) -> u8 {
    match byte {
        b'0'..=b'9' => byte - b'0',
        b'A'..=b'F' => byte - b'A' + 10,
        b'a'..=b'f' => byte - b'a' + 10,
        _ => panic!("invalid form escape"),
    }
}
