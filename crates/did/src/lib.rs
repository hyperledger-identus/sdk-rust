//! Chain-neutral DID (Decentralized Identifier) syntax and domain primitives
//! for the Identus Rust SDK.
//!
//! Owns validated DID and DID URL values plus foundational method primitives.
//! DID documents, resolution and method-specific semantics are layered on top.

use identus_core::Component;

mod did;
pub mod error;
pub mod method;
pub mod multihash;
pub mod version;

pub use did::{Did, DidUrl, MAX_DID_BYTES, MAX_DID_URL_BYTES};
pub use error::{DidSyntaxError, Error};
pub use method::DidMethod;
pub use multihash::Multihash;
pub use version::Version;

/// Metadata for the `identus-did` crate.
pub const COMPONENT: Component = Component {
    name: "identus-did",
    summary: "Chain-neutral DID syntax and domain primitives for the Identus Rust SDK.",
};
