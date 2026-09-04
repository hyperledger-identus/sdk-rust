use std::{fmt, str::FromStr};

use crate::CredentialError;

/// Number of policy-neutral stages in a complete verification report.
pub const VERIFICATION_STAGE_COUNT: usize = 6;
/// Maximum encoded length of a verification reason code.
pub const MAX_VERIFICATION_REASON_CODE_BYTES: usize = 128;

/// Policy-neutral verification stages in canonical report order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VerificationStageName {
    /// Credential structure and required fields.
    Structural,
    /// Issuer identity and verification-key resolution.
    IssuerKey,
    /// Credential proof or signature.
    Proof,
    /// Issuance, activation, and expiry constraints.
    Temporal,
    /// Revocation, suspension, or other credential status.
    Status,
    /// Schema conformance.
    Schema,
}

impl VerificationStageName {
    /// Every stage in canonical report order.
    pub const ALL: [Self; VERIFICATION_STAGE_COUNT] = [
        Self::Structural,
        Self::IssuerKey,
        Self::Proof,
        Self::Temporal,
        Self::Status,
        Self::Schema,
    ];

    /// Return the stable machine spelling.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Structural => "structural",
            Self::IssuerKey => "issuer_key",
            Self::Proof => "proof",
            Self::Temporal => "temporal",
            Self::Status => "status",
            Self::Schema => "schema",
        }
    }

    const fn index(self) -> usize {
        match self {
            Self::Structural => 0,
            Self::IssuerKey => 1,
            Self::Proof => 2,
            Self::Temporal => 3,
            Self::Status => 4,
            Self::Schema => 5,
        }
    }
}

impl fmt::Display for VerificationStageName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Evidence result for one verification stage.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VerificationStageStatus {
    /// The contracted check completed successfully.
    Passed,
    /// The check completed and found invalid evidence.
    Failed,
    /// The check did not produce a validity result.
    NotChecked,
}

impl VerificationStageStatus {
    /// Return the stable machine spelling.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::NotChecked => "not_checked",
        }
    }
}

/// Bounded machine reason for a failed or not-checked stage.
#[must_use]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VerificationReasonCode(String);

impl VerificationReasonCode {
    /// Parse and own a reason code after validating the borrowed input.
    pub fn parse(value: &str) -> Result<Self, CredentialError> {
        let bytes = value.as_bytes();
        let Some((first, rest)) = bytes.split_first() else {
            return Err(CredentialError::InvalidVerificationReasonCode);
        };

        let is_alphanumeric = |byte: &u8| byte.is_ascii_lowercase() || byte.is_ascii_digit();
        if bytes.len() > MAX_VERIFICATION_REASON_CODE_BYTES
            || !is_alphanumeric(first)
            || !rest
                .iter()
                .all(|byte| is_alphanumeric(byte) || matches!(byte, b'.' | b'_' | b'-' | b':'))
        {
            return Err(CredentialError::InvalidVerificationReasonCode);
        }

        Ok(Self(value.to_owned()))
    }

    /// Return the exact validated reason spelling.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for VerificationReasonCode {
    type Err = CredentialError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl fmt::Debug for VerificationReasonCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("VerificationReasonCode")
            .field(&self.0)
            .finish()
    }
}

impl fmt::Display for VerificationReasonCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// One invariant-preserving stage result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerificationStage {
    name: VerificationStageName,
    status: VerificationStageStatus,
    reason: Option<VerificationReasonCode>,
}

impl VerificationStage {
    /// Construct a stage while enforcing status/reason consistency.
    pub fn new(
        name: VerificationStageName,
        status: VerificationStageStatus,
        reason: Option<VerificationReasonCode>,
    ) -> Result<Self, CredentialError> {
        match (status, reason.is_some()) {
            (VerificationStageStatus::Passed, true) => {
                return Err(CredentialError::UnexpectedVerificationReason);
            }
            (VerificationStageStatus::Failed | VerificationStageStatus::NotChecked, false) => {
                return Err(CredentialError::MissingVerificationReason);
            }
            _ => {}
        }

        Ok(Self {
            name,
            status,
            reason,
        })
    }

    /// Return the stage name.
    pub const fn name(&self) -> VerificationStageName {
        self.name
    }

    /// Return the stage status.
    pub const fn status(&self) -> VerificationStageStatus {
        self.status
    }

    /// Return the machine reason when required by the status.
    pub fn reason(&self) -> Option<&VerificationReasonCode> {
        self.reason.as_ref()
    }
}

/// Aggregate validity of the verification evidence, independent of trust.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VerificationOutcome {
    /// Every contracted stage passed.
    Valid,
    /// At least one stage found invalid evidence.
    Invalid,
    /// No stage failed, but at least one stage was not checked.
    Indeterminate,
}

impl VerificationOutcome {
    /// Return the stable machine spelling.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Valid => "valid",
            Self::Invalid => "invalid",
            Self::Indeterminate => "indeterminate",
        }
    }
}

/// Complete canonical verification evidence without a trust decision.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerificationReport {
    stages: [VerificationStage; VERIFICATION_STAGE_COUNT],
    outcome: VerificationOutcome,
}

impl VerificationReport {
    /// Construct a complete report from stages in canonical order.
    pub fn new(
        stages: [VerificationStage; VERIFICATION_STAGE_COUNT],
    ) -> Result<Self, CredentialError> {
        let mut has_failed = false;
        let mut has_not_checked = false;
        for (stage, expected) in stages.iter().zip(VerificationStageName::ALL) {
            if stage.name() != expected {
                return Err(CredentialError::NonCanonicalVerificationReport);
            }
            match stage.status() {
                VerificationStageStatus::Passed => {}
                VerificationStageStatus::Failed => has_failed = true,
                VerificationStageStatus::NotChecked => has_not_checked = true,
            }
        }

        let outcome = if has_failed {
            VerificationOutcome::Invalid
        } else if has_not_checked {
            VerificationOutcome::Indeterminate
        } else {
            VerificationOutcome::Valid
        };

        Ok(Self { stages, outcome })
    }

    /// Return the derived aggregate evidence outcome.
    pub const fn outcome(&self) -> VerificationOutcome {
        self.outcome
    }

    /// Return all stages in canonical order.
    pub const fn stages(&self) -> &[VerificationStage; VERIFICATION_STAGE_COUNT] {
        &self.stages
    }

    /// Return one stage through its canonical array index.
    pub fn stage(&self, name: VerificationStageName) -> &VerificationStage {
        &self.stages[name.index()]
    }
}
