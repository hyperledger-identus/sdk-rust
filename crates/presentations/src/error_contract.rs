use identus_core::{ErrorCode, ErrorKind, IdentusError};

pub(crate) mod artifact_assembly;
pub(crate) mod candidate_matching;
pub(crate) mod disclosure_selection;
pub(crate) mod lifecycle;
pub(crate) mod request_query;

#[derive(Clone, Copy)]
pub(crate) struct ErrorContract {
    code: ErrorCode,
    message: &'static str,
}

impl ErrorContract {
    pub(crate) const fn new(code: ErrorCode, message: &'static str) -> Self {
        Self { code, message }
    }

    pub(crate) const fn message(self) -> &'static str {
        self.message
    }

    pub(crate) const fn to_identus_error(self) -> IdentusError {
        IdentusError::public(
            self.code,
            ErrorKind::InvalidInput,
            crate::error::CAPABILITY,
            self.message,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::PresentationError;

    const GOLDEN: &str = include_str!("../tests/fixtures/presentations-error-contract-v1.csv");

    fn fixture_variants() -> Vec<String> {
        let mut lines = GOLDEN.lines().filter(|line| !line.starts_with('#'));
        let header = lines.next().expect("golden header");
        assert_eq!(header.split(',').count(), 11);

        let mut variants = Vec::new();
        for line in lines {
            let columns: Vec<_> = line.split(',').collect();
            assert_eq!(columns.len(), 11);
            assert_eq!(columns[0], "PresentationError");
            variants.push(columns[1].to_owned());
        }
        variants
    }

    #[test]
    fn compile_exhaustive_variant_inventory_equals_fixture_keys() {
        let variants: Vec<_> = PresentationError::CONTRACT_VARIANTS
            .iter()
            .map(|error| format!("{error:?}"))
            .collect();

        assert_eq!(PresentationError::CONTRACT_VARIANTS.len(), 48);
        assert_eq!(variants, fixture_variants());
    }
}
