use std::{
    collections::BTreeMap,
    future::Future,
    hint::black_box,
    pin::pin,
    sync::{Arc, Barrier},
    task::{Context, Poll, Wake, Waker},
    time::Instant,
};

use identus_core::ErrorKind;
use identus_did::{
    DereferencedContent, DereferencingOptions, Did, DidDocument, DidDocumentMetadata, DidMethod,
    DidMethodBinding, DidMethodRegistry, DidResolutionErrorKind, DidResolutionFuture,
    DidResolutionMetadata, DidResolutionResult, DidResolver, DidUrl, DidUrlContentMetadata,
    DidUrlDereferencer, DidUrlDereferencingFuture, DidUrlDereferencingMetadata,
    DidUrlDereferencingResult, Error, MAX_DID_METHOD_REGISTRY_ENTRIES, MediaType, RegistryError,
    ResolutionOptions, Uri, VerificationRelationshipName,
};

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

fn success(did: &Did, source: &str) -> DidResolutionResult {
    let metadata = DidResolutionMetadata::new(
        Some(MediaType::parse("application/did").unwrap()),
        None,
        BTreeMap::from([("source".to_owned(), serde_json::json!(source))]),
    )
    .unwrap();
    DidResolutionResult::success(
        metadata,
        DidDocument::builder(did.clone()).build().unwrap(),
        DidDocumentMetadata::empty(),
    )
    .unwrap()
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
            assert_eq!(options.extensions()["network"], "preprod");
            success(did, "prism")
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
            assert_eq!(options.expand_relative_urls(), Some(false));
            success(did, "midnight")
        })
    }
}

struct PrismDereferencer;

impl DidUrlDereferencer for PrismDereferencer {
    fn dereference<'a>(
        &'a self,
        did_url: &'a DidUrl,
        options: &'a DereferencingOptions,
    ) -> DidUrlDereferencingFuture<'a> {
        Box::pin(async move {
            assert_eq!(did_url.as_str(), "did:prism:123#key-1");
            assert_eq!(
                options.verification_relationship().unwrap().as_str(),
                "assertionMethod"
            );
            DidUrlDereferencingResult::success(
                DidUrlDereferencingMetadata::empty(),
                DereferencedContent::from_uri(&Uri::parse("did:prism:123#key-1").unwrap()).unwrap(),
                DidUrlContentMetadata::empty(),
            )
            .unwrap()
        })
    }
}

fn binding(name: &str, resolver: Arc<dyn DidResolver>) -> DidMethodBinding {
    DidMethodBinding::new(DidMethod::parse(name).unwrap(), resolver)
}

fn registry() -> DidMethodRegistry {
    DidMethodRegistry::builder()
        .register(
            binding("prism", Arc::new(PrismResolver))
                .with_dereferencer(Arc::new(PrismDereferencer)),
        )
        .unwrap()
        .register(binding("midnight", Arc::new(MidnightResolver)))
        .unwrap()
        .build()
}

fn error_kind(result: &DidResolutionResult) -> DidResolutionErrorKind {
    result.metadata().error().unwrap().kind().unwrap()
}

fn dereferencing_error_kind(result: &DidUrlDereferencingResult) -> DidResolutionErrorKind {
    result.metadata().error().unwrap().kind().unwrap()
}

#[test]
fn registry_routes_independent_methods_through_one_object_safe_seam() {
    let registry: Arc<dyn DidResolver> = Arc::new(registry());

    let prism = Did::parse("did:prism:123").unwrap();
    let prism_options = ResolutionOptions::builder()
        .extensions(BTreeMap::from([(
            "network".to_owned(),
            serde_json::json!("preprod"),
        )]))
        .build()
        .unwrap();
    let prism_result = block_on(registry.resolve(&prism, &prism_options));
    assert_eq!(prism_result.metadata().extensions()["source"], "prism");
    prism_result.validate_for(&prism).unwrap();

    let midnight = Did::parse("did:midnight:undeployed:contract").unwrap();
    let midnight_options = ResolutionOptions::builder()
        .expand_relative_urls(false)
        .build()
        .unwrap();
    let midnight_result = block_on(registry.resolve(&midnight, &midnight_options));
    assert_eq!(
        midnight_result.metadata().extensions()["source"],
        "midnight"
    );
    midnight_result.validate_for(&midnight).unwrap();
}

#[test]
fn registry_uses_exact_method_names_and_standard_unknown_failure() {
    let registry = registry();
    let unknown = Did::parse("did:prismx:123").unwrap();
    let result = block_on(registry.resolve(&unknown, &ResolutionOptions::empty()));
    assert_eq!(
        error_kind(&result),
        DidResolutionErrorKind::MethodNotSupported
    );
    assert!(result.document().is_none());

    let empty = DidMethodRegistry::empty();
    assert!(empty.is_empty());
    assert_eq!(empty.method_count(), 0);
    let result = block_on(empty.resolve(
        &Did::parse("did:example:123").unwrap(),
        &ResolutionOptions::empty(),
    ));
    assert_eq!(
        error_kind(&result),
        DidResolutionErrorKind::MethodNotSupported
    );
}

#[test]
fn dereferencing_is_independent_and_reports_precise_support() {
    let registry = registry();
    let prism = DidMethod::parse("prism").unwrap();
    let midnight = DidMethod::parse("midnight").unwrap();
    assert!(registry.supports_dereferencing(&prism));
    assert!(!registry.supports_dereferencing(&midnight));

    let options = DereferencingOptions::builder()
        .verification_relationship(VerificationRelationshipName::parse("assertionMethod").unwrap())
        .build()
        .unwrap();
    let result =
        block_on(registry.dereference(&DidUrl::parse("did:prism:123#key-1").unwrap(), &options));
    assert_eq!(
        result.content().unwrap().to_uri().unwrap().as_str(),
        "did:prism:123#key-1"
    );

    let unsupported = block_on(registry.dereference(
        &DidUrl::parse("did:midnight:undeployed:contract#key-1").unwrap(),
        &DereferencingOptions::empty(),
    ));
    assert_eq!(
        dereferencing_error_kind(&unsupported),
        DidResolutionErrorKind::FeatureNotSupported
    );

    let unknown = block_on(registry.dereference(
        &DidUrl::parse("did:example:123#key-1").unwrap(),
        &DereferencingOptions::empty(),
    ));
    assert_eq!(
        dereferencing_error_kind(&unknown),
        DidResolutionErrorKind::MethodNotSupported
    );
}

#[test]
fn duplicate_and_capacity_errors_are_stable_and_redacted() {
    let duplicate = DidMethodRegistry::builder()
        .register(binding("secretmethod", Arc::new(PrismResolver)))
        .unwrap()
        .register(binding("secretmethod", Arc::new(PrismResolver)))
        .unwrap_err();
    assert!(matches!(
        duplicate,
        Error::InvalidRegistry(RegistryError::DuplicateMethod)
    ));
    let public = duplicate.to_identus_error();
    assert_eq!(public.code().as_str(), "did.invalid_method_registry");
    assert_eq!(public.kind(), ErrorKind::Conflict);
    assert!(!public.to_string().contains("secretmethod"));

    let mut builder = DidMethodRegistry::builder();
    for index in 0..MAX_DID_METHOD_REGISTRY_ENTRIES {
        builder = builder
            .register(binding(&format!("method{index}"), Arc::new(PrismResolver)))
            .unwrap();
    }
    let excessive = builder
        .register(binding("overflow", Arc::new(PrismResolver)))
        .unwrap_err();
    assert!(matches!(
        excessive,
        Error::InvalidRegistry(RegistryError::TooManyMethods)
    ));
    assert_eq!(excessive.to_identus_error().kind(), ErrorKind::InvalidInput);
}

#[test]
fn introspection_is_sorted_bounded_and_does_not_expose_adapters() {
    let registry = registry();
    assert_eq!(registry.method_count(), 2);
    assert!(!registry.is_empty());
    assert!(registry.contains_method(&DidMethod::parse("prism").unwrap()));
    assert!(!registry.contains_method(&DidMethod::parse("example").unwrap()));
    assert_eq!(
        registry.methods().collect::<Vec<_>>(),
        ["midnight", "prism"]
    );

    let debug = format!("{registry:?}");
    assert!(debug.contains("midnight"));
    assert!(debug.contains("prism"));
    assert!(!debug.contains("Resolver"));
}

#[test]
fn cloned_registry_dispatches_concurrently_without_mutation() {
    let registry = registry();
    let barrier = Arc::new(Barrier::new(8));
    let handles = (0..8)
        .map(|index| {
            let registry = registry.clone();
            let barrier = Arc::clone(&barrier);
            std::thread::spawn(move || {
                barrier.wait();
                let (did, options) = if index % 2 == 0 {
                    (
                        Did::parse("did:prism:123").unwrap(),
                        ResolutionOptions::builder()
                            .extensions(BTreeMap::from([(
                                "network".to_owned(),
                                serde_json::json!("preprod"),
                            )]))
                            .build()
                            .unwrap(),
                    )
                } else {
                    (
                        Did::parse("did:midnight:undeployed:contract").unwrap(),
                        ResolutionOptions::builder()
                            .expand_relative_urls(false)
                            .build()
                            .unwrap(),
                    )
                };
                block_on(registry.resolve(&did, &options))
                    .validate_for(&did)
                    .unwrap();
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().unwrap();
    }
    assert_eq!(registry.method_count(), 2);
}

#[test]
#[ignore = "manual release-mode performance diagnostic"]
fn registry_lookup_and_dispatch_throughput_diagnostic() {
    const ITERATIONS: usize = 100_000;
    let registry: Arc<dyn DidResolver> = Arc::new(registry());
    let did = Did::parse("did:midnight:undeployed:contract").unwrap();
    let options = ResolutionOptions::builder()
        .expand_relative_urls(false)
        .build()
        .unwrap();

    let started = Instant::now();
    for _ in 0..ITERATIONS {
        black_box(block_on(
            registry.resolve(black_box(&did), black_box(&options)),
        ));
    }
    let elapsed = started.elapsed();
    println!(
        "dispatched {ITERATIONS} registered DID queries in {elapsed:?} ({:.0} queries/s)",
        ITERATIONS as f64 / elapsed.as_secs_f64()
    );
}
