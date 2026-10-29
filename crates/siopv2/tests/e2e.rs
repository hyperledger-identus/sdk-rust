use identus_crypto::{Ed25519PrivateKey, EncodeJwk, P256PrivateKey};
use identus_did::{Did, DidUrl};
use identus_jose::{
    Ed25519Signer, Es256Signer, JwsAlgorithm, JwsLimits, JwsSigner, JwsSigningInput,
    JwsVerificationKey, ProtectedHeader, SignatureSuiteRegistry, SignerFailure,
    UnverifiedCompactJws,
};
use identus_openid4vc::{
    REQUEST_OBJECT_MEDIA_TYPE, RequestRetrievalInput, RequestRetrievalLimits, RequestUriMethod,
    RequestUriReference,
};
use identus_siopv2::{
    EventId, REQUEST_OBJECT_JWT_TYPE, SIOPV2_AUDIENCE, SiopFlow, Siopv2Error, Siopv2Limits,
    UnverifiedSiopRequest,
};
use serde_json::{Value, json};

const REQUEST_PRIVATE: [u8; 32] = [0x31; 32];
const SUBJECT_PRIVATE: [u8; 32] = [0x52; 32];
const NOW: i64 = 1_800_000_000;

struct RejectingSigner;

impl JwsSigner for RejectingSigner {
    fn algorithm(&self) -> JwsAlgorithm {
        JwsAlgorithm::Ed25519
    }

    fn sign(&self, _signing_input: &[u8]) -> Result<[u8; 64], SignerFailure> {
        Err(SignerFailure::Rejected)
    }
}

fn request_key() -> Ed25519PrivateKey {
    Ed25519PrivateKey::from_slice(&REQUEST_PRIVATE).expect("request key")
}

fn subject_key() -> Ed25519PrivateKey {
    Ed25519PrivateKey::from_slice(&SUBJECT_PRIVATE).expect("subject key")
}

fn request_claims() -> Value {
    json!({
        "iss": "https://rp.example",
        "aud": SIOPV2_AUDIENCE,
        "client_id": "https://rp.example",
        "redirect_uri": "https://rp.example/direct-post",
        "response_type": "id_token",
        "response_mode": "direct_post",
        "scope": "openid",
        "id_token_type": "subject_signed_id_token",
        "nonce": "nonce-123",
        "state": "state-456",
        "iat": NOW - 10,
        "exp": NOW + 300
    })
}

fn signed_request(value: &Value) -> String {
    let header = ProtectedHeader::new(
        "Ed25519",
        Some(REQUEST_OBJECT_JWT_TYPE),
        Some("did:example:rp#key-1"),
        JwsLimits::default(),
    )
    .expect("header");
    JwsSigningInput::new(
        header,
        serde_json::to_vec(value).expect("claims"),
        JwsLimits::default(),
    )
    .expect("input")
    .sign_with(&Ed25519Signer::new(&request_key()))
    .expect("signature")
    .compact()
    .to_owned()
}

fn validated_request(value: &Value) -> identus_siopv2::ValidatedSiopRequest {
    validated_request_with_limits(value, Siopv2Limits::default())
}

fn validated_request_with_limits(
    value: &Value,
    limits: Siopv2Limits,
) -> identus_siopv2::ValidatedSiopRequest {
    let compact = signed_request(value);
    let retrieved = RequestUriReference::new(
        "https://rp.example",
        "https://rp.example/request.jwt",
        RequestUriMethod::Get,
        RequestRetrievalLimits::default(),
    )
    .expect("reference")
    .prepare(
        RequestRetrievalInput::get(),
        RequestRetrievalLimits::default(),
    )
    .expect("retrieval request")
    .bind_response(200, Some(REQUEST_OBJECT_MEDIA_TYPE), compact.as_bytes())
    .expect("bound response");
    let public = request_key().to_public_key().encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).expect("key binding");
    UnverifiedSiopRequest::from_retrieved(&retrieved, limits)
        .expect("parse")
        .verify(&SignatureSuiteRegistry::recommended(), &key, NOW)
        .expect("verify")
}

#[test]
fn complete_did_subject_flow_round_trips_through_an_independent_verifier() {
    let request = validated_request(&request_claims());
    let mut flow = SiopFlow::new(request);
    flow.approve(
        EventId::new("approval-1", 128).expect("event"),
        Did::parse("did:example:alice").expect("DID"),
        DidUrl::parse("did:example:alice#auth-1").expect("DID URL"),
        JwsAlgorithm::Ed25519,
        NOW,
        NOW,
        NOW + 120,
        &Ed25519Signer::new(&subject_key()),
    )
    .expect("approved response");
    let SiopFlow::ResponseReady { response, .. } = flow else {
        panic!("response ready");
    };

    assert_eq!(response.method(), "POST");
    assert_eq!(
        response.expose_sensitive_endpoint(),
        "https://rp.example/direct-post"
    );
    let pairs = decode_form(response.expose_sensitive_body());
    assert_eq!(pairs.get("state").map(String::as_str), Some("state-456"));
    let compact = UnverifiedCompactJws::parse(
        pairs.get("id_token").expect("id token"),
        JwsLimits::default(),
    )
    .expect("response JWS");
    assert_eq!(
        compact.protected_header().key_id(),
        Some("did:example:alice#auth-1")
    );
    let public = subject_key().to_public_key().encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).expect("subject key");
    let verified = SignatureSuiteRegistry::recommended()
        .verify(&compact, &key)
        .expect("token signature");
    let claims: Value =
        serde_json::from_slice(verified.as_compact().payload()).expect("token claims");
    assert_eq!(claims["iss"], "did:example:alice");
    assert_eq!(claims["sub"], "did:example:alice");
    assert_eq!(claims["aud"], "https://rp.example");
    assert_eq!(claims["nonce"], "nonce-123");
    assert_eq!(claims["iat"], NOW);
    assert_eq!(claims["exp"], NOW + 120);
}

#[test]
fn refusal_timeout_and_cancellation_never_need_a_signer() {
    for transition in ["refuse", "timeout", "cancel"] {
        let mut flow = SiopFlow::new(validated_request(&request_claims()));
        let event = EventId::new(transition, 128).unwrap();
        match transition {
            "refuse" => flow.refuse(event),
            "timeout" => flow.timeout(event),
            _ => flow.cancel(event),
        }
        .expect("terminal transition");
        assert!(matches!(
            flow,
            SiopFlow::Refused { .. } | SiopFlow::TimedOut { .. } | SiopFlow::Canceled { .. }
        ));
    }
}

#[test]
fn profile_binding_and_duplicate_claims_fail_closed() {
    let mut wrong_audience = request_claims();
    wrong_audience["aud"] = json!("https://wrong.example");
    assert_eq!(
        validated_error(&signed_request(&wrong_audience)),
        identus_siopv2::Siopv2Error::UnsupportedProfile,
    );

    let duplicate = format!(
        "{{\"iss\":\"https://rp.example\",\"aud\":\"{SIOPV2_AUDIENCE}\",\"client_id\":\"https://rp.example\",\"client_id\":\"https://evil.example\",\"redirect_uri\":\"https://rp.example/direct-post\",\"response_type\":\"id_token\",\"response_mode\":\"direct_post\",\"scope\":\"openid\",\"id_token_type\":\"subject_signed_id_token\",\"nonce\":\"n\",\"iat\":{},\"exp\":{}}}",
        NOW - 1,
        NOW + 10,
    );
    let header = ProtectedHeader::new(
        "Ed25519",
        Some(REQUEST_OBJECT_JWT_TYPE),
        Some("did:example:rp#k"),
        JwsLimits::default(),
    )
    .unwrap();
    let compact = JwsSigningInput::new(header, duplicate.into_bytes(), JwsLimits::default())
        .unwrap()
        .sign_with(&Ed25519Signer::new(&request_key()))
        .unwrap()
        .compact()
        .to_owned();
    assert_eq!(
        validated_error(&compact),
        identus_siopv2::Siopv2Error::InvalidRequestClaims
    );
}

#[test]
fn retrieved_client_identifier_must_match_the_signed_request() {
    let compact = signed_request(&request_claims());
    let retrieved = RequestUriReference::new(
        "https://other-rp.example",
        "https://rp.example/request.jwt",
        RequestUriMethod::Get,
        RequestRetrievalLimits::default(),
    )
    .expect("reference")
    .prepare(
        RequestRetrievalInput::get(),
        RequestRetrievalLimits::default(),
    )
    .expect("retrieval request")
    .bind_response(200, Some(REQUEST_OBJECT_MEDIA_TYPE), compact.as_bytes())
    .expect("bound response");
    let public = request_key().to_public_key().encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).expect("key binding");

    assert_eq!(
        UnverifiedSiopRequest::from_retrieved(&retrieved, Siopv2Limits::default())
            .expect("parse")
            .verify(&SignatureSuiteRegistry::recommended(), &key, NOW)
            .unwrap_err(),
        identus_siopv2::Siopv2Error::ClientBindingMismatch,
    );
}

#[test]
fn post_retrieval_nonce_is_bound_to_the_signed_request() {
    let mut claims = request_claims();
    claims["wallet_nonce"] = json!("wallet-789");
    let compact = signed_request(&claims);
    let retrieve = |nonce| {
        RequestUriReference::new(
            "https://rp.example",
            "https://rp.example/request.jwt",
            RequestUriMethod::Post,
            RequestRetrievalLimits::default(),
        )
        .unwrap()
        .prepare(
            RequestRetrievalInput::post(None, Some(nonce)),
            RequestRetrievalLimits::default(),
        )
        .unwrap()
        .bind_response(200, Some(REQUEST_OBJECT_MEDIA_TYPE), compact.as_bytes())
        .unwrap()
    };
    let public = request_key().to_public_key().encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).unwrap();

    assert!(
        UnverifiedSiopRequest::from_retrieved(&retrieve("wallet-789"), Siopv2Limits::default())
            .unwrap()
            .verify(&SignatureSuiteRegistry::recommended(), &key, NOW)
            .is_ok()
    );
    assert_eq!(
        UnverifiedSiopRequest::from_retrieved(&retrieve("wallet-other"), Siopv2Limits::default())
            .unwrap()
            .verify(&SignatureSuiteRegistry::recommended(), &key, NOW)
            .unwrap_err(),
        Siopv2Error::WalletNonceMismatch
    );
}

#[test]
fn refined_diagnostics_are_stable_and_redacted() {
    assert_eq!(
        Siopv2Error::WalletNonceMismatch.code(),
        "siopv2_wallet_nonce_mismatch"
    );
    assert_eq!(
        Siopv2Error::ResponseEncodingFailed.code(),
        "siopv2_response_encoding_failed"
    );
    for error in [
        Siopv2Error::WalletNonceMismatch,
        Siopv2Error::ResponseEncodingFailed,
    ] {
        let rendered = error.to_string();
        assert_eq!(rendered, error.code());
        assert!(!rendered.contains("wallet-789"));
        assert!(!rendered.contains("nonce-123"));
        assert!(!rendered.contains("did:example"));
    }
}

#[test]
fn response_header_bound_is_not_reported_as_an_invalid_did_key() {
    let jws = JwsLimits::new(65_536, 4_096, 49_152, 1_024, 24).unwrap();
    let limits = Siopv2Limits::new(jws, 16, 256, 32, 4_096, 4_096, 65_536, 128, 600, 60).unwrap();
    let request = validated_request_with_limits(&request_claims(), limits);

    assert!(matches!(
        request.plan_response(
            Did::parse("did:example:alice").unwrap(),
            DidUrl::parse("did:example:alice#authentication-key").unwrap(),
            JwsAlgorithm::Ed25519,
            NOW,
            NOW + 120,
        ),
        Err(Siopv2Error::ResponseTooLarge)
    ));
}

#[test]
fn response_algorithm_is_selected_by_the_subject_key_not_the_request_key() {
    let mut flow = SiopFlow::new(validated_request(&request_claims()));
    let subject_private = P256PrivateKey::from_slice(&SUBJECT_PRIVATE).expect("P-256 key");
    flow.approve(
        EventId::new("approval-es256", 128).unwrap(),
        Did::parse("did:example:alice").unwrap(),
        DidUrl::parse("did:example:alice#p256-1").unwrap(),
        JwsAlgorithm::Es256,
        NOW,
        NOW,
        NOW + 120,
        &Es256Signer::new(&subject_private),
    )
    .unwrap();
    let SiopFlow::ResponseReady { response, .. } = &flow else {
        panic!("response ready");
    };
    let pairs = decode_form(response.expose_sensitive_body());
    let compact =
        UnverifiedCompactJws::parse(pairs.get("id_token").unwrap(), JwsLimits::default()).unwrap();
    assert_eq!(compact.protected_header().algorithm(), "ES256");
    let public = subject_private.to_public_key().encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Es256, &public).unwrap();
    assert!(
        SignatureSuiteRegistry::recommended()
            .verify(&compact, &key)
            .is_ok()
    );
}

#[test]
fn failed_transitions_preserve_state_and_response_timeout_is_unknown() {
    let mut flow = SiopFlow::new(validated_request(&request_claims()));
    flow.approve(
        EventId::new("approval-1", 128).unwrap(),
        Did::parse("did:example:alice").unwrap(),
        DidUrl::parse("did:example:alice#auth-1").unwrap(),
        JwsAlgorithm::Ed25519,
        NOW,
        NOW,
        NOW + 120,
        &Ed25519Signer::new(&subject_key()),
    )
    .unwrap();
    assert_eq!(
        flow.refuse(EventId::new("refusal-after-approval", 128).unwrap())
            .unwrap_err(),
        identus_siopv2::Siopv2Error::InvalidTransition
    );
    assert!(matches!(flow, SiopFlow::ResponseReady { .. }));

    let timeout = EventId::new("delivery-timeout", 128).unwrap();
    flow.timeout(timeout.clone()).unwrap();
    assert!(matches!(flow, SiopFlow::OutcomeUnknown { .. }));
    flow.timeout(timeout).expect("same timeout is idempotent");
    assert_eq!(
        flow.timeout(EventId::new("other-timeout", 128).unwrap())
            .unwrap_err(),
        identus_siopv2::Siopv2Error::EventConflict
    );
    assert!(matches!(flow, SiopFlow::OutcomeUnknown { .. }));
}

#[test]
fn idempotency_is_local_to_the_current_lifecycle_state() {
    let approval = EventId::new("approval-local", 128).unwrap();
    let mut flow = SiopFlow::new(validated_request(&request_claims()));
    flow.approve(
        approval.clone(),
        Did::parse("did:example:alice").unwrap(),
        DidUrl::parse("did:example:alice#auth-1").unwrap(),
        JwsAlgorithm::Ed25519,
        NOW,
        NOW,
        NOW + 120,
        &Ed25519Signer::new(&subject_key()),
    )
    .unwrap();
    flow.approve(
        approval.clone(),
        Did::parse("did:example:alice").unwrap(),
        DidUrl::parse("did:example:alice#auth-1").unwrap(),
        JwsAlgorithm::Ed25519,
        NOW,
        NOW,
        NOW + 120,
        &RejectingSigner,
    )
    .expect("same approval receipt is idempotent while response-ready is current");

    let delivery = EventId::new("delivery-local", 128).unwrap();
    flow.delivered(delivery.clone()).unwrap();
    flow.delivered(delivery)
        .expect("current delivery receipt is idempotent");
    assert_eq!(
        flow.approve(
            approval,
            Did::parse("did:example:alice").unwrap(),
            DidUrl::parse("did:example:alice#auth-1").unwrap(),
            JwsAlgorithm::Ed25519,
            NOW,
            NOW,
            NOW + 120,
            &RejectingSigner,
        )
        .unwrap_err(),
        Siopv2Error::InvalidTransition
    );
    assert!(matches!(flow, SiopFlow::Delivered { .. }));
}

#[test]
fn signer_failure_preserves_the_awaiting_decision_state() {
    let mut flow = SiopFlow::new(validated_request(&request_claims()));
    assert_eq!(
        flow.approve(
            EventId::new("approval-rejected", 128).unwrap(),
            Did::parse("did:example:alice").unwrap(),
            DidUrl::parse("did:example:alice#auth-1").unwrap(),
            JwsAlgorithm::Ed25519,
            NOW,
            NOW,
            NOW + 120,
            &RejectingSigner,
        )
        .unwrap_err(),
        identus_siopv2::Siopv2Error::SigningRejected
    );
    assert!(matches!(flow, SiopFlow::AwaitingDecision { .. }));
}

#[test]
fn approval_rechecks_request_expiry_before_invoking_the_signer() {
    let mut flow = SiopFlow::new(validated_request(&request_claims()));
    assert_eq!(
        flow.approve(
            EventId::new("approval-after-expiry", 128).unwrap(),
            Did::parse("did:example:alice").unwrap(),
            DidUrl::parse("did:example:alice#auth-1").unwrap(),
            JwsAlgorithm::Ed25519,
            NOW + 300,
            NOW,
            NOW + 120,
            &RejectingSigner,
        )
        .unwrap_err(),
        identus_siopv2::Siopv2Error::RequestExpired
    );
    assert!(matches!(flow, SiopFlow::AwaitingDecision { .. }));
}

#[test]
fn signature_time_destination_and_method_fail_closed() {
    let compact = signed_request(&request_claims());
    let wrong_private = Ed25519PrivateKey::from_slice(&[0x7f; 32]).unwrap();
    let wrong_public = wrong_private.to_public_key().encode_jwk();
    let wrong_key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &wrong_public).unwrap();
    assert_eq!(
        UnverifiedSiopRequest::parse(&compact, Siopv2Limits::default())
            .unwrap()
            .verify(&SignatureSuiteRegistry::recommended(), &wrong_key, NOW)
            .unwrap_err(),
        identus_siopv2::Siopv2Error::InvalidRequestSignature
    );

    let mut expired = request_claims();
    expired["iat"] = json!(NOW - 700);
    expired["exp"] = json!(NOW - 100);
    assert_eq!(
        validated_error(&signed_request(&expired)),
        identus_siopv2::Siopv2Error::RequestExpired
    );

    let mut future = request_claims();
    future["iat"] = json!(NOW + 61);
    future["exp"] = json!(NOW + 120);
    assert_eq!(
        validated_error(&signed_request(&future)),
        identus_siopv2::Siopv2Error::InvalidTimeWindow
    );

    let mut unsafe_destination = request_claims();
    unsafe_destination["redirect_uri"] = json!("https://user@rp.example/direct-post");
    assert_eq!(
        validated_error(&signed_request(&unsafe_destination)),
        identus_siopv2::Siopv2Error::InvalidRedirectUri
    );

    let request = validated_request(&request_claims());
    assert!(matches!(
        request.plan_response(
            Did::parse("did:example:alice").unwrap(),
            DidUrl::parse("did:example:bob#auth-1").unwrap(),
            JwsAlgorithm::Ed25519,
            NOW,
            NOW + 120,
        ),
        Err(identus_siopv2::Siopv2Error::InvalidKeyId)
    ));

    let request = validated_request(&request_claims());
    assert!(matches!(
        request.plan_response(
            Did::parse("did:example:alice").unwrap(),
            DidUrl::parse("did:example:alice/path").unwrap(),
            JwsAlgorithm::Ed25519,
            NOW,
            NOW + 120,
        ),
        Err(identus_siopv2::Siopv2Error::InvalidKeyId)
    ));
}

fn validated_error(compact: &str) -> identus_siopv2::Siopv2Error {
    let public = request_key().to_public_key().encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).unwrap();
    UnverifiedSiopRequest::parse(compact, Siopv2Limits::default())
        .unwrap()
        .verify(&SignatureSuiteRegistry::recommended(), &key, NOW)
        .unwrap_err()
}

fn decode_form(bytes: &[u8]) -> std::collections::BTreeMap<String, String> {
    std::str::from_utf8(bytes)
        .expect("UTF-8 form")
        .split('&')
        .map(|pair| {
            let (name, value) = pair.split_once('=').expect("pair");
            (name.to_owned(), percent_decode(value))
        })
        .collect()
}

fn percent_decode(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut output = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            let high = hex(bytes[index + 1]);
            let low = hex(bytes[index + 2]);
            output.push((high << 4) | low);
            index += 3;
        } else {
            output.push(bytes[index]);
            index += 1;
        }
    }
    String::from_utf8(output).expect("decoded UTF-8")
}

fn hex(byte: u8) -> u8 {
    match byte {
        b'0'..=b'9' => byte - b'0',
        b'A'..=b'F' => byte - b'A' + 10,
        _ => panic!("hex"),
    }
}
