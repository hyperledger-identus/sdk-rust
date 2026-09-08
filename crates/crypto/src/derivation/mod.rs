//! Hierarchical key derivation primitives.
//!
//! General BIP32 secp256k1, SLIP-0010 Ed25519, and BIP39 support is gated by
//! `derivation`. Cardano/IOG Ed25519-BIP32 V2 support is independently gated
//! by `cardano-bip32`.

/// Maximum UTF-8 byte length accepted by a textual derivation path.
pub const MAX_DERIVATION_PATH_BYTES: usize = 4_096;

/// Maximum number of child axes accepted by a derivation operation.
///
/// BIP-32 serializes depth in one byte, so 255 is the deepest interoperable
/// non-master node.
pub const MAX_DERIVATION_PATH_AXES: usize = u8::MAX as usize;

/// Minimum seed length accepted by BIP-32 and SLIP-0010 master derivation.
pub const MIN_HD_SEED_BYTES: usize = 16;

/// Maximum seed length accepted by BIP-32 and SLIP-0010 master derivation.
pub const MAX_HD_SEED_BYTES: usize = 64;

#[cfg(feature = "cardano-bip32")]
pub mod cardano_v2;
#[cfg(feature = "derivation")]
pub mod edhdkey;
#[cfg(feature = "derivation")]
pub mod hdkey;
#[cfg(feature = "derivation")]
pub mod mnemonic;
pub mod path;

#[cfg(feature = "cardano-bip32")]
pub use cardano_v2::{CardanoV2ExtendedPrivateKey, CardanoV2ExtendedPublicKey};
#[cfg(feature = "derivation")]
pub use edhdkey::EdHDKey;
#[cfg(feature = "derivation")]
pub use hdkey::HDKey;
#[cfg(feature = "derivation")]
pub use mnemonic::MnemonicHelper;
