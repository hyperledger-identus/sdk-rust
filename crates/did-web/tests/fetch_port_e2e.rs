use std::{
    future::Future,
    pin::pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
};

use identus_did::{Did, DidResolver, ResolutionOptions};
use identus_did_web::{
    DidWebFetchFuture, DidWebFetcher, DidWebLimits, DidWebRequest, DidWebResolver, TransportFailure,
};

#[derive(Clone)]
struct InMemoryFetcher {
    seen: Arc<Mutex<Vec<String>>>,
    document: Vec<u8>,
}

impl DidWebFetcher for InMemoryFetcher {
    fn fetch<'a>(&'a self, request: &'a DidWebRequest) -> DidWebFetchFuture<'a> {
        Box::pin(async move {
            self.seen.lock().unwrap().push(request.url().to_owned());
            request
                .response(200, Some("application/did+json"), self.document.clone())
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
        document: br#"{"id":"did:web:example.com:users:alice","service":[{"id":"did:web:example.com:users:alice#inbox","type":"DIDCommMessaging","serviceEndpoint":"https://example.com/inbox"}]}"#.to_vec(),
    };
    let resolver = DidWebResolver::new(fetcher, DidWebLimits::default());
    let result = block_on(resolver.resolve(&did, &ResolutionOptions::default()));
    assert_eq!(result.document().unwrap().id(), &did);
    assert_eq!(result.document().unwrap().services().unwrap().len(), 1);
    assert_eq!(
        seen.lock().unwrap().as_slice(),
        ["https://example.com/users/alice/did.json"]
    );
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
