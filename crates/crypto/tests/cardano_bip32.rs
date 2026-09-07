use identus_crypto::{
    CardanoV2ExtendedPrivateKey, CardanoV2ExtendedPublicKey,
    error::{Error, error_code},
    path::{DerivationAxis, DerivationPath},
};
use zeroize::Zeroize;

// The parent and hardened-child vectors are shared by Apollo's pinned native
// donor and typed-io/ed25519-bip32. Apollo's Kotlin tests add the m/1' private
// and m/1852/1815/0 public vectors below.
const D1: &str = "f8a29231ee38d6c5bf715d5bac21c750577aa3798b22d79d65bf97d6fadea15adcd1ee1abdf78bd4be64731a12deb94d3671784112eb6f364b871851fd1c9a247384db9ad6003bbd08b3b1ddc0d07a597293ff85e961bf252b331262eddfad0d";
const D1_H0: &str = "60d399da83ef80d8d4f8d223239efdc2b8fef387e1b5219137ffb4e8fbdea15adc9366b7d003af37c11396de9a83734e30e05e851efa32745c9cd7b42712c890608763770eddf77248ab652984b21b849760d1da74a6f5bd633ce41adceef07a";
const D1_S0: &str = "e86a12ba078cdbdf044b488624a50b9f681086c5e7c005222c6fb69e02dfa15a28630505d5878465269ecf096b7ec855780e6e4aed06852676e8ced5bd66d1dad6324d15fe0641021a711f3ef93865b2e41c3cef61b155d57a988156074ce2a8";
const APOLLO_D1_H1: &str = "4057eb6cab9000e3b6fe7e556341da1ca2f5dde0b689a7b58cb93f1902dfa15a5a10732ff348051c6e0865c62931d4a73fa8050b8ff543b43fc0000a7e2c57009a170f689c8b9b3502ee846f457ab3dd1b017cfb2cd68865c7f24dbabcbc2256";
const APOLLO_PUBLIC_KEY: &str = "6fd8d9c696b01525cc45f15583fc9447c66e1c71fd1a11c8885368404cd0a4ab";
const APOLLO_PUBLIC_CHAIN_CODE: &str =
    "00b5f1652f5cbe257e567c883dc2b16e0a9568b19c5b81ea8bd197fc95e8bdcf";
const APOLLO_DERIVED_PUBLIC_KEY: &str =
    "b857a8cd1dbbfed1824359d9d9e58bc8ffb9f66812b404f4c6ffc315629835bf";
const APOLLO_DERIVED_PUBLIC_CHAIN_CODE: &str =
    "9db12d11a3559131a47f51f854a6234725ab8767d3fcc4c9908be55508f3c712";

fn bytes<const N: usize>(encoded: &str) -> [u8; N] {
    hex::decode(encoded)
        .expect("test vector must be valid hex")
        .try_into()
        .expect("test vector must have the expected length")
}

#[test]
fn upstream_hardened_private_vector_matches_byte_for_byte() {
    let parent = CardanoV2ExtendedPrivateKey::from_bytes(bytes(D1)).unwrap();
    let child = parent.derive_child(DerivationAxis::hardened(0)).unwrap();

    assert_eq!(child.expose_secret_bytes(), bytes(D1_H0));
}

#[test]
fn apollo_hardened_private_vector_matches_byte_for_byte() {
    let parent = CardanoV2ExtendedPrivateKey::from_bytes(bytes(D1)).unwrap();
    let path = DerivationPath::from_path("m/1'").unwrap();
    let child = parent.derive_path(&path).unwrap();

    assert_eq!(child.expose_secret_bytes(), bytes(APOLLO_D1_H1));
}

#[test]
fn independent_soft_private_vector_matches_byte_for_byte() {
    // Independently reproduced with outscript 0.1.2's native Rust V2
    // implementation during ADR 0078 research; outscript is not a runtime or
    // development dependency of this crate.
    let parent = CardanoV2ExtendedPrivateKey::from_bytes(bytes(D1)).unwrap();
    let child = parent.derive_child(DerivationAxis::normal(0)).unwrap();

    assert_eq!(child.expose_secret_bytes(), bytes(D1_S0));
}

#[test]
fn apollo_soft_public_path_matches_byte_for_byte() {
    let parent = CardanoV2ExtendedPublicKey::from_public_key_and_chain_code(
        &bytes(APOLLO_PUBLIC_KEY),
        &bytes(APOLLO_PUBLIC_CHAIN_CODE),
    );
    let path = DerivationPath::from_path("m/1852/1815/0").unwrap();
    let child = parent.derive_path(&path).unwrap();

    assert_eq!(child.public_key_bytes(), bytes(APOLLO_DERIVED_PUBLIC_KEY));
    assert_eq!(child.chain_code(), bytes(APOLLO_DERIVED_PUBLIC_CHAIN_CODE));
}

#[test]
fn soft_public_derivation_equals_private_derivation() {
    let parent_private = CardanoV2ExtendedPrivateKey::from_bytes(bytes(D1)).unwrap();
    let axis = DerivationAxis::normal(42);

    let private_child_public = parent_private
        .derive_child(axis)
        .unwrap()
        .to_public_key()
        .unwrap();
    let public_child = parent_private
        .to_public_key()
        .unwrap()
        .derive_child(axis)
        .unwrap();

    assert_eq!(public_child, private_child_public);
}

#[test]
fn hardened_public_derivation_is_rejected_with_stable_redacted_error() {
    let public = CardanoV2ExtendedPrivateKey::from_bytes(bytes(D1))
        .unwrap()
        .to_public_key()
        .unwrap();
    let error = public
        .derive_child(DerivationAxis::hardened(0))
        .unwrap_err();

    assert!(matches!(error, Error::DerivationFailed));
    assert_eq!(error.to_string(), "key derivation failed");
    assert_eq!(
        error.to_identus_error().code(),
        error_code::DERIVATION_FAILED
    );
}

#[test]
fn private_material_is_redacted_and_zeroizable() {
    let mut private = CardanoV2ExtendedPrivateKey::from_bytes(bytes(D1)).unwrap();
    let private_debug = format!("{private:?}");
    let public_debug = format!("{:?}", private.to_public_key().unwrap());

    assert_eq!(private_debug, "CardanoV2ExtendedPrivateKey { .. }");
    assert_eq!(public_debug, "CardanoV2ExtendedPublicKey { .. }");
    assert!(!private_debug.contains(&D1[..32]));
    assert!(std::mem::needs_drop::<CardanoV2ExtendedPrivateKey>());

    private.zeroize();
    assert_eq!(private.expose_secret_bytes(), [0; 96]);
    assert!(matches!(
        private.derive_child(DerivationAxis::normal(0)),
        Err(Error::DerivationFailed)
    ));
}

#[test]
fn invalid_extended_private_keys_and_seed_lengths_are_rejected() {
    assert!(matches!(
        CardanoV2ExtendedPrivateKey::from_bytes([0; 96]),
        Err(Error::DerivationFailed)
    ));
    assert!(matches!(
        CardanoV2ExtendedPrivateKey::from_seed(&[0; 63]),
        Err(Error::InvalidKeySize {
            expected: 64,
            actual: 63,
            ..
        })
    ));
}

#[test]
fn apollo_seed_representation_constructs_a_valid_extended_key() {
    let seed = [7u8; 64];
    let private = CardanoV2ExtendedPrivateKey::from_seed(&seed).unwrap();

    assert!(private.to_public_key().is_ok());
    assert!(private.derive_child(DerivationAxis::normal(0)).is_ok());
}
