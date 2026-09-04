//! Redaction-safe credential construction errors.

use std::fmt;

use identus_core::{CapabilityId, ErrorKind, IdentusError};

/// Owning capability for credential construction errors.
pub const CAPABILITY: CapabilityId = CapabilityId::new("credential");

/// Stable error codes used by the shared SDK error boundary.
pub mod error_code {
    use identus_core::ErrorCode;

    pub const INVALID_FORMAT: ErrorCode = ErrorCode::new("credential.invalid_format");
    pub const EMPTY_PAYLOAD: ErrorCode = ErrorCode::new("credential.empty_payload");
    pub const PAYLOAD_TOO_LARGE: ErrorCode = ErrorCode::new("credential.payload_too_large");
    pub const EMPTY_DETACHED_PROOF: ErrorCode = ErrorCode::new("credential.empty_detached_proof");
    pub const DETACHED_PROOF_TOO_LARGE: ErrorCode =
        ErrorCode::new("credential.detached_proof_too_large");
    pub const EMPTY_PRIVATE_MATERIAL: ErrorCode =
        ErrorCode::new("credential.empty_private_material");
    pub const PRIVATE_MATERIAL_TOO_LARGE: ErrorCode =
        ErrorCode::new("credential.private_material_too_large");
    pub const INVALID_VERIFICATION_STAGE_NAME: ErrorCode =
        ErrorCode::new("credential.invalid_verification_stage_name");
    pub const INVALID_VERIFICATION_REASON_CODE: ErrorCode =
        ErrorCode::new("credential.invalid_verification_reason_code");
    pub const MISSING_VERIFICATION_REASON: ErrorCode =
        ErrorCode::new("credential.missing_verification_reason");
    pub const UNEXPECTED_VERIFICATION_REASON: ErrorCode =
        ErrorCode::new("credential.unexpected_verification_reason");
    pub const NON_CANONICAL_VERIFICATION_REPORT: ErrorCode =
        ErrorCode::new("credential.non_canonical_verification_report");
    pub const INVALID_ENTITY_IDENTIFIER: ErrorCode =
        ErrorCode::new("credential.invalid_entity_identifier");
    pub const INVALID_CREDENTIAL_TYPE: ErrorCode =
        ErrorCode::new("credential.invalid_credential_type");
    pub const INVALID_SCHEMA_IDENTIFIER: ErrorCode =
        ErrorCode::new("credential.invalid_schema_identifier");
    pub const INVALID_SCHEMA_VERSION: ErrorCode =
        ErrorCode::new("credential.invalid_schema_version");
    pub const INVALID_CLAIM_IDENTIFIER: ErrorCode =
        ErrorCode::new("credential.invalid_claim_identifier");
    pub const INVALID_CLAIM_VALUE_TYPE: ErrorCode =
        ErrorCode::new("credential.invalid_claim_value_type");
    pub const INVALID_CLAIM_PATH_SEGMENT: ErrorCode =
        ErrorCode::new("credential.invalid_claim_path_segment");
    pub const INVALID_CLAIM_PATH: ErrorCode = ErrorCode::new("credential.invalid_claim_path");
    pub const INVALID_CLAIM_DISCLOSURE: ErrorCode =
        ErrorCode::new("credential.invalid_claim_disclosure");
    pub const INVALID_DESCRIPTOR_COLLECTION: ErrorCode =
        ErrorCode::new("credential.invalid_descriptor_collection");
    pub const DUPLICATE_CREDENTIAL_SUBJECT: ErrorCode =
        ErrorCode::new("credential.duplicate_credential_subject");
    pub const DUPLICATE_CREDENTIAL_TYPE: ErrorCode =
        ErrorCode::new("credential.duplicate_credential_type");
    pub const DUPLICATE_SCHEMA_IDENTIFIER: ErrorCode =
        ErrorCode::new("credential.duplicate_schema_identifier");
    pub const DUPLICATE_CLAIM_IDENTIFIER: ErrorCode =
        ErrorCode::new("credential.duplicate_claim_identifier");
    pub const DUPLICATE_CLAIM_PATH: ErrorCode = ErrorCode::new("credential.duplicate_claim_path");
    pub const INVALID_VALIDITY_RANGE: ErrorCode =
        ErrorCode::new("credential.invalid_validity_range");
}

/// Typed reason that a credential domain value could not be constructed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum CredentialError {
    /// A format identifier violates the open token grammar or size limit.
    InvalidFormat,
    /// An encoded credential payload is empty.
    EmptyPayload,
    /// An encoded credential payload exceeds its size limit.
    PayloadTooLarge,
    /// A detached proof is empty.
    EmptyDetachedProof,
    /// A detached proof exceeds its size limit.
    DetachedProofTooLarge,
    /// Format-private material is empty.
    EmptyPrivateMaterial,
    /// Format-private material exceeds its size limit.
    PrivateMaterialTooLarge,
    /// A verification stage spelling is not part of the fixed taxonomy.
    InvalidVerificationStageName,
    /// A verification reason violates its token grammar or size limit.
    InvalidVerificationReasonCode,
    /// A failed or not-checked stage has no machine reason.
    MissingVerificationReason,
    /// A passed stage incorrectly carries a machine reason.
    UnexpectedVerificationReason,
    /// Verification stages are not complete and canonically ordered.
    NonCanonicalVerificationReport,
    /// An issuer or subject identifier violates the descriptor bounds.
    InvalidEntityIdentifier,
    /// A credential type violates the descriptor bounds.
    InvalidCredentialType,
    /// A schema identifier violates the descriptor bounds.
    InvalidSchemaIdentifier,
    /// A schema version violates the descriptor bounds.
    InvalidSchemaVersion,
    /// A claim identifier violates the descriptor bounds.
    InvalidClaimIdentifier,
    /// A claim value-type hint violates the descriptor bounds.
    InvalidClaimValueType,
    /// A claim-path segment violates the descriptor bounds.
    InvalidClaimPathSegment,
    /// A claim path is empty or exceeds its segment bound.
    InvalidClaimPath,
    /// A claim disclosure spelling is not part of the fixed vocabulary.
    InvalidClaimDisclosure,
    /// A descriptor collection is empty when required or exceeds its bound.
    InvalidDescriptorCollection,
    /// Credential metadata repeats a subject identifier.
    DuplicateCredentialSubject,
    /// Metadata or a schema repeats a credential type.
    DuplicateCredentialType,
    /// Credential metadata repeats a schema identifier.
    DuplicateSchemaIdentifier,
    /// A schema repeats a claim identifier.
    DuplicateClaimIdentifier,
    /// A schema repeats a complete claim path.
    DuplicateClaimPath,
    /// A validity interval ends before it starts.
    InvalidValidityRange,
}

impl CredentialError {
    /// Convert to the stable, redaction-safe shared SDK error.
    pub fn to_identus_error(self) -> IdentusError {
        let (code, public_message) = match self {
            Self::InvalidFormat => (error_code::INVALID_FORMAT, "invalid credential format"),
            Self::EmptyPayload => (error_code::EMPTY_PAYLOAD, "credential payload is empty"),
            Self::PayloadTooLarge => (
                error_code::PAYLOAD_TOO_LARGE,
                "credential payload exceeds the size limit",
            ),
            Self::EmptyDetachedProof => (
                error_code::EMPTY_DETACHED_PROOF,
                "credential detached proof is empty",
            ),
            Self::DetachedProofTooLarge => (
                error_code::DETACHED_PROOF_TOO_LARGE,
                "credential detached proof exceeds the size limit",
            ),
            Self::EmptyPrivateMaterial => (
                error_code::EMPTY_PRIVATE_MATERIAL,
                "credential private material is empty",
            ),
            Self::PrivateMaterialTooLarge => (
                error_code::PRIVATE_MATERIAL_TOO_LARGE,
                "credential private material exceeds the size limit",
            ),
            Self::InvalidVerificationStageName => (
                error_code::INVALID_VERIFICATION_STAGE_NAME,
                "invalid credential verification stage name",
            ),
            Self::InvalidVerificationReasonCode => (
                error_code::INVALID_VERIFICATION_REASON_CODE,
                "invalid credential verification reason code",
            ),
            Self::MissingVerificationReason => (
                error_code::MISSING_VERIFICATION_REASON,
                "credential verification reason is required",
            ),
            Self::UnexpectedVerificationReason => (
                error_code::UNEXPECTED_VERIFICATION_REASON,
                "credential verification reason is not allowed",
            ),
            Self::NonCanonicalVerificationReport => (
                error_code::NON_CANONICAL_VERIFICATION_REPORT,
                "credential verification report is not canonical",
            ),
            Self::InvalidEntityIdentifier => (
                error_code::INVALID_ENTITY_IDENTIFIER,
                "invalid credential entity identifier",
            ),
            Self::InvalidCredentialType => (
                error_code::INVALID_CREDENTIAL_TYPE,
                "invalid credential type",
            ),
            Self::InvalidSchemaIdentifier => (
                error_code::INVALID_SCHEMA_IDENTIFIER,
                "invalid credential schema identifier",
            ),
            Self::InvalidSchemaVersion => (
                error_code::INVALID_SCHEMA_VERSION,
                "invalid credential schema version",
            ),
            Self::InvalidClaimIdentifier => (
                error_code::INVALID_CLAIM_IDENTIFIER,
                "invalid credential claim identifier",
            ),
            Self::InvalidClaimValueType => (
                error_code::INVALID_CLAIM_VALUE_TYPE,
                "invalid credential claim value type",
            ),
            Self::InvalidClaimPathSegment => (
                error_code::INVALID_CLAIM_PATH_SEGMENT,
                "invalid credential claim path segment",
            ),
            Self::InvalidClaimPath => (
                error_code::INVALID_CLAIM_PATH,
                "invalid credential claim path",
            ),
            Self::InvalidClaimDisclosure => (
                error_code::INVALID_CLAIM_DISCLOSURE,
                "invalid credential claim disclosure",
            ),
            Self::InvalidDescriptorCollection => (
                error_code::INVALID_DESCRIPTOR_COLLECTION,
                "invalid credential descriptor collection",
            ),
            Self::DuplicateCredentialSubject => (
                error_code::DUPLICATE_CREDENTIAL_SUBJECT,
                "credential subject identifier is duplicated",
            ),
            Self::DuplicateCredentialType => (
                error_code::DUPLICATE_CREDENTIAL_TYPE,
                "credential type is duplicated",
            ),
            Self::DuplicateSchemaIdentifier => (
                error_code::DUPLICATE_SCHEMA_IDENTIFIER,
                "credential schema identifier is duplicated",
            ),
            Self::DuplicateClaimIdentifier => (
                error_code::DUPLICATE_CLAIM_IDENTIFIER,
                "credential claim identifier is duplicated",
            ),
            Self::DuplicateClaimPath => (
                error_code::DUPLICATE_CLAIM_PATH,
                "credential claim path is duplicated",
            ),
            Self::InvalidValidityRange => (
                error_code::INVALID_VALIDITY_RANGE,
                "credential validity range is invalid",
            ),
        };

        IdentusError::public(code, ErrorKind::InvalidInput, CAPABILITY, public_message)
    }
}

impl fmt::Display for CredentialError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidFormat => "credential format is invalid",
            Self::EmptyPayload => "credential payload is empty",
            Self::PayloadTooLarge => "credential payload exceeds the size limit",
            Self::EmptyDetachedProof => "credential detached proof is empty",
            Self::DetachedProofTooLarge => "credential detached proof exceeds the size limit",
            Self::EmptyPrivateMaterial => "credential private material is empty",
            Self::PrivateMaterialTooLarge => "credential private material exceeds the size limit",
            Self::InvalidVerificationStageName => "credential verification stage name is invalid",
            Self::InvalidVerificationReasonCode => "credential verification reason code is invalid",
            Self::MissingVerificationReason => "credential verification reason is required",
            Self::UnexpectedVerificationReason => "credential verification reason is not allowed",
            Self::NonCanonicalVerificationReport => {
                "credential verification report is not canonical"
            }
            Self::InvalidEntityIdentifier => "credential entity identifier is invalid",
            Self::InvalidCredentialType => "credential type is invalid",
            Self::InvalidSchemaIdentifier => "credential schema identifier is invalid",
            Self::InvalidSchemaVersion => "credential schema version is invalid",
            Self::InvalidClaimIdentifier => "credential claim identifier is invalid",
            Self::InvalidClaimValueType => "credential claim value type is invalid",
            Self::InvalidClaimPathSegment => "credential claim path segment is invalid",
            Self::InvalidClaimPath => "credential claim path is invalid",
            Self::InvalidClaimDisclosure => "credential claim disclosure is invalid",
            Self::InvalidDescriptorCollection => "credential descriptor collection is invalid",
            Self::DuplicateCredentialSubject => "credential subject identifier is duplicated",
            Self::DuplicateCredentialType => "credential type is duplicated",
            Self::DuplicateSchemaIdentifier => "credential schema identifier is duplicated",
            Self::DuplicateClaimIdentifier => "credential claim identifier is duplicated",
            Self::DuplicateClaimPath => "credential claim path is duplicated",
            Self::InvalidValidityRange => "credential validity range is invalid",
        })
    }
}

impl std::error::Error for CredentialError {}
