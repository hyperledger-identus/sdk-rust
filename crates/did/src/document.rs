//! Bounded, extensible W3C DID document structural model.
//!
//! The model owns syntax, shape and resource invariants. DID method,
//! cryptosuite, resolution, dereferencing and authorization policy belong to
//! adapters above this crate.

use crate::{Error, error::DocumentError};

mod cardinality;
mod extension;
mod model;
mod service;
mod verification;

pub use cardinality::OneOrMany;
pub use extension::{ContextEntry, ContextObject};
pub use model::{DidDocument, DidDocumentBuilder};
pub use service::{Service, ServiceEndpoint, ServiceEndpointValue};
pub use verification::{VerificationMethod, VerificationRelationship};

pub(crate) use extension::{JsonBudget, validate_json_map, validate_json_value};
use service::collect_service_json;
use verification::collect_verification_method_json;

/// Maximum raw JSON size accepted by [`DidDocument::from_json_slice`].
pub const MAX_DID_DOCUMENT_BYTES: usize = 256 * 1_024;
/// Maximum items in any DID document or extension collection.
pub const MAX_DOCUMENT_ITEMS: usize = 128;
/// Maximum nested levels in an arbitrary JSON extension tree.
pub const MAX_EXTENSION_DEPTH: usize = 32;
/// Maximum JSON nodes across arbitrary data in one DID document.
pub const MAX_EXTENSION_NODES: usize = 4_096;
/// Maximum containers nested in raw DID document JSON during preflight.
pub const MAX_DID_DOCUMENT_WIRE_DEPTH: usize = 64;
/// Maximum JSON values visited during raw DID document preflight.
pub const MAX_DID_DOCUMENT_WIRE_NODES: usize = 16_384;
/// Maximum members permitted in one raw DID document JSON object.
pub const MAX_DID_DOCUMENT_WIRE_OBJECT_MEMBERS: usize = 128;
/// Maximum decoded object-name bytes retained simultaneously during preflight.
pub const MAX_DID_DOCUMENT_WIRE_LIVE_KEY_BYTES: usize = 128 * 1_024;

pub(crate) const MAX_EXTENSION_PROPERTIES: usize = 64;
pub(crate) const MAX_PROPERTY_NAME_BYTES: usize = 256;
pub(crate) const MAX_EXTENSION_STRING_BYTES: usize = 64 * 1_024;
const MAX_OPEN_TYPE_BYTES: usize = 256;
const MAX_PUBLIC_KEY_MULTIBASE_BYTES: usize = 4 * 1_024;

const DOCUMENT_RESERVED: &[&str] = &[
    "@context",
    "id",
    "controller",
    "alsoKnownAs",
    "verificationMethod",
    "authentication",
    "assertionMethod",
    "keyAgreement",
    "capabilityInvocation",
    "capabilityDelegation",
    "service",
];
const VERIFICATION_RESERVED: &[&str] = &["id", "type", "controller"];
const SERVICE_RESERVED: &[&str] = &["id", "type", "serviceEndpoint"];
const PRIVATE_JWK_MEMBERS: &[&str] = &["d", "p", "q", "dp", "dq", "qi", "oth", "k"];

fn validate_open_string(value: &str, max_bytes: usize) -> Result<(), Error> {
    if value.is_empty()
        || value.len() > max_bytes
        || value.chars().any(char::is_control)
        || value.trim() != value
    {
        return Err(invalid(DocumentError::InvalidString));
    }
    Ok(())
}

fn validate_collection<T>(values: &[T]) -> Result<(), Error> {
    if values.is_empty() {
        return Err(invalid(DocumentError::EmptyValue));
    }
    if values.len() > MAX_DOCUMENT_ITEMS {
        return Err(invalid(DocumentError::TooManyItems));
    }
    Ok(())
}

const fn invalid(reason: DocumentError) -> Error {
    Error::InvalidDocument(reason)
}
