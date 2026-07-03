//! OpenID for Verifiable Credentials (OpenID4VC) protocol semantics for the
//! Identus Rust SDK.
//!
//! Owns the OpenID4VCI issuance, OpenID4VP presentation, and SIOPv2
//! self-issued identity protocol semantics. Sits in the `protocol-semantics`
//! layer and depends on foundation, credential semantics, presentation
//! semantics, and trust primitives.

use identus_core::Component;

/// Metadata for the `identus-openid4vc` crate.
pub const COMPONENT: Component = Component {
    name: "identus-openid4vc",
    summary: "OpenID for Verifiable Credentials protocol semantics for the Identus Rust SDK.",
};
