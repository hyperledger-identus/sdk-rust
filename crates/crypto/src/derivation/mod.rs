//! Hierarchical key derivation primitives.
//!
//! General BIP32 secp256k1, SLIP-0010 Ed25519, and BIP39 support is gated by
//! `derivation`. Cardano/IOG Ed25519-BIP32 V2 support is independently gated
//! by `cardano-bip32`.

#[cfg(feature = "cardano-bip32")]
pub mod cardano_v2;
#[cfg(feature = "derivation")]
pub mod edhdkey;
#[cfg(feature = "derivation")]
pub mod hdkey;
#[cfg(feature = "derivation")]
pub mod mnemonic;
pub mod path;
#[cfg(feature = "derivation")]
pub mod wordlist;

#[cfg(feature = "cardano-bip32")]
pub use cardano_v2::{CardanoV2ExtendedPrivateKey, CardanoV2ExtendedPublicKey};
#[cfg(feature = "derivation")]
pub use edhdkey::EdHDKey;
#[cfg(feature = "derivation")]
pub use hdkey::HDKey;
#[cfg(feature = "derivation")]
pub use mnemonic::MnemonicHelper;
