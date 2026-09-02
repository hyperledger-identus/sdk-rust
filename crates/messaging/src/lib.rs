//! Quarantined messaging placeholder for the Identus Rust SDK.
//!
//! No DIDComm or messaging protocol API is accepted by this marker.

use identus_core::Component;

/// Metadata for the `identus-messaging` crate.
pub const COMPONENT: Component = Component {
    name: "identus-messaging",
    summary: "Quarantined messaging placeholder; no accepted protocol API.",
};
