//! Cardano/IOG Ed25519-BIP32 V2 extended-key derivation.
//!
//! The implementation delegates the primitive arithmetic to the private
//! `ed25519-bip32` dependency. Its types never cross this module boundary:
//! extended private material is kept in an SDK-owned zeroizing type whose
//! formatting is deliberately redacted.

use crate::derivation::path::{DerivationAxis, DerivationPath};
use crate::error::Error;
use ed25519_bip32::{DerivationScheme, XPrv, XPub};
use zeroize::{Zeroize, ZeroizeOnDrop};

const SEED_SIZE: usize = 64;
const EXTENDED_PRIVATE_KEY_SIZE: usize = 96;
const EXTENDED_PUBLIC_KEY_SIZE: usize = 64;
const PRIVATE_KEY_SIZE: usize = 32;
const PUBLIC_KEY_SIZE: usize = 32;
const CHAIN_CODE_SIZE: usize = 32;

/// A Cardano/IOG Ed25519-BIP32 V2 extended private key.
///
/// The 96-byte representation contains a 64-byte extended secret followed by
/// a 32-byte chain code. Owned material is zeroized on drop and never included
/// in [`Debug`](std::fmt::Debug). Raw export is intentionally explicit because
/// every returned copy becomes caller-owned secret material.
#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct CardanoV2ExtendedPrivateKey {
    bytes: [u8; EXTENDED_PRIVATE_KEY_SIZE],
}

impl std::fmt::Debug for CardanoV2ExtendedPrivateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CardanoV2ExtendedPrivateKey")
            .finish_non_exhaustive()
    }
}

impl CardanoV2ExtendedPrivateKey {
    /// Construct from Apollo's 64-byte seed representation: a 32-byte
    /// non-extended Ed25519 private key followed by a 32-byte chain code.
    pub fn from_seed(seed: &[u8]) -> Result<Self, Error> {
        if seed.len() != SEED_SIZE {
            return Err(Error::InvalidKeySize {
                expected: SEED_SIZE,
                actual: seed.len(),
                key_type: "Cardano V2 seed",
            });
        }

        let private_key = seed[..PRIVATE_KEY_SIZE]
            .try_into()
            .map_err(|_| Error::DerivationFailed)?;
        let chain_code = seed[PRIVATE_KEY_SIZE..]
            .try_into()
            .map_err(|_| Error::DerivationFailed)?;
        Ok(Self::from_nonextended(private_key, chain_code))
    }

    /// Construct from a 32-byte non-extended Ed25519 private key and chain
    /// code, applying the Cardano V2 normalization used by Apollo.
    #[must_use]
    pub fn from_nonextended(
        private_key: &[u8; PRIVATE_KEY_SIZE],
        chain_code: &[u8; CHAIN_CODE_SIZE],
    ) -> Self {
        Self::from_inner(XPrv::from_nonextended_force(private_key, chain_code))
    }

    /// Parse and validate a serialized 96-byte extended private key.
    pub fn from_bytes(mut bytes: [u8; EXTENDED_PRIVATE_KEY_SIZE]) -> Result<Self, Error> {
        let lowest_scalar_bits_are_clear = (bytes[0] & 0b0000_0111) == 0;
        let highest_scalar_bits_are_normalized = (bytes[31] & 0b1100_0000) == 0b0100_0000;
        if !lowest_scalar_bits_are_clear || !highest_scalar_bits_are_normalized {
            bytes.zeroize();
            return Err(Error::DerivationFailed);
        }

        Ok(Self { bytes })
    }

    /// Export the serialized extended private key.
    ///
    /// The returned value is an independent secret copy that the caller must
    /// protect and erase.
    #[must_use]
    pub fn expose_secret_bytes(&self) -> [u8; EXTENDED_PRIVATE_KEY_SIZE] {
        self.bytes
    }

    /// Derive a hardened or soft child extended private key.
    pub fn derive_child(&self, axis: DerivationAxis) -> Result<Self, Error> {
        let parent = self.to_inner()?;
        Ok(Self::from_inner(
            parent.derive(DerivationScheme::V2, axis.raw()),
        ))
    }

    /// Derive an extended private key along a path of at most 255 axes.
    pub fn derive_path(&self, path: &DerivationPath) -> Result<Self, Error> {
        path.ensure_work_bound()?;
        let mut current = self.clone();
        for axis in path.axes() {
            current = current.derive_child(*axis)?;
        }
        Ok(current)
    }

    /// Derive the corresponding extended public key.
    pub fn to_public_key(&self) -> Result<CardanoV2ExtendedPublicKey, Error> {
        Ok(CardanoV2ExtendedPublicKey::from_inner(
            self.to_inner()?.public(),
        ))
    }

    fn from_inner(inner: XPrv) -> Self {
        Self {
            bytes: inner.into(),
        }
    }

    fn to_inner(&self) -> Result<XPrv, Error> {
        XPrv::from_bytes_verified(self.bytes).map_err(|_| Error::DerivationFailed)
    }
}

/// A Cardano/IOG Ed25519-BIP32 V2 extended public key.
///
/// Its 64-byte representation contains a 32-byte Ed25519 public key followed
/// by a 32-byte chain code. Formatting intentionally omits both fields so the
/// public facade cannot accidentally establish a logging convention that is
/// unsafe for the corresponding private type.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct CardanoV2ExtendedPublicKey {
    bytes: [u8; EXTENDED_PUBLIC_KEY_SIZE],
}

impl std::fmt::Debug for CardanoV2ExtendedPublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CardanoV2ExtendedPublicKey")
            .finish_non_exhaustive()
    }
}

impl CardanoV2ExtendedPublicKey {
    /// Construct from its serialized 64-byte representation.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; EXTENDED_PUBLIC_KEY_SIZE]) -> Self {
        Self { bytes }
    }

    /// Construct from a 32-byte Ed25519 public key and chain code.
    #[must_use]
    pub fn from_public_key_and_chain_code(
        public_key: &[u8; PUBLIC_KEY_SIZE],
        chain_code: &[u8; CHAIN_CODE_SIZE],
    ) -> Self {
        Self::from_inner(XPub::from_pk_and_chaincode(public_key, chain_code))
    }

    /// Return the serialized 64-byte extended public key.
    #[must_use]
    pub const fn to_bytes(&self) -> [u8; EXTENDED_PUBLIC_KEY_SIZE] {
        self.bytes
    }

    /// Return the 32-byte Ed25519 public-key component.
    #[must_use]
    pub fn public_key_bytes(&self) -> [u8; PUBLIC_KEY_SIZE] {
        self.bytes[..PUBLIC_KEY_SIZE]
            .try_into()
            .expect("the source range has the public-key size")
    }

    /// Return the 32-byte chain-code component.
    #[must_use]
    pub fn chain_code(&self) -> [u8; CHAIN_CODE_SIZE] {
        self.bytes[PUBLIC_KEY_SIZE..]
            .try_into()
            .expect("the source range has the chain-code size")
    }

    /// Derive a soft child extended public key.
    ///
    /// Hardened public derivation and invalid public points return the stable,
    /// redaction-safe [`Error::DerivationFailed`] variant.
    pub fn derive_child(&self, axis: DerivationAxis) -> Result<Self, Error> {
        if axis.is_hardened() {
            return Err(Error::DerivationFailed);
        }

        XPub::from_bytes(self.bytes)
            .derive(DerivationScheme::V2, axis.raw())
            .map(Self::from_inner)
            .map_err(|_| Error::DerivationFailed)
    }

    /// Derive an extended public key along an all-soft path of at most 255 axes.
    pub fn derive_path(&self, path: &DerivationPath) -> Result<Self, Error> {
        path.ensure_work_bound()?;
        let mut current = *self;
        for axis in path.axes() {
            current = current.derive_child(*axis)?;
        }
        Ok(current)
    }

    fn from_inner(inner: XPub) -> Self {
        Self {
            bytes: inner.into(),
        }
    }
}
