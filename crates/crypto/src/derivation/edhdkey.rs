//! SLIP-0010 hierarchical-deterministic key derivation for ed25519 (`EdHDKey`).
//!
//! The spec's stated algorithm is SLIP-0010 (hardened-only ed25519 derivation),
//! whose published test vectors this module matches. NOTE: the KMP port origin
//! (`derivation.EdHDKey`) actually used the ed25519-bip32 (Khovratovich)
//! algorithm via a native uniffi wrapper (64-byte extended keys), not SLIP-0010
//! (32-byte keys). This module implements SLIP-0010 — the spec's authoritative
//! contract — and records the divergence from the KMP port origin in
//! `design.md`. See the change's `add-crypto-capability` Decision 8 / Open
//! Questions.

use crate::derivation::path::{DerivationAxis, DerivationPath};
use crate::error::Error;
use crate::hash::hmac_sha512;
use zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};

const KEY_SIZE: usize = 32;
const MASTER_KEY: &[u8] = b"ed25519 seed";

/// A SLIP-0010 ed25519 HD key.
///
/// Owned key material is zeroized on drop and omitted from [`Debug`](std::fmt::Debug).
/// The raw fields remain available for compatibility; callers are responsible
/// for protecting and erasing any copies they create.
#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct EdHDKey {
    /// The 32-byte private key. Any copied value becomes caller-owned secret material.
    pub private_key: [u8; KEY_SIZE],
    /// The 32-byte chain code. Any copied value becomes caller-owned secret material.
    pub chain_code: [u8; KEY_SIZE],
    /// The depth in the derivation tree.
    pub depth: u32,
    /// The child index that produced this key.
    pub index: u32,
}

impl std::fmt::Debug for EdHDKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EdHDKey")
            .field("depth", &self.depth)
            .field("index", &self.index)
            .finish_non_exhaustive()
    }
}

impl EdHDKey {
    /// Derive the master ed25519 HD key from a seed via SLIP-0010's master step
    /// (HMAC-SHA512 keyed with `"ed25519 seed"`).
    pub fn init_from_seed(seed: &[u8]) -> Result<Self, Error> {
        let i = Zeroizing::new(hmac_sha512(MASTER_KEY, seed));
        let mut private_key = [0u8; KEY_SIZE];
        let mut chain_code = [0u8; KEY_SIZE];
        private_key.copy_from_slice(&i[0..KEY_SIZE]);
        chain_code.copy_from_slice(&i[KEY_SIZE..]);
        Ok(Self {
            private_key,
            chain_code,
            depth: 0,
            index: 0,
        })
    }

    /// Derive a hardened child key. SLIP-0010 ed25519 supports hardened
    /// derivation only; for ed25519 the child private key IS the IL half of the
    /// HMAC output (no scalar addition).
    pub fn derive_child(&self, axis: DerivationAxis) -> Result<Self, Error> {
        if !axis.is_hardened() {
            return Err(Error::DerivationFailed);
        }
        // data = 0x00 || ser256(k_par) || ser32(i)
        let mut data = Zeroizing::new(Vec::with_capacity(1 + KEY_SIZE + 4));
        data.push(0x00);
        data.extend_from_slice(&self.private_key);
        data.extend_from_slice(&axis.raw().to_be_bytes());
        let i = Zeroizing::new(hmac_sha512(&self.chain_code, &data));
        let mut private_key = [0u8; KEY_SIZE];
        let mut chain_code = [0u8; KEY_SIZE];
        private_key.copy_from_slice(&i[0..KEY_SIZE]);
        chain_code.copy_from_slice(&i[KEY_SIZE..]);
        Ok(Self {
            private_key,
            chain_code,
            depth: self.depth + 1,
            index: axis.raw(),
        })
    }

    /// Derive a key along a path string (e.g. `m/0'/1'`). Only hardened axes
    /// are permitted for ed25519.
    pub fn derive(&self, path: &str) -> Result<Self, Error> {
        let parsed = DerivationPath::from_path(path)?;
        let mut current = self.clone();
        for axis in parsed.axes() {
            current = current.derive_child(*axis)?;
        }
        Ok(current)
    }
}
