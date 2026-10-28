use identus_crypto::{Ed25519PrivateKey, EncodeJwk, PublicKeyJwk};
use identus_jose::{
    Ed25519Signer, JwsAlgorithm, JwsLimits, JwsSigningInput, JwsVerificationKey, ProtectedHeader,
    SignatureSuiteRegistry,
};
use identus_oid4vp::{
    AuthorizationRequestInvocation, AuthorizationRequestInvocationLimits,
    AuthorizationRequestValidationLimits, AuthorizationResponseMode, DcqlLimits,
    DirectPostAcknowledgement, DirectPostAcknowledgementLimits, DirectPostErrorCode,
    DirectPostErrorDetails, DirectPostErrorResponseLimits, DirectPostPresentation,
    DirectPostQueryResult, DirectPostResponseLimits, Oid4vpError, REQUEST_OBJECT_JWT_TYPE,
    REQUEST_OBJECT_MEDIA_TYPE, RedirectResponseKind, RedirectResponseLimits,
    RequestObjectValidationLimits, RequestUriRetrievalInput, RequestUriRetrievalLimits,
    ValidatedAuthorizationRequest,
};
use serde_json::{Value, json};

mod support;

const PRIVATE_BYTES: [u8; 32] = [0x72; 32];
const CLIENT_ID: &str = "decentralized_identifier:did:example:redirect-verifier";
const REFERENCE: &str = "openid4vp:?client_id=decentralized_identifier%3Adid%3Aexample%3Aredirect-verifier&request_uri=https%3A%2F%2Fverifier.example%2Frequest";
const PACKET: &str =
    include_str!("../../../docs/conformance/fixtures/oid4vp-redirect-response/v1/vectors.json");

fn private_key() -> Ed25519PrivateKey {
    Ed25519PrivateKey::from_slice(&PRIVATE_BYTES).expect("fixed private key")
}

fn signed(payload: &[u8]) -> String {
    let header = ProtectedHeader::new(
        "Ed25519",
        Some(REQUEST_OBJECT_JWT_TYPE),
        Some("did:example:redirect-verifier#key-1"),
        JwsLimits::default(),
    )
    .expect("fixed header");
    JwsSigningInput::new(header, payload.to_vec(), JwsLimits::default())
        .expect("bounded signing input")
        .sign_with(&Ed25519Signer::new(&private_key()))
        .expect("fixed signature")
        .compact()
        .to_owned()
}

fn bind_verified(
    compact: &str,
    key: &PublicKeyJwk,
) -> Result<ValidatedAuthorizationRequest, Oid4vpError> {
    let AuthorizationRequestInvocation::Referenced(reference) =
        AuthorizationRequestInvocation::parse(
            REFERENCE,
            AuthorizationRequestInvocationLimits::default(),
        )?;
    let unverified = reference
        .prepare_retrieval(
            RequestUriRetrievalInput::get(),
            RequestUriRetrievalLimits::default(),
        )?
        .bind_response(
            200,
            Some(REQUEST_OBJECT_MEDIA_TYPE),
            compact.as_bytes(),
            RequestObjectValidationLimits::default(),
        )?;
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, key).expect("fixed verification key");
    support::authorize(support::secure(
        unverified
            .verify(&SignatureSuiteRegistry::recommended(), &key)
            .expect("verified request"),
    ))
    .into_authorization_request(
        AuthorizationRequestValidationLimits::default(),
        DcqlLimits::default(),
    )
}

fn validated(
    mode: &str,
    destination_name: &str,
    destination: &str,
) -> Result<ValidatedAuthorizationRequest, Oid4vpError> {
    let mut request = json!({
        "client_id": CLIENT_ID,
        "response_type": "vp_token",
        "response_mode": mode,
        "nonce": "fresh-nonce_1",
        "state": "state-123",
        "dcql_query": {
            "credentials": [{
                "id": "degree",
                "format": "dc+sd-jwt",
                "multiple": false,
                "meta": {}
            }]
        },
        "aud": identus_oid4vp::STATIC_DISCOVERY_AUDIENCE,
        "iat": support::ISSUED_AT_SECONDS
    });
    request
        .as_object_mut()
        .expect("object")
        .insert(destination_name.to_owned(), json!(destination));
    let compact = signed(&serde_json::to_vec(&request).expect("request JSON"));
    bind_verified(&compact, &private_key().to_public_key().encode_jwk())
}

fn result() -> DirectPostQueryResult {
    let limits = DirectPostResponseLimits::default();
    DirectPostQueryResult::new(
        "degree",
        vec![DirectPostPresentation::string("vp.compact", limits).expect("presentation")],
        limits,
    )
    .expect("result")
}

#[test]
fn final_e2e_redirect_modes_preserve_full_authorized_lineage() {
    let packet: Value = serde_json::from_str(PACKET).expect("clean-room packet");
    let fragment_target = packet["cases"][0]["expected"]["target"]
        .as_str()
        .expect("fragment target");
    let cases = [
        (
            "fragment",
            AuthorizationResponseMode::Fragment,
            "openid://wallet/cb?tenant=one#screen=done",
            fragment_target,
            false,
        ),
        (
            "query",
            AuthorizationResponseMode::Query,
            "https://verifier.example/cb?tenant=one#screen=done",
            "https://verifier.example/cb?tenant=one&vp_token=%7B%22degree%22%3A%5B%22vp.compact%22%5D%7D&state=state-123#screen=done",
            false,
        ),
        (
            "form_post",
            AuthorizationResponseMode::FormPost,
            "https://verifier.example/cb?tenant=one",
            "https://verifier.example/cb?tenant=one",
            true,
        ),
    ];
    for (wire_mode, mode, destination, target, has_form) in cases {
        let response = validated(wire_mode, "redirect_uri", destination)
            .expect("validated redirect request")
            .into_redirect_response(
                vec![result()],
                DirectPostResponseLimits::default(),
                RedirectResponseLimits::default(),
            )
            .expect("redirect response");
        assert_eq!(response.mode(), mode);
        assert_eq!(response.kind(), RedirectResponseKind::Success);
        assert_eq!(response.expose_sensitive_target(), target);
        assert_eq!(response.expose_sensitive_form_body().is_some(), has_form);
        if let Some(body) = response.expose_sensitive_form_body() {
            assert_eq!(
                body,
                "vp_token=%7B%22degree%22%3A%5B%22vp.compact%22%5D%7D&state=state-123"
            );
        }
    }
}

#[test]
fn mode_destination_cross_product_and_collisions_fail_closed() {
    assert_eq!(
        validated(
            "fragment",
            "response_uri",
            "https://verifier.example/response"
        )
        .unwrap_err(),
        Oid4vpError::MissingResponseUri
    );
    assert_eq!(
        validated("direct_post", "redirect_uri", "https://verifier.example/cb").unwrap_err(),
        Oid4vpError::MissingResponseUri
    );

    for destination in [
        "https://verifier.example/cb?vp_token=old",
        "https://verifier.example/cb#%76p_token=old",
        "https://verifier.example/cb?state=old#screen=done",
    ] {
        assert_eq!(
            validated("fragment", "redirect_uri", destination)
                .unwrap()
                .into_redirect_response(
                    vec![result()],
                    DirectPostResponseLimits::default(),
                    RedirectResponseLimits::default(),
                )
                .unwrap_err(),
            Oid4vpError::ConflictingResponseDestination
        );
    }
}

#[test]
fn redirect_error_is_explicit_bounded_and_contains_no_presentation() {
    let details = DirectPostErrorDetails::new(
        Some("request rejected"),
        None,
        DirectPostErrorResponseLimits::default(),
    )
    .unwrap();
    let response = validated("query", "redirect_uri", "https://verifier.example/cb")
        .unwrap()
        .into_redirect_error_response(
            DirectPostErrorCode::AccessDenied,
            details,
            DirectPostErrorResponseLimits::default(),
            RedirectResponseLimits::default(),
        )
        .unwrap();
    assert_eq!(
        response.kind(),
        RedirectResponseKind::Error(DirectPostErrorCode::AccessDenied)
    );
    assert_eq!(
        response.expose_sensitive_target(),
        "https://verifier.example/cb?error=access_denied&error_description=request+rejected&state=state-123"
    );
    assert!(!response.expose_sensitive_target().contains("vp_token"));
}

#[test]
fn direct_post_acknowledgement_is_consuming_bounded_and_opaque() {
    let packet: Value = serde_json::from_str(PACKET).expect("clean-room packet");
    let case = &packet["cases"][1];
    let response = validated(
        "direct_post",
        "response_uri",
        "https://verifier.example/response",
    )
    .unwrap()
    .into_direct_post_response(vec![result()], DirectPostResponseLimits::default())
    .unwrap();
    let acknowledgement = response
        .bind_endpoint_acknowledgement(
            200,
            case["input"]["contentType"].as_str(),
            case["input"]["cacheControl"].as_str(),
            case["input"]["body"].as_str().unwrap().as_bytes(),
            DirectPostAcknowledgementLimits::default(),
        )
        .unwrap();
    let DirectPostAcknowledgement::Redirect(candidate) = acknowledgement else {
        panic!("redirect acknowledgement")
    };
    assert_eq!(
        candidate.expose_sensitive_redirect_uri(),
        case["expected"]["redirectUri"].as_str().unwrap()
    );
    assert_eq!(
        candidate.expose_sensitive_response_code(),
        case["expected"]["responseCode"].as_str()
    );
}

#[test]
fn acknowledgement_rejects_detached_or_ambiguous_navigation_evidence() {
    fn response() -> identus_oid4vp::DirectPostResponse {
        validated(
            "direct_post",
            "response_uri",
            "https://verifier.example/response",
        )
        .unwrap()
        .into_direct_post_response(vec![result()], DirectPostResponseLimits::default())
        .unwrap()
    }

    let limits = DirectPostAcknowledgementLimits::default();
    assert!(matches!(
        response()
            .bind_endpoint_acknowledgement(
                200,
                Some("application/json"),
                Some("no-store"),
                b"{}",
                limits
            )
            .unwrap(),
        DirectPostAcknowledgement::Complete
    ));
    for (status, content_type, cache_control, body, error) in [
        (
            201,
            Some("application/json"),
            Some("no-store"),
            b"{}".as_slice(),
            Oid4vpError::InvalidDirectPostAcknowledgement,
        ),
        (
            200,
            Some("text/html"),
            Some("no-store"),
            b"{}".as_slice(),
            Oid4vpError::InvalidDirectPostAcknowledgement,
        ),
        (
            200,
            Some("application/json; charset"),
            Some("no-store"),
            b"{}".as_slice(),
            Oid4vpError::InvalidDirectPostAcknowledgement,
        ),
        (
            200,
            Some("application/json;charset=utf-8;CHARSET=iso-8859-1"),
            Some("no-store"),
            b"{}".as_slice(),
            Oid4vpError::InvalidDirectPostAcknowledgement,
        ),
        (
            200,
            Some("application/json"),
            Some("max-age=60"),
            b"{}".as_slice(),
            Oid4vpError::InvalidDirectPostAcknowledgement,
        ),
        (
            200,
            Some("application/json"),
            Some("no-store=value"),
            b"{}".as_slice(),
            Oid4vpError::InvalidDirectPostAcknowledgement,
        ),
        (
            200,
            Some("application/json"),
            Some("private=\"a,no-store,b\""),
            b"{}".as_slice(),
            Oid4vpError::InvalidDirectPostAcknowledgement,
        ),
        (
            200,
            Some("application/json"),
            Some("no-store"),
            br#"{"redirect_uri":"https://v.example/cb?response_code=one#response_code=two"}"#
                .as_slice(),
            Oid4vpError::AmbiguousResponseCode,
        ),
        (
            200,
            Some("application/json"),
            Some("no-store"),
            br#"{"redirect_uri":"https://one" , "redirect_uri":"https://two"}"#.as_slice(),
            Oid4vpError::InvalidDirectPostAcknowledgement,
        ),
    ] {
        assert_eq!(
            response()
                .bind_endpoint_acknowledgement(status, content_type, cache_control, body, limits)
                .unwrap_err(),
            error
        );
    }
}

#[test]
fn redirect_and_acknowledgement_diagnostics_are_redacted() {
    const CANARY: &str = "VERIFIER_SECRET_CANARY";
    let response = validated(
        "fragment",
        "redirect_uri",
        &format!("openid://wallet/cb#screen={CANARY}"),
    )
    .unwrap()
    .into_redirect_response(
        vec![result()],
        DirectPostResponseLimits::default(),
        RedirectResponseLimits::default(),
    )
    .unwrap();
    assert!(!format!("{response:?}").contains(CANARY));
}

#[test]
fn property_redirect_modes_obey_exact_output_and_collision_boundaries() {
    for (mode, destination) in [
        ("query", "https://verifier.example/cb?existing=one"),
        ("fragment", "openid://wallet/cb#existing=one"),
    ] {
        let broad = validated(mode, "redirect_uri", destination)
            .unwrap()
            .into_redirect_response(
                vec![result()],
                DirectPostResponseLimits::default(),
                RedirectResponseLimits::default(),
            )
            .unwrap();
        let exact_len = broad.expose_sensitive_target().len();
        validated(mode, "redirect_uri", destination)
            .unwrap()
            .into_redirect_response(
                vec![result()],
                DirectPostResponseLimits::default(),
                RedirectResponseLimits::new(exact_len, 2).unwrap(),
            )
            .expect("exact output boundary");
        assert_eq!(
            validated(mode, "redirect_uri", destination)
                .unwrap()
                .into_redirect_response(
                    vec![result()],
                    DirectPostResponseLimits::default(),
                    RedirectResponseLimits::new(exact_len - 1, 2).unwrap(),
                )
                .unwrap_err(),
            Oid4vpError::ResponseUriTooLarge
        );
    }

    assert_eq!(
        RedirectResponseLimits::new(0, 1),
        Err(Oid4vpError::InvalidLimits)
    );
    assert_eq!(
        RedirectResponseLimits::new(1, 0),
        Err(Oid4vpError::InvalidLimits)
    );
    assert_eq!(
        DirectPostAcknowledgementLimits::new(1, 1, 1, 1, 1, 1, 0),
        Err(Oid4vpError::InvalidLimits)
    );
}
