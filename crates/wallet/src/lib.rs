//! Quarantined wallet placeholder for the Identus Rust SDK.
//!
//! No wallet product, custody, storage, consent or orchestration API is
//! accepted by this marker.

use identus_core::Component;

/// Metadata for the `identus-wallet` crate.
pub const COMPONENT: Component = Component {
    name: "identus-wallet",
    summary: "Quarantined wallet placeholder; product policy stays downstream.",
};
