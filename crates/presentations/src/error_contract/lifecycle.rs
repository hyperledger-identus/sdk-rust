use super::ErrorContract;
use crate::error::error_code;

pub(crate) const INVALID_LIFECYCLE_PHASE: ErrorContract = ErrorContract::new(
    error_code::INVALID_LIFECYCLE_PHASE,
    "presentation lifecycle phase is invalid",
);
pub(crate) const INVALID_TERMINAL_OUTCOME: ErrorContract = ErrorContract::new(
    error_code::INVALID_TERMINAL_OUTCOME,
    "presentation terminal outcome is invalid",
);
pub(crate) const INVALID_PROTOCOL_STATE: ErrorContract = ErrorContract::new(
    error_code::INVALID_PROTOCOL_STATE,
    "presentation protocol state is invalid",
);
pub(crate) const INVALID_PROTOCOL_TRANSITION: ErrorContract = ErrorContract::new(
    error_code::INVALID_PROTOCOL_TRANSITION,
    "presentation protocol transition is invalid",
);
