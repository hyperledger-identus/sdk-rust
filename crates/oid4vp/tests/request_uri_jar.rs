use identus_crypto::{Ed25519PrivateKey, EncodeJwk, PublicKeyJwk};
use identus_jose::{
    Ed25519Signer, JwsAlgorithm, JwsLimits, JwsSigningInput, JwsVerificationKey, ProtectedHeader,
    SignatureSuiteRegistry,
};
use identus_oid4vp::{
    AuthorizationRequestInvocation, AuthorizationRequestInvocationLimits, FORM_MEDIA_TYPE,
    Oid4vpError, REQUEST_OBJECT_JWT_TYPE, REQUEST_OBJECT_MEDIA_TYPE,
    ReferencedAuthorizationRequest, RequestObjectValidationLimits, RequestUriMethod,
    RequestUriRetrievalInput, RequestUriRetrievalLimits,
};

const PRIVATE_BYTES: [u8; 32] = [0x41; 32];
const OTHER_PRIVATE_BYTES: [u8; 32] = [0x52; 32];
const CLIENT_ID: &str = "decentralized_identifier:did:example:verifier";
const REFERENCE: &str = "openid4vp:?client_id=decentralized_identifier%3Adid%3Aexample%3Averifier&request_uri=https%3A%2F%2Fverifier.example%2Frequest";
const CANARY: &str = "VERIFIER_SECRET_CANARY";

fn reference(method: RequestUriMethod) -> ReferencedAuthorizationRequest {
    let suffix = match method {
        RequestUriMethod::Get => "",
        RequestUriMethod::Post => "&request_uri_method=post",
    };
    let AuthorizationRequestInvocation::Referenced(request) =
        AuthorizationRequestInvocation::parse(
            &format!("{REFERENCE}{suffix}"),
            AuthorizationRequestInvocationLimits::default(),
        )
        .expect("reference invocation");
    request
}

fn private_key(bytes: &[u8; 32]) -> Ed25519PrivateKey {
    Ed25519PrivateKey::from_slice(bytes).expect("Ed25519 private key")
}

fn public_key(bytes: &[u8; 32]) -> PublicKeyJwk {
    private_key(bytes).to_public_key().encode_jwk()
}

fn signed(payload: &str, type_: Option<&str>) -> String {
    let private = private_key(&PRIVATE_BYTES);
    let header = ProtectedHeader::new(
        "Ed25519",
        type_,
        Some("did:example:verifier#key-1"),
        JwsLimits::default(),
    )
    .expect("protected header");
    JwsSigningInput::new(header, payload.as_bytes().to_vec(), JwsLimits::default())
        .expect("signing input")
        .sign_with(&Ed25519Signer::new(&private))
        .expect("signed compact")
        .compact()
        .to_owned()
}

fn verify(
    unverified: identus_oid4vp::UnverifiedRequestObject,
) -> Result<identus_oid4vp::VerifiedRequestObject, Oid4vpError> {
    let public = public_key(&PRIVATE_BYTES);
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).expect("algorithm-bound key");
    unverified.verify(&SignatureSuiteRegistry::recommended(), &key)
}

#[test]
fn get_is_runtime_neutral_and_binds_a_signed_request_object() {
    let request = reference(RequestUriMethod::Get)
        .prepare_retrieval(
            RequestUriRetrievalInput::get(),
            RequestUriRetrievalLimits::default(),
        )
        .expect("GET request");
    assert_eq!(request.method(), RequestUriMethod::Get);
    assert_eq!(request.accept(), REQUEST_OBJECT_MEDIA_TYPE);
    assert_eq!(request.content_type(), None);
    assert_eq!(request.expose_sensitive_body(), b"");
    assert_eq!(
        request.expose_sensitive_endpoint(),
        "https://verifier.example/request"
    );

    let payload = format!(r#"{{"client_id":"{CLIENT_ID}","response_type":"vp_token"}}"#);
    let compact = signed(&payload, Some(REQUEST_OBJECT_JWT_TYPE));
    let unverified = request
        .bind_response(
            200,
            Some("Application/OAuth-Authz-Req+JWT"),
            compact.as_bytes(),
            RequestObjectValidationLimits::default(),
        )
        .expect("bound response");
    assert_eq!(unverified.protected_header().algorithm(), "Ed25519");
    assert_eq!(
        unverified.protected_header().key_id(),
        Some("did:example:verifier#key-1")
    );
    assert_eq!(unverified.expose_sensitive_outer_client_id(), CLIENT_ID);

    let verified = verify(unverified).expect("verified request object");
    assert_eq!(verified.algorithm(), JwsAlgorithm::Ed25519);
    assert_eq!(verified.expose_sensitive_client_id(), CLIENT_ID);
    assert_eq!(verified.expose_sensitive_wallet_nonce(), None);
    assert_eq!(verified.expose_sensitive_payload(), payload.as_bytes());
}

#[test]
fn post_encodes_metadata_and_correlates_the_wallet_nonce() {
    let request = reference(RequestUriMethod::Post)
        .prepare_retrieval(
            RequestUriRetrievalInput::post(
                Some(r#"{"vp_formats_supported":{"dc+sd-jwt":{}}}"#),
                Some("fresh nonce"),
            ),
            RequestUriRetrievalLimits::default(),
        )
        .expect("POST request");
    assert_eq!(request.method(), RequestUriMethod::Post);
    assert_eq!(request.content_type(), Some(FORM_MEDIA_TYPE));
    assert_eq!(
        request.expose_sensitive_body(),
        b"wallet_metadata=%7B%22vp_formats_supported%22%3A%7B%22dc%2Bsd-jwt%22%3A%7B%7D%7D%7D&wallet_nonce=fresh%20nonce"
    );

    let compact = signed(
        &format!(
            r#"{{"client_id":"{CLIENT_ID}","wallet_nonce":"fresh nonce","dcql_query":{{"credentials":[]}}}}"#
        ),
        Some(REQUEST_OBJECT_JWT_TYPE),
    );
    let verified = verify(
        request
            .bind_response(
                201,
                Some(REQUEST_OBJECT_MEDIA_TYPE),
                compact.as_bytes(),
                RequestObjectValidationLimits::default(),
            )
            .expect("bound response"),
    )
    .expect("verified request object");
    assert_eq!(
        verified.expose_sensitive_wallet_nonce(),
        Some("fresh nonce")
    );
}

#[test]
fn retrieval_method_and_outbound_inputs_fail_closed() {
    assert_eq!(
        reference(RequestUriMethod::Get)
            .prepare_retrieval(
                RequestUriRetrievalInput::post(None, None),
                RequestUriRetrievalLimits::default()
            )
            .unwrap_err(),
        Oid4vpError::RetrievalMethodMismatch
    );
    assert_eq!(
        reference(RequestUriMethod::Post)
            .prepare_retrieval(
                RequestUriRetrievalInput::post(Some("[]"), None),
                RequestUriRetrievalLimits::default()
            )
            .unwrap_err(),
        Oid4vpError::InvalidWalletMetadata
    );
    assert_eq!(
        reference(RequestUriMethod::Post)
            .prepare_retrieval(
                RequestUriRetrievalInput::post(None, Some("")),
                RequestUriRetrievalLimits::default()
            )
            .unwrap_err(),
        Oid4vpError::InvalidWalletNonce
    );

    let limits = RequestUriRetrievalLimits::new(2, 2, 64, 64, 64).unwrap();
    assert_eq!(
        reference(RequestUriMethod::Post)
            .prepare_retrieval(RequestUriRetrievalInput::post(Some("{}x"), None), limits)
            .unwrap_err(),
        Oid4vpError::WalletMetadataTooLarge
    );
    assert_eq!(
        reference(RequestUriMethod::Post)
            .prepare_retrieval(RequestUriRetrievalInput::post(None, Some("abc")), limits)
            .unwrap_err(),
        Oid4vpError::WalletNonceTooLarge
    );
    let tiny_body = RequestUriRetrievalLimits::new(8, 8, 1, 64, 64).unwrap();
    assert_eq!(
        reference(RequestUriMethod::Post)
            .prepare_retrieval(RequestUriRetrievalInput::post(None, Some("a")), tiny_body)
            .unwrap_err(),
        Oid4vpError::RequestBodyTooLarge
    );
}

#[test]
fn response_status_media_shape_and_size_are_bound_before_jar_processing() {
    let compact = signed(
        &format!(r#"{{"client_id":"{CLIENT_ID}"}}"#),
        Some(REQUEST_OBJECT_JWT_TYPE),
    );
    let bind = |status, content_type: Option<&str>, body: &[u8], limits| {
        reference(RequestUriMethod::Get)
            .prepare_retrieval(RequestUriRetrievalInput::get(), limits)
            .expect("request")
            .bind_response(
                status,
                content_type,
                body,
                RequestObjectValidationLimits::default(),
            )
            .map(drop)
    };

    assert_eq!(
        bind(
            404,
            Some(REQUEST_OBJECT_MEDIA_TYPE),
            compact.as_bytes(),
            RequestUriRetrievalLimits::default()
        ),
        Err(Oid4vpError::RequestUriHttpError)
    );
    assert_eq!(
        bind(
            200,
            None,
            compact.as_bytes(),
            RequestUriRetrievalLimits::default()
        ),
        Err(Oid4vpError::InvalidRequestObjectMediaType)
    );
    assert_eq!(
        bind(
            200,
            Some("application/jwt"),
            compact.as_bytes(),
            RequestUriRetrievalLimits::default()
        ),
        Err(Oid4vpError::InvalidRequestObjectMediaType)
    );
    assert_eq!(
        bind(
            200,
            Some(REQUEST_OBJECT_MEDIA_TYPE),
            b"",
            RequestUriRetrievalLimits::default()
        ),
        Err(Oid4vpError::EmptyRequestObject)
    );
    assert_eq!(
        bind(
            200,
            Some(REQUEST_OBJECT_MEDIA_TYPE),
            b"a.b.c.d.e",
            RequestUriRetrievalLimits::default()
        ),
        Err(Oid4vpError::UnsupportedEncryptedRequestObject)
    );
    assert_eq!(
        bind(
            200,
            Some(REQUEST_OBJECT_MEDIA_TYPE),
            b"a.b",
            RequestUriRetrievalLimits::default()
        ),
        Err(Oid4vpError::InvalidRequestObject)
    );

    let short_content_type = RequestUriRetrievalLimits::new(1, 1, 1, 3, 1_024).unwrap();
    assert_eq!(
        bind(
            200,
            Some(REQUEST_OBJECT_MEDIA_TYPE),
            compact.as_bytes(),
            short_content_type
        ),
        Err(Oid4vpError::ResponseContentTypeTooLarge)
    );
    let short_body = RequestUriRetrievalLimits::new(1, 1, 1, 64, 3).unwrap();
    assert_eq!(
        bind(
            200,
            Some(REQUEST_OBJECT_MEDIA_TYPE),
            compact.as_bytes(),
            short_body
        ),
        Err(Oid4vpError::ResponseBodyTooLarge)
    );
}

#[test]
fn protected_type_and_signature_are_mandatory() {
    for type_ in [None, Some("JWT"), Some("oauth-authz-req+JWT")] {
        let compact = signed(&format!(r#"{{"client_id":"{CLIENT_ID}"}}"#), type_);
        let error = reference(RequestUriMethod::Get)
            .prepare_retrieval(
                RequestUriRetrievalInput::get(),
                RequestUriRetrievalLimits::default(),
            )
            .unwrap()
            .bind_response(
                200,
                Some(REQUEST_OBJECT_MEDIA_TYPE),
                compact.as_bytes(),
                RequestObjectValidationLimits::default(),
            )
            .unwrap_err();
        assert_eq!(error, Oid4vpError::InvalidRequestObjectType);
    }

    let compact = signed(
        &format!(r#"{{"client_id":"{CLIENT_ID}"}}"#),
        Some(REQUEST_OBJECT_JWT_TYPE),
    );
    let unverified = reference(RequestUriMethod::Get)
        .prepare_retrieval(
            RequestUriRetrievalInput::get(),
            RequestUriRetrievalLimits::default(),
        )
        .unwrap()
        .bind_response(
            200,
            Some(REQUEST_OBJECT_MEDIA_TYPE),
            compact.as_bytes(),
            RequestObjectValidationLimits::default(),
        )
        .unwrap();
    let wrong_public = public_key(&OTHER_PRIVATE_BYTES);
    let wrong_key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &wrong_public).unwrap();
    assert_eq!(
        unverified
            .verify(&SignatureSuiteRegistry::recommended(), &wrong_key)
            .unwrap_err(),
        Oid4vpError::InvalidRequestObjectSignature
    );
}

#[test]
fn signed_client_and_wallet_nonce_must_match_exactly() {
    let cases = [
        (
            r#"{"client_id":"different"}"#.to_owned(),
            RequestUriMethod::Get,
            Oid4vpError::RequestObjectClientIdMismatch,
        ),
        (
            r#"{"response_type":"vp_token"}"#.to_owned(),
            RequestUriMethod::Get,
            Oid4vpError::RequestObjectClientIdMismatch,
        ),
        (
            r#"{"client_id":42}"#.to_owned(),
            RequestUriMethod::Get,
            Oid4vpError::RequestObjectClientIdMismatch,
        ),
        (
            format!(r#"{{"client_id":"{CLIENT_ID}","client_id":"{CLIENT_ID}"}}"#),
            RequestUriMethod::Get,
            Oid4vpError::RequestObjectClientIdMismatch,
        ),
        (
            format!(r#"{{"client_id":"{CLIENT_ID}"}}"#),
            RequestUriMethod::Post,
            Oid4vpError::RequestObjectWalletNonceMismatch,
        ),
        (
            format!(r#"{{"client_id":"{CLIENT_ID}","wallet_nonce":"wrong"}}"#),
            RequestUriMethod::Post,
            Oid4vpError::RequestObjectWalletNonceMismatch,
        ),
    ];

    for (payload, method, expected) in cases {
        let input = match method {
            RequestUriMethod::Get => RequestUriRetrievalInput::get(),
            RequestUriMethod::Post => RequestUriRetrievalInput::post(None, Some("fresh")),
        };
        let compact = signed(&payload, Some(REQUEST_OBJECT_JWT_TYPE));
        let unverified = reference(method)
            .prepare_retrieval(input, RequestUriRetrievalLimits::default())
            .unwrap()
            .bind_response(
                200,
                Some(REQUEST_OBJECT_MEDIA_TYPE),
                compact.as_bytes(),
                RequestObjectValidationLimits::default(),
            )
            .unwrap();
        assert_eq!(verify(unverified).unwrap_err(), expected);
    }
}

#[test]
fn verified_payload_json_rejects_duplicates_and_resource_exhaustion() {
    let duplicate = format!(r#"{{"client_id":"{CLIENT_ID}","x":1,"x":2}}"#);
    let nested = format!(r#"{{"client_id":"{CLIENT_ID}","x":{{"y":{{}}}}}}"#);
    let members = format!(r#"{{"client_id":"{CLIENT_ID}","a":1,"b":2}}"#);
    let long_string = format!(r#"{{"client_id":"{CLIENT_ID}"}}"#);
    let cases = [
        (duplicate, RequestObjectValidationLimits::default()),
        (
            nested,
            RequestObjectValidationLimits::new(JwsLimits::default(), 2, 100, 100, 100).unwrap(),
        ),
        (
            format!(r#"{{"client_id":"{CLIENT_ID}","a":1}}"#),
            RequestObjectValidationLimits::new(JwsLimits::default(), 10, 2, 100, 100).unwrap(),
        ),
        (
            members,
            RequestObjectValidationLimits::new(JwsLimits::default(), 10, 100, 2, 100).unwrap(),
        ),
        (
            long_string,
            RequestObjectValidationLimits::new(JwsLimits::default(), 10, 100, 10, 8).unwrap(),
        ),
    ];

    for (payload, limits) in cases {
        let compact = signed(&payload, Some(REQUEST_OBJECT_JWT_TYPE));
        let unverified = reference(RequestUriMethod::Get)
            .prepare_retrieval(
                RequestUriRetrievalInput::get(),
                RequestUriRetrievalLimits::default(),
            )
            .unwrap()
            .bind_response(
                200,
                Some(REQUEST_OBJECT_MEDIA_TYPE),
                compact.as_bytes(),
                limits,
            )
            .unwrap();
        assert_eq!(
            verify(unverified).unwrap_err(),
            Oid4vpError::InvalidRequestObjectPayload
        );
    }
}

#[test]
fn limit_constructors_and_diagnostics_are_safe() {
    assert_eq!(
        RequestUriRetrievalLimits::new(0, 1, 1, 1, 1),
        Err(Oid4vpError::InvalidLimits)
    );
    assert_eq!(
        RequestObjectValidationLimits::new(JwsLimits::default(), 0, 1, 1, 1),
        Err(Oid4vpError::InvalidLimits)
    );

    let input = format!(r#"{{"client_id":"{CLIENT_ID}","note":"{CANARY}"}}"#);
    let compact = signed(&input, Some(REQUEST_OBJECT_JWT_TYPE));
    let request = reference(RequestUriMethod::Post)
        .prepare_retrieval(
            RequestUriRetrievalInput::post(
                Some(r#"{"note":"VERIFIER_SECRET_CANARY"}"#),
                Some(CANARY),
            ),
            RequestUriRetrievalLimits::default(),
        )
        .unwrap();
    assert!(!format!("{request:?}").contains(CANARY));
    let unverified = request
        .bind_response(
            200,
            Some(REQUEST_OBJECT_MEDIA_TYPE),
            compact.as_bytes(),
            RequestObjectValidationLimits::default(),
        )
        .unwrap();
    assert!(!format!("{unverified:?}").contains(CANARY));
    let error = verify(unverified).unwrap_err();
    for rendered in [
        format!("{error}"),
        format!("{error:?}"),
        format!("{}", error.to_identus_error()),
    ] {
        assert!(!rendered.contains(CANARY));
    }
}
