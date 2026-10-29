use std::{collections::BTreeSet, error::Error as _};

use identus_core::{ErrorKind, IdentusError};
use identus_jose::{CAPABILITY, JweEncryptionError};

const GOLDEN: &str = include_str!("fixtures/jwe-encryption-error-contract-v1.csv");
const GOLDEN_HEADER: &str = "error_type,variant,code_constant,constant_visibility,code,kind,capability,local_display,public_message,identus_display,source";
const SOURCE_REPOSITORY: &str = "# source_repository=hyperledger-identus/sdk-rust";
const SOURCE_REVISION: &str = "# source_revision=c66d506690d3cd6f119f9ae6430b285f98115d10";
const GENERATED_AT: &str = "# generated_at=2026-10-29";

const CASES: [(&str, JweEncryptionError); 8] = [
    ("InvalidLimits", JweEncryptionError::InvalidLimits),
    (
        "InvalidRecipientKey",
        JweEncryptionError::InvalidRecipientKey,
    ),
    ("InvalidKeyId", JweEncryptionError::InvalidKeyId),
    ("PlaintextTooLarge", JweEncryptionError::PlaintextTooLarge),
    ("CompactTooLarge", JweEncryptionError::CompactTooLarge),
    (
        "InvalidCompactStructure",
        JweEncryptionError::InvalidCompactStructure,
    ),
    ("EncryptionRejected", JweEncryptionError::EncryptionRejected),
    (
        "EncrypterUnavailable",
        JweEncryptionError::EncrypterUnavailable,
    ),
];

fn kind_name(kind: ErrorKind) -> &'static str {
    match kind {
        ErrorKind::InvalidInput => "InvalidInput",
        ErrorKind::Unsupported => "Unsupported",
        ErrorKind::NotFound => "NotFound",
        ErrorKind::Conflict => "Conflict",
        ErrorKind::PolicyViolation => "PolicyViolation",
        ErrorKind::VerificationFailed => "VerificationFailed",
        ErrorKind::Transport => "Transport",
        ErrorKind::Storage => "Storage",
        ErrorKind::Crypto => "Crypto",
        ErrorKind::Trust => "Trust",
        ErrorKind::Internal => "Internal",
    }
}

fn golden_rows() -> Vec<Vec<&'static str>> {
    let mut lines = GOLDEN.lines().filter(|line| !line.starts_with('#'));
    assert_eq!(lines.next(), Some(GOLDEN_HEADER));
    lines
        .enumerate()
        .map(|(index, line)| {
            let columns: Vec<_> = line.split(',').collect();
            assert_eq!(
                columns.len(),
                11,
                "golden data row {} must have exactly 11 columns",
                index + 1
            );
            columns
        })
        .collect()
}

#[test]
fn embedded_stable_fixture_has_exact_provenance() {
    let mut lines = GOLDEN.lines();
    assert_eq!(lines.next(), Some(SOURCE_REPOSITORY));
    assert_eq!(lines.next(), Some(SOURCE_REVISION));
    assert_eq!(lines.next(), Some(GENERATED_AT));
    assert_eq!(lines.next(), Some(GOLDEN_HEADER));
}

#[test]
fn every_jwe_error_matches_the_independent_v1_golden() {
    let rows = golden_rows();
    assert_eq!(rows.len(), 8, "golden must contain exactly eight rows");

    let mut variants = BTreeSet::new();
    let mut codes = BTreeSet::new();

    for (index, ((variant, error), row)) in CASES.into_iter().zip(rows).enumerate() {
        assert!(variants.insert(variant), "duplicate variant {variant}");

        let local_display = error.to_string();
        let public = error.to_identus_error();
        let via_from: IdentusError = error.into();
        let identus_display = public.to_string();
        let debug_variant = format!("{error:?}");

        assert_eq!(row[0], "JweEncryptionError", "row {index}");
        assert_eq!(row[1], variant, "row {index}");
        assert_eq!(error as usize, index, "public enum order drifted");
        assert_eq!(debug_variant, variant, "row {index}");
        assert_eq!(row[2], "none", "row {index}");
        assert_eq!(row[3], "none", "row {index}");
        assert!(
            codes.insert(public.code().as_str()),
            "duplicate stable code"
        );
        assert_eq!(row[4], public.code().as_str(), "row {index}");
        assert_eq!(row[5], kind_name(public.kind()), "row {index}");
        assert_eq!(row[6], CAPABILITY.as_str(), "row {index}");
        assert_eq!(public.capability(), Some(CAPABILITY), "row {index}");
        assert_eq!(row[7], local_display, "row {index}");
        assert_eq!(row[8], public.public_message(), "row {index}");
        assert_eq!(row[7], row[8], "row {index}");
        assert_eq!(row[9], identus_display, "row {index}");
        assert_eq!(row[10], "none", "row {index}");
        assert_eq!(via_from, public, "row {index}");
        assert!(error.source().is_none(), "row {index}");
        assert!(public.source().is_none(), "row {index}");

        for canary in [
            "jwe-private-key",
            "jwe-private-plaintext",
            "jwe-private-ciphertext",
            "jwe-private-compact",
            "jwe-private-state",
            "jwe-private-engine-detail",
        ] {
            assert!(!local_display.contains(canary), "row {index}");
            assert!(!public.public_message().contains(canary), "row {index}");
            assert!(!identus_display.contains(canary), "row {index}");
            assert!(!debug_variant.contains(canary), "row {index}");
        }
    }

    assert_eq!(variants.len(), 8);
    assert_eq!(codes.len(), 8);
}
