//! Redaction-safe credential construction errors.

use std::fmt;

use identus_core::{CapabilityId, IdentusError};

use crate::error_contract::{ErrorContract, envelope, metadata, status, verification};

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
    pub const DUPLICATE_CREDENTIAL_VERIFIER_FORMAT: ErrorCode =
        ErrorCode::new("credential.duplicate_verifier_format");
    pub const TOO_MANY_CREDENTIAL_VERIFIER_FORMATS: ErrorCode =
        ErrorCode::new("credential.too_many_verifier_formats");
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
    pub const INVALID_STATUS_METHOD: ErrorCode = ErrorCode::new("credential.invalid_status_method");
    pub const INVALID_STATUS_PURPOSE: ErrorCode =
        ErrorCode::new("credential.invalid_status_purpose");
    pub const INVALID_STATUS_REFERENCE: ErrorCode =
        ErrorCode::new("credential.invalid_status_reference");
    pub const INVALID_STATUS_HANDLE: ErrorCode = ErrorCode::new("credential.invalid_status_handle");
    pub const INVALID_STATUS_REVISION: ErrorCode =
        ErrorCode::new("credential.invalid_status_revision");
    pub const INVALID_STATUS_VALUE: ErrorCode = ErrorCode::new("credential.invalid_status_value");
    pub const INVALID_STATUS_BINDING_COLLECTION: ErrorCode =
        ErrorCode::new("credential.invalid_status_binding_collection");
    pub const DUPLICATE_STATUS_BINDING: ErrorCode =
        ErrorCode::new("credential.duplicate_status_binding");
    pub const INVALID_STATUS_FRESHNESS: ErrorCode =
        ErrorCode::new("credential.invalid_status_freshness");
    pub const INVALID_STATUS_REQUIREMENTS: ErrorCode =
        ErrorCode::new("credential.invalid_status_requirements");
    pub const DUPLICATE_STATUS_METHOD: ErrorCode =
        ErrorCode::new("credential.duplicate_status_method");
    pub const DUPLICATE_STATUS_PURPOSE: ErrorCode =
        ErrorCode::new("credential.duplicate_status_purpose");
    pub const INVALID_STATUS_EVIDENCE_RANGE: ErrorCode =
        ErrorCode::new("credential.invalid_status_evidence_range");
    pub const STATUS_QUERY_MISMATCH: ErrorCode = ErrorCode::new("credential.status_query_mismatch");
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
    /// A verifier registry repeats an exact credential format binding.
    DuplicateCredentialVerifierFormat,
    /// A verifier registry exceeds its bounded format capacity.
    TooManyCredentialVerifierFormats,
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
    /// A status method identifier violates its text bounds.
    InvalidStatusMethod,
    /// A status purpose identifier violates its text bounds.
    InvalidStatusPurpose,
    /// A status reference violates its text or byte bounds.
    InvalidStatusReference,
    /// A status handle violates its text or byte bounds.
    InvalidStatusHandle,
    /// A status revision violates its text or byte bounds.
    InvalidStatusRevision,
    /// An observed status value violates its text or byte bounds.
    InvalidStatusValue,
    /// A status-binding collection is empty or exceeds its bound.
    InvalidStatusBindingCollection,
    /// A status-binding collection repeats an exact binding.
    DuplicateStatusBinding,
    /// A freshness value has no criterion.
    InvalidStatusFreshness,
    /// A status requirement allow-list is empty or exceeds its bound.
    InvalidStatusRequirements,
    /// A status requirement repeats a method.
    DuplicateStatusMethod,
    /// A status requirement repeats a purpose.
    DuplicateStatusPurpose,
    /// Status evidence expires before it was observed.
    InvalidStatusEvidenceRange,
    /// A query binding is excluded by its own structural allow-list.
    StatusQueryMismatch,
}

macro_rules! define_credential_error_contracts {
    ($($variant:ident => $contract:path),+ $(,)?) => {
        impl CredentialError {
            const fn contract(self) -> ErrorContract {
                match self {
                    $(Self::$variant => $contract),+
                }
            }

            #[cfg(test)]
            pub(crate) const CONTRACT_VARIANTS: &'static [Self] = &[
                $(Self::$variant),+
            ];
        }
    };
}

define_credential_error_contracts! {
    InvalidFormat => envelope::INVALID_FORMAT,
    EmptyPayload => envelope::EMPTY_PAYLOAD,
    PayloadTooLarge => envelope::PAYLOAD_TOO_LARGE,
    EmptyDetachedProof => envelope::EMPTY_DETACHED_PROOF,
    DetachedProofTooLarge => envelope::DETACHED_PROOF_TOO_LARGE,
    EmptyPrivateMaterial => envelope::EMPTY_PRIVATE_MATERIAL,
    PrivateMaterialTooLarge => envelope::PRIVATE_MATERIAL_TOO_LARGE,
    InvalidVerificationStageName => verification::INVALID_VERIFICATION_STAGE_NAME,
    InvalidVerificationReasonCode => verification::INVALID_VERIFICATION_REASON_CODE,
    MissingVerificationReason => verification::MISSING_VERIFICATION_REASON,
    UnexpectedVerificationReason => verification::UNEXPECTED_VERIFICATION_REASON,
    NonCanonicalVerificationReport => verification::NON_CANONICAL_VERIFICATION_REPORT,
    DuplicateCredentialVerifierFormat => verification::DUPLICATE_CREDENTIAL_VERIFIER_FORMAT,
    TooManyCredentialVerifierFormats => verification::TOO_MANY_CREDENTIAL_VERIFIER_FORMATS,
    InvalidEntityIdentifier => metadata::INVALID_ENTITY_IDENTIFIER,
    InvalidCredentialType => metadata::INVALID_CREDENTIAL_TYPE,
    InvalidSchemaIdentifier => metadata::INVALID_SCHEMA_IDENTIFIER,
    InvalidSchemaVersion => metadata::INVALID_SCHEMA_VERSION,
    InvalidClaimIdentifier => metadata::INVALID_CLAIM_IDENTIFIER,
    InvalidClaimValueType => metadata::INVALID_CLAIM_VALUE_TYPE,
    InvalidClaimPathSegment => metadata::INVALID_CLAIM_PATH_SEGMENT,
    InvalidClaimPath => metadata::INVALID_CLAIM_PATH,
    InvalidClaimDisclosure => metadata::INVALID_CLAIM_DISCLOSURE,
    InvalidDescriptorCollection => metadata::INVALID_DESCRIPTOR_COLLECTION,
    DuplicateCredentialSubject => metadata::DUPLICATE_CREDENTIAL_SUBJECT,
    DuplicateCredentialType => metadata::DUPLICATE_CREDENTIAL_TYPE,
    DuplicateSchemaIdentifier => metadata::DUPLICATE_SCHEMA_IDENTIFIER,
    DuplicateClaimIdentifier => metadata::DUPLICATE_CLAIM_IDENTIFIER,
    DuplicateClaimPath => metadata::DUPLICATE_CLAIM_PATH,
    InvalidValidityRange => metadata::INVALID_VALIDITY_RANGE,
    InvalidStatusMethod => status::INVALID_STATUS_METHOD,
    InvalidStatusPurpose => status::INVALID_STATUS_PURPOSE,
    InvalidStatusReference => status::INVALID_STATUS_REFERENCE,
    InvalidStatusHandle => status::INVALID_STATUS_HANDLE,
    InvalidStatusRevision => status::INVALID_STATUS_REVISION,
    InvalidStatusValue => status::INVALID_STATUS_VALUE,
    InvalidStatusBindingCollection => status::INVALID_STATUS_BINDING_COLLECTION,
    DuplicateStatusBinding => status::DUPLICATE_STATUS_BINDING,
    InvalidStatusFreshness => status::INVALID_STATUS_FRESHNESS,
    InvalidStatusRequirements => status::INVALID_STATUS_REQUIREMENTS,
    DuplicateStatusMethod => status::DUPLICATE_STATUS_METHOD,
    DuplicateStatusPurpose => status::DUPLICATE_STATUS_PURPOSE,
    InvalidStatusEvidenceRange => status::INVALID_STATUS_EVIDENCE_RANGE,
    StatusQueryMismatch => status::STATUS_QUERY_MISMATCH,
}

impl CredentialError {
    /// Convert to the stable, redaction-safe shared SDK error.
    pub fn to_identus_error(self) -> IdentusError {
        self.contract().to_identus_error()
    }
}

impl fmt::Display for CredentialError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.contract().local_display())
    }
}

impl std::error::Error for CredentialError {}
