use std::{
    collections::BTreeMap,
    future::Future,
    hint::black_box,
    pin::pin,
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    task::{Context, Poll, Wake, Waker},
    time::Instant,
};

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use identus_core::{ClockError, ErrorKind, UnixTimestampMillis, WallClock};
use identus_crypto::{Ed25519PrivateKey, EncodeJwk, P256PrivateKey, PublicKeyJwk};
use identus_did::{
    DereferencedContent, DereferencingOptions, Did, DidDocument, DidDocumentMetadata,
    DidResolutionFuture, DidResolutionMetadata, DidResolutionResult, DidResolver, DidUrl,
    DidUrlContentMetadata, DidUrlDereferencer, DidUrlDereferencingFuture,
    DidUrlDereferencingMetadata, DidUrlDereferencingResult, GenericDidUrlDereferencer,
    ResolutionOptions, VerificationMethod,
};
use identus_jose::{
    Ed25519Signer, JoseError, JwsAlgorithm, JwsKeyReference, JwsSigningInput,
    OID4VCI_PROOF_JWT_TYPE, Oid4vciProofJwtBuilder, Oid4vciProofJwtClaims, Oid4vciProofJwtClient,
    Oid4vciProofJwtLimits, Oid4vciProofJwtNonce, Oid4vciProofJwtPolicy, Oid4vciProofJwtVerifier,
    Oid4vciProofReplayFailure, Oid4vciProofReplayFuture, Oid4vciProofReplayGuard,
    Oid4vciProofReplayInput, Oid4vciX5cKeyFailure, Oid4vciX5cKeyFuture, Oid4vciX5cKeyProvider,
    ProtectedHeader, SignatureSuiteRegistry, error_code,
};
use serde_json::{Value, json};

const SAMPLE_PRIVATE: [u8; 32] = [
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
    0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20,
];
const ISSUED_AT: i64 = 1_700_000_000;
const AUDIENCE: &str = "https://credential-issuer.example";
const NONCE: &str = "server-nonce";

struct NoopWake;

impl Wake for NoopWake {
    fn wake(self: Arc<Self>) {}
}

fn block_on<F: Future>(future: F) -> F::Output {
    let waker = Waker::from(Arc::new(NoopWake));
    let mut context = Context::from_waker(&waker);
    let mut future = pin!(future);
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => return output,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}

fn private_key() -> Ed25519PrivateKey {
    Ed25519PrivateKey::from_slice(&SAMPLE_PRIVATE).expect("Ed25519 private key")
}

fn public_key() -> PublicKeyJwk {
    private_key().to_public_key().encode_jwk()
}

fn proof(
    key_reference: JwsKeyReference,
    client: Oid4vciProofJwtClient,
    audience: &str,
    issued_at: i64,
    nonce: Option<&str>,
) -> String {
    let limits = Oid4vciProofJwtLimits::default();
    let claims = Oid4vciProofJwtClaims::new(
        client,
        audience,
        issued_at,
        nonce.map(str::to_owned),
        limits,
    )
    .expect("valid claims");
    let private = private_key();
    Oid4vciProofJwtBuilder::new(limits)
        .prepare(JwsAlgorithm::Ed25519, key_reference, claims)
        .expect("proof input")
        .sign_with(&Ed25519Signer::new(&private))
        .expect("signed proof")
        .compact()
        .to_owned()
}

fn anonymous_policy(
    audience: &str,
    nonce: Oid4vciProofJwtNonce,
    max_age: u64,
    skew: u64,
) -> Oid4vciProofJwtPolicy {
    Oid4vciProofJwtPolicy::new(
        Oid4vciProofJwtClient::AnonymousPreAuthorized,
        audience,
        nonce,
        max_age,
        skew,
        Oid4vciProofJwtLimits::default(),
    )
    .expect("valid policy")
}

struct FixedClock {
    result: Result<UnixTimestampMillis, ClockError>,
    calls: AtomicUsize,
}

impl FixedClock {
    fn at(seconds: u64) -> Self {
        Self {
            result: Ok(UnixTimestampMillis::new(seconds * 1_000)),
            calls: AtomicUsize::new(0),
        }
    }

    fn unavailable() -> Self {
        Self {
            result: Err(ClockError::Unavailable),
            calls: AtomicUsize::new(0),
        }
    }
}

impl WallClock for FixedClock {
    fn now(&self) -> Result<UnixTimestampMillis, ClockError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        self.result
    }
}

struct RecordingReplay {
    result: Result<(), Oid4vciProofReplayFailure>,
    calls: AtomicUsize,
    debug: Mutex<Vec<String>>,
}

impl RecordingReplay {
    fn accepting() -> Self {
        Self {
            result: Ok(()),
            calls: AtomicUsize::new(0),
            debug: Mutex::new(Vec::new()),
        }
    }

    fn failing(failure: Oid4vciProofReplayFailure) -> Self {
        Self {
            result: Err(failure),
            calls: AtomicUsize::new(0),
            debug: Mutex::new(Vec::new()),
        }
    }
}

impl Oid4vciProofReplayGuard for RecordingReplay {
    fn accept<'a>(&'a self, input: Oid4vciProofReplayInput<'a>) -> Oid4vciProofReplayFuture<'a> {
        Box::pin(async move {
            self.calls.fetch_add(1, Ordering::SeqCst);
            self.debug
                .lock()
                .expect("debug lock")
                .push(format!("{input:?}"));
            self.result
        })
    }
}

struct AcceptAllReplay;

impl Oid4vciProofReplayGuard for AcceptAllReplay {
    fn accept<'a>(&'a self, _input: Oid4vciProofReplayInput<'a>) -> Oid4vciProofReplayFuture<'a> {
        Box::pin(async { Ok(()) })
    }
}

#[derive(Clone)]
enum X5cOutcome {
    Key(PublicKeyJwk),
    Failure(Oid4vciX5cKeyFailure),
}

struct RecordingX5cProvider {
    outcome: X5cOutcome,
    calls: AtomicUsize,
    observations: Mutex<Vec<(JwsAlgorithm, usize)>>,
}

impl RecordingX5cProvider {
    fn key(key: PublicKeyJwk) -> Self {
        Self {
            outcome: X5cOutcome::Key(key),
            calls: AtomicUsize::new(0),
            observations: Mutex::new(Vec::new()),
        }
    }

    fn failing(failure: Oid4vciX5cKeyFailure) -> Self {
        Self {
            outcome: X5cOutcome::Failure(failure),
            calls: AtomicUsize::new(0),
            observations: Mutex::new(Vec::new()),
        }
    }
}

impl Oid4vciX5cKeyProvider for RecordingX5cProvider {
    fn verification_key<'a>(
        &'a self,
        algorithm: JwsAlgorithm,
        chain: &'a [String],
    ) -> Oid4vciX5cKeyFuture<'a> {
        Box::pin(async move {
            self.calls.fetch_add(1, Ordering::SeqCst);
            self.observations
                .lock()
                .expect("observation lock")
                .push((algorithm, chain.len()));
            match &self.outcome {
                X5cOutcome::Key(key) => Ok(key.clone()),
                X5cOutcome::Failure(failure) => Err(*failure),
            }
        })
    }
}

struct RecordingResolver {
    expected: Did,
    result: DidResolutionResult,
    calls: AtomicUsize,
}

impl RecordingResolver {
    fn new(did: &str, result: DidResolutionResult) -> Self {
        Self {
            expected: Did::parse(did).expect("DID"),
            result,
            calls: AtomicUsize::new(0),
        }
    }
}

impl DidResolver for RecordingResolver {
    fn resolve<'a>(
        &'a self,
        did: &'a Did,
        _options: &'a ResolutionOptions,
    ) -> DidResolutionFuture<'a> {
        Box::pin(async move {
            assert_eq!(did, &self.expected);
            self.calls.fetch_add(1, Ordering::SeqCst);
            self.result.clone()
        })
    }
}

struct StaticDereferencer(DidUrlDereferencingResult);

impl DidUrlDereferencer for StaticDereferencer {
    fn dereference<'a>(
        &'a self,
        _did_url: &'a DidUrl,
        _options: &'a DereferencingOptions,
    ) -> DidUrlDereferencingFuture<'a> {
        Box::pin(async move { self.0.clone() })
    }
}

fn dereferenced_method(id: &str, key_material: Value) -> DidUrlDereferencingResult {
    let mut method = json!({
        "id": id,
        "type": "JsonWebKey",
        "controller": "did:midnight:testnet:holder"
    });
    method.as_object_mut().expect("method object").extend(
        key_material
            .as_object()
            .expect("key material object")
            .clone(),
    );
    let method: VerificationMethod = serde_json::from_value(method).expect("verification method");
    DidUrlDereferencingResult::success(
        DidUrlDereferencingMetadata::empty(),
        DereferencedContent::from_verification_method(&method).expect("dereferenced method"),
        DidUrlContentMetadata::empty(),
    )
    .expect("dereferencing result")
}

fn resolved_document(did: &str, key: &PublicKeyJwk, authentication: bool) -> DidResolutionResult {
    let relationship = if authentication {
        json!({ "authentication": [format!("{did}#auth-1")] })
    } else {
        json!({ "assertionMethod": [format!("{did}#auth-1")] })
    };
    let mut document = json!({
        "id": did,
        "verificationMethod": [{
            "id": format!("{did}#auth-1"),
            "type": "JsonWebKey",
            "controller": did,
            "publicKeyJwk": key
        }]
    });
    document.as_object_mut().expect("document object").extend(
        relationship
            .as_object()
            .expect("relationship object")
            .clone(),
    );
    let document = DidDocument::from_json_str(&document.to_string()).expect("DID document");
    DidResolutionResult::success(
        DidResolutionMetadata::empty(),
        document,
        DidDocumentMetadata::empty(),
    )
    .expect("resolution result")
}

fn raw_compact(header: ProtectedHeader, payload: &[u8]) -> String {
    JwsSigningInput::new(
        header,
        payload.to_vec(),
        Oid4vciProofJwtLimits::default().jws(),
    )
    .expect("signing input")
    .attach_signature(vec![0x42; 64])
    .expect("compact")
    .compact()
    .to_owned()
}

#[test]
fn inline_jwk_moves_through_all_three_states() {
    let compact = proof(
        JwsKeyReference::Jwk(public_key()),
        Oid4vciProofJwtClient::AnonymousPreAuthorized,
        AUDIENCE,
        ISSUED_AT,
        Some(NONCE),
    );
    let suites = SignatureSuiteRegistry::recommended();
    let verifier =
        Oid4vciProofJwtVerifier::new(Oid4vciProofJwtLimits::default(), &suites, None, None);
    let parsed = verifier.parse(&compact).expect("parsed profile");
    assert_eq!(parsed.as_unverified().compact(), compact);
    assert_eq!(parsed.claims().audience(), AUDIENCE);

    let verified = block_on(verifier.verify_signature(parsed)).expect("verified signature");
    assert_eq!(verified.proof().algorithm(), JwsAlgorithm::Ed25519);
    let policy = anonymous_policy(
        AUDIENCE,
        Oid4vciProofJwtNonce::required(NONCE, Oid4vciProofJwtLimits::default()).expect("nonce"),
        300,
        60,
    );
    let clock = FixedClock::at(ISSUED_AT as u64 + 10);
    let replay = RecordingReplay::accepting();
    let authorized =
        block_on(verifier.authorize(verified, &policy, &clock, &replay)).expect("authorized proof");

    assert_eq!(authorized.claims().nonce(), Some(NONCE));
    assert_eq!(clock.calls.load(Ordering::SeqCst), 1);
    assert_eq!(replay.calls.load(Ordering::SeqCst), 1);
}

#[test]
fn identified_and_anonymous_client_modes_are_exact() {
    let limits = Oid4vciProofJwtLimits::default();
    let client = Oid4vciProofJwtClient::identified("wallet-client", limits).expect("client");
    let compact = proof(
        JwsKeyReference::Jwk(public_key()),
        client.clone(),
        AUDIENCE,
        ISSUED_AT,
        None,
    );
    let suites = SignatureSuiteRegistry::recommended();
    let verifier = Oid4vciProofJwtVerifier::new(limits, &suites, None, None);
    let verified = block_on(verifier.verify_signature(verifier.parse(&compact).unwrap())).unwrap();
    let identified = Oid4vciProofJwtPolicy::new(
        client,
        AUDIENCE,
        Oid4vciProofJwtNonce::absent(),
        300,
        0,
        limits,
    )
    .unwrap();
    let replay = RecordingReplay::accepting();
    assert!(
        block_on(verifier.authorize(
            verified.clone(),
            &identified,
            &FixedClock::at(ISSUED_AT as u64),
            &replay,
        ))
        .is_ok()
    );

    let anonymous = anonymous_policy(AUDIENCE, Oid4vciProofJwtNonce::absent(), 300, 0);
    let rejected = block_on(verifier.authorize(
        verified,
        &anonymous,
        &FixedClock::at(ISSUED_AT as u64),
        &replay,
    ));
    assert_eq!(rejected, Err(JoseError::ProofClientMismatch));
}

#[test]
fn malformed_profile_and_known_claim_types_fail_before_providers() {
    let limits = Oid4vciProofJwtLimits::default();
    let key = JwsKeyReference::Jwk(public_key());
    let header = |algorithm: &str, type_: Option<&str>, key_reference: Option<JwsKeyReference>| {
        ProtectedHeader::with_key_reference(algorithm, type_, key_reference, limits.jws()).unwrap()
    };
    let cases = [
        (
            raw_compact(
                header("Ed25519", Some("JWT"), Some(key.clone())),
                br#"{"aud":"issuer","iat":1}"#,
            ),
            JoseError::InvalidProofType,
        ),
        (
            raw_compact(
                header("Ed25519", Some(OID4VCI_PROOF_JWT_TYPE), None),
                br#"{"aud":"issuer","iat":1}"#,
            ),
            JoseError::MissingProofKeyReference,
        ),
        (
            raw_compact(
                header("HS256", Some(OID4VCI_PROOF_JWT_TYPE), Some(key.clone())),
                br#"{"aud":"issuer","iat":1}"#,
            ),
            JoseError::UnsupportedAlgorithm,
        ),
        (
            raw_compact(
                header(
                    "EdDSA",
                    Some(OID4VCI_PROOF_JWT_TYPE),
                    Some(JwsKeyReference::X5c(vec!["AQID".to_owned()])),
                ),
                br#"{"aud":"issuer","iat":1}"#,
            ),
            JoseError::AlgorithmNotAllowed,
        ),
        (
            raw_compact(
                header("Ed25519", Some(OID4VCI_PROOF_JWT_TYPE), Some(key.clone())),
                br#"{"aud":"one","aud":"two","iat":1}"#,
            ),
            JoseError::InvalidProofClaims,
        ),
        (
            raw_compact(
                header("Ed25519", Some(OID4VCI_PROOF_JWT_TYPE), Some(key.clone())),
                br#"{"iss":null,"aud":"issuer","iat":1}"#,
            ),
            JoseError::InvalidProofClaims,
        ),
        (
            raw_compact(
                header("Ed25519", Some(OID4VCI_PROOF_JWT_TYPE), Some(key)),
                br#"{"aud":"issuer","iat":1.5}"#,
            ),
            JoseError::InvalidProofClaims,
        ),
    ];
    let suites = SignatureSuiteRegistry::recommended();
    let x5c = RecordingX5cProvider::key(public_key());
    let verifier = Oid4vciProofJwtVerifier::new(limits, &suites, None, Some(&x5c));
    for (compact, expected) in cases {
        assert_eq!(verifier.parse(&compact), Err(expected));
    }
    assert_eq!(x5c.calls.load(Ordering::SeqCst), 0);

    let with_extension = raw_compact(
        header(
            "Ed25519",
            Some(OID4VCI_PROOF_JWT_TYPE),
            Some(JwsKeyReference::Jwk(public_key())),
        ),
        br#"{"aud":"issuer","iat":1,"extension":{"nested":true}}"#,
    );
    assert!(verifier.parse(&with_extension).is_ok());
}

#[test]
fn did_url_key_requires_exact_authentication_relationship() {
    let did = "did:midnight:testnet:holder";
    let method = format!("{did}#auth-1");
    let compact = proof(
        JwsKeyReference::KeyId(method.clone()),
        Oid4vciProofJwtClient::AnonymousPreAuthorized,
        AUDIENCE,
        ISSUED_AT,
        Some(NONCE),
    );
    let suites = SignatureSuiteRegistry::recommended();

    let accepted_resolver = Arc::new(RecordingResolver::new(
        did,
        resolved_document(did, &public_key(), true),
    ));
    let accepted_dereferencer = GenericDidUrlDereferencer::new(accepted_resolver.clone());
    let accepted = Oid4vciProofJwtVerifier::new(
        Oid4vciProofJwtLimits::default(),
        &suites,
        Some(&accepted_dereferencer),
        None,
    );
    let verified = block_on(accepted.verify_signature(accepted.parse(&compact).unwrap()))
        .expect("authenticated DID key");
    assert_eq!(verified.proof().algorithm(), JwsAlgorithm::Ed25519);
    assert_eq!(accepted_resolver.calls.load(Ordering::SeqCst), 1);

    let rejected_resolver = Arc::new(RecordingResolver::new(
        did,
        resolved_document(did, &public_key(), false),
    ));
    let rejected_dereferencer = GenericDidUrlDereferencer::new(rejected_resolver.clone());
    let rejected = Oid4vciProofJwtVerifier::new(
        Oid4vciProofJwtLimits::default(),
        &suites,
        Some(&rejected_dereferencer),
        None,
    );
    assert_eq!(
        block_on(rejected.verify_signature(rejected.parse(&compact).unwrap())),
        Err(JoseError::ProofKeyNotAuthorized)
    );
    assert_eq!(rejected_resolver.calls.load(Ordering::SeqCst), 1);
}

#[test]
fn did_provider_cannot_substitute_method_or_unsupported_key_material() {
    let did = "did:midnight:testnet:holder";
    let selected = format!("{did}#auth-1");
    let compact = proof(
        JwsKeyReference::KeyId(selected),
        Oid4vciProofJwtClient::AnonymousPreAuthorized,
        AUDIENCE,
        ISSUED_AT,
        None,
    );
    let suites = SignatureSuiteRegistry::recommended();

    let substituted = StaticDereferencer(dereferenced_method(
        &format!("{did}#other"),
        json!({ "publicKeyJwk": public_key() }),
    ));
    let verifier = Oid4vciProofJwtVerifier::new(
        Oid4vciProofJwtLimits::default(),
        &suites,
        Some(&substituted),
        None,
    );
    assert_eq!(
        block_on(verifier.verify_signature(verifier.parse(&compact).unwrap())),
        Err(JoseError::ProofKeyResolutionFailed)
    );

    let multibase = StaticDereferencer(dereferenced_method(
        &format!("{did}#auth-1"),
        json!({ "publicKeyMultibase": "z6Mexample" }),
    ));
    let verifier = Oid4vciProofJwtVerifier::new(
        Oid4vciProofJwtLimits::default(),
        &suites,
        Some(&multibase),
        None,
    );
    assert_eq!(
        block_on(verifier.verify_signature(verifier.parse(&compact).unwrap())),
        Err(JoseError::UnsupportedProofKeyReference)
    );
}

#[test]
fn non_did_and_selector_kids_fail_without_dereferencing() {
    let suites = SignatureSuiteRegistry::recommended();
    for kid in [
        "local-key",
        "did:example:holder",
        "did:example:holder/path#key-1",
        "did:example:holder?versionId=1#key-1",
    ] {
        let compact = proof(
            JwsKeyReference::KeyId(kid.to_owned()),
            Oid4vciProofJwtClient::AnonymousPreAuthorized,
            AUDIENCE,
            ISSUED_AT,
            None,
        );
        let verifier =
            Oid4vciProofJwtVerifier::new(Oid4vciProofJwtLimits::default(), &suites, None, None);
        let parsed = verifier.parse(&compact).expect("profile parses");
        assert_eq!(
            block_on(verifier.verify_signature(parsed)),
            Err(JoseError::UnsupportedProofKeyReference),
            "{kid}"
        );
    }
}

#[test]
fn x5c_provider_is_explicit_and_leaf_key_is_rebound() {
    let compact = proof(
        JwsKeyReference::X5c(vec!["AQID".to_owned(), "BAUG".to_owned()]),
        Oid4vciProofJwtClient::AnonymousPreAuthorized,
        AUDIENCE,
        ISSUED_AT,
        None,
    );
    let suites = SignatureSuiteRegistry::recommended();
    let no_provider =
        Oid4vciProofJwtVerifier::new(Oid4vciProofJwtLimits::default(), &suites, None, None);
    assert_eq!(
        block_on(no_provider.verify_signature(no_provider.parse(&compact).unwrap())),
        Err(JoseError::X5cProviderRequired)
    );

    let provider = RecordingX5cProvider::key(public_key());
    let verifier = Oid4vciProofJwtVerifier::new(
        Oid4vciProofJwtLimits::default(),
        &suites,
        None,
        Some(&provider),
    );
    assert!(block_on(verifier.verify_signature(verifier.parse(&compact).unwrap())).is_ok());
    assert_eq!(provider.calls.load(Ordering::SeqCst), 1);
    assert_eq!(
        *provider.observations.lock().unwrap(),
        vec![(JwsAlgorithm::Ed25519, 2)]
    );

    for (failure, expected) in [
        (Oid4vciX5cKeyFailure::Rejected, JoseError::X5cRejected),
        (
            Oid4vciX5cKeyFailure::Unavailable,
            JoseError::X5cProviderUnavailable,
        ),
    ] {
        let provider = RecordingX5cProvider::failing(failure);
        let verifier = Oid4vciProofJwtVerifier::new(
            Oid4vciProofJwtLimits::default(),
            &suites,
            None,
            Some(&provider),
        );
        assert_eq!(
            block_on(verifier.verify_signature(verifier.parse(&compact).unwrap())),
            Err(expected)
        );
    }

    let p256 = P256PrivateKey::from_slice(&SAMPLE_PRIVATE)
        .expect("P-256 key")
        .to_public_key()
        .encode_jwk();
    let provider = RecordingX5cProvider::key(p256);
    let verifier = Oid4vciProofJwtVerifier::new(
        Oid4vciProofJwtLimits::default(),
        &suites,
        None,
        Some(&provider),
    );
    assert_eq!(
        block_on(verifier.verify_signature(verifier.parse(&compact).unwrap())),
        Err(JoseError::InvalidVerificationKey)
    );
}

#[test]
fn issuer_policy_failures_are_independent_and_never_touch_replay() {
    let compact = proof(
        JwsKeyReference::Jwk(public_key()),
        Oid4vciProofJwtClient::AnonymousPreAuthorized,
        AUDIENCE,
        ISSUED_AT,
        Some(NONCE),
    );
    let suites = SignatureSuiteRegistry::recommended();
    let verifier =
        Oid4vciProofJwtVerifier::new(Oid4vciProofJwtLimits::default(), &suites, None, None);
    let verified = block_on(verifier.verify_signature(verifier.parse(&compact).unwrap())).unwrap();
    let replay = RecordingReplay::accepting();
    let limits = Oid4vciProofJwtLimits::default();

    let identified = Oid4vciProofJwtPolicy::new(
        Oid4vciProofJwtClient::identified("other-client", limits).unwrap(),
        AUDIENCE,
        Oid4vciProofJwtNonce::required(NONCE, limits).unwrap(),
        300,
        60,
        limits,
    )
    .unwrap();
    let wrong_audience = anonymous_policy(
        "https://other-issuer.example",
        Oid4vciProofJwtNonce::required(NONCE, limits).unwrap(),
        300,
        60,
    );
    let wrong_nonce = anonymous_policy(
        AUDIENCE,
        Oid4vciProofJwtNonce::required("other-nonce", limits).unwrap(),
        300,
        60,
    );
    let absent_nonce = anonymous_policy(AUDIENCE, Oid4vciProofJwtNonce::absent(), 300, 60);
    let cases = [
        (
            identified,
            FixedClock::at(ISSUED_AT as u64),
            JoseError::ProofClientMismatch,
        ),
        (
            wrong_audience,
            FixedClock::at(ISSUED_AT as u64),
            JoseError::ProofAudienceMismatch,
        ),
        (
            wrong_nonce,
            FixedClock::at(ISSUED_AT as u64),
            JoseError::ProofNonceMismatch,
        ),
        (
            absent_nonce,
            FixedClock::at(ISSUED_AT as u64),
            JoseError::ProofNonceMismatch,
        ),
        (
            anonymous_policy(
                AUDIENCE,
                Oid4vciProofJwtNonce::required(NONCE, limits).unwrap(),
                300,
                60,
            ),
            FixedClock::at(ISSUED_AT as u64 + 361),
            JoseError::ProofStale,
        ),
        (
            anonymous_policy(
                AUDIENCE,
                Oid4vciProofJwtNonce::required(NONCE, limits).unwrap(),
                300,
                60,
            ),
            FixedClock::at(ISSUED_AT as u64 - 61),
            JoseError::ProofIssuedInFuture,
        ),
        (
            anonymous_policy(
                AUDIENCE,
                Oid4vciProofJwtNonce::required(NONCE, limits).unwrap(),
                300,
                60,
            ),
            FixedClock::unavailable(),
            JoseError::ProofClockUnavailable,
        ),
    ];

    for (policy, clock, expected) in cases {
        assert_eq!(
            block_on(verifier.authorize(verified.clone(), &policy, &clock, &replay)),
            Err(expected)
        );
    }
    assert_eq!(replay.calls.load(Ordering::SeqCst), 0);
}

#[test]
fn freshness_boundaries_are_inclusive_and_negative_iat_is_rejected() {
    let limits = Oid4vciProofJwtLimits::default();
    let suites = SignatureSuiteRegistry::recommended();
    let verifier = Oid4vciProofJwtVerifier::new(limits, &suites, None, None);
    let policy = anonymous_policy(AUDIENCE, Oid4vciProofJwtNonce::absent(), 300, 60);
    let replay = RecordingReplay::accepting();

    for (issued_at, now) in [(ISSUED_AT, ISSUED_AT + 360), (ISSUED_AT + 60, ISSUED_AT)] {
        let compact = proof(
            JwsKeyReference::Jwk(public_key()),
            Oid4vciProofJwtClient::AnonymousPreAuthorized,
            AUDIENCE,
            issued_at,
            None,
        );
        assert!(
            block_on(verifier.verify_and_authorize(
                &compact,
                &policy,
                &FixedClock::at(now as u64),
                &replay,
            ))
            .is_ok()
        );
    }

    let negative = proof(
        JwsKeyReference::Jwk(public_key()),
        Oid4vciProofJwtClient::AnonymousPreAuthorized,
        AUDIENCE,
        -1,
        None,
    );
    assert_eq!(
        block_on(verifier.verify_and_authorize(&negative, &policy, &FixedClock::at(0), &replay,)),
        Err(JoseError::ProofStale)
    );
}

#[test]
fn convenience_path_rejects_policy_before_x5c_and_replay_providers() {
    let compact = proof(
        JwsKeyReference::X5c(vec!["AQID".to_owned()]),
        Oid4vciProofJwtClient::AnonymousPreAuthorized,
        AUDIENCE,
        ISSUED_AT,
        None,
    );
    let suites = SignatureSuiteRegistry::recommended();
    let x5c = RecordingX5cProvider::key(public_key());
    let verifier =
        Oid4vciProofJwtVerifier::new(Oid4vciProofJwtLimits::default(), &suites, None, Some(&x5c));
    let policy = anonymous_policy(
        "https://wrong-issuer.example",
        Oid4vciProofJwtNonce::absent(),
        300,
        60,
    );
    let clock = FixedClock::at(ISSUED_AT as u64);
    let replay = RecordingReplay::accepting();

    assert_eq!(
        block_on(verifier.verify_and_authorize(&compact, &policy, &clock, &replay)),
        Err(JoseError::ProofAudienceMismatch)
    );
    assert_eq!(clock.calls.load(Ordering::SeqCst), 0);
    assert_eq!(x5c.calls.load(Ordering::SeqCst), 0);
    assert_eq!(replay.calls.load(Ordering::SeqCst), 0);
}

#[test]
fn invalid_signature_and_replay_results_fail_closed() {
    let compact = proof(
        JwsKeyReference::Jwk(public_key()),
        Oid4vciProofJwtClient::AnonymousPreAuthorized,
        AUDIENCE,
        ISSUED_AT,
        None,
    );
    let mut segments = compact.split('.');
    let header = segments.next().unwrap();
    let payload = segments.next().unwrap();
    let mut signature = URL_SAFE_NO_PAD.decode(segments.next().unwrap()).unwrap();
    signature[0] ^= 1;
    let tampered = format!("{header}.{payload}.{}", URL_SAFE_NO_PAD.encode(signature));
    let suites = SignatureSuiteRegistry::recommended();
    let verifier =
        Oid4vciProofJwtVerifier::new(Oid4vciProofJwtLimits::default(), &suites, None, None);
    assert_eq!(
        block_on(verifier.verify_signature(verifier.parse(&tampered).unwrap())),
        Err(JoseError::SignatureInvalid)
    );

    let policy = anonymous_policy(AUDIENCE, Oid4vciProofJwtNonce::absent(), 300, 0);
    let clock = FixedClock::at(ISSUED_AT as u64);
    for (failure, expected) in [
        (
            Oid4vciProofReplayFailure::Rejected,
            JoseError::ProofReplayRejected,
        ),
        (
            Oid4vciProofReplayFailure::Unavailable,
            JoseError::ProofReplayUnavailable,
        ),
    ] {
        let replay = RecordingReplay::failing(failure);
        assert_eq!(
            block_on(verifier.verify_and_authorize(&compact, &policy, &clock, &replay)),
            Err(expected)
        );
        assert_eq!(replay.calls.load(Ordering::SeqCst), 1);
    }
}

#[test]
fn policy_and_state_diagnostics_are_redacted() {
    let limits = Oid4vciProofJwtLimits::default();
    let issuer = "ISSUER_CANARY";
    let audience = "AUDIENCE_CANARY";
    let nonce = "NONCE_CANARY";
    let certificate = "Q0VSVF9DQU5BUlk=";
    let compact = proof(
        JwsKeyReference::X5c(vec![certificate.to_owned()]),
        Oid4vciProofJwtClient::identified(issuer, limits).unwrap(),
        audience,
        ISSUED_AT,
        Some(nonce),
    );
    let suites = SignatureSuiteRegistry::recommended();
    let x5c = RecordingX5cProvider::key(public_key());
    let verifier = Oid4vciProofJwtVerifier::new(limits, &suites, None, Some(&x5c));
    let parsed = verifier.parse(&compact).unwrap();
    let parsed_debug = format!("{parsed:?}");
    let verified = block_on(verifier.verify_signature(parsed)).unwrap();
    let verified_debug = format!("{verified:?}");
    let policy = Oid4vciProofJwtPolicy::new(
        Oid4vciProofJwtClient::identified(issuer, limits).unwrap(),
        audience,
        Oid4vciProofJwtNonce::required(nonce, limits).unwrap(),
        300,
        60,
        limits,
    )
    .unwrap();
    let policy_debug = format!("{policy:?}");
    let replay = RecordingReplay::accepting();
    let authorized = block_on(verifier.authorize(
        verified,
        &policy,
        &FixedClock::at(ISSUED_AT as u64),
        &replay,
    ))
    .unwrap();
    let diagnostics = format!(
        "{parsed_debug} {verified_debug} {policy_debug} {authorized:?} {:?} {}",
        JoseError::ProofNonceMismatch,
        JoseError::ProofNonceMismatch
    );
    let replay_debug = replay.debug.lock().unwrap().join(" ");

    for canary in [
        issuer,
        audience,
        nonce,
        certificate,
        compact.as_str(),
        &ISSUED_AT.to_string(),
    ] {
        assert!(!diagnostics.contains(canary), "diagnostic leaked {canary}");
        assert!(
            !replay_debug.contains(canary),
            "replay Debug leaked {canary}"
        );
    }
}

#[test]
fn verifier_errors_bridge_to_stable_static_contracts() {
    let cases = [
        (
            JoseError::InvalidProofType,
            error_code::INVALID_PROOF_TYPE,
            ErrorKind::InvalidInput,
        ),
        (
            JoseError::MissingProofKeyReference,
            error_code::MISSING_PROOF_KEY_REFERENCE,
            ErrorKind::InvalidInput,
        ),
        (
            JoseError::UnsupportedProofKeyReference,
            error_code::UNSUPPORTED_PROOF_KEY_REFERENCE,
            ErrorKind::Unsupported,
        ),
        (
            JoseError::ProofKeyResolutionFailed,
            error_code::PROOF_KEY_RESOLUTION_FAILED,
            ErrorKind::VerificationFailed,
        ),
        (
            JoseError::ProofKeyNotAuthorized,
            error_code::PROOF_KEY_NOT_AUTHORIZED,
            ErrorKind::VerificationFailed,
        ),
        (
            JoseError::X5cProviderRequired,
            error_code::X5C_PROVIDER_REQUIRED,
            ErrorKind::InvalidInput,
        ),
        (
            JoseError::X5cRejected,
            error_code::X5C_REJECTED,
            ErrorKind::VerificationFailed,
        ),
        (
            JoseError::X5cProviderUnavailable,
            error_code::X5C_PROVIDER_UNAVAILABLE,
            ErrorKind::Internal,
        ),
        (
            JoseError::InvalidProofPolicy,
            error_code::INVALID_PROOF_POLICY,
            ErrorKind::InvalidInput,
        ),
        (
            JoseError::ProofClientMismatch,
            error_code::PROOF_CLIENT_MISMATCH,
            ErrorKind::VerificationFailed,
        ),
        (
            JoseError::ProofAudienceMismatch,
            error_code::PROOF_AUDIENCE_MISMATCH,
            ErrorKind::VerificationFailed,
        ),
        (
            JoseError::ProofNonceMismatch,
            error_code::PROOF_NONCE_MISMATCH,
            ErrorKind::VerificationFailed,
        ),
        (
            JoseError::ProofStale,
            error_code::PROOF_STALE,
            ErrorKind::VerificationFailed,
        ),
        (
            JoseError::ProofIssuedInFuture,
            error_code::PROOF_ISSUED_IN_FUTURE,
            ErrorKind::VerificationFailed,
        ),
        (
            JoseError::ProofClockUnavailable,
            error_code::PROOF_CLOCK_UNAVAILABLE,
            ErrorKind::Internal,
        ),
        (
            JoseError::ProofReplayRejected,
            error_code::PROOF_REPLAY_REJECTED,
            ErrorKind::VerificationFailed,
        ),
        (
            JoseError::ProofReplayUnavailable,
            error_code::PROOF_REPLAY_UNAVAILABLE,
            ErrorKind::Internal,
        ),
    ];

    for (source, code, kind) in cases {
        let bridged = source.to_identus_error();
        assert_eq!(bridged.code(), code);
        assert_eq!(bridged.kind(), kind);
        assert_eq!(bridged.capability(), Some(identus_jose::CAPABILITY));
    }
}

#[test]
fn policy_construction_rejects_invalid_and_overflowing_inputs() {
    let limits = Oid4vciProofJwtLimits::default();
    assert_eq!(
        Oid4vciProofJwtNonce::required("", limits),
        Err(JoseError::InvalidProofPolicy)
    );
    assert_eq!(
        Oid4vciProofJwtPolicy::new(
            Oid4vciProofJwtClient::AnonymousPreAuthorized,
            "",
            Oid4vciProofJwtNonce::absent(),
            1,
            1,
            limits,
        ),
        Err(JoseError::InvalidProofPolicy)
    );
    assert_eq!(
        Oid4vciProofJwtPolicy::new(
            Oid4vciProofJwtClient::AnonymousPreAuthorized,
            AUDIENCE,
            Oid4vciProofJwtNonce::absent(),
            u64::MAX,
            1,
            limits,
        ),
        Err(JoseError::InvalidProofPolicy)
    );
}

#[test]
#[ignore = "manual release diagnostic"]
fn issuer_verification_throughput_diagnostic() {
    const ITERATIONS: usize = 20_000;
    let compact = proof(
        JwsKeyReference::Jwk(public_key()),
        Oid4vciProofJwtClient::AnonymousPreAuthorized,
        AUDIENCE,
        ISSUED_AT,
        Some(NONCE),
    );
    let suites = SignatureSuiteRegistry::recommended();
    let verifier =
        Oid4vciProofJwtVerifier::new(Oid4vciProofJwtLimits::default(), &suites, None, None);
    let policy = anonymous_policy(
        AUDIENCE,
        Oid4vciProofJwtNonce::required(NONCE, Oid4vciProofJwtLimits::default()).unwrap(),
        300,
        60,
    );
    let clock = FixedClock::at(ISSUED_AT as u64 + 10);
    let replay = AcceptAllReplay;
    let started = Instant::now();
    for _ in 0..ITERATIONS {
        let accepted = block_on(verifier.verify_and_authorize(
            black_box(&compact),
            black_box(&policy),
            black_box(&clock),
            black_box(&replay),
        ));
        black_box(accepted.expect("accepted proof"));
    }
    let elapsed = started.elapsed();
    println!(
        "verified and authorized {ITERATIONS} OID4VCI proofs in {elapsed:?} ({:.0} operations/s)",
        ITERATIONS as f64 / elapsed.as_secs_f64()
    );
}

#[test]
fn proof_claim_parser_accepts_bounded_unknown_values_without_exposing_them() {
    let limits = Oid4vciProofJwtLimits::default();
    let header = ProtectedHeader::with_key_reference(
        "Ed25519",
        Some(OID4VCI_PROOF_JWT_TYPE),
        Some(JwsKeyReference::Jwk(public_key())),
        limits.jws(),
    )
    .unwrap();
    let payload = json!({
        "aud": AUDIENCE,
        "iat": ISSUED_AT,
        "nonce": NONCE,
        "extension": {
            "values": [true, false, Value::Null],
            "metadata": BTreeMap::from([("safe", "bounded")])
        }
    });
    let compact = raw_compact(header, payload.to_string().as_bytes());
    let suites = SignatureSuiteRegistry::recommended();
    let verifier = Oid4vciProofJwtVerifier::new(limits, &suites, None, None);
    let parsed = verifier.parse(&compact).expect("extension ignored");
    assert_eq!(parsed.claims().audience(), AUDIENCE);
    assert_eq!(parsed.claims().nonce(), Some(NONCE));
}
