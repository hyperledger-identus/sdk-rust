//! Safe, chain-neutral DID Registration lifecycle primitives and port.
//!
//! This module deliberately models an internal Rust profile rather than the
//! experimental DIF DID Registration JSON/HTTP representation. It owns bounded
//! requests, jobs and results; method adapters retain custody, persistence,
//! signing, ledger execution and finality policy.

use std::{collections::BTreeMap, fmt, future::Future, pin::Pin};

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use serde_json::Value;

use crate::{Did, DidDocument, DidDocumentMetadata, DidMethod, Error, error::RegistrationError};

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

const MAX_PROPERTIES: usize = 64;
const MAX_PROPERTY_NAME_BYTES: usize = 256;
const MAX_DEPTH: usize = 32;
const MAX_NODES: usize = 4_096;
const MAX_STRING_BYTES: usize = 64 * 1_024;
const PRIVATE_MEMBER_NAMES: &[&str] = &[
    "secret",
    "secrets",
    "privatekey",
    "privatekeyjwk",
    "privatekeymultibase",
    "seed",
    "password",
    "passphrase",
    "mnemonic",
    "decryptedpayload",
    "plaintext",
];
const PRIVATE_JWK_MEMBERS: &[&str] = &["d", "p", "q", "dp", "dq", "qi", "oth", "k"];
const RESERVED_MEMBER_NAMES: &[&str] = &[
    "jobid",
    "didstate",
    "didregistrationmetadata",
    "diddocumentmetadata",
];

macro_rules! opaque_identifier {
    ($(#[$meta:meta])* $name:ident, $limit:expr) => {
        $(#[$meta])*
        #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(String);

        impl $name {
            /// Validate and retain an owned identifier.
            pub fn try_new(value: String) -> Result<Self, Error> {
                validate_identifier(&value, $limit)?;
                Ok(Self(value))
            }

            /// Parse and retain a borrowed identifier.
            pub fn parse(value: &str) -> Result<Self, Error> {
                validate_identifier(value, $limit)?;
                Ok(Self(value.to_owned()))
            }

            /// Borrow the validated identifier.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Consume the value and return its string representation.
            #[must_use]
            pub fn into_string(self) -> String {
                self.0
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_tuple(stringify!($name)).field(&"<redacted>").finish()
            }
        }
    };
}

opaque_identifier!(
    /// A retry key that identifies one canonical mutation request.
    RegistrationIdempotencyKey,
    MAX_REGISTRATION_ID_BYTES
);
opaque_identifier!(
    /// An opaque, method-owned identifier for a non-terminal registration job.
    RegistrationJobId,
    MAX_REGISTRATION_OPAQUE_ID_BYTES
);
opaque_identifier!(
    /// A bounded identifier correlating one client-managed action.
    RegistrationActionId,
    MAX_REGISTRATION_ID_BYTES
);
opaque_identifier!(
    /// An opaque reference into an adapter-owned custody system.
    RegistrationSecretHandle,
    MAX_REGISTRATION_OPAQUE_ID_BYTES
);
opaque_identifier!(
    /// A bounded method-specific update or action name.
    RegistrationOperationName,
    MAX_REGISTRATION_ID_BYTES
);
opaque_identifier!(
    /// A bounded, stable standard or method-defined registration failure code.
    RegistrationFailureCode,
    MAX_REGISTRATION_ID_BYTES
);

/// Stable generic registration failure classifications.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum DidRegistrationErrorKind {
    /// The exact DID method is not registered.
    MethodNotSupported,
    /// The method is known but has no registration capability.
    FeatureNotSupported,
    /// One idempotency key was reused with different canonical input.
    Conflict,
    /// An explicit cancellation completed before irreversible work.
    Cancelled,
    /// Registration failed unexpectedly.
    InternalError,
}

impl DidRegistrationErrorKind {
    /// Return the stable failure code.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MethodNotSupported => "methodNotSupported",
            Self::FeatureNotSupported => "featureNotSupported",
            Self::Conflict => "conflict",
            Self::Cancelled => "cancelled",
            Self::InternalError => "internalError",
        }
    }
}

impl RegistrationFailureCode {
    /// Construct one infallible standard failure code.
    #[must_use]
    pub fn standard(kind: DidRegistrationErrorKind) -> Self {
        Self(kind.as_str().to_owned())
    }

    /// Classify this code when it is one of the SDK standard values.
    #[must_use]
    pub fn kind(&self) -> Option<DidRegistrationErrorKind> {
        Some(match self.as_str() {
            "methodNotSupported" => DidRegistrationErrorKind::MethodNotSupported,
            "featureNotSupported" => DidRegistrationErrorKind::FeatureNotSupported,
            "conflict" => DidRegistrationErrorKind::Conflict,
            "cancelled" => DidRegistrationErrorKind::Cancelled,
            "internalError" => DidRegistrationErrorKind::InternalError,
            _ => return None,
        })
    }
}

/// Bounded JSON object containing public protocol data only.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct RegistrationPublicData(BTreeMap<String, Value>);

impl RegistrationPublicData {
    /// Validate a native public-data map.
    pub fn new(values: BTreeMap<String, Value>) -> Result<Self, Error> {
        let mut budget = JsonBudget::default();
        validate_map(&values, 1, &mut budget)?;
        Ok(Self(values))
    }

    /// Return an empty public-data object.
    #[must_use]
    pub const fn empty() -> Self {
        Self(BTreeMap::new())
    }

    /// Parse a bounded public-data JSON object.
    pub fn from_json_slice(input: &[u8]) -> Result<Self, Error> {
        if input.len() > MAX_DID_REGISTRATION_BYTES {
            return Err(invalid(RegistrationError::TooLarge));
        }
        let values =
            serde_json::from_slice(input).map_err(|_| invalid(RegistrationError::MalformedJson))?;
        Self::new(values)
    }

    /// Parse a bounded public-data JSON object string.
    pub fn from_json_str(input: &str) -> Result<Self, Error> {
        Self::from_json_slice(input.as_bytes())
    }

    /// Borrow the validated object.
    #[must_use]
    pub const fn as_map(&self) -> &BTreeMap<String, Value> {
        &self.0
    }

    /// Consume the value and return the validated object.
    #[must_use]
    pub fn into_map(self) -> BTreeMap<String, Value> {
        self.0
    }
}

impl fmt::Debug for RegistrationPublicData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RegistrationPublicData")
            .field("property_count", &self.0.len())
            .finish()
    }
}

impl Serialize for RegistrationPublicData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RegistrationPublicData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let values = BTreeMap::<String, Value>::deserialize(deserializer)?;
        Self::new(values).map_err(de::Error::custom)
    }
}

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

    fn subject(&self) -> Option<&Did> {
        match self {
            Self::Create(request) => request.requested_did(),
            Self::Update(request) => Some(request.did()),
            Self::Deactivate(request) => Some(request.did()),
            Self::Continue(_) | Self::Cancel(_) => None,
        }
    }

    fn job(&self) -> Option<&RegistrationJob> {
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

/// One validated registration lifecycle state.
#[derive(Clone, PartialEq, Eq)]
pub enum DidRegistrationState {
    /// The operation reached a terminal outcome.
    Finished {
        /// The created or affected DID.
        did: Did,
        /// Optional validated document visible at this finality point.
        document: Option<Box<DidDocument>>,
        /// Optional opaque handles to adapter-custodied material.
        secret_handles: Vec<RegistrationSecretHandle>,
    },
    /// The operation reached a terminal failure.
    Failed {
        /// The affected DID, when it is known.
        did: Option<Did>,
        /// Stable standard or method-defined failure code.
        code: RegistrationFailureCode,
    },
    /// The caller must perform one public action.
    Action {
        /// The affected DID, when assigned.
        did: Option<Did>,
        /// The exact outstanding action.
        action: RegistrationAction,
    },
    /// The caller may poll after an optional advisory duration.
    Wait {
        /// The affected DID, when assigned.
        did: Option<Did>,
        /// Advisory wait only; the SDK never sleeps automatically.
        retry_after_millis: Option<u64>,
    },
}

impl DidRegistrationState {
    /// Borrow the DID carried by any lifecycle state.
    #[must_use]
    pub const fn did(&self) -> Option<&Did> {
        match self {
            Self::Finished { did, .. } => Some(did),
            Self::Failed { did, .. } | Self::Action { did, .. } | Self::Wait { did, .. } => {
                did.as_ref()
            }
        }
    }

    /// Whether this is a terminal state.
    #[must_use]
    pub const fn is_terminal(&self) -> bool {
        matches!(self, Self::Finished { .. } | Self::Failed { .. })
    }
}

impl fmt::Debug for DidRegistrationState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let variant = match self {
            Self::Finished { .. } => "Finished",
            Self::Failed { .. } => "Failed",
            Self::Action { .. } => "Action",
            Self::Wait { .. } => "Wait",
        };
        f.debug_struct("DidRegistrationState")
            .field("variant", &variant)
            .field("did_present", &self.did().is_some())
            .field("payload", &"<redacted>")
            .finish()
    }
}

/// A validated terminal or continuing DID Registration outcome.
#[derive(Clone, PartialEq, Eq)]
pub struct DidRegistrationResult {
    method: DidMethod,
    job: Option<RegistrationJob>,
    state: DidRegistrationState,
    registration_metadata: RegistrationPublicData,
    document_metadata: DidDocumentMetadata,
}

impl DidRegistrationResult {
    /// Construct a result while enforcing job, state and method invariants.
    pub fn new(
        method: DidMethod,
        job: Option<RegistrationJob>,
        state: DidRegistrationState,
        registration_metadata: RegistrationPublicData,
        document_metadata: DidDocumentMetadata,
    ) -> Result<Self, Error> {
        validate_result(&method, job.as_ref(), &state, &document_metadata)?;
        Ok(Self {
            method,
            job,
            state,
            registration_metadata,
            document_metadata,
        })
    }

    /// Construct a valid terminal standard failure with empty metadata.
    #[must_use]
    pub fn standard_failure(method: DidMethod, kind: DidRegistrationErrorKind) -> Self {
        Self {
            method,
            job: None,
            state: DidRegistrationState::Failed {
                did: None,
                code: RegistrationFailureCode::standard(kind),
            },
            registration_metadata: RegistrationPublicData::empty(),
            document_metadata: DidDocumentMetadata::empty(),
        }
    }

    /// Borrow the exact method that produced this result.
    #[must_use]
    pub const fn method(&self) -> &DidMethod {
        &self.method
    }

    /// Borrow the required non-terminal job, when present.
    #[must_use]
    pub const fn job(&self) -> Option<&RegistrationJob> {
        self.job.as_ref()
    }

    /// Borrow the lifecycle state.
    #[must_use]
    pub const fn state(&self) -> &DidRegistrationState {
        &self.state
    }

    /// Borrow public operation metadata.
    #[must_use]
    pub const fn registration_metadata(&self) -> &RegistrationPublicData {
        &self.registration_metadata
    }

    /// Borrow validated DID document metadata.
    #[must_use]
    pub const fn document_metadata(&self) -> &DidDocumentMetadata {
        &self.document_metadata
    }

    /// Validate this result against the exact request identity and job.
    pub fn validate_for_request(&self, request: &RegistrationRequest) -> Result<(), Error> {
        if self.method != *request.method() {
            return Err(invalid(RegistrationError::MethodOrDidMismatch));
        }
        if request
            .subject()
            .zip(self.state.did())
            .is_some_and(|(expected, actual)| expected != actual)
        {
            return Err(invalid(RegistrationError::MethodOrDidMismatch));
        }
        if request.subject().is_some()
            && matches!(self.state, DidRegistrationState::Finished { .. })
            && self.state.did() != request.subject()
        {
            return Err(invalid(RegistrationError::MethodOrDidMismatch));
        }
        if request
            .job()
            .zip(self.job())
            .is_some_and(|(expected, actual)| {
                expected.method() != actual.method() || expected.id() != actual.id()
            })
        {
            return Err(invalid(RegistrationError::JobMismatch));
        }
        Ok(())
    }
}

impl fmt::Debug for DidRegistrationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DidRegistrationResult")
            .field("method", &self.method)
            .field("has_job", &self.job.is_some())
            .field("state", &self.state)
            .field("registration_metadata", &self.registration_metadata)
            .field(
                "document_metadata_present",
                &!self.document_metadata.is_empty(),
            )
            .finish()
    }
}

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

fn validate_result(
    method: &DidMethod,
    job: Option<&RegistrationJob>,
    state: &DidRegistrationState,
    document_metadata: &DidDocumentMetadata,
) -> Result<(), Error> {
    if job.is_some_and(|job| job.method() != method) {
        return Err(invalid(RegistrationError::JobMismatch));
    }
    match (state, job) {
        (
            DidRegistrationState::Finished {
                did,
                document,
                secret_handles,
            },
            None,
        ) => {
            if secret_handles.len() > MAX_REGISTRATION_ITEMS {
                return Err(invalid(RegistrationError::TooManyItems));
            }
            if did.method() != method.as_str()
                || document
                    .as_ref()
                    .is_some_and(|document| document.id() != did)
            {
                return Err(invalid(RegistrationError::MethodOrDidMismatch));
            }
            if let Some(document) = document {
                validate_public_document(document)?;
            }
        }
        (DidRegistrationState::Failed { did, .. }, None) => {
            if did
                .as_ref()
                .is_some_and(|did| did.method() != method.as_str())
            {
                return Err(invalid(RegistrationError::MethodOrDidMismatch));
            }
        }
        (DidRegistrationState::Action { did, action }, Some(job)) => {
            if did
                .as_ref()
                .is_some_and(|did| did.method() != method.as_str())
                || !matches!(job.continuation(), RegistrationContinuation::Action(id) if id == action.id())
            {
                return Err(invalid(RegistrationError::ActionMismatch));
            }
        }
        (
            DidRegistrationState::Wait {
                did,
                retry_after_millis,
            },
            Some(job),
        ) => {
            if did
                .as_ref()
                .is_some_and(|did| did.method() != method.as_str())
                || !matches!(job.continuation(), RegistrationContinuation::Wait)
                || retry_after_millis.is_some_and(|value| value > MAX_REGISTRATION_WAIT_MILLIS)
            {
                return Err(invalid(RegistrationError::InvalidState));
            }
        }
        _ => return Err(invalid(RegistrationError::InvalidState)),
    }
    if let Some(did) = state.did()
        && document_metadata.validate_for(did).is_err()
    {
        return Err(invalid(RegistrationError::MethodOrDidMismatch));
    }
    RegistrationPublicData::new(document_metadata.extensions().clone())?;
    Ok(())
}

fn validate_public_document(document: &DidDocument) -> Result<(), Error> {
    let value =
        serde_json::to_value(document).map_err(|_| invalid(RegistrationError::MalformedJson))?;
    let Value::Object(map) = value else {
        return Err(invalid(RegistrationError::MalformedJson));
    };
    validate_object(&map, 1, &mut JsonBudget::default())
}

fn validate_identifier(value: &str, max_bytes: usize) -> Result<(), Error> {
    if value.is_empty()
        || value.len() > max_bytes
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        return Err(invalid(RegistrationError::InvalidString));
    }
    Ok(())
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

#[derive(Default)]
struct JsonBudget {
    nodes: usize,
}

impl JsonBudget {
    fn visit(&mut self) -> Result<(), Error> {
        self.nodes += 1;
        if self.nodes > MAX_NODES {
            return Err(invalid(RegistrationError::TooManyNodes));
        }
        Ok(())
    }
}

fn validate_map(
    map: &BTreeMap<String, Value>,
    depth: usize,
    budget: &mut JsonBudget,
) -> Result<(), Error> {
    budget.visit()?;
    if map.len() > MAX_PROPERTIES {
        return Err(invalid(RegistrationError::TooManyProperties));
    }
    validate_object_members(
        map.iter().map(|(key, value)| (key.as_str(), value)),
        depth,
        budget,
    )
}

fn validate_object(
    map: &serde_json::Map<String, Value>,
    depth: usize,
    budget: &mut JsonBudget,
) -> Result<(), Error> {
    budget.visit()?;
    if map.len() > MAX_PROPERTIES {
        return Err(invalid(RegistrationError::TooManyProperties));
    }
    validate_object_members(
        map.iter().map(|(key, value)| (key.as_str(), value)),
        depth,
        budget,
    )
}

fn validate_object_members<'a>(
    members: impl Iterator<Item = (&'a str, &'a Value)> + Clone,
    depth: usize,
    budget: &mut JsonBudget,
) -> Result<(), Error> {
    let is_jwk = members.clone().any(|(key, _)| key == "kty");
    for (key, value) in members {
        if key.is_empty()
            || key.len() > MAX_PROPERTY_NAME_BYTES
            || key.trim() != key
            || key.chars().any(char::is_control)
        {
            return Err(invalid(RegistrationError::InvalidPropertyName));
        }
        let normalized: String = key
            .chars()
            .filter(|character| !matches!(character, '-' | '_' | '.'))
            .flat_map(char::to_lowercase)
            .collect();
        if PRIVATE_MEMBER_NAMES.contains(&normalized.as_str())
            || RESERVED_MEMBER_NAMES.contains(&normalized.as_str())
            || normalized.contains("privatekey")
            || matches!(normalized.as_str(), "secretkey" | "recoveryphrase")
            || (is_jwk && PRIVATE_JWK_MEMBERS.contains(&key))
        {
            return Err(invalid(
                if RESERVED_MEMBER_NAMES.contains(&normalized.as_str()) {
                    RegistrationError::ReservedProperty
                } else {
                    RegistrationError::PrivateMaterial
                },
            ));
        }
        validate_value(value, depth + 1, budget)?;
    }
    Ok(())
}

fn validate_value(value: &Value, depth: usize, budget: &mut JsonBudget) -> Result<(), Error> {
    if depth > MAX_DEPTH {
        return Err(invalid(RegistrationError::TooDeep));
    }
    match value {
        Value::String(value)
            if value.len() > MAX_STRING_BYTES || value.chars().any(char::is_control) =>
        {
            Err(invalid(RegistrationError::InvalidString))
        }
        Value::Array(values) => {
            budget.visit()?;
            if values.len() > MAX_REGISTRATION_ITEMS {
                return Err(invalid(RegistrationError::TooManyItems));
            }
            for value in values {
                validate_value(value, depth + 1, budget)?;
            }
            Ok(())
        }
        Value::Object(map) => validate_object(map, depth, budget),
        _ => budget.visit(),
    }
}

const fn invalid(reason: RegistrationError) -> Error {
    Error::InvalidRegistration(reason)
}
