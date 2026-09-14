use std::{collections::BTreeSet, error::Error as _};

use identus_core::{ErrorCode, ErrorKind, IdentusError};
use identus_presentations::{PresentationError, error::error_code};

const GOLDEN: &str = include_str!("fixtures/presentations-error-contract-v1.csv");
const GOLDEN_HEADER: &str = "error_type,variant,code_constant,constant_visibility,code,kind,capability,local_display,public_message,identus_display,source";
const SOURCE_REPOSITORY: &str = "# source_repository=hyperledger-identus/sdk-rust";
const SOURCE_REVISION: &str = "# source_revision=105308771dbebceb473b99d9eb82b0fa0178ab09";
const GENERATED_AT: &str = "# generated_at=2026-09-15";

#[derive(Clone, Copy)]
struct Case {
    variant: &'static str,
    code_constant: &'static str,
    code: ErrorCode,
    error: PresentationError,
    const_public: IdentusError,
}

macro_rules! presentation_case {
    ($variant:ident, $constant:ident) => {
        Case {
            variant: stringify!($variant),
            code_constant: stringify!($constant),
            code: error_code::$constant,
            error: PresentationError::$variant,
            const_public: PresentationError::$variant.to_identus_error(),
        }
    };
}

const CASES: [Case; 48] = [
    presentation_case!(InvalidQueryId, INVALID_QUERY_ID),
    presentation_case!(InvalidPurpose, INVALID_PURPOSE),
    presentation_case!(InvalidChallenge, INVALID_CHALLENGE),
    presentation_case!(InvalidCredentialHandle, INVALID_CREDENTIAL_HANDLE),
    presentation_case!(InvalidClaimIntent, INVALID_CLAIM_INTENT),
    presentation_case!(InvalidQueryFilters, INVALID_QUERY_FILTERS),
    presentation_case!(DuplicateIssuerFilter, DUPLICATE_ISSUER_FILTER),
    presentation_case!(DuplicateTypeFilter, DUPLICATE_TYPE_FILTER),
    presentation_case!(DuplicateSchemaFilter, DUPLICATE_SCHEMA_FILTER),
    presentation_case!(InvalidQueryClaims, INVALID_QUERY_CLAIMS),
    presentation_case!(DuplicateQueryClaim, DUPLICATE_QUERY_CLAIM),
    presentation_case!(InvalidRequestQueries, INVALID_REQUEST_QUERIES),
    presentation_case!(DuplicateQueryId, DUPLICATE_QUERY_ID),
    presentation_case!(InvalidCandidateClaims, INVALID_CANDIDATE_CLAIMS),
    presentation_case!(DuplicateCandidateClaim, DUPLICATE_CANDIDATE_CLAIM),
    presentation_case!(InvalidCandidates, INVALID_CANDIDATES),
    presentation_case!(DuplicateCandidate, DUPLICATE_CANDIDATE),
    presentation_case!(UnknownCandidateQuery, UNKNOWN_CANDIDATE_QUERY),
    presentation_case!(CandidateFormatMismatch, CANDIDATE_FORMAT_MISMATCH),
    presentation_case!(CandidateUnrequestedClaim, CANDIDATE_UNREQUESTED_CLAIM),
    presentation_case!(
        CandidateMissingRequiredClaim,
        CANDIDATE_MISSING_REQUIRED_CLAIM
    ),
    presentation_case!(CandidateRequestMismatch, CANDIDATE_REQUEST_MISMATCH),
    presentation_case!(InvalidSelectionClaims, INVALID_SELECTION_CLAIMS),
    presentation_case!(DuplicateSelectionClaim, DUPLICATE_SELECTION_CLAIM),
    presentation_case!(InvalidDisclosureSelections, INVALID_DISCLOSURE_SELECTIONS),
    presentation_case!(DuplicateDisclosureSelection, DUPLICATE_DISCLOSURE_SELECTION),
    presentation_case!(UnknownSelectionQuery, UNKNOWN_SELECTION_QUERY),
    presentation_case!(UnknownSelectionCandidate, UNKNOWN_SELECTION_CANDIDATE),
    presentation_case!(SelectionUnrequestedClaim, SELECTION_UNREQUESTED_CLAIM),
    presentation_case!(
        SelectionClaimIntentMismatch,
        SELECTION_CLAIM_INTENT_MISMATCH
    ),
    presentation_case!(SelectionUnavailableClaim, SELECTION_UNAVAILABLE_CLAIM),
    presentation_case!(
        SelectionMissingRequiredClaim,
        SELECTION_MISSING_REQUIRED_CLAIM
    ),
    presentation_case!(MissingQuerySelection, MISSING_QUERY_SELECTION),
    presentation_case!(QueryMultiplicityExceeded, QUERY_MULTIPLICITY_EXCEEDED),
    presentation_case!(DisclosureRequestMismatch, DISCLOSURE_REQUEST_MISMATCH),
    presentation_case!(InvalidArtifactBindings, INVALID_ARTIFACT_BINDINGS),
    presentation_case!(DuplicateArtifactBinding, DUPLICATE_ARTIFACT_BINDING),
    presentation_case!(InvalidArtifactPayload, INVALID_ARTIFACT_PAYLOAD),
    presentation_case!(InvalidGeneratedArtifacts, INVALID_GENERATED_ARTIFACTS),
    presentation_case!(
        ArtifactPayloadBudgetExceeded,
        ARTIFACT_PAYLOAD_BUDGET_EXCEEDED
    ),
    presentation_case!(UnknownArtifactSelection, UNKNOWN_ARTIFACT_SELECTION),
    presentation_case!(ArtifactFormatMismatch, ARTIFACT_FORMAT_MISMATCH),
    presentation_case!(
        DuplicateGeneratedArtifactBinding,
        DUPLICATE_GENERATED_ARTIFACT_BINDING
    ),
    presentation_case!(MissingArtifactSelection, MISSING_ARTIFACT_SELECTION),
    presentation_case!(InvalidLifecyclePhase, INVALID_LIFECYCLE_PHASE),
    presentation_case!(InvalidTerminalOutcome, INVALID_TERMINAL_OUTCOME),
    presentation_case!(InvalidProtocolState, INVALID_PROTOCOL_STATE),
    presentation_case!(InvalidProtocolTransition, INVALID_PROTOCOL_TRANSITION),
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

#[test]
fn embedded_stable_fixture_has_exact_provenance() {
    let mut lines = GOLDEN.lines();
    assert_eq!(lines.next(), Some(SOURCE_REPOSITORY));
    assert_eq!(lines.next(), Some(SOURCE_REVISION));
    assert_eq!(lines.next(), Some(GENERATED_AT));
    assert_eq!(lines.next(), Some(GOLDEN_HEADER));
}

#[test]
fn every_presentation_error_matches_the_planning_golden() {
    let mut lines = GOLDEN.lines().filter(|line| !line.starts_with('#'));
    assert_eq!(lines.next(), Some(GOLDEN_HEADER));

    let mut rows = Vec::new();
    for (index, line) in lines.enumerate() {
        let columns: Vec<_> = line.split(',').collect();
        assert_eq!(
            columns.len(),
            11,
            "golden data row {} must have exactly 11 columns",
            index + 1
        );
        rows.push(columns);
    }
    assert_eq!(rows.len(), 48, "golden must contain exactly 48 rows");

    let mut constant_names = BTreeSet::new();
    let mut stable_codes = BTreeSet::new();
    for (index, (case, row)) in CASES.into_iter().zip(rows).enumerate() {
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
        let from_public = IdentusError::from(case.error);
        let identus_display = public.to_string();
        let debug_variant = format!("{:?}", case.error);

        assert_eq!(row[0], "PresentationError");
        assert_eq!(row[1], case.variant);
        assert_eq!(case.error as usize, index, "public enum order drifted");
        assert_eq!(debug_variant, case.variant);
        assert_eq!(row[2], case.code_constant);
        assert_eq!(row[3], "public");
        assert_eq!(row[4], case.code.as_str());
        assert_eq!(public, case.const_public);
        assert_eq!(from_public, public);
        assert_eq!(public.code(), case.code);
        assert_eq!(row[4], public.code().as_str());
        assert_eq!(row[5], kind_name(public.kind()));
        assert_eq!(public.kind(), ErrorKind::InvalidInput);
        assert_eq!(
            row[6],
            public.capability().expect("public capability").as_str()
        );
        assert_eq!(row[6], "presentation");
        assert_eq!(row[7], local_display);
        assert_eq!(row[8], public.public_message());
        assert_eq!(row[7], row[8]);
        assert_eq!(row[9], identus_display);
        assert_eq!(row[10], "none");
        assert!(case.error.source().is_none());
        assert!(public.source().is_none());

        for canary in [
            "holder-private-input",
            "verifier-private-input",
            "credential-secret",
            "claim-value",
            "parser-detail",
            "https://private.example/presentation/123",
        ] {
            assert!(!local_display.contains(canary));
            assert!(!public.public_message().contains(canary));
            assert!(!identus_display.contains(canary));
            assert!(!debug_variant.contains(canary));
        }
    }

    assert_eq!(constant_names.len(), 48);
    assert_eq!(stable_codes.len(), 48);
}

#[test]
fn catalogue_is_private_and_every_bridge_is_const_usable() {
    assert_eq!(CASES.len(), 48);
    assert!(
        CASES
            .iter()
            .all(|case| case.const_public == case.error.to_identus_error())
    );

    let crate_root = include_str!("../src/lib.rs");
    assert!(crate_root.contains("mod error_contract;"));
    assert!(!crate_root.contains("pub mod error_contract;"));
}
