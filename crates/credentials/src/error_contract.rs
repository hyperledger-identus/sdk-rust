use identus_core::{CapabilityId, ErrorCode, ErrorKind, IdentusError};

pub(crate) mod envelope;
pub(crate) mod metadata;
pub(crate) mod status;
pub(crate) mod verification;
pub(crate) mod verifier;

#[derive(Clone, Copy)]
pub(crate) struct ErrorContract {
    code: ErrorCode,
    kind: ErrorKind,
    capability: CapabilityId,
    local_display: &'static str,
    public_message: &'static str,
}

impl ErrorContract {
    pub(crate) const fn new(
        code: ErrorCode,
        kind: ErrorKind,
        capability: CapabilityId,
        local_display: &'static str,
        public_message: &'static str,
    ) -> Self {
        Self {
            code,
            kind,
            capability,
            local_display,
            public_message,
        }
    }

    pub(crate) const fn code(self) -> ErrorCode {
        self.code
    }

    pub(crate) const fn kind(self) -> ErrorKind {
        self.kind
    }

    pub(crate) const fn capability(self) -> CapabilityId {
        self.capability
    }

    pub(crate) const fn local_display(self) -> &'static str {
        self.local_display
    }

    pub(crate) const fn public_message(self) -> &'static str {
        self.public_message
    }

    pub(crate) const fn to_identus_error(self) -> IdentusError {
        IdentusError::public(
            self.code(),
            self.kind(),
            self.capability(),
            self.public_message(),
        )
    }
}

const fn invalid_input(
    code: ErrorCode,
    local_display: &'static str,
    public_message: &'static str,
) -> ErrorContract {
    ErrorContract::new(
        code,
        ErrorKind::InvalidInput,
        crate::error::CAPABILITY,
        local_display,
        public_message,
    )
}
