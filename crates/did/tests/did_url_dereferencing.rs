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

use identus_did::{
    DereferencingOptions, Did, DidDocument, DidDocumentMetadata, DidResolutionError,
    DidResolutionErrorKind, DidResolutionFuture, DidResolutionMetadata, DidResolutionResult,
    DidResolver, DidUrl, DidUrlDereferencer, GenericDidUrlDereferencer, MediaType,
    ResolutionOptions, VerificationRelationshipName,
};
use serde_json::json;

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

fn document(did: &str) -> DidDocument {
    DidDocument::from_json_str(&format!(
        r#"{{
            "id":"{did}",
            "verificationMethod":[{{
                "id":"{did}#key-1",
                "type":"JsonWebKey2020",
                "controller":"{did}",
                "publicKeyJwk":{{"kty":"OKP","crv":"Ed25519","x":"abc"}}
            }}],
            "authentication":["{did}#key-1"],
            "assertionMethod":["{did}#key-1"],
            "keyAgreement":[{{
                "id":"{did}#key-2",
                "type":"X25519KeyAgreementKey2020",
                "controller":"{did}",
                "publicKeyMultibase":"z6MkmM42vxfqZQsv4ehtTjFFxQ4sQKS2w6WR7emozFAn5cxu"
            }}],
            "service":[
                {{
                    "id":"{did}#files",
                    "type":["FileStore","Shared"],
                    "serviceEndpoint":[
                        "https://wallet.example/api/v1/",
                        {{"uri":"https://method.example/not-generic"}}
                    ]
                }},
                {{
                    "id":"{did}#profile",
                    "type":["Profile","Shared"],
                    "serviceEndpoint":"https://wallet.example/profile"
                }},
                {{
                    "id":"{did}#mapped",
                    "type":"Mapped",
                    "serviceEndpoint":{{"origins":["https://wallet.example"]}}
                }},
                {{
                    "id":"{did}#root",
                    "type":"Root",
                    "serviceEndpoint":"https://root.example"
                }}
            ]
        }}"#
    ))
    .unwrap()
}

fn success(did: &str) -> DidResolutionResult {
    DidResolutionResult::success(
        DidResolutionMetadata::new(
            Some(MediaType::parse("application/did+json").unwrap()),
            None,
            BTreeMap::new(),
        )
        .unwrap(),
        document(did),
        DidDocumentMetadata::builder()
            .version_id(identus_did::VersionId::parse("resolved-7").unwrap())
            .build()
            .unwrap(),
    )
    .unwrap()
}

fn failure(kind: DidResolutionErrorKind) -> DidResolutionResult {
    DidResolutionResult::failure(
        DidResolutionMetadata::new(
            None,
            Some(DidResolutionError::standard(kind)),
            BTreeMap::new(),
        )
        .unwrap(),
    )
    .unwrap()
}

struct RecordingResolver {
    did: Did,
    result: DidResolutionResult,
    calls: AtomicUsize,
    options: Mutex<Vec<ResolutionOptions>>,
}

impl RecordingResolver {
    fn new(did: &str, result: DidResolutionResult) -> Self {
        Self {
            did: Did::parse(did).unwrap(),
            result,
            calls: AtomicUsize::new(0),
            options: Mutex::new(Vec::new()),
        }
    }
}

impl DidResolver for RecordingResolver {
    fn resolve<'a>(
        &'a self,
        did: &'a Did,
        options: &'a ResolutionOptions,
    ) -> DidResolutionFuture<'a> {
        Box::pin(async move {
            assert_eq!(did, &self.did);
            self.calls.fetch_add(1, Ordering::SeqCst);
            self.options.lock().unwrap().push(options.clone());
            self.result.clone()
        })
    }
}

fn dereference(
    resolver: Arc<RecordingResolver>,
    url: &str,
    options: &DereferencingOptions,
) -> identus_did::DidUrlDereferencingResult {
    let dereferencer = GenericDidUrlDereferencer::new(resolver);
    block_on(dereferencer.dereference(&DidUrl::parse(url).unwrap(), options))
}

fn error_kind(result: &identus_did::DidUrlDereferencingResult) -> Option<DidResolutionErrorKind> {
    result.metadata().error().and_then(DidResolutionError::kind)
}

#[test]
fn prism_and_midnight_resolvers_share_bare_document_behavior() {
    for did in ["did:prism:abc123", "did:midnight:testnet:abc123"] {
        let resolver = Arc::new(RecordingResolver::new(did, success(did)));
        let result = dereference(resolver.clone(), did, &DereferencingOptions::empty());

        assert_eq!(
            result.content().unwrap().to_did_document().unwrap().id(),
            &resolver.did
        );
        assert_eq!(
            result.metadata().content_type().unwrap().as_str(),
            "application/did+json"
        );
        assert_eq!(
            result.content_metadata().values()["versionId"],
            "resolved-7"
        );
        assert_eq!(resolver.calls.load(Ordering::SeqCst), 1);
    }
}

#[test]
fn url_parameters_are_decoded_once_and_projected_without_form_semantics() {
    let did = "did:prism:abc123";
    let resolver = Arc::new(RecordingResolver::new(did, success(did)));
    let result = dereference(
        resolver.clone(),
        concat!(
            "did:prism:abc123?versionId=operation%2B7&",
            "versionTime=2026-09-03T12%3A30%3A00Z&hl=zQm%2Bhash&noCache=true"
        ),
        &DereferencingOptions::empty(),
    );

    assert!(result.metadata().error().is_none());
    let observed = resolver.options.lock().unwrap();
    assert_eq!(observed[0].version_id().unwrap().as_str(), "operation+7");
    assert_eq!(
        observed[0].version_time().unwrap().as_str(),
        "2026-09-03T12:30:00Z"
    );
    assert_eq!(observed[0].no_cache(), Some(true));
    assert_eq!(observed[0].extensions()["hl"], "zQm+hash");
}

#[test]
fn malformed_duplicate_and_colliding_parameters_fail_before_resolution() {
    let did = "did:prism:abc123";
    for url in [
        "did:prism:abc123?service=files&%73ervice=profile",
        "did:prism:abc123?=value",
        "did:prism:abc123?versionId=%00secret",
        "did:prism:abc123?noCache=maybe",
        "did:prism:abc123?",
    ] {
        let resolver = Arc::new(RecordingResolver::new(did, success(did)));
        let result = dereference(resolver.clone(), url, &DereferencingOptions::empty());
        assert_eq!(
            error_kind(&result),
            Some(DidResolutionErrorKind::InvalidDidUrl),
            "{url}"
        );
        assert_eq!(resolver.calls.load(Ordering::SeqCst), 0, "{url}");
    }

    let resolver = Arc::new(RecordingResolver::new(did, success(did)));
    let options = DereferencingOptions::builder()
        .extensions(BTreeMap::from([("hl".to_owned(), json!("from-options"))]))
        .build()
        .unwrap();
    let result = dereference(resolver.clone(), "did:prism:abc123?hl=from-url", &options);
    assert_eq!(
        error_kind(&result),
        Some(DidResolutionErrorKind::InvalidDidUrl)
    );
    assert_eq!(resolver.calls.load(Ordering::SeqCst), 0);

    let resolver = Arc::new(RecordingResolver::new(did, success(did)));
    let options = DereferencingOptions::builder()
        .accept(MediaType::parse("application/did+json").unwrap())
        .build()
        .unwrap();
    let result = dereference(
        resolver.clone(),
        "did:prism:abc123?accept=application%2Fdid%2Bjson",
        &options,
    );
    assert_eq!(
        error_kind(&result),
        Some(DidResolutionErrorKind::InvalidDidUrl)
    );
    assert_eq!(resolver.calls.load(Ordering::SeqCst), 0);
}

#[test]
fn fragments_match_exact_verification_methods_and_services() {
    let did = "did:prism:abc123";
    let resolver = Arc::new(RecordingResolver::new(did, success(did)));
    let method = dereference(
        resolver.clone(),
        "did:prism:abc123#key-1",
        &DereferencingOptions::empty(),
    );
    assert_eq!(
        method
            .content()
            .unwrap()
            .to_verification_method()
            .unwrap()
            .id()
            .as_str(),
        "did:prism:abc123#key-1"
    );

    let embedded = dereference(
        resolver.clone(),
        "did:prism:abc123#key-2",
        &DereferencingOptions::empty(),
    );
    assert_eq!(
        embedded
            .content()
            .unwrap()
            .to_verification_method()
            .unwrap()
            .id()
            .as_str(),
        "did:prism:abc123#key-2"
    );

    let service = dereference(
        resolver.clone(),
        "did:prism:abc123#profile",
        &DereferencingOptions::empty(),
    );
    assert_eq!(
        service
            .content()
            .unwrap()
            .to_service()
            .unwrap()
            .id()
            .as_str(),
        "did:prism:abc123#profile"
    );

    for fragment in ["key", "key-1-extra", "key%2D1"] {
        let missing = dereference(
            resolver.clone(),
            &format!("did:prism:abc123#{fragment}"),
            &DereferencingOptions::empty(),
        );
        assert_eq!(error_kind(&missing), Some(DidResolutionErrorKind::NotFound));
    }
}

#[test]
fn verification_relationship_requires_exact_membership_and_cid_errors() {
    let did = "did:prism:abc123";
    let resolver = Arc::new(RecordingResolver::new(did, success(did)));
    let assertion = DereferencingOptions::builder()
        .verification_relationship(VerificationRelationshipName::parse("assertionMethod").unwrap())
        .build()
        .unwrap();
    let accepted = dereference(resolver.clone(), "did:prism:abc123#key-1", &assertion);
    assert!(accepted.metadata().error().is_none());

    let agreement = DereferencingOptions::builder()
        .verification_relationship(VerificationRelationshipName::parse("keyAgreement").unwrap())
        .build()
        .unwrap();
    let unassociated = dereference(resolver.clone(), "did:prism:abc123#key-1", &agreement);
    assert_eq!(
        unassociated.metadata().error().unwrap().type_uri().as_str(),
        "https://w3id.org/security#INVALID_RELATIONSHIP_FOR_VERIFICATION_METHOD"
    );

    let missing = dereference(resolver.clone(), "did:prism:abc123#missing", &assertion);
    assert_eq!(
        missing.metadata().error().unwrap().type_uri().as_str(),
        "https://w3id.org/security#INVALID_VERIFICATION_METHOD"
    );

    let open = DereferencingOptions::builder()
        .verification_relationship(
            VerificationRelationshipName::parse("exampleRelationship").unwrap(),
        )
        .build()
        .unwrap();
    let unknown = dereference(resolver, "did:prism:abc123#key-1", &open);
    assert_eq!(
        unknown.metadata().error().unwrap().type_uri().as_str(),
        "https://w3id.org/security#INVALID_RELATIONSHIP_FOR_VERIFICATION_METHOD"
    );

    let resolver = Arc::new(RecordingResolver::new(did, success(did)));
    let invalid = dereference(resolver.clone(), did, &assertion);
    assert_eq!(
        error_kind(&invalid),
        Some(DidResolutionErrorKind::InvalidOptions)
    );
    assert_eq!(resolver.calls.load(Ordering::SeqCst), 0);

    let invalid = dereference(
        resolver.clone(),
        "did:prism:abc123?service=files#key-1",
        &assertion,
    );
    assert_eq!(
        error_kind(&invalid),
        Some(DidResolutionErrorKind::InvalidOptions)
    );
    assert_eq!(resolver.calls.load(Ordering::SeqCst), 0);
}

#[test]
fn service_and_type_selection_are_conjunctive_and_filter_documents() {
    let did = "did:prism:abc123";
    let resolver = Arc::new(RecordingResolver::new(did, success(did)));
    for selector in ["files", "%23files", "did%3Aprism%3Aabc123%23files"] {
        let result = dereference(
            resolver.clone(),
            &format!("did:prism:abc123?service={selector}&serviceType=FileStore"),
            &DereferencingOptions::empty(),
        );
        let selected = result.content().unwrap().to_did_document().unwrap();
        assert_eq!(selected.services().unwrap().len(), 1);
        assert_eq!(
            selected.services().unwrap()[0].id().as_str(),
            "did:prism:abc123#files"
        );
        let observed = resolver.options.lock().unwrap();
        assert!(
            observed
                .last()
                .unwrap()
                .extensions()
                .contains_key("service")
        );
        assert_eq!(
            observed.last().unwrap().extensions()["serviceType"],
            "FileStore"
        );
    }

    let mismatch = dereference(
        resolver,
        "did:prism:abc123?service=profile&serviceType=FileStore",
        &DereferencingOptions::empty(),
    );
    assert_eq!(
        error_kind(&mismatch),
        Some(DidResolutionErrorKind::NotFound)
    );
}

#[test]
fn uri_list_skips_endpoint_maps_and_rejects_unknown_representations() {
    let did = "did:prism:abc123";
    let resolver = Arc::new(RecordingResolver::new(did, success(did)));
    let uri_list = DereferencingOptions::builder()
        .accept(MediaType::parse("text/uri-list;charset=utf-8").unwrap())
        .build()
        .unwrap();
    let result = dereference(
        resolver.clone(),
        "did:prism:abc123?serviceType=Shared",
        &uri_list,
    );
    assert_eq!(
        result.metadata().content_type().unwrap().as_str(),
        "text/uri-list"
    );
    assert_eq!(
        result.content().unwrap().as_value(),
        &json!([
            "https://wallet.example/api/v1/",
            "https://wallet.example/profile"
        ])
    );

    let mapped = dereference(
        resolver.clone(),
        "did:prism:abc123?service=mapped",
        &uri_list,
    );
    assert_eq!(error_kind(&mapped), Some(DidResolutionErrorKind::NotFound));

    let unsupported = DereferencingOptions::builder()
        .accept(MediaType::parse("application/pdf").unwrap())
        .build()
        .unwrap();
    let result = dereference(resolver, "did:prism:abc123?service=files", &unsupported);
    assert_eq!(
        error_kind(&result),
        Some(DidResolutionErrorKind::RepresentationNotSupported)
    );
}

#[test]
fn safe_relative_references_resolve_without_retrieval_and_inherit_fragment() {
    let did = "did:prism:abc123";
    let resolver = Arc::new(RecordingResolver::new(did, success(did)));
    let result = dereference(
        resolver.clone(),
        "did:prism:abc123?service=files&relativeRef=receipts%2F42%3Fview%3Dfull#proof",
        &DereferencingOptions::empty(),
    );
    assert_eq!(
        result.content().unwrap().as_value(),
        &json!(["https://wallet.example/api/v1/receipts/42?view=full#proof"])
    );
    assert_eq!(
        result.content_metadata().values()["versionId"],
        "resolved-7"
    );
    assert_eq!(resolver.calls.load(Ordering::SeqCst), 1);
    assert_eq!(
        resolver.options.lock().unwrap()[0].extensions()["relativeRef"],
        "receipts/42?view=full"
    );

    for relative in ["%3Fpage%3D2", "%23section"] {
        let result = dereference(
            resolver.clone(),
            &format!("did:prism:abc123?service=files&relativeRef={relative}"),
            &DereferencingOptions::empty(),
        );
        assert!(result.metadata().error().is_none(), "{relative}");
    }

    let root = dereference(
        resolver,
        "did:prism:abc123?service=root&relativeRef=child",
        &DereferencingOptions::empty(),
    );
    assert_eq!(
        root.content().unwrap().as_value(),
        &json!(["https://root.example/child"])
    );

    let ambiguous = dereference(
        Arc::new(RecordingResolver::new(did, success(did))),
        "did:prism:abc123?serviceType=Shared#proof",
        &DereferencingOptions::empty(),
    );
    assert_eq!(
        error_kind(&ambiguous),
        Some(DidResolutionErrorKind::InvalidOptions)
    );
}

#[test]
fn traversal_absolute_and_platform_relative_references_fail_closed() {
    let did = "did:prism:abc123";
    for relative in [
        "..%2Fsecret",
        "%252e%252e%252fsecret",
        "%25252e%25252e%25252fsecret",
        "%255c..%255csecret",
        "%2F%2Fevil.example%2Fsecret",
        "https%3A%2F%2Fevil.example%2Fsecret",
        "%2Foutside",
    ] {
        let resolver = Arc::new(RecordingResolver::new(did, success(did)));
        let result = dereference(
            resolver,
            &format!("did:prism:abc123?service=files&relativeRef={relative}"),
            &DereferencingOptions::empty(),
        );
        assert_eq!(
            error_kind(&result),
            Some(DidResolutionErrorKind::InvalidOptions),
            "{relative}"
        );
    }

    let unsafe_document = DidDocument::from_json_str(
        r#"{
            "id":"did:prism:abc123",
            "service":[{
                "id":"did:prism:abc123#unsafe-base",
                "type":"UnsafeBase",
                "serviceEndpoint":"https://wallet.example/api/%252e%252e/private/"
            }]
        }"#,
    )
    .unwrap();
    let unsafe_result = DidResolutionResult::success(
        DidResolutionMetadata::empty(),
        unsafe_document,
        DidDocumentMetadata::empty(),
    )
    .unwrap();
    let resolver = Arc::new(RecordingResolver::new(did, unsafe_result));
    let result = dereference(
        resolver,
        "did:prism:abc123?service=unsafe-base&relativeRef=child",
        &DereferencingOptions::empty(),
    );
    assert_eq!(
        error_kind(&result),
        Some(DidResolutionErrorKind::InvalidOptions)
    );
}

#[test]
fn method_specific_resources_and_relative_ref_without_service_are_not_guessed() {
    let did = "did:prism:abc123";
    for url in [
        "did:prism:abc123/custom/path",
        "did:prism:abc123?methodResource=7",
    ] {
        let resolver = Arc::new(RecordingResolver::new(did, success(did)));
        let result = dereference(resolver.clone(), url, &DereferencingOptions::empty());
        assert_eq!(error_kind(&result), Some(DidResolutionErrorKind::NotFound));
        assert_eq!(resolver.calls.load(Ordering::SeqCst), 1);
    }

    let resolver = Arc::new(RecordingResolver::new(did, success(did)));
    let result = dereference(
        resolver,
        "did:prism:abc123?relativeRef=child",
        &DereferencingOptions::empty(),
    );
    assert_eq!(
        error_kind(&result),
        Some(DidResolutionErrorKind::InvalidOptions)
    );
}

#[test]
fn upstream_errors_are_preserved_and_debug_is_redacted() {
    let did = "did:prism:private-subject";
    let resolver = Arc::new(RecordingResolver::new(
        did,
        failure(DidResolutionErrorKind::MethodNotSupported),
    ));
    let dereferencer = GenericDidUrlDereferencer::new(resolver.clone());
    let debug = format!("{dereferencer:?}");
    assert!(!debug.contains("private-subject"));

    let result = block_on(
        dereferencer.dereference(&DidUrl::parse(did).unwrap(), &DereferencingOptions::empty()),
    );
    assert_eq!(
        error_kind(&result),
        Some(DidResolutionErrorKind::MethodNotSupported)
    );
}

#[test]
fn generic_adapter_is_object_safe_and_concurrent() {
    let did = "did:midnight:testnet:abc123";
    let resolver = Arc::new(RecordingResolver::new(did, success(did)));
    let dereferencer: Arc<dyn DidUrlDereferencer> =
        Arc::new(GenericDidUrlDereferencer::new(resolver.clone()));

    let threads: Vec<_> = (0..8)
        .map(|_| {
            let dereferencer = dereferencer.clone();
            std::thread::spawn(move || {
                let url = DidUrl::parse("did:midnight:testnet:abc123#key-1").unwrap();
                block_on(dereferencer.dereference(&url, &DereferencingOptions::empty()))
            })
        })
        .collect();
    for thread in threads {
        assert!(thread.join().unwrap().metadata().error().is_none());
    }
    assert_eq!(resolver.calls.load(Ordering::SeqCst), 8);
}

#[test]
#[ignore = "manual release-mode performance diagnostic"]
fn bare_document_dereferencing_throughput_diagnostic() {
    const ITERATIONS: usize = 100_000;
    let did = "did:prism:abc123";
    let resolver = Arc::new(RecordingResolver::new(did, success(did)));
    let dereferencer = GenericDidUrlDereferencer::new(resolver);
    let url = DidUrl::parse(did).unwrap();
    let options = DereferencingOptions::empty();

    let started = Instant::now();
    for _ in 0..ITERATIONS {
        black_box(block_on(
            dereferencer.dereference(black_box(&url), black_box(&options)),
        ));
    }
    let elapsed = started.elapsed();
    println!(
        "dereferenced {ITERATIONS} bare DID documents in {elapsed:?} ({:.0} operations/s)",
        ITERATIONS as f64 / elapsed.as_secs_f64()
    );
}
