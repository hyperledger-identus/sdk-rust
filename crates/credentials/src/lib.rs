//! Verifiable credential data model and semantics for the Identus Rust SDK.
//!
//! Owns the W3C Verifiable Credentials data model, issuance and verification
//! semantics. Sits in the `credential-semantics` layer and depends on the
//! foundation and domain-primitive layers.

use identus_core::Component;

/// Metadata for the `identus-credentials` crate.
pub const COMPONENT: Component = Component {
    name: "identus-credentials",
    summary: "Verifiable credential data model and semantics for the Identus Rust SDK.",
};
