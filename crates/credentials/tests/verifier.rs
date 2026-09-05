use std::{
    hint::black_box,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    task::{Context, Poll, Wake, Waker},
    time::Instant,
};

use identus_core::ErrorKind;
use identus_credentials::{
    CredentialDetachedProof, CredentialEnvelope, CredentialError, CredentialFormat,
    CredentialPayload, CredentialPrivateMaterial, CredentialVerificationError,
    CredentialVerificationFuture, CredentialVerificationRequest, CredentialVerifier,
    CredentialVerifierRegistry, MAX_CREDENTIAL_VERIFIER_REGISTRY_ENTRIES, VERIFICATION_STAGE_COUNT,
    VerificationOutcome, VerificationReasonCode, VerificationReport, VerificationStage,
    VerificationStageName, VerificationStageStatus,
};

struct NoopWake;

impl Wake for NoopWake {
    fn wake(self: Arc<Self>) {}
}

fn block_on(
    mut future: CredentialVerificationFuture<'_>,
) -> identus_credentials::CredentialVerificationResult {
    let waker = Waker::from(Arc::new(NoopWake));
    let mut context = Context::from_waker(&waker);
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(result) => return result,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}

fn reason(value: &str) -> VerificationReasonCode {
    VerificationReasonCode::parse(value).expect("valid reason")
}

fn report(
    changed: Option<(VerificationStageName, VerificationStageStatus, &str)>,
) -> VerificationReport {
    let stages: [VerificationStage; VERIFICATION_STAGE_COUNT] =
        VerificationStageName::ALL.map(|name| match changed {
            Some((changed_name, status, code)) if name == changed_name => {
                VerificationStage::new(name, status, Some(reason(code))).expect("changed stage")
            }
            _ => VerificationStage::new(name, VerificationStageStatus::Passed, None)
                .expect("passed stage"),
        });
    VerificationReport::new(stages).expect("canonical report")
}

fn envelope(format: &str, payload: &[u8]) -> CredentialEnvelope {
    CredentialEnvelope::new(
        CredentialFormat::parse(format).expect("format"),
        CredentialPayload::new(payload.to_vec()).expect("payload"),
    )
}

struct FixedVerifier {
    report: VerificationReport,
    calls: AtomicUsize,
}

impl FixedVerifier {
    fn new(report: VerificationReport) -> Self {
        Self {
            report,
            calls: AtomicUsize::new(0),
        }
    }

    fn calls(&self) -> usize {
        self.calls.load(Ordering::Relaxed)
    }
}

impl CredentialVerifier for FixedVerifier {
    fn verify<'a>(
        &'a self,
        _: CredentialVerificationRequest<'a>,
    ) -> CredentialVerificationFuture<'a> {
        self.calls.fetch_add(1, Ordering::Relaxed);
        Box::pin(async move { Ok(self.report.clone()) })
    }
}

struct FailingVerifier(CredentialVerificationError);

impl CredentialVerifier for FailingVerifier {
    fn verify<'a>(
        &'a self,
        _: CredentialVerificationRequest<'a>,
    ) -> CredentialVerificationFuture<'a> {
        Box::pin(async move { Err(self.0) })
    }
}

struct ReadyVerifier(VerificationReport);

impl CredentialVerifier for ReadyVerifier {
    fn verify<'a>(
        &'a self,
        _: CredentialVerificationRequest<'a>,
    ) -> CredentialVerificationFuture<'a> {
        Box::pin(async move { Ok(self.0.clone()) })
    }
}

#[test]
fn request_borrows_only_verification_artifacts_and_redacts_contents() {
    let envelope = envelope("midnight_cbor_phase1", b"credential-pii")
        .with_detached_proof(CredentialDetachedProof::new(b"proof-secret".to_vec()).expect("proof"))
        .with_private_material(
            CredentialPrivateMaterial::new(b"holder-opening".to_vec()).expect("private material"),
        );
    let request = CredentialVerificationRequest::from(&envelope);

    assert!(std::ptr::eq(request.format(), envelope.format()));
    assert!(std::ptr::eq(request.payload(), envelope.payload()));
    assert!(std::ptr::eq(
        request.detached_proof().expect("request proof"),
        envelope.detached_proof().expect("envelope proof")
    ));

    let debug = format!("{request:?}");
    assert!(debug.contains("midnight_cbor_phase1"));
    assert!(debug.contains("payload_length: 14"));
    assert!(debug.contains("detached_proof_length: Some(12)"));
    for secret in ["credential-pii", "proof-secret", "holder-opening"] {
        assert!(!debug.contains(secret));
    }
}

#[test]
fn registry_is_object_safe_and_dispatches_unrelated_formats_exactly() {
    let midnight = Arc::new(FixedVerifier::new(report(None)));
    let unrelated = Arc::new(FixedVerifier::new(report(Some((
        VerificationStageName::Status,
        VerificationStageStatus::NotChecked,
        "status:not_supported",
    )))));
    let registry = CredentialVerifierRegistry::builder()
        .register(
            CredentialFormat::parse("midnight_cbor_phase1").unwrap(),
            midnight.clone(),
        )
        .unwrap()
        .register(
            CredentialFormat::parse("example+opaque:v1").unwrap(),
            unrelated.clone(),
        )
        .unwrap()
        .build();
    let cloned = registry.clone();
    let verifier: Arc<dyn CredentialVerifier> = Arc::new(cloned);

    let shared_payload = [0xa1, 0x01];
    let midnight_envelope = envelope("midnight_cbor_phase1", &shared_payload);
    let unrelated_envelope = envelope("example+opaque:v1", &shared_payload);
    let midnight_report = block_on(verifier.verify((&midnight_envelope).into())).unwrap();
    let unrelated_report = block_on(verifier.verify((&unrelated_envelope).into())).unwrap();

    assert_eq!(midnight_report.outcome(), VerificationOutcome::Valid);
    assert_eq!(
        unrelated_report.outcome(),
        VerificationOutcome::Indeterminate
    );
    assert_eq!(midnight.calls(), 1);
    assert_eq!(unrelated.calls(), 1);
    assert_eq!(registry.format_count(), 2);
    assert!(!registry.is_empty());
    assert_eq!(
        registry.formats().collect::<Vec<_>>(),
        ["example+opaque:v1", "midnight_cbor_phase1"]
    );
}

#[test]
fn invalid_evidence_is_a_report_not_an_operational_error_or_trust_decision() {
    let invalid = report(Some((
        VerificationStageName::Proof,
        VerificationStageStatus::Failed,
        "proof.invalid_signature",
    )));
    let registry = CredentialVerifierRegistry::builder()
        .register(
            CredentialFormat::parse("vc+sd-jwt").unwrap(),
            Arc::new(FixedVerifier::new(invalid)),
        )
        .unwrap()
        .build();
    let credential = envelope("vc+sd-jwt", b"same-evidence-for-any-trust-policy");

    let result = block_on(registry.verify((&credential).into()));
    let report = result.expect("completed invalid evidence remains a report");
    assert_eq!(report.outcome(), VerificationOutcome::Invalid);
    assert_eq!(
        report.stage(VerificationStageName::Proof).status(),
        VerificationStageStatus::Failed
    );
}

#[test]
fn unknown_format_fails_closed_without_invoking_another_verifier() {
    let bound = Arc::new(FixedVerifier::new(report(None)));
    let registry = CredentialVerifierRegistry::builder()
        .register(
            CredentialFormat::parse("bound-format").unwrap(),
            bound.clone(),
        )
        .unwrap()
        .build();
    let unknown = envelope("unknown-format", b"same-prefix");

    assert_eq!(
        block_on(registry.verify((&unknown).into())),
        Err(CredentialVerificationError::UnsupportedFormat)
    );
    assert_eq!(bound.calls(), 0);
    assert!(!registry.supports_format(unknown.format()));
    assert!(registry.supports_format(&CredentialFormat::parse("bound-format").unwrap()));
    assert_eq!(
        block_on(CredentialVerifierRegistry::empty().verify((&unknown).into())),
        Err(CredentialVerificationError::UnsupportedFormat)
    );
}

#[test]
fn operational_failures_are_static_and_never_claim_verification_failure() {
    let credential = envelope("example", b"caller-controlled-value");
    let cases = [
        (
            CredentialVerificationError::UnsupportedFormat,
            "credential.verification_unsupported_format",
            ErrorKind::Unsupported,
        ),
        (
            CredentialVerificationError::Unavailable,
            "credential.verification_unavailable",
            ErrorKind::Internal,
        ),
        (
            CredentialVerificationError::Internal,
            "credential.verification_internal",
            ErrorKind::Internal,
        ),
    ];

    for (error, code, kind) in cases {
        let verifier: Arc<dyn CredentialVerifier> = Arc::new(FailingVerifier(error));
        assert_eq!(block_on(verifier.verify((&credential).into())), Err(error));
        let public = error.to_identus_error();
        assert_eq!(public.capability().unwrap().as_str(), "credential");
        assert_eq!(public.code().as_str(), code);
        assert_eq!(public.kind(), kind);
        assert_ne!(public.kind(), ErrorKind::VerificationFailed);
        assert!(!format!("{error:?} {error} {public}").contains("caller-controlled-value"));
    }
}

#[test]
fn builder_rejects_duplicate_and_excess_formats_without_replacement() {
    let duplicate = CredentialVerifierRegistry::builder()
        .register(
            CredentialFormat::parse("duplicate").unwrap(),
            Arc::new(FixedVerifier::new(report(None))),
        )
        .unwrap()
        .register(
            CredentialFormat::parse("duplicate").unwrap(),
            Arc::new(FixedVerifier::new(report(None))),
        );
    assert!(matches!(
        duplicate,
        Err(CredentialError::DuplicateCredentialVerifierFormat)
    ));

    let mut builder = CredentialVerifierRegistry::builder();
    for index in 0..MAX_CREDENTIAL_VERIFIER_REGISTRY_ENTRIES {
        builder = builder
            .register(
                CredentialFormat::parse(&format!("format-{index:02}")).unwrap(),
                Arc::new(FixedVerifier::new(report(None))),
            )
            .unwrap();
    }
    assert!(matches!(
        builder.register(
            CredentialFormat::parse("overflow").unwrap(),
            Arc::new(FixedVerifier::new(report(None))),
        ),
        Err(CredentialError::TooManyCredentialVerifierFormats)
    ));
}

#[test]
fn registry_construction_errors_use_static_credential_codes() {
    let cases = [
        (
            CredentialError::DuplicateCredentialVerifierFormat,
            "credential.duplicate_verifier_format",
        ),
        (
            CredentialError::TooManyCredentialVerifierFormats,
            "credential.too_many_verifier_formats",
        ),
    ];
    for (error, code) in cases {
        let public = error.to_identus_error();
        assert_eq!(public.capability().unwrap().as_str(), "credential");
        assert_eq!(public.kind(), ErrorKind::InvalidInput);
        assert_eq!(public.code().as_str(), code);
        assert!(!format!("{error:?} {error} {public}").contains("caller-controlled-value"));
    }
}

#[test]
#[ignore = "manual release-mode throughput observation"]
fn credential_verifier_registry_dispatch_throughput_diagnostic() {
    const ITERATIONS: u32 = 1_000_000;
    let registry = CredentialVerifierRegistry::builder()
        .register(
            CredentialFormat::parse("example+ready:v1").unwrap(),
            Arc::new(ReadyVerifier(report(None))),
        )
        .unwrap()
        .build();
    let credential = envelope("example+ready:v1", &[0xa1]);
    let request = CredentialVerificationRequest::from(&credential);
    let waker = Waker::from(Arc::new(NoopWake));
    let mut context = Context::from_waker(&waker);

    let started = Instant::now();
    for _ in 0..ITERATIONS {
        let mut future = registry.verify(request);
        let Poll::Ready(result) = future.as_mut().poll(&mut context) else {
            panic!("diagnostic verifier must be ready")
        };
        black_box(result.expect("ready report"));
    }
    let elapsed = started.elapsed();
    let operations_per_second = f64::from(ITERATIONS) / elapsed.as_secs_f64();
    eprintln!(
        "credential verifier registry: {ITERATIONS} ready dispatches in {elapsed:?} ({operations_per_second:.0} dispatches/s)"
    );
}
