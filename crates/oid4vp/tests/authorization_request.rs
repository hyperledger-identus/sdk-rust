use identus_crypto::{Ed25519PrivateKey, EncodeJwk, PublicKeyJwk};
use identus_jose::{
    Ed25519Signer, JwsAlgorithm, JwsLimits, JwsSigningInput, JwsVerificationKey, ProtectedHeader,
    SignatureSuiteRegistry,
};
use identus_oid4vp::{
    AuthorizationRequestInvocation, AuthorizationRequestInvocationLimits,
    AuthorizationRequestValidationLimits, AuthorizationResponseMode, AuthorizationResponseType,
    DcqlLimits, Oid4vpError, REQUEST_OBJECT_JWT_TYPE, REQUEST_OBJECT_MEDIA_TYPE,
    RequestObjectValidationLimits, RequestUriRetrievalInput, RequestUriRetrievalLimits,
    ValidatedAuthorizationRequest,
};
use serde_json::{Value, json};

const PRIVATE_BYTES: [u8; 32] = [0x71; 32];
const CLIENT_ID: &str = "decentralized_identifier:did:example:verifier";
const REFERENCE: &str = "openid4vp:?client_id=decentralized_identifier%3Adid%3Aexample%3Averifier&request_uri=https%3A%2F%2Fverifier.example%2Frequest";
const CANARY: &str = "VERIFIER_SECRET_CANARY";

fn private_key() -> Ed25519PrivateKey {
    Ed25519PrivateKey::from_slice(&PRIVATE_BYTES).expect("fixed private key")
}

fn public_key() -> PublicKeyJwk {
    private_key().to_public_key().encode_jwk()
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

fn valid_request() -> Value {
    json!({
        "client_id": CLIENT_ID,
        "response_type": "vp_token",
        "response_mode": "direct_post",
        "nonce": "fresh-nonce_1",
        "response_uri": "https://verifier.example/response",
        "dcql_query": {
            "credentials": [{
                "id": "pid",
                "format": "dc+sd-jwt",
                "meta": {}
            }]
        }
    })
}

fn verified_request(request: &Value) -> identus_oid4vp::VerifiedRequestObject {
    let payload = serde_json::to_vec(request).expect("fixed JSON");
    let compact = signed(&payload);
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
    let public = public_key();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).expect("bound key");
    unverified
        .verify(&SignatureSuiteRegistry::recommended(), &key)
        .expect("verified request object")
}

fn validate(
    request: &Value,
    routing_limits: AuthorizationRequestValidationLimits,
) -> Result<ValidatedAuthorizationRequest, Oid4vpError> {
    verified_request(request).into_authorization_request(routing_limits, DcqlLimits::default())
}

#[test]
fn valid_direct_post_request_preserves_composed_evidence() {
    let request = validate(
        &valid_request(),
        AuthorizationRequestValidationLimits::default(),
    )
    .expect("valid authorization request");

    assert_eq!(request.response_type(), AuthorizationResponseType::VpToken);
    assert_eq!(
        request.response_mode(),
        AuthorizationResponseMode::DirectPost
    );
    assert_eq!(request.algorithm(), JwsAlgorithm::Ed25519);
    assert_eq!(
        request.protected_header().key_id(),
        Some("did:example:verifier#key-1")
    );
    assert_eq!(request.expose_sensitive_client_id(), CLIENT_ID);
    assert_eq!(request.expose_sensitive_wallet_nonce(), None);
    assert_eq!(request.expose_sensitive_nonce(), "fresh-nonce_1");
    assert_eq!(
        request.expose_sensitive_response_uri(),
        "https://verifier.example/response"
    );
    assert_eq!(request.dcql_query().credential_query_count(), 1);
}

#[test]
fn unsupported_and_missing_response_fields_are_explicit() {
    let mut request = valid_request();
    request.as_object_mut().unwrap().remove("response_type");
    assert_eq!(
        validate(&request, AuthorizationRequestValidationLimits::default()).unwrap_err(),
        Oid4vpError::MissingResponseType
    );

    let mut request = valid_request();
    request["response_type"] = json!("code");
    assert_eq!(
        validate(&request, AuthorizationRequestValidationLimits::default()).unwrap_err(),
        Oid4vpError::UnsupportedResponseType
    );

    let mut request = valid_request();
    request.as_object_mut().unwrap().remove("response_mode");
    assert_eq!(
        validate(&request, AuthorizationRequestValidationLimits::default()).unwrap_err(),
        Oid4vpError::MissingResponseMode
    );

    let mut request = valid_request();
    request["response_mode"] = json!("direct_post.jwt");
    assert_eq!(
        validate(&request, AuthorizationRequestValidationLimits::default()).unwrap_err(),
        Oid4vpError::UnsupportedResponseMode
    );
}

#[test]
fn authorization_nonce_uses_final_grammar_and_its_own_limit() {
    let mut request = valid_request();
    request.as_object_mut().unwrap().remove("nonce");
    assert_eq!(
        validate(&request, AuthorizationRequestValidationLimits::default()).unwrap_err(),
        Oid4vpError::MissingAuthorizationNonce
    );

    for nonce in [
        json!(""),
        json!("contains space"),
        json!("non-ascii-💣"),
        json!(7),
    ] {
        let mut request = valid_request();
        request["nonce"] = nonce;
        assert_eq!(
            validate(&request, AuthorizationRequestValidationLimits::default()).unwrap_err(),
            Oid4vpError::InvalidAuthorizationNonce
        );
    }

    let mut request = valid_request();
    request["nonce"] = json!("abc");
    let limits = AuthorizationRequestValidationLimits::new(2, 4_096).unwrap();
    assert_eq!(
        validate(&request, limits).unwrap_err(),
        Oid4vpError::AuthorizationNonceTooLarge
    );
}

#[test]
fn direct_post_destination_is_bounded_and_unambiguous() {
    let mut request = valid_request();
    request.as_object_mut().unwrap().remove("response_uri");
    assert_eq!(
        validate(&request, AuthorizationRequestValidationLimits::default()).unwrap_err(),
        Oid4vpError::MissingResponseUri
    );

    let mut request = valid_request();
    request["redirect_uri"] = json!("https://verifier.example/callback");
    assert_eq!(
        validate(&request, AuthorizationRequestValidationLimits::default()).unwrap_err(),
        Oid4vpError::ConflictingResponseDestination
    );

    let mut request = valid_request();
    request["response_uri"] = json!("https://verifier.example/response");
    let limits = AuthorizationRequestValidationLimits::new(256, 8).unwrap();
    assert_eq!(
        validate(&request, limits).unwrap_err(),
        Oid4vpError::ResponseUriTooLarge
    );
}

#[test]
fn response_uri_rejects_non_https_authority_and_fragment_shapes() {
    for response_uri in [
        "",
        "/relative",
        "http://verifier.example/response",
        "https:///response",
        "https://user@verifier.example/response",
        "https://verifier.example/response#fragment",
    ] {
        let mut request = valid_request();
        request["response_uri"] = json!(response_uri);
        assert_eq!(
            validate(&request, AuthorizationRequestValidationLimits::default()).unwrap_err(),
            Oid4vpError::UnsafeResponseUri,
            "unexpected result for {response_uri}"
        );
    }
}

#[test]
fn validation_order_precedes_dcql_and_is_map_order_independent() {
    let mut request = json!({
        "dcql_query": {"credentials": []},
        "response_uri": "http://unsafe.example",
        "redirect_uri": "https://verifier.example/callback",
        "nonce": "contains space",
        "response_mode": "query",
        "response_type": "code",
        "client_id": CLIENT_ID
    });
    assert_eq!(
        validate(&request, AuthorizationRequestValidationLimits::default()).unwrap_err(),
        Oid4vpError::UnsupportedResponseType
    );

    request["response_type"] = json!("vp_token");
    assert_eq!(
        validate(&request, AuthorizationRequestValidationLimits::default()).unwrap_err(),
        Oid4vpError::UnsupportedResponseMode
    );

    request["response_mode"] = json!("direct_post");
    assert_eq!(
        validate(&request, AuthorizationRequestValidationLimits::default()).unwrap_err(),
        Oid4vpError::InvalidAuthorizationNonce
    );

    request["nonce"] = json!("valid-nonce");
    assert_eq!(
        validate(&request, AuthorizationRequestValidationLimits::default()).unwrap_err(),
        Oid4vpError::ConflictingResponseDestination
    );

    request.as_object_mut().unwrap().remove("redirect_uri");
    assert_eq!(
        validate(&request, AuthorizationRequestValidationLimits::default()).unwrap_err(),
        Oid4vpError::UnsafeResponseUri
    );

    request["response_uri"] = json!("https://verifier.example/response");
    assert_eq!(
        validate(&request, AuthorizationRequestValidationLimits::default()).unwrap_err(),
        Oid4vpError::InvalidDcqlQuery
    );
}

#[test]
fn query_only_transition_remains_compatible_without_routing_fields() {
    let request = json!({
        "client_id": CLIENT_ID,
        "dcql_query": {
            "credentials": [{"id": "pid", "format": "dc+sd-jwt", "meta": {}}]
        }
    });
    let query = verified_request(&request)
        .into_dcql_query(DcqlLimits::default())
        .expect("existing query-only transition");
    assert_eq!(query.credential_query_count(), 1);
}

#[test]
fn debug_and_errors_do_not_disclose_routing_values() {
    let mut request = valid_request();
    request["nonce"] = json!(CANARY);
    request["response_uri"] = json!(format!("https://verifier.example/{CANARY}"));
    request["dcql_query"]["credentials"][0]["id"] = json!(CANARY);
    let validated = validate(&request, AuthorizationRequestValidationLimits::default())
        .expect("valid canary-bearing request");
    assert!(!format!("{validated:?}").contains(CANARY));

    let mut invalid = valid_request();
    invalid["nonce"] = json!(format!("{CANARY} value"));
    let error = validate(&invalid, AuthorizationRequestValidationLimits::default()).unwrap_err();
    assert!(!format!("{error}").contains(CANARY));
    assert!(!format!("{error:?}").contains(CANARY));
}

#[test]
fn routing_limits_require_positive_values() {
    assert_eq!(
        AuthorizationRequestValidationLimits::new(0, 1),
        Err(Oid4vpError::InvalidLimits)
    );
    assert_eq!(
        AuthorizationRequestValidationLimits::new(1, 0),
        Err(Oid4vpError::InvalidLimits)
    );
}
