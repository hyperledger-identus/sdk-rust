use std::fmt;

use crate::{Did, DidDocument, DidDocumentMetadata, DidMethod, Error, error::RegistrationError};

use super::{
    MAX_REGISTRATION_ITEMS, MAX_REGISTRATION_WAIT_MILLIS,
    identifier::{DidRegistrationErrorKind, RegistrationFailureCode, RegistrationSecretHandle},
    public_data::{RegistrationPublicData, validate_public_document},
    request::{RegistrationAction, RegistrationContinuation, RegistrationJob, RegistrationRequest},
};

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
fn validate_result(
    method: &DidMethod,
    job: Option<&RegistrationJob>,
    state: &DidRegistrationState,
    document_metadata: &DidDocumentMetadata,
) -> Result<(), Error> {
    RegistrationResultValidator {
        method,
        job,
        state,
        document_metadata,
    }
    .validate()
}

struct RegistrationResultValidator<'a> {
    method: &'a DidMethod,
    job: Option<&'a RegistrationJob>,
    state: &'a DidRegistrationState,
    document_metadata: &'a DidDocumentMetadata,
}

impl RegistrationResultValidator<'_> {
    fn validate(&self) -> Result<(), Error> {
        self.validate_job_method()?;
        self.validate_state()?;
        self.validate_document_metadata()
    }

    fn validate_job_method(&self) -> Result<(), Error> {
        if self.job.is_some_and(|job| job.method() != self.method) {
            return Err(invalid(RegistrationError::JobMismatch));
        }
        Ok(())
    }

    fn validate_state(&self) -> Result<(), Error> {
        match (self.state, self.job) {
            (
                DidRegistrationState::Finished {
                    did,
                    document,
                    secret_handles,
                },
                None,
            ) => self.validate_finished(did, document.as_deref(), secret_handles),
            (DidRegistrationState::Failed { did, .. }, None) => self.validate_failed(did.as_ref()),
            (DidRegistrationState::Action { did, action }, Some(job)) => {
                self.validate_action(did.as_ref(), action, job)
            }
            (
                DidRegistrationState::Wait {
                    did,
                    retry_after_millis,
                },
                Some(job),
            ) => self.validate_wait(did.as_ref(), *retry_after_millis, job),
            _ => Err(invalid(RegistrationError::InvalidState)),
        }
    }

    fn validate_finished(
        &self,
        did: &Did,
        document: Option<&DidDocument>,
        secret_handles: &[RegistrationSecretHandle],
    ) -> Result<(), Error> {
        if secret_handles.len() > MAX_REGISTRATION_ITEMS {
            return Err(invalid(RegistrationError::TooManyItems));
        }
        if did.method() != self.method.as_str()
            || document.is_some_and(|document| document.id() != did)
        {
            return Err(invalid(RegistrationError::MethodOrDidMismatch));
        }
        if let Some(document) = document {
            validate_public_document(document)?;
        }
        Ok(())
    }

    fn validate_failed(&self, did: Option<&Did>) -> Result<(), Error> {
        if did.is_some_and(|did| did.method() != self.method.as_str()) {
            return Err(invalid(RegistrationError::MethodOrDidMismatch));
        }
        Ok(())
    }

    fn validate_action(
        &self,
        did: Option<&Did>,
        action: &RegistrationAction,
        job: &RegistrationJob,
    ) -> Result<(), Error> {
        if did.is_some_and(|did| did.method() != self.method.as_str())
            || !matches!(job.continuation(), RegistrationContinuation::Action(id) if id == action.id())
        {
            return Err(invalid(RegistrationError::ActionMismatch));
        }
        Ok(())
    }

    fn validate_wait(
        &self,
        did: Option<&Did>,
        retry_after_millis: Option<u64>,
        job: &RegistrationJob,
    ) -> Result<(), Error> {
        if did.is_some_and(|did| did.method() != self.method.as_str())
            || !matches!(job.continuation(), RegistrationContinuation::Wait)
            || retry_after_millis.is_some_and(|value| value > MAX_REGISTRATION_WAIT_MILLIS)
        {
            return Err(invalid(RegistrationError::InvalidState));
        }
        Ok(())
    }

    fn validate_document_metadata(&self) -> Result<(), Error> {
        if let Some(did) = self.state.did()
            && self.document_metadata.validate_for(did).is_err()
        {
            return Err(invalid(RegistrationError::MethodOrDidMismatch));
        }
        RegistrationPublicData::new(self.document_metadata.extensions().clone())?;
        Ok(())
    }
}

const fn invalid(reason: RegistrationError) -> Error {
    Error::InvalidRegistration(reason)
}
