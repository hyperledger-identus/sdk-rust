use std::fmt;

use crate::{Did, DidDocument, DidMethod, Error, error::RegistrationError};

use super::{
    MAX_REGISTRATION_ITEMS,
    identifier::{
        RegistrationActionId, RegistrationIdempotencyKey, RegistrationJobId,
        RegistrationOperationName, RegistrationSecretHandle,
    },
    public_data::{RegistrationPublicData, validate_public_document},
};

/// Policy for adapter-internal key generation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InternalSecretPolicy {
    store_generated: bool,
    return_handle: bool,
}

impl InternalSecretPolicy {
    /// Construct a policy that retains generated capability in at least one way.
    pub fn new(store_generated: bool, return_handle: bool) -> Result<Self, Error> {
        if !store_generated && !return_handle {
            return Err(invalid(RegistrationError::InvalidSecretPolicy));
        }
        Ok(Self {
            store_generated,
            return_handle,
        })
    }

    /// Whether the adapter should store generated private material.
    #[must_use]
    pub const fn store_generated(self) -> bool {
        self.store_generated
    }

    /// Whether the adapter should return an opaque custody handle.
    #[must_use]
    pub const fn return_handle(self) -> bool {
        self.return_handle
    }
}

/// The private-material ownership mode for one registration request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RegistrationSecretMode {
    /// The adapter generates private material under an explicit retention policy.
    Internal(InternalSecretPolicy),
    /// The adapter uses an existing opaque custody reference.
    External(RegistrationSecretHandle),
    /// The caller completes bounded public signing or proof actions.
    ClientManaged,
}

/// One ordered mutation of a DID document.
#[derive(Clone, PartialEq, Eq)]
pub enum DidDocumentOperation {
    /// Replace the complete DID document with a validated document.
    SetDidDocument(Box<DidDocument>),
    /// Add method-interpreted public document members.
    AddToDidDocument(RegistrationPublicData),
    /// Remove method-interpreted public document members.
    RemoveFromDidDocument(RegistrationPublicData),
    /// Preserve one bounded method-specific public operation.
    MethodSpecific {
        /// Method-defined operation name.
        name: RegistrationOperationName,
        /// Optional public method data.
        data: Option<RegistrationPublicData>,
    },
}

impl fmt::Debug for DidDocumentOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::SetDidDocument(_) => "SetDidDocument(<redacted>)",
            Self::AddToDidDocument(_) => "AddToDidDocument(<redacted>)",
            Self::RemoveFromDidDocument(_) => "RemoveFromDidDocument(<redacted>)",
            Self::MethodSpecific { .. } => "MethodSpecific(<redacted>)",
        })
    }
}

/// Public work the caller must perform before continuing a job.
#[derive(Clone, PartialEq, Eq)]
pub struct RegistrationAction {
    id: RegistrationActionId,
    name: RegistrationOperationName,
    data: RegistrationPublicData,
}

impl RegistrationAction {
    /// Construct a bounded public action.
    #[must_use]
    pub const fn new(
        id: RegistrationActionId,
        name: RegistrationOperationName,
        data: RegistrationPublicData,
    ) -> Self {
        Self { id, name, data }
    }

    /// Borrow the exact action identifier.
    #[must_use]
    pub const fn id(&self) -> &RegistrationActionId {
        &self.id
    }

    /// Borrow the method-defined action name.
    #[must_use]
    pub const fn name(&self) -> &RegistrationOperationName {
        &self.name
    }

    /// Borrow the public action data.
    #[must_use]
    pub const fn data(&self) -> &RegistrationPublicData {
        &self.data
    }
}

impl fmt::Debug for RegistrationAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RegistrationAction")
            .field("id", &"<redacted>")
            .field("name", &"<redacted>")
            .field("data", &self.data)
            .finish()
    }
}

/// Public response to exactly one outstanding client-managed action.
#[derive(Clone, PartialEq, Eq)]
pub struct RegistrationActionResponse {
    id: RegistrationActionId,
    data: RegistrationPublicData,
}

impl RegistrationActionResponse {
    /// Construct a response correlated by its exact action identifier.
    #[must_use]
    pub const fn new(id: RegistrationActionId, data: RegistrationPublicData) -> Self {
        Self { id, data }
    }

    /// Borrow the action identifier.
    #[must_use]
    pub const fn id(&self) -> &RegistrationActionId {
        &self.id
    }

    /// Borrow the public response data.
    #[must_use]
    pub const fn data(&self) -> &RegistrationPublicData {
        &self.data
    }
}

impl fmt::Debug for RegistrationActionResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RegistrationActionResponse")
            .field("id", &"<redacted>")
            .field("data", &self.data)
            .finish()
    }
}

/// The only valid way to continue a non-terminal job.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RegistrationContinuation {
    /// Poll the method adapter without inventing a response.
    Wait,
    /// Respond to this exact outstanding action.
    Action(RegistrationActionId),
}

/// A method-scoped opaque non-terminal registration job.
#[derive(Clone, PartialEq, Eq)]
pub struct RegistrationJob {
    method: DidMethod,
    id: RegistrationJobId,
    continuation: RegistrationContinuation,
}

impl RegistrationJob {
    /// Construct a method-scoped job.
    #[must_use]
    pub const fn new(
        method: DidMethod,
        id: RegistrationJobId,
        continuation: RegistrationContinuation,
    ) -> Self {
        Self {
            method,
            id,
            continuation,
        }
    }

    /// Borrow the exact method used for registry dispatch.
    #[must_use]
    pub const fn method(&self) -> &DidMethod {
        &self.method
    }

    /// Borrow the opaque method-owned job identifier.
    #[must_use]
    pub const fn id(&self) -> &RegistrationJobId {
        &self.id
    }

    /// Borrow the required continuation kind.
    #[must_use]
    pub const fn continuation(&self) -> &RegistrationContinuation {
        &self.continuation
    }
}

impl fmt::Debug for RegistrationJob {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let continuation = match self.continuation {
            RegistrationContinuation::Wait => "wait",
            RegistrationContinuation::Action(_) => "action",
        };
        f.debug_struct("RegistrationJob")
            .field("method", &self.method)
            .field("id", &"<redacted>")
            .field("continuation", &continuation)
            .finish()
    }
}

/// Immutable input for DID creation.
#[derive(Clone, PartialEq, Eq)]
pub struct CreateRegistrationRequest {
    method: DidMethod,
    requested_did: Option<Did>,
    document: Option<Box<DidDocument>>,
    options: RegistrationPublicData,
    secret_mode: RegistrationSecretMode,
    idempotency_key: RegistrationIdempotencyKey,
}

impl CreateRegistrationRequest {
    /// Construct a creation request and enforce method identity.
    pub fn new(
        method: DidMethod,
        requested_did: Option<Did>,
        document: Option<DidDocument>,
        options: RegistrationPublicData,
        secret_mode: RegistrationSecretMode,
        idempotency_key: RegistrationIdempotencyKey,
    ) -> Result<Self, Error> {
        if requested_did
            .as_ref()
            .is_some_and(|did| did.method() != method.as_str())
            || document
                .as_ref()
                .is_some_and(|document| document.id().method() != method.as_str())
            || requested_did
                .as_ref()
                .zip(document.as_ref())
                .is_some_and(|(did, document)| did != document.id())
        {
            return Err(invalid(RegistrationError::MethodOrDidMismatch));
        }
        if let Some(document) = &document {
            validate_public_document(document)?;
        }
        Ok(Self {
            method,
            requested_did,
            document: document.map(Box::new),
            options,
            secret_mode,
            idempotency_key,
        })
    }

    /// Borrow the requested DID method.
    #[must_use]
    pub const fn method(&self) -> &DidMethod {
        &self.method
    }

    /// Borrow the optional caller-requested DID.
    #[must_use]
    pub const fn requested_did(&self) -> Option<&Did> {
        self.requested_did.as_ref()
    }

    /// Borrow the optional initial DID document.
    #[must_use]
    pub fn document(&self) -> Option<&DidDocument> {
        self.document.as_deref()
    }

    /// Borrow public method creation options.
    #[must_use]
    pub const fn options(&self) -> &RegistrationPublicData {
        &self.options
    }

    /// Borrow the private-material ownership mode.
    #[must_use]
    pub const fn secret_mode(&self) -> &RegistrationSecretMode {
        &self.secret_mode
    }

    /// Borrow the required idempotency key.
    #[must_use]
    pub const fn idempotency_key(&self) -> &RegistrationIdempotencyKey {
        &self.idempotency_key
    }
}

/// Immutable input for an ordered DID document update.
#[derive(Clone, PartialEq, Eq)]
pub struct UpdateRegistrationRequest {
    method: DidMethod,
    did: Did,
    operations: Vec<DidDocumentOperation>,
    options: RegistrationPublicData,
    secret_mode: RegistrationSecretMode,
    idempotency_key: RegistrationIdempotencyKey,
}

impl UpdateRegistrationRequest {
    /// Construct a bounded non-empty ordered update.
    pub fn new(
        did: Did,
        operations: Vec<DidDocumentOperation>,
        options: RegistrationPublicData,
        secret_mode: RegistrationSecretMode,
        idempotency_key: RegistrationIdempotencyKey,
    ) -> Result<Self, Error> {
        validate_items(&operations)?;
        if operations.iter().any(|operation| {
            matches!(operation, DidDocumentOperation::SetDidDocument(document) if document.id() != &did)
        }) {
            return Err(invalid(RegistrationError::MethodOrDidMismatch));
        }
        for operation in &operations {
            if let DidDocumentOperation::SetDidDocument(document) = operation {
                validate_public_document(document)?;
            }
        }
        let method = DidMethod::parse(did.method())
            .map_err(|_| invalid(RegistrationError::MethodOrDidMismatch))?;
        Ok(Self {
            method,
            did,
            operations,
            options,
            secret_mode,
            idempotency_key,
        })
    }

    /// Borrow the exact validated method derived from the DID.
    #[must_use]
    pub const fn method(&self) -> &DidMethod {
        &self.method
    }

    /// Borrow the exact DID being updated.
    #[must_use]
    pub const fn did(&self) -> &Did {
        &self.did
    }

    /// Borrow the ordered operations without normalization.
    #[must_use]
    pub fn operations(&self) -> &[DidDocumentOperation] {
        &self.operations
    }

    /// Borrow public method update options.
    #[must_use]
    pub const fn options(&self) -> &RegistrationPublicData {
        &self.options
    }

    /// Borrow the private-material ownership mode.
    #[must_use]
    pub const fn secret_mode(&self) -> &RegistrationSecretMode {
        &self.secret_mode
    }

    /// Borrow the required idempotency key.
    #[must_use]
    pub const fn idempotency_key(&self) -> &RegistrationIdempotencyKey {
        &self.idempotency_key
    }
}

/// Immutable input for DID deactivation.
#[derive(Clone, PartialEq, Eq)]
pub struct DeactivateRegistrationRequest {
    method: DidMethod,
    did: Did,
    options: RegistrationPublicData,
    secret_mode: RegistrationSecretMode,
    idempotency_key: RegistrationIdempotencyKey,
}

impl DeactivateRegistrationRequest {
    /// Construct a DID deactivation request.
    pub fn new(
        did: Did,
        options: RegistrationPublicData,
        secret_mode: RegistrationSecretMode,
        idempotency_key: RegistrationIdempotencyKey,
    ) -> Result<Self, Error> {
        let method = DidMethod::parse(did.method())
            .map_err(|_| invalid(RegistrationError::MethodOrDidMismatch))?;
        Ok(Self {
            method,
            did,
            options,
            secret_mode,
            idempotency_key,
        })
    }

    /// Borrow the exact validated method derived from the DID.
    #[must_use]
    pub const fn method(&self) -> &DidMethod {
        &self.method
    }

    /// Borrow the exact DID being deactivated.
    #[must_use]
    pub const fn did(&self) -> &Did {
        &self.did
    }

    /// Borrow public method deactivation options.
    #[must_use]
    pub const fn options(&self) -> &RegistrationPublicData {
        &self.options
    }

    /// Borrow the private-material ownership mode.
    #[must_use]
    pub const fn secret_mode(&self) -> &RegistrationSecretMode {
        &self.secret_mode
    }

    /// Borrow the required idempotency key.
    #[must_use]
    pub const fn idempotency_key(&self) -> &RegistrationIdempotencyKey {
        &self.idempotency_key
    }
}

/// Immutable input for continuing a method-scoped job.
#[derive(Clone, PartialEq, Eq)]
pub struct ContinueRegistrationRequest {
    job: RegistrationJob,
    response: Option<RegistrationActionResponse>,
    idempotency_key: RegistrationIdempotencyKey,
}

impl ContinueRegistrationRequest {
    /// Construct an exact wait poll or action response.
    pub fn new(
        job: RegistrationJob,
        response: Option<RegistrationActionResponse>,
        idempotency_key: RegistrationIdempotencyKey,
    ) -> Result<Self, Error> {
        let valid = match (job.continuation(), response.as_ref()) {
            (RegistrationContinuation::Wait, None) => true,
            (RegistrationContinuation::Action(expected), Some(response)) => {
                expected == response.id()
            }
            _ => false,
        };
        if !valid {
            return Err(invalid(RegistrationError::ActionMismatch));
        }
        Ok(Self {
            job,
            response,
            idempotency_key,
        })
    }

    /// Borrow the exact job being continued.
    #[must_use]
    pub const fn job(&self) -> &RegistrationJob {
        &self.job
    }

    /// Borrow the correlated action response, when required.
    #[must_use]
    pub const fn response(&self) -> Option<&RegistrationActionResponse> {
        self.response.as_ref()
    }

    /// Borrow the required idempotency key.
    #[must_use]
    pub const fn idempotency_key(&self) -> &RegistrationIdempotencyKey {
        &self.idempotency_key
    }
}

/// Immutable input for best-effort cancellation.
#[derive(Clone, PartialEq, Eq)]
pub struct CancelRegistrationRequest {
    job: RegistrationJob,
    idempotency_key: RegistrationIdempotencyKey,
}

impl CancelRegistrationRequest {
    /// Construct an explicit best-effort cancellation request.
    #[must_use]
    pub const fn new(job: RegistrationJob, idempotency_key: RegistrationIdempotencyKey) -> Self {
        Self {
            job,
            idempotency_key,
        }
    }

    /// Borrow the exact job being cancelled.
    #[must_use]
    pub const fn job(&self) -> &RegistrationJob {
        &self.job
    }

    /// Borrow the required idempotency key.
    #[must_use]
    pub const fn idempotency_key(&self) -> &RegistrationIdempotencyKey {
        &self.idempotency_key
    }
}

/// One immutable chain-neutral registration operation.
#[derive(Clone, PartialEq, Eq)]
pub enum RegistrationRequest {
    /// Create a DID.
    Create(CreateRegistrationRequest),
    /// Apply ordered operations to a DID.
    Update(UpdateRegistrationRequest),
    /// Deactivate a DID.
    Deactivate(DeactivateRegistrationRequest),
    /// Continue a non-terminal job.
    Continue(ContinueRegistrationRequest),
    /// Request best-effort cancellation.
    Cancel(CancelRegistrationRequest),
}

impl RegistrationRequest {
    /// Borrow the exact validated method used for dispatch.
    #[must_use]
    pub fn method(&self) -> &DidMethod {
        match self {
            Self::Create(request) => request.method(),
            Self::Update(request) => &request.method,
            Self::Deactivate(request) => &request.method,
            Self::Continue(request) => request.job().method(),
            Self::Cancel(request) => request.job().method(),
        }
    }

    /// Borrow the idempotency key common to every request variant.
    #[must_use]
    pub fn idempotency_key(&self) -> &RegistrationIdempotencyKey {
        match self {
            Self::Create(request) => request.idempotency_key(),
            Self::Update(request) => request.idempotency_key(),
            Self::Deactivate(request) => request.idempotency_key(),
            Self::Continue(request) => request.idempotency_key(),
            Self::Cancel(request) => request.idempotency_key(),
        }
    }

    pub(super) fn subject(&self) -> Option<&Did> {
        match self {
            Self::Create(request) => request.requested_did(),
            Self::Update(request) => Some(request.did()),
            Self::Deactivate(request) => Some(request.did()),
            Self::Continue(_) | Self::Cancel(_) => None,
        }
    }

    pub(super) fn job(&self) -> Option<&RegistrationJob> {
        match self {
            Self::Continue(request) => Some(request.job()),
            Self::Cancel(request) => Some(request.job()),
            _ => None,
        }
    }
}

impl fmt::Debug for RegistrationRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let variant = match self {
            Self::Create(_) => "Create",
            Self::Update(_) => "Update",
            Self::Deactivate(_) => "Deactivate",
            Self::Continue(_) => "Continue",
            Self::Cancel(_) => "Cancel",
        };
        f.debug_struct("RegistrationRequest")
            .field("variant", &variant)
            .field("method", self.method())
            .field("payload", &"<redacted>")
            .finish()
    }
}
fn validate_items<T>(values: &[T]) -> Result<(), Error> {
    if values.is_empty() {
        return Err(invalid(RegistrationError::EmptyValue));
    }
    if values.len() > MAX_REGISTRATION_ITEMS {
        return Err(invalid(RegistrationError::TooManyItems));
    }
    Ok(())
}
const fn invalid(reason: RegistrationError) -> Error {
    Error::InvalidRegistration(reason)
}
