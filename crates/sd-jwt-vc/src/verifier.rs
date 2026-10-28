use std::{fmt, sync::Arc};

use identus_credentials::{
    CredentialVerificationError, CredentialVerificationFuture, CredentialVerificationRequest,
    CredentialVerifier, VerificationReasonCode, VerificationReport, VerificationStage,
    VerificationStageName, VerificationStageStatus,
};
use identus_crypto::PublicKeyJwk;
use identus_jose::{JwsAlgorithm, JwsVerificationKey, SignatureSuiteRegistry};
use identus_sd_jwt::{SdJwtLimits, UnverifiedSdJwt};

use crate::{
    Draft22SdJwtVc, ENVELOPE_FORMAT, IssuerEvidence, ProfileValidationInputs, SdJwtVcLimits,
    StatusListEvidence, TypeMetadataChain, profile::ProfileFailureStage,
};

/// Optional owned evidence used by the generic credential-verifier adapter.
#[derive(Clone, Default)]
pub struct Draft22VerifierEvidence {
    pub require_key_binding: bool,
    pub issuer: Option<IssuerEvidence>,
    pub type_metadata: Option<TypeMetadataChain>,
    pub status: Option<StatusListEvidence>,
}

impl fmt::Debug for Draft22VerifierEvidence {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Draft22VerifierEvidence")
            .field("require_key_binding", &self.require_key_binding)
            .field("issuer", &self.issuer.is_some())
            .field("type_metadata", &self.type_metadata.is_some())
            .field("status", &self.status.is_some())
            .finish()
    }
}

/// Exact-format adapter over a caller-selected issuer verification key.
///
/// The adapter reports policy-neutral evidence only. The selected key and
/// optional discovery metadata do not imply issuer authorization or trust.
pub struct Draft22CredentialVerifier {
    suites: Arc<SignatureSuiteRegistry>,
    algorithm: JwsAlgorithm,
    issuer_key: PublicKeyJwk,
    evidence: Draft22VerifierEvidence,
    base_limits: SdJwtLimits,
    profile_limits: SdJwtVcLimits,
}

impl Draft22CredentialVerifier {
    pub fn new(
        suites: Arc<SignatureSuiteRegistry>,
        algorithm: JwsAlgorithm,
        issuer_key: PublicKeyJwk,
        evidence: Draft22VerifierEvidence,
        base_limits: SdJwtLimits,
        profile_limits: SdJwtVcLimits,
    ) -> Result<Self, crate::SdJwtVcError> {
        profile_limits.validate()?;
        JwsVerificationKey::new(algorithm, &issuer_key)
            .map_err(|_| crate::SdJwtVcError::IssuerEvidence)?;
        Ok(Self {
            suites,
            algorithm,
            issuer_key,
            evidence,
            base_limits,
            profile_limits,
        })
    }
}

impl fmt::Debug for Draft22CredentialVerifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Draft22CredentialVerifier")
            .field("algorithm", &self.algorithm)
            .field("evidence", &self.evidence)
            .finish_non_exhaustive()
    }
}

impl CredentialVerifier for Draft22CredentialVerifier {
    fn verify<'a>(
        &'a self,
        request: CredentialVerificationRequest<'a>,
    ) -> CredentialVerificationFuture<'a> {
        Box::pin(async move {
            if request.format().as_str() != ENVELOPE_FORMAT || request.detached_proof().is_some() {
                return Err(CredentialVerificationError::UnsupportedFormat);
            }
            let key = JwsVerificationKey::new(self.algorithm, &self.issuer_key)
                .map_err(|_| CredentialVerificationError::Internal)?;
            let result = verify_request(self, request.payload().as_bytes(), &key);
            match result {
                Ok(()) => valid_report(&self.evidence),
                Err(stage) => invalid_report(stage, &self.evidence),
            }
            .map_err(|_| CredentialVerificationError::Internal)
        })
    }
}

#[derive(Clone, Copy)]
enum FailureStage {
    StructuralBeforeProof,
    Proof,
    Profile(ProfileFailureStage),
}

fn verify_request(
    verifier: &Draft22CredentialVerifier,
    payload: &[u8],
    key: &JwsVerificationKey<'_>,
) -> Result<(), FailureStage> {
    let compact = std::str::from_utf8(payload).map_err(|_| FailureStage::StructuralBeforeProof)?;
    let parsed = UnverifiedSdJwt::parse(compact, verifier.base_limits)
        .map_err(|_| FailureStage::StructuralBeforeProof)?;
    let verified = parsed
        .verify_issuer(&verifier.suites, key)
        .map_err(|_| FailureStage::Proof)?;
    let base = verified
        .process_disclosures()
        .map_err(|_| FailureStage::Profile(ProfileFailureStage::Structural))?;
    Draft22SdJwtVc::validate_with_stage(
        base,
        ProfileValidationInputs {
            require_key_binding: verifier.evidence.require_key_binding,
            issuer_evidence: verifier.evidence.issuer.as_ref(),
            type_metadata: verifier.evidence.type_metadata.as_ref(),
            status_evidence: verifier.evidence.status.as_ref(),
        },
        verifier.profile_limits,
    )
    .map_err(|(_, stage)| FailureStage::Profile(stage))?;
    Ok(())
}

fn reason_code(
    value: &'static str,
) -> Result<VerificationReasonCode, identus_credentials::CredentialError> {
    VerificationReasonCode::parse(value)
}

fn stage(
    name: VerificationStageName,
    status: VerificationStageStatus,
    reason: Option<&'static str>,
) -> Result<VerificationStage, identus_credentials::CredentialError> {
    VerificationStage::new(name, status, reason.map(reason_code).transpose()?)
}

fn valid_report(
    evidence: &Draft22VerifierEvidence,
) -> Result<VerificationReport, identus_credentials::CredentialError> {
    VerificationReport::new([
        stage(
            VerificationStageName::Structural,
            VerificationStageStatus::Passed,
            None,
        )?,
        if evidence.issuer.is_some() {
            stage(
                VerificationStageName::IssuerKey,
                VerificationStageStatus::Passed,
                None,
            )?
        } else {
            stage(
                VerificationStageName::IssuerKey,
                VerificationStageStatus::NotChecked,
                Some("sd_jwt_vc.issuer_resolution_not_checked"),
            )?
        },
        stage(
            VerificationStageName::Proof,
            VerificationStageStatus::Passed,
            None,
        )?,
        stage(
            VerificationStageName::Temporal,
            VerificationStageStatus::NotChecked,
            Some("sd_jwt_vc.temporal_policy_not_checked"),
        )?,
        stage(
            VerificationStageName::Status,
            VerificationStageStatus::NotChecked,
            Some(if evidence.status.is_some() {
                "sd_jwt_vc.status_correlated_policy_not_checked"
            } else {
                "sd_jwt_vc.status_not_checked"
            }),
        )?,
        if evidence.type_metadata.is_some() {
            stage(
                VerificationStageName::Schema,
                VerificationStageStatus::Passed,
                None,
            )?
        } else {
            stage(
                VerificationStageName::Schema,
                VerificationStageStatus::NotChecked,
                Some("sd_jwt_vc.metadata_not_checked"),
            )?
        },
    ])
}

fn invalid_report(
    failure: FailureStage,
    evidence: &Draft22VerifierEvidence,
) -> Result<VerificationReport, identus_credentials::CredentialError> {
    let structural = if matches!(
        failure,
        FailureStage::StructuralBeforeProof
            | FailureStage::Profile(ProfileFailureStage::Structural)
    ) {
        stage(
            VerificationStageName::Structural,
            VerificationStageStatus::Failed,
            Some("sd_jwt_vc.invalid"),
        )?
    } else {
        stage(
            VerificationStageName::Structural,
            VerificationStageStatus::Passed,
            None,
        )?
    };
    let proof = match failure {
        FailureStage::Proof => stage(
            VerificationStageName::Proof,
            VerificationStageStatus::Failed,
            Some("sd_jwt_vc.proof_invalid"),
        )?,
        FailureStage::Profile(_) => stage(
            VerificationStageName::Proof,
            VerificationStageStatus::Passed,
            None,
        )?,
        FailureStage::StructuralBeforeProof => stage(
            VerificationStageName::Proof,
            VerificationStageStatus::NotChecked,
            Some("sd_jwt_vc.prerequisite_failed"),
        )?,
    };
    VerificationReport::new([
        structural,
        match failure {
            FailureStage::Profile(ProfileFailureStage::IssuerKey) => stage(
                VerificationStageName::IssuerKey,
                VerificationStageStatus::Failed,
                Some("sd_jwt_vc.issuer_evidence_invalid"),
            )?,
            FailureStage::Profile(ProfileFailureStage::Schema | ProfileFailureStage::Status)
                if evidence.issuer.is_some() =>
            {
                stage(
                    VerificationStageName::IssuerKey,
                    VerificationStageStatus::Passed,
                    None,
                )?
            }
            _ => stage(
                VerificationStageName::IssuerKey,
                VerificationStageStatus::NotChecked,
                Some("sd_jwt_vc.issuer_resolution_not_checked"),
            )?,
        },
        proof,
        stage(
            VerificationStageName::Temporal,
            VerificationStageStatus::NotChecked,
            Some("sd_jwt_vc.prerequisite_failed"),
        )?,
        if matches!(failure, FailureStage::Profile(ProfileFailureStage::Status)) {
            stage(
                VerificationStageName::Status,
                VerificationStageStatus::Failed,
                Some("sd_jwt_vc.status_evidence_invalid"),
            )?
        } else {
            stage(
                VerificationStageName::Status,
                VerificationStageStatus::NotChecked,
                Some("sd_jwt_vc.prerequisite_failed"),
            )?
        },
        match failure {
            FailureStage::Profile(ProfileFailureStage::Schema) => stage(
                VerificationStageName::Schema,
                VerificationStageStatus::Failed,
                Some("sd_jwt_vc.metadata_invalid"),
            )?,
            FailureStage::Profile(ProfileFailureStage::Status)
                if evidence.type_metadata.is_some() =>
            {
                stage(
                    VerificationStageName::Schema,
                    VerificationStageStatus::Passed,
                    None,
                )?
            }
            _ => stage(
                VerificationStageName::Schema,
                VerificationStageStatus::NotChecked,
                Some("sd_jwt_vc.prerequisite_failed"),
            )?,
        },
    ])
}
