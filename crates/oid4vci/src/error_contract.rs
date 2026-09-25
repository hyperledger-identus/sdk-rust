use identus_core::{ErrorCode, ErrorKind, IdentusError};

pub(crate) mod authorization_code_server;
pub(crate) mod authorization_response;
pub(crate) mod credential_nonce_http;
pub(crate) mod deferred_immediate_issuance;
pub(crate) mod issuer_authorization_server_metadata;
pub(crate) mod offer_semantics_grants;
pub(crate) mod offer_transport_json;
pub(crate) mod token_request_response_errors;

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

    use crate::CredentialOfferError;

    const GOLDEN: &str = include_str!("../tests/fixtures/oid4vci-error-contract-v1.csv");

    fn fixture_variants() -> Vec<String> {
        let mut lines = GOLDEN.lines().filter(|line| !line.starts_with('#'));
        let header = lines.next().expect("golden header");
        assert_eq!(header.split(',').count(), 11);

        lines
            .map(|line| {
                let columns: Vec<_> = line.split(',').collect();
                assert_eq!(columns.len(), 11);
                assert_eq!(columns[0], "CredentialOfferError");
                columns[1].to_owned()
            })
            .collect()
    }

    fn assert_inventory_extends_baseline(inventory: &[String], baseline: &[String]) {
        let unique: BTreeSet<_> = inventory.iter().collect();

        assert!(inventory.len() >= baseline.len());
        assert_eq!(unique.len(), inventory.len());
        assert_eq!(&inventory[..baseline.len()], baseline);
    }

    #[test]
    fn compile_exhaustive_variant_inventory_extends_ordered_fixture_keys() {
        let inventory: Vec<_> = CredentialOfferError::CONTRACT_VARIANTS
            .iter()
            .map(|error| format!("{error:?}"))
            .collect();
        let baseline = fixture_variants();

        assert_eq!(baseline.len(), 171);
        assert_inventory_extends_baseline(&inventory, &baseline);
    }

    #[test]
    fn append_only_rule_accepts_a_unique_suffix() {
        let baseline = vec!["First".to_owned(), "Second".to_owned()];
        let inventory = vec![
            "First".to_owned(),
            "Second".to_owned(),
            "Appended".to_owned(),
        ];

        assert_inventory_extends_baseline(&inventory, &baseline);
    }

    #[test]
    #[should_panic]
    fn append_only_rule_rejects_baseline_reordering() {
        let baseline = vec!["First".to_owned(), "Second".to_owned()];
        let inventory = vec!["Second".to_owned(), "First".to_owned()];

        assert_inventory_extends_baseline(&inventory, &baseline);
    }

    #[test]
    #[should_panic]
    fn append_only_rule_rejects_duplicate_suffix_entries() {
        let baseline = vec!["First".to_owned(), "Second".to_owned()];
        let inventory = vec!["First".to_owned(), "Second".to_owned(), "Second".to_owned()];

        assert_inventory_extends_baseline(&inventory, &baseline);
    }
}
