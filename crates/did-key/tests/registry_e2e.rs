use std::{
    future::Future,
    pin::pin,
    task::{Context, Poll, Waker},
};

use identus_crypto::Ed25519PrivateKey;
use identus_did::{
    ContextEntry, Did, DidMethodRegistry, DidResolutionErrorKind, DidResolver, MediaType,
    ResolutionOptions,
};
use identus_did_key::{DidKey, DidKeyResolver};

#[test]
fn opt_in_registry_round_trip_is_deterministic_and_effect_free() {
    let created = DidKey::create(
        Ed25519PrivateKey::from_slice(&[42; 32])
            .unwrap()
            .to_public_key(),
    )
    .unwrap();
    let registry = DidMethodRegistry::builder()
        .register(DidKeyResolver.into_method_binding())
        .unwrap()
        .build();
    let result = block_on(registry.resolve(created.did(), &ResolutionOptions::empty()));
    let document = result.document().expect("local document");
    assert_eq!(document.id(), created.did());
    let json = serde_json::to_string(document).unwrap();
    let reparsed = identus_did::DidDocument::from_json_str(&json).unwrap();
    assert_eq!(&reparsed, document);
    assert_eq!(
        reparsed.verification_methods().unwrap()[0].public_key_multibase(),
        Some(created.fingerprint())
    );
    assert!(document.context().is_none());
}

#[test]
fn registry_projects_json_ld_contexts_for_json_ld_representation() {
    let created = DidKey::create(
        Ed25519PrivateKey::from_slice(&[43; 32])
            .unwrap()
            .to_public_key(),
    )
    .unwrap();
    let registry = DidMethodRegistry::builder()
        .register(DidKeyResolver.into_method_binding())
        .unwrap()
        .build();
    let options = ResolutionOptions::builder()
        .accept(MediaType::parse("application/did+ld+json").unwrap())
        .build()
        .unwrap();

    let result = block_on(registry.resolve(created.did(), &options));
    assert_eq!(
        result.metadata().content_type().unwrap().as_str(),
        "application/did+ld+json"
    );
    let contexts = result
        .document()
        .expect("local JSON-LD document")
        .context()
        .expect("JSON-LD contexts")
        .as_slice();
    let context_uris: Vec<_> = contexts
        .iter()
        .map(|context| match context {
            ContextEntry::Uri(uri) => uri.as_str(),
            ContextEntry::Object(_) => panic!("did:key contexts are URI-only"),
        })
        .collect();
    assert_eq!(
        context_uris,
        [
            "https://www.w3.org/ns/did/v1",
            "https://w3id.org/security/multikey/v1"
        ]
    );
}

#[test]
fn resolver_rejects_oversized_identifier_before_method_owned_projection() {
    let did = Did::parse(&format!("did:key:{}", "1".repeat(129))).unwrap();
    let result = block_on(DidKeyResolver.resolve(&did, &ResolutionOptions::empty()));

    assert!(result.document().is_none());
    assert_eq!(
        result.metadata().error().and_then(|error| error.kind()),
        Some(DidResolutionErrorKind::InvalidDid)
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
