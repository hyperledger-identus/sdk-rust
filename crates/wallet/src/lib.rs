//! Policy-neutral wallet orchestration contracts for the Identus Rust SDK.
//!
//! This crate currently owns only least-authority storage ports and their
//! bounded coordination vocabulary. It does not provide a wallet product,
//! storage adapter, custody, encryption, consent or persistence format.

#![forbid(unsafe_code)]

mod storage;

pub use storage::*;

use identus_core::Component;

/// Metadata for the `identus-wallet` crate.
pub const COMPONENT: Component = Component {
    name: "identus-wallet",
    summary: "Policy-neutral, least-authority wallet storage port contracts.",
};
