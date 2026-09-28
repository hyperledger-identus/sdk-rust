use std::{collections::BTreeSet, error::Error as _};

use identus_core::{ErrorCode, ErrorKind, IdentusError};
use identus_jose::{CAPABILITY, JoseError, error_code};

const GOLDEN: &str = include_str!("fixtures/jose-error-contract-v2.csv");
const GOLDEN_HEADER: &str = "error_type,variant,code_constant,constant_visibility,code,kind,capability,local_display,public_message,identus_display,source";
const SOURCE_REPOSITORY: &str = "# source_repository=hyperledger-identus/sdk-rust";
const SOURCE_REVISION: &str = "# source_revision=61b8650452812900203658f3072df11b0b2a378c";
const GENERATED_AT: &str = "# generated_at=2026-09-28";

#[derive(Clone, Copy)]
struct Case {
    variant: &'static str,
    code_constant: &'static str,
    code: ErrorCode,
    error: JoseError,
    const_public: IdentusError,
}

macro_rules! jose_case {
    ($variant:ident, $constant:ident) => {
        Case {
            variant: stringify!($variant),
            code_constant: stringify!($constant),
            code: error_code::$constant,
            error: JoseError::$variant,
            const_public: JoseError::$variant.to_identus_error(),
        }
    };
}

const CASES: [Case; 26] = [
    jose_case!(InvalidLimits, INVALID_LIMITS),
    jose_case!(CompactTooLarge, COMPACT_TOO_LARGE),
    jose_case!(InvalidCompactStructure, INVALID_COMPACT_STRUCTURE),
    jose_case!(NonCanonicalBase64Url, NON_CANONICAL_BASE64URL),
    jose_case!(ProtectedHeaderTooLarge, PROTECTED_HEADER_TOO_LARGE),
    jose_case!(PayloadTooLarge, PAYLOAD_TOO_LARGE),
    jose_case!(SignatureTooLarge, SIGNATURE_TOO_LARGE),
    jose_case!(InvalidProtectedHeader, INVALID_PROTECTED_HEADER),
    jose_case!(DuplicateProtectedHeader, DUPLICATE_PROTECTED_HEADER),
    jose_case!(UnknownProtectedHeader, UNKNOWN_PROTECTED_HEADER),
    jose_case!(MissingAlgorithm, MISSING_ALGORITHM),
    jose_case!(InvalidHeaderValue, INVALID_HEADER_VALUE),
    jose_case!(AmbiguousKeyReference, AMBIGUOUS_KEY_REFERENCE),
    jose_case!(EmptySignature, EMPTY_SIGNATURE),
    jose_case!(SizeOverflow, SIZE_OVERFLOW),
    jose_case!(UnsupportedAlgorithm, UNSUPPORTED_ALGORITHM),
    jose_case!(AlgorithmMismatch, ALGORITHM_MISMATCH),
    jose_case!(InvalidVerificationKey, INVALID_VERIFICATION_KEY),
    jose_case!(InvalidRegistryCapacity, INVALID_REGISTRY_CAPACITY),
    jose_case!(RegistryFull, REGISTRY_FULL),
    jose_case!(DuplicateAlgorithm, DUPLICATE_ALGORITHM),
    jose_case!(AlgorithmNotAllowed, ALGORITHM_NOT_ALLOWED),
    jose_case!(InvalidSignatureLength, INVALID_SIGNATURE_LENGTH),
    jose_case!(SigningRejected, SIGNING_REJECTED),
    jose_case!(SignerUnavailable, SIGNER_UNAVAILABLE),
    jose_case!(SignatureInvalid, SIGNATURE_INVALID),
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
fn every_jose_error_matches_the_ordered_planning_golden() {
    let rows = golden_rows();
    assert_eq!(rows.len(), 26, "golden must contain exactly 26 rows");

    let mut variant_names = BTreeSet::new();
    let mut constant_names = BTreeSet::new();
    let mut stable_codes = BTreeSet::new();

    for (index, (case, row)) in CASES.into_iter().zip(rows).enumerate() {
        assert!(
            variant_names.insert(case.variant),
            "duplicate variant inventory entry {}",
            case.variant
        );
        assert!(
            constant_names.insert(case.code_constant),
            "duplicate constant inventory entry {}",
            case.code_constant
        );
        assert!(
            stable_codes.insert(case.code.as_str()),
            "duplicate stable code inventory entry {}",
            case.code
        );

        let local_display = case.error.to_string();
        let public = case.error.to_identus_error();
        let via_from: IdentusError = case.error.into();
        let identus_display = public.to_string();
        let debug_variant = format!("{:?}", case.error);

        assert_eq!(row[0], "JoseError", "row {index}");
        assert_eq!(row[1], case.variant, "row {index}");
        assert_eq!(case.error as usize, index, "public enum order drifted");
        assert_eq!(debug_variant, case.variant, "row {index}");
        assert_eq!(row[2], case.code_constant, "row {index}");
        assert_eq!(row[3], "public", "row {index}");
        assert_eq!(row[4], case.code.as_str(), "row {index}");
        assert_eq!(public, case.const_public, "row {index}");
        assert_eq!(via_from, public, "row {index}");
        assert_eq!(public.code(), case.code, "row {index}");
        assert_eq!(row[4], public.code().as_str(), "row {index}");
        assert_eq!(row[5], kind_name(public.kind()), "row {index}");
        assert_eq!(row[6], CAPABILITY.as_str(), "row {index}");
        assert_eq!(public.capability(), Some(CAPABILITY), "row {index}");
        assert_eq!(row[7], local_display, "row {index}");
        assert_eq!(row[8], public.public_message(), "row {index}");
        assert_eq!(row[7], row[8], "row {index}");
        assert_eq!(row[9], identus_display, "row {index}");
        assert_eq!(row[10], "none", "row {index}");
        assert!(case.error.source().is_none(), "row {index}");
        assert!(public.source().is_none(), "row {index}");

        for canary in [
            "token-private-input",
            "header-private-input",
            "key-private-input",
            "algorithm-private-input",
            "parser-private-detail",
            "did:example:private#key-1",
        ] {
            assert!(!local_display.contains(canary), "row {index}");
            assert!(!public.public_message().contains(canary), "row {index}");
            assert!(!identus_display.contains(canary), "row {index}");
            assert!(!debug_variant.contains(canary), "row {index}");
        }
    }

    assert_eq!(variant_names.len(), 26);
    assert_eq!(constant_names.len(), 26);
    assert_eq!(stable_codes.len(), 26);
}

#[test]
fn size_overflow_contract_is_explicit_and_stable() {
    let local = JoseError::SizeOverflow;
    let public = local.to_identus_error();

    assert_eq!(local.to_string(), "JWS size is invalid");
    assert_eq!(public.code(), error_code::SIZE_OVERFLOW);
    assert_eq!(public.kind(), ErrorKind::InvalidInput);
    assert_eq!(public.capability(), Some(CAPABILITY));
    assert_eq!(public.public_message(), "JWS size is invalid");
    assert_eq!(
        public.to_string(),
        "jose.size_overflow: JWS size is invalid"
    );
    assert!(local.source().is_none());
    assert!(public.source().is_none());
}

#[test]
fn catalogue_is_private_and_every_bridge_is_const_usable() {
    assert_eq!(CASES.len(), 26);
    assert!(
        CASES
            .iter()
            .all(|case| case.const_public == case.error.to_identus_error())
    );

    let crate_root = include_str!("../src/lib.rs");
    assert!(crate_root.contains("mod error_contract;"));
    assert!(!crate_root.contains("pub mod error_contract;"));
}
