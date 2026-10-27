use crate::{DirectPostResponse, SiopResponsePlan, Siopv2Error};
use identus_did::{Did, DidUrl};
use identus_jose::{JwsAlgorithm, JwsSigner};
use zeroize::Zeroizing;

/// Caller-supplied bounded idempotency identity for one transition.
#[derive(Clone, PartialEq, Eq)]
pub struct EventId(Zeroizing<String>);

impl EventId {
    /// Validate a non-empty bounded opaque event identity.
    pub fn new(value: &str, maximum: usize) -> Result<Self, Siopv2Error> {
        if value.is_empty() || value.len() > maximum || value.chars().any(char::is_control) {
            return Err(Siopv2Error::InvalidEventId);
        }
        Ok(Self(Zeroizing::new(value.to_owned())))
    }
}

impl std::fmt::Debug for EventId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("EventId(..)")
    }
}

/// Pure application-neutral SIOPv2 lifecycle.
#[derive(Debug)]
#[non_exhaustive]
pub enum SiopFlow {
    AwaitingDecision {
        request: crate::ValidatedSiopRequest,
    },
    ResponseReady {
        response: DirectPostResponse,
        event_id: EventId,
    },
    Refused {
        event_id: EventId,
    },
    Canceled {
        event_id: EventId,
    },
    TimedOut {
        event_id: EventId,
    },
    Delivered {
        event_id: EventId,
    },
    OutcomeUnknown {
        event_id: EventId,
    },
}

impl SiopFlow {
    /// Start a flow awaiting an explicit caller decision.
    pub fn new(request: crate::ValidatedSiopRequest) -> Self {
        Self::AwaitingDecision { request }
    }

    #[allow(clippy::too_many_arguments)]
    /// Approve and sign once; failures leave the previous flow state intact.
    pub fn approve(
        &mut self,
        event_id: EventId,
        subject: Did,
        key_id: DidUrl,
        algorithm: JwsAlgorithm,
        issued_at: i64,
        expires_at: i64,
        signer: &dyn JwsSigner,
    ) -> Result<(), Siopv2Error> {
        let request = match self {
            Self::AwaitingDecision { request } => request,
            Self::ResponseReady {
                event_id: existing, ..
            } if *existing == event_id => return Ok(()),
            Self::ResponseReady { .. } => return Err(Siopv2Error::EventConflict),
            _ => return Err(Siopv2Error::InvalidTransition),
        };
        let plan: SiopResponsePlan =
            request.plan_response(subject, key_id, algorithm, issued_at, expires_at)?;
        let response = plan.sign(signer)?;
        *self = Self::ResponseReady { response, event_id };
        Ok(())
    }

    /// Record refusal before approval.
    pub fn refuse(&mut self, event_id: EventId) -> Result<(), Siopv2Error> {
        match self {
            Self::AwaitingDecision { .. } => {
                *self = Self::Refused { event_id };
                Ok(())
            }
            Self::Refused { event_id: existing } if *existing == event_id => Ok(()),
            Self::Refused { .. } => Err(Siopv2Error::EventConflict),
            _ => Err(Siopv2Error::InvalidTransition),
        }
    }
    /// Record caller cancellation before approval.
    pub fn cancel(&mut self, event_id: EventId) -> Result<(), Siopv2Error> {
        match self {
            Self::AwaitingDecision { .. } => {
                *self = Self::Canceled { event_id };
                Ok(())
            }
            Self::Canceled { event_id: existing } if *existing == event_id => Ok(()),
            Self::Canceled { .. } => Err(Siopv2Error::EventConflict),
            _ => Err(Siopv2Error::InvalidTransition),
        }
    }
    /// Record expiry before approval, or unknown delivery after response creation.
    pub fn timeout(&mut self, event_id: EventId) -> Result<(), Siopv2Error> {
        match self {
            Self::AwaitingDecision { .. } => {
                *self = Self::TimedOut { event_id };
                Ok(())
            }
            Self::ResponseReady {
                event_id: approval, ..
            } if *approval == event_id => Err(Siopv2Error::EventConflict),
            Self::ResponseReady { .. } => {
                *self = Self::OutcomeUnknown { event_id };
                Ok(())
            }
            Self::TimedOut { event_id: existing } if *existing == event_id => Ok(()),
            Self::TimedOut { .. } => Err(Siopv2Error::EventConflict),
            Self::OutcomeUnknown { event_id: existing } if *existing == event_id => Ok(()),
            Self::OutcomeUnknown { .. } => Err(Siopv2Error::EventConflict),
            _ => Err(Siopv2Error::InvalidTransition),
        }
    }
    /// Record externally confirmed response acceptance.
    pub fn delivered(&mut self, event_id: EventId) -> Result<(), Siopv2Error> {
        match self {
            Self::ResponseReady {
                event_id: approval, ..
            } if *approval == event_id => Err(Siopv2Error::EventConflict),
            Self::ResponseReady { .. } => {
                *self = Self::Delivered { event_id };
                Ok(())
            }
            Self::Delivered { event_id: existing } if *existing == event_id => Ok(()),
            Self::Delivered { .. } => Err(Siopv2Error::EventConflict),
            _ => Err(Siopv2Error::InvalidTransition),
        }
    }
    /// Record an uncertain external delivery outcome without authorizing retry.
    pub fn delivery_unknown(&mut self, event_id: EventId) -> Result<(), Siopv2Error> {
        match self {
            Self::ResponseReady {
                event_id: approval, ..
            } if *approval == event_id => Err(Siopv2Error::EventConflict),
            Self::ResponseReady { .. } => {
                *self = Self::OutcomeUnknown { event_id };
                Ok(())
            }
            Self::OutcomeUnknown { event_id: existing } if *existing == event_id => Ok(()),
            Self::OutcomeUnknown { .. } => Err(Siopv2Error::EventConflict),
            _ => Err(Siopv2Error::InvalidTransition),
        }
    }
}
