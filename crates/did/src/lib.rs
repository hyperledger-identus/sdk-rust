//! Chain-neutral DID (Decentralized Identifier) syntax and domain primitives
//! for the Identus Rust SDK.
//!
//! Owns validated DID, DID URL, URI and DID document values plus foundational
//! method primitives. Resolution and method-specific semantics are layered on
//! top.

use identus_core::Component;

mod cache;
mod dereference;
mod did;
mod document;
pub mod error;
pub mod method;
mod multibase;
pub mod multihash;
mod query;
mod registration;
mod registry;
mod resolution;
mod uri;
pub mod version;
mod wire_json;

pub use cache::{
    CacheFailureMode, CachedDidResolution, CachedDidResolutionFuture, CachingDidResolver,
    DidResolutionCache, DidResolutionCacheEntry, DidResolutionCacheKey,
    DidResolutionCacheLookupFuture, DidResolutionCachePolicy, DidResolutionCacheStatus,
    DidResolutionCacheWriteFuture, MAX_DID_RESOLUTION_CACHE_ENTRIES,
    MAX_DID_RESOLUTION_CACHE_KEY_BYTES, MAX_DID_RESOLUTION_NOT_FOUND_TTL_MILLIS,
    MAX_DID_RESOLUTION_POSITIVE_TTL_MILLIS,
};
pub use dereference::GenericDidUrlDereferencer;
pub use did::{Did, DidUrl, MAX_DID_BYTES, MAX_DID_URL_BYTES};
pub use document::{
    ContextEntry, DidDocument, DidDocumentBuilder, MAX_DID_DOCUMENT_BYTES,
    MAX_DID_DOCUMENT_WIRE_DEPTH, MAX_DID_DOCUMENT_WIRE_LIVE_KEY_BYTES, MAX_DID_DOCUMENT_WIRE_NODES,
    MAX_DID_DOCUMENT_WIRE_OBJECT_MEMBERS, MAX_DOCUMENT_ITEMS, MAX_EXTENSION_DEPTH,
    MAX_EXTENSION_NODES, OneOrMany, Service, ServiceEndpoint, ServiceEndpointValue,
    VerificationMethod, VerificationRelationship,
};
pub use error::{
    CacheError, DidSyntaxError, DocumentError, Error, RegistrationError, RegistryError,
    ResolutionError, UriSyntaxError,
};
pub use method::DidMethod;
pub use multihash::Multihash;
pub use query::{
    DereferencingOptions, DereferencingOptionsBuilder, DidResolutionFuture, DidResolver,
    DidUrlDereferencer, DidUrlDereferencingFuture, MAX_DID_RESOLUTION_OPTIONS_BYTES,
    MAX_VERIFICATION_RELATIONSHIP_BYTES, ResolutionOptions, ResolutionOptionsBuilder,
    VerificationRelationshipName,
};
pub use registration::{
    CancelRegistrationRequest, ContinueRegistrationRequest, CreateRegistrationRequest,
    DeactivateRegistrationRequest, DidDocumentOperation, DidRegistrar, DidRegistrationErrorKind,
    DidRegistrationFuture, DidRegistrationResult, DidRegistrationState, InternalSecretPolicy,
    MAX_DID_REGISTRATION_BYTES, MAX_REGISTRATION_ID_BYTES, MAX_REGISTRATION_ITEMS,
    MAX_REGISTRATION_OPAQUE_ID_BYTES, MAX_REGISTRATION_WAIT_MILLIS, RegistrationAction,
    RegistrationActionId, RegistrationActionResponse, RegistrationContinuation,
    RegistrationFailureCode, RegistrationIdempotencyKey, RegistrationJob, RegistrationJobId,
    RegistrationOperationName, RegistrationPublicData, RegistrationRequest,
    RegistrationSecretHandle, RegistrationSecretMode, UpdateRegistrationRequest,
};
pub use registry::{
    DidMethodBinding, DidMethodRegistry, DidMethodRegistryBuilder, MAX_DID_METHOD_REGISTRY_ENTRIES,
};
pub use resolution::{
    DereferencedContent, DidDocumentMetadata, DidDocumentMetadataBuilder, DidResolutionDateTime,
    DidResolutionError, DidResolutionErrorKind, DidResolutionMetadata, DidResolutionResult,
    DidUrlContentMetadata, DidUrlDereferencingMetadata, DidUrlDereferencingResult,
    MAX_DID_RESOLUTION_DATETIME_BYTES, MAX_DID_RESOLUTION_RESULT_BYTES,
    MAX_DID_RESOLUTION_WIRE_DEPTH, MAX_DID_RESOLUTION_WIRE_LIVE_KEY_BYTES,
    MAX_DID_RESOLUTION_WIRE_NODES, MAX_DID_RESOLUTION_WIRE_OBJECT_MEMBERS, MAX_MEDIA_TYPE_BYTES,
    MAX_PROBLEM_DETAIL_BYTES, MAX_VERSION_ID_BYTES, MediaType, VersionId,
};
pub use uri::{MAX_URI_BYTES, Uri};
pub use version::Version;

/// Metadata for the `identus-did` crate.
pub const COMPONENT: Component = Component {
    name: "identus-did",
    summary: "Chain-neutral DID syntax and domain primitives for the Identus Rust SDK.",
};
