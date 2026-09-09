//! Static, redaction-safe OID4VCI input-validation errors.

use std::fmt;

use identus_core::{CapabilityId, ErrorKind, IdentusError};

/// Owning capability for OID4VCI errors.
pub const CAPABILITY: CapabilityId = CapabilityId::new("oid4vci");

/// Stable OID4VCI error codes used at the shared SDK boundary.
pub mod error_code {
    use identus_core::ErrorCode;

    pub const INVALID_LIMITS: ErrorCode = ErrorCode::new("oid4vci.invalid_limits");
    pub const INVOCATION_TOO_LARGE: ErrorCode = ErrorCode::new("oid4vci.invocation_too_large");
    pub const INVALID_INVOCATION: ErrorCode = ErrorCode::new("oid4vci.invalid_invocation");
    pub const UNSUPPORTED_TRANSPORT: ErrorCode = ErrorCode::new("oid4vci.unsupported_transport");
    pub const INVALID_FORM_ENCODING: ErrorCode = ErrorCode::new("oid4vci.invalid_form_encoding");
    pub const EMBEDDED_TOO_LARGE: ErrorCode = ErrorCode::new("oid4vci.embedded_too_large");
    pub const INVALID_EMBEDDED_JSON: ErrorCode = ErrorCode::new("oid4vci.invalid_embedded_json");
    pub const DUPLICATE_JSON_PROPERTY: ErrorCode =
        ErrorCode::new("oid4vci.duplicate_json_property");
    pub const JSON_TOO_DEEP: ErrorCode = ErrorCode::new("oid4vci.json_too_deep");
    pub const JSON_TOO_MANY_NODES: ErrorCode = ErrorCode::new("oid4vci.json_too_many_nodes");
    pub const REFERENCE_TOO_LARGE: ErrorCode = ErrorCode::new("oid4vci.reference_too_large");
    pub const UNSAFE_REFERENCE_URI: ErrorCode = ErrorCode::new("oid4vci.unsafe_reference_uri");
    pub const INVALID_SEMANTIC_LIMITS: ErrorCode =
        ErrorCode::new("oid4vci.invalid_semantic_limits");
    pub const INVALID_OFFER_FIELDS: ErrorCode = ErrorCode::new("oid4vci.invalid_offer_fields");
    pub const ISSUER_TOO_LARGE: ErrorCode = ErrorCode::new("oid4vci.issuer_too_large");
    pub const UNSAFE_CREDENTIAL_ISSUER: ErrorCode =
        ErrorCode::new("oid4vci.unsafe_credential_issuer");
    pub const INVALID_CONFIGURATION_IDS: ErrorCode =
        ErrorCode::new("oid4vci.invalid_configuration_ids");
    pub const CONFIGURATION_ID_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.configuration_id_too_large");
    pub const TOO_MANY_CONFIGURATION_IDS: ErrorCode =
        ErrorCode::new("oid4vci.too_many_configuration_ids");
    pub const DUPLICATE_CONFIGURATION_ID: ErrorCode =
        ErrorCode::new("oid4vci.duplicate_configuration_id");
    pub const INVALID_GRANTS: ErrorCode = ErrorCode::new("oid4vci.invalid_grants");
    pub const INVALID_GRANT_LIMITS: ErrorCode = ErrorCode::new("oid4vci.invalid_grant_limits");
    pub const INVALID_AUTHORIZATION_CODE_GRANT: ErrorCode =
        ErrorCode::new("oid4vci.invalid_authorization_code_grant");
    pub const INVALID_PRE_AUTHORIZED_CODE_GRANT: ErrorCode =
        ErrorCode::new("oid4vci.invalid_pre_authorized_code_grant");
    pub const ISSUER_STATE_TOO_LARGE: ErrorCode = ErrorCode::new("oid4vci.issuer_state_too_large");
    pub const PRE_AUTHORIZED_CODE_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.pre_authorized_code_too_large");
    pub const AUTHORIZATION_SERVER_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.authorization_server_too_large");
    pub const UNSAFE_AUTHORIZATION_SERVER: ErrorCode =
        ErrorCode::new("oid4vci.unsafe_authorization_server");
    pub const INVALID_TRANSACTION_CODE: ErrorCode =
        ErrorCode::new("oid4vci.invalid_transaction_code");
    pub const INVALID_TRANSACTION_CODE_MODE: ErrorCode =
        ErrorCode::new("oid4vci.invalid_transaction_code_mode");
    pub const INVALID_TRANSACTION_CODE_LENGTH: ErrorCode =
        ErrorCode::new("oid4vci.invalid_transaction_code_length");
    pub const TRANSACTION_CODE_LENGTH_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.transaction_code_length_too_large");
    pub const TRANSACTION_CODE_DESCRIPTION_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.transaction_code_description_too_large");
    pub const INVALID_METADATA_LIMITS: ErrorCode =
        ErrorCode::new("oid4vci.invalid_metadata_limits");
    pub const METADATA_TOO_LARGE: ErrorCode = ErrorCode::new("oid4vci.metadata_too_large");
    pub const INVALID_METADATA: ErrorCode = ErrorCode::new("oid4vci.invalid_metadata");
    pub const METADATA_ISSUER_MISMATCH: ErrorCode =
        ErrorCode::new("oid4vci.metadata_issuer_mismatch");
    pub const CREDENTIAL_ENDPOINT_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.credential_endpoint_too_large");
    pub const UNSAFE_CREDENTIAL_ENDPOINT: ErrorCode =
        ErrorCode::new("oid4vci.unsafe_credential_endpoint");
    pub const NONCE_ENDPOINT_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.nonce_endpoint_too_large");
    pub const UNSAFE_NONCE_ENDPOINT: ErrorCode = ErrorCode::new("oid4vci.unsafe_nonce_endpoint");
    pub const DEFERRED_CREDENTIAL_ENDPOINT_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.deferred_credential_endpoint_too_large");
    pub const UNSAFE_DEFERRED_CREDENTIAL_ENDPOINT: ErrorCode =
        ErrorCode::new("oid4vci.unsafe_deferred_credential_endpoint");
    pub const NONCE_ENDPOINT_REQUIRED: ErrorCode =
        ErrorCode::new("oid4vci.nonce_endpoint_required");
    pub const INVALID_AUTHORIZATION_SERVERS: ErrorCode =
        ErrorCode::new("oid4vci.invalid_authorization_servers");
    pub const TOO_MANY_AUTHORIZATION_SERVERS: ErrorCode =
        ErrorCode::new("oid4vci.too_many_authorization_servers");
    pub const DUPLICATE_AUTHORIZATION_SERVER: ErrorCode =
        ErrorCode::new("oid4vci.duplicate_authorization_server");
    pub const INVALID_CREDENTIAL_CONFIGURATIONS: ErrorCode =
        ErrorCode::new("oid4vci.invalid_credential_configurations");
    pub const TOO_MANY_CREDENTIAL_CONFIGURATIONS: ErrorCode =
        ErrorCode::new("oid4vci.too_many_credential_configurations");
    pub const INVALID_CREDENTIAL_FORMAT: ErrorCode =
        ErrorCode::new("oid4vci.invalid_credential_format");
    pub const CREDENTIAL_FORMAT_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.credential_format_too_large");
    pub const OFFER_METADATA_ISSUER_MISMATCH: ErrorCode =
        ErrorCode::new("oid4vci.offer_metadata_issuer_mismatch");
    pub const OFFERED_CONFIGURATION_MISSING: ErrorCode =
        ErrorCode::new("oid4vci.offered_configuration_missing");
    pub const INVALID_AUTHORIZATION_SERVER_HINT: ErrorCode =
        ErrorCode::new("oid4vci.invalid_authorization_server_hint");
    pub const INVALID_AUTHORIZATION_SERVER_METADATA_LIMITS: ErrorCode =
        ErrorCode::new("oid4vci.invalid_authorization_server_metadata_limits");
    pub const AUTHORIZATION_SERVER_METADATA_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.authorization_server_metadata_too_large");
    pub const INVALID_AUTHORIZATION_SERVER_METADATA: ErrorCode =
        ErrorCode::new("oid4vci.invalid_authorization_server_metadata");
    pub const AUTHORIZATION_SERVER_METADATA_ISSUER_MISMATCH: ErrorCode =
        ErrorCode::new("oid4vci.authorization_server_metadata_issuer_mismatch");
    pub const AUTHORIZATION_ENDPOINT_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.authorization_endpoint_too_large");
    pub const UNSAFE_AUTHORIZATION_ENDPOINT: ErrorCode =
        ErrorCode::new("oid4vci.unsafe_authorization_endpoint");
    pub const TOKEN_ENDPOINT_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.token_endpoint_too_large");
    pub const UNSAFE_TOKEN_ENDPOINT: ErrorCode = ErrorCode::new("oid4vci.unsafe_token_endpoint");
    pub const INVALID_GRANT_TYPES: ErrorCode = ErrorCode::new("oid4vci.invalid_grant_types");
    pub const GRANT_TYPE_TOO_LARGE: ErrorCode = ErrorCode::new("oid4vci.grant_type_too_large");
    pub const TOO_MANY_GRANT_TYPES: ErrorCode = ErrorCode::new("oid4vci.too_many_grant_types");
    pub const DUPLICATE_GRANT_TYPE: ErrorCode = ErrorCode::new("oid4vci.duplicate_grant_type");
    pub const INVALID_ANONYMOUS_PRE_AUTHORIZED_ACCESS: ErrorCode =
        ErrorCode::new("oid4vci.invalid_anonymous_pre_authorized_access");
    pub const PRE_AUTHORIZED_CODE_GRANT_MISSING: ErrorCode =
        ErrorCode::new("oid4vci.pre_authorized_code_grant_missing");
    pub const AUTHORIZATION_SERVER_NOT_ADVERTISED: ErrorCode =
        ErrorCode::new("oid4vci.authorization_server_not_advertised");
    pub const PRE_AUTHORIZED_SERVER_HINT_MISMATCH: ErrorCode =
        ErrorCode::new("oid4vci.pre_authorized_server_hint_mismatch");
    pub const PRE_AUTHORIZED_GRANT_NOT_SUPPORTED: ErrorCode =
        ErrorCode::new("oid4vci.pre_authorized_grant_not_supported");
    pub const TOKEN_ENDPOINT_REQUIRED: ErrorCode =
        ErrorCode::new("oid4vci.token_endpoint_required");
    pub const INVALID_TRANSACTION_CODE_INPUT_LIMITS: ErrorCode =
        ErrorCode::new("oid4vci.invalid_transaction_code_input_limits");
    pub const TRANSACTION_CODE_INPUT_REQUIRED: ErrorCode =
        ErrorCode::new("oid4vci.transaction_code_input_required");
    pub const TRANSACTION_CODE_INPUT_UNEXPECTED: ErrorCode =
        ErrorCode::new("oid4vci.transaction_code_input_unexpected");
    pub const TRANSACTION_CODE_INPUT_EMPTY: ErrorCode =
        ErrorCode::new("oid4vci.transaction_code_input_empty");
    pub const TRANSACTION_CODE_INPUT_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.transaction_code_input_too_large");
    pub const INVALID_PRE_AUTHORIZED_TOKEN_REQUEST_LIMITS: ErrorCode =
        ErrorCode::new("oid4vci.invalid_pre_authorized_token_request_limits");
    pub const PRE_AUTHORIZED_TOKEN_REQUEST_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.pre_authorized_token_request_too_large");
    pub const INVALID_TOKEN_RESPONSE_LIMITS: ErrorCode =
        ErrorCode::new("oid4vci.invalid_token_response_limits");
    pub const TOKEN_RESPONSE_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.token_response_too_large");
    pub const INVALID_TOKEN_RESPONSE: ErrorCode = ErrorCode::new("oid4vci.invalid_token_response");
    pub const INVALID_ACCESS_TOKEN: ErrorCode = ErrorCode::new("oid4vci.invalid_access_token");
    pub const ACCESS_TOKEN_TOO_LARGE: ErrorCode = ErrorCode::new("oid4vci.access_token_too_large");
    pub const INVALID_TOKEN_TYPE: ErrorCode = ErrorCode::new("oid4vci.invalid_token_type");
    pub const TOKEN_TYPE_TOO_LARGE: ErrorCode = ErrorCode::new("oid4vci.token_type_too_large");
    pub const INVALID_TOKEN_EXPIRES_IN: ErrorCode =
        ErrorCode::new("oid4vci.invalid_token_expires_in");
    pub const INVALID_REFRESH_TOKEN: ErrorCode = ErrorCode::new("oid4vci.invalid_refresh_token");
    pub const REFRESH_TOKEN_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.refresh_token_too_large");
    pub const INVALID_TOKEN_SCOPE: ErrorCode = ErrorCode::new("oid4vci.invalid_token_scope");
    pub const TOKEN_SCOPE_TOO_LARGE: ErrorCode = ErrorCode::new("oid4vci.token_scope_too_large");
    pub const INVALID_TOKEN_AUTHORIZATION_DETAILS_LIMITS: ErrorCode =
        ErrorCode::new("oid4vci.invalid_token_authorization_details_limits");
    pub const INVALID_TOKEN_AUTHORIZATION_DETAILS: ErrorCode =
        ErrorCode::new("oid4vci.invalid_token_authorization_details");
    pub const TOO_MANY_TOKEN_AUTHORIZATION_DETAILS: ErrorCode =
        ErrorCode::new("oid4vci.too_many_token_authorization_details");
    pub const TOKEN_AUTHORIZATION_DETAIL_VALUE_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.token_authorization_detail_value_too_large");
    pub const TOO_MANY_CREDENTIAL_IDENTIFIERS: ErrorCode =
        ErrorCode::new("oid4vci.too_many_credential_identifiers");
    pub const DUPLICATE_CREDENTIAL_IDENTIFIER: ErrorCode =
        ErrorCode::new("oid4vci.duplicate_credential_identifier");
    pub const INVALID_TOKEN_ERROR_RESPONSE_LIMITS: ErrorCode =
        ErrorCode::new("oid4vci.invalid_token_error_response_limits");
    pub const TOKEN_ERROR_RESPONSE_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.token_error_response_too_large");
    pub const INVALID_TOKEN_ERROR_RESPONSE: ErrorCode =
        ErrorCode::new("oid4vci.invalid_token_error_response");
    pub const INVALID_TOKEN_ENDPOINT_ERROR_CODE: ErrorCode =
        ErrorCode::new("oid4vci.invalid_token_endpoint_error_code");
    pub const TOKEN_ENDPOINT_ERROR_CODE_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.token_endpoint_error_code_too_large");
    pub const INVALID_TOKEN_ERROR_DESCRIPTION: ErrorCode =
        ErrorCode::new("oid4vci.invalid_token_error_description");
    pub const TOKEN_ERROR_DESCRIPTION_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.token_error_description_too_large");
    pub const INVALID_TOKEN_ERROR_URI: ErrorCode =
        ErrorCode::new("oid4vci.invalid_token_error_uri");
    pub const TOKEN_ERROR_URI_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.token_error_uri_too_large");
    pub const INVALID_CREDENTIAL_ERROR_RESPONSE_LIMITS: ErrorCode =
        ErrorCode::new("oid4vci.invalid_credential_error_response_limits");
    pub const CREDENTIAL_ERROR_RESPONSE_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.credential_error_response_too_large");
    pub const INVALID_CREDENTIAL_ERROR_RESPONSE: ErrorCode =
        ErrorCode::new("oid4vci.invalid_credential_error_response");
    pub const INVALID_CREDENTIAL_ENDPOINT_ERROR_CODE: ErrorCode =
        ErrorCode::new("oid4vci.invalid_credential_endpoint_error_code");
    pub const CREDENTIAL_ENDPOINT_ERROR_CODE_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.credential_endpoint_error_code_too_large");
    pub const INVALID_CREDENTIAL_ERROR_DESCRIPTION: ErrorCode =
        ErrorCode::new("oid4vci.invalid_credential_error_description");
    pub const CREDENTIAL_ERROR_DESCRIPTION_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.credential_error_description_too_large");
    pub const INVALID_CREDENTIAL_ERROR_HTTP_RESPONSE_LIMITS: ErrorCode =
        ErrorCode::new("oid4vci.invalid_credential_error_http_response_limits");
    pub const INVALID_CREDENTIAL_ERROR_HTTP_STATUS: ErrorCode =
        ErrorCode::new("oid4vci.invalid_credential_error_http_status");
    pub const CREDENTIAL_ERROR_CONTENT_TYPE_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.credential_error_content_type_too_large");
    pub const INVALID_CREDENTIAL_ERROR_CONTENT_TYPE: ErrorCode =
        ErrorCode::new("oid4vci.invalid_credential_error_content_type");
    pub const GENERIC_CREDENTIAL_ERROR_CODE_FORBIDDEN: ErrorCode =
        ErrorCode::new("oid4vci.generic_credential_error_code_forbidden");
    pub const INVALID_CREDENTIAL_NONCE_RESPONSE_LIMITS: ErrorCode =
        ErrorCode::new("oid4vci.invalid_credential_nonce_response_limits");
    pub const CREDENTIAL_NONCE_RESPONSE_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.credential_nonce_response_too_large");
    pub const INVALID_CREDENTIAL_NONCE_RESPONSE: ErrorCode =
        ErrorCode::new("oid4vci.invalid_credential_nonce_response");
    pub const INVALID_CREDENTIAL_NONCE: ErrorCode =
        ErrorCode::new("oid4vci.invalid_credential_nonce");
    pub const CREDENTIAL_NONCE_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.credential_nonce_too_large");
    pub const INVALID_CREDENTIAL_NONCE_HTTP_RESPONSE_LIMITS: ErrorCode =
        ErrorCode::new("oid4vci.invalid_credential_nonce_http_response_limits");
    pub const INVALID_CREDENTIAL_NONCE_HTTP_STATUS: ErrorCode =
        ErrorCode::new("oid4vci.invalid_credential_nonce_http_status");
    pub const CREDENTIAL_NONCE_CONTENT_TYPE_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.credential_nonce_content_type_too_large");
    pub const INVALID_CREDENTIAL_NONCE_CONTENT_TYPE: ErrorCode =
        ErrorCode::new("oid4vci.invalid_credential_nonce_content_type");
    pub const CREDENTIAL_NONCE_CACHE_CONTROL_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.credential_nonce_cache_control_too_large");
    pub const INVALID_CREDENTIAL_NONCE_CACHE_CONTROL: ErrorCode =
        ErrorCode::new("oid4vci.invalid_credential_nonce_cache_control");
    pub const INVALID_JWT_CREDENTIAL_REQUEST_LIMITS: ErrorCode =
        ErrorCode::new("oid4vci.invalid_jwt_credential_request_limits");
    pub const CREDENTIAL_REQUEST_CONFIGURATION_MISSING: ErrorCode =
        ErrorCode::new("oid4vci.credential_request_configuration_missing");
    pub const CREDENTIAL_REQUEST_AUTHORIZATION_DETAIL_MISSING: ErrorCode =
        ErrorCode::new("oid4vci.credential_request_authorization_detail_missing");
    pub const CREDENTIAL_REQUEST_IDENTIFIER_MISSING: ErrorCode =
        ErrorCode::new("oid4vci.credential_request_identifier_missing");
    pub const CREDENTIAL_REQUEST_AUTHORIZATION_CONFIGURATION_MISMATCH: ErrorCode =
        ErrorCode::new("oid4vci.credential_request_authorization_configuration_mismatch");
    pub const CREDENTIAL_REQUEST_AUTHORIZATION_DETAILS_UNSUPPORTED: ErrorCode =
        ErrorCode::new("oid4vci.credential_request_authorization_details_unsupported");
    pub const CREDENTIAL_REQUEST_TOKEN_TYPE_UNSUPPORTED: ErrorCode =
        ErrorCode::new("oid4vci.credential_request_token_type_unsupported");
    pub const INVALID_CREDENTIAL_REQUEST_BEARER_TOKEN: ErrorCode =
        ErrorCode::new("oid4vci.invalid_credential_request_bearer_token");
    pub const CREDENTIAL_REQUEST_PROOFS_REQUIRED: ErrorCode =
        ErrorCode::new("oid4vci.credential_request_proofs_required");
    pub const TOO_MANY_CREDENTIAL_REQUEST_PROOFS: ErrorCode =
        ErrorCode::new("oid4vci.too_many_credential_request_proofs");
    pub const CREDENTIAL_REQUEST_PROOF_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.credential_request_proof_too_large");
    pub const CREDENTIAL_REQUEST_AUTHORIZATION_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.credential_request_authorization_too_large");
    pub const CREDENTIAL_REQUEST_BODY_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.credential_request_body_too_large");
    pub const INVALID_DEFERRED_CREDENTIAL_RESPONSE_LIMITS: ErrorCode =
        ErrorCode::new("oid4vci.invalid_deferred_credential_response_limits");
    pub const DEFERRED_CREDENTIAL_RESPONSE_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.deferred_credential_response_too_large");
    pub const INVALID_DEFERRED_CREDENTIAL_RESPONSE: ErrorCode =
        ErrorCode::new("oid4vci.invalid_deferred_credential_response");
    pub const TOO_MANY_DEFERRED_CREDENTIAL_RESPONSE_MEMBERS: ErrorCode =
        ErrorCode::new("oid4vci.too_many_deferred_credential_response_members");
    pub const INVALID_DEFERRED_TRANSACTION_ID: ErrorCode =
        ErrorCode::new("oid4vci.invalid_deferred_transaction_id");
    pub const DEFERRED_TRANSACTION_ID_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.deferred_transaction_id_too_large");
    pub const INVALID_DEFERRED_CREDENTIAL_INTERVAL: ErrorCode =
        ErrorCode::new("oid4vci.invalid_deferred_credential_interval");
    pub const DEFERRED_CREDENTIAL_INTERVAL_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.deferred_credential_interval_too_large");
    pub const DEFERRED_CREDENTIAL_RESPONSE_BRANCH_CONFLICT: ErrorCode =
        ErrorCode::new("oid4vci.deferred_credential_response_branch_conflict");
    pub const INVALID_IMMEDIATE_CREDENTIAL_RESPONSE_LIMITS: ErrorCode =
        ErrorCode::new("oid4vci.invalid_immediate_credential_response_limits");
    pub const IMMEDIATE_CREDENTIAL_RESPONSE_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.immediate_credential_response_too_large");
    pub const INVALID_IMMEDIATE_CREDENTIAL_RESPONSE: ErrorCode =
        ErrorCode::new("oid4vci.invalid_immediate_credential_response");
    pub const DEFERRED_CREDENTIAL_RESPONSE_UNSUPPORTED: ErrorCode =
        ErrorCode::new("oid4vci.deferred_credential_response_unsupported");
    pub const TOO_MANY_CREDENTIAL_RESPONSE_MEMBERS: ErrorCode =
        ErrorCode::new("oid4vci.too_many_credential_response_members");
    pub const TOO_MANY_ISSUED_CREDENTIALS: ErrorCode =
        ErrorCode::new("oid4vci.too_many_issued_credentials");
    pub const TOO_MANY_ISSUED_CREDENTIAL_MEMBERS: ErrorCode =
        ErrorCode::new("oid4vci.too_many_issued_credential_members");
    pub const INVALID_ISSUED_CREDENTIAL: ErrorCode =
        ErrorCode::new("oid4vci.invalid_issued_credential");
    pub const ISSUED_CREDENTIAL_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.issued_credential_too_large");
    pub const ISSUED_CREDENTIALS_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.issued_credentials_too_large");
    pub const INVALID_CREDENTIAL_NOTIFICATION_ID: ErrorCode =
        ErrorCode::new("oid4vci.invalid_credential_notification_id");
    pub const CREDENTIAL_NOTIFICATION_ID_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.credential_notification_id_too_large");
    pub const INVALID_IMMEDIATE_CREDENTIAL_HTTP_RESPONSE_LIMITS: ErrorCode =
        ErrorCode::new("oid4vci.invalid_immediate_credential_http_response_limits");
    pub const INVALID_IMMEDIATE_CREDENTIAL_HTTP_STATUS: ErrorCode =
        ErrorCode::new("oid4vci.invalid_immediate_credential_http_status");
    pub const IMMEDIATE_CREDENTIAL_CONTENT_TYPE_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.immediate_credential_content_type_too_large");
    pub const INVALID_IMMEDIATE_CREDENTIAL_CONTENT_TYPE: ErrorCode =
        ErrorCode::new("oid4vci.invalid_immediate_credential_content_type");
    pub const CREDENTIAL_RESPONSE_EXCEEDS_PROOF_COUNT: ErrorCode =
        ErrorCode::new("oid4vci.credential_response_exceeds_proof_count");
}

/// A static reason that OID4VCI validation failed.
///
/// Variants deliberately carry no input, offset, JSON, URI, or parser cause.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum CredentialOfferError {
    InvalidLimits,
    InvocationTooLarge,
    InvalidInvocation,
    UnsupportedTransport,
    InvalidFormEncoding,
    EmbeddedTooLarge,
    InvalidEmbeddedJson,
    DuplicateJsonProperty,
    JsonTooDeep,
    JsonTooManyNodes,
    ReferenceTooLarge,
    UnsafeReferenceUri,
    InvalidSemanticLimits,
    InvalidOfferFields,
    IssuerTooLarge,
    UnsafeCredentialIssuer,
    InvalidConfigurationIds,
    ConfigurationIdTooLarge,
    TooManyConfigurationIds,
    DuplicateConfigurationId,
    InvalidGrants,
    InvalidGrantLimits,
    InvalidAuthorizationCodeGrant,
    InvalidPreAuthorizedCodeGrant,
    IssuerStateTooLarge,
    PreAuthorizedCodeTooLarge,
    AuthorizationServerTooLarge,
    UnsafeAuthorizationServer,
    InvalidTransactionCode,
    InvalidTransactionCodeMode,
    InvalidTransactionCodeLength,
    TransactionCodeLengthTooLarge,
    TransactionCodeDescriptionTooLarge,
    InvalidMetadataLimits,
    MetadataTooLarge,
    InvalidMetadata,
    MetadataIssuerMismatch,
    CredentialEndpointTooLarge,
    UnsafeCredentialEndpoint,
    NonceEndpointTooLarge,
    UnsafeNonceEndpoint,
    DeferredCredentialEndpointTooLarge,
    UnsafeDeferredCredentialEndpoint,
    NonceEndpointRequired,
    InvalidAuthorizationServers,
    TooManyAuthorizationServers,
    DuplicateAuthorizationServer,
    InvalidCredentialConfigurations,
    TooManyCredentialConfigurations,
    InvalidCredentialFormat,
    CredentialFormatTooLarge,
    OfferMetadataIssuerMismatch,
    OfferedConfigurationMissing,
    InvalidAuthorizationServerHint,
    InvalidAuthorizationServerMetadataLimits,
    AuthorizationServerMetadataTooLarge,
    InvalidAuthorizationServerMetadata,
    AuthorizationServerMetadataIssuerMismatch,
    AuthorizationEndpointTooLarge,
    UnsafeAuthorizationEndpoint,
    TokenEndpointTooLarge,
    UnsafeTokenEndpoint,
    InvalidGrantTypes,
    GrantTypeTooLarge,
    TooManyGrantTypes,
    DuplicateGrantType,
    InvalidAnonymousPreAuthorizedAccess,
    PreAuthorizedCodeGrantMissing,
    AuthorizationServerNotAdvertised,
    PreAuthorizedServerHintMismatch,
    PreAuthorizedGrantNotSupported,
    TokenEndpointRequired,
    InvalidTransactionCodeInputLimits,
    TransactionCodeInputRequired,
    TransactionCodeInputUnexpected,
    TransactionCodeInputEmpty,
    TransactionCodeInputTooLarge,
    InvalidPreAuthorizedTokenRequestLimits,
    PreAuthorizedTokenRequestTooLarge,
    InvalidTokenResponseLimits,
    TokenResponseTooLarge,
    InvalidTokenResponse,
    InvalidAccessToken,
    AccessTokenTooLarge,
    InvalidTokenType,
    TokenTypeTooLarge,
    InvalidTokenExpiresIn,
    InvalidRefreshToken,
    RefreshTokenTooLarge,
    InvalidTokenScope,
    TokenScopeTooLarge,
    InvalidTokenAuthorizationDetailsLimits,
    InvalidTokenAuthorizationDetails,
    TooManyTokenAuthorizationDetails,
    TokenAuthorizationDetailValueTooLarge,
    TooManyCredentialIdentifiers,
    DuplicateCredentialIdentifier,
    InvalidTokenErrorResponseLimits,
    TokenErrorResponseTooLarge,
    InvalidTokenErrorResponse,
    InvalidTokenEndpointErrorCode,
    TokenEndpointErrorCodeTooLarge,
    InvalidTokenErrorDescription,
    TokenErrorDescriptionTooLarge,
    InvalidTokenErrorUri,
    TokenErrorUriTooLarge,
    InvalidCredentialErrorResponseLimits,
    CredentialErrorResponseTooLarge,
    InvalidCredentialErrorResponse,
    InvalidCredentialEndpointErrorCode,
    CredentialEndpointErrorCodeTooLarge,
    InvalidCredentialErrorDescription,
    CredentialErrorDescriptionTooLarge,
    InvalidCredentialErrorHttpResponseLimits,
    InvalidCredentialErrorHttpStatus,
    CredentialErrorContentTypeTooLarge,
    InvalidCredentialErrorContentType,
    GenericCredentialErrorCodeForbidden,
    InvalidCredentialNonceResponseLimits,
    CredentialNonceResponseTooLarge,
    InvalidCredentialNonceResponse,
    InvalidCredentialNonce,
    CredentialNonceTooLarge,
    InvalidCredentialNonceHttpResponseLimits,
    InvalidCredentialNonceHttpStatus,
    CredentialNonceContentTypeTooLarge,
    InvalidCredentialNonceContentType,
    CredentialNonceCacheControlTooLarge,
    InvalidCredentialNonceCacheControl,
    InvalidJwtCredentialRequestLimits,
    CredentialRequestConfigurationMissing,
    CredentialRequestAuthorizationDetailMissing,
    CredentialRequestIdentifierMissing,
    CredentialRequestAuthorizationConfigurationMismatch,
    CredentialRequestAuthorizationDetailsUnsupported,
    CredentialRequestTokenTypeUnsupported,
    InvalidCredentialRequestBearerToken,
    CredentialRequestProofsRequired,
    TooManyCredentialRequestProofs,
    CredentialRequestProofTooLarge,
    CredentialRequestAuthorizationTooLarge,
    CredentialRequestBodyTooLarge,
    InvalidDeferredCredentialResponseLimits,
    DeferredCredentialResponseTooLarge,
    InvalidDeferredCredentialResponse,
    TooManyDeferredCredentialResponseMembers,
    InvalidDeferredTransactionId,
    DeferredTransactionIdTooLarge,
    InvalidDeferredCredentialInterval,
    DeferredCredentialIntervalTooLarge,
    DeferredCredentialResponseBranchConflict,
    InvalidImmediateCredentialResponseLimits,
    ImmediateCredentialResponseTooLarge,
    InvalidImmediateCredentialResponse,
    DeferredCredentialResponseUnsupported,
    TooManyCredentialResponseMembers,
    TooManyIssuedCredentials,
    TooManyIssuedCredentialMembers,
    InvalidIssuedCredential,
    IssuedCredentialTooLarge,
    IssuedCredentialsTooLarge,
    InvalidCredentialNotificationId,
    CredentialNotificationIdTooLarge,
    InvalidImmediateCredentialHttpResponseLimits,
    InvalidImmediateCredentialHttpStatus,
    ImmediateCredentialContentTypeTooLarge,
    InvalidImmediateCredentialContentType,
    CredentialResponseExceedsProofCount,
}

impl CredentialOfferError {
    /// Convert to the workspace-wide redaction-safe error contract.
    pub const fn to_identus_error(self) -> IdentusError {
        let (code, kind, message) = match self {
            Self::InvalidLimits => (
                error_code::INVALID_LIMITS,
                ErrorKind::InvalidInput,
                "OID4VCI limits are invalid",
            ),
            Self::InvocationTooLarge => (
                error_code::INVOCATION_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Offer invocation is too large",
            ),
            Self::InvalidInvocation => (
                error_code::INVALID_INVOCATION,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Offer invocation is invalid",
            ),
            Self::UnsupportedTransport => (
                error_code::UNSUPPORTED_TRANSPORT,
                ErrorKind::Unsupported,
                "OID4VCI Credential Offer transport is unsupported",
            ),
            Self::InvalidFormEncoding => (
                error_code::INVALID_FORM_ENCODING,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Offer encoding is invalid",
            ),
            Self::EmbeddedTooLarge => (
                error_code::EMBEDDED_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI embedded Credential Offer is too large",
            ),
            Self::InvalidEmbeddedJson => (
                error_code::INVALID_EMBEDDED_JSON,
                ErrorKind::InvalidInput,
                "OID4VCI JSON object is invalid",
            ),
            Self::DuplicateJsonProperty => (
                error_code::DUPLICATE_JSON_PROPERTY,
                ErrorKind::InvalidInput,
                "OID4VCI JSON object repeats a member",
            ),
            Self::JsonTooDeep => (
                error_code::JSON_TOO_DEEP,
                ErrorKind::InvalidInput,
                "OID4VCI JSON object is too deep",
            ),
            Self::JsonTooManyNodes => (
                error_code::JSON_TOO_MANY_NODES,
                ErrorKind::InvalidInput,
                "OID4VCI JSON object has too many nodes",
            ),
            Self::ReferenceTooLarge => (
                error_code::REFERENCE_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Offer reference is too large",
            ),
            Self::UnsafeReferenceUri => (
                error_code::UNSAFE_REFERENCE_URI,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Offer reference URI is unsafe",
            ),
            Self::InvalidSemanticLimits => (
                error_code::INVALID_SEMANTIC_LIMITS,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Offer semantic limits are invalid",
            ),
            Self::InvalidOfferFields => (
                error_code::INVALID_OFFER_FIELDS,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Offer required fields are invalid",
            ),
            Self::IssuerTooLarge => (
                error_code::ISSUER_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Issuer Identifier is too large",
            ),
            Self::UnsafeCredentialIssuer => (
                error_code::UNSAFE_CREDENTIAL_ISSUER,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Issuer Identifier is unsafe",
            ),
            Self::InvalidConfigurationIds => (
                error_code::INVALID_CONFIGURATION_IDS,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Configuration IDs are invalid",
            ),
            Self::ConfigurationIdTooLarge => (
                error_code::CONFIGURATION_ID_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Configuration ID is too large",
            ),
            Self::TooManyConfigurationIds => (
                error_code::TOO_MANY_CONFIGURATION_IDS,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Offer has too many configuration IDs",
            ),
            Self::DuplicateConfigurationId => (
                error_code::DUPLICATE_CONFIGURATION_ID,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Offer repeats a configuration ID",
            ),
            Self::InvalidGrants => (
                error_code::INVALID_GRANTS,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Offer grants are invalid",
            ),
            Self::InvalidGrantLimits => (
                error_code::INVALID_GRANT_LIMITS,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Offer grant limits are invalid",
            ),
            Self::InvalidAuthorizationCodeGrant => (
                error_code::INVALID_AUTHORIZATION_CODE_GRANT,
                ErrorKind::InvalidInput,
                "OID4VCI Authorization Code grant is invalid",
            ),
            Self::InvalidPreAuthorizedCodeGrant => (
                error_code::INVALID_PRE_AUTHORIZED_CODE_GRANT,
                ErrorKind::InvalidInput,
                "OID4VCI Pre-Authorized Code grant is invalid",
            ),
            Self::IssuerStateTooLarge => (
                error_code::ISSUER_STATE_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI issuer state is too large",
            ),
            Self::PreAuthorizedCodeTooLarge => (
                error_code::PRE_AUTHORIZED_CODE_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Pre-Authorized Code is too large",
            ),
            Self::AuthorizationServerTooLarge => (
                error_code::AUTHORIZATION_SERVER_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Authorization Server identifier is too large",
            ),
            Self::UnsafeAuthorizationServer => (
                error_code::UNSAFE_AUTHORIZATION_SERVER,
                ErrorKind::InvalidInput,
                "OID4VCI Authorization Server identifier is unsafe",
            ),
            Self::InvalidTransactionCode => (
                error_code::INVALID_TRANSACTION_CODE,
                ErrorKind::InvalidInput,
                "OID4VCI Transaction Code requirements are invalid",
            ),
            Self::InvalidTransactionCodeMode => (
                error_code::INVALID_TRANSACTION_CODE_MODE,
                ErrorKind::InvalidInput,
                "OID4VCI Transaction Code input mode is invalid",
            ),
            Self::InvalidTransactionCodeLength => (
                error_code::INVALID_TRANSACTION_CODE_LENGTH,
                ErrorKind::InvalidInput,
                "OID4VCI Transaction Code length is invalid",
            ),
            Self::TransactionCodeLengthTooLarge => (
                error_code::TRANSACTION_CODE_LENGTH_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Transaction Code length is too large",
            ),
            Self::TransactionCodeDescriptionTooLarge => (
                error_code::TRANSACTION_CODE_DESCRIPTION_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Transaction Code description is too large",
            ),
            Self::InvalidMetadataLimits => (
                error_code::INVALID_METADATA_LIMITS,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Issuer Metadata limits are invalid",
            ),
            Self::MetadataTooLarge => (
                error_code::METADATA_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Issuer Metadata is too large",
            ),
            Self::InvalidMetadata => (
                error_code::INVALID_METADATA,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Issuer Metadata is invalid",
            ),
            Self::MetadataIssuerMismatch => (
                error_code::METADATA_ISSUER_MISMATCH,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Issuer Metadata identifier does not match",
            ),
            Self::CredentialEndpointTooLarge => (
                error_code::CREDENTIAL_ENDPOINT_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Endpoint is too large",
            ),
            Self::UnsafeCredentialEndpoint => (
                error_code::UNSAFE_CREDENTIAL_ENDPOINT,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Endpoint is unsafe",
            ),
            Self::NonceEndpointTooLarge => (
                error_code::NONCE_ENDPOINT_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Nonce Endpoint is too large",
            ),
            Self::UnsafeNonceEndpoint => (
                error_code::UNSAFE_NONCE_ENDPOINT,
                ErrorKind::InvalidInput,
                "OID4VCI Nonce Endpoint is unsafe",
            ),
            Self::DeferredCredentialEndpointTooLarge => (
                error_code::DEFERRED_CREDENTIAL_ENDPOINT_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Deferred Credential Endpoint is too large",
            ),
            Self::UnsafeDeferredCredentialEndpoint => (
                error_code::UNSAFE_DEFERRED_CREDENTIAL_ENDPOINT,
                ErrorKind::InvalidInput,
                "OID4VCI Deferred Credential Endpoint is unsafe",
            ),
            Self::NonceEndpointRequired => (
                error_code::NONCE_ENDPOINT_REQUIRED,
                ErrorKind::InvalidInput,
                "OID4VCI Nonce Endpoint is required",
            ),
            Self::InvalidAuthorizationServers => (
                error_code::INVALID_AUTHORIZATION_SERVERS,
                ErrorKind::InvalidInput,
                "OID4VCI Authorization Server metadata is invalid",
            ),
            Self::TooManyAuthorizationServers => (
                error_code::TOO_MANY_AUTHORIZATION_SERVERS,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Issuer Metadata has too many Authorization Servers",
            ),
            Self::DuplicateAuthorizationServer => (
                error_code::DUPLICATE_AUTHORIZATION_SERVER,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Issuer Metadata repeats an Authorization Server",
            ),
            Self::InvalidCredentialConfigurations => (
                error_code::INVALID_CREDENTIAL_CONFIGURATIONS,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Configurations are invalid",
            ),
            Self::TooManyCredentialConfigurations => (
                error_code::TOO_MANY_CREDENTIAL_CONFIGURATIONS,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Issuer Metadata has too many configurations",
            ),
            Self::InvalidCredentialFormat => (
                error_code::INVALID_CREDENTIAL_FORMAT,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Format identifier is invalid",
            ),
            Self::CredentialFormatTooLarge => (
                error_code::CREDENTIAL_FORMAT_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Format identifier is too large",
            ),
            Self::OfferMetadataIssuerMismatch => (
                error_code::OFFER_METADATA_ISSUER_MISMATCH,
                ErrorKind::InvalidInput,
                "OID4VCI offer and metadata issuer identifiers do not match",
            ),
            Self::OfferedConfigurationMissing => (
                error_code::OFFERED_CONFIGURATION_MISSING,
                ErrorKind::InvalidInput,
                "OID4VCI offered Credential Configuration is missing from metadata",
            ),
            Self::InvalidAuthorizationServerHint => (
                error_code::INVALID_AUTHORIZATION_SERVER_HINT,
                ErrorKind::InvalidInput,
                "OID4VCI offered Authorization Server hint is invalid for metadata",
            ),
            Self::InvalidAuthorizationServerMetadataLimits => (
                error_code::INVALID_AUTHORIZATION_SERVER_METADATA_LIMITS,
                ErrorKind::InvalidInput,
                "OID4VCI Authorization Server Metadata limits are invalid",
            ),
            Self::AuthorizationServerMetadataTooLarge => (
                error_code::AUTHORIZATION_SERVER_METADATA_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Authorization Server Metadata is too large",
            ),
            Self::InvalidAuthorizationServerMetadata => (
                error_code::INVALID_AUTHORIZATION_SERVER_METADATA,
                ErrorKind::InvalidInput,
                "OID4VCI Authorization Server Metadata core is invalid",
            ),
            Self::AuthorizationServerMetadataIssuerMismatch => (
                error_code::AUTHORIZATION_SERVER_METADATA_ISSUER_MISMATCH,
                ErrorKind::InvalidInput,
                "OID4VCI Authorization Server Metadata issuer does not match",
            ),
            Self::AuthorizationEndpointTooLarge => (
                error_code::AUTHORIZATION_ENDPOINT_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Authorization Endpoint is too large",
            ),
            Self::UnsafeAuthorizationEndpoint => (
                error_code::UNSAFE_AUTHORIZATION_ENDPOINT,
                ErrorKind::InvalidInput,
                "OID4VCI Authorization Endpoint is unsafe",
            ),
            Self::TokenEndpointTooLarge => (
                error_code::TOKEN_ENDPOINT_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Token Endpoint is too large",
            ),
            Self::UnsafeTokenEndpoint => (
                error_code::UNSAFE_TOKEN_ENDPOINT,
                ErrorKind::InvalidInput,
                "OID4VCI Token Endpoint is unsafe",
            ),
            Self::InvalidGrantTypes => (
                error_code::INVALID_GRANT_TYPES,
                ErrorKind::InvalidInput,
                "OID4VCI Authorization Server grant types are invalid",
            ),
            Self::GrantTypeTooLarge => (
                error_code::GRANT_TYPE_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Authorization Server grant type is too large",
            ),
            Self::TooManyGrantTypes => (
                error_code::TOO_MANY_GRANT_TYPES,
                ErrorKind::InvalidInput,
                "OID4VCI Authorization Server Metadata has too many grant types",
            ),
            Self::DuplicateGrantType => (
                error_code::DUPLICATE_GRANT_TYPE,
                ErrorKind::InvalidInput,
                "OID4VCI Authorization Server Metadata repeats a grant type",
            ),
            Self::InvalidAnonymousPreAuthorizedAccess => (
                error_code::INVALID_ANONYMOUS_PRE_AUTHORIZED_ACCESS,
                ErrorKind::InvalidInput,
                "OID4VCI anonymous Pre-Authorized Code metadata is invalid",
            ),
            Self::PreAuthorizedCodeGrantMissing => (
                error_code::PRE_AUTHORIZED_CODE_GRANT_MISSING,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Offer has no Pre-Authorized Code grant",
            ),
            Self::AuthorizationServerNotAdvertised => (
                error_code::AUTHORIZATION_SERVER_NOT_ADVERTISED,
                ErrorKind::InvalidInput,
                "OID4VCI selected Authorization Server is not advertised",
            ),
            Self::PreAuthorizedServerHintMismatch => (
                error_code::PRE_AUTHORIZED_SERVER_HINT_MISMATCH,
                ErrorKind::InvalidInput,
                "OID4VCI selected Authorization Server does not match the offered hint",
            ),
            Self::PreAuthorizedGrantNotSupported => (
                error_code::PRE_AUTHORIZED_GRANT_NOT_SUPPORTED,
                ErrorKind::InvalidInput,
                "OID4VCI selected Authorization Server does not support the Pre-Authorized Code grant",
            ),
            Self::TokenEndpointRequired => (
                error_code::TOKEN_ENDPOINT_REQUIRED,
                ErrorKind::InvalidInput,
                "OID4VCI selected Authorization Server has no Token Endpoint",
            ),
            Self::InvalidTransactionCodeInputLimits => (
                error_code::INVALID_TRANSACTION_CODE_INPUT_LIMITS,
                ErrorKind::InvalidInput,
                "OID4VCI Transaction Code input limits are invalid",
            ),
            Self::TransactionCodeInputRequired => (
                error_code::TRANSACTION_CODE_INPUT_REQUIRED,
                ErrorKind::InvalidInput,
                "OID4VCI Transaction Code input is required",
            ),
            Self::TransactionCodeInputUnexpected => (
                error_code::TRANSACTION_CODE_INPUT_UNEXPECTED,
                ErrorKind::InvalidInput,
                "OID4VCI Transaction Code input is unexpected",
            ),
            Self::TransactionCodeInputEmpty => (
                error_code::TRANSACTION_CODE_INPUT_EMPTY,
                ErrorKind::InvalidInput,
                "OID4VCI Transaction Code input is empty",
            ),
            Self::TransactionCodeInputTooLarge => (
                error_code::TRANSACTION_CODE_INPUT_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Transaction Code input is too large",
            ),
            Self::InvalidPreAuthorizedTokenRequestLimits => (
                error_code::INVALID_PRE_AUTHORIZED_TOKEN_REQUEST_LIMITS,
                ErrorKind::InvalidInput,
                "OID4VCI Pre-Authorized Token Request limits are invalid",
            ),
            Self::PreAuthorizedTokenRequestTooLarge => (
                error_code::PRE_AUTHORIZED_TOKEN_REQUEST_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Pre-Authorized Token Request is too large",
            ),
            Self::InvalidTokenResponseLimits => (
                error_code::INVALID_TOKEN_RESPONSE_LIMITS,
                ErrorKind::InvalidInput,
                "OID4VCI Token Response limits are invalid",
            ),
            Self::TokenResponseTooLarge => (
                error_code::TOKEN_RESPONSE_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Token Response is too large",
            ),
            Self::InvalidTokenResponse => (
                error_code::INVALID_TOKEN_RESPONSE,
                ErrorKind::InvalidInput,
                "OID4VCI Token Response core is invalid",
            ),
            Self::InvalidAccessToken => (
                error_code::INVALID_ACCESS_TOKEN,
                ErrorKind::InvalidInput,
                "OID4VCI access token is invalid",
            ),
            Self::AccessTokenTooLarge => (
                error_code::ACCESS_TOKEN_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI access token is too large",
            ),
            Self::InvalidTokenType => (
                error_code::INVALID_TOKEN_TYPE,
                ErrorKind::InvalidInput,
                "OID4VCI token type is invalid",
            ),
            Self::TokenTypeTooLarge => (
                error_code::TOKEN_TYPE_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI token type is too large",
            ),
            Self::InvalidTokenExpiresIn => (
                error_code::INVALID_TOKEN_EXPIRES_IN,
                ErrorKind::InvalidInput,
                "OID4VCI token expiry is invalid",
            ),
            Self::InvalidRefreshToken => (
                error_code::INVALID_REFRESH_TOKEN,
                ErrorKind::InvalidInput,
                "OID4VCI refresh token is invalid",
            ),
            Self::RefreshTokenTooLarge => (
                error_code::REFRESH_TOKEN_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI refresh token is too large",
            ),
            Self::InvalidTokenScope => (
                error_code::INVALID_TOKEN_SCOPE,
                ErrorKind::InvalidInput,
                "OID4VCI token scope is invalid",
            ),
            Self::TokenScopeTooLarge => (
                error_code::TOKEN_SCOPE_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI token scope is too large",
            ),
            Self::InvalidTokenAuthorizationDetailsLimits => (
                error_code::INVALID_TOKEN_AUTHORIZATION_DETAILS_LIMITS,
                ErrorKind::InvalidInput,
                "OID4VCI Token Response Authorization Details limits are invalid",
            ),
            Self::InvalidTokenAuthorizationDetails => (
                error_code::INVALID_TOKEN_AUTHORIZATION_DETAILS,
                ErrorKind::InvalidInput,
                "OID4VCI Token Response Authorization Details are invalid",
            ),
            Self::TooManyTokenAuthorizationDetails => (
                error_code::TOO_MANY_TOKEN_AUTHORIZATION_DETAILS,
                ErrorKind::InvalidInput,
                "OID4VCI Token Response has too many Authorization Details",
            ),
            Self::TokenAuthorizationDetailValueTooLarge => (
                error_code::TOKEN_AUTHORIZATION_DETAIL_VALUE_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Token Response Authorization Details value is too large",
            ),
            Self::TooManyCredentialIdentifiers => (
                error_code::TOO_MANY_CREDENTIAL_IDENTIFIERS,
                ErrorKind::InvalidInput,
                "OID4VCI Token Response has too many Credential identifiers",
            ),
            Self::DuplicateCredentialIdentifier => (
                error_code::DUPLICATE_CREDENTIAL_IDENTIFIER,
                ErrorKind::InvalidInput,
                "OID4VCI Token Response has a duplicate Credential identifier",
            ),
            Self::InvalidTokenErrorResponseLimits => (
                error_code::INVALID_TOKEN_ERROR_RESPONSE_LIMITS,
                ErrorKind::InvalidInput,
                "OID4VCI Token Error Response limits are invalid",
            ),
            Self::TokenErrorResponseTooLarge => (
                error_code::TOKEN_ERROR_RESPONSE_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Token Error Response is too large",
            ),
            Self::InvalidTokenErrorResponse => (
                error_code::INVALID_TOKEN_ERROR_RESPONSE,
                ErrorKind::InvalidInput,
                "OID4VCI Token Error Response core is invalid",
            ),
            Self::InvalidTokenEndpointErrorCode => (
                error_code::INVALID_TOKEN_ENDPOINT_ERROR_CODE,
                ErrorKind::InvalidInput,
                "OID4VCI token endpoint error code is invalid",
            ),
            Self::TokenEndpointErrorCodeTooLarge => (
                error_code::TOKEN_ENDPOINT_ERROR_CODE_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI token endpoint error code is too large",
            ),
            Self::InvalidTokenErrorDescription => (
                error_code::INVALID_TOKEN_ERROR_DESCRIPTION,
                ErrorKind::InvalidInput,
                "OID4VCI Token Error Response description is invalid",
            ),
            Self::TokenErrorDescriptionTooLarge => (
                error_code::TOKEN_ERROR_DESCRIPTION_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Token Error Response description is too large",
            ),
            Self::InvalidTokenErrorUri => (
                error_code::INVALID_TOKEN_ERROR_URI,
                ErrorKind::InvalidInput,
                "OID4VCI Token Error Response URI is invalid",
            ),
            Self::TokenErrorUriTooLarge => (
                error_code::TOKEN_ERROR_URI_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Token Error Response URI is too large",
            ),
            Self::InvalidCredentialErrorResponseLimits => (
                error_code::INVALID_CREDENTIAL_ERROR_RESPONSE_LIMITS,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Error Response limits are invalid",
            ),
            Self::CredentialErrorResponseTooLarge => (
                error_code::CREDENTIAL_ERROR_RESPONSE_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Error Response is too large",
            ),
            Self::InvalidCredentialErrorResponse => (
                error_code::INVALID_CREDENTIAL_ERROR_RESPONSE,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Error Response core is invalid",
            ),
            Self::InvalidCredentialEndpointErrorCode => (
                error_code::INVALID_CREDENTIAL_ENDPOINT_ERROR_CODE,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Endpoint error code is invalid",
            ),
            Self::CredentialEndpointErrorCodeTooLarge => (
                error_code::CREDENTIAL_ENDPOINT_ERROR_CODE_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Endpoint error code is too large",
            ),
            Self::InvalidCredentialErrorDescription => (
                error_code::INVALID_CREDENTIAL_ERROR_DESCRIPTION,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Error Response description is invalid",
            ),
            Self::CredentialErrorDescriptionTooLarge => (
                error_code::CREDENTIAL_ERROR_DESCRIPTION_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Error Response description is too large",
            ),
            Self::InvalidCredentialErrorHttpResponseLimits => (
                error_code::INVALID_CREDENTIAL_ERROR_HTTP_RESPONSE_LIMITS,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Error HTTP response limits are invalid",
            ),
            Self::InvalidCredentialErrorHttpStatus => (
                error_code::INVALID_CREDENTIAL_ERROR_HTTP_STATUS,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Error HTTP status is invalid",
            ),
            Self::CredentialErrorContentTypeTooLarge => (
                error_code::CREDENTIAL_ERROR_CONTENT_TYPE_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Error Content-Type is too large",
            ),
            Self::InvalidCredentialErrorContentType => (
                error_code::INVALID_CREDENTIAL_ERROR_CONTENT_TYPE,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Error Content-Type is invalid",
            ),
            Self::GenericCredentialErrorCodeForbidden => (
                error_code::GENERIC_CREDENTIAL_ERROR_CODE_FORBIDDEN,
                ErrorKind::InvalidInput,
                "OID4VCI Credential payload error uses a forbidden generic code",
            ),
            Self::InvalidCredentialNonceResponseLimits => (
                error_code::INVALID_CREDENTIAL_NONCE_RESPONSE_LIMITS,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Nonce Response limits are invalid",
            ),
            Self::CredentialNonceResponseTooLarge => (
                error_code::CREDENTIAL_NONCE_RESPONSE_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Nonce Response is too large",
            ),
            Self::InvalidCredentialNonceResponse => (
                error_code::INVALID_CREDENTIAL_NONCE_RESPONSE,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Nonce Response core is invalid",
            ),
            Self::InvalidCredentialNonce => (
                error_code::INVALID_CREDENTIAL_NONCE,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Nonce is invalid",
            ),
            Self::CredentialNonceTooLarge => (
                error_code::CREDENTIAL_NONCE_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Nonce is too large",
            ),
            Self::InvalidCredentialNonceHttpResponseLimits => (
                error_code::INVALID_CREDENTIAL_NONCE_HTTP_RESPONSE_LIMITS,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Nonce HTTP response limits are invalid",
            ),
            Self::InvalidCredentialNonceHttpStatus => (
                error_code::INVALID_CREDENTIAL_NONCE_HTTP_STATUS,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Nonce HTTP status is invalid",
            ),
            Self::CredentialNonceContentTypeTooLarge => (
                error_code::CREDENTIAL_NONCE_CONTENT_TYPE_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Nonce Content-Type is too large",
            ),
            Self::InvalidCredentialNonceContentType => (
                error_code::INVALID_CREDENTIAL_NONCE_CONTENT_TYPE,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Nonce Content-Type is invalid",
            ),
            Self::CredentialNonceCacheControlTooLarge => (
                error_code::CREDENTIAL_NONCE_CACHE_CONTROL_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Nonce Cache-Control is too large",
            ),
            Self::InvalidCredentialNonceCacheControl => (
                error_code::INVALID_CREDENTIAL_NONCE_CACHE_CONTROL,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Nonce Cache-Control is invalid",
            ),
            Self::InvalidJwtCredentialRequestLimits => (
                error_code::INVALID_JWT_CREDENTIAL_REQUEST_LIMITS,
                ErrorKind::InvalidInput,
                "OID4VCI JWT Credential Request limits are invalid",
            ),
            Self::CredentialRequestConfigurationMissing => (
                error_code::CREDENTIAL_REQUEST_CONFIGURATION_MISSING,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Request configuration is not offered",
            ),
            Self::CredentialRequestAuthorizationDetailMissing => (
                error_code::CREDENTIAL_REQUEST_AUTHORIZATION_DETAIL_MISSING,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Request Authorization Detail is missing",
            ),
            Self::CredentialRequestIdentifierMissing => (
                error_code::CREDENTIAL_REQUEST_IDENTIFIER_MISSING,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Request identifier is missing",
            ),
            Self::CredentialRequestAuthorizationConfigurationMismatch => (
                error_code::CREDENTIAL_REQUEST_AUTHORIZATION_CONFIGURATION_MISMATCH,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Request authorization configuration is not offered",
            ),
            Self::CredentialRequestAuthorizationDetailsUnsupported => (
                error_code::CREDENTIAL_REQUEST_AUTHORIZATION_DETAILS_UNSUPPORTED,
                ErrorKind::Unsupported,
                "OID4VCI Credential Request Authorization Details are unsupported",
            ),
            Self::CredentialRequestTokenTypeUnsupported => (
                error_code::CREDENTIAL_REQUEST_TOKEN_TYPE_UNSUPPORTED,
                ErrorKind::Unsupported,
                "OID4VCI Credential Request token type is unsupported",
            ),
            Self::InvalidCredentialRequestBearerToken => (
                error_code::INVALID_CREDENTIAL_REQUEST_BEARER_TOKEN,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Request Bearer token is invalid",
            ),
            Self::CredentialRequestProofsRequired => (
                error_code::CREDENTIAL_REQUEST_PROOFS_REQUIRED,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Request requires JWT proofs",
            ),
            Self::TooManyCredentialRequestProofs => (
                error_code::TOO_MANY_CREDENTIAL_REQUEST_PROOFS,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Request has too many JWT proofs",
            ),
            Self::CredentialRequestProofTooLarge => (
                error_code::CREDENTIAL_REQUEST_PROOF_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Request JWT proof is too large",
            ),
            Self::CredentialRequestAuthorizationTooLarge => (
                error_code::CREDENTIAL_REQUEST_AUTHORIZATION_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Request Authorization value is too large",
            ),
            Self::CredentialRequestBodyTooLarge => (
                error_code::CREDENTIAL_REQUEST_BODY_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Request body is too large",
            ),
            Self::InvalidDeferredCredentialResponseLimits => (
                error_code::INVALID_DEFERRED_CREDENTIAL_RESPONSE_LIMITS,
                ErrorKind::InvalidInput,
                "OID4VCI deferred Credential Response limits are invalid",
            ),
            Self::DeferredCredentialResponseTooLarge => (
                error_code::DEFERRED_CREDENTIAL_RESPONSE_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI deferred Credential Response is too large",
            ),
            Self::InvalidDeferredCredentialResponse => (
                error_code::INVALID_DEFERRED_CREDENTIAL_RESPONSE,
                ErrorKind::InvalidInput,
                "OID4VCI deferred Credential Response core is invalid",
            ),
            Self::TooManyDeferredCredentialResponseMembers => (
                error_code::TOO_MANY_DEFERRED_CREDENTIAL_RESPONSE_MEMBERS,
                ErrorKind::InvalidInput,
                "OID4VCI deferred Credential Response has too many members",
            ),
            Self::InvalidDeferredTransactionId => (
                error_code::INVALID_DEFERRED_TRANSACTION_ID,
                ErrorKind::InvalidInput,
                "OID4VCI deferred transaction identifier is invalid",
            ),
            Self::DeferredTransactionIdTooLarge => (
                error_code::DEFERRED_TRANSACTION_ID_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI deferred transaction identifier is too large",
            ),
            Self::InvalidDeferredCredentialInterval => (
                error_code::INVALID_DEFERRED_CREDENTIAL_INTERVAL,
                ErrorKind::InvalidInput,
                "OID4VCI deferred Credential interval is invalid",
            ),
            Self::DeferredCredentialIntervalTooLarge => (
                error_code::DEFERRED_CREDENTIAL_INTERVAL_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI deferred Credential interval is too large",
            ),
            Self::DeferredCredentialResponseBranchConflict => (
                error_code::DEFERRED_CREDENTIAL_RESPONSE_BRANCH_CONFLICT,
                ErrorKind::InvalidInput,
                "OID4VCI deferred Credential Response branch is ambiguous",
            ),
            Self::InvalidImmediateCredentialResponseLimits => (
                error_code::INVALID_IMMEDIATE_CREDENTIAL_RESPONSE_LIMITS,
                ErrorKind::InvalidInput,
                "OID4VCI immediate Credential Response limits are invalid",
            ),
            Self::ImmediateCredentialResponseTooLarge => (
                error_code::IMMEDIATE_CREDENTIAL_RESPONSE_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI immediate Credential Response is too large",
            ),
            Self::InvalidImmediateCredentialResponse => (
                error_code::INVALID_IMMEDIATE_CREDENTIAL_RESPONSE,
                ErrorKind::InvalidInput,
                "OID4VCI immediate Credential Response core is invalid",
            ),
            Self::DeferredCredentialResponseUnsupported => (
                error_code::DEFERRED_CREDENTIAL_RESPONSE_UNSUPPORTED,
                ErrorKind::Unsupported,
                "OID4VCI deferred Credential Response is unsupported",
            ),
            Self::TooManyCredentialResponseMembers => (
                error_code::TOO_MANY_CREDENTIAL_RESPONSE_MEMBERS,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Response has too many members",
            ),
            Self::TooManyIssuedCredentials => (
                error_code::TOO_MANY_ISSUED_CREDENTIALS,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Response has too many credentials",
            ),
            Self::TooManyIssuedCredentialMembers => (
                error_code::TOO_MANY_ISSUED_CREDENTIAL_MEMBERS,
                ErrorKind::InvalidInput,
                "OID4VCI issued credential has too many members",
            ),
            Self::InvalidIssuedCredential => (
                error_code::INVALID_ISSUED_CREDENTIAL,
                ErrorKind::InvalidInput,
                "OID4VCI issued credential is invalid",
            ),
            Self::IssuedCredentialTooLarge => (
                error_code::ISSUED_CREDENTIAL_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI issued credential is too large",
            ),
            Self::IssuedCredentialsTooLarge => (
                error_code::ISSUED_CREDENTIALS_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI issued credentials are too large",
            ),
            Self::InvalidCredentialNotificationId => (
                error_code::INVALID_CREDENTIAL_NOTIFICATION_ID,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Response notification identifier is invalid",
            ),
            Self::CredentialNotificationIdTooLarge => (
                error_code::CREDENTIAL_NOTIFICATION_ID_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Response notification identifier is too large",
            ),
            Self::InvalidImmediateCredentialHttpResponseLimits => (
                error_code::INVALID_IMMEDIATE_CREDENTIAL_HTTP_RESPONSE_LIMITS,
                ErrorKind::InvalidInput,
                "OID4VCI immediate Credential HTTP response limits are invalid",
            ),
            Self::InvalidImmediateCredentialHttpStatus => (
                error_code::INVALID_IMMEDIATE_CREDENTIAL_HTTP_STATUS,
                ErrorKind::InvalidInput,
                "OID4VCI immediate Credential HTTP status is invalid",
            ),
            Self::ImmediateCredentialContentTypeTooLarge => (
                error_code::IMMEDIATE_CREDENTIAL_CONTENT_TYPE_TOO_LARGE,
                ErrorKind::InvalidInput,
                "OID4VCI immediate Credential Content-Type is too large",
            ),
            Self::InvalidImmediateCredentialContentType => (
                error_code::INVALID_IMMEDIATE_CREDENTIAL_CONTENT_TYPE,
                ErrorKind::InvalidInput,
                "OID4VCI immediate Credential Content-Type is invalid",
            ),
            Self::CredentialResponseExceedsProofCount => (
                error_code::CREDENTIAL_RESPONSE_EXCEEDS_PROOF_COUNT,
                ErrorKind::InvalidInput,
                "OID4VCI Credential Response exceeds request proof count",
            ),
        };
        IdentusError::public(code, kind, CAPABILITY, message)
    }
}

impl From<CredentialOfferError> for IdentusError {
    fn from(value: CredentialOfferError) -> Self {
        value.to_identus_error()
    }
}

impl fmt::Display for CredentialOfferError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.to_identus_error().public_message())
    }
}

impl std::error::Error for CredentialOfferError {}
