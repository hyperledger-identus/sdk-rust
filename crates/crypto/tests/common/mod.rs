//! Shared test helpers for `identus-crypto` integration tests.
//!
//! `identus-crypto` cannot dev-depend on the outer-boundary adapter crate, so
//! each test injects a small in-test deterministic [`SecureRandom`] impl.

use identus_crypto::SecureRandom;

/// A deterministic, stateful `SecureRandom` for tests. Each `generate_seed`
/// call advances an internal cursor so successive calls yield distinct bytes.
/// Never use this for production entropy.
pub struct DetRandom {
    pub cursor: usize,
}

impl DetRandom {
    pub fn new() -> Self {
        Self { cursor: 0 }
    }
}

impl Default for DetRandom {
    fn default() -> Self {
        Self::new()
    }
}

impl SecureRandom for DetRandom {
    fn generate_seed(&mut self, num_bytes: usize) -> Vec<u8> {
        let start = self.cursor;
        self.cursor = self.cursor.wrapping_add(num_bytes);
        (0..num_bytes)
            .map(|i| ((start.wrapping_add(i)) % 256) as u8)
            .collect()
    }
}
