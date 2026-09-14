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

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::{CredentialError, CredentialVerificationError};

    const GOLDEN: &str = include_str!("../tests/fixtures/credentials-error-contract-v1.csv");

    fn fixture_variants(error_type: &str) -> BTreeSet<String> {
        let mut lines = GOLDEN.lines().filter(|line| !line.starts_with('#'));
        let header = lines.next().expect("golden header");
        assert_eq!(header.split(',').count(), 11);

        let mut variants = BTreeSet::new();
        for line in lines {
            let columns: Vec<_> = line.split(',').collect();
            assert_eq!(columns.len(), 11);
            if columns[0] == error_type {
                assert!(variants.insert(columns[1].to_owned()));
            }
        }
        variants
    }

    #[test]
    fn compile_exhaustive_variant_inventories_equal_fixture_keys() {
        let credentials: BTreeSet<_> = CredentialError::CONTRACT_VARIANTS
            .iter()
            .map(|error| format!("{error:?}"))
            .collect();
        let verification: BTreeSet<_> = CredentialVerificationError::CONTRACT_VARIANTS
            .iter()
            .map(|error| format!("{error:?}"))
            .collect();

        assert_eq!(CredentialError::CONTRACT_VARIANTS.len(), 44);
        assert_eq!(CredentialVerificationError::CONTRACT_VARIANTS.len(), 3);
        assert_eq!(credentials, fixture_variants("CredentialError"));
        assert_eq!(
            verification,
            fixture_variants("CredentialVerificationError")
        );
    }
}
