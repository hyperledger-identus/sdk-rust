//! Chain-neutral DID (Decentralized Identifier) syntax and domain primitives
//! for the Identus Rust SDK.
//!
//! Owns validated DID, DID URL, URI and DID document values plus foundational
//! method primitives. Resolution and method-specific semantics are layered on
//! top.

use identus_core::Component;

mod did;
mod document;
pub mod error;
pub mod method;
pub mod multihash;
mod uri;
pub mod version;

pub use did::{Did, DidUrl, MAX_DID_BYTES, MAX_DID_URL_BYTES};
pub use document::{
    ContextEntry, DidDocument, DidDocumentBuilder, MAX_DID_DOCUMENT_BYTES, MAX_DOCUMENT_ITEMS,
    MAX_EXTENSION_DEPTH, MAX_EXTENSION_NODES, OneOrMany, Service, ServiceEndpoint,
    ServiceEndpointValue, VerificationMethod, VerificationRelationship,
};
pub use error::{DidSyntaxError, DocumentError, Error, UriSyntaxError};
pub use method::DidMethod;
pub use multihash::Multihash;
pub use uri::{MAX_URI_BYTES, Uri};
pub use version::Version;

/// Metadata for the `identus-did` crate.
pub const COMPONENT: Component = Component {
    name: "identus-did",
    summary: "Chain-neutral DID syntax and domain primitives for the Identus Rust SDK.",
};
