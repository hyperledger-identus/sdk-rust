//! Verifiable presentation data model and semantics for the Identus Rust SDK.
//!
//! Owns the W3C Verifiable Presentations data model and derivation semantics.
//! Sits in the `credential-semantics` layer and depends on foundation, the
//! credential primitives, and the trust primitives.

use identus_core::Component;

/// Metadata for the `identus-presentations` crate.
pub const COMPONENT: Component = Component {
    name: "identus-presentations",
    summary: "Verifiable presentation data model and semantics for the Identus Rust SDK.",
};
