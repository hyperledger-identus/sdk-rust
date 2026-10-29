use std::fmt;

use identus_credentials::CredentialEnvelope;
use identus_sd_jwt::{DisclosureVerifiedSdJwt, SdJwtLimits, UnverifiedSdJwt};

use crate::{
    IssuerEvidence, IssuerEvidenceKind, SdJwtVcError, SdJwtVcLimits,
    profile::{
        ensure_retained_output, valid_confirmation, validate_claims,
        validate_non_selective_placement,
    },
};

/// Stable capability identifier for the opt-in migration boundary.
pub const LEGACY_CAPABILITY_ID: &str = "credentials.sd-jwt-vc.legacy-draft03-input.v1";
pub const LEGACY_PROTECTED_TYPE: &str = "vc+sd-jwt";
pub const LEGACY_MEDIA_TYPE: &str = "application/vc+sd-jwt";
pub const LEGACY_ENVELOPE_FORMAT: &str = "vc+sd-jwt";
pub const LEGACY_DRAFT_NAME: &str = "draft-ietf-oauth-sd-jwt-vc-03";
pub const LEGACY_DRAFT_DATE: &str = "2024-03-04";
pub const LEGACY_DRAFT_SOURCE: &str =
    "https://www.ietf.org/archive/id/draft-ietf-oauth-sd-jwt-vc-03.txt";
pub const LEGACY_DRAFT_SHA256: &str =
    "9599225eb8732c0ef3f9a6524589e637f9272ba742da185c43ecfd272b81bf5a";

/// Caller-owned evidence accepted by the legacy common-subset adapter.
#[derive(Default)]
pub struct LegacyValidationInputs<'a> {
    pub require_key_binding: bool,
    pub issuer_evidence: Option<&'a IssuerEvidence>,
}

impl fmt::Debug for LegacyValidationInputs<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LegacyValidationInputs")
            .field("require_key_binding", &self.require_key_binding)
            .field("issuer_evidence", &self.issuer_evidence.is_some())
            .finish()
    }
}

/// Value-free proof that only the qualified legacy common subset was checked.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LegacyDraft03VerificationReceipt {
    pub issuer_signature: bool,
    pub disclosures_processed: bool,
    pub common_subset_validated: bool,
    pub issuer_evidence: Option<IssuerEvidenceKind>,
    pub canonical_reissuance_required: bool,
}

/// A verified legacy draft-03 credential. This type cannot become canonical
/// draft-22 without issuer reissuance.
pub struct LegacyDraft03SdJwtVc {
    base: DisclosureVerifiedSdJwt,
    vct: String,
    receipt: LegacyDraft03VerificationReceipt,
}

impl LegacyDraft03SdJwtVc {
    pub fn matches_type(&self, candidate: &str) -> bool {
        self.vct == candidate
    }

    pub const fn receipt(&self) -> LegacyDraft03VerificationReceipt {
        self.receipt
    }

    pub const fn base(&self) -> &DisclosureVerifiedSdJwt {
        &self.base
    }

    pub fn into_base(self) -> DisclosureVerifiedSdJwt {
        self.base
    }
}

impl fmt::Debug for LegacyDraft03SdJwtVc {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LegacyDraft03SdJwtVc")
            .field("receipt", &self.receipt)
            .finish_non_exhaustive()
    }
}

/// Explicit input-only adapter for the draft-03 common subset.
#[derive(Clone, Copy, Debug, Default)]
pub struct LegacyVcSdJwtDraft03Adapter;

impl LegacyVcSdJwtDraft03Adapter {
    pub const CAPABILITY_ID: &'static str = LEGACY_CAPABILITY_ID;

    /// Parse the exact legacy generic envelope without assigning trust.
    pub fn parse_envelope(
        envelope: &CredentialEnvelope,
        limits: SdJwtLimits,
    ) -> Result<UnverifiedSdJwt, SdJwtVcError> {
        if envelope.format().as_str() != LEGACY_ENVELOPE_FORMAT {
            return Err(SdJwtVcError::Envelope);
        }
        let compact = std::str::from_utf8(envelope.payload().as_bytes())
            .map_err(|_| SdJwtVcError::Envelope)?;
        UnverifiedSdJwt::parse(compact, limits).map_err(|_| SdJwtVcError::Base)
    }

    /// Validate a signature-verified, disclosure-processed legacy credential.
    ///
    /// This proves only the draft-03/draft-22 common subset and never relabels
    /// or emits the signed credential.
    pub fn validate(
        base: DisclosureVerifiedSdJwt,
        inputs: LegacyValidationInputs<'_>,
        limits: SdJwtVcLimits,
    ) -> Result<LegacyDraft03SdJwtVc, SdJwtVcError> {
        limits.validate()?;
        if base.issuer_protected_header().type_() != Some(LEGACY_PROTECTED_TYPE) {
            return Err(SdJwtVcError::ProtectedType);
        }
        let claims = base
            .claims()
            .as_object()
            .ok_or(SdJwtVcError::ProfileClaims)?;
        if ["status", "aka_vcts", "vct#integrity"]
            .iter()
            .any(|name| claims.contains_key(*name))
        {
            return Err(SdJwtVcError::LegacyDraftFeature);
        }
        let (vct, additional_types) = validate_claims(claims, limits, true)?;
        debug_assert!(additional_types.is_empty());
        ensure_retained_output(&vct, &additional_types, None, limits)?;
        validate_non_selective_placement(&base)?;
        if inputs.require_key_binding && !valid_confirmation(claims.get("cnf"), limits) {
            return Err(SdJwtVcError::KeyBindingRequired);
        }
        let issuer = claims
            .get("iss")
            .and_then(serde_json::Value::as_str)
            .ok_or(SdJwtVcError::ProfileClaims)?;
        let issuer_kind = inputs
            .issuer_evidence
            .map(|evidence| {
                evidence.correlate(
                    base.issuer_protected_header(),
                    base.issuer_verification_key(),
                    issuer,
                    limits,
                )
            })
            .transpose()?;
        Ok(LegacyDraft03SdJwtVc {
            base,
            vct,
            receipt: LegacyDraft03VerificationReceipt {
                issuer_signature: true,
                disclosures_processed: true,
                common_subset_validated: true,
                issuer_evidence: issuer_kind,
                canonical_reissuance_required: true,
            },
        })
    }
}
