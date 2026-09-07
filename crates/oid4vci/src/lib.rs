//! Bounded OpenID for Verifiable Credential Issuance protocol semantics.
//!
//! The crate distinguishes Credential Offer invocation transport, offer/grant
//! semantics, unsigned Credential Issuer Metadata with optional Final Nonce
//! Endpoint discovery, exact cross-document
//! agreement, a partial Authorization Server Metadata core, and explicit
//! Pre-Authorized Code server and Transaction Code input binding, and bounded
//! construction of the mandatory Pre-Authorized Token Request form plus a
//! partial successful Token Response, Token Error Response, and Credential
//! Nonce Response core, plus a transport-neutral Final Credential Nonce
//! Request description and bounded validation of its mandatory HTTP response
//! metadata, and bounded construction of the unencrypted Final
//! configuration-ID/JWT-proof Credential Request, and bounded parsing of the
//! unencrypted immediate Final Credential Response core. It performs no network
//! access, does not validate complete RFC 8414 metadata or Token Response
//! Authorization Details, and establishes no issuer, server, token, proof, or
//! nonce trust.

#![forbid(unsafe_code)]

mod authorization_metadata;
mod credential_nonce_http_response;
mod credential_nonce_request;
mod credential_nonce_response;
mod error;
mod grants;
mod http_field;
mod immediate_credential_http_response;
mod immediate_credential_response;
mod json;
mod jwt_credential_request;
mod limits;
mod metadata;
mod pre_authorized_server;
mod pre_authorized_token_request;
mod semantic;
mod token_error_response;
mod token_response;
mod transaction_code_input;
mod transport;

pub use authorization_metadata::{
    AUTHORIZATION_CODE_GRANT_TYPE, AuthorizationEndpoint, AuthorizationServerMetadataCore,
    GrantTypeIdentifier, IMPLICIT_GRANT_TYPE, PRE_AUTHORIZED_CODE_GRANT_TYPE, TokenEndpoint,
};
pub use credential_nonce_request::{
    CredentialNonceRequest, NONCE_REQUEST_BODY, NONCE_REQUEST_HTTP_METHOD,
};
pub use credential_nonce_response::{CredentialNonce, CredentialNonceResponseCore};
pub use error::{CAPABILITY, CredentialOfferError, error_code};
pub use grants::{
    AuthorizationCodeGrant, AuthorizationServerIdentifier, CredentialOfferWithGrants, IssuerState,
    PreAuthorizedCode, PreAuthorizedCodeGrant, TransactionCodeDescription,
    TransactionCodeInputMode, TransactionCodeRequirements,
};
pub use immediate_credential_http_response::RequestBoundImmediateCredentialResponse;
pub use immediate_credential_response::{
    CredentialValueKind, ImmediateCredentialResponseCore, IssuedCredential,
};
pub use jwt_credential_request::{
    CREDENTIAL_REQUEST_HTTP_METHOD, CREDENTIAL_REQUEST_MEDIA_TYPE, JwtCredentialRequest,
};
pub use limits::{
    AuthorizationServerMetadataLimits, CredentialIssuerMetadataLimits,
    CredentialNonceHttpResponseLimits, CredentialNonceResponseLimits, CredentialOfferGrantLimits,
    CredentialOfferLimits, CredentialOfferSemanticLimits, ImmediateCredentialHttpResponseLimits,
    ImmediateCredentialResponseLimits, JwtCredentialRequestLimits, MAX_CONFIGURABLE_JSON_DEPTH,
    PreAuthorizedTokenRequestLimits, TokenErrorResponseLimits, TokenResponseLimits,
    TransactionCodeInputLimits,
};
pub use metadata::{
    CredentialConfigurationSummary, CredentialEndpoint, CredentialFormatIdentifier,
    CredentialIssuerMetadata, CredentialOfferWithMetadata, NonceEndpoint,
};
pub use pre_authorized_server::CredentialOfferWithPreAuthorizedServer;
pub use pre_authorized_token_request::{
    PreAuthorizedTokenRequest, TOKEN_REQUEST_HTTP_METHOD, TOKEN_REQUEST_MEDIA_TYPE,
};
pub use semantic::{CredentialConfigurationId, CredentialIssuerIdentifier, CredentialOffer};
pub use token_error_response::{
    TokenEndpointErrorCode, TokenEndpointErrorKind, TokenErrorResponseCore, TokenErrorUri,
};
pub use token_response::{TokenResponseCore, TokenType};
pub use transaction_code_input::CredentialOfferWithPreAuthorizedTokenInput;
pub use transport::{CredentialOfferReference, CredentialOfferRequest, EmbeddedCredentialOffer};

use identus_core::Component;

/// Metadata for the `identus-oid4vci` crate.
pub const COMPONENT: Component = Component {
    name: "identus-oid4vci",
    summary: "Bounded OID4VCI Final protocol semantics.",
};
