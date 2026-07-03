//! Trust establishment and trust-resolution primitives for the Identus Rust
//! SDK.
//!
//! Owns trust registries, trust policies, and verification-status primitives.
//! Sits in the `domain-primitives` layer and depends on foundation
//! (`identus-core`) and the cryptographic and DID primitives.

use identus_core::Component;

/// Metadata for the `identus-trust` crate.
pub const COMPONENT: Component = Component {
    name: "identus-trust",
    summary: "Trust establishment and trust-resolution primitives for the Identus Rust SDK.",
};
