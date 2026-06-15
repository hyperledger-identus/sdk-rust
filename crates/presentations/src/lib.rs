//! Presentation exchange and proof boundaries.
//!
//! This crate will own Presentation Exchange, `DCQL` mapping,
//! credential selection, disclosure framing, challenge/domain checks,
//! and verifier response models.

/// Component metadata.
pub const COMPONENT: identus_core::Component = identus_core::Component {
    name: "identus-presentations",
    summary: "Presentation request, selection, exchange, and verification.",
};

use identus_core::{CapabilityId, ErrorCode, ErrorKind, IdentusError, IdentusResult};
use identus_trust::{TrustPolicyDecision, TrustPolicyEngine, TrustPolicyInput};

/// Stable capability id for presentation verification errors.
pub const PRESENTATION_CAPABILITY: CapabilityId = CapabilityId::new("presentation");

/// Negative presentation verification case from conformance fixtures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PresentationVerificationCase<'a> {
    /// Stable fixture case id.
    pub case_id: &'a str,
    /// Credential or presentation format recorded by the fixture.
    pub format: &'a str,
    /// Expected stable typed error code.
    pub expected_error: &'a str,
}

/// Docker-free presentation verification replay port.
pub trait PresentationVerificationReplay {
    /// Replay one negative verification case.
    ///
    /// # Errors
    ///
    /// Returns the expected typed verification error for known negative cases.
    fn replay_presentation_negative_case(
        &self,
        case: &PresentationVerificationCase<'_>,
    ) -> IdentusResult<()>;
}

/// Deterministic fixture policy for presentation negative-case replay.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FixturePresentationVerificationPolicy;

impl PresentationVerificationReplay for FixturePresentationVerificationPolicy {
    fn replay_presentation_negative_case(
        &self,
        case: &PresentationVerificationCase<'_>,
    ) -> IdentusResult<()> {
        let expected = expected_presentation_negative_error(case.case_id)
            .ok_or_else(unsupported_verification_case)?;
        if expected != case.expected_error {
            return Err(invalid_verification_fixture());
        }

        Err(presentation_verification_error(expected))
    }
}

/// Return the expected typed error for a stable presentation negative-case id.
#[must_use]
pub const fn expected_presentation_negative_error(case_id: &str) -> Option<&'static str> {
    match case_id.as_bytes() {
        b"presentation-wrong-audience" => Some("audience_mismatch"),
        b"presentation-wrong-domain" => Some("domain_mismatch"),
        b"presentation-wrong-challenge" => Some("challenge_mismatch"),
        b"missing-holder-binding" => Some("holder_binding_missing"),
        b"selective-disclosure-claim-missing" => Some("required_disclosure_missing"),
        _ => None,
    }
}

/// Create a redaction-safe presentation verification error for a fixture code.
#[must_use]
pub fn presentation_verification_error(code: &str) -> IdentusError {
    match code {
        "audience_mismatch" => presentation_error(
            "audience_mismatch",
            ErrorKind::VerificationFailed,
            "presentation audience does not match",
        ),
        "domain_mismatch" => presentation_error(
            "domain_mismatch",
            ErrorKind::VerificationFailed,
            "presentation domain does not match",
        ),
        "challenge_mismatch" => presentation_error(
            "challenge_mismatch",
            ErrorKind::VerificationFailed,
            "presentation challenge does not match",
        ),
        "holder_binding_missing" => presentation_error(
            "holder_binding_missing",
            ErrorKind::VerificationFailed,
            "presentation holder binding is missing",
        ),
        "required_disclosure_missing" => presentation_error(
            "required_disclosure_missing",
            ErrorKind::VerificationFailed,
            "required disclosure is missing",
        ),
        _ => invalid_verification_fixture(),
    }
}

fn presentation_error(
    code: &'static str,
    kind: ErrorKind,
    public_message: &'static str,
) -> IdentusError {
    IdentusError::public(
        ErrorCode::new(code),
        kind,
        PRESENTATION_CAPABILITY,
        public_message,
    )
}

fn unsupported_verification_case() -> IdentusError {
    presentation_error(
        "unsupported_verification_case",
        ErrorKind::Unsupported,
        "verification case is unsupported",
    )
}

fn invalid_verification_fixture() -> IdentusError {
    presentation_error(
        "invalid_verification_fixture",
        ErrorKind::InvalidInput,
        "verification fixture is invalid",
    )
}

/// Evaluate presentation trust and status policy through `identus-trust`.
///
/// # Errors
///
/// Returns typed trust errors from the supplied policy engine.
pub fn evaluate_presentation_trust(
    policy_engine: &impl TrustPolicyEngine,
    input: &TrustPolicyInput,
) -> IdentusResult<TrustPolicyDecision> {
    policy_engine.evaluate(input)
}

#[cfg(test)]
mod tests {
    use identus_trust::{
        InMemoryTrustRegistry, StatusMechanism, StatusPurpose, StatusReference, TrustPolicyInput,
    };

    use super::{
        FixturePresentationVerificationPolicy, PresentationVerificationCase,
        PresentationVerificationReplay, evaluate_presentation_trust,
    };

    #[test]
    fn presentation_trust_delegates_to_trust_policy_engine() {
        let policy_engine = InMemoryTrustRegistry::with_default_fixtures();
        let decision = evaluate_presentation_trust(
            &policy_engine,
            &TrustPolicyInput {
                subject: "did:example:presentation".to_owned(),
                status: Some(StatusReference {
                    mechanism: StatusMechanism::TokenStatusList,
                    purpose: StatusPurpose::Revocation,
                    reference_id: "fixture-TokenStatusList".to_owned(),
                }),
                trust_chain: None,
                require_status: true,
                require_trust: false,
            },
        )
        .expect("default trust fixture should accept presentation");

        assert!(decision.accepted);
    }

    #[test]
    fn presentation_negative_cases_replay_expected_errors() {
        let policy = FixturePresentationVerificationPolicy;
        for (case_id, expected_error) in [
            ("presentation-wrong-audience", "audience_mismatch"),
            ("presentation-wrong-domain", "domain_mismatch"),
            ("presentation-wrong-challenge", "challenge_mismatch"),
            ("missing-holder-binding", "holder_binding_missing"),
            (
                "selective-disclosure-claim-missing",
                "required_disclosure_missing",
            ),
        ] {
            let error = policy
                .replay_presentation_negative_case(&PresentationVerificationCase {
                    case_id,
                    format: "jwt_vp_json",
                    expected_error,
                })
                .expect_err("negative fixture must reject");
            assert_eq!(error.code().as_str(), expected_error);
            assert!(!error.to_string().contains("redacted"));
        }
    }
}
