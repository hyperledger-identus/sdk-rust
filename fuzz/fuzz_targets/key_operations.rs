#![no_main]

use identus_crypto::{
    Ed25519PrivateKey, EncodeVec, Verifiable, X25519PrivateKey, X25519PublicKey,
};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Some(private_bytes) = data.get(..32) else {
        return;
    };
    let message = &data[32..];
    if let Ok(private) = Ed25519PrivateKey::from_slice(private_bytes) {
        let public = private.to_public_key();
        let signature = private.sign(message);
        assert!(public.verify(message, &signature));
        assert_eq!(public.encode_vec().len(), 32);
    }

    let Ok(private) = X25519PrivateKey::from_slice(private_bytes) else {
        return;
    };
    let Some(peer_bytes) = data.get(32..64) else {
        return;
    };
    let Ok(peer) = X25519PublicKey::from_slice(peer_bytes) else {
        return;
    };
    if peer.has_canonical_encoding() && peer.is_contributory() {
        assert_eq!(private.derive_shared(&peer).len(), 32);
    }
});
