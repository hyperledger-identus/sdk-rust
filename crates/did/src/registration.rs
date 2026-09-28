//! Safe, chain-neutral DID Registration lifecycle primitives and port.
//!
//! This module deliberately models an internal Rust profile rather than the
//! experimental DIF DID Registration JSON/HTTP representation. It owns bounded
//! requests, jobs and results; method adapters retain custody, persistence,
//! signing, ledger execution and finality policy.

mod identifier;
mod port;
mod public_data;
mod request;
mod result;

pub use identifier::{
    DidRegistrationErrorKind, RegistrationActionId, RegistrationFailureCode,
    RegistrationIdempotencyKey, RegistrationJobId, RegistrationOperationName,
    RegistrationSecretHandle,
};
pub use port::{DidRegistrar, DidRegistrationFuture};
pub use public_data::RegistrationPublicData;
pub use request::{
    CancelRegistrationRequest, ContinueRegistrationRequest, CreateRegistrationRequest,
    DeactivateRegistrationRequest, DidDocumentOperation, InternalSecretPolicy, RegistrationAction,
    RegistrationActionResponse, RegistrationContinuation, RegistrationJob, RegistrationRequest,
    RegistrationSecretMode, UpdateRegistrationRequest,
};
pub use result::{DidRegistrationResult, DidRegistrationState};

/// Maximum raw JSON bytes accepted by [`RegistrationPublicData`].
pub const MAX_DID_REGISTRATION_BYTES: usize = 512 * 1_024;
/// Maximum operations in one update request and handles in one result.
pub const MAX_REGISTRATION_ITEMS: usize = 128;
/// Maximum bytes in an idempotency key or action identifier.
pub const MAX_REGISTRATION_ID_BYTES: usize = 256;
/// Maximum bytes in an opaque job or custody handle.
pub const MAX_REGISTRATION_OPAQUE_ID_BYTES: usize = 1_024;
/// Maximum advisory wait duration in milliseconds.
pub const MAX_REGISTRATION_WAIT_MILLIS: u64 = 86_400_000;
