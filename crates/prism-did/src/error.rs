//! Redaction-safe PRISM method failures.

use std::fmt;

use identus_core::{CapabilityId, ErrorCode, ErrorKind, IdentusError};

const CAPABILITY: CapabilityId = CapabilityId::new("did-prism");
const SYNTAX_CODE: ErrorCode = ErrorCode::new("did-prism.invalid_syntax");
const WIRE_CODE: ErrorCode = ErrorCode::new("did-prism.invalid_wire");
const OPERATION_CODE: ErrorCode = ErrorCode::new("did-prism.invalid_operation");
const STATE_CODE: ErrorCode = ErrorCode::new("did-prism.invalid_state");

/// A non-sensitive method-identifier failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum SyntaxError {
    /// The value is not a `did:prism` identifier.
    InvalidMethod,
    /// The canonical suffix is not lowercase 64-hex SHA-256.
    InvalidSuffix,
    /// The long-form state is missing or malformed.
    InvalidLongForm,
    /// The initial-state digest differs from the canonical suffix.
    InitialStateHashMismatch,
    /// The input exceeds the finite method profile.
    TooLong,
}

/// A non-sensitive protobuf envelope failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum WireError {
    /// The signed envelope exceeds its byte budget.
    EnvelopeTooLarge,
    /// The nested operation exceeds its byte budget.
    OperationTooLarge,
    /// A required envelope field is absent or empty.
    RequiredFieldMissing,
    /// A singular security-relevant field occurs more than once.
    DuplicateSingularField,
    /// More than one recognized operation kind occurs.
    DuplicateOperationKind,
    /// The envelope carries no supported operation kind.
    OperationMissing,
    /// A field, varint, length or wire type is malformed.
    Malformed,
}

/// A non-sensitive typed operation validation failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum OperationError {
    /// Required operation data is absent.
    RequiredDataMissing,
    /// A collection exceeds the method profile.
    TooManyItems,
    /// A field exceeds its character or byte limit.
    ValueTooLong,
    /// A field has invalid method syntax.
    InvalidValue,
    /// A supposedly unique identifier or context is repeated.
    DuplicateValue,
    /// A create or updated state has no active master key.
    MissingMasterKey,
    /// A key usage or curve is unsupported.
    UnsupportedKey,
    /// Signature verification failed.
    InvalidSignature,
}

/// A non-sensitive state-transition failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum StateError {
    /// The operation targets another DID or previous operation.
    HistoryMismatch,
    /// The state is already deactivated.
    Deactivated,
    /// An identifier is missing, retired, or already reserved.
    IdentifierConflict,
    /// The candidate state violates a final active-state invariant.
    InvalidCandidate,
}

/// Rich local PRISM method failure with a stable redacted bridge.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// Method syntax failed.
    InvalidSyntax(SyntaxError),
    /// Protobuf wire validation failed.
    InvalidWire(WireError),
    /// Typed operation validation failed.
    InvalidOperation(OperationError),
    /// State transition validation failed.
    InvalidState(StateError),
}

impl Error {
    /// Convert to the SDK's stable public error contract.
    pub const fn to_identus_error(self) -> IdentusError {
        let (code, message) = match self {
            Self::InvalidSyntax(_) => (SYNTAX_CODE, "invalid PRISM DID syntax"),
            Self::InvalidWire(_) => (WIRE_CODE, "invalid PRISM operation encoding"),
            Self::InvalidOperation(_) => (OPERATION_CODE, "invalid PRISM DID operation"),
            Self::InvalidState(_) => (STATE_CODE, "invalid PRISM DID state transition"),
        };
        IdentusError::public(code, ErrorKind::InvalidInput, CAPABILITY, message)
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidMethod => "identifier does not use the PRISM DID method",
            Self::InvalidSuffix => "canonical PRISM DID suffix is invalid",
            Self::InvalidLongForm => "PRISM long-form initial state is invalid",
            Self::InitialStateHashMismatch => {
                "PRISM long-form initial state does not match its suffix"
            }
            Self::TooLong => "PRISM DID value exceeds its byte limit",
        })
    }
}

impl fmt::Display for WireError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::EnvelopeTooLarge => "signed PRISM operation exceeds its byte limit",
            Self::OperationTooLarge => "PRISM operation exceeds its byte limit",
            Self::RequiredFieldMissing => "signed PRISM operation is missing required data",
            Self::DuplicateSingularField => "signed PRISM operation repeats a singular field",
            Self::DuplicateOperationKind => "PRISM operation repeats its operation kind",
            Self::OperationMissing => "PRISM operation kind is missing or unsupported",
            Self::Malformed => "PRISM operation protobuf is malformed",
        })
    }
}

impl fmt::Display for OperationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::RequiredDataMissing => "PRISM operation is missing required data",
            Self::TooManyItems => "PRISM operation contains too many items",
            Self::ValueTooLong => "PRISM operation value exceeds its limit",
            Self::InvalidValue => "PRISM operation value is invalid",
            Self::DuplicateValue => "PRISM operation contains a duplicate value",
            Self::MissingMasterKey => "PRISM operation has no active master key",
            Self::UnsupportedKey => "PRISM operation key profile is unsupported",
            Self::InvalidSignature => "PRISM operation signature is invalid",
        })
    }
}

impl fmt::Display for StateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::HistoryMismatch => "PRISM operation does not extend this state",
            Self::Deactivated => "PRISM DID state is deactivated",
            Self::IdentifierConflict => "PRISM state identifier conflicts with its history",
            Self::InvalidCandidate => "PRISM candidate state violates an invariant",
        })
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSyntax(reason) => write!(formatter, "invalid PRISM DID syntax: {reason}"),
            Self::InvalidWire(reason) => {
                write!(formatter, "invalid PRISM operation encoding: {reason}")
            }
            Self::InvalidOperation(reason) => {
                write!(formatter, "invalid PRISM DID operation: {reason}")
            }
            Self::InvalidState(reason) => {
                write!(formatter, "invalid PRISM DID state transition: {reason}")
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<WireError> for Error {
    fn from(value: WireError) -> Self {
        Self::InvalidWire(value)
    }
}

impl From<OperationError> for Error {
    fn from(value: OperationError) -> Self {
        Self::InvalidOperation(value)
    }
}

impl From<StateError> for Error {
    fn from(value: StateError) -> Self {
        Self::InvalidState(value)
    }
}
