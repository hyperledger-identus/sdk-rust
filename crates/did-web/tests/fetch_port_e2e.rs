use std::{
    future::Future,
    pin::pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
};

use identus_did::{Did, DidResolutionErrorKind, DidResolver, MediaType, ResolutionOptions};
use identus_did_web::{
    DidWebFetchFuture, DidWebFetcher, DidWebLimits, DidWebRequest, DidWebResolver, TransportFailure,
};

#[derive(Clone)]
struct InMemoryFetcher {
    seen: Arc<Mutex<Vec<(String, String, bool)>>>,
    status: u16,
    content_type: &'static str,
    document: Vec<u8>,
}

impl DidWebFetcher for InMemoryFetcher {
    fn fetch<'a>(&'a self, request: &'a DidWebRequest) -> DidWebFetchFuture<'a> {
        Box::pin(async move {
            self.seen.lock().unwrap().push((
                request.url().to_owned(),
                request.accept().to_owned(),
                request.no_cache(),
            ));
            request
                .response(self.status, Some(self.content_type), self.document.clone())
                .map_err(|_| TransportFailure)
        })
    }
}

#[test]
fn public_fetch_port_resolves_one_exact_document_end_to_end() {
    let did = Did::parse("did:web:example.com:users:alice").unwrap();
    let seen = Arc::new(Mutex::new(Vec::new()));
    let fetcher = InMemoryFetcher {
        seen: seen.clone(),
        status: 200,
        content_type: "application/did+json",
        document: br#"{"id":"did:web:example.com:users:alice","service":[{"id":"did:web:example.com:users:alice#inbox","type":"DIDCommMessaging","serviceEndpoint":"https://example.com/inbox"}]}"#.to_vec(),
    };
    let resolver = DidWebResolver::new(fetcher, DidWebLimits::default());
    let result = block_on(resolver.resolve(&did, &ResolutionOptions::default()));
    assert_eq!(result.document().unwrap().id(), &did);
    assert_eq!(result.document().unwrap().services().unwrap().len(), 1);
    assert_eq!(
        seen.lock().unwrap().as_slice(),
        [(
            "https://example.com/users/alice/did.json".to_owned(),
            "application/did+json, application/json".to_owned(),
            false,
        )]
    );
}

#[test]
fn resolver_separates_not_found_transient_and_representation_failures() {
    let did = Did::parse("did:web:example.com").unwrap();
    for (status, expected) in [
        (404, DidResolutionErrorKind::NotFound),
        (503, DidResolutionErrorKind::InternalError),
    ] {
        let resolver = DidWebResolver::new(
            InMemoryFetcher {
                seen: Arc::default(),
                status,
                content_type: "application/did+json",
                document: Vec::new(),
            },
            DidWebLimits::default(),
        );
        let result = block_on(resolver.resolve(&did, &ResolutionOptions::empty()));
        assert_eq!(
            result.metadata().error().and_then(|error| error.kind()),
            Some(expected)
        );
    }

    let options = ResolutionOptions::builder()
        .accept(MediaType::parse("application/did+ld+json").unwrap())
        .no_cache(true)
        .build()
        .unwrap();
    for (content_type, document, expected) in [
        (
            "application/did+json",
            br#"{"id":"did:web:example.com"}"#.as_slice(),
            DidResolutionErrorKind::RepresentationNotSupported,
        ),
        (
            "application/did+ld+json",
            br#"{"id":"did:web:example.com"}"#.as_slice(),
            DidResolutionErrorKind::InvalidDidDocument,
        ),
        (
            "application/did+ld+json",
            br#"{"@context":"https://example.com/context","id":"did:web:example.com"}"#.as_slice(),
            DidResolutionErrorKind::InvalidDidDocument,
        ),
    ] {
        let seen: Arc<Mutex<Vec<(String, String, bool)>>> = Arc::default();
        let resolver = DidWebResolver::new(
            InMemoryFetcher {
                seen: seen.clone(),
                status: 200,
                content_type,
                document: document.to_vec(),
            },
            DidWebLimits::default(),
        );
        let result = block_on(resolver.resolve(&did, &options));
        assert_eq!(
            result.metadata().error().and_then(|error| error.kind()),
            Some(expected)
        );
        let requests = seen.lock().unwrap();
        assert_eq!(
            requests.as_slice(),
            [(
                "https://example.com/.well-known/did.json".to_owned(),
                "application/did+ld+json".to_owned(),
                true,
            )]
        );
    }

    let resolver = DidWebResolver::new(
        InMemoryFetcher {
            seen: Arc::default(),
            status: 200,
            content_type: "application/did+ld+json",
            document: br#"{"@context":["https://www.w3.org/ns/did/v1","https://example.com/context"],"id":"did:web:example.com"}"#.to_vec(),
        },
        DidWebLimits::default(),
    );
    let result = block_on(resolver.resolve(&did, &options));
    assert_eq!(result.document().unwrap().id(), &did);
}

#[test]
fn resolver_normalizes_media_types_and_rejects_unsupported_expansion() {
    let did = Did::parse("did:web:example.com").unwrap();
    let seen: Arc<Mutex<Vec<(String, String, bool)>>> = Arc::default();
    let resolver = DidWebResolver::new(
        InMemoryFetcher {
            seen: seen.clone(),
            status: 200,
            content_type: "application/did+json",
            document: br#"{"id":"did:web:example.com"}"#.to_vec(),
        },
        DidWebLimits::default(),
    );
    let options = ResolutionOptions::builder()
        .accept(MediaType::parse("Application/DID+JSON").unwrap())
        .build()
        .unwrap();
    let result = block_on(resolver.resolve(&did, &options));
    assert_eq!(result.document().unwrap().id(), &did);
    assert_eq!(
        seen.lock().unwrap().as_slice(),
        [(
            "https://example.com/.well-known/did.json".to_owned(),
            "application/did+json, application/json".to_owned(),
            false,
        )]
    );

    seen.lock().unwrap().clear();
    let options = ResolutionOptions::builder()
        .expand_relative_urls(true)
        .build()
        .unwrap();
    let result = block_on(resolver.resolve(&did, &options));
    assert_eq!(
        result.metadata().error().and_then(|error| error.kind()),
        Some(DidResolutionErrorKind::FeatureNotSupported)
    );
    assert!(seen.lock().unwrap().is_empty());
}

fn block_on<T>(future: impl Future<Output = T>) -> T {
    let waker = Waker::noop();
    let mut context = Context::from_waker(waker);
    let mut future = pin!(future);
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => return value,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}
