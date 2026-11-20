#![no_main]

use identus_crypto::{
    path::DerivationPath, CardanoV2ExtendedPrivateKey, Ed25519PrivateKey, EdHDKey, EncodeVec,
    HDKey, Secp256k1PrivateKey,
};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Some((&profile, rest)) = data.split_first() else {
        return;
    };
    match profile % 3 {
        0 => {
            let Some((&length, payload)) = rest.split_first() else {
                return;
            };
            let seed_length = 16 + usize::from(length % 49);
            let Some(seed) = payload.get(..seed_length) else {
                return;
            };
            let Some(path_bytes) = payload.get(seed_length..) else {
                return;
            };
            let Ok(path) = std::str::from_utf8(path_bytes) else {
                return;
            };
            if let Ok(root) = HDKey::init_from_seed(seed) {
                if let Ok(child) = root.derive(path) {
                    let secret = child.expose_private_key();
                    let private = Secp256k1PrivateKey::from_slice(secret.expose_secret_bytes())
                        .expect("accepted BIP-32 secret must remain a secp256k1 scalar");
                    assert_eq!(private.to_public_key().encode_vec().len(), 33);
                }
            }
        }
        1 => {
            let Some((&length, payload)) = rest.split_first() else {
                return;
            };
            let seed_length = 16 + usize::from(length % 49);
            let Some(seed) = payload.get(..seed_length) else {
                return;
            };
            let Some(path_bytes) = payload.get(seed_length..) else {
                return;
            };
            let Ok(path) = std::str::from_utf8(path_bytes) else {
                return;
            };
            if let Ok(root) = EdHDKey::init_from_seed(seed) {
                if let Ok(child) = root.derive(path) {
                    let secret = child.expose_private_key();
                    let private = Ed25519PrivateKey::from_slice(secret.expose_secret_bytes())
                        .expect("accepted SLIP-0010 secret must remain 32 bytes");
                    assert_eq!(private.to_public_key().encode_vec().len(), 32);
                }
            }
        }
        _ => {
            let Some(seed) = rest.get(..64) else { return };
            let Some(path_bytes) = rest.get(64..) else {
                return;
            };
            let Ok(path_text) = std::str::from_utf8(path_bytes) else {
                return;
            };
            let Ok(path) = DerivationPath::from_path(path_text) else {
                return;
            };
            let Ok(root) = CardanoV2ExtendedPrivateKey::from_seed(seed) else {
                return;
            };
            if let Ok(private_child) = root.derive_path(&path) {
                assert_eq!(private_child.to_public_key().unwrap().to_bytes().len(), 64);
            }
            if path.axes().iter().all(|axis| !axis.is_hardened()) {
                let public = root.to_public_key().unwrap();
                if let Ok(public_child) = public.derive_path(&path) {
                    assert_eq!(public_child.to_bytes().len(), 64);
                }
            }
        }
    }
});
