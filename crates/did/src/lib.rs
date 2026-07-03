//! DID (Decentralized Identifier) resolution and document model for the
//! Identus Rust SDK.
//!
//! Owns DID method primitives and the DID-document data model. Sits in the
//! `domain-primitives` layer and depends on foundation (`identus-core`) and
//! the cryptographic primitives (`identus-crypto`).

use identus_core::Component;

pub mod error;
pub mod method;
pub mod multihash;
pub mod version;

pub use error::Error;
pub use method::DidMethod;
pub use multihash::Multihash;
pub use version::Version;

/// Metadata for the `identus-did` crate.
pub const COMPONENT: Component = Component {
    name: "identus-did",
    summary: "DID resolution and document model for the Identus Rust SDK.",
};
