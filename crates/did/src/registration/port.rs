use std::{future::Future, pin::Pin};

use super::{request::RegistrationRequest, result::DidRegistrationResult};

/// Boxed runtime-neutral future returned by [`DidRegistrar`].
pub type DidRegistrationFuture<'a> =
    Pin<Box<dyn Future<Output = DidRegistrationResult> + Send + 'a>>;

/// Object-safe port implemented by one DID method registration adapter.
///
/// Implementations must durably bind each idempotency key to canonical request
/// input. Dropping the future cancels observation only and never implies
/// rollback; callers use [`RegistrationRequest::Cancel`] explicitly.
pub trait DidRegistrar: Send + Sync {
    /// Execute, continue, poll, or request cancellation of one operation.
    fn execute<'a>(&'a self, request: &'a RegistrationRequest) -> DidRegistrationFuture<'a>;
}
