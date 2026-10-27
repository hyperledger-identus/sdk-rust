//! Portable, chain-neutral PRISM DID method primitives.
//!
//! The crate preserves exact signed operation bytes and owns only method
//! syntax, validation, state reduction and DID document projection. Ledger,
//! persistence, transport and node composition belong to outer adapters.

use identus_core::Component;

mod error;
mod limits;
mod operation;
mod projection;
// The pinned private schema includes operation variants consumed incrementally
// by this crate and the later VDR capability; generated message reachability is
// not a public API signal.
#[allow(dead_code)]
mod proto;
mod resolver;
mod state;
mod syntax;
mod wire;

pub use error::{Error, OperationError, StateError, SyntaxError, WireError};
pub use limits::{
    MAX_CONTEXTS, MAX_ID_CHARACTERS, MAX_LONG_FORM_DID_BYTES, MAX_OPERATION_BYTES,
    MAX_PROJECTED_DOCUMENT_BYTES, MAX_SERVICE_ENDPOINT_BYTES, MAX_SERVICE_ENDPOINT_CHARACTERS,
    MAX_SERVICES, MAX_SIGNATURE_BYTES, MAX_SIGNED_OPERATION_BYTES, MAX_SIGNED_WITH_BYTES,
    MAX_TYPE_CHARACTERS, MAX_VERIFICATION_METHODS,
};
pub use operation::{
    CreateDidOperation, DeactivateDidOperation, KeyUsage, PrismPublicKey, PrismPublicKeyMaterial,
    PrismService, ServiceEndpoint, ServiceEndpointValue, ServiceType, UpdateAction,
    UpdateDidOperation,
};
pub use projection::{project_did_document, project_did_document_for};
pub use resolver::{PrismDidHistory, PrismDidHistoryFuture, PrismDidResolver};
pub use state::PrismDidState;
pub use syntax::{CanonicalPrismDid, LongFormPrismDid, PrismDid};
pub use wire::{OperationKind, RawPrismOperation, SignedPrismOperation};

/// Metadata for the `identus-prism-did` crate.
pub const COMPONENT: Component = Component {
    name: "identus-prism-did",
    summary: "Portable PRISM DID method syntax, operations, state, and projection.",
};
