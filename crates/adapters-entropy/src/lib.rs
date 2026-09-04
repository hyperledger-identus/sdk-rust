//! Entropy-port adapters for the Identus Rust SDK.
//!
//! First adapter-family crate in the `outer-boundary` layer: it owns the
//! concrete adapters for the [`identus_crypto::SecureRandom`] entropy port.
//! The cross-platform system-RNG adapter is `GetrandomSystemRandomAdapter`
//! (behind the `getrandom` cargo feature, `default = []`). The implemented
//! portable crate set is host-tested on Linux/macOS and compile-checked for
//! Android ARM64, iOS ARM64 and browser WASM (`wasm32-unknown-unknown`, where
//! the `getrandom` `js` feature resolves entropy to `crypto.getRandomValues()`).
//! Cross-compilation is not a runtime or FFI support claim; see the repository
//! support policy. A deterministic test adapter is available behind the
//! `deterministic` feature for cross-crate test consumers (never for production
//! entropy).
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
/// Host-tested on Linux/macOS and compile-checked for Android ARM64, iOS ARM64
/// and browser WASM (`wasm32-unknown-unknown`). On browser WASM the `getrandom`
/// `js` feature resolves entropy to `crypto.getRandomValues()`; on native
/// targets it resolves to the host OS CSPRNG. Linking, bindings and runtime
/// integration remain downstream evidence.
#[cfg(feature = "getrandom")]
#[derive(Debug, Default, Clone, Copy)]
pub struct GetrandomSystemRandomAdapter;

#[cfg(feature = "getrandom")]
impl SecureRandom for GetrandomSystemRandomAdapter {
    fn fill_bytes(&mut self, output: &mut [u8]) -> Result<(), identus_crypto::Error> {
        getrandom::getrandom(output).map_err(|_| identus_crypto::Error::SecureRandomFailure)
    }
}

/// A deterministic [`SecureRandom`] adapter for **tests only**. It returns
/// bytes drawn from a fixed, repeatable 256-byte sequence, so
/// `generate`/`create_random_mnemonics` calls are reproducible across crates.
///
/// Never use this for production entropy.
#[cfg(feature = "deterministic")]
#[derive(Debug, Default, Clone, Copy)]
pub struct DeterministicRandomAdapter;

#[cfg(feature = "deterministic")]
impl SecureRandom for DeterministicRandomAdapter {
    fn fill_bytes(&mut self, output: &mut [u8]) -> Result<(), identus_crypto::Error> {
        for (index, byte) in output.iter_mut().enumerate() {
            *byte = (index % 256) as u8;
        }
        Ok(())
    }
}

#[cfg(all(test, feature = "getrandom"))]
mod tests {
    use super::*;

    #[test]
    fn getrandom_adapter_returns_requested_length_and_is_nonzero() {
        let mut adapter = GetrandomSystemRandomAdapter;
        let mut seed = [0u8; 32];
        adapter.fill_bytes(&mut seed).unwrap();
        assert!(seed.iter().any(|&b| b != 0), "seed must not be all-zero");

        adapter.fill_bytes(&mut []).unwrap();
    }
}

#[cfg(all(test, feature = "deterministic"))]
mod deterministic_tests {
    use super::*;

    #[test]
    fn deterministic_adapter_is_reproducible_across_calls() {
        let mut a = DeterministicRandomAdapter;
        let mut first = [0u8; 32];
        let mut second = [0u8; 32];
        a.fill_bytes(&mut first).unwrap();
        a.fill_bytes(&mut second).unwrap();
        assert_eq!(first, second, "same N must yield byte-identical seeds");
    }

    #[test]
    fn deterministic_adapter_is_reproducible_across_instances() {
        let mut seed_a = [0u8; 64];
        let mut seed_b = [0u8; 64];
        DeterministicRandomAdapter.fill_bytes(&mut seed_a).unwrap();
        DeterministicRandomAdapter.fill_bytes(&mut seed_b).unwrap();
        assert_eq!(
            seed_a, seed_b,
            "distinct instances must agree for the same N"
        );
    }

    #[test]
    fn deterministic_adapter_accepts_an_empty_slice() {
        DeterministicRandomAdapter.fill_bytes(&mut []).unwrap();
    }
}
