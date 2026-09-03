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
mod query;
mod resolution;
mod uri;
pub mod version;

pub use did::{Did, DidUrl, MAX_DID_BYTES, MAX_DID_URL_BYTES};
pub use document::{
    ContextEntry, DidDocument, DidDocumentBuilder, MAX_DID_DOCUMENT_BYTES, MAX_DOCUMENT_ITEMS,
    MAX_EXTENSION_DEPTH, MAX_EXTENSION_NODES, OneOrMany, Service, ServiceEndpoint,
    ServiceEndpointValue, VerificationMethod, VerificationRelationship,
};
pub use error::{DidSyntaxError, DocumentError, Error, ResolutionError, UriSyntaxError};
pub use method::DidMethod;
pub use multihash::Multihash;
pub use query::{
    DereferencingOptions, DereferencingOptionsBuilder, DidResolutionFuture, DidResolver,
    DidUrlDereferencer, DidUrlDereferencingFuture, MAX_DID_RESOLUTION_OPTIONS_BYTES,
    MAX_VERIFICATION_RELATIONSHIP_BYTES, ResolutionOptions, ResolutionOptionsBuilder,
    VerificationRelationshipName,
};
pub use resolution::{
    DereferencedContent, DidDocumentMetadata, DidDocumentMetadataBuilder, DidResolutionDateTime,
    DidResolutionError, DidResolutionErrorKind, DidResolutionMetadata, DidResolutionResult,
    DidUrlContentMetadata, DidUrlDereferencingMetadata, DidUrlDereferencingResult,
    MAX_DID_RESOLUTION_RESULT_BYTES, MAX_MEDIA_TYPE_BYTES, MAX_PROBLEM_DETAIL_BYTES,
    MAX_VERSION_ID_BYTES, MediaType, VersionId,
};
pub use uri::{MAX_URI_BYTES, Uri};
pub use version::Version;

/// Metadata for the `identus-did` crate.
pub const COMPONENT: Component = Component {
    name: "identus-did",
    summary: "Chain-neutral DID syntax and domain primitives for the Identus Rust SDK.",
};
