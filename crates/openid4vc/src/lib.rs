//! `OpenID` for Verifiable Credentials boundaries.
//!
//! This crate will own `OID4VCI`, `OID4VP`, `SIOPv2`, `HAIP` profile
//! constraints, `OpenID` Federation integration, credential offers,
//! authorization requests, request `URI` flows, and `direct_post` flows.

use identus_core::{CapabilityId, ErrorCode, ErrorKind, IdentusError, IdentusResult};
use identus_trust::{TrustPolicyDecision, TrustPolicyEngine, TrustPolicyInput};

/// Component metadata.
pub const COMPONENT: identus_core::Component = identus_core::Component {
    name: "identus-openid4vc",
    summary: "OID4VCI, OID4VP, SIOPv2, HAIP, and federation boundaries.",
};

/// Stable capability id for `OpenID4VC` errors.
pub const OPENID4VC_CAPABILITY: CapabilityId = CapabilityId::new("openid4vc");

/// OID4VCI credential issuance flow family.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CredentialIssuanceFlowKind {
    /// OAuth authorization-code issuance flow.
    AuthorizationCode,
    /// Pre-authorized-code issuance flow.
    PreAuthorizedCode,
    /// Deferred credential issuance flow.
    DeferredCredential,
}

/// Typed OID4VCI state.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CredentialIssuanceState {
    /// No OID4VCI message has been accepted.
    Initial,
    /// Credential issuer metadata has been resolved.
    MetadataResolved,
    /// Credential offer has been received by value or by reference.
    OfferReceived,
    /// Authorization request has been submitted.
    AuthorizationRequested,
    /// Token request has been submitted.
    TokenRequested,
    /// Token response has been received.
    TokenReceived,
    /// Credential request has been submitted.
    CredentialRequested,
    /// Credential has been issued.
    CredentialIssued,
    /// Deferred credential transaction is pending.
    DeferredCredentialPending,
    /// Deferred credential request has been submitted.
    DeferredCredentialRequested,
    /// Deferred credential is ready.
    DeferredCredentialReady,
}

impl CredentialIssuanceState {
    /// Stable fixture state id.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Initial => "oid4vci_initial",
            Self::MetadataResolved => "oid4vci_metadata_resolved",
            Self::OfferReceived => "oid4vci_offer_received",
            Self::AuthorizationRequested => "oid4vci_authorization_requested",
            Self::TokenRequested => "oid4vci_token_requested",
            Self::TokenReceived => "oid4vci_token_received",
            Self::CredentialRequested => "oid4vci_credential_requested",
            Self::CredentialIssued => "oid4vci_credential_issued",
            Self::DeferredCredentialPending => "oid4vci_deferred_credential_pending",
            Self::DeferredCredentialRequested => "oid4vci_deferred_credential_requested",
            Self::DeferredCredentialReady => "oid4vci_deferred_credential_ready",
        }
    }
}

/// Typed OID4VCI event accepted by the credential issuance state machine.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CredentialIssuanceEvent {
    /// Credential issuer metadata was resolved.
    CredentialIssuerMetadata,
    /// Credential offer was received by value.
    CredentialOfferByValue,
    /// Credential offer was received by reference.
    CredentialOfferByReference,
    /// Authorization request was submitted.
    AuthorizationRequest,
    /// Token request was submitted.
    TokenRequest,
    /// Token response was received.
    TokenResponse,
    /// Credential request was submitted.
    CredentialRequest,
    /// Credential response was received.
    CredentialResponse,
    /// Deferred credential response indicated pending state.
    DeferredCredentialResponsePending,
    /// Deferred credential request was submitted.
    DeferredCredentialRequest,
    /// Deferred credential response returned the credential.
    DeferredCredentialResponseReady,
}

impl CredentialIssuanceEvent {
    /// Build an OID4VCI event from a transcript message.
    #[must_use]
    pub fn from_transcript_message(message: &OpenId4VcTranscriptMessage<'_>) -> Option<Self> {
        let event = match (message.flow, message.message_type) {
            ("oid4vci_authorization_code", "credential_issuer_metadata") => {
                Self::CredentialIssuerMetadata
            }
            ("oid4vci_authorization_code", "credential_offer_by_value") => {
                Self::CredentialOfferByValue
            }
            ("oid4vci_authorization_code", "authorization_request") => Self::AuthorizationRequest,
            ("oid4vci_authorization_code", "token_response") => Self::TokenResponse,
            ("oid4vci_authorization_code", "credential_request") => Self::CredentialRequest,
            ("oid4vci_pre_authorized_code", "credential_offer_by_reference") => {
                Self::CredentialOfferByReference
            }
            ("oid4vci_pre_authorized_code", "token_request") => Self::TokenRequest,
            (
                "oid4vci_authorization_code" | "oid4vci_pre_authorized_code",
                "credential_response",
            ) => Self::CredentialResponse,
            ("oid4vci_deferred_credential", "deferred_credential_response_pending") => {
                Self::DeferredCredentialResponsePending
            }
            ("oid4vci_deferred_credential", "deferred_credential_request") => {
                Self::DeferredCredentialRequest
            }
            ("oid4vci_deferred_credential", "deferred_credential_response_ready") => {
                Self::DeferredCredentialResponseReady
            }
            _ => return None,
        };
        Some(event)
    }
}

/// One accepted OID4VCI transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CredentialIssuanceTransition {
    /// State before the event was applied.
    pub from: CredentialIssuanceState,
    /// Accepted event.
    pub event: CredentialIssuanceEvent,
    /// State after the event was applied.
    pub to: CredentialIssuanceState,
}

/// Stateful OID4VCI transition boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CredentialIssuanceStateMachine {
    flow: CredentialIssuanceFlowKind,
    state: CredentialIssuanceState,
}

impl CredentialIssuanceStateMachine {
    /// Create a state machine for an OID4VCI flow.
    #[must_use]
    pub const fn new(flow: CredentialIssuanceFlowKind) -> Self {
        Self {
            flow,
            state: CredentialIssuanceState::Initial,
        }
    }

    /// Return the configured flow kind.
    #[must_use]
    pub const fn flow(&self) -> CredentialIssuanceFlowKind {
        self.flow
    }

    /// Return the current OID4VCI state.
    #[must_use]
    pub const fn state(&self) -> CredentialIssuanceState {
        self.state
    }

    /// Apply one OID4VCI event.
    ///
    /// # Errors
    ///
    /// Returns a typed, redaction-safe conflict when the event is not valid
    /// for the configured flow and current state.
    pub fn apply(
        &mut self,
        event: CredentialIssuanceEvent,
    ) -> IdentusResult<CredentialIssuanceTransition> {
        let from = self.state;
        let Some(to) = self.next_state(event) else {
            return Err(invalid_openid4vc_transition());
        };
        self.state = to;
        Ok(CredentialIssuanceTransition { from, event, to })
    }

    fn next_state(&self, event: CredentialIssuanceEvent) -> Option<CredentialIssuanceState> {
        match (self.flow, self.state, event) {
            (
                CredentialIssuanceFlowKind::AuthorizationCode,
                CredentialIssuanceState::Initial,
                CredentialIssuanceEvent::CredentialIssuerMetadata,
            ) => Some(CredentialIssuanceState::MetadataResolved),
            (
                CredentialIssuanceFlowKind::AuthorizationCode,
                CredentialIssuanceState::MetadataResolved,
                CredentialIssuanceEvent::CredentialOfferByValue,
            )
            | (
                CredentialIssuanceFlowKind::PreAuthorizedCode,
                CredentialIssuanceState::Initial,
                CredentialIssuanceEvent::CredentialOfferByReference,
            ) => Some(CredentialIssuanceState::OfferReceived),
            (
                CredentialIssuanceFlowKind::AuthorizationCode,
                CredentialIssuanceState::OfferReceived,
                CredentialIssuanceEvent::AuthorizationRequest,
            ) => Some(CredentialIssuanceState::AuthorizationRequested),
            (
                CredentialIssuanceFlowKind::AuthorizationCode,
                CredentialIssuanceState::AuthorizationRequested,
                CredentialIssuanceEvent::TokenResponse,
            ) => Some(CredentialIssuanceState::TokenReceived),
            (
                CredentialIssuanceFlowKind::AuthorizationCode,
                CredentialIssuanceState::TokenReceived,
                CredentialIssuanceEvent::CredentialRequest,
            ) => Some(CredentialIssuanceState::CredentialRequested),
            (
                CredentialIssuanceFlowKind::AuthorizationCode,
                CredentialIssuanceState::CredentialRequested,
                CredentialIssuanceEvent::CredentialResponse,
            )
            | (
                CredentialIssuanceFlowKind::PreAuthorizedCode,
                CredentialIssuanceState::TokenRequested,
                CredentialIssuanceEvent::CredentialResponse,
            ) => Some(CredentialIssuanceState::CredentialIssued),
            (
                CredentialIssuanceFlowKind::PreAuthorizedCode,
                CredentialIssuanceState::OfferReceived,
                CredentialIssuanceEvent::TokenRequest,
            ) => Some(CredentialIssuanceState::TokenRequested),
            (
                CredentialIssuanceFlowKind::DeferredCredential,
                CredentialIssuanceState::Initial,
                CredentialIssuanceEvent::DeferredCredentialResponsePending,
            ) => Some(CredentialIssuanceState::DeferredCredentialPending),
            (
                CredentialIssuanceFlowKind::DeferredCredential,
                CredentialIssuanceState::DeferredCredentialPending,
                CredentialIssuanceEvent::DeferredCredentialRequest,
            ) => Some(CredentialIssuanceState::DeferredCredentialRequested),
            (
                CredentialIssuanceFlowKind::DeferredCredential,
                CredentialIssuanceState::DeferredCredentialRequested,
                CredentialIssuanceEvent::DeferredCredentialResponseReady,
            ) => Some(CredentialIssuanceState::DeferredCredentialReady),
            _ => None,
        }
    }
}

/// OID4VP presentation flow family.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PresentationFlowKind {
    /// Same-device `direct_post` presentation flow.
    SameDeviceDirectPost,
    /// Cross-device `direct_post.jwt` presentation flow.
    CrossDeviceDirectPost,
}

/// Typed OID4VP presentation state.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PresentationState {
    /// No OID4VP message has been accepted.
    Initial,
    /// Authorization request object has been validated.
    RequestValidated,
    /// Request `URI` has been resolved.
    RequestUriResolved,
    /// Authorization response has been submitted.
    ResponseSubmitted,
    /// Encrypted authorization response has been submitted.
    EncryptedResponseSubmitted,
}

impl PresentationState {
    /// Stable fixture state id.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Initial => "oid4vp_initial",
            Self::RequestValidated => "oid4vp_request_validated",
            Self::RequestUriResolved => "oid4vp_request_uri_resolved",
            Self::ResponseSubmitted => "oid4vp_response_submitted",
            Self::EncryptedResponseSubmitted => "oid4vp_encrypted_response_submitted",
        }
    }
}

/// Typed OID4VP event accepted by the presentation state machine.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PresentationEvent {
    /// Authorization request object was received.
    AuthorizationRequestObject,
    /// Authorization response was submitted.
    AuthorizationResponse,
    /// Request `URI` was resolved.
    RequestUriResolution,
    /// Encrypted authorization response was submitted.
    EncryptedAuthorizationResponse,
    /// Authorization response was rejected by verifier nonce policy.
    AuthorizationResponseRejected,
}

impl PresentationEvent {
    /// Build an OID4VP event from a transcript message.
    #[must_use]
    pub fn from_transcript_message(message: &OpenId4VcTranscriptMessage<'_>) -> Option<Self> {
        let event = match (message.flow, message.message_type) {
            ("oid4vp_same_device_direct_post", "authorization_request_object") => {
                Self::AuthorizationRequestObject
            }
            ("oid4vp_same_device_direct_post", "authorization_response") => {
                Self::AuthorizationResponse
            }
            ("oid4vp_cross_device_direct_post", "request_uri_resolution") => {
                Self::RequestUriResolution
            }
            ("oid4vp_cross_device_direct_post", "encrypted_authorization_response") => {
                Self::EncryptedAuthorizationResponse
            }
            ("negative_wrong_nonce", "authorization_response_rejected") => {
                Self::AuthorizationResponseRejected
            }
            _ => return None,
        };
        Some(event)
    }
}

/// One accepted OID4VP transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PresentationTransition {
    /// State before the event was applied.
    pub from: PresentationState,
    /// Accepted event.
    pub event: PresentationEvent,
    /// State after the event was applied.
    pub to: PresentationState,
}

/// Stateful OID4VP transition boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PresentationStateMachine {
    flow: PresentationFlowKind,
    state: PresentationState,
}

impl PresentationStateMachine {
    /// Create a state machine for an OID4VP flow.
    #[must_use]
    pub const fn new(flow: PresentationFlowKind) -> Self {
        Self {
            flow,
            state: PresentationState::Initial,
        }
    }

    /// Return the configured flow kind.
    #[must_use]
    pub const fn flow(&self) -> PresentationFlowKind {
        self.flow
    }

    /// Return the current OID4VP state.
    #[must_use]
    pub const fn state(&self) -> PresentationState {
        self.state
    }

    /// Apply one OID4VP event.
    ///
    /// # Errors
    ///
    /// Returns a typed, redaction-safe verification error for wrong nonce
    /// rejection or a typed conflict for invalid sequencing.
    pub fn apply(&mut self, event: PresentationEvent) -> IdentusResult<PresentationTransition> {
        if event == PresentationEvent::AuthorizationResponseRejected {
            return Err(nonce_mismatch());
        }

        let from = self.state;
        let Some(to) = self.next_state(event) else {
            return Err(invalid_openid4vc_transition());
        };
        self.state = to;
        Ok(PresentationTransition { from, event, to })
    }

    fn next_state(&self, event: PresentationEvent) -> Option<PresentationState> {
        match (self.flow, self.state, event) {
            (
                PresentationFlowKind::SameDeviceDirectPost,
                PresentationState::Initial,
                PresentationEvent::AuthorizationRequestObject,
            ) => Some(PresentationState::RequestValidated),
            (
                PresentationFlowKind::SameDeviceDirectPost,
                PresentationState::RequestValidated,
                PresentationEvent::AuthorizationResponse,
            ) => Some(PresentationState::ResponseSubmitted),
            (
                PresentationFlowKind::CrossDeviceDirectPost,
                PresentationState::Initial,
                PresentationEvent::RequestUriResolution,
            ) => Some(PresentationState::RequestUriResolved),
            (
                PresentationFlowKind::CrossDeviceDirectPost,
                PresentationState::RequestUriResolved,
                PresentationEvent::EncryptedAuthorizationResponse,
            ) => Some(PresentationState::EncryptedResponseSubmitted),
            _ => None,
        }
    }
}

/// Typed `SIOPv2` state.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SelfIssuedOpenIdProviderState {
    /// No `SIOPv2` message has been accepted.
    Initial,
    /// Self-issued ID token request has been validated.
    IdTokenRequestValidated,
    /// Self-issued ID token response has been validated.
    IdTokenValidated,
}

impl SelfIssuedOpenIdProviderState {
    /// Stable fixture state id.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Initial => "siopv2_initial",
            Self::IdTokenRequestValidated => "siopv2_id_token_request_validated",
            Self::IdTokenValidated => "siopv2_id_token_validated",
        }
    }
}

/// Typed `SIOPv2` event accepted by the state machine.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SelfIssuedOpenIdProviderEvent {
    /// Self-issued ID token request was received.
    SelfIssuedIdTokenRequest,
    /// Self-issued ID token response was received.
    SelfIssuedIdTokenResponse,
}

impl SelfIssuedOpenIdProviderEvent {
    /// Build a `SIOPv2` event from a transcript message.
    #[must_use]
    pub fn from_transcript_message(message: &OpenId4VcTranscriptMessage<'_>) -> Option<Self> {
        let event = match (message.flow, message.message_type) {
            ("siopv2_self_issued", "self_issued_id_token_request") => {
                Self::SelfIssuedIdTokenRequest
            }
            ("siopv2_self_issued", "self_issued_id_token_response") => {
                Self::SelfIssuedIdTokenResponse
            }
            _ => return None,
        };
        Some(event)
    }
}

/// One accepted `SIOPv2` transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SelfIssuedOpenIdProviderTransition {
    /// State before the event was applied.
    pub from: SelfIssuedOpenIdProviderState,
    /// Accepted event.
    pub event: SelfIssuedOpenIdProviderEvent,
    /// State after the event was applied.
    pub to: SelfIssuedOpenIdProviderState,
}

/// Stateful `SIOPv2` transition boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelfIssuedOpenIdProviderStateMachine {
    state: SelfIssuedOpenIdProviderState,
}

impl SelfIssuedOpenIdProviderStateMachine {
    /// Create a `SIOPv2` state machine.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            state: SelfIssuedOpenIdProviderState::Initial,
        }
    }

    /// Return the current `SIOPv2` state.
    #[must_use]
    pub const fn state(&self) -> SelfIssuedOpenIdProviderState {
        self.state
    }

    /// Apply one `SIOPv2` event.
    ///
    /// # Errors
    ///
    /// Returns a typed, redaction-safe conflict when the event is not valid
    /// for the current state.
    pub fn apply(
        &mut self,
        event: SelfIssuedOpenIdProviderEvent,
    ) -> IdentusResult<SelfIssuedOpenIdProviderTransition> {
        let from = self.state;
        let Some(to) = self.next_state(event) else {
            return Err(invalid_openid4vc_transition());
        };
        self.state = to;
        Ok(SelfIssuedOpenIdProviderTransition { from, event, to })
    }

    fn next_state(
        &self,
        event: SelfIssuedOpenIdProviderEvent,
    ) -> Option<SelfIssuedOpenIdProviderState> {
        match (self.state, event) {
            (
                SelfIssuedOpenIdProviderState::Initial,
                SelfIssuedOpenIdProviderEvent::SelfIssuedIdTokenRequest,
            ) => Some(SelfIssuedOpenIdProviderState::IdTokenRequestValidated),
            (
                SelfIssuedOpenIdProviderState::IdTokenRequestValidated,
                SelfIssuedOpenIdProviderEvent::SelfIssuedIdTokenResponse,
            ) => Some(SelfIssuedOpenIdProviderState::IdTokenValidated),
            _ => None,
        }
    }
}

impl Default for SelfIssuedOpenIdProviderStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

/// Transcript message consumed by Docker-free state machines.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OpenId4VcTranscriptMessage<'a> {
    /// Stable flow id from the transcript fixture.
    pub flow: &'a str,
    /// Stable message type from the transcript fixture.
    pub message_type: &'a str,
    /// Expected typed error for negative transcript entries.
    pub expected_error: Option<&'a str>,
}

/// Typed replay state emitted by `OpenID4VC` state machines.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum OpenId4VcState {
    /// OID4VCI authorization-code metadata was resolved.
    AuthorizationCodeMetadataResolved,
    /// OID4VCI authorization-code token was issued.
    AuthorizationCodeTokenIssued,
    /// OID4VCI authorization-code credential was issued.
    AuthorizationCodeCredentialIssued,
    /// OID4VCI pre-authorized-code offer was resolved.
    PreAuthorizedCodeOfferResolved,
    /// OID4VCI pre-authorized-code credential was issued.
    PreAuthorizedCodeCredentialIssued,
    /// OID4VCI deferred credential is pending.
    DeferredCredentialPending,
    /// OID4VCI deferred credential is ready.
    DeferredCredentialReady,
    /// OID4VP same-device request was validated.
    SameDeviceDirectPostRequestValidated,
    /// OID4VP same-device response was submitted.
    SameDeviceDirectPostResponseSubmitted,
    /// OID4VP cross-device request URI was resolved.
    CrossDeviceDirectPostRequestUriResolved,
    /// OID4VP cross-device encrypted response was submitted.
    CrossDeviceDirectPostEncryptedResponseSubmitted,
    /// `SIOPv2` self-issued ID token was validated.
    SelfIssuedIdTokenValidated,
    /// HAIP wallet attestation was validated.
    WalletAttestationValidated,
    /// Digital Credentials API request was classified.
    DigitalCredentialsApiRequestClassified,
    /// Federation entity statement was resolved.
    FederationEntityStatementResolved,
    /// Federation trust chain was validated.
    FederationTrustChainValidated,
    /// Negative wrong-nonce transcript was rejected.
    NegativeWrongNonceRejected,
    /// Negative trust-chain transcript was rejected.
    NegativeTrustChainRejected,
}

impl OpenId4VcState {
    /// Stable fixture state id.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthorizationCodeMetadataResolved => {
                "oid4vci_authorization_code_metadata_resolved"
            }
            Self::AuthorizationCodeTokenIssued => "oid4vci_authorization_code_token_issued",
            Self::AuthorizationCodeCredentialIssued => {
                "oid4vci_authorization_code_credential_issued"
            }
            Self::PreAuthorizedCodeOfferResolved => "oid4vci_pre_authorized_code_offer_resolved",
            Self::PreAuthorizedCodeCredentialIssued => {
                "oid4vci_pre_authorized_code_credential_issued"
            }
            Self::DeferredCredentialPending => "oid4vci_deferred_credential_pending",
            Self::DeferredCredentialReady => "oid4vci_deferred_credential_ready",
            Self::SameDeviceDirectPostRequestValidated => {
                "oid4vp_same_device_direct_post_request_validated"
            }
            Self::SameDeviceDirectPostResponseSubmitted => {
                "oid4vp_same_device_direct_post_response_submitted"
            }
            Self::CrossDeviceDirectPostRequestUriResolved => {
                "oid4vp_cross_device_direct_post_request_uri_resolved"
            }
            Self::CrossDeviceDirectPostEncryptedResponseSubmitted => {
                "oid4vp_cross_device_direct_post_encrypted_response_submitted"
            }
            Self::SelfIssuedIdTokenValidated => "siopv2_self_issued_id_token_validated",
            Self::WalletAttestationValidated => "haip_wallet_attestation_validated",
            Self::DigitalCredentialsApiRequestClassified => {
                "haip_digital_credentials_api_request_classified"
            }
            Self::FederationEntityStatementResolved => "federation_entity_statement_resolved",
            Self::FederationTrustChainValidated => "federation_trust_chain_validated",
            Self::NegativeWrongNonceRejected => "negative_wrong_nonce_rejected",
            Self::NegativeTrustChainRejected => "negative_trust_chain_rejected",
        }
    }
}

/// State-machine replay event.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenId4VcReplayEvent {
    /// Emitted typed state.
    pub state: OpenId4VcState,
    /// Optional typed error for negative replay entries.
    pub error: Option<IdentusError>,
}

/// OID4VCI state machine.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CredentialIssuanceFlow;

impl CredentialIssuanceFlow {
    /// Apply one transcript message.
    #[must_use]
    pub fn apply(message: &OpenId4VcTranscriptMessage<'_>) -> Option<OpenId4VcReplayEvent> {
        let state = match (message.flow, message.message_type) {
            ("oid4vci_authorization_code", "credential_issuer_metadata") => {
                OpenId4VcState::AuthorizationCodeMetadataResolved
            }
            ("oid4vci_authorization_code", "token_response") => {
                OpenId4VcState::AuthorizationCodeTokenIssued
            }
            ("oid4vci_authorization_code", "credential_response") => {
                OpenId4VcState::AuthorizationCodeCredentialIssued
            }
            ("oid4vci_pre_authorized_code", "credential_offer_by_reference") => {
                OpenId4VcState::PreAuthorizedCodeOfferResolved
            }
            ("oid4vci_pre_authorized_code", "credential_response") => {
                OpenId4VcState::PreAuthorizedCodeCredentialIssued
            }
            ("oid4vci_deferred_credential", "deferred_credential_response_pending") => {
                OpenId4VcState::DeferredCredentialPending
            }
            ("oid4vci_deferred_credential", "deferred_credential_response_ready") => {
                OpenId4VcState::DeferredCredentialReady
            }
            _ => return None,
        };
        Some(OpenId4VcReplayEvent { state, error: None })
    }
}

/// OID4VP state machine.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PresentationFlow;

impl PresentationFlow {
    /// Apply one transcript message.
    #[must_use]
    pub fn apply(message: &OpenId4VcTranscriptMessage<'_>) -> Option<OpenId4VcReplayEvent> {
        match (message.flow, message.message_type) {
            ("oid4vp_same_device_direct_post", "authorization_request_object") => {
                Some(OpenId4VcReplayEvent {
                    state: OpenId4VcState::SameDeviceDirectPostRequestValidated,
                    error: None,
                })
            }
            ("oid4vp_same_device_direct_post", "authorization_response") => {
                Some(OpenId4VcReplayEvent {
                    state: OpenId4VcState::SameDeviceDirectPostResponseSubmitted,
                    error: None,
                })
            }
            ("oid4vp_cross_device_direct_post", "request_uri_resolution") => {
                Some(OpenId4VcReplayEvent {
                    state: OpenId4VcState::CrossDeviceDirectPostRequestUriResolved,
                    error: None,
                })
            }
            ("oid4vp_cross_device_direct_post", "encrypted_authorization_response") => {
                Some(OpenId4VcReplayEvent {
                    state: OpenId4VcState::CrossDeviceDirectPostEncryptedResponseSubmitted,
                    error: None,
                })
            }
            ("negative_wrong_nonce", "authorization_response_rejected") => {
                Some(OpenId4VcReplayEvent {
                    state: OpenId4VcState::NegativeWrongNonceRejected,
                    error: Some(nonce_mismatch()),
                })
            }
            _ => None,
        }
    }
}

/// `SIOPv2` state machine.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SelfIssuedOpenIdProviderFlow;

impl SelfIssuedOpenIdProviderFlow {
    /// Apply one transcript message.
    #[must_use]
    pub fn apply(message: &OpenId4VcTranscriptMessage<'_>) -> Option<OpenId4VcReplayEvent> {
        if matches!(
            (message.flow, message.message_type),
            ("siopv2_self_issued", "self_issued_id_token_response")
        ) {
            return Some(OpenId4VcReplayEvent {
                state: OpenId4VcState::SelfIssuedIdTokenValidated,
                error: None,
            });
        }
        None
    }
}

/// HAIP state machine.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HighAssuranceProfileFlow;

impl HighAssuranceProfileFlow {
    /// Apply one transcript message.
    #[must_use]
    pub fn apply(message: &OpenId4VcTranscriptMessage<'_>) -> Option<OpenId4VcReplayEvent> {
        let state = match (message.flow, message.message_type) {
            ("haip_wallet_attestation", "wallet_attestation_validation") => {
                OpenId4VcState::WalletAttestationValidated
            }
            ("haip_digital_credentials_api", "digital_credentials_api_request") => {
                OpenId4VcState::DigitalCredentialsApiRequestClassified
            }
            _ => return None,
        };
        Some(OpenId4VcReplayEvent { state, error: None })
    }
}

/// `OpenID` Federation trust state machine.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FederationTrustFlow;

impl FederationTrustFlow {
    /// Apply one transcript message.
    #[must_use]
    pub fn apply(message: &OpenId4VcTranscriptMessage<'_>) -> Option<OpenId4VcReplayEvent> {
        match (message.flow, message.message_type) {
            ("federation_backed_trust", "entity_statement_resolution") => {
                Some(OpenId4VcReplayEvent {
                    state: OpenId4VcState::FederationEntityStatementResolved,
                    error: None,
                })
            }
            ("federation_backed_trust", "trust_chain_validation") => Some(OpenId4VcReplayEvent {
                state: OpenId4VcState::FederationTrustChainValidated,
                error: None,
            }),
            ("negative_trust_chain_rejection", "trust_chain_rejected") => {
                Some(OpenId4VcReplayEvent {
                    state: OpenId4VcState::NegativeTrustChainRejected,
                    error: Some(identus_trust::trust_chain_rejected()),
                })
            }
            _ => None,
        }
    }
}

/// Replay transcript messages through all typed `OpenID4VC` state machines.
#[must_use]
pub fn replay_openid4vc_transcript(
    messages: &[OpenId4VcTranscriptMessage<'_>],
) -> Vec<OpenId4VcReplayEvent> {
    messages
        .iter()
        .filter_map(|message| {
            CredentialIssuanceFlow::apply(message)
                .or_else(|| PresentationFlow::apply(message))
                .or_else(|| SelfIssuedOpenIdProviderFlow::apply(message))
                .or_else(|| HighAssuranceProfileFlow::apply(message))
                .or_else(|| FederationTrustFlow::apply(message))
        })
        .collect()
}

/// Invalid `OpenID4VC` state transition typed error.
#[must_use]
pub const fn invalid_openid4vc_transition() -> IdentusError {
    IdentusError::public(
        ErrorCode::new("invalid_openid4vc_transition"),
        ErrorKind::Conflict,
        OPENID4VC_CAPABILITY,
        "OpenID4VC state transition is invalid",
    )
}

/// Wrong nonce typed error used by OID4VP negative fixtures.
#[must_use]
pub const fn nonce_mismatch() -> IdentusError {
    IdentusError::public(
        ErrorCode::new("nonce_mismatch"),
        ErrorKind::VerificationFailed,
        OPENID4VC_CAPABILITY,
        "OpenID4VC nonce does not match",
    )
}

/// Evaluate `OpenID4VC` trust policy through `identus-trust`.
///
/// # Errors
///
/// Returns typed trust errors from the supplied policy engine.
pub fn evaluate_openid4vc_trust(
    policy_engine: &impl TrustPolicyEngine,
    input: &TrustPolicyInput,
) -> IdentusResult<TrustPolicyDecision> {
    policy_engine.evaluate(input)
}

#[cfg(test)]
mod tests {
    use identus_core::{CapabilityId, ErrorKind};
    use identus_trust::{InMemoryTrustRegistry, StatusMechanism, TrustChain, TrustPolicyInput};

    use super::{
        CredentialIssuanceEvent, CredentialIssuanceFlowKind, CredentialIssuanceState,
        CredentialIssuanceStateMachine, OpenId4VcState, OpenId4VcTranscriptMessage,
        PresentationEvent, PresentationFlowKind, PresentationState, PresentationStateMachine,
        SelfIssuedOpenIdProviderEvent, SelfIssuedOpenIdProviderState,
        SelfIssuedOpenIdProviderStateMachine, evaluate_openid4vc_trust,
        invalid_openid4vc_transition, nonce_mismatch, replay_openid4vc_transcript,
    };

    #[test]
    fn openid4vc_trust_delegates_to_trust_policy_engine() {
        let policy_engine = InMemoryTrustRegistry::with_default_fixtures();
        let decision = evaluate_openid4vc_trust(
            &policy_engine,
            &TrustPolicyInput {
                subject: "openid-federation-issuer".to_owned(),
                status: None,
                trust_chain: Some(TrustChain {
                    subject: "openid-federation-issuer".to_owned(),
                    anchors: vec!["fixture-anchor-OpenIdFederation".to_owned()],
                    mechanism: StatusMechanism::OpenIdFederation,
                }),
                require_status: false,
                require_trust: true,
            },
        )
        .expect("default federation trust fixture should accept OpenID4VC");

        assert!(decision.accepted);
    }

    #[test]
    fn transcript_replay_emits_typed_states_and_negative_errors() {
        let messages = [
            OpenId4VcTranscriptMessage {
                flow: "oid4vci_authorization_code",
                message_type: "credential_issuer_metadata",
                expected_error: None,
            },
            OpenId4VcTranscriptMessage {
                flow: "oid4vci_authorization_code",
                message_type: "token_response",
                expected_error: None,
            },
            OpenId4VcTranscriptMessage {
                flow: "oid4vp_same_device_direct_post",
                message_type: "authorization_response",
                expected_error: None,
            },
            OpenId4VcTranscriptMessage {
                flow: "negative_wrong_nonce",
                message_type: "authorization_response_rejected",
                expected_error: Some("nonce_mismatch"),
            },
        ];

        let events = replay_openid4vc_transcript(&messages);
        let states = events.iter().map(|event| event.state).collect::<Vec<_>>();
        assert!(states.contains(&OpenId4VcState::AuthorizationCodeMetadataResolved));
        assert!(states.contains(&OpenId4VcState::AuthorizationCodeTokenIssued));
        assert!(states.contains(&OpenId4VcState::SameDeviceDirectPostResponseSubmitted));
        assert!(states.contains(&OpenId4VcState::NegativeWrongNonceRejected));
        assert!(
            events
                .iter()
                .any(|event| event.error.as_ref() == Some(&nonce_mismatch()))
        );
    }

    #[test]
    fn credential_issuance_state_machine_accepts_oid4vci_sequences() {
        let mut authorization_code =
            CredentialIssuanceStateMachine::new(CredentialIssuanceFlowKind::AuthorizationCode);
        let transitions = [
            CredentialIssuanceEvent::CredentialIssuerMetadata,
            CredentialIssuanceEvent::CredentialOfferByValue,
            CredentialIssuanceEvent::AuthorizationRequest,
            CredentialIssuanceEvent::TokenResponse,
            CredentialIssuanceEvent::CredentialRequest,
            CredentialIssuanceEvent::CredentialResponse,
        ]
        .into_iter()
        .map(|event| {
            authorization_code
                .apply(event)
                .expect("authorization-code transition should be accepted")
        })
        .collect::<Vec<_>>();

        assert_eq!(
            transitions.last().map(|transition| transition.to),
            Some(CredentialIssuanceState::CredentialIssued)
        );
        assert_eq!(
            authorization_code.state().as_str(),
            "oid4vci_credential_issued"
        );

        let mut pre_authorized =
            CredentialIssuanceStateMachine::new(CredentialIssuanceFlowKind::PreAuthorizedCode);
        for event in [
            CredentialIssuanceEvent::CredentialOfferByReference,
            CredentialIssuanceEvent::TokenRequest,
            CredentialIssuanceEvent::CredentialResponse,
        ] {
            pre_authorized
                .apply(event)
                .expect("pre-authorized transition should be accepted");
        }
        assert_eq!(
            pre_authorized.state(),
            CredentialIssuanceState::CredentialIssued
        );

        let mut deferred =
            CredentialIssuanceStateMachine::new(CredentialIssuanceFlowKind::DeferredCredential);
        for event in [
            CredentialIssuanceEvent::DeferredCredentialResponsePending,
            CredentialIssuanceEvent::DeferredCredentialRequest,
            CredentialIssuanceEvent::DeferredCredentialResponseReady,
        ] {
            deferred
                .apply(event)
                .expect("deferred transition should be accepted");
        }
        assert_eq!(
            deferred.state(),
            CredentialIssuanceState::DeferredCredentialReady
        );
    }

    #[test]
    fn credential_issuance_state_machine_rejects_invalid_transition() {
        let mut flow =
            CredentialIssuanceStateMachine::new(CredentialIssuanceFlowKind::AuthorizationCode);

        let error = flow
            .apply(CredentialIssuanceEvent::CredentialResponse)
            .expect_err("credential response cannot be first");

        assert_eq!(error, invalid_openid4vc_transition());
        assert_eq!(error.code().as_str(), "invalid_openid4vc_transition");
        assert_eq!(error.kind(), ErrorKind::Conflict);
        assert_eq!(
            error.capability().map(CapabilityId::as_str),
            Some("openid4vc")
        );
        assert!(!error.to_string().contains("redacted"));
        assert_eq!(flow.state(), CredentialIssuanceState::Initial);
    }

    #[test]
    fn presentation_state_machine_accepts_oid4vp_sequences() {
        let mut same_device =
            PresentationStateMachine::new(PresentationFlowKind::SameDeviceDirectPost);
        for event in [
            PresentationEvent::AuthorizationRequestObject,
            PresentationEvent::AuthorizationResponse,
        ] {
            same_device
                .apply(event)
                .expect("same-device OID4VP transition should be accepted");
        }
        assert_eq!(same_device.state(), PresentationState::ResponseSubmitted);

        let mut cross_device =
            PresentationStateMachine::new(PresentationFlowKind::CrossDeviceDirectPost);
        for event in [
            PresentationEvent::RequestUriResolution,
            PresentationEvent::EncryptedAuthorizationResponse,
        ] {
            cross_device
                .apply(event)
                .expect("cross-device OID4VP transition should be accepted");
        }
        assert_eq!(
            cross_device.state(),
            PresentationState::EncryptedResponseSubmitted
        );
    }

    #[test]
    fn presentation_state_machine_rejects_wrong_nonce_without_state_change() {
        let mut flow = PresentationStateMachine::new(PresentationFlowKind::SameDeviceDirectPost);
        flow.apply(PresentationEvent::AuthorizationRequestObject)
            .expect("request object should be accepted");

        let error = flow
            .apply(PresentationEvent::AuthorizationResponseRejected)
            .expect_err("wrong nonce should be rejected");

        assert_eq!(error, nonce_mismatch());
        assert_eq!(error.code().as_str(), "nonce_mismatch");
        assert_eq!(error.kind(), ErrorKind::VerificationFailed);
        assert_eq!(
            error.capability().map(CapabilityId::as_str),
            Some("openid4vc")
        );
        assert!(!error.to_string().contains("redacted"));
        assert_eq!(flow.state(), PresentationState::RequestValidated);
    }

    #[test]
    fn self_issued_openid_provider_state_machine_accepts_siopv2_sequence() {
        let mut flow = SelfIssuedOpenIdProviderStateMachine::new();
        for event in [
            SelfIssuedOpenIdProviderEvent::SelfIssuedIdTokenRequest,
            SelfIssuedOpenIdProviderEvent::SelfIssuedIdTokenResponse,
        ] {
            flow.apply(event)
                .expect("SIOPv2 transition should be accepted");
        }
        assert_eq!(
            flow.state(),
            SelfIssuedOpenIdProviderState::IdTokenValidated
        );
    }

    #[test]
    fn self_issued_openid_provider_state_machine_rejects_invalid_transition() {
        let mut flow = SelfIssuedOpenIdProviderStateMachine::new();

        let error = flow
            .apply(SelfIssuedOpenIdProviderEvent::SelfIssuedIdTokenResponse)
            .expect_err("ID token response cannot be first");

        assert_eq!(error, invalid_openid4vc_transition());
        assert_eq!(error.code().as_str(), "invalid_openid4vc_transition");
        assert_eq!(error.kind(), ErrorKind::Conflict);
        assert_eq!(
            error.capability().map(CapabilityId::as_str),
            Some("openid4vc")
        );
        assert!(!error.to_string().contains("redacted"));
        assert_eq!(flow.state(), SelfIssuedOpenIdProviderState::Initial);
    }
}
