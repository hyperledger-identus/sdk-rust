//! Lightweight agent and embedded mediator model for BDD acceptance tests.
//!
//! This crate intentionally avoids Docker, HTTP servers, databases, and real
//! cryptography. It captures the portable behavior from the existing Identus
//! BDD suites so protocol crates can later replace the internals behind the
//! same acceptance-test shape.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::error::Error;
use std::fmt::{Display, Formatter};

/// Component metadata.
pub const COMPONENT: identus_core::Component = identus_core::Component {
    name: "identus-agent",
    summary: "Issuer, holder, verifier, peer, and embedded mediator test model.",
};

/// Agent role used by acceptance fixtures.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum AgentRole {
    /// Issues credentials.
    Issuer,
    /// Stores credentials and presents proofs.
    Holder,
    /// Requests and verifies presentations.
    Verifier,
    /// Sends and receives peer messages.
    Peer,
}

/// Credential format families covered by current Identus BDD suites.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CredentialFormat {
    /// JWT VC credential flow.
    Jwt,
    /// SD-JWT VC credential flow.
    SdJwt,
    /// `AnonCreds` credential flow.
    AnonCred,
}

impl CredentialFormat {
    #[must_use]
    fn slug(self) -> &'static str {
        match self {
            Self::Jwt => "jwt",
            Self::SdJwt => "sdjwt",
            Self::AnonCred => "anoncred",
        }
    }
}

/// Credential stored by the lightweight agent model.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Credential {
    id: String,
    issuer_id: String,
    subject_id: String,
    format: CredentialFormat,
    revoked: bool,
}

impl Credential {
    /// Stable credential identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Credential format.
    #[must_use]
    pub const fn format(&self) -> CredentialFormat {
        self.format
    }

    /// Whether the credential is revoked in the local wallet view.
    #[must_use]
    pub const fn is_revoked(&self) -> bool {
        self.revoked
    }
}

/// Backup snapshot used by wallet restore BDD scenarios.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackupSnapshot {
    agent_id: String,
    seed: String,
    credentials: Vec<Credential>,
    connections: BTreeSet<String>,
    peer_dids: usize,
    prism_dids: usize,
}

/// Message envelope queued by the embedded mediator.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Envelope {
    /// Sender agent id.
    pub from: String,
    /// Recipient agent id.
    pub to: String,
    /// Message payload.
    pub message: Message,
}

/// Message payloads needed by current portable BDD scenarios.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Message {
    /// Out-of-band connection invitation.
    ConnectionInvitation {
        /// Optional display label.
        label: Option<String>,
        /// Optional goal code.
        goal_code: Option<String>,
        /// Optional human-readable goal.
        goal: Option<String>,
    },
    /// Connection request sent by the invitee.
    ConnectionRequest,
    /// Connection response sent by the inviter.
    ConnectionResponse,
    /// Credential offer.
    CredentialOffer {
        /// Stable offer id.
        offer_id: String,
        /// Credential format.
        format: CredentialFormat,
        /// Number of credentials offered.
        count: usize,
    },
    /// Out-of-band invitation that carries a credential offer attachment.
    OutOfBandCredentialOffer {
        /// Stable invitation id.
        invitation_id: String,
        /// Stable offer id.
        offer_id: String,
        /// Credential format.
        format: CredentialFormat,
        /// Number of credentials offered.
        count: usize,
    },
    /// Credential offer acceptance.
    CredentialAcceptance {
        /// Accepted offer id.
        offer_id: String,
        /// Accepted credential format.
        format: CredentialFormat,
        /// Number of accepted credentials.
        count: usize,
    },
    /// Issued credentials.
    CredentialIssued {
        /// Issued credentials.
        credentials: Vec<Credential>,
    },
    /// Presentation request.
    ProofRequest {
        /// Stable request id.
        request_id: String,
        /// Requested credential format.
        format: CredentialFormat,
        /// Required claim marker used by negative fixtures.
        required_claim: Option<String>,
    },
    /// Out-of-band invitation that carries a presentation request attachment.
    OutOfBandProofRequest {
        /// Stable invitation id.
        invitation_id: String,
        /// Stable request id.
        request_id: String,
        /// Requested credential format.
        format: CredentialFormat,
        /// Required claim marker used by negative fixtures.
        required_claim: Option<String>,
    },
    /// Presentation response.
    ProofPresentation {
        /// Request id being answered.
        request_id: String,
        /// Presented credential format.
        format: CredentialFormat,
        /// Credential revocation state visible to the holder.
        revoked: bool,
    },
    /// Credential revocation notification.
    RevocationNotification {
        /// Revoked credential id.
        credential_id: String,
    },
}

/// In-process mediator that queues messages by recipient id.
#[derive(Default, Debug)]
pub struct EmbeddedMediator {
    queues: BTreeMap<String, VecDeque<Envelope>>,
}

impl EmbeddedMediator {
    /// Create an empty embedded mediator.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            queues: BTreeMap::new(),
        }
    }

    /// Queue a message for a recipient.
    pub fn send(&mut self, envelope: Envelope) {
        self.queues
            .entry(envelope.to.clone())
            .or_default()
            .push_back(envelope);
    }

    /// Drain all messages queued for `recipient`.
    #[must_use]
    pub fn drain(&mut self, recipient: &str) -> Vec<Envelope> {
        self.queues.remove(recipient).unwrap_or_default().into()
    }

    /// Count queued messages for `recipient`.
    #[must_use]
    pub fn queued(&self, recipient: &str) -> usize {
        self.queues.get(recipient).map_or(0, VecDeque::len)
    }

    fn take_first(
        &mut self,
        recipient: &str,
        matches: impl Fn(&Message) -> bool,
    ) -> Option<Envelope> {
        let mut queue = self.queues.remove(recipient).unwrap_or_default();
        let mut kept = VecDeque::new();
        let mut selected = None;

        while let Some(envelope) = queue.pop_front() {
            if selected.is_none() && matches(&envelope.message) {
                selected = Some(envelope);
            } else {
                kept.push_back(envelope);
            }
        }

        if !kept.is_empty() {
            self.queues.insert(recipient.to_owned(), kept);
        }

        selected
    }
}

/// Lightweight agent used by acceptance tests.
#[derive(Clone, Debug)]
pub struct Agent {
    id: String,
    roles: BTreeSet<AgentRole>,
    connections: BTreeSet<String>,
    credentials: Vec<Credential>,
    peer_dids: usize,
    prism_dids: usize,
    nonce: u64,
}

impl Agent {
    /// Create an agent with a stable id and roles.
    #[must_use]
    pub fn new(id: impl Into<String>, roles: impl IntoIterator<Item = AgentRole>) -> Self {
        Self {
            id: id.into(),
            roles: roles.into_iter().collect(),
            connections: BTreeSet::new(),
            credentials: Vec::new(),
            peer_dids: 0,
            prism_dids: 0,
            nonce: 0,
        }
    }

    /// Agent id.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns true when the agent has `role`.
    #[must_use]
    pub fn has_role(&self, role: AgentRole) -> bool {
        self.roles.contains(&role)
    }

    /// Number of stored credentials.
    #[must_use]
    pub fn credential_count(&self) -> usize {
        self.credentials.len()
    }

    /// Returns true when a connection exists with `other`.
    #[must_use]
    pub fn has_connection(&self, other: &Self) -> bool {
        self.connections.contains(other.id())
    }

    /// Count credentials with a given format.
    #[must_use]
    pub fn credential_count_by_format(&self, format: CredentialFormat) -> usize {
        self.credentials
            .iter()
            .filter(|credential| credential.format == format)
            .count()
    }

    /// Send a connection invitation to another agent through the mediator.
    pub fn invite(
        &mut self,
        mediator: &mut EmbeddedMediator,
        to: &Self,
        label: Option<&str>,
        goal_code: Option<&str>,
        goal: Option<&str>,
    ) {
        mediator.send(Envelope {
            from: self.id.clone(),
            to: to.id.clone(),
            message: Message::ConnectionInvitation {
                label: label.map(str::to_owned),
                goal_code: goal_code.map(str::to_owned),
                goal: goal.map(str::to_owned),
            },
        });
    }

    /// Accept the next queued connection invitation.
    ///
    /// # Errors
    ///
    /// Returns [`AgentError::MissingMessage`] when no invitation is queued.
    pub fn accept_next_invitation(
        &mut self,
        mediator: &mut EmbeddedMediator,
    ) -> Result<(), AgentError> {
        let envelope = mediator
            .take_first(self.id(), |message| {
                matches!(message, Message::ConnectionInvitation { .. })
            })
            .ok_or(AgentError::MissingMessage("connection invitation"))?;

        self.connections.insert(envelope.from.clone());
        mediator.send(Envelope {
            from: self.id.clone(),
            to: envelope.from,
            message: Message::ConnectionRequest,
        });
        Ok(())
    }

    /// Process the next connection request and send a response.
    ///
    /// # Errors
    ///
    /// Returns [`AgentError::MissingMessage`] when no request is queued.
    pub fn process_next_connection_request(
        &mut self,
        mediator: &mut EmbeddedMediator,
    ) -> Result<(), AgentError> {
        let envelope = mediator
            .take_first(self.id(), |message| {
                matches!(message, Message::ConnectionRequest)
            })
            .ok_or(AgentError::MissingMessage("connection request"))?;

        self.connections.insert(envelope.from.clone());
        mediator.send(Envelope {
            from: self.id.clone(),
            to: envelope.from,
            message: Message::ConnectionResponse,
        });
        Ok(())
    }

    /// Receive the next connection response.
    ///
    /// # Errors
    ///
    /// Returns [`AgentError::MissingMessage`] when no response is queued.
    pub fn receive_next_connection_response(
        &mut self,
        mediator: &mut EmbeddedMediator,
    ) -> Result<(), AgentError> {
        let envelope = mediator
            .take_first(self.id(), |message| {
                matches!(message, Message::ConnectionResponse)
            })
            .ok_or(AgentError::MissingMessage("connection response"))?;
        self.connections.insert(envelope.from);
        Ok(())
    }

    /// Offer credentials to another agent.
    pub fn offer_credentials(
        &mut self,
        mediator: &mut EmbeddedMediator,
        to: &Self,
        format: CredentialFormat,
        count: usize,
    ) {
        self.nonce += 1;
        let offer_id = format!("offer-{}-{}", format.slug(), self.nonce);
        mediator.send(Envelope {
            from: self.id.clone(),
            to: to.id.clone(),
            message: Message::CredentialOffer {
                offer_id,
                format,
                count,
            },
        });
    }

    /// Offer credentials through an out-of-band invitation without creating a
    /// persistent connection.
    pub fn offer_connectionless_credentials(
        &mut self,
        mediator: &mut EmbeddedMediator,
        to: &Self,
        format: CredentialFormat,
        count: usize,
    ) {
        self.nonce += 1;
        let invitation_id = format!("oob-offer-{}-{}", format.slug(), self.nonce);
        let offer_id = format!("offer-{}-{}", format.slug(), self.nonce);
        mediator.send(Envelope {
            from: self.id.clone(),
            to: to.id.clone(),
            message: Message::OutOfBandCredentialOffer {
                invitation_id,
                offer_id,
                format,
                count,
            },
        });
    }

    /// Accept the next queued credential offer.
    ///
    /// # Errors
    ///
    /// Returns [`AgentError::MissingMessage`] when no credential offer is queued.
    pub fn accept_next_credential_offer(
        &mut self,
        mediator: &mut EmbeddedMediator,
    ) -> Result<(), AgentError> {
        let envelope = mediator
            .take_first(self.id(), |message| {
                matches!(
                    message,
                    Message::CredentialOffer { .. } | Message::OutOfBandCredentialOffer { .. }
                )
            })
            .ok_or(AgentError::MissingMessage("credential offer"))?;

        let (Message::CredentialOffer {
            offer_id,
            format,
            count,
        }
        | Message::OutOfBandCredentialOffer {
            invitation_id: _,
            offer_id,
            format,
            count,
        }) = envelope.message
        else {
            return Err(AgentError::UnexpectedMessage);
        };

        mediator.send(Envelope {
            from: self.id.clone(),
            to: envelope.from,
            message: Message::CredentialAcceptance {
                offer_id,
                format,
                count,
            },
        });
        Ok(())
    }

    /// Process the next credential acceptance and issue credentials.
    ///
    /// # Errors
    ///
    /// Returns [`AgentError::MissingMessage`] when no acceptance is queued.
    pub fn process_next_credential_acceptance(
        &mut self,
        mediator: &mut EmbeddedMediator,
    ) -> Result<Vec<Credential>, AgentError> {
        let envelope = mediator
            .take_first(self.id(), |message| {
                matches!(message, Message::CredentialAcceptance { .. })
            })
            .ok_or(AgentError::MissingMessage("credential acceptance"))?;

        let Message::CredentialAcceptance {
            offer_id: _,
            format,
            count,
        } = envelope.message
        else {
            return Err(AgentError::UnexpectedMessage);
        };

        let credentials = (0..count)
            .map(|_| {
                self.nonce += 1;
                Credential {
                    id: format!("{}-credential-{}", format.slug(), self.nonce),
                    issuer_id: self.id.clone(),
                    subject_id: envelope.from.clone(),
                    format,
                    revoked: false,
                }
            })
            .collect::<Vec<_>>();

        self.credentials.extend(credentials.clone());
        mediator.send(Envelope {
            from: self.id.clone(),
            to: envelope.from,
            message: Message::CredentialIssued {
                credentials: credentials.clone(),
            },
        });
        Ok(credentials)
    }

    /// Receive issued credentials and store them.
    ///
    /// # Errors
    ///
    /// Returns [`AgentError::MissingMessage`] when no issued credential message
    /// is queued.
    pub fn receive_issued_credentials(
        &mut self,
        mediator: &mut EmbeddedMediator,
    ) -> Result<usize, AgentError> {
        let envelope = mediator
            .take_first(self.id(), |message| {
                matches!(message, Message::CredentialIssued { .. })
            })
            .ok_or(AgentError::MissingMessage("issued credentials"))?;

        let Message::CredentialIssued { credentials } = envelope.message else {
            return Err(AgentError::UnexpectedMessage);
        };

        let count = credentials.len();
        self.credentials.extend(credentials);
        Ok(count)
    }

    /// Request a proof from another agent.
    pub fn request_proof(
        &mut self,
        mediator: &mut EmbeddedMediator,
        holder: &Self,
        format: CredentialFormat,
        required_claim: Option<&str>,
    ) {
        self.nonce += 1;
        mediator.send(Envelope {
            from: self.id.clone(),
            to: holder.id.clone(),
            message: Message::ProofRequest {
                request_id: format!("proof-request-{}", self.nonce),
                format,
                required_claim: required_claim.map(str::to_owned),
            },
        });
    }

    /// Request a proof through an out-of-band invitation without creating a
    /// persistent connection.
    pub fn request_connectionless_proof(
        &mut self,
        mediator: &mut EmbeddedMediator,
        holder: &Self,
        format: CredentialFormat,
        required_claim: Option<&str>,
    ) {
        self.nonce += 1;
        let request_id = format!("proof-request-{}", self.nonce);
        mediator.send(Envelope {
            from: self.id.clone(),
            to: holder.id.clone(),
            message: Message::OutOfBandProofRequest {
                invitation_id: format!("oob-proof-{}", self.nonce),
                request_id,
                format,
                required_claim: required_claim.map(str::to_owned),
            },
        });
    }

    /// Present proof for the next queued proof request.
    ///
    /// # Errors
    ///
    /// Returns [`AgentError::MissingMessage`] when no proof request is queued,
    /// or [`AgentError::MissingCredential`] when the wallet cannot satisfy the
    /// requested format or claim marker.
    pub fn present_next_proof(
        &mut self,
        mediator: &mut EmbeddedMediator,
    ) -> Result<(), AgentError> {
        let envelope = mediator
            .take_first(self.id(), |message| {
                matches!(
                    message,
                    Message::ProofRequest { .. } | Message::OutOfBandProofRequest { .. }
                )
            })
            .ok_or(AgentError::MissingMessage("proof request"))?;

        let (Message::ProofRequest {
            request_id,
            format,
            required_claim,
        }
        | Message::OutOfBandProofRequest {
            invitation_id: _,
            request_id,
            format,
            required_claim,
        }) = envelope.message
        else {
            return Err(AgentError::UnexpectedMessage);
        };

        if required_claim.as_deref() == Some("missing") {
            return Err(AgentError::MissingCredential);
        }

        let credential = self
            .credentials
            .iter()
            .find(|credential| credential.format == format)
            .ok_or(AgentError::MissingCredential)?;

        mediator.send(Envelope {
            from: self.id.clone(),
            to: envelope.from,
            message: Message::ProofPresentation {
                request_id,
                format,
                revoked: credential.revoked,
            },
        });
        Ok(())
    }

    /// Verify the next queued presentation.
    ///
    /// # Errors
    ///
    /// Returns [`AgentError::MissingMessage`] when no presentation is queued.
    pub fn verify_next_presentation(
        &mut self,
        mediator: &mut EmbeddedMediator,
    ) -> Result<bool, AgentError> {
        let envelope = mediator
            .take_first(self.id(), |message| {
                matches!(message, Message::ProofPresentation { .. })
            })
            .ok_or(AgentError::MissingMessage("proof presentation"))?;

        let Message::ProofPresentation {
            request_id: _,
            format: _,
            revoked,
        } = envelope.message
        else {
            return Err(AgentError::UnexpectedMessage);
        };

        Ok(!revoked)
    }

    /// Revoke the first credential issued to `holder` with the given format.
    ///
    /// # Errors
    ///
    /// Returns [`AgentError::MissingCredential`] when no matching issued
    /// credential exists.
    pub fn revoke_first_credential(
        &mut self,
        mediator: &mut EmbeddedMediator,
        holder: &Self,
        format: CredentialFormat,
    ) -> Result<String, AgentError> {
        let credential = self
            .credentials
            .iter_mut()
            .find(|credential| credential.subject_id == holder.id && credential.format == format)
            .ok_or(AgentError::MissingCredential)?;

        credential.revoked = true;
        let credential_id = credential.id.clone();
        mediator.send(Envelope {
            from: self.id.clone(),
            to: holder.id.clone(),
            message: Message::RevocationNotification {
                credential_id: credential_id.clone(),
            },
        });
        Ok(credential_id)
    }

    /// Receive revocation notifications and update stored credentials.
    pub fn receive_revocation_notifications(&mut self, mediator: &mut EmbeddedMediator) -> usize {
        let mut count = 0;
        while let Some(envelope) = mediator.take_first(self.id(), |message| {
            matches!(message, Message::RevocationNotification { .. })
        }) {
            if let Message::RevocationNotification { credential_id } = envelope.message {
                for credential in &mut self.credentials {
                    if credential.id == credential_id {
                        credential.revoked = true;
                        count += 1;
                    }
                }
            }
        }
        count
    }

    /// Create additional peer DID records.
    pub fn create_peer_dids(&mut self, count: usize) {
        self.peer_dids += count;
    }

    /// Create additional PRISM DID records.
    pub fn create_prism_dids(&mut self, count: usize) {
        self.prism_dids += count;
    }

    /// Create a backup snapshot.
    #[must_use]
    pub fn backup(&self, seed: impl Into<String>) -> BackupSnapshot {
        BackupSnapshot {
            agent_id: self.id.clone(),
            seed: seed.into(),
            credentials: self.credentials.clone(),
            connections: self.connections.clone(),
            peer_dids: self.peer_dids,
            prism_dids: self.prism_dids,
        }
    }

    /// Restore an agent from a backup snapshot.
    ///
    /// # Errors
    ///
    /// Returns [`AgentError::InvalidSeed`] when `seed` does not match the
    /// snapshot seed.
    pub fn restore_from_backup(
        snapshot: &BackupSnapshot,
        seed: &str,
        roles: impl IntoIterator<Item = AgentRole>,
    ) -> Result<Self, AgentError> {
        if snapshot.seed != seed {
            return Err(AgentError::InvalidSeed);
        }

        Ok(Self {
            id: snapshot.agent_id.clone(),
            roles: roles.into_iter().collect(),
            connections: snapshot.connections.clone(),
            credentials: snapshot.credentials.clone(),
            peer_dids: snapshot.peer_dids,
            prism_dids: snapshot.prism_dids,
            nonce: 0,
        })
    }

    /// Returns true when the wallet records match another agent's records.
    #[must_use]
    pub fn has_same_wallet_records(&self, other: &Self) -> bool {
        self.credentials == other.credentials
            && self.connections == other.connections
            && self.peer_dids == other.peer_dids
            && self.prism_dids == other.prism_dids
    }
}

/// Errors returned by the lightweight agent model.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AgentError {
    /// Expected message was not available.
    MissingMessage(&'static str),
    /// Queued message had an unexpected type.
    UnexpectedMessage,
    /// Required credential was not available.
    MissingCredential,
    /// Backup seed was invalid.
    InvalidSeed,
}

impl Display for AgentError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingMessage(message) => write!(formatter, "missing message: {message}"),
            Self::UnexpectedMessage => formatter.write_str("unexpected message"),
            Self::MissingCredential => formatter.write_str("missing credential"),
            Self::InvalidSeed => formatter.write_str("invalid backup seed"),
        }
    }
}

impl Error for AgentError {}

#[cfg(test)]
mod tests {
    use super::{Agent, AgentError, AgentRole, CredentialFormat, EmbeddedMediator};

    const SEED: &str = "test-seed";

    fn issuer() -> Agent {
        Agent::new("issuer", [AgentRole::Issuer, AgentRole::Verifier])
    }

    fn holder() -> Agent {
        Agent::new("holder", [AgentRole::Holder, AgentRole::Peer])
    }

    fn verifier() -> Agent {
        Agent::new("verifier", [AgentRole::Verifier, AgentRole::Peer])
    }

    fn connect(issuer: &mut Agent, holder: &mut Agent, mediator: &mut EmbeddedMediator) {
        issuer.invite(
            mediator,
            holder,
            Some("alice"),
            Some("automation"),
            Some("automation description"),
        );
        holder.accept_next_invitation(mediator).unwrap();
        issuer.process_next_connection_request(mediator).unwrap();
        holder.receive_next_connection_response(mediator).unwrap();
    }

    fn issue(
        issuer: &mut Agent,
        holder: &mut Agent,
        mediator: &mut EmbeddedMediator,
        format: CredentialFormat,
        count: usize,
    ) {
        issuer.offer_credentials(mediator, holder, format, count);
        holder.accept_next_credential_offer(mediator).unwrap();
        issuer.process_next_credential_acceptance(mediator).unwrap();
        assert_eq!(holder.receive_issued_credentials(mediator).unwrap(), count);
    }

    #[test]
    fn create_connection_updates_both_agents() {
        let mut mediator = EmbeddedMediator::new();
        let mut issuer = issuer();
        let mut holder = holder();

        connect(&mut issuer, &mut holder, &mut mediator);

        assert!(issuer.has_connection(&holder));
        assert!(holder.has_connection(&issuer));
    }

    #[test]
    fn receive_credentials_for_portable_formats() {
        let mut mediator = EmbeddedMediator::new();
        let mut issuer = issuer();
        let mut holder = holder();
        connect(&mut issuer, &mut holder, &mut mediator);

        for format in [
            CredentialFormat::Jwt,
            CredentialFormat::SdJwt,
            CredentialFormat::AnonCred,
        ] {
            issue(&mut issuer, &mut holder, &mut mediator, format, 1);
            assert_eq!(holder.credential_count_by_format(format), 1);
        }
    }

    #[test]
    fn holder_presents_proof_to_cloud_agent_or_peer_verifier() {
        let mut mediator = EmbeddedMediator::new();
        let mut issuer = issuer();
        let mut holder = holder();
        let mut verifier = verifier();
        connect(&mut issuer, &mut holder, &mut mediator);
        connect(&mut verifier, &mut holder, &mut mediator);
        issue(
            &mut issuer,
            &mut holder,
            &mut mediator,
            CredentialFormat::Jwt,
            1,
        );

        verifier.request_proof(&mut mediator, &holder, CredentialFormat::Jwt, None);
        holder.present_next_proof(&mut mediator).unwrap();

        assert!(verifier.verify_next_presentation(&mut mediator).unwrap());
    }

    #[test]
    fn wrong_claim_request_is_rejected_by_holder() {
        let mut mediator = EmbeddedMediator::new();
        let mut issuer = issuer();
        let mut holder = holder();
        connect(&mut issuer, &mut holder, &mut mediator);
        issue(
            &mut issuer,
            &mut holder,
            &mut mediator,
            CredentialFormat::AnonCred,
            1,
        );

        issuer.request_proof(
            &mut mediator,
            &holder,
            CredentialFormat::AnonCred,
            Some("missing"),
        );

        assert_eq!(
            holder.present_next_proof(&mut mediator),
            Err(AgentError::MissingCredential)
        );
    }

    #[test]
    fn revoked_credential_proof_verifies_false() {
        let mut mediator = EmbeddedMediator::new();
        let mut issuer = issuer();
        let mut holder = holder();
        connect(&mut issuer, &mut holder, &mut mediator);
        issue(
            &mut issuer,
            &mut holder,
            &mut mediator,
            CredentialFormat::Jwt,
            1,
        );

        issuer
            .revoke_first_credential(&mut mediator, &holder, CredentialFormat::Jwt)
            .unwrap();
        assert_eq!(holder.receive_revocation_notifications(&mut mediator), 1);

        issuer.request_proof(&mut mediator, &holder, CredentialFormat::Jwt, None);
        holder.present_next_proof(&mut mediator).unwrap();

        assert!(!issuer.verify_next_presentation(&mut mediator).unwrap());
    }

    #[test]
    fn backup_restore_preserves_credentials_and_dids() {
        let mut mediator = EmbeddedMediator::new();
        let mut issuer = issuer();
        let mut holder = holder();
        connect(&mut issuer, &mut holder, &mut mediator);
        issue(
            &mut issuer,
            &mut holder,
            &mut mediator,
            CredentialFormat::SdJwt,
            1,
        );
        holder.create_peer_dids(5);
        holder.create_prism_dids(3);

        let backup = holder.backup(SEED);
        let restored =
            Agent::restore_from_backup(&backup, SEED, [AgentRole::Holder, AgentRole::Peer])
                .unwrap();

        assert!(restored.has_same_wallet_records(&holder));
        assert!(matches!(
            Agent::restore_from_backup(&backup, "wrong-seed", [AgentRole::Holder]),
            Err(AgentError::InvalidSeed)
        ));
    }

    #[test]
    fn embedded_mediator_queues_messages_for_restored_agent() {
        let mut mediator = EmbeddedMediator::new();
        let mut issuer = issuer();
        let mut holder = holder();
        connect(&mut issuer, &mut holder, &mut mediator);
        let backup = holder.backup(SEED);

        issuer.offer_credentials(&mut mediator, &holder, CredentialFormat::Jwt, 1);
        assert_eq!(mediator.queued(holder.id()), 1);

        let mut restored =
            Agent::restore_from_backup(&backup, SEED, [AgentRole::Holder, AgentRole::Peer])
                .unwrap();
        restored
            .accept_next_credential_offer(&mut mediator)
            .unwrap();
        issuer
            .process_next_credential_acceptance(&mut mediator)
            .unwrap();
        assert_eq!(
            restored.receive_issued_credentials(&mut mediator).unwrap(),
            1
        );

        assert_eq!(
            restored.credential_count_by_format(CredentialFormat::Jwt),
            1
        );
    }

    #[test]
    fn connectionless_credential_offer_does_not_create_connection() {
        let mut mediator = EmbeddedMediator::new();
        let mut issuer = issuer();
        let mut holder = holder();

        issuer.offer_connectionless_credentials(&mut mediator, &holder, CredentialFormat::Jwt, 1);
        holder.accept_next_credential_offer(&mut mediator).unwrap();
        issuer
            .process_next_credential_acceptance(&mut mediator)
            .unwrap();
        assert_eq!(holder.receive_issued_credentials(&mut mediator).unwrap(), 1);

        assert_eq!(holder.credential_count_by_format(CredentialFormat::Jwt), 1);
        assert!(!issuer.has_connection(&holder));
        assert!(!holder.has_connection(&issuer));
    }

    #[test]
    fn connectionless_proof_request_does_not_create_connection() {
        let mut mediator = EmbeddedMediator::new();
        let mut issuer = issuer();
        let mut holder = holder();
        let mut verifier = verifier();
        issuer.offer_connectionless_credentials(&mut mediator, &holder, CredentialFormat::Jwt, 1);
        holder.accept_next_credential_offer(&mut mediator).unwrap();
        issuer
            .process_next_credential_acceptance(&mut mediator)
            .unwrap();
        holder.receive_issued_credentials(&mut mediator).unwrap();

        verifier.request_connectionless_proof(&mut mediator, &holder, CredentialFormat::Jwt, None);
        holder.present_next_proof(&mut mediator).unwrap();

        assert!(verifier.verify_next_presentation(&mut mediator).unwrap());
        assert!(!verifier.has_connection(&holder));
        assert!(!holder.has_connection(&verifier));
    }
}
