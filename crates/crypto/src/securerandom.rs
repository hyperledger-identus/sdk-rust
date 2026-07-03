//! `SecureRandom` — the single infrastructure port of `identus-crypto`.
//!
//! This crate defines the **port only**. Concrete adapters live in the
//! outer-boundary `identus-adapters-entropy` crate (the `getrandom`-backed
//! `GetrandomSystemRandomAdapter` is the cross-platform backend, building on
//! native and browser WASM via the `getrandom` `js` feature). `SecureRandom`
//! stays a port because a known second backend exists — `ring` does not build
//! on `wasm32-unknown-unknown`, so a `getrandom` backend is used instead.
//!
//! Key generation and mnemonic creation take an injected `&mut impl
//! SecureRandom` (dependency injection), since the domain crate cannot reach an
//! outer-boundary adapter.

use identus_derive as identus;

/// The entropy port: a source of cryptographically-secure random bytes.
///
/// Implementations live in `identus-adapters-entropy`; the domain injects an
/// adapter at the composition root. The `#[identus::port]` attribute declares
/// port-ness (an inert marker discovered by the `identus-conformance` naming
/// guard) and enforces the no-`Port`-suffix naming rule at compile time.
#[identus::port]
pub trait SecureRandom {
    /// Generate `num_bytes` cryptographically-secure random bytes.
    fn generate_seed(&mut self, num_bytes: usize) -> Vec<u8>;
}
