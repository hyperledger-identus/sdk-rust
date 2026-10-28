use std::{
    future::Future,
    pin::pin,
    task::{Context, Poll, Waker},
};

use identus_crypto::Ed25519PrivateKey;
use identus_did::{DidMethodRegistry, DidResolver, ResolutionOptions};
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
