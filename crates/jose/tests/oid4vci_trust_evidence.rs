use std::{
    future::Future,
    pin::pin,
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    task::{Context, Poll, Wake, Waker},
};

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use identus_core::{ClockError, UnixTimestampMillis, WallClock};
use identus_crypto::{Ed25519PrivateKey, EncodeJwk, P256PrivateKey, PublicKeyJwk};
use identus_jose::{
    Ed25519Signer, JoseError, JwsAlgorithm, JwsKeyReference, Oid4vciKeyAttestationFailure,
    Oid4vciKeyAttestationFuture, Oid4vciKeyAttestationInput, Oid4vciKeyAttestationValidator,
    Oid4vciProofJwtBuilder, Oid4vciProofJwtClaims, Oid4vciProofJwtClient, Oid4vciProofJwtEvidence,
    Oid4vciProofJwtLimits, Oid4vciProofJwtNonce, Oid4vciProofJwtPolicy, Oid4vciProofJwtVerifier,
    Oid4vciProofReplayFuture, Oid4vciProofReplayGuard, Oid4vciProofReplayInput,
    Oid4vciTrustChainFailure, Oid4vciTrustChainKeyFuture, Oid4vciTrustChainKeyProvider,
    SignatureSuiteRegistry,
};

const PRIVATE_BYTES: [u8; 32] = [0x41; 32];
const ATTESTATION: &str = "eyJ0eXAiOiJrZXktYXR0ZXN0YXRpb24rand0In0.e30.AQ";
const ENTITY_CONFIGURATION: &str = "eyJ0eXAiOiJlbnRpdHktc3RhdGVtZW50K2p3dCJ9.e30.AQ";
const SUBORDINATE_STATEMENT: &str =
    "eyJ0eXAiOiJlbnRpdHktc3RhdGVtZW50K2p3dCJ9.eyJzdWIiOiJsZWFmIn0.Ag";
const KEY_ID: &str = "federation-proof-key";
const AUDIENCE: &str = "https://credential-issuer.example";
const NONCE: &str = "server-nonce";
const ISSUED_AT: i64 = 1_700_000_000;

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
    Ed25519PrivateKey::from_slice(&PRIVATE_BYTES).expect("private key")
}

fn public_key() -> PublicKeyJwk {
    private_key().to_public_key().encode_jwk()
}

fn claims() -> Oid4vciProofJwtClaims {
    Oid4vciProofJwtClaims::new(
        Oid4vciProofJwtClient::AnonymousPreAuthorized,
        AUDIENCE,
        ISSUED_AT,
        Some(NONCE.to_owned()),
        Oid4vciProofJwtLimits::default(),
    )
    .expect("claims")
}

fn evidence(attestation: bool, trust_chain: bool) -> Oid4vciProofJwtEvidence {
    Oid4vciProofJwtEvidence::new(
        attestation.then(|| ATTESTATION.to_owned()),
        trust_chain.then(|| {
            vec![
                ENTITY_CONFIGURATION.to_owned(),
                SUBORDINATE_STATEMENT.to_owned(),
            ]
        }),
        Oid4vciProofJwtLimits::default(),
    )
    .expect("evidence")
}

fn proof_with_evidence(
    key_reference: JwsKeyReference,
    evidence: Oid4vciProofJwtEvidence,
) -> String {
    Oid4vciProofJwtBuilder::default()
        .prepare_with_evidence(JwsAlgorithm::Ed25519, key_reference, claims(), evidence)
        .expect("proof input")
        .sign_with(&Ed25519Signer::new(&private_key()))
        .expect("signed proof")
        .compact()
        .to_owned()
}

fn raw_compact(header: &str) -> String {
    let encoded_header = URL_SAFE_NO_PAD.encode(header);
    let encoded_payload = URL_SAFE_NO_PAD.encode(format!(
        r#"{{"aud":"{AUDIENCE}","iat":{ISSUED_AT},"nonce":"{NONCE}"}}"#
    ));
    let encoded_signature = URL_SAFE_NO_PAD.encode([0x42; 64]);
    format!("{encoded_header}.{encoded_payload}.{encoded_signature}")
}

#[derive(Clone)]
enum ChainOutcome {
    Key(PublicKeyJwk),
    Failure(Oid4vciTrustChainFailure),
}

struct RecordingTrustChainProvider {
    outcome: ChainOutcome,
    calls: AtomicUsize,
    observations: Mutex<Vec<(JwsAlgorithm, String, usize)>>,
}

impl RecordingTrustChainProvider {
    fn key(key: PublicKeyJwk) -> Self {
        Self {
            outcome: ChainOutcome::Key(key),
            calls: AtomicUsize::new(0),
            observations: Mutex::new(Vec::new()),
        }
    }

    fn failing(failure: Oid4vciTrustChainFailure) -> Self {
        Self {
            outcome: ChainOutcome::Failure(failure),
            calls: AtomicUsize::new(0),
            observations: Mutex::new(Vec::new()),
        }
    }
}

impl Oid4vciTrustChainKeyProvider for RecordingTrustChainProvider {
    fn verification_key<'a>(
        &'a self,
        algorithm: JwsAlgorithm,
        key_id: &'a str,
        chain: &'a [String],
    ) -> Oid4vciTrustChainKeyFuture<'a> {
        Box::pin(async move {
            self.calls.fetch_add(1, Ordering::SeqCst);
            self.observations.lock().expect("observations").push((
                algorithm,
                key_id.to_owned(),
                chain.len(),
            ));
            match &self.outcome {
                ChainOutcome::Key(key) => Ok(key.clone()),
                ChainOutcome::Failure(failure) => Err(*failure),
            }
        })
    }
}

struct AttestationObservation {
    attestation_len: usize,
    proof_key: PublicKeyJwk,
    nonce: Option<String>,
    debug: String,
}

struct RecordingAttestationValidator {
    result: Result<(), Oid4vciKeyAttestationFailure>,
    calls: AtomicUsize,
    observations: Mutex<Vec<AttestationObservation>>,
}

impl RecordingAttestationValidator {
    fn accepting() -> Self {
        Self {
            result: Ok(()),
            calls: AtomicUsize::new(0),
            observations: Mutex::new(Vec::new()),
        }
    }

    fn failing(failure: Oid4vciKeyAttestationFailure) -> Self {
        Self {
            result: Err(failure),
            calls: AtomicUsize::new(0),
            observations: Mutex::new(Vec::new()),
        }
    }
}

impl Oid4vciKeyAttestationValidator for RecordingAttestationValidator {
    fn validate<'a>(
        &'a self,
        input: Oid4vciKeyAttestationInput<'a>,
    ) -> Oid4vciKeyAttestationFuture<'a> {
        Box::pin(async move {
            self.calls.fetch_add(1, Ordering::SeqCst);
            self.observations
                .lock()
                .expect("observations")
                .push(AttestationObservation {
                    attestation_len: input.attestation().len(),
                    proof_key: input.proof_key().clone(),
                    nonce: input.proof_nonce().map(str::to_owned),
                    debug: format!("{input:?}"),
                });
            self.result
        })
    }
}

struct FixedClock;

impl WallClock for FixedClock {
    fn now(&self) -> Result<UnixTimestampMillis, ClockError> {
        Ok(UnixTimestampMillis::new(ISSUED_AT as u64 * 1_000))
    }
}

struct AcceptReplay(AtomicUsize);

impl Oid4vciProofReplayGuard for AcceptReplay {
    fn accept<'a>(&'a self, _input: Oid4vciProofReplayInput<'a>) -> Oid4vciProofReplayFuture<'a> {
        Box::pin(async move {
            self.0.fetch_add(1, Ordering::SeqCst);
            Ok(())
        })
    }
}

#[test]
fn holder_round_trips_bounded_evidence_without_trust_claims() {
    let proof = proof_with_evidence(
        JwsKeyReference::KeyId(KEY_ID.to_owned()),
        evidence(true, true),
    );
    let suites = SignatureSuiteRegistry::recommended();
    let verifier =
        Oid4vciProofJwtVerifier::new(Oid4vciProofJwtLimits::default(), &suites, None, None);
    let parsed = verifier.parse(&proof).expect("parsed only");
    let header = parsed.as_unverified().protected_header();

    assert_eq!(header.key_attestation(), Some(ATTESTATION));
    assert_eq!(
        header.trust_chain().expect("trust chain"),
        [ENTITY_CONFIGURATION, SUBORDINATE_STATEMENT]
    );
    for formatted in [format!("{header:?}"), format!("{parsed:?}")] {
        assert!(!formatted.contains(ATTESTATION));
        assert!(!formatted.contains(ENTITY_CONFIGURATION));
        assert!(!formatted.contains(KEY_ID));
    }
}

#[test]
fn invalid_evidence_and_ambiguous_key_sources_fail_before_signing() {
    let limits = Oid4vciProofJwtLimits::default();
    for invalid in [
        Oid4vciProofJwtEvidence::new(Some("not-a-jwt".to_owned()), None, limits),
        Oid4vciProofJwtEvidence::new(Some("A.A.A".to_owned()), None, limits),
        Oid4vciProofJwtEvidence::new(None, Some(Vec::new()), limits),
        Oid4vciProofJwtEvidence::new(None, Some(vec![ENTITY_CONFIGURATION.to_owned(); 9]), limits),
    ] {
        assert_eq!(invalid, Err(JoseError::InvalidProofEvidence));
    }

    let result = Oid4vciProofJwtBuilder::default().prepare_with_evidence(
        JwsAlgorithm::Ed25519,
        JwsKeyReference::Jwk(public_key()),
        claims(),
        evidence(false, true),
    );
    assert!(matches!(result, Err(JoseError::InvalidProofEvidence)));
}

#[test]
fn parser_rejects_malformed_duplicate_and_unknown_evidence_members() {
    let suites = SignatureSuiteRegistry::recommended();
    let verifier =
        Oid4vciProofJwtVerifier::new(Oid4vciProofJwtLimits::default(), &suites, None, None);
    let public_jwk = serde_json::to_string(&public_key()).expect("public JWK");
    let cases = [
        (
            raw_compact(&format!(
                r#"{{"alg":"Ed25519","typ":"openid4vci-proof+jwt","kid":"{KEY_ID}","key_attestation":"bad"}}"#
            )),
            JoseError::InvalidHeaderValue,
        ),
        (
            raw_compact(&format!(
                r#"{{"alg":"Ed25519","typ":"openid4vci-proof+jwt","kid":"{KEY_ID}","key_attestation":"A.A.A"}}"#
            )),
            JoseError::InvalidHeaderValue,
        ),
        (
            raw_compact(&format!(
                r#"{{"alg":"Ed25519","typ":"openid4vci-proof+jwt","kid":"{KEY_ID}","trust_chain":[]}}"#
            )),
            JoseError::InvalidHeaderValue,
        ),
        (
            raw_compact(&format!(
                r#"{{"alg":"Ed25519","typ":"openid4vci-proof+jwt","kid":"{KEY_ID}","key_attestation":"{ATTESTATION}","key_attestation":"{ATTESTATION}"}}"#
            )),
            JoseError::DuplicateProtectedHeader,
        ),
        (
            raw_compact(&format!(
                r#"{{"alg":"Ed25519","typ":"openid4vci-proof+jwt","kid":"{KEY_ID}","trust_chain":["{ENTITY_CONFIGURATION}"],"unsupported":true}}"#
            )),
            JoseError::UnknownProtectedHeader,
        ),
        (
            raw_compact(&format!(
                r#"{{"alg":"Ed25519","typ":"openid4vci-proof+jwt","jwk":{public_jwk},"trust_chain":["{ENTITY_CONFIGURATION}"]}}"#
            )),
            JoseError::InvalidProofEvidence,
        ),
    ];
    for (compact, expected) in cases {
        assert_eq!(verifier.parse(&compact), Err(expected));
    }
}

#[test]
fn trust_chain_selects_one_exact_key_and_never_falls_back() {
    let compact = proof_with_evidence(
        JwsKeyReference::KeyId(KEY_ID.to_owned()),
        evidence(false, true),
    );
    let suites = SignatureSuiteRegistry::recommended();
    let provider = RecordingTrustChainProvider::key(public_key());
    let verifier =
        Oid4vciProofJwtVerifier::new(Oid4vciProofJwtLimits::default(), &suites, None, None)
            .with_trust_chain_provider(&provider);
    let verified = block_on(verifier.verify_signature(verifier.parse(&compact).unwrap()))
        .expect("verified through trust chain");
    let trusted = block_on(verifier.validate_trust(verified)).expect("trusted chain proof");

    assert_eq!(trusted.claims().nonce(), Some(NONCE));
    assert_eq!(provider.calls.load(Ordering::SeqCst), 1);
    assert_eq!(
        *provider.observations.lock().unwrap(),
        vec![(JwsAlgorithm::Ed25519, KEY_ID.to_owned(), 2)]
    );
}

#[test]
fn trust_chain_missing_rejected_unavailable_and_wrong_keys_fail_closed() {
    let compact = proof_with_evidence(
        JwsKeyReference::KeyId(KEY_ID.to_owned()),
        evidence(false, true),
    );
    let suites = SignatureSuiteRegistry::recommended();
    let bare = Oid4vciProofJwtVerifier::new(Oid4vciProofJwtLimits::default(), &suites, None, None);
    assert_eq!(
        block_on(bare.verify_signature(bare.parse(&compact).unwrap())),
        Err(JoseError::TrustChainProviderRequired)
    );

    for (failure, expected) in [
        (
            Oid4vciTrustChainFailure::Rejected,
            JoseError::TrustChainRejected,
        ),
        (
            Oid4vciTrustChainFailure::Unavailable,
            JoseError::TrustChainProviderUnavailable,
        ),
    ] {
        let provider = RecordingTrustChainProvider::failing(failure);
        let verifier =
            Oid4vciProofJwtVerifier::new(Oid4vciProofJwtLimits::default(), &suites, None, None)
                .with_trust_chain_provider(&provider);
        assert_eq!(
            block_on(verifier.verify_signature(verifier.parse(&compact).unwrap())),
            Err(expected)
        );
        assert_eq!(provider.calls.load(Ordering::SeqCst), 1);
    }

    let wrong_key = Ed25519PrivateKey::from_slice(&[0x52; 32])
        .unwrap()
        .to_public_key()
        .encode_jwk();
    let provider = RecordingTrustChainProvider::key(wrong_key);
    let verifier =
        Oid4vciProofJwtVerifier::new(Oid4vciProofJwtLimits::default(), &suites, None, None)
            .with_trust_chain_provider(&provider);
    assert_eq!(
        block_on(verifier.verify_signature(verifier.parse(&compact).unwrap())),
        Err(JoseError::SignatureInvalid)
    );

    let wrong_algorithm = P256PrivateKey::from_slice(&[0x01; 32])
        .unwrap()
        .to_public_key()
        .encode_jwk();
    let provider = RecordingTrustChainProvider::key(wrong_algorithm);
    let verifier =
        Oid4vciProofJwtVerifier::new(Oid4vciProofJwtLimits::default(), &suites, None, None)
            .with_trust_chain_provider(&provider);
    assert_eq!(
        block_on(verifier.verify_signature(verifier.parse(&compact).unwrap())),
        Err(JoseError::InvalidVerificationKey)
    );
}

#[test]
fn key_attestation_has_a_distinct_trust_transition_bound_to_key_and_nonce() {
    let compact = proof_with_evidence(JwsKeyReference::Jwk(public_key()), evidence(true, false));
    let suites = SignatureSuiteRegistry::recommended();
    let validator = RecordingAttestationValidator::accepting();
    let verifier =
        Oid4vciProofJwtVerifier::new(Oid4vciProofJwtLimits::default(), &suites, None, None)
            .with_key_attestation_validator(&validator);
    let verified = block_on(verifier.verify_signature(verifier.parse(&compact).unwrap()))
        .expect("cryptographic state");
    assert_eq!(validator.calls.load(Ordering::SeqCst), 0);
    let trusted = block_on(verifier.validate_trust(verified)).expect("trusted state");

    assert_eq!(trusted.claims().audience(), AUDIENCE);
    assert_eq!(validator.calls.load(Ordering::SeqCst), 1);
    let observations = validator.observations.lock().unwrap();
    assert_eq!(observations[0].attestation_len, ATTESTATION.len());
    assert_eq!(observations[0].proof_key, public_key());
    assert_eq!(observations[0].nonce.as_deref(), Some(NONCE));
    assert!(!observations[0].debug.contains(ATTESTATION));
    assert!(!observations[0].debug.contains(NONCE));
}

#[test]
fn attestation_requires_one_accepting_provider_after_a_valid_signature() {
    let compact = proof_with_evidence(JwsKeyReference::Jwk(public_key()), evidence(true, false));
    let suites = SignatureSuiteRegistry::recommended();
    let bare = Oid4vciProofJwtVerifier::new(Oid4vciProofJwtLimits::default(), &suites, None, None);
    let verified = block_on(bare.verify_signature(bare.parse(&compact).unwrap())).unwrap();
    assert_eq!(
        block_on(bare.validate_trust(verified.clone())),
        Err(JoseError::KeyAttestationProviderRequired)
    );

    for (failure, expected) in [
        (
            Oid4vciKeyAttestationFailure::Rejected,
            JoseError::KeyAttestationRejected,
        ),
        (
            Oid4vciKeyAttestationFailure::Unavailable,
            JoseError::KeyAttestationProviderUnavailable,
        ),
    ] {
        let validator = RecordingAttestationValidator::failing(failure);
        let verifier =
            Oid4vciProofJwtVerifier::new(Oid4vciProofJwtLimits::default(), &suites, None, None)
                .with_key_attestation_validator(&validator);
        assert_eq!(
            block_on(verifier.validate_trust(verified.clone())),
            Err(expected)
        );
        assert_eq!(validator.calls.load(Ordering::SeqCst), 1);
    }

    let validator = RecordingAttestationValidator::accepting();
    let verifier =
        Oid4vciProofJwtVerifier::new(Oid4vciProofJwtLimits::default(), &suites, None, None)
            .with_key_attestation_validator(&validator);
    let mut tampered = compact.into_bytes();
    let last = tampered.last_mut().expect("signature byte");
    *last = if *last == b'A' { b'B' } else { b'A' };
    let tampered = String::from_utf8(tampered).unwrap();
    assert_eq!(
        block_on(verifier.verify_signature(verifier.parse(&tampered).unwrap())),
        Err(JoseError::SignatureInvalid)
    );
    assert_eq!(validator.calls.load(Ordering::SeqCst), 0);
}

#[test]
fn composed_authorization_requires_both_trust_capabilities_before_replay() {
    let compact = proof_with_evidence(
        JwsKeyReference::KeyId(KEY_ID.to_owned()),
        evidence(true, true),
    );
    let suites = SignatureSuiteRegistry::recommended();
    let chain = RecordingTrustChainProvider::key(public_key());
    let attestation = RecordingAttestationValidator::accepting();
    let verifier =
        Oid4vciProofJwtVerifier::new(Oid4vciProofJwtLimits::default(), &suites, None, None)
            .with_trust_chain_provider(&chain)
            .with_key_attestation_validator(&attestation);
    let policy = Oid4vciProofJwtPolicy::new(
        Oid4vciProofJwtClient::AnonymousPreAuthorized,
        AUDIENCE,
        Oid4vciProofJwtNonce::required(NONCE, Oid4vciProofJwtLimits::default()).unwrap(),
        60,
        0,
        Oid4vciProofJwtLimits::default(),
    )
    .unwrap();
    let replay = AcceptReplay(AtomicUsize::new(0));

    let authorized =
        block_on(verifier.verify_and_authorize(&compact, &policy, &FixedClock, &replay))
            .expect("authorized");
    assert_eq!(authorized.trusted().claims().nonce(), Some(NONCE));
    assert_eq!(chain.calls.load(Ordering::SeqCst), 1);
    assert_eq!(attestation.calls.load(Ordering::SeqCst), 1);
    assert_eq!(replay.0.load(Ordering::SeqCst), 1);
}
