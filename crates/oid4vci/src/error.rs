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
