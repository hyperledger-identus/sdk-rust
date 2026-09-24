//! Static, redaction-safe OID4VCI input-validation errors.

use std::fmt;

use identus_core::{CapabilityId, IdentusError};

use crate::error_contract::{
    ErrorContract, credential_nonce_http, deferred_immediate_issuance,
    issuer_authorization_server_metadata, offer_semantics_grants, offer_transport_json,
    token_request_response_errors,
};

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
    pub const INVALID_DEFERRED_CREDENTIAL_REQUEST_LIMITS: ErrorCode =
        ErrorCode::new("oid4vci.invalid_deferred_credential_request_limits");
    pub const DEFERRED_CREDENTIAL_ENDPOINT_REQUIRED: ErrorCode =
        ErrorCode::new("oid4vci.deferred_credential_endpoint_required");
    pub const DEFERRED_CREDENTIAL_REQUEST_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.deferred_credential_request_too_large");
    pub const INVALID_DEFERRED_CREDENTIAL_HTTP_RESPONSE_LIMITS: ErrorCode =
        ErrorCode::new("oid4vci.invalid_deferred_credential_http_response_limits");
    pub const INVALID_DEFERRED_CREDENTIAL_HTTP_STATUS: ErrorCode =
        ErrorCode::new("oid4vci.invalid_deferred_credential_http_status");
    pub const DEFERRED_CREDENTIAL_CONTENT_TYPE_TOO_LARGE: ErrorCode =
        ErrorCode::new("oid4vci.deferred_credential_content_type_too_large");
    pub const INVALID_DEFERRED_CREDENTIAL_CONTENT_TYPE: ErrorCode =
        ErrorCode::new("oid4vci.invalid_deferred_credential_content_type");
    pub const DEFERRED_CREDENTIAL_TRANSACTION_MISMATCH: ErrorCode =
        ErrorCode::new("oid4vci.deferred_credential_transaction_mismatch");
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
    InvalidDeferredCredentialRequestLimits,
    DeferredCredentialEndpointRequired,
    DeferredCredentialRequestTooLarge,
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
    InvalidDeferredCredentialHttpResponseLimits,
    InvalidDeferredCredentialHttpStatus,
    DeferredCredentialContentTypeTooLarge,
    InvalidDeferredCredentialContentType,
    DeferredCredentialTransactionMismatch,
}

macro_rules! define_credential_offer_error_contracts {
    ($($variant:ident => $contract:path),+ $(,)?) => {
        impl CredentialOfferError {
            const fn contract(self) -> ErrorContract {
                match self {
                    $(Self::$variant => $contract),+
                }
            }

            #[cfg(test)]
            pub(crate) const CONTRACT_VARIANTS: &'static [Self] = &[
                $(Self::$variant),+
            ];
        }
    };
}

define_credential_offer_error_contracts! {
    InvalidLimits => offer_transport_json::INVALID_LIMITS,
    InvocationTooLarge => offer_transport_json::INVOCATION_TOO_LARGE,
    InvalidInvocation => offer_transport_json::INVALID_INVOCATION,
    UnsupportedTransport => offer_transport_json::UNSUPPORTED_TRANSPORT,
    InvalidFormEncoding => offer_transport_json::INVALID_FORM_ENCODING,
    EmbeddedTooLarge => offer_transport_json::EMBEDDED_TOO_LARGE,
    InvalidEmbeddedJson => offer_transport_json::INVALID_EMBEDDED_JSON,
    DuplicateJsonProperty => offer_transport_json::DUPLICATE_JSON_PROPERTY,
    JsonTooDeep => offer_transport_json::JSON_TOO_DEEP,
    JsonTooManyNodes => offer_transport_json::JSON_TOO_MANY_NODES,
    ReferenceTooLarge => offer_transport_json::REFERENCE_TOO_LARGE,
    UnsafeReferenceUri => offer_transport_json::UNSAFE_REFERENCE_URI,
    InvalidSemanticLimits => offer_semantics_grants::INVALID_SEMANTIC_LIMITS,
    InvalidOfferFields => offer_semantics_grants::INVALID_OFFER_FIELDS,
    IssuerTooLarge => offer_semantics_grants::ISSUER_TOO_LARGE,
    UnsafeCredentialIssuer => offer_semantics_grants::UNSAFE_CREDENTIAL_ISSUER,
    InvalidConfigurationIds => offer_semantics_grants::INVALID_CONFIGURATION_IDS,
    ConfigurationIdTooLarge => offer_semantics_grants::CONFIGURATION_ID_TOO_LARGE,
    TooManyConfigurationIds => offer_semantics_grants::TOO_MANY_CONFIGURATION_IDS,
    DuplicateConfigurationId => offer_semantics_grants::DUPLICATE_CONFIGURATION_ID,
    InvalidGrants => offer_semantics_grants::INVALID_GRANTS,
    InvalidGrantLimits => offer_semantics_grants::INVALID_GRANT_LIMITS,
    InvalidAuthorizationCodeGrant => offer_semantics_grants::INVALID_AUTHORIZATION_CODE_GRANT,
    InvalidPreAuthorizedCodeGrant => offer_semantics_grants::INVALID_PRE_AUTHORIZED_CODE_GRANT,
    IssuerStateTooLarge => offer_semantics_grants::ISSUER_STATE_TOO_LARGE,
    PreAuthorizedCodeTooLarge => offer_semantics_grants::PRE_AUTHORIZED_CODE_TOO_LARGE,
    AuthorizationServerTooLarge => offer_semantics_grants::AUTHORIZATION_SERVER_TOO_LARGE,
    UnsafeAuthorizationServer => offer_semantics_grants::UNSAFE_AUTHORIZATION_SERVER,
    InvalidTransactionCode => offer_semantics_grants::INVALID_TRANSACTION_CODE,
    InvalidTransactionCodeMode => offer_semantics_grants::INVALID_TRANSACTION_CODE_MODE,
    InvalidTransactionCodeLength => offer_semantics_grants::INVALID_TRANSACTION_CODE_LENGTH,
    TransactionCodeLengthTooLarge => offer_semantics_grants::TRANSACTION_CODE_LENGTH_TOO_LARGE,
    TransactionCodeDescriptionTooLarge => offer_semantics_grants::TRANSACTION_CODE_DESCRIPTION_TOO_LARGE,
    InvalidMetadataLimits => issuer_authorization_server_metadata::INVALID_METADATA_LIMITS,
    MetadataTooLarge => issuer_authorization_server_metadata::METADATA_TOO_LARGE,
    InvalidMetadata => issuer_authorization_server_metadata::INVALID_METADATA,
    MetadataIssuerMismatch => issuer_authorization_server_metadata::METADATA_ISSUER_MISMATCH,
    CredentialEndpointTooLarge => issuer_authorization_server_metadata::CREDENTIAL_ENDPOINT_TOO_LARGE,
    UnsafeCredentialEndpoint => issuer_authorization_server_metadata::UNSAFE_CREDENTIAL_ENDPOINT,
    NonceEndpointTooLarge => issuer_authorization_server_metadata::NONCE_ENDPOINT_TOO_LARGE,
    UnsafeNonceEndpoint => issuer_authorization_server_metadata::UNSAFE_NONCE_ENDPOINT,
    DeferredCredentialEndpointTooLarge => issuer_authorization_server_metadata::DEFERRED_CREDENTIAL_ENDPOINT_TOO_LARGE,
    UnsafeDeferredCredentialEndpoint => issuer_authorization_server_metadata::UNSAFE_DEFERRED_CREDENTIAL_ENDPOINT,
    NonceEndpointRequired => issuer_authorization_server_metadata::NONCE_ENDPOINT_REQUIRED,
    InvalidAuthorizationServers => issuer_authorization_server_metadata::INVALID_AUTHORIZATION_SERVERS,
    TooManyAuthorizationServers => issuer_authorization_server_metadata::TOO_MANY_AUTHORIZATION_SERVERS,
    DuplicateAuthorizationServer => issuer_authorization_server_metadata::DUPLICATE_AUTHORIZATION_SERVER,
    InvalidCredentialConfigurations => issuer_authorization_server_metadata::INVALID_CREDENTIAL_CONFIGURATIONS,
    TooManyCredentialConfigurations => issuer_authorization_server_metadata::TOO_MANY_CREDENTIAL_CONFIGURATIONS,
    InvalidCredentialFormat => issuer_authorization_server_metadata::INVALID_CREDENTIAL_FORMAT,
    CredentialFormatTooLarge => issuer_authorization_server_metadata::CREDENTIAL_FORMAT_TOO_LARGE,
    OfferMetadataIssuerMismatch => issuer_authorization_server_metadata::OFFER_METADATA_ISSUER_MISMATCH,
    OfferedConfigurationMissing => issuer_authorization_server_metadata::OFFERED_CONFIGURATION_MISSING,
    InvalidAuthorizationServerHint => issuer_authorization_server_metadata::INVALID_AUTHORIZATION_SERVER_HINT,
    InvalidAuthorizationServerMetadataLimits => issuer_authorization_server_metadata::INVALID_AUTHORIZATION_SERVER_METADATA_LIMITS,
    AuthorizationServerMetadataTooLarge => issuer_authorization_server_metadata::AUTHORIZATION_SERVER_METADATA_TOO_LARGE,
    InvalidAuthorizationServerMetadata => issuer_authorization_server_metadata::INVALID_AUTHORIZATION_SERVER_METADATA,
    AuthorizationServerMetadataIssuerMismatch => issuer_authorization_server_metadata::AUTHORIZATION_SERVER_METADATA_ISSUER_MISMATCH,
    AuthorizationEndpointTooLarge => issuer_authorization_server_metadata::AUTHORIZATION_ENDPOINT_TOO_LARGE,
    UnsafeAuthorizationEndpoint => issuer_authorization_server_metadata::UNSAFE_AUTHORIZATION_ENDPOINT,
    TokenEndpointTooLarge => issuer_authorization_server_metadata::TOKEN_ENDPOINT_TOO_LARGE,
    UnsafeTokenEndpoint => issuer_authorization_server_metadata::UNSAFE_TOKEN_ENDPOINT,
    InvalidGrantTypes => issuer_authorization_server_metadata::INVALID_GRANT_TYPES,
    GrantTypeTooLarge => issuer_authorization_server_metadata::GRANT_TYPE_TOO_LARGE,
    TooManyGrantTypes => issuer_authorization_server_metadata::TOO_MANY_GRANT_TYPES,
    DuplicateGrantType => issuer_authorization_server_metadata::DUPLICATE_GRANT_TYPE,
    InvalidAnonymousPreAuthorizedAccess => issuer_authorization_server_metadata::INVALID_ANONYMOUS_PRE_AUTHORIZED_ACCESS,
    PreAuthorizedCodeGrantMissing => issuer_authorization_server_metadata::PRE_AUTHORIZED_CODE_GRANT_MISSING,
    AuthorizationServerNotAdvertised => issuer_authorization_server_metadata::AUTHORIZATION_SERVER_NOT_ADVERTISED,
    PreAuthorizedServerHintMismatch => issuer_authorization_server_metadata::PRE_AUTHORIZED_SERVER_HINT_MISMATCH,
    PreAuthorizedGrantNotSupported => issuer_authorization_server_metadata::PRE_AUTHORIZED_GRANT_NOT_SUPPORTED,
    TokenEndpointRequired => issuer_authorization_server_metadata::TOKEN_ENDPOINT_REQUIRED,
    InvalidTransactionCodeInputLimits => token_request_response_errors::INVALID_TRANSACTION_CODE_INPUT_LIMITS,
    TransactionCodeInputRequired => token_request_response_errors::TRANSACTION_CODE_INPUT_REQUIRED,
    TransactionCodeInputUnexpected => token_request_response_errors::TRANSACTION_CODE_INPUT_UNEXPECTED,
    TransactionCodeInputEmpty => token_request_response_errors::TRANSACTION_CODE_INPUT_EMPTY,
    TransactionCodeInputTooLarge => token_request_response_errors::TRANSACTION_CODE_INPUT_TOO_LARGE,
    InvalidPreAuthorizedTokenRequestLimits => token_request_response_errors::INVALID_PRE_AUTHORIZED_TOKEN_REQUEST_LIMITS,
    PreAuthorizedTokenRequestTooLarge => token_request_response_errors::PRE_AUTHORIZED_TOKEN_REQUEST_TOO_LARGE,
    InvalidTokenResponseLimits => token_request_response_errors::INVALID_TOKEN_RESPONSE_LIMITS,
    TokenResponseTooLarge => token_request_response_errors::TOKEN_RESPONSE_TOO_LARGE,
    InvalidTokenResponse => token_request_response_errors::INVALID_TOKEN_RESPONSE,
    InvalidAccessToken => token_request_response_errors::INVALID_ACCESS_TOKEN,
    AccessTokenTooLarge => token_request_response_errors::ACCESS_TOKEN_TOO_LARGE,
    InvalidTokenType => token_request_response_errors::INVALID_TOKEN_TYPE,
    TokenTypeTooLarge => token_request_response_errors::TOKEN_TYPE_TOO_LARGE,
    InvalidTokenExpiresIn => token_request_response_errors::INVALID_TOKEN_EXPIRES_IN,
    InvalidRefreshToken => token_request_response_errors::INVALID_REFRESH_TOKEN,
    RefreshTokenTooLarge => token_request_response_errors::REFRESH_TOKEN_TOO_LARGE,
    InvalidTokenScope => token_request_response_errors::INVALID_TOKEN_SCOPE,
    TokenScopeTooLarge => token_request_response_errors::TOKEN_SCOPE_TOO_LARGE,
    InvalidTokenAuthorizationDetailsLimits => token_request_response_errors::INVALID_TOKEN_AUTHORIZATION_DETAILS_LIMITS,
    InvalidTokenAuthorizationDetails => token_request_response_errors::INVALID_TOKEN_AUTHORIZATION_DETAILS,
    TooManyTokenAuthorizationDetails => token_request_response_errors::TOO_MANY_TOKEN_AUTHORIZATION_DETAILS,
    TokenAuthorizationDetailValueTooLarge => token_request_response_errors::TOKEN_AUTHORIZATION_DETAIL_VALUE_TOO_LARGE,
    TooManyCredentialIdentifiers => token_request_response_errors::TOO_MANY_CREDENTIAL_IDENTIFIERS,
    DuplicateCredentialIdentifier => token_request_response_errors::DUPLICATE_CREDENTIAL_IDENTIFIER,
    InvalidTokenErrorResponseLimits => token_request_response_errors::INVALID_TOKEN_ERROR_RESPONSE_LIMITS,
    TokenErrorResponseTooLarge => token_request_response_errors::TOKEN_ERROR_RESPONSE_TOO_LARGE,
    InvalidTokenErrorResponse => token_request_response_errors::INVALID_TOKEN_ERROR_RESPONSE,
    InvalidTokenEndpointErrorCode => token_request_response_errors::INVALID_TOKEN_ENDPOINT_ERROR_CODE,
    TokenEndpointErrorCodeTooLarge => token_request_response_errors::TOKEN_ENDPOINT_ERROR_CODE_TOO_LARGE,
    InvalidTokenErrorDescription => token_request_response_errors::INVALID_TOKEN_ERROR_DESCRIPTION,
    TokenErrorDescriptionTooLarge => token_request_response_errors::TOKEN_ERROR_DESCRIPTION_TOO_LARGE,
    InvalidTokenErrorUri => token_request_response_errors::INVALID_TOKEN_ERROR_URI,
    TokenErrorUriTooLarge => token_request_response_errors::TOKEN_ERROR_URI_TOO_LARGE,
    InvalidCredentialErrorResponseLimits => credential_nonce_http::INVALID_CREDENTIAL_ERROR_RESPONSE_LIMITS,
    CredentialErrorResponseTooLarge => credential_nonce_http::CREDENTIAL_ERROR_RESPONSE_TOO_LARGE,
    InvalidCredentialErrorResponse => credential_nonce_http::INVALID_CREDENTIAL_ERROR_RESPONSE,
    InvalidCredentialEndpointErrorCode => credential_nonce_http::INVALID_CREDENTIAL_ENDPOINT_ERROR_CODE,
    CredentialEndpointErrorCodeTooLarge => credential_nonce_http::CREDENTIAL_ENDPOINT_ERROR_CODE_TOO_LARGE,
    InvalidCredentialErrorDescription => credential_nonce_http::INVALID_CREDENTIAL_ERROR_DESCRIPTION,
    CredentialErrorDescriptionTooLarge => credential_nonce_http::CREDENTIAL_ERROR_DESCRIPTION_TOO_LARGE,
    InvalidCredentialErrorHttpResponseLimits => credential_nonce_http::INVALID_CREDENTIAL_ERROR_HTTP_RESPONSE_LIMITS,
    InvalidCredentialErrorHttpStatus => credential_nonce_http::INVALID_CREDENTIAL_ERROR_HTTP_STATUS,
    CredentialErrorContentTypeTooLarge => credential_nonce_http::CREDENTIAL_ERROR_CONTENT_TYPE_TOO_LARGE,
    InvalidCredentialErrorContentType => credential_nonce_http::INVALID_CREDENTIAL_ERROR_CONTENT_TYPE,
    GenericCredentialErrorCodeForbidden => credential_nonce_http::GENERIC_CREDENTIAL_ERROR_CODE_FORBIDDEN,
    InvalidCredentialNonceResponseLimits => credential_nonce_http::INVALID_CREDENTIAL_NONCE_RESPONSE_LIMITS,
    CredentialNonceResponseTooLarge => credential_nonce_http::CREDENTIAL_NONCE_RESPONSE_TOO_LARGE,
    InvalidCredentialNonceResponse => credential_nonce_http::INVALID_CREDENTIAL_NONCE_RESPONSE,
    InvalidCredentialNonce => credential_nonce_http::INVALID_CREDENTIAL_NONCE,
    CredentialNonceTooLarge => credential_nonce_http::CREDENTIAL_NONCE_TOO_LARGE,
    InvalidCredentialNonceHttpResponseLimits => credential_nonce_http::INVALID_CREDENTIAL_NONCE_HTTP_RESPONSE_LIMITS,
    InvalidCredentialNonceHttpStatus => credential_nonce_http::INVALID_CREDENTIAL_NONCE_HTTP_STATUS,
    CredentialNonceContentTypeTooLarge => credential_nonce_http::CREDENTIAL_NONCE_CONTENT_TYPE_TOO_LARGE,
    InvalidCredentialNonceContentType => credential_nonce_http::INVALID_CREDENTIAL_NONCE_CONTENT_TYPE,
    CredentialNonceCacheControlTooLarge => credential_nonce_http::CREDENTIAL_NONCE_CACHE_CONTROL_TOO_LARGE,
    InvalidCredentialNonceCacheControl => credential_nonce_http::INVALID_CREDENTIAL_NONCE_CACHE_CONTROL,
    InvalidJwtCredentialRequestLimits => credential_nonce_http::INVALID_JWT_CREDENTIAL_REQUEST_LIMITS,
    CredentialRequestConfigurationMissing => credential_nonce_http::CREDENTIAL_REQUEST_CONFIGURATION_MISSING,
    CredentialRequestAuthorizationDetailMissing => credential_nonce_http::CREDENTIAL_REQUEST_AUTHORIZATION_DETAIL_MISSING,
    CredentialRequestIdentifierMissing => credential_nonce_http::CREDENTIAL_REQUEST_IDENTIFIER_MISSING,
    CredentialRequestAuthorizationConfigurationMismatch => credential_nonce_http::CREDENTIAL_REQUEST_AUTHORIZATION_CONFIGURATION_MISMATCH,
    CredentialRequestAuthorizationDetailsUnsupported => credential_nonce_http::CREDENTIAL_REQUEST_AUTHORIZATION_DETAILS_UNSUPPORTED,
    CredentialRequestTokenTypeUnsupported => credential_nonce_http::CREDENTIAL_REQUEST_TOKEN_TYPE_UNSUPPORTED,
    InvalidCredentialRequestBearerToken => credential_nonce_http::INVALID_CREDENTIAL_REQUEST_BEARER_TOKEN,
    CredentialRequestProofsRequired => credential_nonce_http::CREDENTIAL_REQUEST_PROOFS_REQUIRED,
    TooManyCredentialRequestProofs => credential_nonce_http::TOO_MANY_CREDENTIAL_REQUEST_PROOFS,
    CredentialRequestProofTooLarge => credential_nonce_http::CREDENTIAL_REQUEST_PROOF_TOO_LARGE,
    CredentialRequestAuthorizationTooLarge => credential_nonce_http::CREDENTIAL_REQUEST_AUTHORIZATION_TOO_LARGE,
    CredentialRequestBodyTooLarge => credential_nonce_http::CREDENTIAL_REQUEST_BODY_TOO_LARGE,
    InvalidDeferredCredentialRequestLimits => deferred_immediate_issuance::INVALID_DEFERRED_CREDENTIAL_REQUEST_LIMITS,
    DeferredCredentialEndpointRequired => deferred_immediate_issuance::DEFERRED_CREDENTIAL_ENDPOINT_REQUIRED,
    DeferredCredentialRequestTooLarge => deferred_immediate_issuance::DEFERRED_CREDENTIAL_REQUEST_TOO_LARGE,
    InvalidDeferredCredentialResponseLimits => deferred_immediate_issuance::INVALID_DEFERRED_CREDENTIAL_RESPONSE_LIMITS,
    DeferredCredentialResponseTooLarge => deferred_immediate_issuance::DEFERRED_CREDENTIAL_RESPONSE_TOO_LARGE,
    InvalidDeferredCredentialResponse => deferred_immediate_issuance::INVALID_DEFERRED_CREDENTIAL_RESPONSE,
    TooManyDeferredCredentialResponseMembers => deferred_immediate_issuance::TOO_MANY_DEFERRED_CREDENTIAL_RESPONSE_MEMBERS,
    InvalidDeferredTransactionId => deferred_immediate_issuance::INVALID_DEFERRED_TRANSACTION_ID,
    DeferredTransactionIdTooLarge => deferred_immediate_issuance::DEFERRED_TRANSACTION_ID_TOO_LARGE,
    InvalidDeferredCredentialInterval => deferred_immediate_issuance::INVALID_DEFERRED_CREDENTIAL_INTERVAL,
    DeferredCredentialIntervalTooLarge => deferred_immediate_issuance::DEFERRED_CREDENTIAL_INTERVAL_TOO_LARGE,
    DeferredCredentialResponseBranchConflict => deferred_immediate_issuance::DEFERRED_CREDENTIAL_RESPONSE_BRANCH_CONFLICT,
    InvalidImmediateCredentialResponseLimits => deferred_immediate_issuance::INVALID_IMMEDIATE_CREDENTIAL_RESPONSE_LIMITS,
    ImmediateCredentialResponseTooLarge => deferred_immediate_issuance::IMMEDIATE_CREDENTIAL_RESPONSE_TOO_LARGE,
    InvalidImmediateCredentialResponse => deferred_immediate_issuance::INVALID_IMMEDIATE_CREDENTIAL_RESPONSE,
    DeferredCredentialResponseUnsupported => deferred_immediate_issuance::DEFERRED_CREDENTIAL_RESPONSE_UNSUPPORTED,
    TooManyCredentialResponseMembers => deferred_immediate_issuance::TOO_MANY_CREDENTIAL_RESPONSE_MEMBERS,
    TooManyIssuedCredentials => deferred_immediate_issuance::TOO_MANY_ISSUED_CREDENTIALS,
    TooManyIssuedCredentialMembers => deferred_immediate_issuance::TOO_MANY_ISSUED_CREDENTIAL_MEMBERS,
    InvalidIssuedCredential => deferred_immediate_issuance::INVALID_ISSUED_CREDENTIAL,
    IssuedCredentialTooLarge => deferred_immediate_issuance::ISSUED_CREDENTIAL_TOO_LARGE,
    IssuedCredentialsTooLarge => deferred_immediate_issuance::ISSUED_CREDENTIALS_TOO_LARGE,
    InvalidCredentialNotificationId => deferred_immediate_issuance::INVALID_CREDENTIAL_NOTIFICATION_ID,
    CredentialNotificationIdTooLarge => deferred_immediate_issuance::CREDENTIAL_NOTIFICATION_ID_TOO_LARGE,
    InvalidImmediateCredentialHttpResponseLimits => deferred_immediate_issuance::INVALID_IMMEDIATE_CREDENTIAL_HTTP_RESPONSE_LIMITS,
    InvalidImmediateCredentialHttpStatus => deferred_immediate_issuance::INVALID_IMMEDIATE_CREDENTIAL_HTTP_STATUS,
    ImmediateCredentialContentTypeTooLarge => deferred_immediate_issuance::IMMEDIATE_CREDENTIAL_CONTENT_TYPE_TOO_LARGE,
    InvalidImmediateCredentialContentType => deferred_immediate_issuance::INVALID_IMMEDIATE_CREDENTIAL_CONTENT_TYPE,
    CredentialResponseExceedsProofCount => deferred_immediate_issuance::CREDENTIAL_RESPONSE_EXCEEDS_PROOF_COUNT,
    InvalidDeferredCredentialHttpResponseLimits => deferred_immediate_issuance::INVALID_DEFERRED_CREDENTIAL_HTTP_RESPONSE_LIMITS,
    InvalidDeferredCredentialHttpStatus => deferred_immediate_issuance::INVALID_DEFERRED_CREDENTIAL_HTTP_STATUS,
    DeferredCredentialContentTypeTooLarge => deferred_immediate_issuance::DEFERRED_CREDENTIAL_CONTENT_TYPE_TOO_LARGE,
    InvalidDeferredCredentialContentType => deferred_immediate_issuance::INVALID_DEFERRED_CREDENTIAL_CONTENT_TYPE,
    DeferredCredentialTransactionMismatch => deferred_immediate_issuance::DEFERRED_CREDENTIAL_TRANSACTION_MISMATCH,
}

impl CredentialOfferError {
    /// Convert to the workspace-wide redaction-safe error contract.
    pub const fn to_identus_error(self) -> IdentusError {
        self.contract().to_identus_error()
    }
}

impl From<CredentialOfferError> for IdentusError {
    fn from(value: CredentialOfferError) -> Self {
        value.to_identus_error()
    }
}

impl fmt::Display for CredentialOfferError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.contract().message())
    }
}

impl std::error::Error for CredentialOfferError {}
