//! DIDComm and messaging protocol semantics for the Identus Rust SDK.
//!
//! Owns the DIDComm v2 message envelope, forwarding, and protocol-message
//! semantics. Sits in the `protocol-semantics` layer and depends on foundation
//! and the domain-primitive layers.

use identus_core::Component;

/// Metadata for the `identus-messaging` crate.
pub const COMPONENT: Component = Component {
    name: "identus-messaging",
    summary: "DIDComm and messaging protocol semantics for the Identus Rust SDK.",
};
