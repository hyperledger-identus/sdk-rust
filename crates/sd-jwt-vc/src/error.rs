use std::fmt;

use identus_core::ErrorCode;

/// Stable, value-free SD-JWT VC profile failures.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum SdJwtVcError {
    InvalidLimits,
    ProtectedType,
    ProfileClaims,
    CredentialType,
    AdditionalTypes,
    ClaimPlacement,
    NumericDate,
    KeyBindingRequired,
    IssuerEvidence,
    TypeMetadata,
    TypeMetadataCycle,
    TypeMetadataRule,
    Status,
    UnsupportedStatus,
    EvidenceMismatch,
    ResourceLimit,
    Base,
    Envelope,
    LegacyDraftFeature,
    InternalInvariant,
}

impl SdJwtVcError {
    pub const fn code(self) -> ErrorCode {
        ErrorCode::new(match self {
            Self::InvalidLimits => "sd_jwt_vc.invalid_limits",
            Self::ProtectedType => "sd_jwt_vc.protected_type",
            Self::ProfileClaims => "sd_jwt_vc.profile_claims",
            Self::CredentialType => "sd_jwt_vc.credential_type",
            Self::AdditionalTypes => "sd_jwt_vc.additional_types",
            Self::ClaimPlacement => "sd_jwt_vc.claim_placement",
            Self::NumericDate => "sd_jwt_vc.numeric_date",
            Self::KeyBindingRequired => "sd_jwt_vc.key_binding_required",
            Self::IssuerEvidence => "sd_jwt_vc.issuer_evidence",
            Self::TypeMetadata => "sd_jwt_vc.type_metadata",
            Self::TypeMetadataCycle => "sd_jwt_vc.type_metadata_cycle",
            Self::TypeMetadataRule => "sd_jwt_vc.type_metadata_rule",
            Self::Status => "sd_jwt_vc.status",
            Self::UnsupportedStatus => "sd_jwt_vc.unsupported_status",
            Self::EvidenceMismatch => "sd_jwt_vc.evidence_mismatch",
            Self::ResourceLimit => "sd_jwt_vc.resource_limit",
            Self::Base => "sd_jwt_vc.base",
            Self::Envelope => "sd_jwt_vc.envelope",
            Self::LegacyDraftFeature => "sd_jwt_vc.legacy_draft_feature",
            Self::InternalInvariant => "sd_jwt_vc.internal_invariant",
        })
    }
}

impl fmt::Display for SdJwtVcError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code().as_str())
    }
}

impl std::error::Error for SdJwtVcError {}
