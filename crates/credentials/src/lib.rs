//! Credential models and verification boundaries for legacy SDK parity.
//!
//! This crate will own `W3C VC`, `JWT VC`, `SD-JWT VC`, `AnonCreds`,
//! `OpenBadges`, `mdoc` credential model integration, and status-aware
//! verification ports.

/// Component metadata.
pub const COMPONENT: identus_core::Component = identus_core::Component {
    name: "identus-credentials",
    summary: "Format-pluggable credential models, issuance, and verification.",
};

use identus_core::{CapabilityId, ErrorCode, ErrorKind, IdentusError, IdentusResult};
use identus_trust::{TrustPolicyDecision, TrustPolicyEngine, TrustPolicyInput};

/// Stable capability id for credential verification errors.
pub const CREDENTIAL_CAPABILITY: CapabilityId = CapabilityId::new("credential");

/// Stable credential format id used by protocol crates and bindings.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CredentialFormatId(&'static str);

impl CredentialFormatId {
    /// Create a credential format id.
    #[must_use]
    pub const fn new(value: &'static str) -> Self {
        Self(value)
    }

    /// Borrow the stable id.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

/// Credential proof/model family.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CredentialFormatFamily {
    /// W3C Verifiable Credential data model.
    W3cVc,
    /// JWT secured credential.
    Jwt,
    /// Selective Disclosure JWT credential.
    SdJwt,
    /// `AnonCreds` credential.
    AnonCreds,
    /// Open Badges credential profile.
    OpenBadges,
    /// ISO mdoc credential.
    IsoMdoc,
}

/// Credential format support stage.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CredentialFormatStage {
    /// Required for current Identus SDK parity.
    Parity,
    /// Required for the target Rust SDK roadmap.
    Roadmap,
    /// Optional or infrastructure-gated format.
    Optional,
}

/// One credential format profile.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CredentialFormatProfile {
    /// Stable format id used by fixtures and protocol messages.
    pub id: CredentialFormatId,
    /// Format family.
    pub family: CredentialFormatFamily,
    /// Human-readable name.
    pub name: &'static str,
    /// Specification id from conformance catalog when applicable.
    pub specification_id: &'static str,
    /// Planned support stage.
    pub stage: CredentialFormatStage,
    /// Whether this format can be offered or issued over `DIDComm`.
    pub didcomm_supported: bool,
    /// Whether this format can be offered or issued over `OpenID4VCI`.
    pub openid4vci_supported: bool,
    /// Whether this format can be stored by the wallet record model.
    pub wallet_supported: bool,
}

impl CredentialFormatProfile {
    /// Stable profile id.
    #[must_use]
    pub const fn id(&self) -> CredentialFormatId {
        self.id
    }
}

/// Format-neutral credential descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CredentialDescriptor<'a> {
    /// Stable application or fixture credential id.
    pub id: &'a str,
    /// Credential format.
    pub format: CredentialFormatId,
    /// Issuer identifier as a DID, URL, or profile-specific reference.
    pub issuer: &'a str,
    /// Subject identifier as a DID, URL, or profile-specific reference.
    pub subject: &'a str,
    /// Schema, claim set, or document type reference.
    pub schema: Option<&'a str>,
}

/// Capability exposed by a credential format registry.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CredentialFormatCapability {
    /// Format can be carried by `DIDComm` issue-credential.
    DidCommIssueCredential,
    /// Format can be carried by `OpenID4VCI`.
    OpenId4VcIssuance,
    /// Format can be stored by wallet records.
    WalletStorage,
}

/// Registry for credential formats.
pub trait CredentialFormatRegistry {
    /// Return all known credential format profiles.
    fn profiles(&self) -> &'static [CredentialFormatProfile];

    /// Find a format profile by stable id.
    fn find(&self, id: CredentialFormatId) -> Option<&'static CredentialFormatProfile> {
        self.profiles().iter().find(|profile| profile.id == id)
    }

    /// Return whether a format supports a capability.
    fn supports(&self, id: CredentialFormatId, capability: CredentialFormatCapability) -> bool {
        self.find(id).is_some_and(|profile| match capability {
            CredentialFormatCapability::DidCommIssueCredential => profile.didcomm_supported,
            CredentialFormatCapability::OpenId4VcIssuance => profile.openid4vci_supported,
            CredentialFormatCapability::WalletStorage => profile.wallet_supported,
        })
    }
}

/// Static registry covering current parity and roadmap credential formats.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StaticCredentialFormatRegistry;

impl CredentialFormatRegistry for StaticCredentialFormatRegistry {
    fn profiles(&self) -> &'static [CredentialFormatProfile] {
        CREDENTIAL_FORMAT_PROFILES
    }
}

/// Credential formats tracked by `sdk-rust`.
pub const CREDENTIAL_FORMAT_PROFILES: &[CredentialFormatProfile] = &[
    credential_format(
        "jwt_vc_json",
        CredentialFormatFamily::Jwt,
        "JWT VC JSON",
        "jwt-vc",
        CredentialFormatStage::Parity,
        true,
        true,
        true,
    ),
    credential_format(
        "sd_jwt_vc",
        CredentialFormatFamily::SdJwt,
        "SD-JWT VC",
        "sd-jwt-vc",
        CredentialFormatStage::Parity,
        true,
        true,
        true,
    ),
    credential_format(
        "w3c_vc_json",
        CredentialFormatFamily::W3cVc,
        "W3C VC JSON",
        "vc-data-model",
        CredentialFormatStage::Parity,
        true,
        false,
        true,
    ),
    credential_format(
        "anoncreds",
        CredentialFormatFamily::AnonCreds,
        "AnonCreds",
        "anoncreds-v1",
        CredentialFormatStage::Parity,
        true,
        false,
        true,
    ),
    credential_format(
        "openbadges_3",
        CredentialFormatFamily::OpenBadges,
        "Open Badges 3.0",
        "openbadges-3",
        CredentialFormatStage::Roadmap,
        false,
        true,
        true,
    ),
    credential_format(
        "iso_mdoc",
        CredentialFormatFamily::IsoMdoc,
        "ISO mdoc",
        "iso-mdoc",
        CredentialFormatStage::Roadmap,
        false,
        true,
        true,
    ),
];

/// Negative credential verification case from conformance fixtures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CredentialVerificationCase<'a> {
    /// Stable fixture case id.
    pub case_id: &'a str,
    /// Credential or presentation format recorded by the fixture.
    pub format: &'a str,
    /// Expected stable typed error code.
    pub expected_error: &'a str,
}

/// Docker-free credential verification replay port.
pub trait CredentialVerificationReplay {
    /// Replay one negative verification case.
    ///
    /// # Errors
    ///
    /// Returns the expected typed verification error for known negative cases.
    fn replay_credential_negative_case(
        &self,
        case: &CredentialVerificationCase<'_>,
    ) -> IdentusResult<()>;
}

/// Deterministic fixture policy for credential negative-case replay.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FixtureCredentialVerificationPolicy;

impl CredentialVerificationReplay for FixtureCredentialVerificationPolicy {
    fn replay_credential_negative_case(
        &self,
        case: &CredentialVerificationCase<'_>,
    ) -> IdentusResult<()> {
        let expected = expected_credential_negative_error(case.case_id)
            .ok_or_else(unsupported_verification_case)?;
        if expected != case.expected_error {
            return Err(invalid_verification_fixture());
        }

        Err(credential_verification_error(expected))
    }
}

/// Return the expected typed error for a stable credential negative-case id.
#[must_use]
pub const fn expected_credential_negative_error(case_id: &str) -> Option<&'static str> {
    match case_id.as_bytes() {
        b"credential-expired" => Some("credential_expired"),
        b"credential-not-before" => Some("credential_not_yet_valid"),
        b"unsupported-key-purpose" => Some("unsupported_key_purpose"),
        b"status-revoked" => Some("credential_revoked"),
        b"tampered-signature" => Some("signature_verification_failed"),
        b"malformed-schema" => Some("schema_validation_failed"),
        b"unsupported-format" => Some("unsupported_credential_format"),
        _ => None,
    }
}

/// Create a redaction-safe credential verification error for a fixture code.
#[must_use]
pub fn credential_verification_error(code: &str) -> IdentusError {
    match code {
        "credential_expired" => credential_error(
            "credential_expired",
            ErrorKind::VerificationFailed,
            "credential has expired",
        ),
        "credential_not_yet_valid" => credential_error(
            "credential_not_yet_valid",
            ErrorKind::VerificationFailed,
            "credential is not yet valid",
        ),
        "unsupported_key_purpose" => credential_error(
            "unsupported_key_purpose",
            ErrorKind::VerificationFailed,
            "credential key purpose is unsupported",
        ),
        "credential_revoked" => identus_trust::credential_revoked(),
        "signature_verification_failed" => credential_error(
            "signature_verification_failed",
            ErrorKind::VerificationFailed,
            "credential signature verification failed",
        ),
        "schema_validation_failed" => credential_error(
            "schema_validation_failed",
            ErrorKind::VerificationFailed,
            "credential schema validation failed",
        ),
        "unsupported_credential_format" => credential_error(
            "unsupported_credential_format",
            ErrorKind::Unsupported,
            "credential format is unsupported",
        ),
        _ => invalid_verification_fixture(),
    }
}

fn credential_error(
    code: &'static str,
    kind: ErrorKind,
    public_message: &'static str,
) -> IdentusError {
    IdentusError::public(
        ErrorCode::new(code),
        kind,
        CREDENTIAL_CAPABILITY,
        public_message,
    )
}

fn unsupported_verification_case() -> IdentusError {
    credential_error(
        "unsupported_verification_case",
        ErrorKind::Unsupported,
        "verification case is unsupported",
    )
}

fn invalid_verification_fixture() -> IdentusError {
    credential_error(
        "invalid_verification_fixture",
        ErrorKind::InvalidInput,
        "verification fixture is invalid",
    )
}

#[allow(clippy::too_many_arguments)]
const fn credential_format(
    id: &'static str,
    family: CredentialFormatFamily,
    name: &'static str,
    specification_id: &'static str,
    stage: CredentialFormatStage,
    didcomm_supported: bool,
    openid4vci_supported: bool,
    wallet_supported: bool,
) -> CredentialFormatProfile {
    CredentialFormatProfile {
        id: CredentialFormatId::new(id),
        family,
        name,
        specification_id,
        stage,
        didcomm_supported,
        openid4vci_supported,
        wallet_supported,
    }
}

/// Evaluate credential trust and status policy through `identus-trust`.
///
/// # Errors
///
/// Returns typed trust errors from the supplied policy engine.
pub fn evaluate_credential_trust(
    policy_engine: &impl TrustPolicyEngine,
    input: &TrustPolicyInput,
) -> IdentusResult<TrustPolicyDecision> {
    policy_engine.evaluate(input)
}

#[cfg(test)]
mod tests {
    use identus_trust::{
        InMemoryTrustRegistry, StatusMechanism, StatusPurpose, StatusReference, TrustChain,
        TrustPolicyInput,
    };

    use super::{
        CredentialDescriptor, CredentialFormatCapability, CredentialFormatFamily,
        CredentialFormatId, CredentialFormatRegistry, CredentialFormatStage,
        CredentialVerificationCase, CredentialVerificationReplay,
        FixtureCredentialVerificationPolicy, StaticCredentialFormatRegistry,
        evaluate_credential_trust,
    };

    #[test]
    fn credential_trust_delegates_to_trust_policy_engine() {
        let policy_engine = InMemoryTrustRegistry::with_default_fixtures();
        let decision = evaluate_credential_trust(
            &policy_engine,
            &TrustPolicyInput {
                subject: "did:example:credential".to_owned(),
                status: Some(StatusReference {
                    mechanism: StatusMechanism::BitstringStatusList,
                    purpose: StatusPurpose::Revocation,
                    reference_id: "fixture-BitstringStatusList".to_owned(),
                }),
                trust_chain: Some(TrustChain {
                    subject: "did:example:issuer".to_owned(),
                    anchors: vec!["fixture-anchor-BitstringStatusList".to_owned()],
                    mechanism: StatusMechanism::BitstringStatusList,
                }),
                require_status: true,
                require_trust: true,
            },
        )
        .expect("default trust fixture should accept credential");

        assert!(decision.accepted);
    }

    #[test]
    fn credential_negative_cases_replay_expected_errors() {
        let policy = FixtureCredentialVerificationPolicy;
        for (case_id, expected_error) in [
            ("credential-expired", "credential_expired"),
            ("credential-not-before", "credential_not_yet_valid"),
            ("unsupported-key-purpose", "unsupported_key_purpose"),
            ("status-revoked", "credential_revoked"),
            ("tampered-signature", "signature_verification_failed"),
            ("malformed-schema", "schema_validation_failed"),
            ("unsupported-format", "unsupported_credential_format"),
        ] {
            let error = policy
                .replay_credential_negative_case(&CredentialVerificationCase {
                    case_id,
                    format: "jwt_vc_json",
                    expected_error,
                })
                .expect_err("negative fixture must reject");
            assert_eq!(error.code().as_str(), expected_error);
            assert!(!error.to_string().contains("redacted"));
        }
    }

    #[test]
    fn credential_format_registry_covers_parity_and_roadmap_formats() {
        let registry = StaticCredentialFormatRegistry;
        for (format_id, family, stage) in [
            (
                "jwt_vc_json",
                CredentialFormatFamily::Jwt,
                CredentialFormatStage::Parity,
            ),
            (
                "sd_jwt_vc",
                CredentialFormatFamily::SdJwt,
                CredentialFormatStage::Parity,
            ),
            (
                "w3c_vc_json",
                CredentialFormatFamily::W3cVc,
                CredentialFormatStage::Parity,
            ),
            (
                "anoncreds",
                CredentialFormatFamily::AnonCreds,
                CredentialFormatStage::Parity,
            ),
            (
                "openbadges_3",
                CredentialFormatFamily::OpenBadges,
                CredentialFormatStage::Roadmap,
            ),
            (
                "iso_mdoc",
                CredentialFormatFamily::IsoMdoc,
                CredentialFormatStage::Roadmap,
            ),
        ] {
            let profile = registry
                .find(CredentialFormatId::new(format_id))
                .unwrap_or_else(|| panic!("missing credential format {format_id}"));
            assert_eq!(profile.family, family);
            assert_eq!(profile.stage, stage);
            assert!(
                registry.supports(
                    CredentialFormatId::new(format_id),
                    CredentialFormatCapability::WalletStorage
                ),
                "{format_id} must support wallet storage"
            );
        }

        assert!(registry.supports(
            CredentialFormatId::new("jwt_vc_json"),
            CredentialFormatCapability::DidCommIssueCredential
        ));
        assert!(registry.supports(
            CredentialFormatId::new("sd_jwt_vc"),
            CredentialFormatCapability::OpenId4VcIssuance
        ));
        assert!(!registry.supports(
            CredentialFormatId::new("missing"),
            CredentialFormatCapability::WalletStorage
        ));
    }

    #[test]
    fn credential_descriptor_keeps_model_metadata_format_neutral() {
        let descriptor = CredentialDescriptor {
            id: "fixture-credential-1",
            format: CredentialFormatId::new("jwt_vc_json"),
            issuer: "did:example:issuer",
            subject: "did:example:holder",
            schema: Some("fixture-schema"),
        };

        assert_eq!(descriptor.format.as_str(), "jwt_vc_json");
        assert_eq!(descriptor.issuer, "did:example:issuer");
        assert_eq!(descriptor.schema, Some("fixture-schema"));
    }
}
