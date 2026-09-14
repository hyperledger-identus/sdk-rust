use identus_core::ErrorKind;

use super::ErrorContract;
use crate::error::error_code;

pub(crate) const INVALID_METADATA_LIMITS: ErrorContract = ErrorContract::new(
    error_code::INVALID_METADATA_LIMITS,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Issuer Metadata limits are invalid",
);

pub(crate) const METADATA_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::METADATA_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Issuer Metadata is too large",
);

pub(crate) const INVALID_METADATA: ErrorContract = ErrorContract::new(
    error_code::INVALID_METADATA,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Issuer Metadata is invalid",
);

pub(crate) const METADATA_ISSUER_MISMATCH: ErrorContract = ErrorContract::new(
    error_code::METADATA_ISSUER_MISMATCH,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Issuer Metadata identifier does not match",
);

pub(crate) const CREDENTIAL_ENDPOINT_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::CREDENTIAL_ENDPOINT_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Endpoint is too large",
);

pub(crate) const UNSAFE_CREDENTIAL_ENDPOINT: ErrorContract = ErrorContract::new(
    error_code::UNSAFE_CREDENTIAL_ENDPOINT,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Endpoint is unsafe",
);

pub(crate) const NONCE_ENDPOINT_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::NONCE_ENDPOINT_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Nonce Endpoint is too large",
);

pub(crate) const UNSAFE_NONCE_ENDPOINT: ErrorContract = ErrorContract::new(
    error_code::UNSAFE_NONCE_ENDPOINT,
    ErrorKind::InvalidInput,
    "OID4VCI Nonce Endpoint is unsafe",
);

pub(crate) const DEFERRED_CREDENTIAL_ENDPOINT_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::DEFERRED_CREDENTIAL_ENDPOINT_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Deferred Credential Endpoint is too large",
);

pub(crate) const UNSAFE_DEFERRED_CREDENTIAL_ENDPOINT: ErrorContract = ErrorContract::new(
    error_code::UNSAFE_DEFERRED_CREDENTIAL_ENDPOINT,
    ErrorKind::InvalidInput,
    "OID4VCI Deferred Credential Endpoint is unsafe",
);

pub(crate) const NONCE_ENDPOINT_REQUIRED: ErrorContract = ErrorContract::new(
    error_code::NONCE_ENDPOINT_REQUIRED,
    ErrorKind::InvalidInput,
    "OID4VCI Nonce Endpoint is required",
);

pub(crate) const INVALID_AUTHORIZATION_SERVERS: ErrorContract = ErrorContract::new(
    error_code::INVALID_AUTHORIZATION_SERVERS,
    ErrorKind::InvalidInput,
    "OID4VCI Authorization Server metadata is invalid",
);

pub(crate) const TOO_MANY_AUTHORIZATION_SERVERS: ErrorContract = ErrorContract::new(
    error_code::TOO_MANY_AUTHORIZATION_SERVERS,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Issuer Metadata has too many Authorization Servers",
);

pub(crate) const DUPLICATE_AUTHORIZATION_SERVER: ErrorContract = ErrorContract::new(
    error_code::DUPLICATE_AUTHORIZATION_SERVER,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Issuer Metadata repeats an Authorization Server",
);

pub(crate) const INVALID_CREDENTIAL_CONFIGURATIONS: ErrorContract = ErrorContract::new(
    error_code::INVALID_CREDENTIAL_CONFIGURATIONS,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Configurations are invalid",
);

pub(crate) const TOO_MANY_CREDENTIAL_CONFIGURATIONS: ErrorContract = ErrorContract::new(
    error_code::TOO_MANY_CREDENTIAL_CONFIGURATIONS,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Issuer Metadata has too many configurations",
);

pub(crate) const INVALID_CREDENTIAL_FORMAT: ErrorContract = ErrorContract::new(
    error_code::INVALID_CREDENTIAL_FORMAT,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Format identifier is invalid",
);

pub(crate) const CREDENTIAL_FORMAT_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::CREDENTIAL_FORMAT_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Format identifier is too large",
);

pub(crate) const OFFER_METADATA_ISSUER_MISMATCH: ErrorContract = ErrorContract::new(
    error_code::OFFER_METADATA_ISSUER_MISMATCH,
    ErrorKind::InvalidInput,
    "OID4VCI offer and metadata issuer identifiers do not match",
);

pub(crate) const OFFERED_CONFIGURATION_MISSING: ErrorContract = ErrorContract::new(
    error_code::OFFERED_CONFIGURATION_MISSING,
    ErrorKind::InvalidInput,
    "OID4VCI offered Credential Configuration is missing from metadata",
);

pub(crate) const INVALID_AUTHORIZATION_SERVER_HINT: ErrorContract = ErrorContract::new(
    error_code::INVALID_AUTHORIZATION_SERVER_HINT,
    ErrorKind::InvalidInput,
    "OID4VCI offered Authorization Server hint is invalid for metadata",
);

pub(crate) const INVALID_AUTHORIZATION_SERVER_METADATA_LIMITS: ErrorContract = ErrorContract::new(
    error_code::INVALID_AUTHORIZATION_SERVER_METADATA_LIMITS,
    ErrorKind::InvalidInput,
    "OID4VCI Authorization Server Metadata limits are invalid",
);

pub(crate) const AUTHORIZATION_SERVER_METADATA_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::AUTHORIZATION_SERVER_METADATA_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Authorization Server Metadata is too large",
);

pub(crate) const INVALID_AUTHORIZATION_SERVER_METADATA: ErrorContract = ErrorContract::new(
    error_code::INVALID_AUTHORIZATION_SERVER_METADATA,
    ErrorKind::InvalidInput,
    "OID4VCI Authorization Server Metadata core is invalid",
);

pub(crate) const AUTHORIZATION_SERVER_METADATA_ISSUER_MISMATCH: ErrorContract = ErrorContract::new(
    error_code::AUTHORIZATION_SERVER_METADATA_ISSUER_MISMATCH,
    ErrorKind::InvalidInput,
    "OID4VCI Authorization Server Metadata issuer does not match",
);

pub(crate) const AUTHORIZATION_ENDPOINT_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::AUTHORIZATION_ENDPOINT_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Authorization Endpoint is too large",
);

pub(crate) const UNSAFE_AUTHORIZATION_ENDPOINT: ErrorContract = ErrorContract::new(
    error_code::UNSAFE_AUTHORIZATION_ENDPOINT,
    ErrorKind::InvalidInput,
    "OID4VCI Authorization Endpoint is unsafe",
);

pub(crate) const TOKEN_ENDPOINT_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::TOKEN_ENDPOINT_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Token Endpoint is too large",
);

pub(crate) const UNSAFE_TOKEN_ENDPOINT: ErrorContract = ErrorContract::new(
    error_code::UNSAFE_TOKEN_ENDPOINT,
    ErrorKind::InvalidInput,
    "OID4VCI Token Endpoint is unsafe",
);

pub(crate) const INVALID_GRANT_TYPES: ErrorContract = ErrorContract::new(
    error_code::INVALID_GRANT_TYPES,
    ErrorKind::InvalidInput,
    "OID4VCI Authorization Server grant types are invalid",
);

pub(crate) const GRANT_TYPE_TOO_LARGE: ErrorContract = ErrorContract::new(
    error_code::GRANT_TYPE_TOO_LARGE,
    ErrorKind::InvalidInput,
    "OID4VCI Authorization Server grant type is too large",
);

pub(crate) const TOO_MANY_GRANT_TYPES: ErrorContract = ErrorContract::new(
    error_code::TOO_MANY_GRANT_TYPES,
    ErrorKind::InvalidInput,
    "OID4VCI Authorization Server Metadata has too many grant types",
);

pub(crate) const DUPLICATE_GRANT_TYPE: ErrorContract = ErrorContract::new(
    error_code::DUPLICATE_GRANT_TYPE,
    ErrorKind::InvalidInput,
    "OID4VCI Authorization Server Metadata repeats a grant type",
);

pub(crate) const INVALID_ANONYMOUS_PRE_AUTHORIZED_ACCESS: ErrorContract = ErrorContract::new(
    error_code::INVALID_ANONYMOUS_PRE_AUTHORIZED_ACCESS,
    ErrorKind::InvalidInput,
    "OID4VCI anonymous Pre-Authorized Code metadata is invalid",
);

pub(crate) const PRE_AUTHORIZED_CODE_GRANT_MISSING: ErrorContract = ErrorContract::new(
    error_code::PRE_AUTHORIZED_CODE_GRANT_MISSING,
    ErrorKind::InvalidInput,
    "OID4VCI Credential Offer has no Pre-Authorized Code grant",
);

pub(crate) const AUTHORIZATION_SERVER_NOT_ADVERTISED: ErrorContract = ErrorContract::new(
    error_code::AUTHORIZATION_SERVER_NOT_ADVERTISED,
    ErrorKind::InvalidInput,
    "OID4VCI selected Authorization Server is not advertised",
);

pub(crate) const PRE_AUTHORIZED_SERVER_HINT_MISMATCH: ErrorContract = ErrorContract::new(
    error_code::PRE_AUTHORIZED_SERVER_HINT_MISMATCH,
    ErrorKind::InvalidInput,
    "OID4VCI selected Authorization Server does not match the offered hint",
);

pub(crate) const PRE_AUTHORIZED_GRANT_NOT_SUPPORTED: ErrorContract = ErrorContract::new(
    error_code::PRE_AUTHORIZED_GRANT_NOT_SUPPORTED,
    ErrorKind::InvalidInput,
    "OID4VCI selected Authorization Server does not support the Pre-Authorized Code grant",
);

pub(crate) const TOKEN_ENDPOINT_REQUIRED: ErrorContract = ErrorContract::new(
    error_code::TOKEN_ENDPOINT_REQUIRED,
    ErrorKind::InvalidInput,
    "OID4VCI selected Authorization Server has no Token Endpoint",
);
