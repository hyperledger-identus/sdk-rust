//! Transport-free W3C DID resolution and DID URL dereferencing results.
//!
//! This module owns bounded result data only. Resolution algorithms, DID
//! methods, network bindings, caching and trust policy belong in higher rings.
//! Use the explicit `from_json_*` result entry points for untrusted bytes: they
//! reject duplicate decoded names before typed deserialization. Direct serde is
//! a semantic conversion for representations whose unique-name property has
//! already been established.

mod dereferencing;
mod document_metadata;
mod operation;
mod result;
mod value;
mod wire;

pub use dereferencing::{DereferencedContent, DidUrlContentMetadata, DidUrlDereferencingResult};
pub use document_metadata::{DidDocumentMetadata, DidDocumentMetadataBuilder};
pub use operation::{
    DidResolutionError, DidResolutionErrorKind, DidResolutionMetadata, DidUrlDereferencingMetadata,
    MAX_PROBLEM_DETAIL_BYTES,
};
pub use result::DidResolutionResult;
pub(crate) use result::standard_resolution_failure;
pub use value::{
    DidResolutionDateTime, MAX_DID_RESOLUTION_DATETIME_BYTES, MAX_MEDIA_TYPE_BYTES,
    MAX_VERSION_ID_BYTES, MediaType, VersionId,
};
pub use wire::{
    MAX_DID_RESOLUTION_RESULT_BYTES, MAX_DID_RESOLUTION_WIRE_DEPTH,
    MAX_DID_RESOLUTION_WIRE_LIVE_KEY_BYTES, MAX_DID_RESOLUTION_WIRE_NODES,
    MAX_DID_RESOLUTION_WIRE_OBJECT_MEMBERS,
};
