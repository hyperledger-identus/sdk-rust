//! Type-safe `DIDComm` and mediator boundaries.
//!
//! This crate owns the parsing boundary for `DIDComm` message types, protocol
//! families, protocol thread identifiers, addressing, and later pack/unpack
//! state machines. Protocol behavior is intentionally introduced behind typed
//! domain values before transport, storage, or cryptographic adapters are wired
//! in.

use identus_core::{
    CapabilityId, ErrorCode, ErrorEnvelope, ErrorKind, IdentusError, IdentusResult,
};
use identus_did::Did;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Component metadata.
pub const COMPONENT: identus_core::Component = identus_core::Component {
    name: "identus-messaging",
    summary: "DIDComm v2, mediator, and protocol-state boundaries.",
};

/// Stable capability id for `DIDComm` errors.
pub const DIDCOMM_CAPABILITY: CapabilityId = CapabilityId::new("didcomm");

const DIDCOMM_MESSAGE_TYPE_PREFIX: &str = "https://didcomm.org/";

/// `DIDComm` message protocol family.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DidCommProtocolKind {
    /// `DIDComm` Out-of-Band invitations.
    OutOfBand,
    /// Basic human/application message protocol.
    BasicMessage,
    /// Trust Ping liveness protocol.
    TrustPing,
    /// Routing and forwarding protocol.
    Routing,
    /// Discover Features protocol.
    DiscoverFeatures,
    /// DID exchange and legacy connection compatibility.
    Connection,
    /// Mediator coordination protocol.
    CoordinateMediation,
    /// Message pickup protocol.
    MessagePickup,
    /// Issue Credential protocol.
    IssueCredential,
    /// Present Proof protocol.
    PresentProof,
    /// Report Problem protocol.
    ReportProblem,
    /// Identus revocation notification protocol.
    RevocationNotification,
    /// DID rotation protocol family.
    DidRotate,
    /// A syntactically valid `DIDComm` protocol not yet assigned product
    /// semantics.
    Other,
}

impl DidCommProtocolKind {
    /// Classify a `DIDComm` protocol family from the URI segment.
    #[must_use]
    pub fn from_family(family: &str) -> Self {
        match family {
            "out-of-band" => Self::OutOfBand,
            "basicmessage" => Self::BasicMessage,
            "trust-ping" => Self::TrustPing,
            "routing" => Self::Routing,
            "discover-features" => Self::DiscoverFeatures,
            "connections" | "didexchange" => Self::Connection,
            "coordinate-mediation" | "mediator-coordination" => Self::CoordinateMediation,
            "messagepickup" | "message-pickup" | "pickup" => Self::MessagePickup,
            "issue-credential" => Self::IssueCredential,
            "present-proof" => Self::PresentProof,
            "report-problem" => Self::ReportProblem,
            "revocation-notification" | "revocation_notification" => Self::RevocationNotification,
            "did-rotate" => Self::DidRotate,
            _ => Self::Other,
        }
    }
}

/// Planned support maturity for a `DIDComm` protocol.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DidCommSupportStage {
    /// Required for parity with current Cloud Agent, SDK, or Mediator behavior.
    Parity,
    /// Required for the target Rust architecture beyond existing parity.
    Roadmap,
    /// Accepted for protocol compatibility, migration, or fixtures.
    Compatibility,
}

/// One tracked `DIDComm` protocol profile.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DidCommProtocolProfile {
    /// Stable protocol id in the conformance catalog.
    pub id: &'static str,
    /// Protocol family classification.
    pub kind: DidCommProtocolKind,
    /// Canonical protocol URI.
    pub uri: &'static str,
    /// Message names currently seen in Identus source or required by roadmap.
    pub message_names: &'static [&'static str],
    /// Current support stage.
    pub stage: DidCommSupportStage,
    /// Source evidence for tracking this protocol.
    pub source: &'static str,
    /// Backlog task that owns behavior or fixture depth.
    pub backlog_task: &'static str,
}

/// `DIDComm` protocol families currently tracked by `sdk-rust`.
#[rustfmt::skip]
pub const DIDCOMM_PROTOCOL_PROFILES: &[DidCommProtocolProfile] = &[
    protocol("didcomm-oob-2", DidCommProtocolKind::OutOfBand, "https://didcomm.org/out-of-band/2.0", &["invitation"], DidCommSupportStage::Parity, "Cloud Agent connection/OOB modules and docs specifications matrix", "T013"),
    protocol("didcomm-basicmessage-2", DidCommProtocolKind::BasicMessage, "https://didcomm.org/basicmessage/2.0", &["message"], DidCommSupportStage::Parity, "Mediator Discover Features and E2E tests", "T064"),
    protocol("trust-ping-2", DidCommProtocolKind::TrustPing, "https://didcomm.org/trust-ping/2.0", &["ping", "ping-response"], DidCommSupportStage::Parity, "Cloud Agent protocol-trust-ping and Mediator Discover Features", "T064"),
    protocol("routing-2", DidCommProtocolKind::Routing, "https://didcomm.org/routing/2.0", &["forward"], DidCommSupportStage::Parity, "Cloud Agent protocol-routing and Mediator forwarding", "T045"),
    protocol("discover-features-2", DidCommProtocolKind::DiscoverFeatures, "https://didcomm.org/discover-features/2.0", &["queries", "disclose"], DidCommSupportStage::Parity, "Mediator DiscoverFeaturesExecuter and protocol tests", "T064"),
    protocol("coordinate-mediation-2", DidCommProtocolKind::CoordinateMediation, "https://didcomm.org/coordinate-mediation/2.0", &["mediate-request", "mediate-grant", "mediate-deny", "keylist-update", "keylist-update-response", "keylist-query", "keylist"], DidCommSupportStage::Parity, "Cloud Agent coordinate mediation and Mediator E2E tests", "T045"),
    protocol("coordinate-mediation-3", DidCommProtocolKind::CoordinateMediation, "https://didcomm.org/coordinate-mediation/3.0", &[], DidCommSupportStage::Roadmap, "Docs specifications matrix marks mediation 3.0 as TODO", "T045"),
    protocol("message-pickup-3", DidCommProtocolKind::MessagePickup, "https://didcomm.org/messagepickup/3.0", &["status", "status-request", "delivery-request", "messages-received", "live-delivery-change"], DidCommSupportStage::Parity, "Mediator pickup tests and docs specifications matrix", "T045"),
    protocol("didcomm-issue-credential-3", DidCommProtocolKind::IssueCredential, "https://didcomm.org/issue-credential/3.0", &["propose-credential", "offer-credential", "request-credential", "issue-credential", "credential-credential"], DidCommSupportStage::Parity, "Cloud Agent issue credential protocol", "T044"),
    protocol("didcomm-present-proof-3", DidCommProtocolKind::PresentProof, "https://didcomm.org/present-proof/3.0", &["propose-presentation", "request-presentation", "presentation"], DidCommSupportStage::Parity, "Cloud Agent present proof protocol", "T044"),
    protocol("didcomm-report-problem-2", DidCommProtocolKind::ReportProblem, "https://didcomm.org/report-problem/2.0", &["problem-report"], DidCommSupportStage::Parity, "Cloud Agent report problem protocol and docs ADR", "T044"),
    protocol("didcomm-revocation-notification-1", DidCommProtocolKind::RevocationNotification, "https://didcomm.org/revocation-notification/1.0", &["revoke"], DidCommSupportStage::Parity, "Identus revocation notification docs and integration matrix", "T041"),
    protocol("didcomm-didexchange-1", DidCommProtocolKind::Connection, "https://didcomm.org/didexchange/1.0", &["request", "response", "complete", "problem_report"], DidCommSupportStage::Compatibility, "Cloud Agent DID exchange compatibility source", "T066"),
    protocol("didcomm-connections-1", DidCommProtocolKind::Connection, "https://didcomm.org/connections/1.0", &[], DidCommSupportStage::Compatibility, "Cloud Agent invitation compatibility tests", "T066"),
    protocol("didcomm-did-rotate-2", DidCommProtocolKind::DidRotate, "https://didcomm.org/did-rotate/2.0", &["rotate", "ack", "hangup", "problem-report"], DidCommSupportStage::Roadmap, "Backlog theme for DID rotation and wallet identity continuity", "T067"),
];

/// `DIDComm` message type with validated URI parts.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DidCommMessageType {
    value: String,
    family: String,
    version: String,
    name: String,
}

impl DidCommMessageType {
    /// Parse a `DIDComm` message type URI.
    ///
    /// # Errors
    ///
    /// Returns [`DidCommParseError`] when the value is not a
    /// `https://didcomm.org/{family}/{version}/{name}` URI with non-empty,
    /// path-safe segments.
    pub fn parse(input: &str) -> Result<Self, DidCommParseError> {
        let path = input
            .strip_prefix(DIDCOMM_MESSAGE_TYPE_PREFIX)
            .ok_or(DidCommParseError::MissingDidCommPrefix)?;
        let mut parts = path.split('/');
        let family = parse_path_segment(parts.next(), "family")?;
        let version = parse_version_segment(parts.next())?;
        let name = parse_path_segment(parts.next(), "message name")?;

        if parts.next().is_some() {
            return Err(DidCommParseError::UnexpectedPathSegment);
        }

        Ok(Self {
            value: input.to_owned(),
            family,
            version,
            name,
        })
    }

    /// Parse a `DIDComm` message type URI with the shared core error surface.
    ///
    /// # Errors
    ///
    /// Returns [`IdentusError`] when the value is not a
    /// `https://didcomm.org/{family}/{version}/{name}` URI with non-empty,
    /// path-safe segments.
    pub fn parse_with_core_error(input: &str) -> IdentusResult<Self> {
        Self::parse(input).map_err(Into::into)
    }

    /// Borrow the canonical message type URI.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Borrow the protocol family segment.
    #[must_use]
    pub fn family(&self) -> &str {
        &self.family
    }

    /// Borrow the protocol version segment.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Borrow the message name segment.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Classify the message type family.
    #[must_use]
    pub fn protocol_kind(&self) -> DidCommProtocolKind {
        DidCommProtocolKind::from_family(self.family())
    }

    /// Return the tracked protocol profile matching this message type, if any.
    #[must_use]
    pub fn protocol_profile(&self) -> Option<&'static DidCommProtocolProfile> {
        DIDCOMM_PROTOCOL_PROFILES
            .iter()
            .find(|profile| profile.uri == self.protocol_uri())
    }

    /// Return the canonical protocol URI without the message name.
    #[must_use]
    pub fn protocol_uri(&self) -> String {
        format!(
            "{DIDCOMM_MESSAGE_TYPE_PREFIX}{}/{}",
            self.family, self.version
        )
    }
}

impl Display for DidCommMessageType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for DidCommMessageType {
    type Err = DidCommParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::parse(input)
    }
}

/// `DIDComm` message identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DidCommMessageId(String);

impl DidCommMessageId {
    /// Parse a non-empty `DIDComm` message id.
    ///
    /// # Errors
    ///
    /// Returns [`DidCommParseError::InvalidIdentifier`] when the id is empty
    /// or contains whitespace/control characters.
    pub fn parse(input: &str) -> Result<Self, DidCommParseError> {
        parse_identifier(input, "message id").map(Self)
    }

    /// Parse a non-empty `DIDComm` message id with the shared core error
    /// surface.
    ///
    /// # Errors
    ///
    /// Returns [`IdentusError`] when the id is empty or contains
    /// whitespace/control characters.
    pub fn parse_with_core_error(input: &str) -> IdentusResult<Self> {
        Self::parse(input).map_err(Into::into)
    }

    /// Borrow the id as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for DidCommMessageId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for DidCommMessageId {
    type Err = DidCommParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::parse(input)
    }
}

/// `DIDComm` thread identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DidCommThreadId(String);

impl DidCommThreadId {
    /// Parse a non-empty `DIDComm` thread id.
    ///
    /// # Errors
    ///
    /// Returns [`DidCommParseError::InvalidIdentifier`] when the id is empty
    /// or contains whitespace/control characters.
    pub fn parse(input: &str) -> Result<Self, DidCommParseError> {
        parse_identifier(input, "thread id").map(Self)
    }

    /// Parse a non-empty `DIDComm` thread id with the shared core error
    /// surface.
    ///
    /// # Errors
    ///
    /// Returns [`IdentusError`] when the id is empty or contains
    /// whitespace/control characters.
    pub fn parse_with_core_error(input: &str) -> IdentusResult<Self> {
        Self::parse(input).map_err(Into::into)
    }

    /// Borrow the id as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for DidCommThreadId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for DidCommThreadId {
    type Err = DidCommParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::parse(input)
    }
}

/// Typed plaintext `DIDComm` envelope boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DidCommPlaintextMessage {
    id: DidCommMessageId,
    message_type: DidCommMessageType,
    from: Option<Did>,
    to: Vec<Did>,
    thread_id: Option<DidCommThreadId>,
    parent_thread_id: Option<DidCommThreadId>,
}

impl DidCommPlaintextMessage {
    /// Create a message envelope before addressing or threading is attached.
    #[must_use]
    pub fn new(id: DidCommMessageId, message_type: DidCommMessageType) -> Self {
        Self {
            id,
            message_type,
            from: None,
            to: Vec::new(),
            thread_id: None,
            parent_thread_id: None,
        }
    }

    /// Create an addressed message envelope.
    ///
    /// # Errors
    ///
    /// Returns [`DidCommParseError::MissingRecipient`] when no recipients are
    /// provided.
    pub fn addressed(
        id: DidCommMessageId,
        message_type: DidCommMessageType,
        from: Did,
        to: Vec<Did>,
    ) -> Result<Self, DidCommParseError> {
        if to.is_empty() {
            return Err(DidCommParseError::MissingRecipient);
        }

        Ok(Self::new(id, message_type).with_from(from).with_to(to))
    }

    /// Create an addressed message envelope with the shared core error surface.
    ///
    /// # Errors
    ///
    /// Returns [`IdentusError`] when no recipients are provided.
    pub fn addressed_with_core_error(
        id: DidCommMessageId,
        message_type: DidCommMessageType,
        from: Did,
        to: Vec<Did>,
    ) -> IdentusResult<Self> {
        Self::addressed(id, message_type, from, to).map_err(Into::into)
    }

    /// Attach sender DID.
    #[must_use]
    pub fn with_from(mut self, from: Did) -> Self {
        self.from = Some(from);
        self
    }

    /// Attach recipient DIDs.
    #[must_use]
    pub fn with_to(mut self, to: Vec<Did>) -> Self {
        self.to = to;
        self
    }

    /// Attach thread and parent-thread identifiers.
    #[must_use]
    pub fn with_thread(
        mut self,
        thread_id: DidCommThreadId,
        parent_thread_id: Option<DidCommThreadId>,
    ) -> Self {
        self.thread_id = Some(thread_id);
        self.parent_thread_id = parent_thread_id;
        self
    }

    /// Message id.
    #[must_use]
    pub const fn id(&self) -> &DidCommMessageId {
        &self.id
    }

    /// Message type.
    #[must_use]
    pub const fn message_type(&self) -> &DidCommMessageType {
        &self.message_type
    }

    /// Sender DID, if present.
    #[must_use]
    pub const fn from(&self) -> Option<&Did> {
        self.from.as_ref()
    }

    /// Recipient DIDs.
    #[must_use]
    pub fn to(&self) -> &[Did] {
        &self.to
    }

    /// Thread id, if present.
    #[must_use]
    pub const fn thread_id(&self) -> Option<&DidCommThreadId> {
        self.thread_id.as_ref()
    }

    /// Parent thread id, if present.
    #[must_use]
    pub const fn parent_thread_id(&self) -> Option<&DidCommThreadId> {
        self.parent_thread_id.as_ref()
    }
}

/// Coordinate mediation protocol state.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum MediationCoordinationState {
    /// No coordinate mediation message has been accepted.
    Initial,
    /// A mediation request has been sent.
    MediationRequested,
    /// Mediation was granted by the mediator.
    MediationGranted,
    /// Mediation was denied by the mediator.
    MediationDenied,
    /// A keylist update was requested.
    KeylistUpdateRequested,
    /// Recipient key material was registered with the mediator.
    RecipientKeyRegistered,
    /// A keylist query was submitted.
    KeylistQuerySubmitted,
    /// Current keylist was received.
    KeylistReceived,
}

impl MediationCoordinationState {
    /// Stable fixture state id.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Initial => "mediation_initial",
            Self::MediationRequested => "mediation_requested",
            Self::MediationGranted => "mediation_granted",
            Self::MediationDenied => "mediation_denied",
            Self::KeylistUpdateRequested => "keylist_update_requested",
            Self::RecipientKeyRegistered => "recipient_key_registered",
            Self::KeylistQuerySubmitted => "keylist_query_submitted",
            Self::KeylistReceived => "keylist_received",
        }
    }
}

/// Coordinate mediation event.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum MediationCoordinationEvent {
    /// `mediate-request` message.
    MediateRequest,
    /// `mediate-grant` message.
    MediateGrant,
    /// `mediate-deny` message.
    MediateDeny,
    /// `keylist-update` message.
    KeylistUpdate,
    /// `keylist-update-response` message.
    KeylistUpdateResponse,
    /// `keylist-query` message.
    KeylistQuery,
    /// `keylist` message.
    Keylist,
}

impl MediationCoordinationEvent {
    /// Build a mediation event from a parsed `DIDComm` message type.
    #[must_use]
    pub fn from_message_type(message_type: &DidCommMessageType) -> Option<Self> {
        if message_type.protocol_kind() != DidCommProtocolKind::CoordinateMediation {
            return None;
        }

        let event = match message_type.name() {
            "mediate-request" => Self::MediateRequest,
            "mediate-grant" => Self::MediateGrant,
            "mediate-deny" => Self::MediateDeny,
            "keylist-update" => Self::KeylistUpdate,
            "keylist-update-response" => Self::KeylistUpdateResponse,
            "keylist-query" => Self::KeylistQuery,
            "keylist" => Self::Keylist,
            _ => return None,
        };
        Some(event)
    }
}

/// One accepted coordinate mediation transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MediationCoordinationTransition {
    /// State before the event was applied.
    pub from: MediationCoordinationState,
    /// Accepted event.
    pub event: MediationCoordinationEvent,
    /// State after the event was applied.
    pub to: MediationCoordinationState,
}

/// Stateful coordinate mediation transition boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MediationCoordinationStateMachine {
    state: MediationCoordinationState,
}

impl MediationCoordinationStateMachine {
    /// Create a coordinate mediation state machine.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            state: MediationCoordinationState::Initial,
        }
    }

    /// Current coordinate mediation state.
    #[must_use]
    pub const fn state(&self) -> MediationCoordinationState {
        self.state
    }

    /// Apply one coordinate mediation event.
    ///
    /// # Errors
    ///
    /// Returns a typed, redaction-safe conflict when the event is not valid
    /// for the current state.
    pub fn apply(
        &mut self,
        event: MediationCoordinationEvent,
    ) -> IdentusResult<MediationCoordinationTransition> {
        let from = self.state;
        let Some(to) = self.next_state(event) else {
            return Err(invalid_didcomm_transition());
        };
        self.state = to;
        Ok(MediationCoordinationTransition { from, event, to })
    }

    fn next_state(&self, event: MediationCoordinationEvent) -> Option<MediationCoordinationState> {
        match (self.state, event) {
            (MediationCoordinationState::Initial, MediationCoordinationEvent::MediateRequest) => {
                Some(MediationCoordinationState::MediationRequested)
            }
            (
                MediationCoordinationState::MediationRequested,
                MediationCoordinationEvent::MediateGrant,
            ) => Some(MediationCoordinationState::MediationGranted),
            (
                MediationCoordinationState::MediationRequested,
                MediationCoordinationEvent::MediateDeny,
            ) => Some(MediationCoordinationState::MediationDenied),
            (
                MediationCoordinationState::MediationGranted
                | MediationCoordinationState::RecipientKeyRegistered,
                MediationCoordinationEvent::KeylistUpdate,
            ) => Some(MediationCoordinationState::KeylistUpdateRequested),
            (
                MediationCoordinationState::KeylistUpdateRequested,
                MediationCoordinationEvent::KeylistUpdateResponse,
            ) => Some(MediationCoordinationState::RecipientKeyRegistered),
            (
                MediationCoordinationState::MediationGranted
                | MediationCoordinationState::RecipientKeyRegistered,
                MediationCoordinationEvent::KeylistQuery,
            ) => Some(MediationCoordinationState::KeylistQuerySubmitted),
            (
                MediationCoordinationState::KeylistQuerySubmitted,
                MediationCoordinationEvent::Keylist,
            ) => Some(MediationCoordinationState::KeylistReceived),
            _ => None,
        }
    }
}

impl Default for MediationCoordinationStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

/// Message pickup protocol state.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum MessagePickupState {
    /// No pickup message has been accepted.
    Initial,
    /// Pickup status was requested.
    StatusRequested,
    /// Pickup status was received.
    StatusReceived,
    /// Message delivery was requested.
    DeliveryRequested,
    /// Delivered messages were acknowledged as received.
    MessagesReceived,
    /// Live delivery mode was changed.
    LiveDeliveryChanged,
}

impl MessagePickupState {
    /// Stable fixture state id.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Initial => "pickup_initial",
            Self::StatusRequested => "pickup_status_requested",
            Self::StatusReceived => "pickup_status_received",
            Self::DeliveryRequested => "pickup_delivery_requested",
            Self::MessagesReceived => "pickup_completed",
            Self::LiveDeliveryChanged => "pickup_live_delivery_changed",
        }
    }
}

/// Message pickup event.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum MessagePickupEvent {
    /// `status-request` message.
    StatusRequest,
    /// `status` message.
    Status,
    /// `delivery-request` message.
    DeliveryRequest,
    /// `messages-received` message.
    MessagesReceived,
    /// `live-delivery-change` message.
    LiveDeliveryChange,
}

impl MessagePickupEvent {
    /// Build a pickup event from a parsed `DIDComm` message type.
    #[must_use]
    pub fn from_message_type(message_type: &DidCommMessageType) -> Option<Self> {
        if message_type.protocol_kind() != DidCommProtocolKind::MessagePickup {
            return None;
        }

        let event = match message_type.name() {
            "status-request" => Self::StatusRequest,
            "status" => Self::Status,
            "delivery-request" => Self::DeliveryRequest,
            "messages-received" => Self::MessagesReceived,
            "live-delivery-change" => Self::LiveDeliveryChange,
            _ => return None,
        };
        Some(event)
    }
}

/// One accepted message pickup transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MessagePickupTransition {
    /// State before the event was applied.
    pub from: MessagePickupState,
    /// Accepted event.
    pub event: MessagePickupEvent,
    /// State after the event was applied.
    pub to: MessagePickupState,
}

/// Stateful message pickup transition boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MessagePickupStateMachine {
    state: MessagePickupState,
}

impl MessagePickupStateMachine {
    /// Create a message pickup state machine.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            state: MessagePickupState::Initial,
        }
    }

    /// Current message pickup state.
    #[must_use]
    pub const fn state(&self) -> MessagePickupState {
        self.state
    }

    /// Apply one message pickup event.
    ///
    /// # Errors
    ///
    /// Returns a typed, redaction-safe conflict when the event is not valid
    /// for the current state.
    pub fn apply(&mut self, event: MessagePickupEvent) -> IdentusResult<MessagePickupTransition> {
        let from = self.state;
        let Some(to) = self.next_state(event) else {
            return Err(invalid_didcomm_transition());
        };
        self.state = to;
        Ok(MessagePickupTransition { from, event, to })
    }

    fn next_state(&self, event: MessagePickupEvent) -> Option<MessagePickupState> {
        match (self.state, event) {
            (MessagePickupState::Initial, MessagePickupEvent::StatusRequest) => {
                Some(MessagePickupState::StatusRequested)
            }
            (MessagePickupState::StatusRequested, MessagePickupEvent::Status) => {
                Some(MessagePickupState::StatusReceived)
            }
            (MessagePickupState::StatusReceived, MessagePickupEvent::DeliveryRequest) => {
                Some(MessagePickupState::DeliveryRequested)
            }
            (MessagePickupState::DeliveryRequested, MessagePickupEvent::MessagesReceived) => {
                Some(MessagePickupState::MessagesReceived)
            }
            (
                MessagePickupState::StatusReceived | MessagePickupState::MessagesReceived,
                MessagePickupEvent::LiveDeliveryChange,
            ) => Some(MessagePickupState::LiveDeliveryChanged),
            _ => None,
        }
    }
}

impl Default for MessagePickupStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

/// Mediator routing protocol state.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum MediatorRoutingState {
    /// No routing message has been accepted.
    Initial,
    /// A forwarded message was accepted by mediator routing.
    MessageForwarded,
}

impl MediatorRoutingState {
    /// Stable fixture state id.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Initial => "routing_initial",
            Self::MessageForwarded => "message_forwarded",
        }
    }
}

/// Mediator routing event.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum MediatorRoutingEvent {
    /// `forward` message.
    Forward,
}

impl MediatorRoutingEvent {
    /// Build a routing event from a parsed `DIDComm` message type.
    #[must_use]
    pub fn from_message_type(message_type: &DidCommMessageType) -> Option<Self> {
        if matches!(
            (message_type.protocol_kind(), message_type.name()),
            (DidCommProtocolKind::Routing, "forward")
        ) {
            return Some(Self::Forward);
        }
        None
    }
}

/// One accepted mediator routing transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MediatorRoutingTransition {
    /// State before the event was applied.
    pub from: MediatorRoutingState,
    /// Accepted event.
    pub event: MediatorRoutingEvent,
    /// State after the event was applied.
    pub to: MediatorRoutingState,
}

/// Stateful mediator routing transition boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MediatorRoutingStateMachine {
    state: MediatorRoutingState,
}

impl MediatorRoutingStateMachine {
    /// Create a mediator routing state machine.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            state: MediatorRoutingState::Initial,
        }
    }

    /// Current mediator routing state.
    #[must_use]
    pub const fn state(&self) -> MediatorRoutingState {
        self.state
    }

    /// Apply one mediator routing event.
    ///
    /// # Errors
    ///
    /// Returns a typed, redaction-safe conflict when the event is not valid
    /// for the current state.
    pub fn apply(
        &mut self,
        event: MediatorRoutingEvent,
    ) -> IdentusResult<MediatorRoutingTransition> {
        let from = self.state;
        let Some(to) = self.next_state(event) else {
            return Err(invalid_didcomm_transition());
        };
        self.state = to;
        Ok(MediatorRoutingTransition { from, event, to })
    }

    fn next_state(&self, event: MediatorRoutingEvent) -> Option<MediatorRoutingState> {
        match (self.state, event) {
            (MediatorRoutingState::Initial, MediatorRoutingEvent::Forward) => {
                Some(MediatorRoutingState::MessageForwarded)
            }
            _ => None,
        }
    }
}

impl Default for MediatorRoutingStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

/// Issue Credential protocol state.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum IssueCredentialState {
    /// No issue-credential message has been accepted.
    Initial,
    /// Credential proposal has been sent.
    CredentialProposed,
    /// Credential offer has been received.
    CredentialOfferReceived,
    /// Credential has been requested.
    CredentialRequested,
    /// Credential has been issued.
    CredentialIssued,
}

impl IssueCredentialState {
    /// Stable fixture state id.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Initial => "issue_credential_initial",
            Self::CredentialProposed => "credential_proposed",
            Self::CredentialOfferReceived => "credential_offer_received",
            Self::CredentialRequested => "credential_requested",
            Self::CredentialIssued => "credential_issued",
        }
    }
}

/// Issue Credential event.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum IssueCredentialEvent {
    /// `propose-credential` message.
    ProposeCredential,
    /// `offer-credential` message.
    OfferCredential,
    /// `request-credential` message.
    RequestCredential,
    /// `issue-credential` message.
    IssueCredential,
    /// Compatibility alias seen in some `DIDComm` issue-credential material.
    CredentialCredential,
}

impl IssueCredentialEvent {
    /// Build an issue-credential event from a parsed `DIDComm` message type.
    #[must_use]
    pub fn from_message_type(message_type: &DidCommMessageType) -> Option<Self> {
        if message_type.protocol_kind() != DidCommProtocolKind::IssueCredential {
            return None;
        }

        let event = match message_type.name() {
            "propose-credential" => Self::ProposeCredential,
            "offer-credential" => Self::OfferCredential,
            "request-credential" => Self::RequestCredential,
            "issue-credential" => Self::IssueCredential,
            "credential-credential" => Self::CredentialCredential,
            _ => return None,
        };
        Some(event)
    }
}

/// One accepted Issue Credential transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IssueCredentialTransition {
    /// State before the event was applied.
    pub from: IssueCredentialState,
    /// Accepted event.
    pub event: IssueCredentialEvent,
    /// State after the event was applied.
    pub to: IssueCredentialState,
}

/// Stateful Issue Credential transition boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueCredentialStateMachine {
    state: IssueCredentialState,
}

impl IssueCredentialStateMachine {
    /// Create an issue-credential state machine.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            state: IssueCredentialState::Initial,
        }
    }

    /// Current issue-credential state.
    #[must_use]
    pub const fn state(&self) -> IssueCredentialState {
        self.state
    }

    /// Apply one issue-credential event.
    ///
    /// # Errors
    ///
    /// Returns a typed, redaction-safe conflict when the event is not valid
    /// for the current state.
    pub fn apply(
        &mut self,
        event: IssueCredentialEvent,
    ) -> IdentusResult<IssueCredentialTransition> {
        let from = self.state;
        let Some(to) = self.next_state(event) else {
            return Err(invalid_didcomm_transition());
        };
        self.state = to;
        Ok(IssueCredentialTransition { from, event, to })
    }

    fn next_state(&self, event: IssueCredentialEvent) -> Option<IssueCredentialState> {
        match (self.state, event) {
            (IssueCredentialState::Initial, IssueCredentialEvent::ProposeCredential) => {
                Some(IssueCredentialState::CredentialProposed)
            }
            (
                IssueCredentialState::Initial | IssueCredentialState::CredentialProposed,
                IssueCredentialEvent::OfferCredential,
            ) => Some(IssueCredentialState::CredentialOfferReceived),
            (
                IssueCredentialState::Initial | IssueCredentialState::CredentialOfferReceived,
                IssueCredentialEvent::RequestCredential,
            ) => Some(IssueCredentialState::CredentialRequested),
            (
                IssueCredentialState::CredentialRequested,
                IssueCredentialEvent::IssueCredential | IssueCredentialEvent::CredentialCredential,
            ) => Some(IssueCredentialState::CredentialIssued),
            _ => None,
        }
    }
}

impl Default for IssueCredentialStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

/// Present Proof protocol state.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PresentProofState {
    /// No present-proof message has been accepted.
    Initial,
    /// Presentation proposal has been sent.
    PresentationProposed,
    /// Presentation was requested.
    PresentationRequested,
    /// Presentation was submitted.
    PresentationSubmitted,
}

impl PresentProofState {
    /// Stable fixture state id.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Initial => "present_proof_initial",
            Self::PresentationProposed => "presentation_proposed",
            Self::PresentationRequested => "presentation_requested",
            Self::PresentationSubmitted => "presentation_submitted",
        }
    }
}

/// Present Proof event.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PresentProofEvent {
    /// `propose-presentation` message.
    ProposePresentation,
    /// `request-presentation` message.
    RequestPresentation,
    /// `presentation` message.
    Presentation,
}

impl PresentProofEvent {
    /// Build a present-proof event from a parsed `DIDComm` message type.
    #[must_use]
    pub fn from_message_type(message_type: &DidCommMessageType) -> Option<Self> {
        if message_type.protocol_kind() != DidCommProtocolKind::PresentProof {
            return None;
        }

        let event = match message_type.name() {
            "propose-presentation" => Self::ProposePresentation,
            "request-presentation" => Self::RequestPresentation,
            "presentation" => Self::Presentation,
            _ => return None,
        };
        Some(event)
    }
}

/// One accepted Present Proof transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PresentProofTransition {
    /// State before the event was applied.
    pub from: PresentProofState,
    /// Accepted event.
    pub event: PresentProofEvent,
    /// State after the event was applied.
    pub to: PresentProofState,
}

/// Stateful Present Proof transition boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PresentProofStateMachine {
    state: PresentProofState,
}

impl PresentProofStateMachine {
    /// Create a present-proof state machine.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            state: PresentProofState::Initial,
        }
    }

    /// Current present-proof state.
    #[must_use]
    pub const fn state(&self) -> PresentProofState {
        self.state
    }

    /// Apply one present-proof event.
    ///
    /// # Errors
    ///
    /// Returns a typed, redaction-safe conflict when the event is not valid
    /// for the current state.
    pub fn apply(&mut self, event: PresentProofEvent) -> IdentusResult<PresentProofTransition> {
        let from = self.state;
        let Some(to) = self.next_state(event) else {
            return Err(invalid_didcomm_transition());
        };
        self.state = to;
        Ok(PresentProofTransition { from, event, to })
    }

    fn next_state(&self, event: PresentProofEvent) -> Option<PresentProofState> {
        match (self.state, event) {
            (PresentProofState::Initial, PresentProofEvent::ProposePresentation) => {
                Some(PresentProofState::PresentationProposed)
            }
            (
                PresentProofState::Initial | PresentProofState::PresentationProposed,
                PresentProofEvent::RequestPresentation,
            ) => Some(PresentProofState::PresentationRequested),
            (
                PresentProofState::Initial | PresentProofState::PresentationRequested,
                PresentProofEvent::Presentation,
            ) => Some(PresentProofState::PresentationSubmitted),
            _ => None,
        }
    }
}

impl Default for PresentProofStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

/// Report Problem protocol state.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ReportProblemState {
    /// No report-problem message has been accepted.
    Initial,
    /// Problem report was recorded.
    ProblemReportRecorded,
}

impl ReportProblemState {
    /// Stable fixture state id.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Initial => "report_problem_initial",
            Self::ProblemReportRecorded => "problem_report_recorded",
        }
    }
}

/// Report Problem event.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ReportProblemEvent {
    /// `problem-report` message.
    ProblemReport,
}

impl ReportProblemEvent {
    /// Build a report-problem event from a parsed `DIDComm` message type.
    #[must_use]
    pub fn from_message_type(message_type: &DidCommMessageType) -> Option<Self> {
        if matches!(
            (message_type.protocol_kind(), message_type.name()),
            (DidCommProtocolKind::ReportProblem, "problem-report")
        ) {
            return Some(Self::ProblemReport);
        }
        None
    }
}

/// One accepted Report Problem transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReportProblemTransition {
    /// State before the event was applied.
    pub from: ReportProblemState,
    /// Accepted event.
    pub event: ReportProblemEvent,
    /// State after the event was applied.
    pub to: ReportProblemState,
}

/// Stateful Report Problem transition boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportProblemStateMachine {
    state: ReportProblemState,
}

impl ReportProblemStateMachine {
    /// Create a report-problem state machine.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            state: ReportProblemState::Initial,
        }
    }

    /// Current report-problem state.
    #[must_use]
    pub const fn state(&self) -> ReportProblemState {
        self.state
    }

    /// Apply one report-problem event.
    ///
    /// # Errors
    ///
    /// Returns a typed, redaction-safe conflict when the event is not valid
    /// for the current state.
    pub fn apply(&mut self, event: ReportProblemEvent) -> IdentusResult<ReportProblemTransition> {
        let from = self.state;
        let Some(to) = self.next_state(event) else {
            return Err(invalid_didcomm_transition());
        };
        self.state = to;
        Ok(ReportProblemTransition { from, event, to })
    }

    fn next_state(&self, event: ReportProblemEvent) -> Option<ReportProblemState> {
        match (self.state, event) {
            (ReportProblemState::Initial, ReportProblemEvent::ProblemReport) => {
                Some(ReportProblemState::ProblemReportRecorded)
            }
            _ => None,
        }
    }
}

impl Default for ReportProblemStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

/// Trust Ping protocol state.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum TrustPingState {
    /// No trust-ping message has been accepted.
    Initial,
    /// Ping was sent.
    PingSent,
    /// Ping response was received.
    PingAcknowledged,
}

impl TrustPingState {
    /// Stable fixture state id.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Initial => "trust_ping_initial",
            Self::PingSent => "trust_ping_sent",
            Self::PingAcknowledged => "trust_ping_acknowledged",
        }
    }
}

/// Trust Ping event.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum TrustPingEvent {
    /// `ping` message.
    Ping,
    /// `ping-response` message.
    PingResponse,
}

impl TrustPingEvent {
    /// Build a trust-ping event from a parsed `DIDComm` message type.
    #[must_use]
    pub fn from_message_type(message_type: &DidCommMessageType) -> Option<Self> {
        if message_type.protocol_kind() != DidCommProtocolKind::TrustPing {
            return None;
        }

        let event = match message_type.name() {
            "ping" => Self::Ping,
            "ping-response" => Self::PingResponse,
            _ => return None,
        };
        Some(event)
    }
}

/// One accepted Trust Ping transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TrustPingTransition {
    /// State before the event was applied.
    pub from: TrustPingState,
    /// Accepted event.
    pub event: TrustPingEvent,
    /// State after the event was applied.
    pub to: TrustPingState,
}

/// Stateful Trust Ping transition boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrustPingStateMachine {
    state: TrustPingState,
}

impl TrustPingStateMachine {
    /// Create a trust-ping state machine.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            state: TrustPingState::Initial,
        }
    }

    /// Current trust-ping state.
    #[must_use]
    pub const fn state(&self) -> TrustPingState {
        self.state
    }

    /// Apply one trust-ping event.
    ///
    /// # Errors
    ///
    /// Returns a typed, redaction-safe conflict when the event is not valid
    /// for the current state.
    pub fn apply(&mut self, event: TrustPingEvent) -> IdentusResult<TrustPingTransition> {
        let from = self.state;
        let Some(to) = self.next_state(event) else {
            return Err(invalid_didcomm_transition());
        };
        self.state = to;
        Ok(TrustPingTransition { from, event, to })
    }

    fn next_state(&self, event: TrustPingEvent) -> Option<TrustPingState> {
        match (self.state, event) {
            (TrustPingState::Initial, TrustPingEvent::Ping) => Some(TrustPingState::PingSent),
            (TrustPingState::PingSent, TrustPingEvent::PingResponse) => {
                Some(TrustPingState::PingAcknowledged)
            }
            _ => None,
        }
    }
}

impl Default for TrustPingStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

/// Revocation notification protocol state.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RevocationNotificationState {
    /// No revocation notification has been accepted.
    Initial,
    /// Credential revocation notification was received.
    CredentialRevoked,
}

impl RevocationNotificationState {
    /// Stable fixture state id.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Initial => "revocation_notification_initial",
            Self::CredentialRevoked => "credential_revoked",
        }
    }
}

/// Revocation notification event.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RevocationNotificationEvent {
    /// `revoke` message.
    Revoke,
}

impl RevocationNotificationEvent {
    /// Build a revocation notification event from a parsed `DIDComm` message type.
    #[must_use]
    pub fn from_message_type(message_type: &DidCommMessageType) -> Option<Self> {
        if matches!(
            (message_type.protocol_kind(), message_type.name()),
            (DidCommProtocolKind::RevocationNotification, "revoke")
        ) {
            return Some(Self::Revoke);
        }
        None
    }
}

/// One accepted revocation notification transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RevocationNotificationTransition {
    /// State before the event was applied.
    pub from: RevocationNotificationState,
    /// Accepted event.
    pub event: RevocationNotificationEvent,
    /// State after the event was applied.
    pub to: RevocationNotificationState,
}

/// Stateful revocation notification transition boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RevocationNotificationStateMachine {
    state: RevocationNotificationState,
}

impl RevocationNotificationStateMachine {
    /// Create a revocation notification state machine.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            state: RevocationNotificationState::Initial,
        }
    }

    /// Current revocation notification state.
    #[must_use]
    pub const fn state(&self) -> RevocationNotificationState {
        self.state
    }

    /// Apply one revocation notification event.
    ///
    /// # Errors
    ///
    /// Returns a typed, redaction-safe conflict when the event is not valid
    /// for the current state.
    pub fn apply(
        &mut self,
        event: RevocationNotificationEvent,
    ) -> IdentusResult<RevocationNotificationTransition> {
        let from = self.state;
        let Some(to) = self.next_state(event) else {
            return Err(invalid_didcomm_transition());
        };
        self.state = to;
        Ok(RevocationNotificationTransition { from, event, to })
    }

    fn next_state(
        &self,
        event: RevocationNotificationEvent,
    ) -> Option<RevocationNotificationState> {
        match (self.state, event) {
            (RevocationNotificationState::Initial, RevocationNotificationEvent::Revoke) => {
                Some(RevocationNotificationState::CredentialRevoked)
            }
            _ => None,
        }
    }
}

impl Default for RevocationNotificationStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

/// Errors returned while parsing `DIDComm` domain primitives.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DidCommParseError {
    /// The message type does not start with `https://didcomm.org/`.
    MissingDidCommPrefix,
    /// A required path segment is missing.
    MissingPathSegment(&'static str),
    /// A path segment contains an invalid character.
    InvalidPathSegment(&'static str),
    /// The version segment is not a dotted numeric protocol version.
    InvalidVersionSegment,
    /// The message type has more path segments than expected.
    UnexpectedPathSegment,
    /// An id-like value is empty or contains invalid characters.
    InvalidIdentifier(&'static str),
    /// Addressed messages require at least one recipient.
    MissingRecipient,
}

impl DidCommParseError {
    /// Stable core error code for this messaging failure.
    #[must_use]
    pub const fn code(&self) -> ErrorCode {
        match self {
            Self::MissingDidCommPrefix => ErrorCode::new("missing_didcomm_prefix"),
            Self::MissingPathSegment(_) => ErrorCode::new("missing_didcomm_path_segment"),
            Self::InvalidPathSegment(_) => ErrorCode::new("invalid_didcomm_path_segment"),
            Self::InvalidVersionSegment => ErrorCode::new("invalid_didcomm_version"),
            Self::UnexpectedPathSegment => ErrorCode::new("unexpected_didcomm_path_segment"),
            Self::InvalidIdentifier(_) => ErrorCode::new("invalid_didcomm_identifier"),
            Self::MissingRecipient => ErrorCode::new("missing_didcomm_recipient"),
        }
    }

    /// Shared core error family for this messaging failure.
    #[must_use]
    pub const fn kind(&self) -> ErrorKind {
        ErrorKind::InvalidInput
    }

    /// Redaction-safe public error message for binding and service boundaries.
    #[must_use]
    pub const fn public_message(&self) -> &'static str {
        match self {
            Self::MissingDidCommPrefix => {
                "DIDComm message type must start with https://didcomm.org/"
            }
            Self::MissingPathSegment(_) => "DIDComm message type is missing a path segment",
            Self::InvalidPathSegment(_) => "DIDComm message type has an invalid path segment",
            Self::InvalidVersionSegment => "DIDComm message type has invalid version",
            Self::UnexpectedPathSegment => "DIDComm message type has unexpected path segment",
            Self::InvalidIdentifier(_) => "DIDComm identifier is invalid",
            Self::MissingRecipient => "DIDComm addressed message requires at least one recipient",
        }
    }

    /// Convert to the shared core error surface.
    #[must_use]
    pub const fn to_identus_error(&self) -> IdentusError {
        IdentusError::public(
            self.code(),
            self.kind(),
            CapabilityId::new("didcomm"),
            self.public_message(),
        )
    }

    /// Convert to a binding-safe error envelope.
    #[must_use]
    pub fn to_error_envelope(&self) -> ErrorEnvelope {
        ErrorEnvelope::from_error(&self.to_identus_error())
    }
}

impl Display for DidCommParseError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingDidCommPrefix => {
                formatter.write_str("DIDComm message type must start with https://didcomm.org/")
            }
            Self::MissingPathSegment(segment) => {
                write!(formatter, "DIDComm message type is missing {segment}")
            }
            Self::InvalidPathSegment(segment) => {
                write!(formatter, "DIDComm message type has invalid {segment}")
            }
            Self::InvalidVersionSegment => {
                formatter.write_str("DIDComm message type has invalid version")
            }
            Self::UnexpectedPathSegment => {
                formatter.write_str("DIDComm message type has unexpected path segment")
            }
            Self::InvalidIdentifier(identifier) => {
                write!(formatter, "DIDComm {identifier} is invalid")
            }
            Self::MissingRecipient => {
                formatter.write_str("DIDComm addressed message requires at least one recipient")
            }
        }
    }
}

impl Error for DidCommParseError {}

impl From<DidCommParseError> for IdentusError {
    fn from(error: DidCommParseError) -> Self {
        error.to_identus_error()
    }
}

/// Invalid `DIDComm` state transition typed error.
#[must_use]
pub const fn invalid_didcomm_transition() -> IdentusError {
    IdentusError::public(
        ErrorCode::new("invalid_didcomm_transition"),
        ErrorKind::Conflict,
        DIDCOMM_CAPABILITY,
        "DIDComm state transition is invalid",
    )
}

const fn protocol(
    id: &'static str,
    kind: DidCommProtocolKind,
    uri: &'static str,
    message_names: &'static [&'static str],
    stage: DidCommSupportStage,
    source: &'static str,
    backlog_task: &'static str,
) -> DidCommProtocolProfile {
    DidCommProtocolProfile {
        id,
        kind,
        uri,
        message_names,
        stage,
        source,
        backlog_task,
    }
}

fn parse_path_segment(
    segment: Option<&str>,
    segment_name: &'static str,
) -> Result<String, DidCommParseError> {
    let segment = segment.ok_or(DidCommParseError::MissingPathSegment(segment_name))?;
    if segment.is_empty()
        || !segment.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'-' | b'_')
        })
    {
        return Err(DidCommParseError::InvalidPathSegment(segment_name));
    }

    Ok(segment.to_owned())
}

fn parse_version_segment(segment: Option<&str>) -> Result<String, DidCommParseError> {
    let version = segment.ok_or(DidCommParseError::MissingPathSegment("version"))?;
    let mut parts = version.split('.');
    let major = parts.next().unwrap_or_default();
    let minor = parts.next().unwrap_or_default();

    if major.is_empty()
        || minor.is_empty()
        || parts.next().is_some()
        || !major.bytes().all(|byte| byte.is_ascii_digit())
        || !minor.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Err(DidCommParseError::InvalidVersionSegment);
    }

    Ok(version.to_owned())
}

fn parse_identifier(input: &str, name: &'static str) -> Result<String, DidCommParseError> {
    if input.is_empty()
        || !input
            .bytes()
            .all(|byte| byte.is_ascii_graphic() && !byte.is_ascii_whitespace())
    {
        return Err(DidCommParseError::InvalidIdentifier(name));
    }

    Ok(input.to_owned())
}

#[cfg(test)]
mod tests {
    use super::{
        DIDCOMM_PROTOCOL_PROFILES, DidCommMessageId, DidCommMessageType, DidCommParseError,
        DidCommPlaintextMessage, DidCommProtocolKind, DidCommSupportStage, DidCommThreadId,
        IssueCredentialEvent, IssueCredentialState, IssueCredentialStateMachine,
        MediationCoordinationEvent, MediationCoordinationState, MediationCoordinationStateMachine,
        MediatorRoutingEvent, MediatorRoutingState, MediatorRoutingStateMachine,
        MessagePickupEvent, MessagePickupState, MessagePickupStateMachine, PresentProofEvent,
        PresentProofState, PresentProofStateMachine, ReportProblemEvent, ReportProblemState,
        ReportProblemStateMachine, RevocationNotificationEvent, RevocationNotificationState,
        RevocationNotificationStateMachine, TrustPingEvent, TrustPingState, TrustPingStateMachine,
        invalid_didcomm_transition,
    };
    use identus_core::{CapabilityId, ErrorKind};
    use identus_did::Did;
    use std::collections::BTreeSet;

    #[test]
    fn parses_identus_didcomm_protocol_message_types() {
        for (message_type, family, version, name, kind) in [
            (
                "https://didcomm.org/out-of-band/2.0/invitation",
                "out-of-band",
                "2.0",
                "invitation",
                DidCommProtocolKind::OutOfBand,
            ),
            (
                "https://didcomm.org/basicmessage/2.0/message",
                "basicmessage",
                "2.0",
                "message",
                DidCommProtocolKind::BasicMessage,
            ),
            (
                "https://didcomm.org/trust-ping/2.0/ping",
                "trust-ping",
                "2.0",
                "ping",
                DidCommProtocolKind::TrustPing,
            ),
            (
                "https://didcomm.org/routing/2.0/forward",
                "routing",
                "2.0",
                "forward",
                DidCommProtocolKind::Routing,
            ),
            (
                "https://didcomm.org/discover-features/2.0/queries",
                "discover-features",
                "2.0",
                "queries",
                DidCommProtocolKind::DiscoverFeatures,
            ),
            (
                "https://didcomm.org/coordinate-mediation/2.0/mediate-request",
                "coordinate-mediation",
                "2.0",
                "mediate-request",
                DidCommProtocolKind::CoordinateMediation,
            ),
            (
                "https://didcomm.org/messagepickup/3.0/status-request",
                "messagepickup",
                "3.0",
                "status-request",
                DidCommProtocolKind::MessagePickup,
            ),
            (
                "https://didcomm.org/issue-credential/3.0/offer-credential",
                "issue-credential",
                "3.0",
                "offer-credential",
                DidCommProtocolKind::IssueCredential,
            ),
            (
                "https://didcomm.org/present-proof/3.0/request-presentation",
                "present-proof",
                "3.0",
                "request-presentation",
                DidCommProtocolKind::PresentProof,
            ),
            (
                "https://didcomm.org/report-problem/2.0/problem-report",
                "report-problem",
                "2.0",
                "problem-report",
                DidCommProtocolKind::ReportProblem,
            ),
            (
                "https://didcomm.org/revocation-notification/1.0/revoke",
                "revocation-notification",
                "1.0",
                "revoke",
                DidCommProtocolKind::RevocationNotification,
            ),
        ] {
            let parsed = DidCommMessageType::parse(message_type).expect("valid DIDComm URI");
            assert_eq!(parsed.as_str(), message_type);
            assert_eq!(parsed.family(), family);
            assert_eq!(parsed.version(), version);
            assert_eq!(parsed.name(), name);
            assert_eq!(parsed.protocol_kind(), kind);
        }
    }

    #[test]
    fn rejects_malformed_message_types_and_identifiers() {
        for invalid in [
            "https://example.com/basicmessage/2.0/message",
            "https://didcomm.org/basicmessage",
            "https://didcomm.org/basicmessage/two/message",
            "https://didcomm.org/basicmessage/2.0/",
            "https://didcomm.org/basicmessage/2.0/message/extra",
            "https://didcomm.org/BasicMessage/2.0/message",
        ] {
            assert!(
                DidCommMessageType::parse(invalid).is_err(),
                "{invalid} should be rejected"
            );
        }

        assert!(DidCommMessageId::parse("").is_err());
        assert!(DidCommMessageId::parse("has space").is_err());
        assert!(DidCommThreadId::parse("\n").is_err());
    }

    #[test]
    fn recognizes_compatibility_aliases() {
        for family in [
            "message-pickup",
            "pickup",
            "mediator-coordination",
            "connections",
            "didexchange",
        ] {
            assert_ne!(
                DidCommProtocolKind::from_family(family),
                DidCommProtocolKind::Other
            );
        }
    }

    #[test]
    fn protocol_catalog_tracks_backlog_and_message_names() {
        let mut ids = BTreeSet::new();
        for profile in DIDCOMM_PROTOCOL_PROFILES {
            assert!(ids.insert(profile.id), "duplicate {}", profile.id);
            assert!(profile.uri.starts_with("https://didcomm.org/"));
            assert!(profile.backlog_task.starts_with('T'));
            assert!(!profile.source.is_empty());
        }

        for required in [
            "didcomm-oob-2",
            "didcomm-basicmessage-2",
            "trust-ping-2",
            "routing-2",
            "discover-features-2",
            "coordinate-mediation-2",
            "message-pickup-3",
            "didcomm-issue-credential-3",
            "didcomm-present-proof-3",
            "didcomm-report-problem-2",
            "didcomm-revocation-notification-1",
            "didcomm-did-rotate-2",
        ] {
            assert!(
                DIDCOMM_PROTOCOL_PROFILES
                    .iter()
                    .any(|profile| profile.id == required),
                "missing {required}"
            );
        }

        assert!(
            DIDCOMM_PROTOCOL_PROFILES.iter().any(|profile| {
                profile.stage == DidCommSupportStage::Roadmap
                    && profile.id == "coordinate-mediation-3"
            }),
            "coordinate mediation 3.0 must stay visible as a roadmap item"
        );
    }

    #[test]
    fn builds_plaintext_message_with_typed_dids_and_thread_ids() {
        let sender = Did::parse("did:peer:2.Ez6LSsender").expect("sender DID");
        let recipient = Did::parse("did:peer:2.Ez6LSrecipient").expect("recipient DID");
        let message = DidCommPlaintextMessage::addressed(
            DidCommMessageId::parse("msg-1").expect("message id"),
            DidCommMessageType::parse("https://didcomm.org/basicmessage/2.0/message")
                .expect("message type"),
            sender.clone(),
            vec![recipient.clone()],
        )
        .expect("addressed message")
        .with_thread(
            DidCommThreadId::parse("thread-1").expect("thread id"),
            Some(DidCommThreadId::parse("parent-1").expect("parent thread id")),
        );

        assert_eq!(message.id().as_str(), "msg-1");
        assert_eq!(
            message.message_type().protocol_kind(),
            DidCommProtocolKind::BasicMessage
        );
        assert_eq!(message.from(), Some(&sender));
        assert_eq!(message.to(), &[recipient]);
        assert_eq!(
            message.thread_id().map(DidCommThreadId::as_str),
            Some("thread-1")
        );
        assert_eq!(
            message.parent_thread_id().map(DidCommThreadId::as_str),
            Some("parent-1")
        );
    }

    #[test]
    fn addressed_message_requires_recipient() {
        let sender = Did::parse("did:peer:2.Ez6LSsender").expect("sender DID");
        let result = DidCommPlaintextMessage::addressed(
            DidCommMessageId::parse("msg-1").expect("message id"),
            DidCommMessageType::parse("https://didcomm.org/basicmessage/2.0/message")
                .expect("message type"),
            sender,
            Vec::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn didcomm_parse_errors_map_to_core_error_surface() {
        let parse_error = DidCommParseError::InvalidIdentifier("message id");
        let core_error = parse_error.to_identus_error();

        assert_eq!(core_error.code().as_str(), "invalid_didcomm_identifier");
        assert_eq!(core_error.kind(), ErrorKind::InvalidInput);
        assert_eq!(
            core_error.capability().map(CapabilityId::as_str),
            Some("didcomm")
        );
        assert_eq!(core_error.public_message(), "DIDComm identifier is invalid");
        assert_eq!(
            core_error.to_string(),
            "invalid_didcomm_identifier: DIDComm identifier is invalid"
        );

        let envelope = parse_error.to_error_envelope();
        assert_eq!(envelope.code, "invalid_didcomm_identifier");
        assert_eq!(envelope.kind, ErrorKind::InvalidInput);
        assert_eq!(envelope.capability, Some("didcomm"));
        assert_eq!(envelope.message, "DIDComm identifier is invalid");
    }

    #[test]
    fn didcomm_parse_with_core_error_preserves_typed_codes() {
        let type_error = DidCommMessageType::parse_with_core_error(
            "https://didcomm.org/basicmessage/two/message",
        )
        .expect_err("message type must fail");
        assert_eq!(type_error.code().as_str(), "invalid_didcomm_version");

        let id_error =
            DidCommMessageId::parse_with_core_error("has space").expect_err("id must fail");
        assert_eq!(id_error.code().as_str(), "invalid_didcomm_identifier");

        let sender = Did::parse("did:peer:2.Ez6LSsender").expect("sender DID");
        let recipient_error = DidCommPlaintextMessage::addressed_with_core_error(
            DidCommMessageId::parse("msg-1").expect("message id"),
            DidCommMessageType::parse("https://didcomm.org/basicmessage/2.0/message")
                .expect("message type"),
            sender,
            Vec::new(),
        )
        .expect_err("recipient must be required");
        assert_eq!(recipient_error.code().as_str(), "missing_didcomm_recipient");
    }

    #[test]
    fn mediation_coordination_state_machine_accepts_grant_and_keylist_update() {
        let mut mediation = MediationCoordinationStateMachine::new();
        for event in [
            MediationCoordinationEvent::MediateRequest,
            MediationCoordinationEvent::MediateGrant,
            MediationCoordinationEvent::KeylistUpdate,
            MediationCoordinationEvent::KeylistUpdateResponse,
        ] {
            mediation
                .apply(event)
                .expect("mediation transition should be accepted");
        }

        assert_eq!(
            mediation.state(),
            MediationCoordinationState::RecipientKeyRegistered
        );
        assert_eq!(mediation.state().as_str(), "recipient_key_registered");
    }

    #[test]
    fn mediation_coordination_state_machine_rejects_invalid_order() {
        let mut mediation = MediationCoordinationStateMachine::new();

        let error = mediation
            .apply(MediationCoordinationEvent::MediateGrant)
            .expect_err("grant cannot be first");

        assert_eq!(error, invalid_didcomm_transition());
        assert_eq!(error.code().as_str(), "invalid_didcomm_transition");
        assert_eq!(error.kind(), ErrorKind::Conflict);
        assert_eq!(
            error.capability().map(CapabilityId::as_str),
            Some("didcomm")
        );
        assert!(!error.to_string().contains("redacted"));
        assert_eq!(mediation.state(), MediationCoordinationState::Initial);
    }

    #[test]
    fn message_pickup_state_machine_accepts_status_delivery_and_ack() {
        let mut pickup = MessagePickupStateMachine::new();
        for event in [
            MessagePickupEvent::StatusRequest,
            MessagePickupEvent::Status,
            MessagePickupEvent::DeliveryRequest,
            MessagePickupEvent::MessagesReceived,
        ] {
            pickup
                .apply(event)
                .expect("pickup transition should be accepted");
        }

        assert_eq!(pickup.state(), MessagePickupState::MessagesReceived);
        assert_eq!(pickup.state().as_str(), "pickup_completed");
    }

    #[test]
    fn mediator_routing_state_machine_accepts_forward_once() {
        let mut routing = MediatorRoutingStateMachine::new();

        routing
            .apply(MediatorRoutingEvent::Forward)
            .expect("routing forward should be accepted");

        assert_eq!(routing.state(), MediatorRoutingState::MessageForwarded);
        assert_eq!(routing.state().as_str(), "message_forwarded");
        assert_eq!(
            routing
                .apply(MediatorRoutingEvent::Forward)
                .expect_err("duplicate forward should be rejected"),
            invalid_didcomm_transition()
        );
    }

    #[test]
    fn mediation_and_pickup_events_parse_from_canonical_and_alias_types() {
        let mediation = DidCommMessageType::parse(
            "https://didcomm.org/coordinate-mediation/2.0/mediate-request",
        )
        .expect("canonical mediation type");
        assert_eq!(
            MediationCoordinationEvent::from_message_type(&mediation),
            Some(MediationCoordinationEvent::MediateRequest)
        );

        let mediation_alias = DidCommMessageType::parse(
            "https://didcomm.org/mediator-coordination/2.0/mediate-request",
        )
        .expect("mediation alias type");
        assert_eq!(
            MediationCoordinationEvent::from_message_type(&mediation_alias),
            Some(MediationCoordinationEvent::MediateRequest)
        );

        let pickup =
            DidCommMessageType::parse("https://didcomm.org/messagepickup/3.0/status-request")
                .expect("canonical pickup type");
        assert_eq!(
            MessagePickupEvent::from_message_type(&pickup),
            Some(MessagePickupEvent::StatusRequest)
        );

        let pickup_alias =
            DidCommMessageType::parse("https://didcomm.org/pickup/3.0/status-request")
                .expect("pickup alias type");
        assert_eq!(
            MessagePickupEvent::from_message_type(&pickup_alias),
            Some(MessagePickupEvent::StatusRequest)
        );
    }

    #[test]
    fn issue_credential_state_machine_accepts_offer_request_issue() {
        let mut flow = IssueCredentialStateMachine::new();
        for event in [
            IssueCredentialEvent::OfferCredential,
            IssueCredentialEvent::RequestCredential,
            IssueCredentialEvent::IssueCredential,
        ] {
            flow.apply(event)
                .expect("issue-credential transition should be accepted");
        }

        assert_eq!(flow.state(), IssueCredentialState::CredentialIssued);
        assert_eq!(flow.state().as_str(), "credential_issued");
    }

    #[test]
    fn issue_credential_state_machine_rejects_issue_before_request() {
        let mut flow = IssueCredentialStateMachine::new();

        let error = flow
            .apply(IssueCredentialEvent::IssueCredential)
            .expect_err("issue-credential cannot be first");

        assert_eq!(error, invalid_didcomm_transition());
        assert_eq!(error.code().as_str(), "invalid_didcomm_transition");
        assert_eq!(error.kind(), ErrorKind::Conflict);
        assert_eq!(
            error.capability().map(CapabilityId::as_str),
            Some("didcomm")
        );
        assert!(!error.to_string().contains("redacted"));
        assert_eq!(flow.state(), IssueCredentialState::Initial);
    }

    #[test]
    fn present_proof_state_machine_accepts_request_presentation() {
        let mut flow = PresentProofStateMachine::new();
        for event in [
            PresentProofEvent::RequestPresentation,
            PresentProofEvent::Presentation,
        ] {
            flow.apply(event)
                .expect("present-proof transition should be accepted");
        }

        assert_eq!(flow.state(), PresentProofState::PresentationSubmitted);
        assert_eq!(flow.state().as_str(), "presentation_submitted");
    }

    #[test]
    fn report_problem_revocation_and_trust_ping_state_machines_accept_protocol_events() {
        let mut report_problem = ReportProblemStateMachine::new();
        report_problem
            .apply(ReportProblemEvent::ProblemReport)
            .expect("problem report should be accepted");
        assert_eq!(
            report_problem.state(),
            ReportProblemState::ProblemReportRecorded
        );

        let mut revocation = RevocationNotificationStateMachine::new();
        revocation
            .apply(RevocationNotificationEvent::Revoke)
            .expect("revocation notification should be accepted");
        assert_eq!(
            revocation.state(),
            RevocationNotificationState::CredentialRevoked
        );

        let mut trust_ping = TrustPingStateMachine::new();
        for event in [TrustPingEvent::Ping, TrustPingEvent::PingResponse] {
            trust_ping
                .apply(event)
                .expect("trust-ping transition should be accepted");
        }
        assert_eq!(trust_ping.state(), TrustPingState::PingAcknowledged);
    }

    #[test]
    fn protocol_events_parse_from_message_types() {
        for (message_type, expected_state) in [
            (
                "https://didcomm.org/issue-credential/3.0/offer-credential",
                "credential_offer_received",
            ),
            (
                "https://didcomm.org/present-proof/3.0/request-presentation",
                "presentation_requested",
            ),
            (
                "https://didcomm.org/report-problem/2.0/problem-report",
                "problem_report_recorded",
            ),
            (
                "https://didcomm.org/revocation-notification/1.0/revoke",
                "credential_revoked",
            ),
            ("https://didcomm.org/trust-ping/2.0/ping", "trust_ping_sent"),
        ] {
            let message_type = DidCommMessageType::parse(message_type).expect("message type");
            let actual_state = IssueCredentialEvent::from_message_type(&message_type)
                .map(|event| {
                    let mut flow = IssueCredentialStateMachine::new();
                    flow.apply(event).expect("issue event").to.as_str()
                })
                .or_else(|| {
                    PresentProofEvent::from_message_type(&message_type).map(|event| {
                        let mut flow = PresentProofStateMachine::new();
                        flow.apply(event).expect("proof event").to.as_str()
                    })
                })
                .or_else(|| {
                    ReportProblemEvent::from_message_type(&message_type).map(|event| {
                        let mut flow = ReportProblemStateMachine::new();
                        flow.apply(event).expect("report event").to.as_str()
                    })
                })
                .or_else(|| {
                    RevocationNotificationEvent::from_message_type(&message_type).map(|event| {
                        let mut flow = RevocationNotificationStateMachine::new();
                        flow.apply(event).expect("revocation event").to.as_str()
                    })
                })
                .or_else(|| {
                    TrustPingEvent::from_message_type(&message_type).map(|event| {
                        let mut flow = TrustPingStateMachine::new();
                        flow.apply(event).expect("trust-ping event").to.as_str()
                    })
                })
                .expect("message type should map to a protocol event");
            assert_eq!(actual_state, expected_state);
        }
    }
}
