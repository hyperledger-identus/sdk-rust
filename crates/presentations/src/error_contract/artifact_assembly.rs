use super::ErrorContract;
use crate::error::error_code;

pub(crate) const INVALID_ARTIFACT_BINDINGS: ErrorContract = ErrorContract::new(
    error_code::INVALID_ARTIFACT_BINDINGS,
    "presentation artifact binding collection is invalid",
);
pub(crate) const DUPLICATE_ARTIFACT_BINDING: ErrorContract = ErrorContract::new(
    error_code::DUPLICATE_ARTIFACT_BINDING,
    "presentation artifact repeats a selection binding",
);
pub(crate) const INVALID_ARTIFACT_PAYLOAD: ErrorContract = ErrorContract::new(
    error_code::INVALID_ARTIFACT_PAYLOAD,
    "presentation artifact payload is invalid",
);
pub(crate) const INVALID_GENERATED_ARTIFACTS: ErrorContract = ErrorContract::new(
    error_code::INVALID_GENERATED_ARTIFACTS,
    "generated presentation artifact collection is invalid",
);
pub(crate) const ARTIFACT_PAYLOAD_BUDGET_EXCEEDED: ErrorContract = ErrorContract::new(
    error_code::ARTIFACT_PAYLOAD_BUDGET_EXCEEDED,
    "generated presentation artifact bytes exceed the budget",
);
pub(crate) const UNKNOWN_ARTIFACT_SELECTION: ErrorContract = ErrorContract::new(
    error_code::UNKNOWN_ARTIFACT_SELECTION,
    "presentation artifact references an unknown selection",
);
pub(crate) const ARTIFACT_FORMAT_MISMATCH: ErrorContract = ErrorContract::new(
    error_code::ARTIFACT_FORMAT_MISMATCH,
    "presentation artifact format does not match its selection",
);
pub(crate) const DUPLICATE_GENERATED_ARTIFACT_BINDING: ErrorContract = ErrorContract::new(
    error_code::DUPLICATE_GENERATED_ARTIFACT_BINDING,
    "generated presentation repeats a selection binding",
);
pub(crate) const MISSING_ARTIFACT_SELECTION: ErrorContract = ErrorContract::new(
    error_code::MISSING_ARTIFACT_SELECTION,
    "generated presentation omits a selection binding",
);
