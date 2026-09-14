use super::{ErrorContract, invalid_input};
use crate::error::error_code;

pub(crate) const INVALID_FORMAT: ErrorContract = invalid_input(
    error_code::INVALID_FORMAT,
    "credential format is invalid",
    "invalid credential format",
);
pub(crate) const EMPTY_PAYLOAD: ErrorContract = invalid_input(
    error_code::EMPTY_PAYLOAD,
    "credential payload is empty",
    "credential payload is empty",
);
pub(crate) const PAYLOAD_TOO_LARGE: ErrorContract = invalid_input(
    error_code::PAYLOAD_TOO_LARGE,
    "credential payload exceeds the size limit",
    "credential payload exceeds the size limit",
);
pub(crate) const EMPTY_DETACHED_PROOF: ErrorContract = invalid_input(
    error_code::EMPTY_DETACHED_PROOF,
    "credential detached proof is empty",
    "credential detached proof is empty",
);
pub(crate) const DETACHED_PROOF_TOO_LARGE: ErrorContract = invalid_input(
    error_code::DETACHED_PROOF_TOO_LARGE,
    "credential detached proof exceeds the size limit",
    "credential detached proof exceeds the size limit",
);
pub(crate) const EMPTY_PRIVATE_MATERIAL: ErrorContract = invalid_input(
    error_code::EMPTY_PRIVATE_MATERIAL,
    "credential private material is empty",
    "credential private material is empty",
);
pub(crate) const PRIVATE_MATERIAL_TOO_LARGE: ErrorContract = invalid_input(
    error_code::PRIVATE_MATERIAL_TOO_LARGE,
    "credential private material exceeds the size limit",
    "credential private material exceeds the size limit",
);
