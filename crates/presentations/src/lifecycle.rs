//! Allocation-free, format-neutral presentation protocol lifecycle values.

use std::{fmt, str::FromStr};

use crate::PresentationError;

/// A non-terminal phase in a presentation protocol coordinator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum PresentationLifecyclePhase {
    /// A request is being parsed, matched, or prepared by outer layers.
    Requested,
    /// An external authorization prerequisite has not yet been satisfied.
    AwaitingAuthorization,
    /// A format adapter is generating presentation artifacts.
    Generating,
    /// Generated artifacts are ready for protocol-specific handoff.
    Ready,
    /// A protocol adapter is handing artifacts to its destination.
    Delivering,
    /// Cancellation was requested while work may already be irreversible.
    CancellationRequested,
}

impl PresentationLifecyclePhase {
    /// Return the stable lowercase spelling used by explicit adapter mappings.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Requested => "requested",
            Self::AwaitingAuthorization => "awaiting_authorization",
            Self::Generating => "generating",
            Self::Ready => "ready",
            Self::Delivering => "delivering",
            Self::CancellationRequested => "cancellation_requested",
        }
    }
}

impl FromStr for PresentationLifecyclePhase {
    type Err = PresentationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "requested" => Ok(Self::Requested),
            "awaiting_authorization" => Ok(Self::AwaitingAuthorization),
            "generating" => Ok(Self::Generating),
            "ready" => Ok(Self::Ready),
            "delivering" => Ok(Self::Delivering),
            "cancellation_requested" => Ok(Self::CancellationRequested),
            _ => Err(PresentationError::InvalidLifecyclePhase),
        }
    }
}

impl fmt::Display for PresentationLifecyclePhase {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A terminal outcome reported by a presentation protocol coordinator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum PresentationTerminalOutcome {
    /// The selected protocol adapter reports terminal completion.
    ///
    /// This does not assert proof validity, verifier acceptance, credential
    /// trust, acknowledgement, or persistence.
    Completed,
    /// An external decision refused the presentation before generation.
    Refused,
    /// Cancellation completed without an observed terminal delivery.
    Cancelled,
    /// The coordinator's externally owned validity window ended.
    Expired,
    /// The coordinator stopped because an externally recorded error occurred.
    Failed,
}

impl PresentationTerminalOutcome {
    /// Return the stable lowercase spelling used by explicit adapter mappings.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Completed => "completed",
            Self::Refused => "refused",
            Self::Cancelled => "cancelled",
            Self::Expired => "expired",
            Self::Failed => "failed",
        }
    }
}

impl FromStr for PresentationTerminalOutcome {
    type Err = PresentationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "completed" => Ok(Self::Completed),
            "refused" => Ok(Self::Refused),
            "cancelled" => Ok(Self::Cancelled),
            "expired" => Ok(Self::Expired),
            "failed" => Ok(Self::Failed),
            _ => Err(PresentationError::InvalidTerminalOutcome),
        }
    }
}

impl fmt::Display for PresentationTerminalOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Active phase or terminal outcome of a format-neutral presentation flow.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum PresentationProtocolState {
    /// The coordinator remains active in the contained phase.
    Active(PresentationLifecyclePhase),
    /// The coordinator reached the contained immutable terminal outcome.
    Terminal(PresentationTerminalOutcome),
}

impl PresentationProtocolState {
    /// Construct an active protocol state.
    #[must_use]
    pub const fn active(phase: PresentationLifecyclePhase) -> Self {
        Self::Active(phase)
    }

    /// Construct a terminal protocol state.
    #[must_use]
    pub const fn terminal(outcome: PresentationTerminalOutcome) -> Self {
        Self::Terminal(outcome)
    }

    /// Return the active phase, if this state is non-terminal.
    #[must_use]
    pub const fn phase(self) -> Option<PresentationLifecyclePhase> {
        match self {
            Self::Active(phase) => Some(phase),
            Self::Terminal(_) => None,
        }
    }

    /// Return the terminal outcome, if this state is terminal.
    #[must_use]
    pub const fn outcome(self) -> Option<PresentationTerminalOutcome> {
        match self {
            Self::Active(_) => None,
            Self::Terminal(outcome) => Some(outcome),
        }
    }

    /// Report whether the state is terminal.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Terminal(_))
    }

    /// Return the stable lowercase spelling used by explicit adapter mappings.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active(phase) => phase.as_str(),
            Self::Terminal(outcome) => outcome.as_str(),
        }
    }

    /// Report whether the conservative lifecycle permits the directed edge.
    #[must_use]
    pub const fn can_transition_to(self, next: Self) -> bool {
        use PresentationLifecyclePhase as Phase;
        use PresentationProtocolState::{Active, Terminal};
        use PresentationTerminalOutcome as Outcome;

        matches!(
            (self, next),
            (
                Active(Phase::Requested),
                Active(Phase::AwaitingAuthorization)
                    | Terminal(
                        Outcome::Refused | Outcome::Cancelled | Outcome::Expired | Outcome::Failed
                    )
            ) | (
                Active(Phase::AwaitingAuthorization),
                Active(Phase::Generating)
                    | Terminal(
                        Outcome::Refused | Outcome::Cancelled | Outcome::Expired | Outcome::Failed
                    )
            ) | (
                Active(Phase::Generating),
                Active(Phase::Ready | Phase::CancellationRequested)
                    | Terminal(Outcome::Cancelled | Outcome::Expired | Outcome::Failed)
            ) | (
                Active(Phase::Ready),
                Active(Phase::Delivering)
                    | Terminal(Outcome::Cancelled | Outcome::Expired | Outcome::Failed)
            ) | (
                Active(Phase::Delivering),
                Active(Phase::CancellationRequested)
                    | Terminal(
                        Outcome::Completed
                            | Outcome::Cancelled
                            | Outcome::Expired
                            | Outcome::Failed
                    )
            ) | (
                Active(Phase::CancellationRequested),
                Terminal(
                    Outcome::Completed | Outcome::Cancelled | Outcome::Expired | Outcome::Failed
                )
            )
        )
    }

    /// Validate and return the next state without mutating the current value.
    pub const fn transition_to(self, next: Self) -> Result<Self, PresentationError> {
        if self.can_transition_to(next) {
            Ok(next)
        } else {
            Err(PresentationError::InvalidProtocolTransition)
        }
    }
}

impl From<PresentationLifecyclePhase> for PresentationProtocolState {
    fn from(phase: PresentationLifecyclePhase) -> Self {
        Self::active(phase)
    }
}

impl From<PresentationTerminalOutcome> for PresentationProtocolState {
    fn from(outcome: PresentationTerminalOutcome) -> Self {
        Self::terminal(outcome)
    }
}

impl FromStr for PresentationProtocolState {
    type Err = PresentationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if let Ok(phase) = PresentationLifecyclePhase::from_str(value) {
            return Ok(Self::active(phase));
        }
        if let Ok(outcome) = PresentationTerminalOutcome::from_str(value) {
            return Ok(Self::terminal(outcome));
        }
        Err(PresentationError::InvalidProtocolState)
    }
}

impl fmt::Display for PresentationProtocolState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
