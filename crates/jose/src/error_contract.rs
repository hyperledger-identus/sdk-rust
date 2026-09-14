use identus_core::{ErrorCode, ErrorKind, IdentusError};

pub(crate) mod algorithm_key_registry_signing;
pub(crate) mod compact_header;
pub(crate) mod proof_key_evidence;
pub(crate) mod proof_policy_time_replay;

#[derive(Clone, Copy)]
pub(crate) struct ErrorContract {
    code: ErrorCode,
    kind: ErrorKind,
    message: &'static str,
}

impl ErrorContract {
    pub(crate) const fn new(code: ErrorCode, kind: ErrorKind, message: &'static str) -> Self {
        Self {
            code,
            kind,
            message,
        }
    }

    pub(crate) const fn message(self) -> &'static str {
        self.message
    }

    pub(crate) const fn to_identus_error(self) -> IdentusError {
        IdentusError::public(self.code, self.kind, crate::error::CAPABILITY, self.message)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::JoseError;

    const GOLDEN: &str = include_str!("../tests/fixtures/jose-error-contract-v1.csv");

    fn fixture_variants() -> Vec<String> {
        let mut lines = GOLDEN.lines().filter(|line| !line.starts_with('#'));
        let header = lines.next().expect("golden header");
        assert_eq!(header.split(',').count(), 11);

        lines
            .map(|line| {
                let columns: Vec<_> = line.split(',').collect();
                assert_eq!(columns.len(), 11);
                assert_eq!(columns[0], "JoseError");
                columns[1].to_owned()
            })
            .collect()
    }

    #[test]
    fn compile_exhaustive_variant_inventory_equals_ordered_fixture_keys() {
        let inventory: Vec<_> = JoseError::CONTRACT_VARIANTS
            .iter()
            .map(|error| format!("{error:?}"))
            .collect();
        let unique: BTreeSet<_> = inventory.iter().collect();

        assert_eq!(JoseError::CONTRACT_VARIANTS.len(), 51);
        assert_eq!(unique.len(), 51);
        assert_eq!(inventory, fixture_variants());
    }
}
