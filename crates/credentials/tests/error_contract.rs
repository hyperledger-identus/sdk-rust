use std::{collections::BTreeMap, error::Error as _};

use identus_core::{ErrorCode, ErrorKind, IdentusError};
use identus_credentials::{CredentialError, CredentialVerificationError, error::error_code};

const GOLDEN: &str = include_str!("fixtures/credentials-error-contract-v1.csv");
const GOLDEN_HEADER: &str = "error_type,variant,code_constant,constant_visibility,code,kind,capability,local_display,public_message,identus_display,source";

const CONST_VERIFICATION_ERRORS: [IdentusError; 3] = [
    CredentialVerificationError::UnsupportedFormat.to_identus_error(),
    CredentialVerificationError::Unavailable.to_identus_error(),
    CredentialVerificationError::Internal.to_identus_error(),
];

#[derive(Clone, Copy)]
enum PilotError {
    Credential(CredentialError),
    Verification(CredentialVerificationError),
}

impl PilotError {
    fn local_display(self) -> String {
        match self {
            Self::Credential(error) => error.to_string(),
            Self::Verification(error) => error.to_string(),
        }
    }

    fn public(self) -> IdentusError {
        match self {
            Self::Credential(error) => error.to_identus_error(),
            Self::Verification(error) => error.to_identus_error(),
        }
    }

    fn debug_variant(self) -> String {
        match self {
            Self::Credential(error) => format!("{error:?}"),
            Self::Verification(error) => format!("{error:?}"),
        }
    }

    fn has_no_source(self) -> bool {
        match self {
            Self::Credential(error) => error.source().is_none(),
            Self::Verification(error) => error.source().is_none(),
        }
    }
}

#[derive(Clone, Copy)]
struct Case {
    error_type: &'static str,
    variant: &'static str,
    code_constant: &'static str,
    constant_visibility: &'static str,
    code: ErrorCode,
    error: PilotError,
}

macro_rules! credential_case {
    ($variant:ident, $constant:ident) => {
        Case {
            error_type: "CredentialError",
            variant: stringify!($variant),
            code_constant: stringify!($constant),
            constant_visibility: "public",
            code: error_code::$constant,
            error: PilotError::Credential(CredentialError::$variant),
        }
    };
}

macro_rules! verification_case {
    ($variant:ident, $constant:ident, $code:literal) => {
        Case {
            error_type: "CredentialVerificationError",
            variant: stringify!($variant),
            code_constant: stringify!($constant),
            constant_visibility: "private",
            code: ErrorCode::new($code),
            error: PilotError::Verification(CredentialVerificationError::$variant),
        }
    };
}

const CASES: [Case; 47] = [
    credential_case!(InvalidFormat, INVALID_FORMAT),
    credential_case!(EmptyPayload, EMPTY_PAYLOAD),
    credential_case!(PayloadTooLarge, PAYLOAD_TOO_LARGE),
    credential_case!(EmptyDetachedProof, EMPTY_DETACHED_PROOF),
    credential_case!(DetachedProofTooLarge, DETACHED_PROOF_TOO_LARGE),
    credential_case!(EmptyPrivateMaterial, EMPTY_PRIVATE_MATERIAL),
    credential_case!(PrivateMaterialTooLarge, PRIVATE_MATERIAL_TOO_LARGE),
    credential_case!(
        InvalidVerificationStageName,
        INVALID_VERIFICATION_STAGE_NAME
    ),
    credential_case!(
        InvalidVerificationReasonCode,
        INVALID_VERIFICATION_REASON_CODE
    ),
    credential_case!(MissingVerificationReason, MISSING_VERIFICATION_REASON),
    credential_case!(UnexpectedVerificationReason, UNEXPECTED_VERIFICATION_REASON),
    credential_case!(
        NonCanonicalVerificationReport,
        NON_CANONICAL_VERIFICATION_REPORT
    ),
    credential_case!(
        DuplicateCredentialVerifierFormat,
        DUPLICATE_CREDENTIAL_VERIFIER_FORMAT
    ),
    credential_case!(
        TooManyCredentialVerifierFormats,
        TOO_MANY_CREDENTIAL_VERIFIER_FORMATS
    ),
    credential_case!(InvalidEntityIdentifier, INVALID_ENTITY_IDENTIFIER),
    credential_case!(InvalidCredentialType, INVALID_CREDENTIAL_TYPE),
    credential_case!(InvalidSchemaIdentifier, INVALID_SCHEMA_IDENTIFIER),
    credential_case!(InvalidSchemaVersion, INVALID_SCHEMA_VERSION),
    credential_case!(InvalidClaimIdentifier, INVALID_CLAIM_IDENTIFIER),
    credential_case!(InvalidClaimValueType, INVALID_CLAIM_VALUE_TYPE),
    credential_case!(InvalidClaimPathSegment, INVALID_CLAIM_PATH_SEGMENT),
    credential_case!(InvalidClaimPath, INVALID_CLAIM_PATH),
    credential_case!(InvalidClaimDisclosure, INVALID_CLAIM_DISCLOSURE),
    credential_case!(InvalidDescriptorCollection, INVALID_DESCRIPTOR_COLLECTION),
    credential_case!(DuplicateCredentialSubject, DUPLICATE_CREDENTIAL_SUBJECT),
    credential_case!(DuplicateCredentialType, DUPLICATE_CREDENTIAL_TYPE),
    credential_case!(DuplicateSchemaIdentifier, DUPLICATE_SCHEMA_IDENTIFIER),
    credential_case!(DuplicateClaimIdentifier, DUPLICATE_CLAIM_IDENTIFIER),
    credential_case!(DuplicateClaimPath, DUPLICATE_CLAIM_PATH),
    credential_case!(InvalidValidityRange, INVALID_VALIDITY_RANGE),
    credential_case!(InvalidStatusMethod, INVALID_STATUS_METHOD),
    credential_case!(InvalidStatusPurpose, INVALID_STATUS_PURPOSE),
    credential_case!(InvalidStatusReference, INVALID_STATUS_REFERENCE),
    credential_case!(InvalidStatusHandle, INVALID_STATUS_HANDLE),
    credential_case!(InvalidStatusRevision, INVALID_STATUS_REVISION),
    credential_case!(InvalidStatusValue, INVALID_STATUS_VALUE),
    credential_case!(
        InvalidStatusBindingCollection,
        INVALID_STATUS_BINDING_COLLECTION
    ),
    credential_case!(DuplicateStatusBinding, DUPLICATE_STATUS_BINDING),
    credential_case!(InvalidStatusFreshness, INVALID_STATUS_FRESHNESS),
    credential_case!(InvalidStatusRequirements, INVALID_STATUS_REQUIREMENTS),
    credential_case!(DuplicateStatusMethod, DUPLICATE_STATUS_METHOD),
    credential_case!(DuplicateStatusPurpose, DUPLICATE_STATUS_PURPOSE),
    credential_case!(InvalidStatusEvidenceRange, INVALID_STATUS_EVIDENCE_RANGE),
    credential_case!(StatusQueryMismatch, STATUS_QUERY_MISMATCH),
    verification_case!(
        UnsupportedFormat,
        VERIFICATION_UNSUPPORTED_FORMAT,
        "credential.verification_unsupported_format"
    ),
    verification_case!(
        Unavailable,
        VERIFICATION_UNAVAILABLE,
        "credential.verification_unavailable"
    ),
    verification_case!(
        Internal,
        VERIFICATION_INTERNAL,
        "credential.verification_internal"
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

#[test]
fn every_pilot_variant_matches_the_planning_golden() {
    let mut lines = GOLDEN.lines().filter(|line| !line.starts_with('#'));
    assert_eq!(lines.next(), Some(GOLDEN_HEADER));

    let mut rows = BTreeMap::new();
    for (index, line) in lines.enumerate() {
        let columns: Vec<_> = line.split(',').collect();
        assert_eq!(
            columns.len(),
            11,
            "golden data row {} must have exactly 11 columns",
            index + 1
        );
        let key = (columns[0], columns[1]);
        assert!(
            rows.insert(key, columns).is_none(),
            "duplicate golden row for {}::{}",
            key.0,
            key.1
        );
    }
    assert_eq!(rows.len(), 47, "golden must contain exactly 47 rows");

    let mut constant_names = std::collections::BTreeSet::new();
    let mut stable_codes = std::collections::BTreeSet::new();
    for case in CASES {
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

        let row = rows
            .remove(&(case.error_type, case.variant))
            .unwrap_or_else(|| {
                panic!(
                    "missing golden row for {}::{}",
                    case.error_type, case.variant
                )
            });
        let public = case.error.public();
        let local_display = case.error.local_display();
        let identus_display = public.to_string();
        let debug_variant = case.error.debug_variant();

        assert_eq!(row[0], case.error_type);
        assert_eq!(row[1], case.variant);
        assert_eq!(debug_variant, case.variant);
        assert_eq!(row[2], case.code_constant);
        assert_eq!(row[3], case.constant_visibility);
        assert_eq!(row[4], case.code.as_str());
        assert_eq!(public.code(), case.code);
        assert_eq!(row[4], public.code().as_str());
        assert_eq!(row[5], kind_name(public.kind()));
        assert_eq!(
            row[6],
            public.capability().expect("public capability").as_str()
        );
        assert_eq!(row[7], local_display);
        assert_eq!(row[8], public.public_message());
        assert_eq!(row[9], identus_display);
        assert_eq!(row[10], "none");
        assert!(case.error.has_no_source());

        for canary in [
            "credential-pii",
            "proof-secret",
            "holder-opening",
            "caller-controlled-value",
            "https://private.example/status/123",
        ] {
            assert!(!local_display.contains(canary));
            assert!(!public.public_message().contains(canary));
            assert!(!identus_display.contains(canary));
            assert!(!debug_variant.contains(canary));
        }
    }

    assert!(rows.is_empty(), "golden contains an unenumerated variant");
    assert_eq!(constant_names.len(), 47);
    assert_eq!(stable_codes.len(), 47);
}

#[test]
fn verifier_bridge_remains_const_and_code_constants_remain_private() {
    assert_eq!(CONST_VERIFICATION_ERRORS.len(), 3);
    assert_eq!(
        CONST_VERIFICATION_ERRORS.map(|error| error.code().as_str()),
        [
            "credential.verification_unsupported_format",
            "credential.verification_unavailable",
            "credential.verification_internal",
        ]
    );

    let crate_root = include_str!("../src/lib.rs");
    assert!(crate_root.contains("mod error_contract;"));
    assert!(!crate_root.contains("pub mod error_contract;"));

    let source = include_str!("../src/error_contract/verifier.rs");
    for name in [
        "VERIFICATION_UNSUPPORTED_FORMAT",
        "VERIFICATION_UNAVAILABLE",
        "VERIFICATION_INTERNAL",
    ] {
        assert!(source.contains(&format!("pub(crate) const {name}: ErrorContract")));
        assert!(!source.contains(&format!("pub const {name}: ErrorContract")));
    }
}
