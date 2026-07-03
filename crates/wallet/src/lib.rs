//! Wallet orchestration for the Identus Rust SDK.
//!
//! Owns the wallet abstraction that coordinates DID management, credential
//! storage, presentation exchange, and messaging flows. Sits in the
//! `orchestration` layer and depends on foundation, all domain primitives,
//! credential semantics, and protocol semantics.

use identus_core::Component;

/// Metadata for the `identus-wallet` crate.
pub const COMPONENT: Component = Component {
    name: "identus-wallet",
    summary: "Wallet orchestration for the Identus Rust SDK.",
};
