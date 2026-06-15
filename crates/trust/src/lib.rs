//! Trust framework and policy boundaries.
//!
//! This crate owns DID trust, X.509/IACA roots, `OpenID` Federation, EUDI
//! trusted lists, wallet attestations, status-list trust, and verifier policy
//! evaluation.

use identus_core::{CapabilityId, ErrorCode, ErrorKind, IdentusError, IdentusResult};

/// Component metadata.
pub const COMPONENT: identus_core::Component = identus_core::Component {
    name: "identus-trust",
    summary: "Trust anchors, federation, attestations, status, and policy.",
};

/// Stable capability id for trust and status errors.
pub const TRUST_CAPABILITY: CapabilityId = CapabilityId::new("trust");

/// Credential or presentation status state.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum StatusState {
    /// Status is active and acceptable.
    Active,
    /// Status is suspended by issuer or ecosystem policy.
    Suspended,
    /// Status is revoked.
    Revoked,
    /// Status cannot be determined from available evidence.
    Unknown,
    /// Status source is temporarily unavailable.
    Unavailable,
    /// Status mechanism is unsupported by the current feature set.
    Unsupported,
}

/// Status purpose requested by a credential or presentation.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum StatusPurpose {
    /// Revocation status.
    Revocation,
    /// Suspension status.
    Suspension,
    /// Message or protocol status.
    Message,
    /// Extension-defined purpose.
    Extension,
}

/// Trust/status mechanism family.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum StatusMechanism {
    /// W3C Bitstring Status List.
    BitstringStatusList,
    /// IETF Token Status List draft family.
    TokenStatusList,
    /// `AnonCreds` revocation registry.
    AnonCredsRevocation,
    /// `OpenID` Federation trust chain or trust mark.
    OpenIdFederation,
    /// X.509 chain and profile evaluation.
    X509,
    /// IACA trust anchor evaluation for mdoc.
    Iaca,
    /// Local allow/deny policy.
    LocalPolicy,
}

/// Typed reference to status material.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StatusReference {
    /// Status mechanism.
    pub mechanism: StatusMechanism,
    /// Status purpose.
    pub purpose: StatusPurpose,
    /// Redacted fixture or source identifier.
    pub reference_id: String,
}

/// Resolved status material for Docker-free policy tests.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StatusMaterial {
    /// Reference that produced the material.
    pub reference: StatusReference,
    /// Current status state.
    pub state: StatusState,
    /// Synthetic freshness marker.
    pub freshness_marker: &'static str,
}

/// Decision returned by status verification.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StatusDecision {
    /// Status state.
    pub state: StatusState,
    /// Whether verification policy accepts this state.
    pub accepted: bool,
    /// Stable typed reason code.
    pub reason_code: &'static str,
}

/// Typed trust anchor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrustAnchor {
    /// Anchor id.
    pub id: String,
    /// Mechanism family.
    pub mechanism: StatusMechanism,
    /// Whether the anchor is currently trusted.
    pub trusted: bool,
}

/// Typed trust chain.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrustChain {
    /// Subject being evaluated.
    pub subject: String,
    /// Ordered anchor ids or intermediate evidence ids.
    pub anchors: Vec<String>,
    /// Mechanism family.
    pub mechanism: StatusMechanism,
}

/// Trust-chain decision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrustDecision {
    /// Whether the chain is trusted.
    pub trusted: bool,
    /// Stable typed reason code.
    pub reason_code: &'static str,
}

/// Policy input shared by credential, presentation, and `OpenID4VC` paths.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrustPolicyInput {
    /// Subject being evaluated.
    pub subject: String,
    /// Optional status reference.
    pub status: Option<StatusReference>,
    /// Optional trust chain.
    pub trust_chain: Option<TrustChain>,
    /// Whether status checks are required.
    pub require_status: bool,
    /// Whether trust checks are required.
    pub require_trust: bool,
}

/// Combined trust/status policy decision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrustPolicyDecision {
    /// Whether the request is accepted by policy.
    pub accepted: bool,
    /// Status decision, when evaluated.
    pub status: Option<StatusDecision>,
    /// Trust decision, when evaluated.
    pub trust: Option<TrustDecision>,
}

/// Resolve status material without forcing network or Docker dependencies.
pub trait StatusResolver {
    /// Resolve status material by typed reference.
    ///
    /// # Errors
    ///
    /// Returns a typed trust error when material is unavailable or unsupported.
    fn resolve_status(&self, reference: &StatusReference) -> IdentusResult<StatusMaterial>;
}

/// Verify resolved status material against policy.
pub trait StatusVerifier {
    /// Evaluate status material.
    ///
    /// # Errors
    ///
    /// Returns a typed trust error when status policy rejects the material.
    fn verify_status(&self, material: &StatusMaterial) -> IdentusResult<StatusDecision>;
}

/// Resolve trust anchors.
pub trait TrustAnchorResolver {
    /// Resolve an anchor by id.
    ///
    /// # Errors
    ///
    /// Returns `untrusted_anchor` when the anchor is absent or rejected.
    fn resolve_anchor(&self, anchor_id: &str) -> IdentusResult<TrustAnchor>;
}

/// Verify trust chains.
pub trait TrustChainVerifier {
    /// Verify a trust chain.
    ///
    /// # Errors
    ///
    /// Returns `trust_chain_rejected` when chain structure or policy fails.
    fn verify_trust_chain(&self, chain: &TrustChain) -> IdentusResult<TrustDecision>;
}

/// Combined policy engine used by higher-level verification paths.
pub trait TrustPolicyEngine {
    /// Evaluate status and trust inputs into one policy decision.
    ///
    /// # Errors
    ///
    /// Returns typed trust errors for rejected required status or trust checks.
    fn evaluate(&self, input: &TrustPolicyInput) -> IdentusResult<TrustPolicyDecision>;
}

/// Evidence cache for status material and trust chains.
pub trait TrustEvidenceStore {
    /// Store resolved status material.
    ///
    /// # Errors
    ///
    /// Returns a typed trust error if evidence cannot be stored.
    fn put_status_material(&mut self, material: StatusMaterial) -> IdentusResult<()>;

    /// Store trust anchor evidence.
    ///
    /// # Errors
    ///
    /// Returns a typed trust error if evidence cannot be stored.
    fn put_trust_anchor(&mut self, anchor: TrustAnchor) -> IdentusResult<()>;
}

/// Docker-free policy fixture used by default tests and conformance.
#[derive(Clone, Debug, Default)]
pub struct InMemoryTrustRegistry {
    statuses: Vec<StatusMaterial>,
    anchors: Vec<TrustAnchor>,
}

impl InMemoryTrustRegistry {
    /// Create an empty registry.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            statuses: Vec::new(),
            anchors: Vec::new(),
        }
    }

    /// Create a registry with the default Docker-free policy fixtures.
    #[must_use]
    pub fn with_default_fixtures() -> Self {
        let mut registry = Self::new();
        for mechanism in [
            StatusMechanism::BitstringStatusList,
            StatusMechanism::TokenStatusList,
            StatusMechanism::AnonCredsRevocation,
            StatusMechanism::OpenIdFederation,
            StatusMechanism::X509,
            StatusMechanism::Iaca,
        ] {
            registry.statuses.push(StatusMaterial {
                reference: StatusReference {
                    mechanism,
                    purpose: StatusPurpose::Revocation,
                    reference_id: format!("fixture-{mechanism:?}"),
                },
                state: StatusState::Active,
                freshness_marker: "docker-free",
            });
            registry.anchors.push(TrustAnchor {
                id: format!("fixture-anchor-{mechanism:?}"),
                mechanism,
                trusted: true,
            });
        }
        registry
    }
}

impl TrustEvidenceStore for InMemoryTrustRegistry {
    fn put_status_material(&mut self, material: StatusMaterial) -> IdentusResult<()> {
        if material.reference.reference_id.is_empty() {
            return Err(invalid_trust_input());
        }
        self.statuses.push(material);
        Ok(())
    }

    fn put_trust_anchor(&mut self, anchor: TrustAnchor) -> IdentusResult<()> {
        if anchor.id.is_empty() {
            return Err(invalid_trust_input());
        }
        self.anchors.push(anchor);
        Ok(())
    }
}

impl StatusResolver for InMemoryTrustRegistry {
    fn resolve_status(&self, reference: &StatusReference) -> IdentusResult<StatusMaterial> {
        self.statuses
            .iter()
            .find(|material| material.reference == *reference)
            .cloned()
            .ok_or_else(status_unavailable)
    }
}

impl StatusVerifier for InMemoryTrustRegistry {
    fn verify_status(&self, material: &StatusMaterial) -> IdentusResult<StatusDecision> {
        match material.state {
            StatusState::Active => Ok(StatusDecision {
                state: material.state,
                accepted: true,
                reason_code: "status_active",
            }),
            StatusState::Suspended => Err(credential_suspended()),
            StatusState::Revoked => Err(credential_revoked()),
            StatusState::Unknown | StatusState::Unavailable => Err(status_unavailable()),
            StatusState::Unsupported => Err(unsupported_status_mechanism()),
        }
    }
}

impl TrustAnchorResolver for InMemoryTrustRegistry {
    fn resolve_anchor(&self, anchor_id: &str) -> IdentusResult<TrustAnchor> {
        self.anchors
            .iter()
            .find(|anchor| anchor.id == anchor_id && anchor.trusted)
            .cloned()
            .ok_or_else(untrusted_anchor)
    }
}

impl TrustChainVerifier for InMemoryTrustRegistry {
    fn verify_trust_chain(&self, chain: &TrustChain) -> IdentusResult<TrustDecision> {
        if chain.anchors.is_empty() {
            return Err(trust_chain_rejected());
        }

        for anchor_id in &chain.anchors {
            self.resolve_anchor(anchor_id)?;
        }

        Ok(TrustDecision {
            trusted: true,
            reason_code: "trust_chain_accepted",
        })
    }
}

impl TrustPolicyEngine for InMemoryTrustRegistry {
    fn evaluate(&self, input: &TrustPolicyInput) -> IdentusResult<TrustPolicyDecision> {
        let status = match (&input.status, input.require_status) {
            (Some(reference), _) => {
                let material = self.resolve_status(reference)?;
                Some(self.verify_status(&material)?)
            }
            (None, true) => return Err(status_unavailable()),
            (None, false) => None,
        };

        let trust = match (&input.trust_chain, input.require_trust) {
            (Some(chain), _) => Some(self.verify_trust_chain(chain)?),
            (None, true) => return Err(untrusted_anchor()),
            (None, false) => None,
        };

        Ok(TrustPolicyDecision {
            accepted: status.as_ref().is_none_or(|decision| decision.accepted)
                && trust.as_ref().is_none_or(|decision| decision.trusted),
            status,
            trust,
        })
    }
}

/// Create a redaction-safe trust error.
#[must_use]
pub const fn trust_error(
    code: ErrorCode,
    kind: ErrorKind,
    public_message: &'static str,
) -> IdentusError {
    IdentusError::public(code, kind, TRUST_CAPABILITY, public_message)
}

/// Status material was unavailable.
#[must_use]
pub const fn status_unavailable() -> IdentusError {
    trust_error(
        ErrorCode::new("status_unavailable"),
        ErrorKind::Trust,
        "status evidence is unavailable",
    )
}

/// Credential status is revoked.
#[must_use]
pub const fn credential_revoked() -> IdentusError {
    trust_error(
        ErrorCode::new("credential_revoked"),
        ErrorKind::VerificationFailed,
        "credential status rejected verification",
    )
}

/// Credential status is suspended.
#[must_use]
pub const fn credential_suspended() -> IdentusError {
    trust_error(
        ErrorCode::new("credential_suspended"),
        ErrorKind::VerificationFailed,
        "credential status is suspended",
    )
}

/// Status mechanism is unsupported.
#[must_use]
pub const fn unsupported_status_mechanism() -> IdentusError {
    trust_error(
        ErrorCode::new("unsupported_status_mechanism"),
        ErrorKind::Unsupported,
        "status mechanism is unsupported",
    )
}

/// Trust anchor was absent or rejected.
#[must_use]
pub const fn untrusted_anchor() -> IdentusError {
    trust_error(
        ErrorCode::new("untrusted_anchor"),
        ErrorKind::Trust,
        "trust anchor is not trusted",
    )
}

/// Trust chain was rejected.
#[must_use]
pub const fn trust_chain_rejected() -> IdentusError {
    trust_error(
        ErrorCode::new("trust_chain_rejected"),
        ErrorKind::Trust,
        "trust chain was rejected",
    )
}

/// Trust input was invalid.
#[must_use]
pub const fn invalid_trust_input() -> IdentusError {
    trust_error(
        ErrorCode::new("invalid_trust_input"),
        ErrorKind::InvalidInput,
        "trust input is invalid",
    )
}

#[cfg(test)]
mod tests {
    use identus_core::CapabilityId;

    use super::{
        InMemoryTrustRegistry, StatusMechanism, StatusPurpose, StatusReference, StatusState,
        TrustAnchor, TrustChain, TrustEvidenceStore, TrustPolicyEngine, TrustPolicyInput,
        credential_revoked,
    };

    #[test]
    fn default_trust_fixtures_cover_status_mechanisms() {
        let registry = InMemoryTrustRegistry::with_default_fixtures();
        for mechanism in [
            StatusMechanism::BitstringStatusList,
            StatusMechanism::TokenStatusList,
            StatusMechanism::AnonCredsRevocation,
            StatusMechanism::OpenIdFederation,
            StatusMechanism::X509,
            StatusMechanism::Iaca,
        ] {
            let reference = StatusReference {
                mechanism,
                purpose: StatusPurpose::Revocation,
                reference_id: format!("fixture-{mechanism:?}"),
            };
            let input = TrustPolicyInput {
                subject: "did:example:holder".to_owned(),
                status: Some(reference),
                trust_chain: Some(TrustChain {
                    subject: "did:example:issuer".to_owned(),
                    anchors: vec![format!("fixture-anchor-{mechanism:?}")],
                    mechanism,
                }),
                require_status: true,
                require_trust: true,
            };

            assert!(
                registry
                    .evaluate(&input)
                    .expect("default fixture should evaluate")
                    .accepted
            );
        }
    }

    #[test]
    fn revoked_status_returns_redaction_safe_typed_error() {
        let mut registry = InMemoryTrustRegistry::new();
        registry
            .put_status_material(super::StatusMaterial {
                reference: StatusReference {
                    mechanism: StatusMechanism::BitstringStatusList,
                    purpose: StatusPurpose::Revocation,
                    reference_id: "revoked-fixture".to_owned(),
                },
                state: StatusState::Revoked,
                freshness_marker: "docker-free",
            })
            .expect("status material should store");

        let error = registry
            .evaluate(&TrustPolicyInput {
                subject: "did:example:holder".to_owned(),
                status: Some(StatusReference {
                    mechanism: StatusMechanism::BitstringStatusList,
                    purpose: StatusPurpose::Revocation,
                    reference_id: "revoked-fixture".to_owned(),
                }),
                trust_chain: None,
                require_status: true,
                require_trust: false,
            })
            .expect_err("revoked status should reject");

        assert_eq!(error, credential_revoked());
        assert_eq!(error.capability().map(CapabilityId::as_str), Some("trust"));
        assert!(!error.to_string().contains("did:example:holder"));
    }

    #[test]
    fn trust_chain_requires_trusted_anchor() {
        let mut registry = InMemoryTrustRegistry::new();
        registry
            .put_trust_anchor(TrustAnchor {
                id: "trusted-anchor".to_owned(),
                mechanism: StatusMechanism::OpenIdFederation,
                trusted: true,
            })
            .expect("anchor should store");

        let accepted = registry
            .evaluate(&TrustPolicyInput {
                subject: "openid-federation-issuer".to_owned(),
                status: None,
                trust_chain: Some(TrustChain {
                    subject: "openid-federation-issuer".to_owned(),
                    anchors: vec!["trusted-anchor".to_owned()],
                    mechanism: StatusMechanism::OpenIdFederation,
                }),
                require_status: false,
                require_trust: true,
            })
            .expect("trusted anchor should accept");
        assert!(accepted.accepted);

        let rejected = registry
            .evaluate(&TrustPolicyInput {
                subject: "openid-federation-issuer".to_owned(),
                status: None,
                trust_chain: Some(TrustChain {
                    subject: "openid-federation-issuer".to_owned(),
                    anchors: vec!["unknown-anchor".to_owned()],
                    mechanism: StatusMechanism::OpenIdFederation,
                }),
                require_status: false,
                require_trust: true,
            })
            .expect_err("unknown anchor should reject");
        assert_eq!(rejected.code().as_str(), "untrusted_anchor");
    }
}
