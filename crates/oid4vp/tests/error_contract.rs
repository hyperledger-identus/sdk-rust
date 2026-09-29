use std::collections::BTreeSet;

use identus_core::{ErrorCode, ErrorKind, IdentusError};
use identus_oid4vp::{CAPABILITY, Oid4vpError, error_code};

const GOLDEN: &str = include_str!("fixtures/oid4vp-error-contract-v1.csv");
const GOLDEN_HEADER: &str = "variant,code,kind,public_message";

#[derive(Clone, Copy)]
struct Case {
    variant: &'static str,
    code: ErrorCode,
    error: Oid4vpError,
}

macro_rules! case {
    ($variant:ident, $constant:ident) => {
        Case {
            variant: stringify!($variant),
            code: error_code::$constant,
            error: Oid4vpError::$variant,
        }
    };
}

const CASES: [Case; 50] = [
    case!(InvalidLimits, INVALID_LIMITS),
    case!(InvocationTooLarge, INVOCATION_TOO_LARGE),
    case!(InvalidInvocation, INVALID_INVOCATION),
    case!(TooManyParameters, TOO_MANY_PARAMETERS),
    case!(InvalidFormEncoding, INVALID_FORM_ENCODING),
    case!(ParameterNameTooLarge, PARAMETER_NAME_TOO_LARGE),
    case!(ParameterValueTooLarge, PARAMETER_VALUE_TOO_LARGE),
    case!(DuplicateParameter, DUPLICATE_PARAMETER),
    case!(ClientIdTooLarge, CLIENT_ID_TOO_LARGE),
    case!(RequestUriTooLarge, REQUEST_URI_TOO_LARGE),
    case!(MissingRequiredParameter, MISSING_REQUIRED_PARAMETER),
    case!(UnsupportedTransport, UNSUPPORTED_TRANSPORT),
    case!(UnsupportedParameter, UNSUPPORTED_PARAMETER),
    case!(UnsupportedRequestUriMethod, UNSUPPORTED_REQUEST_URI_METHOD),
    case!(UnsafeRequestUri, UNSAFE_REQUEST_URI),
    case!(RetrievalMethodMismatch, RETRIEVAL_METHOD_MISMATCH),
    case!(WalletMetadataTooLarge, WALLET_METADATA_TOO_LARGE),
    case!(InvalidWalletMetadata, INVALID_WALLET_METADATA),
    case!(WalletNonceTooLarge, WALLET_NONCE_TOO_LARGE),
    case!(InvalidWalletNonce, INVALID_WALLET_NONCE),
    case!(RequestBodyTooLarge, REQUEST_BODY_TOO_LARGE),
    case!(ResponseContentTypeTooLarge, RESPONSE_CONTENT_TYPE_TOO_LARGE),
    case!(ResponseBodyTooLarge, RESPONSE_BODY_TOO_LARGE),
    case!(RequestUriHttpError, REQUEST_URI_HTTP_ERROR),
    case!(
        InvalidRequestObjectMediaType,
        INVALID_REQUEST_OBJECT_MEDIA_TYPE
    ),
    case!(EmptyRequestObject, EMPTY_REQUEST_OBJECT),
    case!(
        UnsupportedEncryptedRequestObject,
        UNSUPPORTED_ENCRYPTED_REQUEST_OBJECT
    ),
    case!(InvalidRequestObject, INVALID_REQUEST_OBJECT),
    case!(InvalidRequestObjectType, INVALID_REQUEST_OBJECT_TYPE),
    case!(
        InvalidRequestObjectSignature,
        INVALID_REQUEST_OBJECT_SIGNATURE
    ),
    case!(InvalidRequestObjectPayload, INVALID_REQUEST_OBJECT_PAYLOAD),
    case!(
        RequestObjectClientIdMismatch,
        REQUEST_OBJECT_CLIENT_ID_MISMATCH
    ),
    case!(
        RequestObjectWalletNonceMismatch,
        REQUEST_OBJECT_WALLET_NONCE_MISMATCH
    ),
    case!(MissingDcqlQuery, MISSING_DCQL_QUERY),
    case!(UnsupportedDcqlScope, UNSUPPORTED_DCQL_SCOPE),
    case!(InvalidDcqlQuery, INVALID_DCQL_QUERY),
    case!(DcqlQueryTooLarge, DCQL_QUERY_TOO_LARGE),
    case!(DcqlWorkLimitExceeded, DCQL_WORK_LIMIT_EXCEEDED),
    case!(InvalidDcqlCredential, INVALID_DCQL_CREDENTIAL),
    case!(MissingResponseType, MISSING_RESPONSE_TYPE),
    case!(UnsupportedResponseType, UNSUPPORTED_RESPONSE_TYPE),
    case!(MissingResponseMode, MISSING_RESPONSE_MODE),
    case!(UnsupportedResponseMode, UNSUPPORTED_RESPONSE_MODE),
    case!(MissingAuthorizationNonce, MISSING_AUTHORIZATION_NONCE),
    case!(InvalidAuthorizationNonce, INVALID_AUTHORIZATION_NONCE),
    case!(AuthorizationNonceTooLarge, AUTHORIZATION_NONCE_TOO_LARGE),
    case!(MissingResponseUri, MISSING_RESPONSE_URI),
    case!(
        ConflictingResponseDestination,
        CONFLICTING_RESPONSE_DESTINATION
    ),
    case!(ResponseUriTooLarge, RESPONSE_URI_TOO_LARGE),
    case!(UnsafeResponseUri, UNSAFE_RESPONSE_URI),
];

#[test]
fn public_error_contract_matches_the_versioned_fixture() {
    let mut lines = GOLDEN.lines().filter(|line| !line.starts_with('#'));
    assert_eq!(lines.next(), Some(GOLDEN_HEADER));

    let rows: Vec<_> = lines.collect();
    assert_eq!(rows.len(), CASES.len());
    let mut codes = BTreeSet::new();

    for (row, case) in rows.iter().zip(CASES) {
        let columns: Vec<_> = row.splitn(4, ',').collect();
        assert_eq!(columns.len(), 4);
        assert_eq!(columns[0], case.variant);
        assert_eq!(columns[1], case.code.as_str());
        let public = case.error.to_identus_error();
        assert_eq!(columns[2], format!("{:?}", public.kind()));
        assert_eq!(columns[3], public.public_message());
        assert_eq!(public.code(), case.code);
        assert_eq!(public.capability(), Some(CAPABILITY));
        assert!(codes.insert(case.code.as_str()));
    }
}

#[test]
fn local_and_shared_diagnostics_are_static_and_redacted() {
    const CANARY: &str = "VERIFIER_SECRET_CANARY";
    for case in CASES {
        let public: IdentusError = case.error.into();
        for rendered in [
            format!("{}", case.error),
            format!("{:?}", case.error),
            format!("{public}"),
            format!("{public:?}"),
        ] {
            assert!(!rendered.contains(CANARY));
        }
        assert!(matches!(
            public.kind(),
            ErrorKind::InvalidInput | ErrorKind::Unsupported
        ));
    }
}

#[test]
fn compile_inventory_extends_the_ordered_fixture() {
    let fixture: Vec<_> = GOLDEN
        .lines()
        .filter(|line| !line.starts_with('#'))
        .skip(1)
        .map(|line| line.split(',').next().expect("variant"))
        .collect();
    let inventory: Vec<_> = Oid4vpError::CONTRACT_VARIANTS
        .iter()
        .map(|error| format!("{error:?}"))
        .collect();
    assert_eq!(inventory, fixture);
}
