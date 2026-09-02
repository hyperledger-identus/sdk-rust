//! Quarantined agent placeholder for the Identus Rust SDK.
//!
//! No agent API, runtime or crate namespace is accepted by this marker.

use identus_core::Component;

/// Metadata for the `identus-agent` crate.
pub const COMPONENT: Component = Component {
    name: "identus-agent",
    summary: "Quarantined agent placeholder; no accepted runtime API.",
};
