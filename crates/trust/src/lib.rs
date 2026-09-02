//! Quarantined trust placeholder for the Identus Rust SDK.
//!
//! No trust policy, registry or verification-status API is accepted by this
//! marker.

use identus_core::Component;

/// Metadata for the `identus-trust` crate.
pub const COMPONENT: Component = Component {
    name: "identus-trust",
    summary: "Quarantined trust placeholder; no accepted domain API.",
};
