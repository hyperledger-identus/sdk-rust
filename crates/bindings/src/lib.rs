//! Quarantined bindings placeholder for the Identus Rust SDK.
//!
//! No FFI, language binding or raw-secret boundary is accepted by this marker.

use identus_core::Component;

/// Metadata for the `identus-bindings` crate.
pub const COMPONENT: Component = Component {
    name: "identus-bindings",
    summary: "Quarantined bindings placeholder; FFI is unsupported.",
};
