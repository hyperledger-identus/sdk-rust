use std::{
    future::Future,
    pin::pin,
    sync::atomic::{AtomicUsize, Ordering},
    task::{Context, Poll, Waker},
};

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use identus_core::{ClockError, UnixTimestampMillis, WallClock};
use identus_crypto::{Ed25519PrivateKey, EncodeJwk, JwkKeyType};
use identus_jose::{
    Ed25519Signer, JwsAlgorithm, JwsKeyReference, JwsLimits, JwsSigningInput, JwsVerificationKey,
    ProtectedHeader, SignatureSuiteRegistry,
};
use identus_oid4vp::{
    AuthorizationRequestInvocation, AuthorizationRequestInvocationLimits,
    AuthorizationRequestValidationLimits, ClientIdentifierScheme, DcqlLimits, Oid4vpError,
    REQUEST_OBJECT_JWT_TYPE, REQUEST_OBJECT_MEDIA_TYPE, RequestObjectReplayFuture,
    RequestObjectReplayGuard, RequestObjectReplayInput, RequestObjectSecurityPolicy,
    RequestObjectValidationLimits, RequestUriRetrievalInput, RequestUriRetrievalLimits,
    STATIC_DISCOVERY_AUDIENCE, VerifierAuthorizationEvidence, VerifierAuthorizationLimits,
    VerifierTrustFailure, VerifierTrustFuture, VerifierTrustInput, VerifierTrustPolicy,
};
use serde_json::{Value, json};

const PRIVATE_BYTES: [u8; 32] = [0x5a; 32];
const NOW: u64 = 1_735_689_600;
const CANARY: &str = "VERIFIER_AUTHORIZATION_SECRET_CANARY";

struct FixedClock;

impl WallClock for FixedClock {
    fn now(&self) -> Result<UnixTimestampMillis, ClockError> {
        Ok(UnixTimestampMillis::new(NOW * 1_000))
    }
}

struct AcceptReplay;

impl RequestObjectReplayGuard for AcceptReplay {
    fn accept<'a>(&'a self, _input: RequestObjectReplayInput<'a>) -> RequestObjectReplayFuture<'a> {
        Box::pin(async { Ok(()) })
    }
}

struct InspectPolicy {
    expected: ClientIdentifierScheme,
    outcome: Result<(), VerifierTrustFailure>,
    calls: AtomicUsize,
    expect_metadata: bool,
    expect_trust_chain: bool,
}

impl InspectPolicy {
    fn accepting(expected: ClientIdentifierScheme) -> Self {
        Self {
            expected,
            outcome: Ok(()),
            calls: AtomicUsize::new(0),
            expect_metadata: !matches!(
                expected,
                ClientIdentifierScheme::OpenIdFederation | ClientIdentifierScheme::PreRegistered
            ),
            expect_trust_chain: expected == ClientIdentifierScheme::OpenIdFederation,
        }
    }
}

impl VerifierTrustPolicy for InspectPolicy {
    fn authorize<'a>(&'a self, input: VerifierTrustInput<'a>) -> VerifierTrustFuture<'a> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        assert_eq!(input.scheme(), self.expected);
        assert!(!input.expose_sensitive_full_client_id().is_empty());
        assert!(!input.expose_sensitive_original_client_id().is_empty());
        assert_eq!(input.verification_key().kty(), JwkKeyType::Okp);
        assert_eq!(input.metadata().is_some(), self.expect_metadata);
        assert_eq!(input.verifier_info().len(), 1);
        let info = &input.verifier_info()[0];
        assert_eq!(info.expose_sensitive_format(), "example-attestation+jwt");
        assert_eq!(
            info.expose_sensitive_credential_ids().collect::<Vec<_>>(),
            vec!["pid"]
        );
        assert_eq!(
            input.expose_sensitive_trust_chain().count() > 0,
            self.expect_trust_chain
        );
        let outcome = self.outcome;
        Box::pin(async move { outcome })
    }
}

#[test]
fn every_signed_final_scheme_binds_the_exact_verified_key_before_routing() {
    let hash = URL_SAFE_NO_PAD.encode([0_u8; 32]);
    let cases = [
        (
            "registered-client".to_owned(),
            ClientIdentifierScheme::PreRegistered,
            HeaderKind::Kid("registered-key"),
        ),
        (
            "openid_federation:https://verifier.example".to_owned(),
            ClientIdentifierScheme::OpenIdFederation,
            HeaderKind::Kid("federation-key"),
        ),
        (
            "decentralized_identifier:did:example:verifier".to_owned(),
            ClientIdentifierScheme::DecentralizedIdentifier,
            HeaderKind::Kid("did:example:verifier#key-1"),
        ),
        (
            "verifier_attestation:verifier.example".to_owned(),
            ClientIdentifierScheme::VerifierAttestation,
            HeaderKind::Attestation,
        ),
        (
            "x509_san_dns:verifier.example".to_owned(),
            ClientIdentifierScheme::X509SanDns,
            HeaderKind::X5c,
        ),
        (
            format!("x509_hash:{hash}"),
            ClientIdentifierScheme::X509Hash,
            HeaderKind::X5c,
        ),
    ];

    for (client_id, scheme, header) in cases {
        let policy = InspectPolicy::accepting(scheme);
        let evidence = secure(&client_id, request(&client_id, scheme, true), header);
        let authorized =
            block_on(evidence.authorize_verifier(&policy, VerifierAuthorizationLimits::default()))
                .expect("scheme authorization");
        assert_eq!(authorized.scheme(), scheme);
        assert_eq!(policy.calls.load(Ordering::SeqCst), 1);
        let routed = authorized
            .into_authorization_request(
                AuthorizationRequestValidationLimits::default(),
                DcqlLimits::default(),
            )
            .expect("typed routing");
        assert_eq!(routed.expose_sensitive_client_id(), client_id);
    }
}

#[test]
fn unsupported_and_invalid_schemes_fail_before_policy() {
    for client_id in [
        "redirect_uri:https://verifier.example/cb",
        "origin:https://verifier.example",
        "example_extension:value",
    ] {
        let policy = InspectPolicy::accepting(ClientIdentifierScheme::PreRegistered);
        let error = block_on(
            secure(
                client_id,
                request(client_id, ClientIdentifierScheme::PreRegistered, true),
                HeaderKind::Kid("key"),
            )
            .authorize_verifier(&policy, VerifierAuthorizationLimits::default()),
        )
        .unwrap_err();
        assert_eq!(error, Oid4vpError::UnsupportedClientIdentifierScheme);
        assert_eq!(policy.calls.load(Ordering::SeqCst), 0);
    }

    let invalid = [
        (
            "decentralized_identifier:did:example:verifier",
            HeaderKind::Kid("did:example:other#key-1"),
        ),
        (
            "verifier_attestation:verifier.example",
            HeaderKind::Kid("key"),
        ),
        ("x509_san_dns:-bad.example", HeaderKind::X5c),
        ("x509_hash:not-a-hash", HeaderKind::X5c),
    ];
    for (client_id, header) in invalid {
        let policy = InspectPolicy::accepting(ClientIdentifierScheme::PreRegistered);
        let error = block_on(
            secure(
                client_id,
                request(client_id, ClientIdentifierScheme::PreRegistered, true),
                header,
            )
            .authorize_verifier(&policy, VerifierAuthorizationLimits::default()),
        )
        .unwrap_err();
        assert_eq!(error, Oid4vpError::InvalidClientIdentifier);
        assert_eq!(policy.calls.load(Ordering::SeqCst), 0);
    }
}

#[test]
fn metadata_references_limits_and_source_rules_fail_closed() {
    let client_id = "decentralized_identifier:did:example:verifier";
    let mut unknown = request(
        client_id,
        ClientIdentifierScheme::DecentralizedIdentifier,
        true,
    );
    unknown["verifier_info"][0]["credential_ids"] = json!(["unknown"]);
    assert_metadata_error(client_id, unknown, Oid4vpError::InvalidVerifierMetadata);

    let mut wrong_data = request(
        client_id,
        ClientIdentifierScheme::DecentralizedIdentifier,
        true,
    );
    wrong_data["verifier_info"][0]["data"] = json!(42);
    assert_metadata_error(client_id, wrong_data, Oid4vpError::InvalidVerifierMetadata);

    let limits = VerifierAuthorizationLimits::new(8, 1, 1, 64, 1).expect("limits");
    let policy = InspectPolicy::accepting(ClientIdentifierScheme::DecentralizedIdentifier);
    let error = block_on(
        secure(
            client_id,
            request(
                client_id,
                ClientIdentifierScheme::DecentralizedIdentifier,
                true,
            ),
            HeaderKind::Kid("did:example:verifier#key-1"),
        )
        .authorize_verifier(&policy, limits),
    )
    .unwrap_err();
    assert_eq!(error, Oid4vpError::VerifierMetadataTooLarge);
    assert_eq!(policy.calls.load(Ordering::SeqCst), 0);

    let federation = "openid_federation:https://verifier.example";
    let policy = InspectPolicy::accepting(ClientIdentifierScheme::OpenIdFederation);
    let authorized = block_on(
        secure(
            federation,
            request(federation, ClientIdentifierScheme::OpenIdFederation, true),
            HeaderKind::Kid("federation-key"),
        )
        .authorize_verifier(&policy, VerifierAuthorizationLimits::default()),
    );
    assert!(
        authorized.is_ok(),
        "inline metadata is ignored for Federation"
    );

    for (client_id, scheme, header) in [
        (
            "registered-client",
            ClientIdentifierScheme::PreRegistered,
            HeaderKind::Kid("registered-key"),
        ),
        (
            federation,
            ClientIdentifierScheme::OpenIdFederation,
            HeaderKind::Kid("federation-key"),
        ),
    ] {
        let mut encrypted = request(client_id, scheme, true);
        encrypted["response_mode"] = json!("direct_post.jwt");
        let policy = InspectPolicy::accepting(scheme);
        let authorized = block_on(
            secure(client_id, encrypted, header)
                .authorize_verifier(&policy, VerifierAuthorizationLimits::default()),
        )
        .expect("verifier policy");
        assert_eq!(
            authorized
                .into_authorization_request(
                    AuthorizationRequestValidationLimits::default(),
                    DcqlLimits::default(),
                )
                .unwrap_err(),
            Oid4vpError::InvalidVerifierMetadata,
            "untrusted inline metadata must not select encryption keys"
        );
    }
}

#[test]
fn trust_rejection_unavailability_and_diagnostics_are_static() {
    let client_id = "registered-client";
    for (failure, expected) in [
        (
            VerifierTrustFailure::Rejected,
            Oid4vpError::VerifierTrustRejected,
        ),
        (
            VerifierTrustFailure::Unavailable,
            Oid4vpError::VerifierTrustUnavailable,
        ),
    ] {
        let policy = InspectPolicy {
            outcome: Err(failure),
            ..InspectPolicy::accepting(ClientIdentifierScheme::PreRegistered)
        };
        let error = block_on(
            secure(
                client_id,
                request(client_id, ClientIdentifierScheme::PreRegistered, true),
                HeaderKind::Kid("key"),
            )
            .authorize_verifier(&policy, VerifierAuthorizationLimits::default()),
        )
        .unwrap_err();
        assert_eq!(error, expected);
        for rendered in [format!("{error}"), format!("{error:?}")] {
            assert!(!rendered.contains(CANARY));
            assert!(!rendered.contains(client_id));
        }
    }
}

fn assert_metadata_error(client_id: &str, request: Value, expected: Oid4vpError) {
    let policy = InspectPolicy::accepting(ClientIdentifierScheme::DecentralizedIdentifier);
    let error = block_on(
        secure(
            client_id,
            request,
            HeaderKind::Kid("did:example:verifier#key-1"),
        )
        .authorize_verifier(&policy, VerifierAuthorizationLimits::default()),
    )
    .unwrap_err();
    assert_eq!(error, expected);
    assert_eq!(policy.calls.load(Ordering::SeqCst), 0);
}

fn request(client_id: &str, scheme: ClientIdentifierScheme, metadata: bool) -> Value {
    let mut value = json!({
        "client_id": client_id,
        "aud": STATIC_DISCOVERY_AUDIENCE,
        "iat": NOW,
        "response_type": "vp_token",
        "response_mode": "direct_post",
        "nonce": "fresh-nonce",
        "response_uri": "https://verifier.example/response",
        "dcql_query": {"credentials": [{
            "id": "pid",
            "format": "dc+sd-jwt",
            "meta": {}
        }]},
        "verifier_info": [{
            "format": "example-attestation+jwt",
            "data": {"token": CANARY},
            "credential_ids": ["pid"]
        }]
    });
    if metadata {
        value["client_metadata"] = json!({"display_name": CANARY});
    }
    if scheme == ClientIdentifierScheme::OpenIdFederation {
        value["trust_chain"] = json!(["e30.e30.AQ"]);
    }
    value
}

#[derive(Clone, Copy)]
enum HeaderKind {
    Kid(&'static str),
    Attestation,
    X5c,
}

fn secure(
    client_id: &str,
    request: Value,
    header_kind: HeaderKind,
) -> identus_oid4vp::RequestObjectSecurityEvidence {
    let private = Ed25519PrivateKey::from_slice(&PRIVATE_BYTES).expect("fixed key");
    let header = match header_kind {
        HeaderKind::Kid(kid) => ProtectedHeader::new(
            "Ed25519",
            Some(REQUEST_OBJECT_JWT_TYPE),
            Some(kid),
            JwsLimits::default(),
        ),
        HeaderKind::Attestation => ProtectedHeader::with_key_reference_and_extended_evidence(
            "Ed25519",
            Some(REQUEST_OBJECT_JWT_TYPE),
            Some(JwsKeyReference::key_id("attested-key", JwsLimits::default()).expect("kid")),
            Some("e30.e30.AQ".to_owned()),
            None,
            None,
            JwsLimits::default(),
        ),
        HeaderKind::X5c => ProtectedHeader::with_key_reference(
            "Ed25519",
            Some(REQUEST_OBJECT_JWT_TYPE),
            Some(JwsKeyReference::x5c(vec!["AQ==".to_owned()], JwsLimits::default()).expect("x5c")),
            JwsLimits::default(),
        ),
    }
    .expect("header");
    let compact = JwsSigningInput::new(
        header,
        serde_json::to_vec(&request).expect("request JSON"),
        JwsLimits::default(),
    )
    .expect("input")
    .sign_with(&Ed25519Signer::new(&private))
    .expect("signature")
    .compact()
    .to_owned();
    let reference = format!(
        "openid4vp:?client_id={client_id}&request_uri=https%3A%2F%2Fverifier.example%2Frequest"
    );
    let AuthorizationRequestInvocation::Referenced(reference) =
        AuthorizationRequestInvocation::parse(
            &reference,
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
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).expect("key");
    let verified = object
        .verify(&SignatureSuiteRegistry::recommended(), &key)
        .expect("verified");
    block_on(
        verified.validate_security(
            &RequestObjectSecurityPolicy::static_discovery(
                300,
                30,
                RequestObjectValidationLimits::default(),
            )
            .expect("policy"),
            &FixedClock,
            &AcceptReplay,
        ),
    )
    .expect("secured")
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

fn _assert_send_sync(_: &VerifierAuthorizationEvidence) {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<VerifierAuthorizationEvidence>();
}
