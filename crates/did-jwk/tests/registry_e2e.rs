use std::{
    future::Future,
    pin::pin,
    task::{Context, Poll, Waker},
};

use identus_crypto::{Ed25519PrivateKey, EncodeJwk};
use identus_did::{ContextEntry, DidMethodRegistry, DidResolver, MediaType, ResolutionOptions};
use identus_did_jwk::{DidJwk, DidJwkResolver};

#[test]
fn opt_in_registry_round_trip_is_deterministic_and_effect_free() {
    let created = DidJwk::create(
        Ed25519PrivateKey::from_slice(&[42; 32])
            .unwrap()
            .to_public_key()
            .encode_jwk(),
    )
    .unwrap();
    let registry = DidMethodRegistry::builder()
        .register(DidJwkResolver.into_method_binding())
        .unwrap()
        .build();
    let result = block_on(registry.resolve(created.did(), &ResolutionOptions::empty()));
    let document = result.document().expect("local document");
    assert_eq!(document.id(), created.did());
    let json = serde_json::to_string(document).unwrap();
    let reparsed = identus_did::DidDocument::from_json_str(&json).unwrap();
    assert_eq!(&reparsed, document);
    assert_eq!(
        reparsed.verification_methods().unwrap()[0]
            .public_key_jwk()
            .unwrap()["crv"],
        "Ed25519"
    );
    assert!(document.context().is_none());
}

#[test]
fn registry_projects_json_ld_contexts_only_when_negotiated() {
    let created = DidJwk::create(
        Ed25519PrivateKey::from_slice(&[43; 32])
            .unwrap()
            .to_public_key()
            .encode_jwk(),
    )
    .unwrap();
    let registry = DidMethodRegistry::builder()
        .register(DidJwkResolver.into_method_binding())
        .unwrap()
        .build();
    let options = ResolutionOptions::builder()
        .accept(MediaType::parse("application/did+ld+json").unwrap())
        .build()
        .unwrap();
    let result = block_on(registry.resolve(created.did(), &options));
    let contexts = result
        .document()
        .unwrap()
        .context()
        .unwrap()
        .as_slice()
        .iter()
        .map(|context| match context {
            ContextEntry::Uri(uri) => uri.as_str(),
            ContextEntry::Object(_) => panic!("did:jwk contexts are URI-only"),
        })
        .collect::<Vec<_>>();
    assert_eq!(
        contexts,
        [
            "https://www.w3.org/ns/did/v1",
            "https://w3id.org/security/suites/jws-2020/v1"
        ]
    );
}

fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = pin!(future);
    let mut context = Context::from_waker(Waker::noop());
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => return value,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}
