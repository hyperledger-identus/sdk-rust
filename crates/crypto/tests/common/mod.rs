//! Shared test helpers for `identus-crypto` integration tests.
//!
//! `identus-crypto` cannot dev-depend on the outer-boundary adapter crate, so
//! each test injects a small in-test deterministic [`SecureRandom`] impl.

use identus_crypto::SecureRandom;

/// A deterministic, stateful `SecureRandom` for tests. Each `fill_bytes` call
/// advances an internal cursor so successive calls yield distinct bytes.
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
    fn fill_bytes(&mut self, output: &mut [u8]) -> Result<(), identus_crypto::Error> {
        let start = self.cursor;
        self.cursor = self.cursor.wrapping_add(output.len());
        for (index, byte) in output.iter_mut().enumerate() {
            *byte = ((start.wrapping_add(index)) % 256) as u8;
        }
        Ok(())
    }
}
