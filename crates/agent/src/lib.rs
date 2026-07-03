//! Agent orchestration for the Identus Rust SDK.
//!
//! Owns the agent abstraction that drives higher-level DIDComm and
//! credential-exchange workflows. Sits in the `orchestration` layer and
//! depends on foundation (`identus-core`); specific domain edges are added as
//! the orchestration surface is filled.

use identus_core::Component;

/// Metadata for the `identus-agent` crate.
pub const COMPONENT: Component = Component {
    name: "identus-agent",
    summary: "Agent orchestration for the Identus Rust SDK.",
};
