//! BIP32 hierarchical-deterministic key derivation for secp256k1 (`HDKey`).
//!
//! Ported from the KMP `derivation.HDKey`. Hardened child derivation only (the
//! KMP port throws on non-hardened children); the master key is derived from a
//! 64-byte seed via HMAC-SHA512 keyed with `"Bitcoin seed"`.

use k256::elliptic_curve::ops::Reduce;
use k256::{FieldBytes, Scalar, U256};

use crate::derivation::path::{DerivationAxis, DerivationPath};
use crate::error::Error;
use crate::hash::hmac_sha512;

const KEY_SIZE: usize = 32;
const MASTER_KEY: &[u8] = b"Bitcoin seed";

/// A BIP32 HD key for secp256k1.
#[derive(Debug, Clone)]
pub struct HDKey {
    /// The 32-byte private key.
    pub private_key: [u8; KEY_SIZE],
    /// The 32-byte chain code.
    pub chain_code: [u8; KEY_SIZE],
    /// The depth in the derivation tree.
    pub depth: u32,
    /// The child index that produced this key.
    pub child_index: u32,
}

impl HDKey {
    /// Derive the master HD key from a seed (any length, per BIP-32) via
    /// HMAC-SHA512 keyed with `"Bitcoin seed"`.
    pub fn init_from_seed(seed: &[u8]) -> Result<Self, Error> {
        let i = hmac_sha512(MASTER_KEY, seed);
        let mut private_key = [0u8; KEY_SIZE];
        let mut chain_code = [0u8; KEY_SIZE];
        private_key.copy_from_slice(&i[0..KEY_SIZE]);
        chain_code.copy_from_slice(&i[KEY_SIZE..]);
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
        let mut data = Vec::with_capacity(1 + KEY_SIZE + 4);
        data.push(0x00);
        data.extend_from_slice(&self.private_key);
        data.extend_from_slice(&axis.raw().to_be_bytes());
        let i = hmac_sha512(&self.chain_code, &data);
        let il = &i[0..KEY_SIZE];
        let ir = &i[KEY_SIZE..];

        let il_arr: [u8; KEY_SIZE] = il.try_into().map_err(|_| Error::DerivationFailed)?;
        let il_fb: FieldBytes = il_arr.into();
        let il_scalar: Scalar = <Scalar as Reduce<U256>>::reduce_bytes(&il_fb);
        let parent_fb: FieldBytes = self.private_key.into();
        let parent_scalar: Scalar = <Scalar as Reduce<U256>>::reduce_bytes(&parent_fb);
        let child = il_scalar + parent_scalar;
        if bool::from(child.is_zero()) {
            return Err(Error::DerivationFailed);
        }
        let child_bytes = child.to_bytes();
        let mut private_key = [0u8; KEY_SIZE];
        private_key.copy_from_slice(&child_bytes);

        let mut chain_code = [0u8; KEY_SIZE];
        chain_code.copy_from_slice(ir);

        Ok(Self {
            private_key,
            chain_code,
            depth: self.depth + 1,
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
