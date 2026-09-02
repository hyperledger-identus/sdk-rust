//! Quarantined credential placeholder for the Identus Rust SDK.
//!
//! No credential model, format or verification API is accepted by this marker.

use identus_core::Component;

/// Metadata for the `identus-credentials` crate.
pub const COMPONENT: Component = Component {
    name: "identus-credentials",
    summary: "Quarantined credential placeholder; no accepted domain API.",
};
