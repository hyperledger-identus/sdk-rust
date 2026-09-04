use std::{hint::black_box, str::FromStr, time::Instant};

use identus_core::ErrorKind;
use identus_presentations::{
    PresentationError, PresentationLifecyclePhase as Phase, PresentationProtocolState as State,
    PresentationTerminalOutcome as Outcome,
};

const PHASES: [Phase; 6] = [
    Phase::Requested,
    Phase::AwaitingAuthorization,
    Phase::Generating,
    Phase::Ready,
    Phase::Delivering,
    Phase::CancellationRequested,
];

const OUTCOMES: [Outcome; 5] = [
    Outcome::Completed,
    Outcome::Refused,
    Outcome::Cancelled,
    Outcome::Expired,
    Outcome::Failed,
];

const STATES: [State; 11] = [
    State::Active(Phase::Requested),
    State::Active(Phase::AwaitingAuthorization),
    State::Active(Phase::Generating),
    State::Active(Phase::Ready),
    State::Active(Phase::Delivering),
    State::Active(Phase::CancellationRequested),
    State::Terminal(Outcome::Completed),
    State::Terminal(Outcome::Refused),
    State::Terminal(Outcome::Cancelled),
    State::Terminal(Outcome::Expired),
    State::Terminal(Outcome::Failed),
];

const ALLOWED: [(State, State); 28] = [
    (STATES[0], STATES[1]),
    (STATES[0], STATES[7]),
    (STATES[0], STATES[8]),
    (STATES[0], STATES[9]),
    (STATES[0], STATES[10]),
    (STATES[1], STATES[2]),
    (STATES[1], STATES[7]),
    (STATES[1], STATES[8]),
    (STATES[1], STATES[9]),
    (STATES[1], STATES[10]),
    (STATES[2], STATES[3]),
    (STATES[2], STATES[5]),
    (STATES[2], STATES[8]),
    (STATES[2], STATES[9]),
    (STATES[2], STATES[10]),
    (STATES[3], STATES[4]),
    (STATES[3], STATES[8]),
    (STATES[3], STATES[9]),
    (STATES[3], STATES[10]),
    (STATES[4], STATES[5]),
    (STATES[4], STATES[6]),
    (STATES[4], STATES[8]),
    (STATES[4], STATES[9]),
    (STATES[4], STATES[10]),
    (STATES[5], STATES[6]),
    (STATES[5], STATES[8]),
    (STATES[5], STATES[9]),
    (STATES[5], STATES[10]),
];

#[test]
fn lifecycle_vocabulary_round_trips_exactly() {
    let phase_spellings = [
        "requested",
        "awaiting_authorization",
        "generating",
        "ready",
        "delivering",
        "cancellation_requested",
    ];
    for (phase, spelling) in PHASES.into_iter().zip(phase_spellings) {
        assert_eq!(phase.as_str(), spelling);
        assert_eq!(phase.to_string(), spelling);
        assert_eq!(Phase::from_str(spelling), Ok(phase));
        assert_eq!(State::from_str(spelling), Ok(State::active(phase)));
    }

    let outcome_spellings = ["completed", "refused", "cancelled", "expired", "failed"];
    for (outcome, spelling) in OUTCOMES.into_iter().zip(outcome_spellings) {
        assert_eq!(outcome.as_str(), spelling);
        assert_eq!(outcome.to_string(), spelling);
        assert_eq!(Outcome::from_str(spelling), Ok(outcome));
        assert_eq!(State::from_str(spelling), Ok(State::terminal(outcome)));
    }
}

#[test]
fn lifecycle_parsing_is_strict_and_redacted() {
    for invalid in [
        "",
        " requested",
        "requested ",
        "Requested",
        "awaiting-consent",
        "succeeded",
        "state-canary",
    ] {
        let phase_error = Phase::from_str(invalid).expect_err("invalid phase");
        let outcome_error = Outcome::from_str(invalid).expect_err("invalid outcome");
        let state_error = State::from_str(invalid).expect_err("invalid state");
        assert_eq!(phase_error, PresentationError::InvalidLifecyclePhase);
        assert_eq!(outcome_error, PresentationError::InvalidTerminalOutcome);
        assert_eq!(state_error, PresentationError::InvalidProtocolState);
        for error in [phase_error, outcome_error, state_error] {
            if !invalid.is_empty() {
                assert!(!error.to_string().contains(invalid));
                assert!(!format!("{error:?}").contains(invalid));
            }
        }
    }
}

#[test]
fn transition_matrix_is_exhaustive_and_exact() {
    for from in STATES {
        for to in STATES {
            let expected = ALLOWED.contains(&(from, to));
            assert_eq!(
                from.can_transition_to(to),
                expected,
                "unexpected transition decision: {from} -> {to}"
            );
            assert_eq!(
                from.transition_to(to),
                expected
                    .then_some(to)
                    .ok_or(PresentationError::InvalidProtocolTransition),
                "unexpected transition result: {from} -> {to}"
            );
        }
    }
}

#[test]
fn terminal_states_are_immutable_and_data_free() {
    for outcome in OUTCOMES {
        let state = State::terminal(outcome);
        assert!(state.is_terminal());
        assert_eq!(state.phase(), None);
        assert_eq!(state.outcome(), Some(outcome));
        for next in STATES {
            assert_eq!(
                state.transition_to(next),
                Err(PresentationError::InvalidProtocolTransition)
            );
        }
    }

    for phase in PHASES {
        let state = State::active(phase);
        assert!(!state.is_terminal());
        assert_eq!(state.phase(), Some(phase));
        assert_eq!(state.outcome(), None);
    }
}

#[test]
fn cancellation_request_reports_the_observed_race_truthfully() {
    let cancelling = State::active(Phase::Delivering)
        .transition_to(State::active(Phase::CancellationRequested))
        .expect("delivery can request cancellation");
    assert_eq!(
        cancelling.transition_to(State::terminal(Outcome::Cancelled)),
        Ok(State::terminal(Outcome::Cancelled))
    );
    assert_eq!(
        cancelling.transition_to(State::terminal(Outcome::Completed)),
        Ok(State::terminal(Outcome::Completed))
    );

    assert!(
        !State::active(Phase::Generating).can_transition_to(State::terminal(Outcome::Completed))
    );
    assert!(!State::active(Phase::Ready).can_transition_to(State::terminal(Outcome::Completed)));
}

#[test]
fn lifecycle_errors_use_static_presentation_contracts() {
    let cases = [
        (
            PresentationError::InvalidLifecyclePhase,
            "presentation.invalid_lifecycle_phase",
        ),
        (
            PresentationError::InvalidTerminalOutcome,
            "presentation.invalid_terminal_outcome",
        ),
        (
            PresentationError::InvalidProtocolState,
            "presentation.invalid_protocol_state",
        ),
        (
            PresentationError::InvalidProtocolTransition,
            "presentation.invalid_protocol_transition",
        ),
    ];

    for (error, code) in cases {
        let shared = error.to_identus_error();
        assert_eq!(shared.capability().unwrap().as_str(), "presentation");
        assert_eq!(shared.kind(), ErrorKind::InvalidInput);
        assert_eq!(shared.code().as_str(), code);
        assert!(!shared.public_message().is_empty());
    }
}

#[test]
#[ignore = "manual release-mode performance diagnostic"]
fn presentation_lifecycle_transition_throughput_diagnostic() {
    const ITERATIONS: usize = 2_000_000;
    let started = Instant::now();
    let mut accepted = 0_usize;
    for index in 0..ITERATIONS {
        let from = STATES[index % STATES.len()];
        let to = STATES[(index / STATES.len()) % STATES.len()];
        accepted += usize::from(black_box(from.can_transition_to(black_box(to))));
    }
    let elapsed = started.elapsed();
    let per_second = ITERATIONS as f64 / elapsed.as_secs_f64();
    eprintln!(
        "validated {ITERATIONS} presentation lifecycle transitions in {elapsed:?} ({per_second:.0}/s, {accepted} accepted)"
    );
    assert!(accepted > 0);
}
