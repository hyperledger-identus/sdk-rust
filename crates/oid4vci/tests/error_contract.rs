use std::{collections::BTreeSet, error::Error as _};

use identus_core::{ErrorCode, ErrorKind, IdentusError};
use identus_oid4vci::{CAPABILITY, CredentialOfferError, error_code};

const GOLDEN: &str = include_str!("fixtures/oid4vci-error-contract-v1.csv");
const GOLDEN_HEADER: &str = "error_type,variant,code_constant,constant_visibility,code,kind,capability,local_display,public_message,identus_display,source";
const SOURCE_REPOSITORY: &str = "# source_repository=hyperledger-identus/sdk-rust";
const SOURCE_REVISION: &str = "# source_revision=6217384f85ff72003a7b94482bf7879f4587be02";
const GENERATED_AT: &str = "# generated_at=2026-09-15";

#[derive(Clone, Copy)]
struct Case {
    variant: &'static str,
    code_constant: &'static str,
    code: ErrorCode,
    error: CredentialOfferError,
    const_public: IdentusError,
}

macro_rules! oid4vci_case {
    ($variant:ident, $constant:ident) => {
        Case {
            variant: stringify!($variant),
            code_constant: stringify!($constant),
            code: error_code::$constant,
            error: CredentialOfferError::$variant,
            const_public: CredentialOfferError::$variant.to_identus_error(),
        }
    };
}

static CASES: [Case; 171] = [
    oid4vci_case!(InvalidLimits, INVALID_LIMITS),
    oid4vci_case!(InvocationTooLarge, INVOCATION_TOO_LARGE),
    oid4vci_case!(InvalidInvocation, INVALID_INVOCATION),
    oid4vci_case!(UnsupportedTransport, UNSUPPORTED_TRANSPORT),
    oid4vci_case!(InvalidFormEncoding, INVALID_FORM_ENCODING),
    oid4vci_case!(EmbeddedTooLarge, EMBEDDED_TOO_LARGE),
    oid4vci_case!(InvalidEmbeddedJson, INVALID_EMBEDDED_JSON),
    oid4vci_case!(DuplicateJsonProperty, DUPLICATE_JSON_PROPERTY),
    oid4vci_case!(JsonTooDeep, JSON_TOO_DEEP),
    oid4vci_case!(JsonTooManyNodes, JSON_TOO_MANY_NODES),
    oid4vci_case!(ReferenceTooLarge, REFERENCE_TOO_LARGE),
    oid4vci_case!(UnsafeReferenceUri, UNSAFE_REFERENCE_URI),
    oid4vci_case!(InvalidSemanticLimits, INVALID_SEMANTIC_LIMITS),
    oid4vci_case!(InvalidOfferFields, INVALID_OFFER_FIELDS),
    oid4vci_case!(IssuerTooLarge, ISSUER_TOO_LARGE),
    oid4vci_case!(UnsafeCredentialIssuer, UNSAFE_CREDENTIAL_ISSUER),
    oid4vci_case!(InvalidConfigurationIds, INVALID_CONFIGURATION_IDS),
    oid4vci_case!(ConfigurationIdTooLarge, CONFIGURATION_ID_TOO_LARGE),
    oid4vci_case!(TooManyConfigurationIds, TOO_MANY_CONFIGURATION_IDS),
    oid4vci_case!(DuplicateConfigurationId, DUPLICATE_CONFIGURATION_ID),
    oid4vci_case!(InvalidGrants, INVALID_GRANTS),
    oid4vci_case!(InvalidGrantLimits, INVALID_GRANT_LIMITS),
    oid4vci_case!(
        InvalidAuthorizationCodeGrant,
        INVALID_AUTHORIZATION_CODE_GRANT
    ),
    oid4vci_case!(
        InvalidPreAuthorizedCodeGrant,
        INVALID_PRE_AUTHORIZED_CODE_GRANT
    ),
    oid4vci_case!(IssuerStateTooLarge, ISSUER_STATE_TOO_LARGE),
    oid4vci_case!(PreAuthorizedCodeTooLarge, PRE_AUTHORIZED_CODE_TOO_LARGE),
    oid4vci_case!(AuthorizationServerTooLarge, AUTHORIZATION_SERVER_TOO_LARGE),
    oid4vci_case!(UnsafeAuthorizationServer, UNSAFE_AUTHORIZATION_SERVER),
    oid4vci_case!(InvalidTransactionCode, INVALID_TRANSACTION_CODE),
    oid4vci_case!(InvalidTransactionCodeMode, INVALID_TRANSACTION_CODE_MODE),
    oid4vci_case!(
        InvalidTransactionCodeLength,
        INVALID_TRANSACTION_CODE_LENGTH
    ),
    oid4vci_case!(
        TransactionCodeLengthTooLarge,
        TRANSACTION_CODE_LENGTH_TOO_LARGE
    ),
    oid4vci_case!(
        TransactionCodeDescriptionTooLarge,
        TRANSACTION_CODE_DESCRIPTION_TOO_LARGE
    ),
    oid4vci_case!(InvalidMetadataLimits, INVALID_METADATA_LIMITS),
    oid4vci_case!(MetadataTooLarge, METADATA_TOO_LARGE),
    oid4vci_case!(InvalidMetadata, INVALID_METADATA),
    oid4vci_case!(MetadataIssuerMismatch, METADATA_ISSUER_MISMATCH),
    oid4vci_case!(CredentialEndpointTooLarge, CREDENTIAL_ENDPOINT_TOO_LARGE),
    oid4vci_case!(UnsafeCredentialEndpoint, UNSAFE_CREDENTIAL_ENDPOINT),
    oid4vci_case!(NonceEndpointTooLarge, NONCE_ENDPOINT_TOO_LARGE),
    oid4vci_case!(UnsafeNonceEndpoint, UNSAFE_NONCE_ENDPOINT),
    oid4vci_case!(
        DeferredCredentialEndpointTooLarge,
        DEFERRED_CREDENTIAL_ENDPOINT_TOO_LARGE
    ),
    oid4vci_case!(
        UnsafeDeferredCredentialEndpoint,
        UNSAFE_DEFERRED_CREDENTIAL_ENDPOINT
    ),
    oid4vci_case!(NonceEndpointRequired, NONCE_ENDPOINT_REQUIRED),
    oid4vci_case!(InvalidAuthorizationServers, INVALID_AUTHORIZATION_SERVERS),
    oid4vci_case!(TooManyAuthorizationServers, TOO_MANY_AUTHORIZATION_SERVERS),
    oid4vci_case!(DuplicateAuthorizationServer, DUPLICATE_AUTHORIZATION_SERVER),
    oid4vci_case!(
        InvalidCredentialConfigurations,
        INVALID_CREDENTIAL_CONFIGURATIONS
    ),
    oid4vci_case!(
        TooManyCredentialConfigurations,
        TOO_MANY_CREDENTIAL_CONFIGURATIONS
    ),
    oid4vci_case!(InvalidCredentialFormat, INVALID_CREDENTIAL_FORMAT),
    oid4vci_case!(CredentialFormatTooLarge, CREDENTIAL_FORMAT_TOO_LARGE),
    oid4vci_case!(OfferMetadataIssuerMismatch, OFFER_METADATA_ISSUER_MISMATCH),
    oid4vci_case!(OfferedConfigurationMissing, OFFERED_CONFIGURATION_MISSING),
    oid4vci_case!(
        InvalidAuthorizationServerHint,
        INVALID_AUTHORIZATION_SERVER_HINT
    ),
    oid4vci_case!(
        InvalidAuthorizationServerMetadataLimits,
        INVALID_AUTHORIZATION_SERVER_METADATA_LIMITS
    ),
    oid4vci_case!(
        AuthorizationServerMetadataTooLarge,
        AUTHORIZATION_SERVER_METADATA_TOO_LARGE
    ),
    oid4vci_case!(
        InvalidAuthorizationServerMetadata,
        INVALID_AUTHORIZATION_SERVER_METADATA
    ),
    oid4vci_case!(
        AuthorizationServerMetadataIssuerMismatch,
        AUTHORIZATION_SERVER_METADATA_ISSUER_MISMATCH
    ),
    oid4vci_case!(
        AuthorizationEndpointTooLarge,
        AUTHORIZATION_ENDPOINT_TOO_LARGE
    ),
    oid4vci_case!(UnsafeAuthorizationEndpoint, UNSAFE_AUTHORIZATION_ENDPOINT),
    oid4vci_case!(TokenEndpointTooLarge, TOKEN_ENDPOINT_TOO_LARGE),
    oid4vci_case!(UnsafeTokenEndpoint, UNSAFE_TOKEN_ENDPOINT),
    oid4vci_case!(InvalidGrantTypes, INVALID_GRANT_TYPES),
    oid4vci_case!(GrantTypeTooLarge, GRANT_TYPE_TOO_LARGE),
    oid4vci_case!(TooManyGrantTypes, TOO_MANY_GRANT_TYPES),
    oid4vci_case!(DuplicateGrantType, DUPLICATE_GRANT_TYPE),
    oid4vci_case!(
        InvalidAnonymousPreAuthorizedAccess,
        INVALID_ANONYMOUS_PRE_AUTHORIZED_ACCESS
    ),
    oid4vci_case!(
        PreAuthorizedCodeGrantMissing,
        PRE_AUTHORIZED_CODE_GRANT_MISSING
    ),
    oid4vci_case!(
        AuthorizationServerNotAdvertised,
        AUTHORIZATION_SERVER_NOT_ADVERTISED
    ),
    oid4vci_case!(
        PreAuthorizedServerHintMismatch,
        PRE_AUTHORIZED_SERVER_HINT_MISMATCH
    ),
    oid4vci_case!(
        PreAuthorizedGrantNotSupported,
        PRE_AUTHORIZED_GRANT_NOT_SUPPORTED
    ),
    oid4vci_case!(TokenEndpointRequired, TOKEN_ENDPOINT_REQUIRED),
    oid4vci_case!(
        InvalidTransactionCodeInputLimits,
        INVALID_TRANSACTION_CODE_INPUT_LIMITS
    ),
    oid4vci_case!(
        TransactionCodeInputRequired,
        TRANSACTION_CODE_INPUT_REQUIRED
    ),
    oid4vci_case!(
        TransactionCodeInputUnexpected,
        TRANSACTION_CODE_INPUT_UNEXPECTED
    ),
    oid4vci_case!(TransactionCodeInputEmpty, TRANSACTION_CODE_INPUT_EMPTY),
    oid4vci_case!(
        TransactionCodeInputTooLarge,
        TRANSACTION_CODE_INPUT_TOO_LARGE
    ),
    oid4vci_case!(
        InvalidPreAuthorizedTokenRequestLimits,
        INVALID_PRE_AUTHORIZED_TOKEN_REQUEST_LIMITS
    ),
    oid4vci_case!(
        PreAuthorizedTokenRequestTooLarge,
        PRE_AUTHORIZED_TOKEN_REQUEST_TOO_LARGE
    ),
    oid4vci_case!(InvalidTokenResponseLimits, INVALID_TOKEN_RESPONSE_LIMITS),
    oid4vci_case!(TokenResponseTooLarge, TOKEN_RESPONSE_TOO_LARGE),
    oid4vci_case!(InvalidTokenResponse, INVALID_TOKEN_RESPONSE),
    oid4vci_case!(InvalidAccessToken, INVALID_ACCESS_TOKEN),
    oid4vci_case!(AccessTokenTooLarge, ACCESS_TOKEN_TOO_LARGE),
    oid4vci_case!(InvalidTokenType, INVALID_TOKEN_TYPE),
    oid4vci_case!(TokenTypeTooLarge, TOKEN_TYPE_TOO_LARGE),
    oid4vci_case!(InvalidTokenExpiresIn, INVALID_TOKEN_EXPIRES_IN),
    oid4vci_case!(InvalidRefreshToken, INVALID_REFRESH_TOKEN),
    oid4vci_case!(RefreshTokenTooLarge, REFRESH_TOKEN_TOO_LARGE),
    oid4vci_case!(InvalidTokenScope, INVALID_TOKEN_SCOPE),
    oid4vci_case!(TokenScopeTooLarge, TOKEN_SCOPE_TOO_LARGE),
    oid4vci_case!(
        InvalidTokenAuthorizationDetailsLimits,
        INVALID_TOKEN_AUTHORIZATION_DETAILS_LIMITS
    ),
    oid4vci_case!(
        InvalidTokenAuthorizationDetails,
        INVALID_TOKEN_AUTHORIZATION_DETAILS
    ),
    oid4vci_case!(
        TooManyTokenAuthorizationDetails,
        TOO_MANY_TOKEN_AUTHORIZATION_DETAILS
    ),
    oid4vci_case!(
        TokenAuthorizationDetailValueTooLarge,
        TOKEN_AUTHORIZATION_DETAIL_VALUE_TOO_LARGE
    ),
    oid4vci_case!(
        TooManyCredentialIdentifiers,
        TOO_MANY_CREDENTIAL_IDENTIFIERS
    ),
    oid4vci_case!(
        DuplicateCredentialIdentifier,
        DUPLICATE_CREDENTIAL_IDENTIFIER
    ),
    oid4vci_case!(
        InvalidTokenErrorResponseLimits,
        INVALID_TOKEN_ERROR_RESPONSE_LIMITS
    ),
    oid4vci_case!(TokenErrorResponseTooLarge, TOKEN_ERROR_RESPONSE_TOO_LARGE),
    oid4vci_case!(InvalidTokenErrorResponse, INVALID_TOKEN_ERROR_RESPONSE),
    oid4vci_case!(
        InvalidTokenEndpointErrorCode,
        INVALID_TOKEN_ENDPOINT_ERROR_CODE
    ),
    oid4vci_case!(
        TokenEndpointErrorCodeTooLarge,
        TOKEN_ENDPOINT_ERROR_CODE_TOO_LARGE
    ),
    oid4vci_case!(
        InvalidTokenErrorDescription,
        INVALID_TOKEN_ERROR_DESCRIPTION
    ),
    oid4vci_case!(
        TokenErrorDescriptionTooLarge,
        TOKEN_ERROR_DESCRIPTION_TOO_LARGE
    ),
    oid4vci_case!(InvalidTokenErrorUri, INVALID_TOKEN_ERROR_URI),
    oid4vci_case!(TokenErrorUriTooLarge, TOKEN_ERROR_URI_TOO_LARGE),
    oid4vci_case!(
        InvalidCredentialErrorResponseLimits,
        INVALID_CREDENTIAL_ERROR_RESPONSE_LIMITS
    ),
    oid4vci_case!(
        CredentialErrorResponseTooLarge,
        CREDENTIAL_ERROR_RESPONSE_TOO_LARGE
    ),
    oid4vci_case!(
        InvalidCredentialErrorResponse,
        INVALID_CREDENTIAL_ERROR_RESPONSE
    ),
    oid4vci_case!(
        InvalidCredentialEndpointErrorCode,
        INVALID_CREDENTIAL_ENDPOINT_ERROR_CODE
    ),
    oid4vci_case!(
        CredentialEndpointErrorCodeTooLarge,
        CREDENTIAL_ENDPOINT_ERROR_CODE_TOO_LARGE
    ),
    oid4vci_case!(
        InvalidCredentialErrorDescription,
        INVALID_CREDENTIAL_ERROR_DESCRIPTION
    ),
    oid4vci_case!(
        CredentialErrorDescriptionTooLarge,
        CREDENTIAL_ERROR_DESCRIPTION_TOO_LARGE
    ),
    oid4vci_case!(
        InvalidCredentialErrorHttpResponseLimits,
        INVALID_CREDENTIAL_ERROR_HTTP_RESPONSE_LIMITS
    ),
    oid4vci_case!(
        InvalidCredentialErrorHttpStatus,
        INVALID_CREDENTIAL_ERROR_HTTP_STATUS
    ),
    oid4vci_case!(
        CredentialErrorContentTypeTooLarge,
        CREDENTIAL_ERROR_CONTENT_TYPE_TOO_LARGE
    ),
    oid4vci_case!(
        InvalidCredentialErrorContentType,
        INVALID_CREDENTIAL_ERROR_CONTENT_TYPE
    ),
    oid4vci_case!(
        GenericCredentialErrorCodeForbidden,
        GENERIC_CREDENTIAL_ERROR_CODE_FORBIDDEN
    ),
    oid4vci_case!(
        InvalidCredentialNonceResponseLimits,
        INVALID_CREDENTIAL_NONCE_RESPONSE_LIMITS
    ),
    oid4vci_case!(
        CredentialNonceResponseTooLarge,
        CREDENTIAL_NONCE_RESPONSE_TOO_LARGE
    ),
    oid4vci_case!(
        InvalidCredentialNonceResponse,
        INVALID_CREDENTIAL_NONCE_RESPONSE
    ),
    oid4vci_case!(InvalidCredentialNonce, INVALID_CREDENTIAL_NONCE),
    oid4vci_case!(CredentialNonceTooLarge, CREDENTIAL_NONCE_TOO_LARGE),
    oid4vci_case!(
        InvalidCredentialNonceHttpResponseLimits,
        INVALID_CREDENTIAL_NONCE_HTTP_RESPONSE_LIMITS
    ),
    oid4vci_case!(
        InvalidCredentialNonceHttpStatus,
        INVALID_CREDENTIAL_NONCE_HTTP_STATUS
    ),
    oid4vci_case!(
        CredentialNonceContentTypeTooLarge,
        CREDENTIAL_NONCE_CONTENT_TYPE_TOO_LARGE
    ),
    oid4vci_case!(
        InvalidCredentialNonceContentType,
        INVALID_CREDENTIAL_NONCE_CONTENT_TYPE
    ),
    oid4vci_case!(
        CredentialNonceCacheControlTooLarge,
        CREDENTIAL_NONCE_CACHE_CONTROL_TOO_LARGE
    ),
    oid4vci_case!(
        InvalidCredentialNonceCacheControl,
        INVALID_CREDENTIAL_NONCE_CACHE_CONTROL
    ),
    oid4vci_case!(
        InvalidJwtCredentialRequestLimits,
        INVALID_JWT_CREDENTIAL_REQUEST_LIMITS
    ),
    oid4vci_case!(
        CredentialRequestConfigurationMissing,
        CREDENTIAL_REQUEST_CONFIGURATION_MISSING
    ),
    oid4vci_case!(
        CredentialRequestAuthorizationDetailMissing,
        CREDENTIAL_REQUEST_AUTHORIZATION_DETAIL_MISSING
    ),
    oid4vci_case!(
        CredentialRequestIdentifierMissing,
        CREDENTIAL_REQUEST_IDENTIFIER_MISSING
    ),
    oid4vci_case!(
        CredentialRequestAuthorizationConfigurationMismatch,
        CREDENTIAL_REQUEST_AUTHORIZATION_CONFIGURATION_MISMATCH
    ),
    oid4vci_case!(
        CredentialRequestAuthorizationDetailsUnsupported,
        CREDENTIAL_REQUEST_AUTHORIZATION_DETAILS_UNSUPPORTED
    ),
    oid4vci_case!(
        CredentialRequestTokenTypeUnsupported,
        CREDENTIAL_REQUEST_TOKEN_TYPE_UNSUPPORTED
    ),
    oid4vci_case!(
        InvalidCredentialRequestBearerToken,
        INVALID_CREDENTIAL_REQUEST_BEARER_TOKEN
    ),
    oid4vci_case!(
        CredentialRequestProofsRequired,
        CREDENTIAL_REQUEST_PROOFS_REQUIRED
    ),
    oid4vci_case!(
        TooManyCredentialRequestProofs,
        TOO_MANY_CREDENTIAL_REQUEST_PROOFS
    ),
    oid4vci_case!(
        CredentialRequestProofTooLarge,
        CREDENTIAL_REQUEST_PROOF_TOO_LARGE
    ),
    oid4vci_case!(
        CredentialRequestAuthorizationTooLarge,
        CREDENTIAL_REQUEST_AUTHORIZATION_TOO_LARGE
    ),
    oid4vci_case!(
        CredentialRequestBodyTooLarge,
        CREDENTIAL_REQUEST_BODY_TOO_LARGE
    ),
    oid4vci_case!(
        InvalidDeferredCredentialRequestLimits,
        INVALID_DEFERRED_CREDENTIAL_REQUEST_LIMITS
    ),
    oid4vci_case!(
        DeferredCredentialEndpointRequired,
        DEFERRED_CREDENTIAL_ENDPOINT_REQUIRED
    ),
    oid4vci_case!(
        DeferredCredentialRequestTooLarge,
        DEFERRED_CREDENTIAL_REQUEST_TOO_LARGE
    ),
    oid4vci_case!(
        InvalidDeferredCredentialResponseLimits,
        INVALID_DEFERRED_CREDENTIAL_RESPONSE_LIMITS
    ),
    oid4vci_case!(
        DeferredCredentialResponseTooLarge,
        DEFERRED_CREDENTIAL_RESPONSE_TOO_LARGE
    ),
    oid4vci_case!(
        InvalidDeferredCredentialResponse,
        INVALID_DEFERRED_CREDENTIAL_RESPONSE
    ),
    oid4vci_case!(
        TooManyDeferredCredentialResponseMembers,
        TOO_MANY_DEFERRED_CREDENTIAL_RESPONSE_MEMBERS
    ),
    oid4vci_case!(
        InvalidDeferredTransactionId,
        INVALID_DEFERRED_TRANSACTION_ID
    ),
    oid4vci_case!(
        DeferredTransactionIdTooLarge,
        DEFERRED_TRANSACTION_ID_TOO_LARGE
    ),
    oid4vci_case!(
        InvalidDeferredCredentialInterval,
        INVALID_DEFERRED_CREDENTIAL_INTERVAL
    ),
    oid4vci_case!(
        DeferredCredentialIntervalTooLarge,
        DEFERRED_CREDENTIAL_INTERVAL_TOO_LARGE
    ),
    oid4vci_case!(
        DeferredCredentialResponseBranchConflict,
        DEFERRED_CREDENTIAL_RESPONSE_BRANCH_CONFLICT
    ),
    oid4vci_case!(
        InvalidImmediateCredentialResponseLimits,
        INVALID_IMMEDIATE_CREDENTIAL_RESPONSE_LIMITS
    ),
    oid4vci_case!(
        ImmediateCredentialResponseTooLarge,
        IMMEDIATE_CREDENTIAL_RESPONSE_TOO_LARGE
    ),
    oid4vci_case!(
        InvalidImmediateCredentialResponse,
        INVALID_IMMEDIATE_CREDENTIAL_RESPONSE
    ),
    oid4vci_case!(
        DeferredCredentialResponseUnsupported,
        DEFERRED_CREDENTIAL_RESPONSE_UNSUPPORTED
    ),
    oid4vci_case!(
        TooManyCredentialResponseMembers,
        TOO_MANY_CREDENTIAL_RESPONSE_MEMBERS
    ),
    oid4vci_case!(TooManyIssuedCredentials, TOO_MANY_ISSUED_CREDENTIALS),
    oid4vci_case!(
        TooManyIssuedCredentialMembers,
        TOO_MANY_ISSUED_CREDENTIAL_MEMBERS
    ),
    oid4vci_case!(InvalidIssuedCredential, INVALID_ISSUED_CREDENTIAL),
    oid4vci_case!(IssuedCredentialTooLarge, ISSUED_CREDENTIAL_TOO_LARGE),
    oid4vci_case!(IssuedCredentialsTooLarge, ISSUED_CREDENTIALS_TOO_LARGE),
    oid4vci_case!(
        InvalidCredentialNotificationId,
        INVALID_CREDENTIAL_NOTIFICATION_ID
    ),
    oid4vci_case!(
        CredentialNotificationIdTooLarge,
        CREDENTIAL_NOTIFICATION_ID_TOO_LARGE
    ),
    oid4vci_case!(
        InvalidImmediateCredentialHttpResponseLimits,
        INVALID_IMMEDIATE_CREDENTIAL_HTTP_RESPONSE_LIMITS
    ),
    oid4vci_case!(
        InvalidImmediateCredentialHttpStatus,
        INVALID_IMMEDIATE_CREDENTIAL_HTTP_STATUS
    ),
    oid4vci_case!(
        ImmediateCredentialContentTypeTooLarge,
        IMMEDIATE_CREDENTIAL_CONTENT_TYPE_TOO_LARGE
    ),
    oid4vci_case!(
        InvalidImmediateCredentialContentType,
        INVALID_IMMEDIATE_CREDENTIAL_CONTENT_TYPE
    ),
    oid4vci_case!(
        CredentialResponseExceedsProofCount,
        CREDENTIAL_RESPONSE_EXCEEDS_PROOF_COUNT
    ),
];

const PREVIOUSLY_UNREFERENCED_METADATA_CASES: [CredentialOfferError; 21] = [
    CredentialOfferError::InvalidMetadataLimits,
    CredentialOfferError::CredentialEndpointTooLarge,
    CredentialOfferError::UnsafeCredentialEndpoint,
    CredentialOfferError::InvalidAuthorizationServers,
    CredentialOfferError::DuplicateAuthorizationServer,
    CredentialOfferError::InvalidCredentialConfigurations,
    CredentialOfferError::InvalidCredentialFormat,
    CredentialOfferError::CredentialFormatTooLarge,
    CredentialOfferError::InvalidAuthorizationServerMetadataLimits,
    CredentialOfferError::AuthorizationServerMetadataTooLarge,
    CredentialOfferError::InvalidAuthorizationServerMetadata,
    CredentialOfferError::AuthorizationServerMetadataIssuerMismatch,
    CredentialOfferError::AuthorizationEndpointTooLarge,
    CredentialOfferError::UnsafeAuthorizationEndpoint,
    CredentialOfferError::TokenEndpointTooLarge,
    CredentialOfferError::UnsafeTokenEndpoint,
    CredentialOfferError::InvalidGrantTypes,
    CredentialOfferError::GrantTypeTooLarge,
    CredentialOfferError::TooManyGrantTypes,
    CredentialOfferError::DuplicateGrantType,
    CredentialOfferError::InvalidAnonymousPreAuthorizedAccess,
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
fn every_oid4vci_error_matches_the_ordered_planning_golden() {
    let rows = golden_rows();
    assert_eq!(rows.len(), 171, "golden must contain exactly 171 rows");

    let mut variant_names = BTreeSet::new();
    let mut constant_names = BTreeSet::new();
    let mut stable_codes = BTreeSet::new();
    let mut invalid_input = 0;
    let mut unsupported = 0;

    for (index, (case, row)) in CASES.iter().copied().zip(rows).enumerate() {
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

        assert_eq!(row[0], "CredentialOfferError", "row {index}");
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

        match public.kind() {
            ErrorKind::InvalidInput => invalid_input += 1,
            ErrorKind::Unsupported => unsupported += 1,
            other => panic!("unexpected OID4VCI error kind {other:?} at row {index}"),
        }

        for canary in [
            "offer-private-input",
            "issuer-private-input",
            "token-private-input",
            "credential-private-input",
            "nonce-private-input",
            "parser-private-detail",
            "did:example:private#key-1",
        ] {
            assert!(!local_display.contains(canary), "row {index}");
            assert!(!public.public_message().contains(canary), "row {index}");
            assert!(!identus_display.contains(canary), "row {index}");
            assert!(!debug_variant.contains(canary), "row {index}");
        }
    }

    assert_eq!(variant_names.len(), 171);
    assert_eq!(constant_names.len(), 171);
    assert_eq!(stable_codes.len(), 171);
    assert_eq!(invalid_input, 167);
    assert_eq!(unsupported, 4);
}

#[test]
fn previously_unreferenced_metadata_contracts_are_now_explicitly_exercised() {
    assert_eq!(PREVIOUSLY_UNREFERENCED_METADATA_CASES.len(), 21);

    for error in PREVIOUSLY_UNREFERENCED_METADATA_CASES {
        let public = error.to_identus_error();
        assert_eq!(public.kind(), ErrorKind::InvalidInput);
        assert_eq!(public.capability(), Some(CAPABILITY));
        assert!(!public.code().as_str().is_empty());
        assert!(!public.public_message().is_empty());
        assert!(error.source().is_none());
        assert!(public.source().is_none());
    }
}

#[test]
fn catalogue_is_private_and_every_bridge_is_const_usable() {
    assert_eq!(CASES.len(), 171);
    assert!(
        CASES
            .iter()
            .all(|case| case.const_public == case.error.to_identus_error())
    );

    let crate_root = include_str!("../src/lib.rs");
    assert!(crate_root.contains("mod error_contract;"));
    assert!(!crate_root.contains("pub mod error_contract;"));
}
