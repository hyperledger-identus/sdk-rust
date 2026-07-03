//! Foreign-language binding surfaces for the Identus Rust SDK.
//!
//! Owns the FFI and language-binding entry points that expose the wallet and
//! core surface to non-Rust consumers. Sits at the `outer-boundary` of the
//! hexagonal ring; it may depend on stable orchestration (`identus-wallet`)
//! and foundation (`identus-core`) but domain crates must never depend on it.

use identus_core::Component;

/// Metadata for the `identus-bindings` crate.
pub const COMPONENT: Component = Component {
    name: "identus-bindings",
    summary: "Foreign-language binding surfaces for the Identus Rust SDK.",
};
