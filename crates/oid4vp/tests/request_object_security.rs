use std::{
    collections::HashSet,
    future::Future,
    pin::pin,
    sync::{
        Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    task::{Context, Poll, Waker},
};

use identus_core::{ClockError, UnixTimestampMillis, WallClock};
use identus_crypto::{Ed25519PrivateKey, EncodeJwk};
use identus_jose::{
    Ed25519Signer, JwsAlgorithm, JwsLimits, JwsSigningInput, JwsVerificationKey, ProtectedHeader,
    SignatureSuiteRegistry,
};
use identus_oid4vp::{
    AuthorizationRequestInvocation, AuthorizationRequestInvocationLimits,
    AuthorizationRequestValidationLimits, DcqlLimits, Oid4vpError, REQUEST_OBJECT_JWT_TYPE,
    REQUEST_OBJECT_MEDIA_TYPE, RequestObjectReplayFailure, RequestObjectReplayFuture,
    RequestObjectReplayGuard, RequestObjectReplayInput, RequestObjectSecurityEvidence,
    RequestObjectSecurityPolicy, RequestObjectValidationLimits, RequestUriRetrievalInput,
    RequestUriRetrievalLimits, STATIC_DISCOVERY_AUDIENCE, VerifiedRequestObject,
};
use serde_json::{Value, json};

const PRIVATE_BYTES: [u8; 32] = [0x71; 32];
const CLIENT_ID: &str = "decentralized_identifier:did:example:verifier";
const REFERENCE: &str = "openid4vp:?client_id=decentralized_identifier%3Adid%3Aexample%3Averifier&request_uri=https%3A%2F%2Fverifier.example%2Frequest";
const NOW: u64 = 1_735_689_600;
const CANARY: &str = "REQUEST_SECURITY_SECRET_CANARY";

struct FixedClock(Result<u64, ClockError>);

impl WallClock for FixedClock {
    fn now(&self) -> Result<UnixTimestampMillis, ClockError> {
        self.0
            .map(|seconds| UnixTimestampMillis::new(seconds * 1_000))
    }
}

struct Replay {
    outcome: Result<(), RequestObjectReplayFailure>,
    calls: AtomicUsize,
    seen: Mutex<Option<SeenReplay>>,
}

struct SeenReplay {
    issued_at: u64,
    expires_at: Option<u64>,
    jwt_id: Option<String>,
    payload_len: usize,
}

impl Replay {
    fn accepting() -> Self {
        Self {
            outcome: Ok(()),
            calls: AtomicUsize::new(0),
            seen: Mutex::new(None),
        }
    }

    fn failing(failure: RequestObjectReplayFailure) -> Self {
        Self {
            outcome: Err(failure),
            calls: AtomicUsize::new(0),
            seen: Mutex::new(None),
        }
    }
}

impl RequestObjectReplayGuard for Replay {
    fn accept<'a>(&'a self, input: RequestObjectReplayInput<'a>) -> RequestObjectReplayFuture<'a> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        *self.seen.lock().expect("replay evidence lock") = Some(SeenReplay {
            issued_at: input.issued_at(),
            expires_at: input.expires_at(),
            jwt_id: input.expose_sensitive_jwt_id().map(str::to_owned),
            payload_len: input.expose_sensitive_payload().len(),
        });
        let outcome = self.outcome;
        Box::pin(async move { outcome })
    }
}

#[derive(Default)]
struct OnceReplay {
    accepted_payloads: Mutex<HashSet<Vec<u8>>>,
}

impl RequestObjectReplayGuard for OnceReplay {
    fn accept<'a>(&'a self, input: RequestObjectReplayInput<'a>) -> RequestObjectReplayFuture<'a> {
        let accepted = self
            .accepted_payloads
            .lock()
            .expect("atomic replay lock")
            .insert(input.expose_sensitive_payload().to_vec());
        Box::pin(async move {
            if accepted {
                Ok(())
            } else {
                Err(RequestObjectReplayFailure::Rejected)
            }
        })
    }
}

fn request(claims: Value) -> Value {
    let mut request = json!({
        "client_id": CLIENT_ID,
        "response_type": "vp_token",
        "response_mode": "direct_post",
        "nonce": "fresh-nonce_1",
        "response_uri": "https://verifier.example/response",
        "dcql_query": {"credentials": [{
            "id": "pid",
            "format": "dc+sd-jwt",
            "meta": {}
        }]}
    });
    request
        .as_object_mut()
        .expect("request object")
        .extend(claims.as_object().expect("claims object").clone());
    request
}

fn verified(request: &Value) -> VerifiedRequestObject {
    let compact = signed_compact(request);
    let private = Ed25519PrivateKey::from_slice(&PRIVATE_BYTES).expect("fixed key");
    let AuthorizationRequestInvocation::Referenced(reference) =
        AuthorizationRequestInvocation::parse(
            REFERENCE,
            AuthorizationRequestInvocationLimits::default(),
        )
        .expect("invocation");
    let object = reference
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
        .expect("binding");
    let public = private.to_public_key().encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).expect("bound key");
    object
        .verify(&SignatureSuiteRegistry::recommended(), &key)
        .expect("verified")
}

fn signed_compact(request: &Value) -> String {
    let private = Ed25519PrivateKey::from_slice(&PRIVATE_BYTES).expect("fixed key");
    let header = ProtectedHeader::new(
        "Ed25519",
        Some(REQUEST_OBJECT_JWT_TYPE),
        Some("did:example:verifier#key-1"),
        JwsLimits::default(),
    )
    .expect("header");
    JwsSigningInput::new(
        header,
        serde_json::to_vec(request).expect("request JSON"),
        JwsLimits::default(),
    )
    .expect("input")
    .sign_with(&Ed25519Signer::new(&private))
    .expect("signature")
    .compact()
    .to_owned()
}

fn policy(audience: &str) -> RequestObjectSecurityPolicy {
    RequestObjectSecurityPolicy::new(audience, 300, 30, RequestObjectValidationLimits::default())
        .expect("policy")
}

fn validate(
    request: &Value,
    clock: &dyn WallClock,
    replay: &dyn RequestObjectReplayGuard,
) -> Result<RequestObjectSecurityEvidence, Oid4vpError> {
    block_on(verified(request).validate_security(&policy(STATIC_DISCOVERY_AUDIENCE), clock, replay))
}

fn validate_with_policy(
    request: &Value,
    max_age_seconds: u64,
    allowed_clock_skew_seconds: u64,
    replay: &dyn RequestObjectReplayGuard,
) -> Result<RequestObjectSecurityEvidence, Oid4vpError> {
    let policy = RequestObjectSecurityPolicy::static_discovery(
        max_age_seconds,
        allowed_clock_skew_seconds,
        RequestObjectValidationLimits::default(),
    )
    .expect("generated policy");
    block_on(verified(request).validate_security(&policy, &FixedClock(Ok(NOW)), replay))
}

#[test]
fn packet_static_audience_executes_full_security_transition() {
    let request = request(json!({
        "aud": STATIC_DISCOVERY_AUDIENCE,
        "iat": NOW - 120,
        "exp": NOW + 180,
        "nbf": NOW - 150,
        "jti": "request-final-static-001"
    }));
    let replay = Replay::accepting();
    let evidence = validate(&request, &FixedClock(Ok(NOW)), &replay).expect("accepted");
    assert_eq!(evidence.algorithm(), JwsAlgorithm::Ed25519);
    assert_eq!(evidence.expose_sensitive_client_id(), CLIENT_ID);
    assert_eq!(evidence.payload_len(), seen_payload_len(&request));
    assert_eq!(
        evidence.expose_sensitive_audiences().collect::<Vec<_>>(),
        vec![STATIC_DISCOVERY_AUDIENCE]
    );
    assert_eq!(evidence.issued_at(), NOW - 120);
    assert_eq!(evidence.expires_at(), Some(NOW + 180));
    assert_eq!(evidence.not_before(), Some(NOW - 150));
    assert_eq!(
        evidence.expose_sensitive_jwt_id(),
        Some("request-final-static-001")
    );
    assert_eq!(replay.calls.load(Ordering::SeqCst), 1);
    assert_eq!(
        replay
            .seen
            .lock()
            .unwrap()
            .as_ref()
            .map(|seen| &seen.jwt_id),
        Some(&Some("request-final-static-001".to_owned()))
    );
    let seen = replay.seen.lock().unwrap();
    let seen = seen.as_ref().unwrap();
    assert_eq!(seen.issued_at, NOW - 120);
    assert_eq!(seen.expires_at, Some(NOW + 180));
    assert!(seen.payload_len > 0);
    let routed = evidence
        .into_authorization_request(
            AuthorizationRequestValidationLimits::default(),
            DcqlLimits::default(),
        )
        .expect("typed route");
    assert_eq!(routed.expose_sensitive_client_id(), CLIENT_ID);
}

fn seen_payload_len(request: &Value) -> usize {
    serde_json::to_vec(request).expect("request JSON").len()
}

#[test]
fn dynamic_discovery_requires_one_exact_audience_and_ignores_iss() {
    let dynamic_request = request(json!({
        "iss": "attacker-controlled-and-ignored",
        "aud": ["https://wallet.example"],
        "iat": NOW,
        "jti": "dynamic-001"
    }));
    let replay = Replay::accepting();
    let evidence = block_on(verified(&dynamic_request).validate_security(
        &policy("https://wallet.example"),
        &FixedClock(Ok(NOW)),
        &replay,
    ));
    assert!(evidence.is_ok());
    assert_eq!(replay.calls.load(Ordering::SeqCst), 1);

    let multiple = request(json!({
        "aud": ["https://wallet.example", "https://other.example"],
        "iat": NOW
    }));
    assert_eq!(
        block_on(verified(&multiple).validate_security(
            &policy("https://wallet.example"),
            &FixedClock(Ok(NOW)),
            &Replay::accepting(),
        ))
        .unwrap_err(),
        Oid4vpError::RequestObjectAudienceMismatch
    );
}

#[test]
fn exact_temporal_boundaries_are_accepted() {
    for claims in [
        json!({"aud": STATIC_DISCOVERY_AUDIENCE, "iat": NOW + 30}),
        json!({"aud": STATIC_DISCOVERY_AUDIENCE, "iat": NOW, "nbf": NOW + 30}),
        json!({"aud": STATIC_DISCOVERY_AUDIENCE, "iat": NOW, "exp": NOW - 29}),
        json!({"aud": STATIC_DISCOVERY_AUDIENCE, "iat": NOW - 330}),
    ] {
        let replay = Replay::accepting();
        validate(&request(claims), &FixedClock(Ok(NOW)), &replay).expect("accepted boundary");
        assert_eq!(replay.calls.load(Ordering::SeqCst), 1);
    }
}

#[test]
fn temporal_boundaries_fail_before_replay() {
    let cases = [
        (
            json!({"aud": STATIC_DISCOVERY_AUDIENCE, "iat": NOW + 31}),
            Oid4vpError::RequestObjectIssuedInFuture,
        ),
        (
            json!({"aud": STATIC_DISCOVERY_AUDIENCE, "iat": NOW, "nbf": NOW + 31}),
            Oid4vpError::RequestObjectNotYetValid,
        ),
        (
            json!({"aud": STATIC_DISCOVERY_AUDIENCE, "iat": NOW - 20, "exp": NOW - 30}),
            Oid4vpError::RequestObjectExpired,
        ),
        (
            json!({"aud": STATIC_DISCOVERY_AUDIENCE, "iat": NOW - 331}),
            Oid4vpError::RequestObjectStale,
        ),
    ];
    for (claims, expected) in cases {
        let replay = Replay::accepting();
        assert_eq!(
            validate(&request(claims), &FixedClock(Ok(NOW)), &replay).unwrap_err(),
            expected
        );
        assert_eq!(replay.calls.load(Ordering::SeqCst), 0);
    }
}

#[test]
fn generated_temporal_boundary_properties_hold() {
    for skew in [0_u64, 1, 30, 300] {
        for max_age in [0_u64, 1, 300] {
            let accepted = [
                json!({"aud": STATIC_DISCOVERY_AUDIENCE, "iat": NOW + skew}),
                json!({"aud": STATIC_DISCOVERY_AUDIENCE, "iat": NOW, "nbf": NOW + skew}),
                json!({"aud": STATIC_DISCOVERY_AUDIENCE, "iat": NOW, "exp": NOW - skew + 1}),
                json!({"aud": STATIC_DISCOVERY_AUDIENCE, "iat": NOW - max_age - skew}),
            ];
            for claims in accepted {
                validate_with_policy(&request(claims), max_age, skew, &Replay::accepting())
                    .expect("generated accepted boundary");
            }

            let rejected = [
                (
                    json!({"aud": STATIC_DISCOVERY_AUDIENCE, "iat": NOW + skew + 1}),
                    Oid4vpError::RequestObjectIssuedInFuture,
                ),
                (
                    json!({"aud": STATIC_DISCOVERY_AUDIENCE, "iat": NOW, "nbf": NOW + skew + 1}),
                    Oid4vpError::RequestObjectNotYetValid,
                ),
                (
                    json!({"aud": STATIC_DISCOVERY_AUDIENCE, "iat": NOW, "exp": NOW - skew}),
                    Oid4vpError::RequestObjectExpired,
                ),
                (
                    json!({"aud": STATIC_DISCOVERY_AUDIENCE, "iat": NOW - max_age - skew - 1}),
                    Oid4vpError::RequestObjectStale,
                ),
            ];
            for (claims, expected) in rejected {
                assert_eq!(
                    validate_with_policy(&request(claims), max_age, skew, &Replay::accepting())
                        .unwrap_err(),
                    expected
                );
            }
        }
    }
}

#[test]
fn stateful_replay_guard_accepts_each_payload_once() {
    let request = request(json!({
        "aud": STATIC_DISCOVERY_AUDIENCE,
        "iat": NOW,
        "jti": "atomic-replay-001"
    }));
    let replay = OnceReplay::default();
    validate(&request, &FixedClock(Ok(NOW)), &replay).expect("first acceptance");
    assert_eq!(
        validate(&request, &FixedClock(Ok(NOW)), &replay).unwrap_err(),
        Oid4vpError::RequestObjectReplayRejected
    );
    assert_eq!(
        replay
            .accepted_payloads
            .lock()
            .expect("atomic replay lock")
            .len(),
        1
    );
}

#[test]
fn malformed_mismatch_clock_and_replay_fail_closed() {
    for claims in [
        json!({"aud": STATIC_DISCOVERY_AUDIENCE}),
        json!({"aud": [], "iat": NOW}),
        json!({"aud": [STATIC_DISCOVERY_AUDIENCE, 7], "iat": NOW}),
        json!({"aud": STATIC_DISCOVERY_AUDIENCE, "iat": 1.5}),
        json!({"aud": STATIC_DISCOVERY_AUDIENCE, "iat": NOW, "exp": u64::MAX}),
        json!({"aud": STATIC_DISCOVERY_AUDIENCE, "iat": NOW, "jti": ""}),
    ] {
        assert_eq!(
            validate(&request(claims), &FixedClock(Ok(NOW)), &Replay::accepting()).unwrap_err(),
            Oid4vpError::InvalidRequestObjectSecurityClaims
        );
    }

    assert_eq!(
        validate(
            &request(json!({"aud": "https://wrong.example", "iat": NOW})),
            &FixedClock(Ok(NOW)),
            &Replay::accepting(),
        )
        .unwrap_err(),
        Oid4vpError::RequestObjectAudienceMismatch
    );
    assert_eq!(
        validate(
            &request(json!({"aud": STATIC_DISCOVERY_AUDIENCE, "iat": NOW})),
            &FixedClock(Err(ClockError::Unavailable)),
            &Replay::accepting(),
        )
        .unwrap_err(),
        Oid4vpError::RequestObjectClockUnavailable
    );
    for (failure, expected) in [
        (
            RequestObjectReplayFailure::Rejected,
            Oid4vpError::RequestObjectReplayRejected,
        ),
        (
            RequestObjectReplayFailure::Unavailable,
            Oid4vpError::RequestObjectReplayUnavailable,
        ),
    ] {
        let replay = Replay::failing(failure);
        assert_eq!(
            validate(
                &request(json!({"aud": STATIC_DISCOVERY_AUDIENCE, "iat": NOW})),
                &FixedClock(Ok(NOW)),
                &replay,
            )
            .unwrap_err(),
            expected
        );
        assert_eq!(replay.calls.load(Ordering::SeqCst), 1);
    }
}

#[test]
fn policy_and_diagnostics_are_bounded_and_redacted() {
    let limits = RequestObjectValidationLimits::default();
    assert_eq!(
        RequestObjectSecurityPolicy::new("", 300, 30, limits).unwrap_err(),
        Oid4vpError::InvalidRequestObjectSecurityPolicy
    );
    assert_eq!(
        RequestObjectSecurityPolicy::new("audience", u64::MAX, 1, limits).unwrap_err(),
        Oid4vpError::InvalidRequestObjectSecurityPolicy
    );
    let policy = RequestObjectSecurityPolicy::new(CANARY, 300, 30, limits).unwrap();
    assert_eq!(policy.max_age_seconds(), 300);
    assert_eq!(policy.allowed_clock_skew_seconds(), 30);
    assert!(!format!("{policy:?}").contains(CANARY));

    let request = request(json!({
        "aud": STATIC_DISCOVERY_AUDIENCE,
        "iat": NOW,
        "jti": CANARY
    }));
    let evidence = validate(&request, &FixedClock(Ok(NOW)), &Replay::accepting()).unwrap();
    assert!(!format!("{evidence:?}").contains(CANARY));
    assert!(
        !Oid4vpError::RequestObjectReplayRejected
            .to_string()
            .contains(CANARY)
    );
}

fn block_on<F: Future>(future: F) -> F::Output {
    let waker = Waker::noop();
    let mut context = Context::from_waker(waker);
    let mut future = pin!(future);
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => return output,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}
