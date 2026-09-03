use std::{
    collections::BTreeMap,
    future::Future,
    hint::black_box,
    pin::pin,
    sync::Arc,
    task::{Context, Poll, Wake, Waker},
    time::Instant,
};

use identus_did::{
    DereferencedContent, DereferencingOptions, Did, DidDocument, DidDocumentMetadata,
    DidResolutionError, DidResolutionErrorKind, DidResolutionFuture, DidResolutionMetadata,
    DidResolutionResult, DidResolver, DidUrl, DidUrlContentMetadata, DidUrlDereferencer,
    DidUrlDereferencingFuture, DidUrlDereferencingMetadata, DidUrlDereferencingResult, Error,
    MAX_DID_RESOLUTION_OPTIONS_BYTES, MAX_VERIFICATION_RELATIONSHIP_BYTES, MediaType,
    ResolutionError, ResolutionOptions, Uri, VerificationRelationshipName, VersionId,
};
use serde_json::{Value, json};

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

fn success(did: &Did) -> DidResolutionResult {
    let document = DidDocument::builder(did.clone()).build().unwrap();
    DidResolutionResult::success(
        DidResolutionMetadata::empty(),
        document,
        DidDocumentMetadata::empty(),
    )
    .unwrap()
}

fn failure(kind: DidResolutionErrorKind) -> DidResolutionResult {
    let metadata = DidResolutionMetadata::new(
        None,
        Some(DidResolutionError::standard(kind)),
        BTreeMap::new(),
    )
    .unwrap();
    DidResolutionResult::failure(metadata).unwrap()
}

#[test]
fn resolution_options_roundtrip_common_and_extension_values() {
    let options = ResolutionOptions::builder()
        .accept(MediaType::parse("application/did;profile=\"https://example/profile\"").unwrap())
        .expand_relative_urls(false)
        .no_cache(false)
        .version_id(VersionId::parse("ledger-42").unwrap())
        .version_time(identus_did::DidResolutionDateTime::parse("2026-09-03T03:45:00Z").unwrap())
        .extensions(BTreeMap::from([(
            "network".to_owned(),
            json!({"name": "preview", "height": 42}),
        )]))
        .build()
        .unwrap();

    assert_eq!(
        options.accept().unwrap().as_str(),
        "application/did;profile=\"https://example/profile\""
    );
    assert_eq!(options.expand_relative_urls(), Some(false));
    assert_eq!(options.no_cache(), Some(false));
    assert_eq!(ResolutionOptions::empty().no_cache(), None);
    assert_eq!(options.version_id().unwrap().as_str(), "ledger-42");
    assert_eq!(
        options.version_time().unwrap().as_str(),
        "2026-09-03T03:45:00Z"
    );
    assert_eq!(options.extensions()["network"]["height"], 42);

    let wire = serde_json::to_value(&options).unwrap();
    assert_eq!(wire["expandRelativeUrls"], false);
    assert_eq!(wire["noCache"], false);
    assert_eq!(wire["versionId"], "ledger-42");
    assert_eq!(wire["network"]["name"], "preview");
    assert_eq!(
        serde_json::from_value::<ResolutionOptions>(wire).unwrap(),
        options
    );

    let bypass = ResolutionOptions::from_json_str(r#"{"noCache":true}"#).unwrap();
    assert_eq!(bypass.no_cache(), Some(true));
    assert_eq!(serde_json::to_value(&bypass).unwrap()["noCache"], true);

    assert_eq!(
        serde_json::to_value(ResolutionOptions::empty()).unwrap(),
        json!({})
    );
}

#[test]
fn dereferencing_options_keep_relationship_vocabulary_open() {
    for value in [
        "authentication",
        "assertionMethod",
        "keyAgreement",
        "capabilityInvocation",
        "capabilityDelegation",
        "exampleRelationship",
    ] {
        let relationship = VerificationRelationshipName::parse(value).unwrap();
        assert_eq!(relationship.as_str(), value);
    }

    let options = DereferencingOptions::builder()
        .accept(MediaType::parse("application/did").unwrap())
        .verification_relationship(VerificationRelationshipName::parse("assertionMethod").unwrap())
        .extensions(BTreeMap::from([(
            "proofPurpose".to_owned(),
            json!("issuer"),
        )]))
        .build()
        .unwrap();
    let wire = serde_json::to_value(&options).unwrap();
    assert_eq!(wire["verificationRelationship"], "assertionMethod");
    assert_eq!(wire["proofPurpose"], "issuer");
    assert_eq!(
        DereferencingOptions::from_json_str(&wire.to_string()).unwrap(),
        options
    );
}

#[test]
fn option_scalars_reject_malformed_or_unsafe_values() {
    for value in ["", " authentication", "authentication ", "auth\nentication"] {
        assert!(VerificationRelationshipName::parse(value).is_err());
    }
    assert!(
        VerificationRelationshipName::parse(&"a".repeat(MAX_VERIFICATION_RELATIONSHIP_BYTES + 1))
            .is_err()
    );

    for wire in [
        r#"{"accept":"not-a-media-type"}"#,
        r#"{"versionId":""}"#,
        r#"{"versionTime":"2026-02-30T00:00:00Z"}"#,
        r#"{"expandRelativeUrls":"true"}"#,
        r#"{"noCache":"true"}"#,
    ] {
        assert!(ResolutionOptions::from_json_str(wire).is_err(), "{wire}");
    }
    assert!(
        DereferencingOptions::from_json_str(
            r#"{"verificationRelationship":"assertionMethod\r\nInjected"}"#
        )
        .is_err()
    );
}

#[test]
fn option_extensions_reject_collisions_and_resource_exhaustion() {
    let reserved = ResolutionOptions::builder()
        .extensions(BTreeMap::from([(
            "accept".to_owned(),
            json!("application/did"),
        )]))
        .build()
        .unwrap_err();
    assert!(matches!(
        reserved,
        Error::InvalidResolution(ResolutionError::InvalidString)
    ));
    assert!(
        ResolutionOptions::builder()
            .extensions(BTreeMap::from([("noCache".to_owned(), json!(true))]))
            .build()
            .is_err()
    );

    let too_many = (0..65)
        .map(|index| (format!("option{index}"), Value::Null))
        .collect();
    assert!(
        DereferencingOptions::new(None, None, too_many).is_err(),
        "extension member ceiling must apply to native construction"
    );

    let mut deep = Value::Null;
    for _ in 0..33 {
        deep = Value::Array(vec![deep]);
    }
    assert!(
        ResolutionOptions::builder()
            .extensions(BTreeMap::from([("deep".to_owned(), deep)]))
            .build()
            .is_err()
    );

    let wide = (0..64)
        .map(|index| {
            (
                format!("option{index}"),
                Value::Array(vec![Value::Null; 64]),
            )
        })
        .collect();
    assert!(ResolutionOptions::new(None, None, None, None, None, wide).is_err());

    let oversized = vec![b' '; MAX_DID_RESOLUTION_OPTIONS_BYTES + 1];
    assert!(matches!(
        ResolutionOptions::from_json_slice(&oversized),
        Err(Error::InvalidResolution(ResolutionError::TooLarge))
    ));
}

#[test]
fn option_errors_bridge_to_a_stable_redacted_boundary() {
    let attacker = "secret\r\nheader";
    let error = VerificationRelationshipName::parse(attacker).unwrap_err();
    let public = error.to_identus_error();
    assert_eq!(public.code().as_str(), "did.invalid_resolution");
    assert!(!public.to_string().contains(attacker));
}

struct PrismResolver;

impl DidResolver for PrismResolver {
    fn resolve<'a>(
        &'a self,
        did: &'a Did,
        options: &'a ResolutionOptions,
    ) -> DidResolutionFuture<'a> {
        Box::pin(async move {
            assert_eq!(did.method(), "prism");
            assert_eq!(options.version_id().unwrap().as_str(), "operation-7");
            assert_eq!(options.expand_relative_urls(), Some(true));
            success(did)
        })
    }
}

struct MidnightResolver;

impl DidResolver for MidnightResolver {
    fn resolve<'a>(
        &'a self,
        did: &'a Did,
        options: &'a ResolutionOptions,
    ) -> DidResolutionFuture<'a> {
        Box::pin(async move {
            assert_eq!(did.method(), "midnight");
            assert_eq!(options.extensions()["network"], "undeployed");
            failure(DidResolutionErrorKind::NotFound)
        })
    }
}

#[test]
fn resolver_port_is_object_safe_for_independent_method_implementations() {
    let prism: Arc<dyn DidResolver> = Arc::new(PrismResolver);
    let prism_did = Did::parse("did:prism:abc123").unwrap();
    let prism_options = ResolutionOptions::builder()
        .version_id(VersionId::parse("operation-7").unwrap())
        .expand_relative_urls(true)
        .build()
        .unwrap();
    let resolved = block_on(prism.resolve(&prism_did, &prism_options));
    resolved.validate_for(&prism_did).unwrap();
    assert_eq!(resolved.document().unwrap().id(), &prism_did);

    let midnight: Arc<dyn DidResolver> = Arc::new(MidnightResolver);
    let midnight_did = Did::parse("did:midnight:undeployed:contract-7").unwrap();
    let midnight_options = ResolutionOptions::builder()
        .extensions(BTreeMap::from([(
            "network".to_owned(),
            json!("undeployed"),
        )]))
        .build()
        .unwrap();
    let missing = block_on(midnight.resolve(&midnight_did, &midnight_options));
    assert_eq!(
        missing.metadata().error().unwrap().kind(),
        Some(DidResolutionErrorKind::NotFound)
    );
}

struct PrismDereferencer;

impl DidUrlDereferencer for PrismDereferencer {
    fn dereference<'a>(
        &'a self,
        did_url: &'a DidUrl,
        options: &'a DereferencingOptions,
    ) -> DidUrlDereferencingFuture<'a> {
        Box::pin(async move {
            assert_eq!(did_url.method(), "prism");
            assert_eq!(did_url.fragment(), Some("key-1"));
            assert_eq!(
                options.verification_relationship().unwrap().as_str(),
                "assertionMethod"
            );
            DidUrlDereferencingResult::success(
                DidUrlDereferencingMetadata::empty(),
                DereferencedContent::from_uri(&Uri::parse("did:prism:abc123#key-1").unwrap())
                    .unwrap(),
                DidUrlContentMetadata::empty(),
            )
            .unwrap()
        })
    }
}

#[test]
fn dereferencer_port_is_independent_and_object_safe() {
    let dereferencer: Arc<dyn DidUrlDereferencer> = Arc::new(PrismDereferencer);
    let did_url = DidUrl::parse("did:prism:abc123#key-1").unwrap();
    let options = DereferencingOptions::builder()
        .verification_relationship(VerificationRelationshipName::parse("assertionMethod").unwrap())
        .build()
        .unwrap();
    let result = block_on(dereferencer.dereference(&did_url, &options));
    assert_eq!(
        result.content().unwrap().to_uri().unwrap().as_str(),
        "did:prism:abc123#key-1"
    );
}

#[test]
#[ignore = "manual release-mode performance diagnostic"]
fn option_and_dynamic_dispatch_throughput_diagnostic() {
    const ITERATIONS: usize = 50_000;
    let wire = r#"{"accept":"application/did","expandRelativeUrls":false,"versionId":"42","network":{"name":"preview"}}"#;
    let resolver: Arc<dyn DidResolver> = Arc::new(PrismResolver);
    let did = Did::parse("did:prism:abc123").unwrap();
    let dispatch_options = ResolutionOptions::builder()
        .version_id(VersionId::parse("operation-7").unwrap())
        .expand_relative_urls(true)
        .build()
        .unwrap();

    let started = Instant::now();
    for _ in 0..ITERATIONS {
        black_box(ResolutionOptions::from_json_str(black_box(wire)).unwrap());
        black_box(block_on(
            resolver.resolve(black_box(&did), black_box(&dispatch_options)),
        ));
    }
    let elapsed = started.elapsed();
    println!(
        "parsed and dispatched {ITERATIONS} DID queries in {elapsed:?} ({:.0} queries/s)",
        ITERATIONS as f64 / elapsed.as_secs_f64()
    );
}
