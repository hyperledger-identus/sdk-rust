//! Runtime-neutral credential verification execution and exact-format dispatch.
//!
//! Concrete format adapters own parsing, proof verification, DID resolution,
//! status and schema dependencies. This module owns only the least-authority
//! request, asynchronous port, operational failure vocabulary and immutable
//! composition registry.

use std::{collections::BTreeMap, error::Error, fmt, future::Future, pin::Pin, sync::Arc};

use identus_core::{ErrorCode, ErrorKind, IdentusError};
use identus_derive as identus;

use crate::{
    CredentialDetachedProof, CredentialEnvelope, CredentialError, CredentialFormat,
    CredentialPayload, VerificationReport, error::CAPABILITY,
};

/// Maximum number of exact format bindings in one immutable verifier registry.
pub const MAX_CREDENTIAL_VERIFIER_REGISTRY_ENTRIES: usize = 64;

const VERIFICATION_UNSUPPORTED_FORMAT: ErrorCode =
    ErrorCode::new("credential.verification_unsupported_format");
const VERIFICATION_UNAVAILABLE: ErrorCode = ErrorCode::new("credential.verification_unavailable");
const VERIFICATION_INTERNAL: ErrorCode = ErrorCode::new("credential.verification_internal");

/// A borrowed, least-authority view of credential verification inputs.
///
/// Holder-private material is deliberately absent. Constructing this view
/// copies references only; payload and detached-proof byte buffers remain
/// owned by the source [`CredentialEnvelope`].
#[derive(Clone, Copy)]
pub struct CredentialVerificationRequest<'a> {
    format: &'a CredentialFormat,
    payload: &'a CredentialPayload,
    detached_proof: Option<&'a CredentialDetachedProof>,
}

impl<'a> CredentialVerificationRequest<'a> {
    /// Borrow verification inputs from an existing credential envelope.
    #[must_use]
    pub fn from_envelope(envelope: &'a CredentialEnvelope) -> Self {
        Self {
            format: envelope.format(),
            payload: envelope.payload(),
            detached_proof: envelope.detached_proof(),
        }
    }

    /// Return the declared credential format used for exact adapter dispatch.
    pub const fn format(&self) -> &'a CredentialFormat {
        self.format
    }

    /// Return the exact encoded credential payload.
    pub const fn payload(&self) -> &'a CredentialPayload {
        self.payload
    }

    /// Return the format-owned detached proof when present.
    #[must_use]
    pub const fn detached_proof(&self) -> Option<&'a CredentialDetachedProof> {
        self.detached_proof
    }
}

impl<'a> From<&'a CredentialEnvelope> for CredentialVerificationRequest<'a> {
    fn from(envelope: &'a CredentialEnvelope) -> Self {
        Self::from_envelope(envelope)
    }
}

impl fmt::Debug for CredentialVerificationRequest<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialVerificationRequest")
            .field("format", self.format)
            .field("payload_length", &self.payload.as_bytes().len())
            .field(
                "detached_proof_length",
                &self.detached_proof.map(|proof| proof.as_bytes().len()),
            )
            .finish_non_exhaustive()
    }
}

/// Operational failure to execute credential verification.
///
/// Invalid or incomplete credential evidence is not an error here; a verifier
/// returns it as Failed or NotChecked stages in a [`VerificationReport`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum CredentialVerificationError {
    /// No verifier is bound to the exact declared credential format.
    UnsupportedFormat,
    /// An injected verifier dependency cannot currently execute.
    Unavailable,
    /// The adapter cannot safely produce a canonical verification report.
    Internal,
}

impl CredentialVerificationError {
    /// Bridge to the shared redaction-safe SDK error surface.
    pub const fn to_identus_error(self) -> IdentusError {
        match self {
            Self::UnsupportedFormat => IdentusError::public(
                VERIFICATION_UNSUPPORTED_FORMAT,
                ErrorKind::Unsupported,
                CAPABILITY,
                "credential verification format is unsupported",
            ),
            Self::Unavailable => IdentusError::public(
                VERIFICATION_UNAVAILABLE,
                ErrorKind::Internal,
                CAPABILITY,
                "credential verification is unavailable",
            ),
            Self::Internal => IdentusError::public(
                VERIFICATION_INTERNAL,
                ErrorKind::Internal,
                CAPABILITY,
                "credential verification failed internally",
            ),
        }
    }
}

impl fmt::Display for CredentialVerificationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::UnsupportedFormat => "credential verification format is unsupported",
            Self::Unavailable => "credential verification is unavailable",
            Self::Internal => "credential verification failed internally",
        })
    }
}

impl Error for CredentialVerificationError {}

/// Result of one credential-verifier invocation.
pub type CredentialVerificationResult = Result<VerificationReport, CredentialVerificationError>;

/// Type-erased asynchronous result returned by [`CredentialVerifier`].
pub type CredentialVerificationFuture<'a> =
    Pin<Box<dyn Future<Output = CredentialVerificationResult> + Send + 'a>>;

/// Runtime-neutral capability for one credential format's verification.
#[identus::port]
pub trait CredentialVerifier: Send + Sync {
    /// Verify borrowed credential evidence without applying product trust.
    fn verify<'a>(
        &'a self,
        request: CredentialVerificationRequest<'a>,
    ) -> CredentialVerificationFuture<'a>;
}

/// Mutable setup surface for an immutable [`CredentialVerifierRegistry`].
#[derive(Default)]
pub struct CredentialVerifierRegistryBuilder {
    verifiers: BTreeMap<CredentialFormat, Arc<dyn CredentialVerifier>>,
}

impl CredentialVerifierRegistryBuilder {
    /// Bind one exact validated format to its sole verifier.
    pub fn register(
        mut self,
        format: CredentialFormat,
        verifier: Arc<dyn CredentialVerifier>,
    ) -> Result<Self, CredentialError> {
        if self.verifiers.contains_key(&format) {
            return Err(CredentialError::DuplicateCredentialVerifierFormat);
        }
        if self.verifiers.len() >= MAX_CREDENTIAL_VERIFIER_REGISTRY_ENTRIES {
            return Err(CredentialError::TooManyCredentialVerifierFormats);
        }
        self.verifiers.insert(format, verifier);
        Ok(self)
    }

    /// Freeze registered verifier bindings into one shared immutable map.
    #[must_use]
    pub fn build(self) -> CredentialVerifierRegistry {
        CredentialVerifierRegistry {
            verifiers: Arc::new(self.verifiers),
        }
    }
}

impl fmt::Debug for CredentialVerifierRegistryBuilder {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialVerifierRegistryBuilder")
            .field(
                "formats",
                &self
                    .verifiers
                    .keys()
                    .map(CredentialFormat::as_str)
                    .collect::<Vec<_>>(),
            )
            .finish_non_exhaustive()
    }
}

/// Immutable deterministic dispatcher for credential-format verifiers.
#[derive(Clone, Default)]
pub struct CredentialVerifierRegistry {
    verifiers: Arc<BTreeMap<CredentialFormat, Arc<dyn CredentialVerifier>>>,
}

impl CredentialVerifierRegistry {
    /// Begin bounded registry construction.
    #[must_use]
    pub fn builder() -> CredentialVerifierRegistryBuilder {
        CredentialVerifierRegistryBuilder::default()
    }

    /// Return a valid registry with no verifier bindings.
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    /// Return the number of exact format bindings.
    #[must_use]
    pub fn format_count(&self) -> usize {
        self.verifiers.len()
    }

    /// Whether no credential format has a verifier binding.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.verifiers.is_empty()
    }

    /// Whether the exact validated format has a verifier binding.
    #[must_use]
    pub fn supports_format(&self, format: &CredentialFormat) -> bool {
        self.verifiers.contains_key(format)
    }

    /// Iterate bound format names in deterministic lexical order.
    pub fn formats(&self) -> impl ExactSizeIterator<Item = &str> {
        self.verifiers.keys().map(CredentialFormat::as_str)
    }
}

impl fmt::Debug for CredentialVerifierRegistry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialVerifierRegistry")
            .field(
                "formats",
                &self
                    .verifiers
                    .keys()
                    .map(CredentialFormat::as_str)
                    .collect::<Vec<_>>(),
            )
            .finish_non_exhaustive()
    }
}

impl CredentialVerifier for CredentialVerifierRegistry {
    fn verify<'a>(
        &'a self,
        request: CredentialVerificationRequest<'a>,
    ) -> CredentialVerificationFuture<'a> {
        self.verifiers.get(request.format()).map_or_else(
            || {
                Box::pin(async { Err(CredentialVerificationError::UnsupportedFormat) })
                    as CredentialVerificationFuture<'a>
            },
            |verifier| verifier.verify(request),
        )
    }
}
