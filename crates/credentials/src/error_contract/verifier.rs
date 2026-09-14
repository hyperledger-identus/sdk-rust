use identus_core::{ErrorCode, ErrorKind};

use super::ErrorContract;
use crate::error::CAPABILITY;

pub(crate) const VERIFICATION_UNSUPPORTED_FORMAT: ErrorContract = ErrorContract::new(
    ErrorCode::new("credential.verification_unsupported_format"),
    ErrorKind::Unsupported,
    CAPABILITY,
    "credential verification format is unsupported",
    "credential verification format is unsupported",
);
pub(crate) const VERIFICATION_UNAVAILABLE: ErrorContract = ErrorContract::new(
    ErrorCode::new("credential.verification_unavailable"),
    ErrorKind::Internal,
    CAPABILITY,
    "credential verification is unavailable",
    "credential verification is unavailable",
);
pub(crate) const VERIFICATION_INTERNAL: ErrorContract = ErrorContract::new(
    ErrorCode::new("credential.verification_internal"),
    ErrorKind::Internal,
    CAPABILITY,
    "credential verification failed internally",
    "credential verification failed internally",
);
