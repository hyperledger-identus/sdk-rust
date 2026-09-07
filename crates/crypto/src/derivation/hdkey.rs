//! BIP32 hierarchical-deterministic key derivation for secp256k1 (`HDKey`).
//!
//! Ported from the KMP `derivation.HDKey`. Hardened child derivation only (the
//! KMP port throws on non-hardened children); the master key is derived from a
//! 16–64-byte seed via HMAC-SHA512 keyed with `"Bitcoin seed"`.

use k256::elliptic_curve::PrimeField;
use k256::{FieldBytes, Scalar};
use zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};

use crate::derivation::path::{DerivationAxis, DerivationPath};
use crate::error::Error;
use crate::hash::hmac_sha512;

const KEY_SIZE: usize = 32;
const MIN_SEED_SIZE: usize = 16;
const MAX_SEED_SIZE: usize = 64;
const MASTER_KEY: &[u8] = b"Bitcoin seed";

/// A BIP32 HD key for secp256k1.
///
/// Owned key material is zeroized on drop and omitted from [`Debug`](std::fmt::Debug).
/// The raw fields remain available for compatibility; callers are responsible
/// for protecting and erasing any copies they create.
#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct HDKey {
    /// The 32-byte private key. Any copied value becomes caller-owned secret material.
    pub private_key: [u8; KEY_SIZE],
    /// The 32-byte chain code. Any copied value becomes caller-owned secret material.
    pub chain_code: [u8; KEY_SIZE],
    /// The depth in the derivation tree.
    pub depth: u32,
    /// The child index that produced this key.
    pub child_index: u32,
}

impl std::fmt::Debug for HDKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HDKey")
            .field("depth", &self.depth)
            .field("child_index", &self.child_index)
            .finish_non_exhaustive()
    }
}

impl HDKey {
    /// Derive the master HD key from a 16–64-byte seed via HMAC-SHA512 keyed
    /// with `"Bitcoin seed"`.
    pub fn init_from_seed(seed: &[u8]) -> Result<Self, Error> {
        if !(MIN_SEED_SIZE..=MAX_SEED_SIZE).contains(&seed.len()) {
            return Err(Error::DerivationFailed);
        }

        Self::init_from_hmac(Zeroizing::new(hmac_sha512(MASTER_KEY, seed)))
    }

    fn init_from_hmac(i: Zeroizing<[u8; KEY_SIZE * 2]>) -> Result<Self, Error> {
        let mut private_key = [0u8; KEY_SIZE];
        let mut chain_code = [0u8; KEY_SIZE];
        private_key.copy_from_slice(&i[0..KEY_SIZE]);
        chain_code.copy_from_slice(&i[KEY_SIZE..]);

        // BIP-32 requires retrying when the master left half is zero or at
        // least the curve order. This facade derives one requested seed, so it
        // reports the established redacted failure instead of changing input.
        let master_scalar = Zeroizing::new(
            Option::<Scalar>::from(Scalar::from_repr(private_key.into()))
                .ok_or(Error::DerivationFailed)?,
        );
        if bool::from(master_scalar.is_zero()) {
            return Err(Error::DerivationFailed);
        }

        Ok(Self {
            private_key,
            chain_code,
            depth: 0,
            child_index: 0,
        })
    }

    /// Derive a child key at the given hardened [`DerivationAxis`].
    pub fn derive_child(&self, axis: DerivationAxis) -> Result<Self, Error> {
        // Only hardened derivation is supported (matching the KMP port).
        if !axis.is_hardened() {
            return Err(Error::DerivationFailed);
        }
        // data = 0x00 || ser256(k_par) || ser32(i)
        let mut data = Zeroizing::new(Vec::with_capacity(1 + KEY_SIZE + 4));
        data.push(0x00);
        data.extend_from_slice(&self.private_key);
        data.extend_from_slice(&axis.raw().to_be_bytes());

        self.derive_child_from_hmac(axis, Zeroizing::new(hmac_sha512(&self.chain_code, &data)))
    }

    fn derive_child_from_hmac(
        &self,
        axis: DerivationAxis,
        i: Zeroizing<[u8; KEY_SIZE * 2]>,
    ) -> Result<Self, Error> {
        let depth = self.depth.checked_add(1).ok_or(Error::DerivationFailed)?;
        let parent_scalar = Zeroizing::new(
            Option::<Scalar>::from(Scalar::from_repr(self.private_key.into()))
                .ok_or(Error::DerivationFailed)?,
        );
        if bool::from(parent_scalar.is_zero()) {
            return Err(Error::DerivationFailed);
        }

        let mut il = Zeroizing::new([0u8; KEY_SIZE]);
        il.copy_from_slice(&i[0..KEY_SIZE]);
        let il_bytes: FieldBytes = (*il).into();
        let il_scalar = Zeroizing::new(
            Option::<Scalar>::from(Scalar::from_repr(il_bytes)).ok_or(Error::DerivationFailed)?,
        );
        let child = Zeroizing::new(parent_scalar.as_ref() + il_scalar.as_ref());
        if bool::from(child.is_zero()) {
            return Err(Error::DerivationFailed);
        }

        let child_bytes = Zeroizing::new(child.to_bytes());
        let mut private_key = [0u8; KEY_SIZE];
        private_key.copy_from_slice(&child_bytes);

        let mut chain_code = [0u8; KEY_SIZE];
        chain_code.copy_from_slice(&i[KEY_SIZE..]);

        Ok(Self {
            private_key,
            chain_code,
            depth,
            child_index: axis.raw(),
        })
    }

    /// Derive a key along a BIP-32 path string (e.g. `m/0'/0'/0'`).
    pub fn derive(&self, path: &str) -> Result<Self, Error> {
        let parsed = DerivationPath::from_path(path)?;
        let mut current = self.clone();
        for axis in parsed.axes() {
            current = current.derive_child(*axis)?;
        }
        Ok(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ORDER: [u8; KEY_SIZE] = [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xfe, 0xba, 0xae, 0xdc, 0xe6, 0xaf, 0x48, 0xa0, 0x3b, 0xbf, 0xd2, 0x5e, 0x8c, 0xd0, 0x36,
        0x41, 0x41,
    ];
    const ORDER_MINUS_ONE: [u8; KEY_SIZE] = [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xfe, 0xba, 0xae, 0xdc, 0xe6, 0xaf, 0x48, 0xa0, 0x3b, 0xbf, 0xd2, 0x5e, 0x8c, 0xd0, 0x36,
        0x41, 0x40,
    ];

    fn hmac_halves(left: [u8; KEY_SIZE], right: [u8; KEY_SIZE]) -> Zeroizing<[u8; 64]> {
        let mut output = Zeroizing::new([0u8; 64]);
        output[..KEY_SIZE].copy_from_slice(&left);
        output[KEY_SIZE..].copy_from_slice(&right);
        output
    }

    fn one_key(depth: u32) -> HDKey {
        let mut private_key = [0u8; KEY_SIZE];
        private_key[KEY_SIZE - 1] = 1;
        HDKey {
            private_key,
            chain_code: [0x22; KEY_SIZE],
            depth,
            child_index: 0,
        }
    }

    #[test]
    fn every_normative_seed_length_is_accepted() {
        for length in MIN_SEED_SIZE..=MAX_SEED_SIZE {
            assert!(HDKey::init_from_seed(&vec![0x42; length]).is_ok());
        }
    }

    #[test]
    fn seed_lengths_outside_the_normative_range_are_rejected() {
        assert!(matches!(
            HDKey::init_from_seed(&[0x42; MIN_SEED_SIZE - 1]),
            Err(Error::DerivationFailed)
        ));
        assert!(matches!(
            HDKey::init_from_seed(&[0x42; MAX_SEED_SIZE + 1]),
            Err(Error::DerivationFailed)
        ));
    }

    #[test]
    fn invalid_master_scalars_are_rejected() {
        assert!(matches!(
            HDKey::init_from_hmac(hmac_halves([0; KEY_SIZE], [0x11; KEY_SIZE])),
            Err(Error::DerivationFailed)
        ));
        assert!(matches!(
            HDKey::init_from_hmac(hmac_halves(ORDER, [0x11; KEY_SIZE])),
            Err(Error::DerivationFailed)
        ));
    }

    #[test]
    fn child_tweak_at_the_curve_order_is_rejected_without_reduction() {
        let result = one_key(0).derive_child_from_hmac(
            DerivationAxis::hardened(0),
            hmac_halves(ORDER, [0x33; KEY_SIZE]),
        );
        assert!(matches!(result, Err(Error::DerivationFailed)));
    }

    #[test]
    fn zero_child_tweak_is_valid_and_updates_metadata() {
        let parent = one_key(7);
        let child = parent
            .derive_child_from_hmac(
                DerivationAxis::hardened(9),
                hmac_halves([0; KEY_SIZE], [0x44; KEY_SIZE]),
            )
            .unwrap();

        assert_eq!(child.private_key, parent.private_key);
        assert_eq!(child.chain_code, [0x44; KEY_SIZE]);
        assert_eq!(child.depth, 8);
        assert_eq!(child.child_index, DerivationAxis::hardened(9).raw());
    }

    #[test]
    fn zero_resulting_child_is_rejected() {
        let result = one_key(0).derive_child_from_hmac(
            DerivationAxis::hardened(0),
            hmac_halves(ORDER_MINUS_ONE, [0x55; KEY_SIZE]),
        );
        assert!(matches!(result, Err(Error::DerivationFailed)));
    }

    #[test]
    fn depth_overflow_is_rejected() {
        let result = one_key(u32::MAX).derive_child_from_hmac(
            DerivationAxis::hardened(0),
            hmac_halves([0; KEY_SIZE], [0x66; KEY_SIZE]),
        );
        assert!(matches!(result, Err(Error::DerivationFailed)));
    }
}
