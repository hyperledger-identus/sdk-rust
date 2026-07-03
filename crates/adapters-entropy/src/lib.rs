//! Entropy-port adapters for the Identus Rust SDK.
//!
//! First adapter-family crate in the `outer-boundary` layer: it owns the
//! concrete adapters for the [`identus_crypto::SecureRandom`] entropy port.
//! The cross-platform system-RNG adapter is `GetrandomSystemRandomAdapter`
//! (behind the `getrandom` cargo feature, `default = []`): it builds on every
//! uniffi target — Kotlin/JVM, Android, native, and browser WASM
//! (`wasm32-unknown-unknown`, where the `getrandom` `js` feature resolves
//! entropy to `crypto.getRandomValues()`). A deterministic test adapter is
//! available behind the `deterministic` feature for cross-crate test consumers
//! (never for production entropy).
//!
//! WASI (`wasm32-wasi` / `wasip1`) is out of scope: the workspace target set
//! is browser WASM only (`wasm32-unknown-unknown`), matching `sdk-ts`'s
//! `wasm-pack --target=web` pipeline.
//!
//! Adapter structs follow the `<Backend><Capability>Adapter` naming
//! convention: `GetrandomSystemRandomAdapter` (getrandom, system RNG),
//! `DeterministicRandomAdapter` (deterministic test adapter).
//!
//! Sits at the `outer-boundary` of the hexagonal ring and depends only inward
//! (`identus-core`, `identus-crypto`); domain crates must never depend on it.
//! The composition root (binary/binding) wires a concrete adapter into
//! `*KeyPair::generate` / `MnemonicHelper::create_random_mnemonics` via
//! dependency injection.

use identus_core::Component;

#[cfg(any(feature = "getrandom", feature = "deterministic"))]
use identus_crypto::SecureRandom;

/// Metadata for the `identus-adapters-entropy` crate.
pub const COMPONENT: Component = Component {
    name: "identus-adapters-entropy",
    summary: "Entropy-port adapters (SecureRandom) for the Identus Rust SDK.",
};

/// `getrandom`-backed [`SecureRandom`] adapter using the platform system RNG.
///
/// Works across all uniffi targets: Kotlin/JVM, Android, native, and browser
/// WASM (`wasm32-unknown-unknown`). On browser WASM the `getrandom` `js`
/// feature resolves entropy to `crypto.getRandomValues()`; on native targets it
/// resolves to the host OS CSPRNG.
#[cfg(feature = "getrandom")]
#[derive(Debug, Default, Clone, Copy)]
pub struct GetrandomSystemRandomAdapter;

#[cfg(feature = "getrandom")]
impl SecureRandom for GetrandomSystemRandomAdapter {
    fn generate_seed(&mut self, num_bytes: usize) -> Vec<u8> {
        let mut out = vec![0u8; num_bytes];
        getrandom::getrandom(&mut out)
            .expect("getrandom::getrandom must not fail on supported targets");
        out
    }
}

/// A deterministic [`SecureRandom`] adapter for **tests only**. It returns
/// `num_bytes` bytes drawn from a fixed, repeatable 256-byte sequence, so
/// `generate`/`create_random_mnemonics` calls are reproducible across crates.
///
/// Never use this for production entropy.
#[cfg(feature = "deterministic")]
#[derive(Debug, Default, Clone, Copy)]
pub struct DeterministicRandomAdapter;

#[cfg(feature = "deterministic")]
impl SecureRandom for DeterministicRandomAdapter {
    fn generate_seed(&mut self, num_bytes: usize) -> Vec<u8> {
        const POOL: [u8; 256] = {
            let mut p = [0u8; 256];
            let mut i = 0;
            while i < 256 {
                p[i] = (i % 256) as u8;
                i += 1;
            }
            p
        };
        let mut out = Vec::with_capacity(num_bytes);
        let mut idx: usize = 0;
        while out.len() < num_bytes {
            out.push(POOL[idx % 256]);
            idx = idx.wrapping_add(1);
        }
        out
    }
}

#[cfg(all(test, feature = "getrandom"))]
mod tests {
    use super::*;

    #[test]
    fn getrandom_adapter_returns_requested_length_and_is_nonzero() {
        let mut adapter = GetrandomSystemRandomAdapter;
        let seed = adapter.generate_seed(32);
        assert_eq!(seed.len(), 32);
        assert!(seed.iter().any(|&b| b != 0), "seed must not be all-zero");
    }
}

#[cfg(all(test, feature = "deterministic"))]
mod deterministic_tests {
    use super::*;

    #[test]
    fn deterministic_adapter_is_reproducible_across_calls() {
        let mut a = DeterministicRandomAdapter;
        let first = a.generate_seed(32);
        let second = a.generate_seed(32);
        assert_eq!(first, second, "same N must yield byte-identical seeds");
    }

    #[test]
    fn deterministic_adapter_is_reproducible_across_instances() {
        let seed_a = DeterministicRandomAdapter.generate_seed(64);
        let seed_b = DeterministicRandomAdapter.generate_seed(64);
        assert_eq!(
            seed_a, seed_b,
            "distinct instances must agree for the same N"
        );
    }
}
